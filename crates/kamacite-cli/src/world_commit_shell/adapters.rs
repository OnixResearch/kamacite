use kamacite_core::world_commit::{
    WorldAdapterContract, WorldAdapterKind, WorldAdapterObservation, WorldCommitCohortEntry,
    WorldCommitValue, WorldSourceArtifact, WorldSourceRole, compute_world_commit_cohort_identity,
};
use kamacite_core::{
    AdapterIdentity, CanonicalValueIdentity, CompatibilityIdentity, EvaluatorIdentity,
    ProjectionIdentity, SourceIdentity,
};
use serde::{Deserialize, Serialize};

use crate::world_commit_shell::io::{from_json, pretty_json, read_bounded};
use crate::world_commit_shell::model::{CohortConfig, VectorCorpus};
use crate::world_commit_shell::nickel::sandboxed_nickel_export;
use kamacite_cli::root_io::RootCapability;
use kamacite_cli::steel_runner::{
    WORLD_VECTOR_CAPABILITY, WORLD_VECTOR_HOST_FUNCTION, WORLD_VECTOR_RUNNER_DESCRIPTOR,
    run_world_vector_probe,
};

const PRIMARY_VECTOR_ID: i64 = 0;
const NICKEL_EVALUATOR_PROFILE: &str =
    "nickel-export-json;declared-import-closure;world-commit-projection-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterBuild {
    pub cohort: WorldCommitCohortEntry,
    pub observations: Vec<WorldAdapterObservation>,
}

#[derive(Serialize)]
struct AdapterIdentityMaterial<'a> {
    adapter: WorldAdapterKind,
    source_identity: &'a SourceIdentity,
    implementation_identity: &'a SourceIdentity,
    evaluator_identity: Option<&'a EvaluatorIdentity>,
    enforcement_identity: Option<&'a AdapterIdentity>,
    projection_identity: Option<&'a ProjectionIdentity>,
    allowed_capabilities: &'a [String],
}

#[derive(Deserialize)]
struct SteelVectorOutput {
    value: i64,
}

#[derive(Serialize)]
struct SteelEnforcementMaterial<'a> {
    runner_descriptor: &'a str,
    output: &'a [u8],
    observed_host_functions: &'a [String],
    observed_effects: &'a [String],
}

pub fn build_adapters(
    root: &RootCapability,
    config: &CohortConfig,
    corpus: &VectorCorpus,
) -> Result<AdapterBuild, String> {
    let source_artifacts = source_inventory(root, config)?;
    let implementation_identity = source_identity_for_role(
        &source_artifacts,
        WorldSourceRole::KamaciteImplementationManifest,
    )?;
    let rust_source =
        source_identity_for_role(&source_artifacts, WorldSourceRole::MoltenCanonicalFixture)?;
    let nickel_source =
        source_identity_for_path(&source_artifacts, &config.nickel_projection_path)?;
    let steel_source = source_identity_for_path(&source_artifacts, &config.steel_source_path)?;

    let rust_contract = adapter_contract(
        WorldAdapterKind::Rust,
        rust_source,
        implementation_identity.clone(),
        None,
        None,
        None,
        Vec::new(),
    )?;
    let nickel_projection = run_nickel_projection(root, config, &corpus.primary)?;
    let nickel_contract = adapter_contract(
        WorldAdapterKind::Nickel,
        nickel_source,
        implementation_identity.clone(),
        Some(nickel_projection.evaluator_identity.clone()),
        Some(nickel_projection.enforcement_identity.clone()),
        Some(nickel_projection.projection_identity.clone()),
        nickel_projection.observed_effects.clone(),
    )?;
    let steel_projection = run_steel_projection()?;
    let steel_contract = adapter_contract(
        WorldAdapterKind::Steel,
        steel_source,
        implementation_identity,
        Some(steel_projection.evaluator_identity.clone()),
        Some(steel_projection.enforcement_identity.clone()),
        Some(steel_projection.projection_identity.clone()),
        vec![WORLD_VECTOR_CAPABILITY.to_string()],
    )?;
    let adapter_contracts = vec![rust_contract, nickel_contract, steel_contract];

    let fixture_manifest_identity =
        source_identity_for_role(&source_artifacts, WorldSourceRole::FixtureManifest)?;
    let mut cohort = WorldCommitCohortEntry {
        schema: config.schema.clone(),
        product_repository: config.product_repository.clone(),
        product_revision: config.product_revision.clone(),
        product_owner: config.product_owner.clone(),
        product_schema: config.product_schema.clone(),
        canonical_codec: config.canonical_codec.clone(),
        canonical_domain_tag: config.canonical_domain_tag.clone(),
        supported_version: config.supported_version.clone(),
        fixture_manifest_identity,
        license: config.license.clone(),
        source_artifacts,
        adapter_contracts,
        negative_vector_ids: config
            .negative_vectors
            .iter()
            .map(|vector| vector.id.clone())
            .collect(),
        checked_laws: config.checked_laws.clone(),
        non_claims: config.non_claims.clone(),
        semantic_claims: Vec::new(),
        declared_entry_identity: CompatibilityIdentity::compute(b"pending").to_wire(),
    };
    cohort.declared_entry_identity = compute_world_commit_cohort_identity(&cohort)
        .map_err(|issue| format!("compute cohort identity: {issue:?}"))?
        .to_wire();
    let observations = build_observations(
        &cohort,
        corpus,
        nickel_projection,
        steel_projection,
        &config.checked_laws,
    )?;
    Ok(AdapterBuild {
        cohort,
        observations,
    })
}

struct ProjectionRun {
    evaluator_identity: EvaluatorIdentity,
    enforcement_identity: AdapterIdentity,
    projection_identity: ProjectionIdentity,
    observed_effects: Vec<String>,
}

// r[impl kamacite.world_commit_conformance.nickel]
fn run_nickel_projection(
    root: &RootCapability,
    config: &CohortConfig,
    primary: &WorldCommitValue,
) -> Result<ProjectionRun, String> {
    let export = sandboxed_nickel_export(
        root,
        &config.nickel_projection_path,
        &config.nickel_contract_path,
        NICKEL_EVALUATOR_PROFILE,
    )?;
    let projection: WorldCommitValue = from_json(&export.output, "Nickel world-commit projection")?;
    if &projection != primary {
        return Err("Nickel projection does not resolve to the primary world value".to_string());
    }
    let canonical_projection = pretty_json(&projection)?;
    Ok(ProjectionRun {
        evaluator_identity: export.evaluator_identity,
        enforcement_identity: export.enforcement_identity,
        projection_identity: ProjectionIdentity::compute(&canonical_projection),
        observed_effects: export.observed_effects,
    })
}

fn run_steel_projection() -> Result<ProjectionRun, String> {
    let run = run_world_vector_probe()?;
    let output: SteelVectorOutput = from_json(&run.output_bytes, "Steel world vector output")?;
    if output.value != PRIMARY_VECTOR_ID {
        return Err("Steel adapter resolved an unknown world vector".to_string());
    }
    if run.observed_host_functions != [WORLD_VECTOR_HOST_FUNCTION] {
        return Err("Steel world vector probe called an undeclared host function".to_string());
    }
    if !run.observed_effects.is_empty() {
        return Err("Steel world vector probe observed an ambient effect".to_string());
    }
    let enforcement_bytes = serde_json::to_vec(&SteelEnforcementMaterial {
        runner_descriptor: WORLD_VECTOR_RUNNER_DESCRIPTOR,
        output: &run.output_bytes,
        observed_host_functions: &run.observed_host_functions,
        observed_effects: &run.observed_effects,
    })
    .map_err(|error| format!("encode Steel enforcement evidence: {error}"))?;
    Ok(ProjectionRun {
        evaluator_identity: EvaluatorIdentity::compute(WORLD_VECTOR_RUNNER_DESCRIPTOR.as_bytes()),
        enforcement_identity: AdapterIdentity::compute(&enforcement_bytes),
        projection_identity: ProjectionIdentity::compute(&run.output_bytes),
        observed_effects: run.observed_effects,
    })
}

fn build_observations(
    cohort: &WorldCommitCohortEntry,
    corpus: &VectorCorpus,
    nickel: ProjectionRun,
    steel: ProjectionRun,
    checked_laws: &[String],
) -> Result<Vec<WorldAdapterObservation>, String> {
    let value_identity = CanonicalValueIdentity::compute(&corpus.primary_packed);
    let mut observations = Vec::new();
    for contract in &cohort.adapter_contracts {
        let observed_effects = match contract.adapter {
            WorldAdapterKind::Rust => Vec::new(),
            WorldAdapterKind::Nickel => nickel.observed_effects.clone(),
            WorldAdapterKind::Steel => steel.observed_effects.clone(),
        };
        observations.push(WorldAdapterObservation {
            adapter: contract.adapter,
            source_identity: contract.source_identity.clone(),
            implementation_identity: contract.implementation_identity.clone(),
            adapter_identity: contract.adapter_identity.clone(),
            evaluator_identity: contract.evaluator_identity.clone(),
            enforcement_identity: contract.enforcement_identity.clone(),
            projection_identity: contract.projection_identity.clone(),
            resolved_value_identity: value_identity.clone(),
            resolved_packed_bytes: corpus.primary_packed.clone(),
            fresh_execution: true,
            declared_capabilities: contract.allowed_capabilities.clone(),
            observed_effects,
            checked_laws: checked_laws.to_vec(),
            claims_json_canonical: false,
            semantic_claims: Vec::new(),
        });
    }
    Ok(observations)
}

fn adapter_contract(
    adapter: WorldAdapterKind,
    source_identity: SourceIdentity,
    implementation_identity: SourceIdentity,
    evaluator_identity: Option<EvaluatorIdentity>,
    enforcement_identity: Option<AdapterIdentity>,
    projection_identity: Option<ProjectionIdentity>,
    allowed_capabilities: Vec<String>,
) -> Result<WorldAdapterContract, String> {
    let material = AdapterIdentityMaterial {
        adapter,
        source_identity: &source_identity,
        implementation_identity: &implementation_identity,
        evaluator_identity: evaluator_identity.as_ref(),
        enforcement_identity: enforcement_identity.as_ref(),
        projection_identity: projection_identity.as_ref(),
        allowed_capabilities: &allowed_capabilities,
    };
    let bytes = serde_json::to_vec(&material)
        .map_err(|error| format!("encode adapter identity material: {error}"))?;
    Ok(WorldAdapterContract {
        adapter,
        source_identity,
        implementation_identity,
        adapter_identity: AdapterIdentity::compute(&bytes),
        evaluator_identity,
        enforcement_identity,
        projection_identity,
        allowed_capabilities,
    })
}

// r[impl kamacite.world_commit_conformance.sources]
fn source_inventory(
    root: &RootCapability,
    config: &CohortConfig,
) -> Result<Vec<WorldSourceArtifact>, String> {
    config
        .sources
        .iter()
        .map(|source| {
            let bytes = read_bounded(root, &source.path)?;
            let byte_length = u64::try_from(bytes.len())
                .map_err(|error| format!("source length conversion: {error}"))?;
            Ok(WorldSourceArtifact {
                role: source.role,
                path: source.path.clone(),
                identity: SourceIdentity::compute(&bytes),
                byte_length,
            })
        })
        .collect()
}

fn source_identity_for_role(
    sources: &[WorldSourceArtifact],
    role: WorldSourceRole,
) -> Result<SourceIdentity, String> {
    let matches = sources
        .iter()
        .filter(|source| source.role == role)
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(format!(
            "source role must be unique for adapter binding: {role:?}"
        ));
    }
    Ok(matches[0].identity.clone())
}

fn source_identity_for_path(
    sources: &[WorldSourceArtifact],
    path: &str,
) -> Result<SourceIdentity, String> {
    sources
        .iter()
        .find(|source| source.path == path)
        .map(|source| source.identity.clone())
        .ok_or_else(|| format!("adapter source is outside the inventory: {path}"))
}
