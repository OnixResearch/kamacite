#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "This total benchmark core returns ordered diagnostics and optional summaries, so runtime assertions would change its fail-closed contract."
    )
)]

use std::collections::BTreeMap;

use crate::bend_hvm2::model::{
    BASIS_POINTS_PER_UNIT, BenchmarkSummary, BendHvm2Diagnostic, BendHvm2Profile,
    PILOT_MEASURED_RUNS, PilotBoundary, PilotStage, StageObservation,
};
use crate::bend_hvm2::profile::diagnostic;

pub(super) fn summarize_benchmark(
    observations: &[StageObservation],
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) -> Option<BenchmarkSummary> {
    let grouped = measured_durations(observations);
    let oracle = summary_for(grouped.get(&PilotStage::TrellisOracle)?)?;
    let run_rs = summary_for(grouped.get(&PilotStage::BendRunRs)?)?;
    let generated = summary_for(grouped.get(&PilotStage::GeneratedC)?)?;
    let run_rs_ratio = integer_ratio(run_rs.0, oracle.0);
    let generated_ratio = integer_ratio(generated.0, oracle.0);
    let (Some(run_rs_ratio), Some(generated_ratio)) = (run_rs_ratio, generated_ratio) else {
        diagnostics.push(diagnostic(
            PilotBoundary::Benchmark,
            "benchmark-overflow",
            "benchmark.ratio",
            "benchmark ratio cannot fit in the receipt integer domain",
        ));
        return None;
    };
    Some(BenchmarkSummary {
        oracle_median_nanoseconds: oracle.0,
        run_rs_median_nanoseconds: run_rs.0,
        generated_c_median_nanoseconds: generated.0,
        oracle_spread_basis_points: oracle.1,
        run_rs_spread_basis_points: run_rs.1,
        generated_c_spread_basis_points: generated.1,
        run_rs_oracle_ratio: run_rs_ratio,
        generated_c_oracle_ratio: generated_ratio,
    })
}

pub(super) fn validate_benchmark(
    profile: &BendHvm2Profile,
    summary: &BenchmarkSummary,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let maximum_spread = profile.benchmark.maximum_spread_basis_points;
    if summary.oracle_spread_basis_points > maximum_spread
        || summary.run_rs_spread_basis_points > maximum_spread
        || summary.generated_c_spread_basis_points > maximum_spread
    {
        diagnostics.push(diagnostic(
            PilotBoundary::Benchmark,
            "unstable-measurements",
            "benchmark.spread",
            "one route exceeds the declared measurement spread",
        ));
    }
    if summary.run_rs_oracle_ratio > profile.benchmark.maximum_run_rs_oracle_ratio {
        diagnostics.push(diagnostic(
            PilotBoundary::Benchmark,
            "run-rs-performance",
            "benchmark.run_rs_oracle_ratio",
            "bend run-rs missed the declared oracle ratio",
        ));
    }
    if summary.generated_c_oracle_ratio > profile.benchmark.maximum_generated_c_oracle_ratio {
        diagnostics.push(diagnostic(
            PilotBoundary::Benchmark,
            "generated-c-performance",
            "benchmark.generated_c_oracle_ratio",
            "generated C missed the declared oracle ratio",
        ));
    }
}

fn measured_durations(observations: &[StageObservation]) -> BTreeMap<PilotStage, Vec<u64>> {
    let mut grouped = BTreeMap::new();
    for observation in observations
        .iter()
        .filter(|observation| !observation.warmup)
    {
        if matches!(
            observation.stage,
            PilotStage::TrellisOracle | PilotStage::BendRunRs | PilotStage::GeneratedC
        ) {
            grouped
                .entry(observation.stage)
                .or_insert_with(Vec::new)
                .push(observation.wall_nanoseconds);
        }
    }
    grouped
}

fn summary_for(values: &[u64]) -> Option<(u64, u64)> {
    if values.len() != PILOT_MEASURED_RUNS || values.is_empty() {
        return None;
    }
    let mut values = values.to_vec();
    values.sort_unstable();
    let median = values[values.len() / 2].max(1);
    let spread = values.last()?.saturating_sub(*values.first()?);
    let spread_basis_points = u128::from(spread)
        .saturating_mul(BASIS_POINTS_PER_UNIT)
        .checked_div(u128::from(median))?;
    u64::try_from(spread_basis_points)
        .ok()
        .map(|spread| (median, spread))
}

fn integer_ratio(candidate: u64, oracle: u64) -> Option<u64> {
    let denominator = u128::from(oracle.max(1));
    let numerator = u128::from(candidate).saturating_add(denominator.saturating_sub(1));
    u64::try_from(numerator.checked_div(denominator)?).ok()
}
