#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::acronym_style,
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        tigerstyle::path_segment_repetition,
        reason = "These public execution DTO names preserve exact stage and wire-schema vocabulary at the Bend/HVM2 boundary."
    )
)]

use crate::{
    AdapterIdentity, CanonicalValueIdentity, CompilerIdentity, EvidenceChainIdentity,
    ProjectionIdentity, SourceIdentity, TargetArtifactIdentity,
};
use serde::{Deserialize, Serialize};

use super::profile::{ProofValidationObservation, ToolchainObservation};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PilotStage {
    TrellisOracle,
    BendRunRs,
    BendGenC,
    CCompile,
    GeneratedC,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessOutcome {
    Success,
    ExitFailure,
    Timeout,
    ResourceLimit,
    OutputLimit,
    DiagnosticLimit,
    LaunchFailure,
    MalformedOutput,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StageObservation {
    pub stage: PilotStage,
    pub sample_index: usize,
    pub warmup: bool,
    pub command_profile_identity: AdapterIdentity,
    pub outcome: ProcessOutcome,
    pub exit_code: Option<i32>,
    pub result: Option<u64>,
    pub interaction_count: Option<u64>,
    pub combine_count: Option<u64>,
    pub wall_nanoseconds: u64,
    pub peak_memory_bytes: Option<u64>,
    pub stdout_identity: ProjectionIdentity,
    pub stderr_identity: ProjectionIdentity,
    pub stdout_bytes: u64,
    pub stderr_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotArtifactIdentities {
    pub profile: AdapterIdentity,
    pub bend_source: SourceIdentity,
    pub canonical_input: CanonicalValueIdentity,
    pub proof_bundle: SourceIdentity,
    pub proof_validation: EvidenceChainIdentity,
    pub generated_c: TargetArtifactIdentity,
    pub compiler_invocation: CompilerIdentity,
    pub executable: TargetArtifactIdentity,
}

pub struct PilotEvaluationRequest<'a> {
    pub profile_bytes: &'a [u8],
    pub source_bytes: &'a [u8],
    pub canonical_input_bytes: &'a [u8],
    pub bend_license_bytes: &'a [u8],
    pub hvm2_license_bytes: &'a [u8],
    pub proof_bundle_bytes: &'a [u8],
    pub proof_validation: &'a ProofValidationObservation,
    pub toolchain: &'a ToolchainObservation,
    pub observations: &'a [StageObservation],
    pub generated_c_bytes: &'a [u8],
    pub compiler_invocation_bytes: &'a [u8],
    pub executable_bytes: &'a [u8],
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PilotDecision {
    Adopt,
    Defer,
    Reject,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PilotBoundary {
    PreExecution,
    Execution,
    PostOutput,
    Benchmark,
    Receipt,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BendHvm2Diagnostic {
    pub boundary: PilotBoundary,
    pub code: String,
    pub field: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkSummary {
    pub oracle_median_nanoseconds: u64,
    pub run_rs_median_nanoseconds: u64,
    pub generated_c_median_nanoseconds: u64,
    pub oracle_spread_basis_points: u64,
    pub run_rs_spread_basis_points: u64,
    pub generated_c_spread_basis_points: u64,
    pub run_rs_oracle_ratio: u64,
    pub generated_c_oracle_ratio: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotEvaluation {
    pub decision: PilotDecision,
    pub diagnostics: Vec<BendHvm2Diagnostic>,
    pub blockers: Vec<String>,
    pub identities: PilotArtifactIdentities,
    pub benchmark: Option<BenchmarkSummary>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}
