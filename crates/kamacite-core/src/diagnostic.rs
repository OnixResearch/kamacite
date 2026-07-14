use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticCode {
    AlgebraicEffectHashMismatch,
    AmbiguousClaim,
    CanonicalContentMismatch,
    CanonicalIrOverclaim,
    DuplicateArtifactIdentity,
    DuplicateFixture,
    GoldenReceiptMismatch,
    HandlerNameNotDischarge,
    HostCapabilityConfusion,
    IdentityDomainMismatch,
    IncompleteEffectSignature,
    IncompleteMetadata,
    LossyProjectionOverclaim,
    ManifestGenerationMismatch,
    MissingCheckedLaw,
    MissingConformanceEntry,
    MissingInput,
    MissingNonClaim,
    MissingReplayLogIdentity,
    MissingSourceBytes,
    MalformedProjectionOutput,
    NonCanonicalInput,
    NonReplayableEvidenceMisuse,
    PathEscape,
    ReceiptIdentityReuse,
    ResourceLimitExceeded,
    SemanticOverclaim,
    SourceHashMismatch,
    SourceSpanFabrication,
    StaleIdentity,
    UndeclaredAuthority,
    UndeclaredCapability,
    UndeclaredEffectOperation,
    UnknownEffectOperation,
    UnsupportedAlgebraicEffectVersion,
    UnsupportedCanonicalCodec,
    UnsupportedArtifactClass,
    UnsupportedClaim,
    UnsupportedEvaluatorIdentity,
    UnsupportedSchemaVersion,
    WrongExpectedVerdict,
    ContinuationPolicyMismatch,
    OpenRowNotPure,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub requirement_id: String,
    pub field: String,
    pub message: String,
    pub details: BTreeMap<String, String>,
}

impl Diagnostic {
    pub fn new(
        code: DiagnosticCode,
        requirement_id: impl Into<String>,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            requirement_id: requirement_id.into(),
            field: field.into(),
            message: message.into(),
            details: BTreeMap::new(),
        }
    }

    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
}
