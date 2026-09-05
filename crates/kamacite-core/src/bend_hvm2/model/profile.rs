#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        tigerstyle::path_segment_repetition,
        reason = "These typed profile DTOs mirror the reviewed Nickel field names and remain clear when reexported."
    )
)]

use crate::WireIdentity;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePin {
    pub repository: String,
    pub revision: String,
    pub version: String,
    pub nix_nar_sha256: String,
    pub license: String,
    pub license_path: String,
    pub license_blake3: String,
    pub attribution: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommandProfile {
    pub id: String,
    pub executable: String,
    pub arguments: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompilerProfile {
    pub family: String,
    pub version: String,
    pub target: String,
    pub flags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CohortProfile {
    pub bend: SourcePin,
    pub hvm2: SourcePin,
    pub bend_hvm_dependency: String,
    pub compiler: CompilerProfile,
    pub command_profiles: Vec<CommandProfile>,
    pub excluded_toolchains: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkloadProfile {
    pub id: String,
    pub algorithm_profile: String,
    pub leaves: Vec<u64>,
    pub leaf_limit: u64,
    pub numeric_type: String,
    pub modulus: u64,
    pub result: u64,
    pub source_path: String,
    pub canonical_input_path: String,
    pub required_annotations: Vec<String>,
    pub forbidden_source_tokens: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceProfile {
    pub timeout_milliseconds: u64,
    pub cpu_seconds: u64,
    pub memory_limit_bytes: u64,
    pub output_limit_bytes: u64,
    pub diagnostic_limit_bytes: u64,
    pub artifact_limit_bytes: u64,
    pub file_limit_bytes: u64,
    pub process_limit: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkPolicy {
    pub warmup_runs: usize,
    pub measured_runs: usize,
    pub statistic: String,
    pub maximum_spread_basis_points: u64,
    pub maximum_generated_c_oracle_ratio: u64,
    pub maximum_run_rs_oracle_ratio: u64,
    pub maximum_stage_wall_milliseconds: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrellisProfile {
    pub repository: String,
    pub revision: String,
    pub nix_nar_sha256: String,
    pub proof_bundle_path: String,
    pub proof_bundle_blake3: String,
    pub scope: String,
    pub algorithm_profile: String,
    pub executable_oracle_symbol: String,
    pub proof_command: String,
    pub verifier_toolchain: String,
    pub verifier_provider_revision: String,
    pub proof_check: String,
    pub required_requirement_ids: Vec<String>,
    pub required_proof_functions: Vec<String>,
    pub required_non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BendHvm2Profile {
    pub schema_version: String,
    pub profile_id: String,
    pub disposition: String,
    pub canonical_ir: String,
    pub cohort: CohortProfile,
    pub workload: WorkloadProfile,
    pub resources: ResourceProfile,
    pub benchmark: BenchmarkPolicy,
    pub trellis: TrellisProfile,
    pub denied_authority: Vec<String>,
    pub required_parent_roles: Vec<String>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolchainObservation {
    pub bend_version: String,
    pub hvm2_version: String,
    pub compiler_family: String,
    pub compiler_version: String,
    pub compiler_target: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProofValidationObservation {
    pub trellis_revision: String,
    pub verifier_provider_revision: String,
    pub proof_check: String,
    pub proof_result_present: bool,
    pub proof_result_identity: WireIdentity,
    pub verifier_identity: WireIdentity,
    pub passed: bool,
    pub diagnostics: Vec<String>,
}
