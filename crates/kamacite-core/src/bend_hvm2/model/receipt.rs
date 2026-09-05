#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "These receipt DTOs keep explicit identity domains and product names across public reexports."
    )
)]

use crate::{AnyIdentity, ProjectionIdentity, ReceiptIdentity};
use serde::{Deserialize, Serialize};

use super::execution::{BenchmarkSummary, BendHvm2Diagnostic, PilotDecision, StageObservation};
use crate::{
    AdapterIdentity, CanonicalValueIdentity, CompilerIdentity, EvidenceChainIdentity,
    SourceIdentity, TargetArtifactIdentity,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BendHvm2ValenceClass {
    RecordedOnly,
    Boundary,
    VerifiedTrellisLocal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BendHvm2ValenceArtifact {
    pub role: String,
    pub identity: AnyIdentity,
    pub verification_class: BendHvm2ValenceClass,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BendHvm2PilotReceipt {
    pub body: BendHvm2PilotReceiptBody,
    pub receipt_identity: ReceiptIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BendHvm2PilotReceiptBody {
    pub schema_version: String,
    pub decision: PilotDecision,
    pub blockers: Vec<String>,
    pub diagnostics: Vec<BendHvm2Diagnostic>,
    pub profile_identity: AdapterIdentity,
    pub bend_source_identity: SourceIdentity,
    pub canonical_input_identity: CanonicalValueIdentity,
    pub proof_bundle_identity: SourceIdentity,
    pub proof_validation_identity: EvidenceChainIdentity,
    pub generated_c_identity: TargetArtifactIdentity,
    pub compiler_invocation_identity: CompilerIdentity,
    pub executable_identity: TargetArtifactIdentity,
    pub observations: Vec<StageObservation>,
    pub benchmark: Option<BenchmarkSummary>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BendHvm2ValenceProjection {
    pub schema_version: String,
    pub subject_receipt_identity: ReceiptIdentity,
    pub artifacts: Vec<BendHvm2ValenceArtifact>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
    pub projection_identity: ProjectionIdentity,
}
