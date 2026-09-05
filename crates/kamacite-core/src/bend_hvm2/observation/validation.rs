#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::assertion_density,
        tigerstyle::compound_condition,
        tigerstyle::fragile_exhaustive_enum_match,
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        tigerstyle::path_segment_repetition,
        reason = "This diagnostic validator keeps compound protocol predicates explicit and returns all failures without assertion panics."
    )
)]

use super::command_profile_identity;
use crate::bend_hvm2::model::{
    BendHvm2Diagnostic, BendHvm2Profile, PilotBoundary, PilotStage, ProcessOutcome,
    StageObservation,
};
use crate::bend_hvm2::profile::diagnostic;

const NANOSECONDS_PER_MILLISECOND: u64 = 1_000_000;

pub(super) fn validate_stage_counts(
    profile: &BendHvm2Profile,
    observations: &[StageObservation],
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let Some(route_count) = profile
        .benchmark
        .warmup_runs
        .checked_add(profile.benchmark.measured_runs)
    else {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "repetition-overflow",
            "benchmark",
            "benchmark repetition counts overflow the platform integer domain",
        ));
        return;
    };
    for stage in [
        PilotStage::TrellisOracle,
        PilotStage::BendRunRs,
        PilotStage::GeneratedC,
    ] {
        let stage_observations = observations
            .iter()
            .filter(|observation| observation.stage == stage)
            .collect::<Vec<_>>();
        let warmups = stage_observations
            .iter()
            .filter(|observation| observation.warmup)
            .count();
        if stage_observations.len() != route_count || warmups != profile.benchmark.warmup_runs {
            diagnostics.push(diagnostic(
                PilotBoundary::Execution,
                "repetition-count",
                format!("observations.{stage:?}"),
                "route observations do not match the declared warm-up and measured counts",
            ));
        }
    }
    for stage in [PilotStage::BendGenC, PilotStage::CCompile] {
        let count = observations
            .iter()
            .filter(|observation| observation.stage == stage)
            .count();
        if count != 1 {
            diagnostics.push(diagnostic(
                PilotBoundary::Execution,
                "build-stage-count",
                format!("observations.{stage:?}"),
                "generated-source and compiler stages require exactly one observation",
            ));
        }
    }
}

pub(super) fn validate_observation(
    profile: &BendHvm2Profile,
    observation: &StageObservation,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let expected_identity = command_profile_identity(profile, observation.stage);
    if expected_identity.as_ref().ok() != Some(&observation.command_profile_identity) {
        diagnostics.push(diagnostic(
            PilotBoundary::Execution,
            "command-profile-drift",
            "observation.command_profile_identity",
            "stage observation does not bind its reviewed command profile",
        ));
    }
    if observation.outcome != ProcessOutcome::Success || observation.exit_code != Some(0) {
        diagnostics.push(diagnostic(
            PilotBoundary::Execution,
            "stage-failure",
            format!("observations.{:?}.outcome", observation.stage),
            "stage did not finish with a successful exit",
        ));
    }
    if observation.stdout_bytes > profile.resources.output_limit_bytes
        || observation.stderr_bytes > profile.resources.diagnostic_limit_bytes
    {
        diagnostics.push(diagnostic(
            PilotBoundary::Execution,
            "stream-limit",
            format!("observations.{:?}.streams", observation.stage),
            "stage output exceeded a declared stream limit",
        ));
    }
    let wall_limit_nanoseconds = profile
        .benchmark
        .maximum_stage_wall_milliseconds
        .saturating_mul(NANOSECONDS_PER_MILLISECOND);
    if observation.wall_nanoseconds > wall_limit_nanoseconds {
        diagnostics.push(diagnostic(
            PilotBoundary::Benchmark,
            "wall-limit",
            format!("observations.{:?}.wall_nanoseconds", observation.stage),
            "stage duration exceeded the declared wall limit",
        ));
    }
    if observation.outcome == ProcessOutcome::Success && observation.exit_code == Some(0) {
        validate_stage_result(profile, observation, diagnostics);
    }
}

pub(super) fn validate_route_equality(
    profile: &BendHvm2Profile,
    observations: &[StageObservation],
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let measured_results = observations
        .iter()
        .filter(|observation| !observation.warmup)
        .filter_map(|observation| observation.result)
        .collect::<Vec<_>>();
    if measured_results.is_empty()
        || measured_results
            .iter()
            .any(|result| *result != profile.workload.result)
    {
        diagnostics.push(output_diagnostic(
            "backend-disagreement",
            "Trellis and Bend routes do not share one canonical result",
        ));
    }
}

fn validate_stage_result(
    profile: &BendHvm2Profile,
    observation: &StageObservation,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    match observation.stage {
        PilotStage::TrellisOracle => {
            let expected_combines = profile.workload.leaf_limit.saturating_sub(1);
            if observation.result != Some(profile.workload.result)
                || observation.combine_count != Some(expected_combines)
                || observation.interaction_count.is_some()
            {
                diagnostics.push(output_diagnostic(
                    "oracle-result",
                    "Trellis oracle result or abstract combine count differs",
                ));
            }
        }
        PilotStage::BendRunRs | PilotStage::GeneratedC => {
            if observation.result != Some(profile.workload.result)
                || observation.interaction_count == Some(0)
                || observation.interaction_count.is_none()
                || observation.combine_count.is_some()
            {
                diagnostics.push(output_diagnostic(
                    "bend-result",
                    "Bend route result or interaction observation is missing or invalid",
                ));
            }
        }
        PilotStage::BendGenC | PilotStage::CCompile => {
            if observation.result.is_some()
                || observation.interaction_count.is_some()
                || observation.combine_count.is_some()
            {
                diagnostics.push(output_diagnostic(
                    "build-stage-result",
                    "build stage must not claim a runtime result or interaction count",
                ));
            }
        }
    }
}

fn output_diagnostic(code: &str, message: &str) -> BendHvm2Diagnostic {
    diagnostic(PilotBoundary::PostOutput, code, "observations", message)
}
