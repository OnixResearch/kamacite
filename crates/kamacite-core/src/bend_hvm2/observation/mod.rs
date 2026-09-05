#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::assertion_density,
        tigerstyle::fragile_exhaustive_enum_match,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "This pure parser and diagnostic accumulator returns typed failures instead of panicking and matches every closed pilot stage deliberately."
    )
)]

mod benchmark;
mod validation;

use super::model::{
    BenchmarkSummary, BendHvm2Diagnostic, BendHvm2Profile, PilotBoundary, PilotStage,
    StageObservation,
};
use super::profile::diagnostic;
use crate::{AdapterIdentity, canonical_json_bytes};
use benchmark::{summarize_benchmark, validate_benchmark};
use validation::{validate_observation, validate_route_equality, validate_stage_counts};

const ORACLE_COMMAND_PROFILE: &str = "trellis-oracle-v1";
const RUN_RS_COMMAND_PROFILE: &str = "bend-run-rs-v1";
const GEN_C_COMMAND_PROFILE: &str = "bend-gen-c-v1";
const C_COMPILE_COMMAND_PROFILE: &str = "gcc-generated-c-v1";
const GENERATED_C_COMMAND_PROFILE: &str = "generated-c-run-v1";
const RESULT_PREFIX: &str = "Result: ";
const INTERACTION_PREFIX: &str = "- ITRS: ";
const COMBINE_PREFIX: &str = "- COMBINES: ";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParsedRouteOutput {
    pub result: u64,
    pub interaction_count: Option<u64>,
    pub combine_count: Option<u64>,
}

pub fn command_profile_identity(
    profile: &BendHvm2Profile,
    stage: PilotStage,
) -> Result<AdapterIdentity, BendHvm2Diagnostic> {
    let id = command_profile_id(stage);
    let command = profile
        .cohort
        .command_profiles
        .iter()
        .find(|command| command.id == id)
        .ok_or_else(|| {
            diagnostic(
                PilotBoundary::PreExecution,
                "command-profile-missing",
                "cohort.command_profiles",
                format!("required command profile is absent: {id}"),
            )
        })?;
    canonical_json_bytes(command)
        .map(|bytes| AdapterIdentity::compute(&bytes))
        .map_err(|error| {
            diagnostic(
                PilotBoundary::PreExecution,
                "command-profile-encoding",
                "cohort.command_profiles",
                error.to_string(),
            )
        })
}

pub fn parse_route_output(
    stage: PilotStage,
    bytes: &[u8],
) -> Result<ParsedRouteOutput, BendHvm2Diagnostic> {
    let text = std::str::from_utf8(bytes).map_err(|error| {
        diagnostic(
            PilotBoundary::PostOutput,
            "output-encoding",
            "stage.stdout",
            format!("route output is not UTF-8: {error}"),
        )
    })?;
    let result = parse_prefixed_u64(text, RESULT_PREFIX).ok_or_else(|| {
        diagnostic(
            PilotBoundary::PostOutput,
            "missing-result",
            "stage.stdout",
            "route output lacks one canonical result line",
        )
    })?;
    let interaction_count = parse_prefixed_u64(text, INTERACTION_PREFIX);
    let combine_count = parse_prefixed_u64(text, COMBINE_PREFIX);
    match stage {
        PilotStage::TrellisOracle if combine_count.is_none() || interaction_count.is_some() => {
            return Err(diagnostic(
                PilotBoundary::PostOutput,
                "oracle-output-shape",
                "stage.stdout",
                "Trellis oracle output must contain combines and no HVM interaction count",
            ));
        }
        PilotStage::BendRunRs | PilotStage::GeneratedC
            if interaction_count.is_none() || combine_count.is_some() =>
        {
            return Err(diagnostic(
                PilotBoundary::PostOutput,
                "bend-output-shape",
                "stage.stdout",
                "Bend route output must contain an HVM interaction count and no oracle combine count",
            ));
        }
        PilotStage::BendGenC | PilotStage::CCompile => {
            return Err(diagnostic(
                PilotBoundary::PostOutput,
                "non-result-stage",
                "stage",
                "generated-source and compiler stages do not have route results",
            ));
        }
        PilotStage::TrellisOracle | PilotStage::BendRunRs | PilotStage::GeneratedC => {}
    }
    Ok(ParsedRouteOutput {
        result,
        interaction_count,
        combine_count,
    })
}

pub fn evaluate_observations(
    profile: &BendHvm2Profile,
    observations: &[StageObservation],
) -> (Option<BenchmarkSummary>, Vec<BendHvm2Diagnostic>) {
    let mut diagnostics = Vec::new();
    validate_stage_counts(profile, observations, &mut diagnostics);
    for observation in observations {
        validate_observation(profile, observation, &mut diagnostics);
    }
    validate_route_equality(profile, observations, &mut diagnostics);
    let has_incomplete_execution = diagnostics.iter().any(|item| {
        matches!(
            item.boundary,
            PilotBoundary::Execution | PilotBoundary::PostOutput
        )
    });
    let benchmark = if has_incomplete_execution {
        None
    } else {
        summarize_benchmark(observations, &mut diagnostics)
    };
    if let Some(summary) = &benchmark {
        validate_benchmark(profile, summary, &mut diagnostics);
    }
    diagnostics.sort();
    diagnostics.dedup();
    (benchmark, diagnostics)
}

pub fn validate_observations(
    profile: &BendHvm2Profile,
    observations: &[StageObservation],
) -> Result<BenchmarkSummary, Vec<BendHvm2Diagnostic>> {
    let (benchmark, diagnostics) = evaluate_observations(profile, observations);
    match (diagnostics.is_empty(), benchmark) {
        (true, Some(summary)) => Ok(summary),
        _ => Err(diagnostics),
    }
}

fn parse_prefixed_u64(text: &str, prefix: &str) -> Option<u64> {
    let mut values = text
        .lines()
        .filter_map(|line| line.strip_prefix(prefix))
        .filter_map(|value| value.trim().parse::<u64>().ok());
    let first = values.next()?;
    if values.next().is_some() {
        None
    } else {
        Some(first)
    }
}

fn command_profile_id(stage: PilotStage) -> &'static str {
    match stage {
        PilotStage::TrellisOracle => ORACLE_COMMAND_PROFILE,
        PilotStage::BendRunRs => RUN_RS_COMMAND_PROFILE,
        PilotStage::BendGenC => GEN_C_COMMAND_PROFILE,
        PilotStage::CCompile => C_COMPILE_COMMAND_PROFILE,
        PilotStage::GeneratedC => GENERATED_C_COMMAND_PROFILE,
    }
}

#[cfg(test)]
pub(crate) const EXPECTED_ROUTE_OBSERVATION_COUNT: usize = 6;
