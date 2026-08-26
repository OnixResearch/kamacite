use std::collections::BTreeSet;

use serde::Serialize;

use crate::identity::{CompatibilityIdentity, SourceIdentity};
use crate::world_commit::model::{
    CONFORMANCE_ENTRY_SCHEMA, MAX_ADAPTERS, MAX_NEGATIVE_VECTORS, MAX_SOURCE_ARTIFACTS,
    MOLTEN_LICENSE, MOLTEN_OWNER, MOLTEN_PRODUCT_SOURCE_MANIFEST_HEX, MOLTEN_REPOSITORY,
    MOLTEN_REVISION, REQUIRED_CHECKED_LAWS, REQUIRED_NON_CLAIMS, WORLD_COMMIT_CODEC,
    WORLD_COMMIT_IDENTITY_DOMAIN, WORLD_COMMIT_SCHEMA, WORLD_COMMIT_VERSION, WorldAdapterContract,
    WorldAdapterKind, WorldCommitCohortEntry, WorldConformanceIssue, WorldSourceArtifact,
    WorldSourceRole,
};

const STEEL_VECTOR_CAPABILITY: &str = "world-vector-resolution";
const NICKEL_SANDBOX_CAPABILITIES: [&str; 3] =
    ["capture-stdout", "fixture-read", "nickel-reference-export"];

const REQUIRED_SOURCE_ROLES: [WorldSourceRole; 13] = [
    WorldSourceRole::MoltenBoundarySchema,
    WorldSourceRole::MoltenCodecSource,
    WorldSourceRole::MoltenCoreModel,
    WorldSourceRole::MoltenRootDomains,
    WorldSourceRole::MoltenIdentity,
    WorldSourceRole::MoltenCanonicalFixture,
    WorldSourceRole::MoltenNotice,
    WorldSourceRole::MoltenProductManifest,
    WorldSourceRole::FixtureManifest,
    WorldSourceRole::CohortConfig,
    WorldSourceRole::KamaciteImplementationManifest,
    WorldSourceRole::NickelContract,
    WorldSourceRole::SteelAdapter,
];

const REQUIRED_NEGATIVE_VECTORS: [&str; 19] = [
    "unknown-version",
    "unknown-schema",
    "unknown-root-domain",
    "duplicate-root-domain",
    "duplicate-root-reference",
    "missing-required-root",
    "wrong-typed-reference",
    "oversized-parents",
    "oversized-roots",
    "hidden-secret-field",
    "altered-source",
    "unapproved-cohort-update",
    "noncanonical-packed",
    "order-drift",
    "codec-drift",
    "embedded-signature",
    "undeclared-steel-authority",
    "json-as-authority",
    "semantic-equivalence-overclaim",
];

#[derive(Serialize)]
struct CohortIdentityMaterial<'a> {
    schema: &'a str,
    product_repository: &'a str,
    product_revision: &'a str,
    product_owner: &'a str,
    product_schema: &'a str,
    canonical_codec: &'a str,
    canonical_domain_tag: &'a str,
    supported_version: &'a str,
    fixture_manifest_identity: &'a SourceIdentity,
    license: &'a str,
    source_artifacts: &'a [WorldSourceArtifact],
    adapter_contracts: &'a [WorldAdapterContract],
    negative_vector_ids: &'a [String],
    checked_laws: &'a [String],
    non_claims: &'a [String],
    semantic_claims: &'a [String],
}

pub fn compute_world_commit_cohort_identity(
    entry: &WorldCommitCohortEntry,
) -> Result<CompatibilityIdentity, WorldConformanceIssue> {
    let material = CohortIdentityMaterial {
        schema: &entry.schema,
        product_repository: &entry.product_repository,
        product_revision: &entry.product_revision,
        product_owner: &entry.product_owner,
        product_schema: &entry.product_schema,
        canonical_codec: &entry.canonical_codec,
        canonical_domain_tag: &entry.canonical_domain_tag,
        supported_version: &entry.supported_version,
        fixture_manifest_identity: &entry.fixture_manifest_identity,
        license: &entry.license,
        source_artifacts: &entry.source_artifacts,
        adapter_contracts: &entry.adapter_contracts,
        negative_vector_ids: &entry.negative_vector_ids,
        checked_laws: &entry.checked_laws,
        non_claims: &entry.non_claims,
        semantic_claims: &entry.semantic_claims,
    };
    serde_json::to_vec(&material)
        .map(|bytes| CompatibilityIdentity::compute(&bytes))
        .map_err(|error| WorldConformanceIssue::CohortField {
            field: "declared-entry-identity".to_string(),
            detail: error.to_string(),
        })
}

// r[impl kamacite.world_commit_conformance.cohort]
// r[impl kamacite.world_commit_conformance.entry]
// r[impl kamacite.world_commit_conformance.sources]
pub fn validate_world_commit_cohort(entry: &WorldCommitCohortEntry) -> Vec<WorldConformanceIssue> {
    let mut issues = Vec::new();
    require_field(
        "schema",
        &entry.schema,
        CONFORMANCE_ENTRY_SCHEMA,
        &mut issues,
    );
    require_field(
        "product-repository",
        &entry.product_repository,
        MOLTEN_REPOSITORY,
        &mut issues,
    );
    require_field(
        "product-revision",
        &entry.product_revision,
        MOLTEN_REVISION,
        &mut issues,
    );
    require_field(
        "product-owner",
        &entry.product_owner,
        MOLTEN_OWNER,
        &mut issues,
    );
    require_field(
        "product-schema",
        &entry.product_schema,
        WORLD_COMMIT_SCHEMA,
        &mut issues,
    );
    require_field(
        "canonical-codec",
        &entry.canonical_codec,
        WORLD_COMMIT_CODEC,
        &mut issues,
    );
    require_field(
        "canonical-domain-tag",
        &entry.canonical_domain_tag,
        WORLD_COMMIT_IDENTITY_DOMAIN,
        &mut issues,
    );
    require_field(
        "supported-version",
        &entry.supported_version,
        WORLD_COMMIT_VERSION,
        &mut issues,
    );
    require_field("license", &entry.license, MOLTEN_LICENSE, &mut issues);
    validate_source_inventory(entry, &mut issues);
    validate_adapter_contracts(entry, &mut issues);
    validate_negative_vectors(entry, &mut issues);
    require_strings(
        &entry.checked_laws,
        &REQUIRED_CHECKED_LAWS,
        "checked-law",
        &mut issues,
    );
    require_strings(
        &entry.non_claims,
        &REQUIRED_NON_CLAIMS,
        "non-claim",
        &mut issues,
    );
    for claim in &entry.semantic_claims {
        issues.push(WorldConformanceIssue::SemanticOverclaim {
            claim: claim.clone(),
        });
    }
    match compute_world_commit_cohort_identity(entry) {
        Ok(computed) if computed.to_wire() != entry.declared_entry_identity => {
            issues.push(WorldConformanceIssue::CohortField {
                field: "declared-entry-identity".to_string(),
                detail: "declared identity does not match the pinned cohort".to_string(),
            });
        }
        Err(issue) => issues.push(issue),
        Ok(_) => {}
    }
    issues.sort();
    issues.dedup();
    issues
}

fn validate_source_inventory(
    entry: &WorldCommitCohortEntry,
    issues: &mut Vec<WorldConformanceIssue>,
) {
    if entry.source_artifacts.len() > MAX_SOURCE_ARTIFACTS {
        issues.push(WorldConformanceIssue::BoundExceeded {
            field: "source-artifacts".to_string(),
            actual: entry.source_artifacts.len(),
            maximum: MAX_SOURCE_ARTIFACTS,
        });
    }
    let mut paths = BTreeSet::new();
    let mut identities = BTreeSet::new();
    for artifact in &entry.source_artifacts {
        if artifact.byte_length == 0 {
            issues.push(WorldConformanceIssue::SourceIdentity {
                role: format!("{:?}", artifact.role),
                detail: "source artifact is empty".to_string(),
            });
        }
        if !paths.insert(&artifact.path) {
            issues.push(WorldConformanceIssue::SourceIdentity {
                role: format!("{:?}", artifact.role),
                detail: "source path is duplicated".to_string(),
            });
        }
        if !identities.insert(artifact.identity.clone()) {
            issues.push(WorldConformanceIssue::SourceIdentity {
                role: format!("{:?}", artifact.role),
                detail: "source identity is duplicated".to_string(),
            });
        }
    }
    for required in REQUIRED_SOURCE_ROLES {
        if !entry
            .source_artifacts
            .iter()
            .any(|artifact| artifact.role == required)
        {
            issues.push(WorldConformanceIssue::SourceIdentity {
                role: format!("{required:?}"),
                detail: "required source role is absent".to_string(),
            });
        }
    }
    if !entry
        .source_artifacts
        .iter()
        .any(|artifact| artifact.identity == entry.fixture_manifest_identity)
    {
        issues.push(WorldConformanceIssue::SourceIdentity {
            role: "FixtureManifest".to_string(),
            detail: "fixture manifest identity is not in the source inventory".to_string(),
        });
    }
    validate_product_source_manifest(entry, issues);
}

fn validate_product_source_manifest(
    entry: &WorldCommitCohortEntry,
    issues: &mut Vec<WorldConformanceIssue>,
) {
    let manifests = entry
        .source_artifacts
        .iter()
        .filter(|artifact| artifact.role == WorldSourceRole::MoltenProductManifest)
        .collect::<Vec<_>>();
    if manifests.len() != 1 {
        issues.push(WorldConformanceIssue::SourceIdentity {
            role: "MoltenProductManifest".to_string(),
            detail: "exactly one immutable product-source manifest is required".to_string(),
        });
        return;
    }
    if manifests[0].identity.hex().as_str() != MOLTEN_PRODUCT_SOURCE_MANIFEST_HEX {
        issues.push(WorldConformanceIssue::SourceIdentity {
            role: "MoltenProductManifest".to_string(),
            detail: "product-source manifest does not match the reviewed Molten revision"
                .to_string(),
        });
    }
}

fn validate_adapter_contracts(
    entry: &WorldCommitCohortEntry,
    issues: &mut Vec<WorldConformanceIssue>,
) {
    if entry.adapter_contracts.len() != MAX_ADAPTERS {
        issues.push(WorldConformanceIssue::CohortField {
            field: "adapter-contracts".to_string(),
            detail: format!("expected {MAX_ADAPTERS} exact adapter contracts"),
        });
    }
    for kind in [
        WorldAdapterKind::Rust,
        WorldAdapterKind::Nickel,
        WorldAdapterKind::Steel,
    ] {
        let matches = entry
            .adapter_contracts
            .iter()
            .filter(|contract| contract.adapter == kind)
            .count();
        match matches {
            0 => issues.push(WorldConformanceIssue::MissingAdapter {
                adapter: format!("{kind:?}"),
            }),
            1 => {}
            _ => issues.push(WorldConformanceIssue::DuplicateAdapter {
                adapter: format!("{kind:?}"),
            }),
        }
    }
    for contract in &entry.adapter_contracts {
        validate_adapter_contract_shape(contract, issues);
        if !entry
            .source_artifacts
            .iter()
            .any(|artifact| artifact.identity == contract.source_identity)
        {
            issues.push(WorldConformanceIssue::AdapterBoundary {
                adapter: format!("{:?}", contract.adapter),
                detail: "adapter source identity is outside the pinned source inventory"
                    .to_string(),
            });
        }
        if !entry
            .source_artifacts
            .iter()
            .any(|artifact| artifact.identity == contract.implementation_identity)
        {
            issues.push(WorldConformanceIssue::AdapterBoundary {
                adapter: format!("{:?}", contract.adapter),
                detail: "adapter implementation identity is outside the pinned source inventory"
                    .to_string(),
            });
        }
    }
}

fn validate_adapter_contract_shape(
    contract: &WorldAdapterContract,
    issues: &mut Vec<WorldConformanceIssue>,
) {
    let invalid = match contract.adapter {
        WorldAdapterKind::Rust => {
            contract.evaluator_identity.is_some()
                || contract.enforcement_identity.is_some()
                || contract.projection_identity.is_some()
                || !contract.allowed_capabilities.is_empty()
        }
        WorldAdapterKind::Nickel => {
            contract.evaluator_identity.is_none()
                || contract.enforcement_identity.is_none()
                || contract.projection_identity.is_none()
                || contract.allowed_capabilities.as_slice() != NICKEL_SANDBOX_CAPABILITIES
        }
        WorldAdapterKind::Steel => {
            contract.evaluator_identity.is_none()
                || contract.enforcement_identity.is_none()
                || contract.projection_identity.is_none()
                || contract.allowed_capabilities.as_slice() != [STEEL_VECTOR_CAPABILITY]
        }
    };
    if invalid {
        issues.push(WorldConformanceIssue::AdapterBoundary {
            adapter: format!("{:?}", contract.adapter),
            detail: "adapter contract does not match its closed authority boundary".to_string(),
        });
    }
}

fn validate_negative_vectors(
    entry: &WorldCommitCohortEntry,
    issues: &mut Vec<WorldConformanceIssue>,
) {
    if entry.negative_vector_ids.len() > MAX_NEGATIVE_VECTORS {
        issues.push(WorldConformanceIssue::BoundExceeded {
            field: "negative-vector-ids".to_string(),
            actual: entry.negative_vector_ids.len(),
            maximum: MAX_NEGATIVE_VECTORS,
        });
    }
    require_strings(
        &entry.negative_vector_ids,
        &REQUIRED_NEGATIVE_VECTORS,
        "negative-vector",
        issues,
    );
}

fn require_field(
    field: &str,
    actual: &str,
    expected: &str,
    issues: &mut Vec<WorldConformanceIssue>,
) {
    if actual != expected {
        issues.push(WorldConformanceIssue::CohortField {
            field: field.to_string(),
            detail: format!("expected {expected}, found {actual}"),
        });
    }
}

pub(super) fn require_strings(
    actual: &[String],
    required: &[&str],
    class: &str,
    issues: &mut Vec<WorldConformanceIssue>,
) {
    let mut unique = BTreeSet::new();
    for value in actual {
        if !unique.insert(value) {
            issues.push(WorldConformanceIssue::CohortField {
                field: class.to_string(),
                detail: format!("duplicate value {value}"),
            });
        }
    }
    for value in required {
        if !actual.iter().any(|actual| actual == value) {
            let issue = if class.contains("non-claim") {
                WorldConformanceIssue::MissingNonClaim {
                    non_claim: (*value).to_string(),
                }
            } else {
                WorldConformanceIssue::MissingCheckedLaw {
                    law: format!("{class}:{value}"),
                }
            };
            issues.push(issue);
        }
    }
}

pub fn source_identity(bytes: &[u8]) -> SourceIdentity {
    SourceIdentity::compute(bytes)
}
