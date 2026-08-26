use crate::identity::{
    AdapterIdentity, Blake3Hex, CompatibilityIdentity, EvaluatorIdentity, ProjectionIdentity,
    SourceIdentity,
};
use crate::world_commit::{
    CONFORMANCE_ENTRY_SCHEMA, MOLTEN_LICENSE, MOLTEN_OWNER, MOLTEN_PRODUCT_SOURCE_MANIFEST_HEX,
    MOLTEN_REPOSITORY, MOLTEN_REVISION, REQUIRED_CHECKED_LAWS, REQUIRED_NON_CLAIMS,
    WORLD_COMMIT_CODEC, WORLD_COMMIT_IDENTITY_DOMAIN, WORLD_COMMIT_SCHEMA, WORLD_COMMIT_VERSION,
    WorldAdapterContract, WorldAdapterKind, WorldAdapterObservation, WorldCommitCohortEntry,
    WorldParityInput, WorldSourceArtifact, WorldSourceRole, canonicalize_world_commit_text,
    compute_world_commit_cohort_identity,
};

pub const LOGICAL_TEXT: &[u8] =
    include_bytes!("../../../../../fixtures/world-commit/source/logical-v1.preserves");

const NEGATIVE_VECTOR_IDS: [&str; 19] = [
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

const SOURCE_ROLES: [WorldSourceRole; 13] = [
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

pub fn parity_input() -> Result<WorldParityInput, String> {
    let admitted = canonicalize_world_commit_text(LOGICAL_TEXT)
        .map_err(|issues| format!("logical fixture denied: {issues:?}"))?;
    let cohort = cohort_entry()?;
    let adapters = cohort
        .adapter_contracts
        .iter()
        .map(|contract| WorldAdapterObservation {
            adapter: contract.adapter,
            source_identity: contract.source_identity.clone(),
            implementation_identity: contract.implementation_identity.clone(),
            adapter_identity: contract.adapter_identity.clone(),
            evaluator_identity: contract.evaluator_identity.clone(),
            enforcement_identity: contract.enforcement_identity.clone(),
            projection_identity: contract.projection_identity.clone(),
            resolved_value_identity: admitted.value_identity.clone(),
            resolved_packed_bytes: admitted.packed_bytes.clone(),
            fresh_execution: true,
            declared_capabilities: contract.allowed_capabilities.clone(),
            observed_effects: if contract.adapter == WorldAdapterKind::Nickel {
                contract.allowed_capabilities.clone()
            } else {
                Vec::new()
            },
            checked_laws: REQUIRED_CHECKED_LAWS
                .iter()
                .map(ToString::to_string)
                .collect(),
            claims_json_canonical: false,
            semantic_claims: Vec::new(),
        })
        .collect();
    Ok(WorldParityInput {
        cohort,
        authoritative_packed_bytes: admitted.packed_bytes,
        adapters,
    })
}

pub fn cohort_entry() -> Result<WorldCommitCohortEntry, String> {
    let source_artifacts = SOURCE_ROLES
        .iter()
        .copied()
        .map(source_artifact)
        .collect::<Result<Vec<_>, _>>()?;
    let source_for = |role| {
        source_artifacts
            .iter()
            .find(|artifact| artifact.role == role)
            .map(|artifact| artifact.identity.clone())
            .ok_or_else(|| format!("source role is absent: {role:?}"))
    };
    let implementation_identity = source_for(WorldSourceRole::KamaciteImplementationManifest)?;
    let adapter_contracts = vec![
        WorldAdapterContract {
            adapter: WorldAdapterKind::Rust,
            source_identity: source_for(WorldSourceRole::MoltenCanonicalFixture)?,
            implementation_identity: implementation_identity.clone(),
            adapter_identity: AdapterIdentity::compute(b"world-rust-adapter-v1"),
            evaluator_identity: None,
            enforcement_identity: None,
            projection_identity: None,
            allowed_capabilities: Vec::new(),
        },
        WorldAdapterContract {
            adapter: WorldAdapterKind::Nickel,
            source_identity: source_for(WorldSourceRole::NickelContract)?,
            implementation_identity: implementation_identity.clone(),
            adapter_identity: AdapterIdentity::compute(b"world-nickel-adapter-v1"),
            evaluator_identity: Some(EvaluatorIdentity::compute(b"nickel-evaluator-v1")),
            enforcement_identity: Some(AdapterIdentity::compute(b"nickel-enforcement-v1")),
            projection_identity: Some(ProjectionIdentity::compute(b"nickel-projection-v1")),
            allowed_capabilities: vec![
                "capture-stdout".to_string(),
                "fixture-read".to_string(),
                "nickel-reference-export".to_string(),
            ],
        },
        WorldAdapterContract {
            adapter: WorldAdapterKind::Steel,
            source_identity: source_for(WorldSourceRole::SteelAdapter)?,
            implementation_identity,
            adapter_identity: AdapterIdentity::compute(b"world-steel-adapter-v1"),
            evaluator_identity: Some(EvaluatorIdentity::compute(b"steel-evaluator-v1")),
            enforcement_identity: Some(AdapterIdentity::compute(b"steel-enforcement-v1")),
            projection_identity: Some(ProjectionIdentity::compute(b"steel-projection-v1")),
            allowed_capabilities: vec!["world-vector-resolution".to_string()],
        },
    ];
    let fixture_manifest_identity = source_for(WorldSourceRole::FixtureManifest)?;
    let mut entry = WorldCommitCohortEntry {
        schema: CONFORMANCE_ENTRY_SCHEMA.to_string(),
        product_repository: MOLTEN_REPOSITORY.to_string(),
        product_revision: MOLTEN_REVISION.to_string(),
        product_owner: MOLTEN_OWNER.to_string(),
        product_schema: WORLD_COMMIT_SCHEMA.to_string(),
        canonical_codec: WORLD_COMMIT_CODEC.to_string(),
        canonical_domain_tag: WORLD_COMMIT_IDENTITY_DOMAIN.to_string(),
        supported_version: WORLD_COMMIT_VERSION.to_string(),
        fixture_manifest_identity,
        license: MOLTEN_LICENSE.to_string(),
        source_artifacts,
        adapter_contracts,
        negative_vector_ids: NEGATIVE_VECTOR_IDS
            .iter()
            .map(ToString::to_string)
            .collect(),
        checked_laws: REQUIRED_CHECKED_LAWS
            .iter()
            .map(ToString::to_string)
            .collect(),
        non_claims: REQUIRED_NON_CLAIMS
            .iter()
            .map(ToString::to_string)
            .collect(),
        semantic_claims: Vec::new(),
        declared_entry_identity: CompatibilityIdentity::compute(b"pending").to_wire(),
    };
    entry.declared_entry_identity = compute_world_commit_cohort_identity(&entry)
        .map_err(|issue| format!("cohort identity failed: {issue:?}"))?
        .to_wire();
    Ok(entry)
}

fn source_artifact(role: WorldSourceRole) -> Result<WorldSourceArtifact, String> {
    let name = format!("{role:?}");
    let byte_length = u64::try_from(name.len())
        .map_err(|error| format!("source length conversion failed: {error}"))?;
    let identity = if role == WorldSourceRole::MoltenProductManifest {
        SourceIdentity::from_hex(
            Blake3Hex::parse(MOLTEN_PRODUCT_SOURCE_MANIFEST_HEX)
                .map_err(|error| format!("product manifest identity: {error}"))?,
        )
    } else {
        SourceIdentity::compute(name.as_bytes())
    };
    Ok(WorldSourceArtifact {
        role,
        path: format!("fixtures/world-commit/source/{name}"),
        identity,
        byte_length,
    })
}
