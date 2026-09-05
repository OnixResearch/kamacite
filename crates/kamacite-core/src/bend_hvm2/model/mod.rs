#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "This model facade reexports explicit boundary-owned DTOs and constants for external consumers."
    )
)]

mod execution;
mod profile;
mod receipt;

pub use execution::{
    BenchmarkSummary, BendHvm2Diagnostic, PilotArtifactIdentities, PilotBoundary, PilotDecision,
    PilotEvaluation, PilotEvaluationRequest, PilotStage, ProcessOutcome, StageObservation,
};
pub use profile::{
    BenchmarkPolicy, BendHvm2Profile, CohortProfile, CommandProfile, CompilerProfile,
    ProofValidationObservation, ResourceProfile, SourcePin, ToolchainObservation, TrellisProfile,
    WorkloadProfile,
};
pub use receipt::{
    BendHvm2PilotReceipt, BendHvm2PilotReceiptBody, BendHvm2ValenceArtifact, BendHvm2ValenceClass,
    BendHvm2ValenceProjection,
};

pub const BEND_HVM2_PROFILE_SCHEMA: &str = "kamacite.bend-hvm2-evaluation-profile.v1";
pub const BEND_HVM2_PROFILE_ID: &str = "kamacite-bend-hvm2-cpu-pilot-v1";
pub const BEND_VERSION: &str = "0.2.38";
pub const BEND_REVISION: &str = "814453670d0e0d6777c1313c972764dba0491b7f";
pub const HVM2_VERSION: &str = "2.0.22";
pub const HVM2_REVISION: &str = "654276018084b8f44a22b562dd68ab18583bfb5b";
pub const TRELLIS_REVISION: &str = "f26e3cd465101698ade12f8d142e0dc95578326b";
pub const TRELLIS_VERIFIER_PROVIDER_REVISION: &str = "374bd16b26cee2af34211a29bfa531c016811f51";
pub const TRELLIS_SCOPE: &str = "module:bounded_modular_tree_fold";
pub const TRELLIS_ALGORITHM_PROFILE: &str = "trellis.balanced-u24-wrapping-sum.v1";
pub const TRELLIS_ORACLE_SYMBOL: &str =
    "verified_logic::bounded_modular_tree_fold::sequential_u24_sum";
pub const BEND_LICENSE_BLAKE3: &str =
    "7e16ff676b2bb64fea3d601f332aaefb4ae8c8123271a01e8ebf52c7bf5592c2";
pub const HVM2_LICENSE_BLAKE3: &str =
    "4e2804b387ec9c0ed5b18f26bdbb741e14c2dc166d08a6ecb90cce8d0ebddbaa";
pub const TRELLIS_BUNDLE_BLAKE3: &str =
    "e58a90561093f7ae28adfe8eb56c16a618c4e99e056af8822633609d5201a70e";
pub const U24_MODULUS: u64 = 16_777_216;
pub const U24_MAXIMUM: u64 = 16_777_215;
pub const PILOT_LEAF_LIMIT: u64 = 4;
pub const PILOT_RESULT: u64 = 1;
pub const PILOT_MEASURED_RUNS: usize = 5;
pub const PILOT_WARMUP_RUNS: usize = 1;
pub const BASIS_POINTS_PER_UNIT: u128 = 10_000;
pub const BEND_HVM2_RECEIPT_SCHEMA: &str = "kamacite.bend-hvm2-pilot-receipt.v1";
pub const BEND_HVM2_VALENCE_SCHEMA: &str = "kamacite.bend-hvm2-valence-projection.v1";
pub const BEND_HVM2_RECEIPT_ROOT: &str = "BendHvm2PilotReceiptBody";
