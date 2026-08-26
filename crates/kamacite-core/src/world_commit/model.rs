use serde::{Deserialize, Serialize};

use crate::identity::{
    AdapterIdentity, CanonicalValueIdentity, CompatibilityIdentity, EvaluatorIdentity,
    ProjectionIdentity, ReceiptIdentity, SourceIdentity, WireIdentity,
};

pub const CONFORMANCE_ENTRY_SCHEMA: &str = "kamacite.world-commit-conformance-entry.v1";
pub const CONFORMANCE_RECEIPT_SCHEMA: &str = "kamacite.world-commit-conformance-receipt.v1";
pub const VALENCE_PROJECTION_SCHEMA: &str = "kamacite.world-commit-valence-projection.v1";
pub const WORLD_COMMIT_RECORD_LABEL: &str = "molten-world-commit-v1";
pub const WORLD_COMMIT_SCHEMA: &str = "molten.world-commit.v1";
pub const WORLD_COMMIT_VERSION: &str = "v1";
pub const WORLD_COMMIT_CODEC: &str = "molten-preserves-packed-canonical-v1-preserves-rs-5.0.0-rc.7";
pub const WORLD_COMMIT_IDENTITY_DOMAIN: &str = "onixresearch.molten.world-commit.identity.v1";
pub const WORLD_COMMIT_FRAME_VERSION: &[u8] = b"molten-world-commit-frame-v1";
pub const MOLTEN_REPOSITORY: &str = "git@github.com:brittonr/aspen.git";
pub const MOLTEN_REVISION: &str = "2f5d7bf9017b3b00df36e55cf0bbef6d68187611";
pub const MOLTEN_OWNER: &str = "Molten";
pub const MOLTEN_LICENSE: &str = "AGPL-3.0-or-later";
pub const MOLTEN_PRODUCT_SOURCE_MANIFEST_HEX: &str =
    "dfdce7f71c0c0deb59ae99a80423abab42840a68efea023bf1348261a0e556aa";
pub const MAX_WORLD_COMMIT_PARENTS: usize = 32;
pub const MAX_WORLD_COMMIT_ROOTS: usize = 16;
pub const MAX_WORLD_COMMIT_BYTES: usize = 1_048_576;
pub const MAX_SOURCE_ARTIFACTS: usize = 32;
pub const MAX_NEGATIVE_VECTORS: usize = 64;
pub const MAX_ADAPTERS: usize = 3;

pub const REQUIRED_CHECKED_LAWS: [&str; 7] = [
    "canonical-packed-bytes-match",
    "json-remains-a-projection",
    "root-domains-are-nominal",
    "source-identities-remain-detached",
    "steel-has-no-ambient-authority",
    "unknown-versions-fail-closed",
    "world-runtime-authority-remains-with-molten",
];

pub const REQUIRED_NON_CLAIMS: [&str; 7] = [
    "not evaluator semantic equivalence",
    "not Molten runtime behavior",
    "not restore or merge compatibility",
    "not branch or effect authority",
    "not release eligibility",
    "not a generic stack-wide world protocol",
    "not proof of referenced object type or contents",
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorldRootDomain {
    Artifact,
    Schema,
    DurableState,
    Tasks,
    History,
    Effects,
    Scheduler,
    Time,
    Entropy,
    RuntimeProfile,
    Policy,
    AuthorityObservation,
    OpaqueMachineSnapshot,
}

impl WorldRootDomain {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Artifact => "artifact",
            Self::Schema => "schema",
            Self::DurableState => "durable-state",
            Self::Tasks => "tasks",
            Self::History => "history",
            Self::Effects => "effects",
            Self::Scheduler => "scheduler",
            Self::Time => "time",
            Self::Entropy => "entropy",
            Self::RuntimeProfile => "runtime-profile",
            Self::Policy => "policy",
            Self::AuthorityObservation => "authority-observation",
            Self::OpaqueMachineSnapshot => "opaque-machine-snapshot",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        ROOT_DOMAIN_INVENTORY
            .iter()
            .copied()
            .find(|domain| domain.as_str() == value)
    }
}

pub const ROOT_DOMAIN_INVENTORY: [WorldRootDomain; 13] = [
    WorldRootDomain::Artifact,
    WorldRootDomain::Schema,
    WorldRootDomain::DurableState,
    WorldRootDomain::Tasks,
    WorldRootDomain::History,
    WorldRootDomain::Effects,
    WorldRootDomain::Scheduler,
    WorldRootDomain::Time,
    WorldRootDomain::Entropy,
    WorldRootDomain::RuntimeProfile,
    WorldRootDomain::Policy,
    WorldRootDomain::AuthorityObservation,
    WorldRootDomain::OpaqueMachineSnapshot,
];

pub const LOGICAL_REQUIRED_ROOTS: [WorldRootDomain; 11] = [
    WorldRootDomain::Artifact,
    WorldRootDomain::Schema,
    WorldRootDomain::DurableState,
    WorldRootDomain::Tasks,
    WorldRootDomain::History,
    WorldRootDomain::Effects,
    WorldRootDomain::Scheduler,
    WorldRootDomain::Time,
    WorldRootDomain::Entropy,
    WorldRootDomain::RuntimeProfile,
    WorldRootDomain::Policy,
];

pub const OPAQUE_REQUIRED_ROOTS: [WorldRootDomain; 5] = [
    WorldRootDomain::Artifact,
    WorldRootDomain::Schema,
    WorldRootDomain::RuntimeProfile,
    WorldRootDomain::Policy,
    WorldRootDomain::OpaqueMachineSnapshot,
];

pub const MIXED_REQUIRED_ROOTS: [WorldRootDomain; 12] = [
    WorldRootDomain::Artifact,
    WorldRootDomain::Schema,
    WorldRootDomain::DurableState,
    WorldRootDomain::Tasks,
    WorldRootDomain::History,
    WorldRootDomain::Effects,
    WorldRootDomain::Scheduler,
    WorldRootDomain::Time,
    WorldRootDomain::Entropy,
    WorldRootDomain::RuntimeProfile,
    WorldRootDomain::Policy,
    WorldRootDomain::OpaqueMachineSnapshot,
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorldProfileKind {
    Logical,
    Opaque,
    Mixed,
}

impl WorldProfileKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Logical => "logical",
            Self::Opaque => "opaque",
            Self::Mixed => "mixed",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "logical" => Some(Self::Logical),
            "opaque" => Some(Self::Opaque),
            "mixed" => Some(Self::Mixed),
            _ => None,
        }
    }

    pub const fn required_roots(self) -> &'static [WorldRootDomain] {
        match self {
            Self::Logical => &LOGICAL_REQUIRED_ROOTS,
            Self::Opaque => &OPAQUE_REQUIRED_ROOTS,
            Self::Mixed => &MIXED_REQUIRED_ROOTS,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorldRoot {
    pub domain: WorldRootDomain,
    pub reference: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldProfile {
    pub kind: WorldProfileKind,
    pub profile_ref: String,
    pub cohort_ref: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldCommitValue {
    pub schema: String,
    pub version: String,
    pub profile: WorldProfile,
    pub parents: Vec<String>,
    pub roots: Vec<WorldRoot>,
    pub completeness: Vec<WorldRootDomain>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedWorldCommit {
    pub value: WorldCommitValue,
    pub packed_bytes: Vec<u8>,
    pub value_identity: CanonicalValueIdentity,
    pub molten_world_commit_ref: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorldSourceRole {
    MoltenBoundarySchema,
    MoltenCodecSource,
    MoltenCoreModel,
    MoltenRootDomains,
    MoltenIdentity,
    MoltenCanonicalFixture,
    MoltenNotice,
    MoltenProductManifest,
    FixtureManifest,
    CohortConfig,
    KamaciteImplementationManifest,
    NickelContract,
    SteelAdapter,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorldSourceArtifact {
    pub role: WorldSourceRole,
    pub path: String,
    pub identity: SourceIdentity,
    pub byte_length: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldAdapterContract {
    pub adapter: WorldAdapterKind,
    pub source_identity: SourceIdentity,
    pub implementation_identity: SourceIdentity,
    pub adapter_identity: AdapterIdentity,
    pub evaluator_identity: Option<EvaluatorIdentity>,
    pub enforcement_identity: Option<AdapterIdentity>,
    pub projection_identity: Option<ProjectionIdentity>,
    pub allowed_capabilities: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldCommitCohortEntry {
    pub schema: String,
    pub product_repository: String,
    pub product_revision: String,
    pub product_owner: String,
    pub product_schema: String,
    pub canonical_codec: String,
    pub canonical_domain_tag: String,
    pub supported_version: String,
    pub fixture_manifest_identity: SourceIdentity,
    pub license: String,
    pub source_artifacts: Vec<WorldSourceArtifact>,
    pub adapter_contracts: Vec<WorldAdapterContract>,
    pub negative_vector_ids: Vec<String>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
    pub semantic_claims: Vec<String>,
    pub declared_entry_identity: WireIdentity,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorldAdapterKind {
    Rust,
    Nickel,
    Steel,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldAdapterObservation {
    pub adapter: WorldAdapterKind,
    pub source_identity: SourceIdentity,
    pub implementation_identity: SourceIdentity,
    pub adapter_identity: AdapterIdentity,
    pub evaluator_identity: Option<EvaluatorIdentity>,
    pub enforcement_identity: Option<AdapterIdentity>,
    pub projection_identity: Option<ProjectionIdentity>,
    pub resolved_value_identity: CanonicalValueIdentity,
    pub resolved_packed_bytes: Vec<u8>,
    pub fresh_execution: bool,
    pub declared_capabilities: Vec<String>,
    pub observed_effects: Vec<String>,
    pub checked_laws: Vec<String>,
    pub claims_json_canonical: bool,
    pub semantic_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldParityInput {
    pub cohort: WorldCommitCohortEntry,
    pub authoritative_packed_bytes: Vec<u8>,
    pub adapters: Vec<WorldAdapterObservation>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorldAdapterSummary {
    pub adapter: WorldAdapterKind,
    pub adapter_identity: AdapterIdentity,
    pub source_identity: SourceIdentity,
    pub implementation_identity: SourceIdentity,
    pub evaluator_identity: Option<EvaluatorIdentity>,
    pub enforcement_identity: Option<AdapterIdentity>,
    pub projection_identity: Option<ProjectionIdentity>,
    pub exact_value_match: bool,
    pub exact_byte_match: bool,
    pub fresh_execution: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-world-commit-conformance-receipt-v1")]
pub struct WorldParityReceiptBody {
    pub schema: String,
    pub cohort_identity: CompatibilityIdentity,
    pub canonical_value_identity: CanonicalValueIdentity,
    pub molten_world_commit_ref: String,
    pub adapters: Vec<WorldAdapterSummary>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
    pub verdict: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldParityReceipt {
    pub body: WorldParityReceiptBody,
    pub canonical_bytes: Vec<u8>,
    pub receipt_identity: ReceiptIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldCommitValenceProjection {
    pub schema: String,
    pub role: String,
    pub receipt_identity: ReceiptIdentity,
    pub cohort_identity: CompatibilityIdentity,
    pub canonical_value_identity: CanonicalValueIdentity,
    pub adapter_identities: Vec<AdapterIdentity>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WorldConformanceIssue {
    CohortField {
        field: String,
        detail: String,
    },
    SourceIdentity {
        role: String,
        detail: String,
    },
    MalformedCanonical {
        detail: String,
    },
    UnsupportedVersion {
        actual: String,
    },
    UnsupportedProfile {
        actual: String,
    },
    InvalidReference {
        field: String,
    },
    BoundExceeded {
        field: String,
        actual: usize,
        maximum: usize,
    },
    DuplicateParent {
        reference: String,
    },
    DuplicateRootDomain {
        domain: String,
    },
    DuplicateRootReference {
        reference: String,
    },
    RootDomainSubstitution {
        field: String,
        expected: String,
        actual: String,
    },
    MissingRequiredRoot {
        domain: String,
    },
    UnexpectedRoot {
        domain: String,
    },
    CompletenessMismatch,
    CanonicalOrder {
        field: String,
    },
    MissingAdapter {
        adapter: String,
    },
    DuplicateAdapter {
        adapter: String,
    },
    AdapterBoundary {
        adapter: String,
        detail: String,
    },
    ValueMismatch {
        adapter: String,
    },
    ByteMismatch {
        adapter: String,
    },
    MissingCheckedLaw {
        law: String,
    },
    MissingNonClaim {
        non_claim: String,
    },
    SemanticOverclaim {
        claim: String,
    },
    ReceiptEncoding {
        detail: String,
    },
}

impl WorldConformanceIssue {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::CohortField { .. } => "world-cohort-field",
            Self::SourceIdentity { .. } => "world-source-identity",
            Self::MalformedCanonical { .. } => "world-malformed-canonical",
            Self::UnsupportedVersion { .. } => "world-unsupported-version",
            Self::UnsupportedProfile { .. } => "world-unsupported-profile",
            Self::InvalidReference { .. } => "world-invalid-reference",
            Self::BoundExceeded { .. } => "world-bound-exceeded",
            Self::DuplicateParent { .. } => "world-duplicate-parent",
            Self::DuplicateRootDomain { .. } => "world-duplicate-root-domain",
            Self::DuplicateRootReference { .. } => "world-duplicate-root-reference",
            Self::RootDomainSubstitution { .. } => "world-root-domain-substitution",
            Self::MissingRequiredRoot { .. } => "world-missing-required-root",
            Self::UnexpectedRoot { .. } => "world-unexpected-root",
            Self::CompletenessMismatch => "world-completeness-mismatch",
            Self::CanonicalOrder { .. } => "world-canonical-order",
            Self::MissingAdapter { .. } => "world-missing-adapter",
            Self::DuplicateAdapter { .. } => "world-duplicate-adapter",
            Self::AdapterBoundary { .. } => "world-adapter-boundary",
            Self::ValueMismatch { .. } => "world-value-mismatch",
            Self::ByteMismatch { .. } => "world-byte-mismatch",
            Self::MissingCheckedLaw { .. } => "world-missing-checked-law",
            Self::MissingNonClaim { .. } => "world-missing-non-claim",
            Self::SemanticOverclaim { .. } => "world-semantic-overclaim",
            Self::ReceiptEncoding { .. } => "world-receipt-encoding",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldCohortUpdateRequest {
    pub current_revision: String,
    pub proposed_revision: String,
    pub current_schema_identity: SourceIdentity,
    pub proposed_schema_identity: SourceIdentity,
    pub current_codec_identity: SourceIdentity,
    pub proposed_codec_identity: SourceIdentity,
    pub explicit_regeneration: bool,
    pub convergence_passes: u32,
    pub removed_negative_vectors: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldCohortUpdatePlan {
    pub changed_fields: Vec<String>,
    pub requires_new_cohort: bool,
    pub convergence_required: bool,
    pub review_removed_negative_vectors: Vec<String>,
    pub admitted: bool,
}
