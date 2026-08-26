use std::path::PathBuf;

use kamacite_cli::root_io::RootCapability;
use kamacite_core::SourceIdentity;
use kamacite_core::world_commit::{
    MOLTEN_REVISION, WorldCohortUpdatePlan, WorldCohortUpdateRequest, WorldConformanceIssue,
    WorldParityInput, evaluate_world_commit_parity, plan_world_commit_cohort_update,
    project_world_commit_receipt_for_valence, validate_world_commit_cohort,
};
use serde::Serialize;

use crate::world_commit_shell::adapters::{AdapterBuild, build_adapters};
use crate::world_commit_shell::implementation::materialize_implementation_manifest;
use crate::world_commit_shell::io::{
    from_json, pretty_json, read_bounded, typed_wire, write_or_compare,
};
use crate::world_commit_shell::model::{
    AlteredSourceVector, CohortConfig, ReceiptProjection, VectorCorpus,
};
use crate::world_commit_shell::nickel::sandboxed_nickel_export;
use crate::world_commit_shell::product::materialize_product_source_manifest;
use crate::world_commit_shell::vectors::{build_vector_corpus, materialize_derived_vectors};

const COHORT_CONFIG_PATH: &str = "fixtures/world-commit/cohort.ncl";
const COHORT_CONTRACT_PATH: &str = "fixtures/world-commit/cohort-contract.ncl";
const COHORT_EVALUATOR_PROFILE: &str =
    "nickel-export-json;declared-import-closure;world-commit-cohort-v1";
const ALTERED_SOURCE_VECTOR_ID: &str = "altered-source";
const RECEIPT_PROJECTION_SCHEMA: &str = "kamacite.world-commit-receipt-projection.v1";
const EXPECTED_ADAPTER_COUNT: usize = 3;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunOptions {
    pub root: PathBuf,
    pub update: bool,
    pub accepted_revision: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanUpdateOptions {
    pub root: PathBuf,
    pub proposed_revision: String,
    pub proposed_schema_path: Option<String>,
    pub proposed_codec_path: Option<String>,
}

#[derive(Serialize)]
struct UpdatePlanProjection<'a> {
    schema: &'static str,
    request: &'a WorldCohortUpdateRequest,
    plan: &'a WorldCohortUpdatePlan,
    non_claims: &'a [String],
}

#[derive(Serialize)]
struct RunSummary {
    schema: &'static str,
    cohort_identity: String,
    canonical_value_identity: String,
    receipt_identity: String,
    positive_vector_count: usize,
    negative_vector_count: usize,
    adapter_count: usize,
    verdict: &'static str,
}

// r[impl kamacite.world_commit_conformance.update]
pub fn plan_update(options: &PlanUpdateOptions) -> Result<Vec<u8>, String> {
    let root = RootCapability::open(&options.root)
        .map_err(|error| format!("open repository root: {error}"))?;
    let config = load_config(&root)?;
    let proposed_schema_path = options
        .proposed_schema_path
        .as_deref()
        .unwrap_or(&config.molten_schema_path);
    let proposed_codec_path = options
        .proposed_codec_path
        .as_deref()
        .unwrap_or(&config.molten_codec_path);
    let current_schema = read_bounded(&root, &config.molten_schema_path)?;
    let proposed_schema = read_bounded(&root, proposed_schema_path)?;
    let current_codec = read_bounded(&root, &config.molten_codec_path)?;
    let proposed_codec = read_bounded(&root, proposed_codec_path)?;
    let request = WorldCohortUpdateRequest {
        current_revision: config.product_revision.clone(),
        proposed_revision: options.proposed_revision.clone(),
        current_schema_identity: SourceIdentity::compute(&current_schema),
        proposed_schema_identity: SourceIdentity::compute(&proposed_schema),
        current_codec_identity: SourceIdentity::compute(&current_codec),
        proposed_codec_identity: SourceIdentity::compute(&proposed_codec),
        explicit_regeneration: false,
        convergence_passes: 0,
        removed_negative_vectors: Vec::new(),
    };
    let plan = plan_world_commit_cohort_update(&request);
    pretty_json(&UpdatePlanProjection {
        schema: "kamacite.world-commit-cohort-update-plan.v1",
        request: &request,
        plan: &plan,
        non_claims: &config.non_claims,
    })
}

pub fn run(options: &RunOptions) -> Result<Vec<u8>, String> {
    let root = RootCapability::open(&options.root)
        .map_err(|error| format!("open repository root: {error}"))?;
    let config = load_config(&root)?;
    authorize_update(options, &config)?;
    materialize_product_source_manifest(&root, &config, options.update)?;
    materialize_implementation_manifest(&root, &config, options.update)?;
    let primary_text = primary_text(&root, &config)?;
    materialize_derived_vectors(&root, &config, &primary_text, options.update)?;
    let corpus = build_vector_corpus(&root, &config, options.update)?;

    let first = build_adapters(&root, &config, &corpus)?;
    let second = build_adapters(&root, &config, &corpus)?;
    if first != second {
        return Err("fresh adapter passes did not converge on one cohort and value".to_string());
    }
    if first.cohort.adapter_contracts.len() != EXPECTED_ADAPTER_COUNT {
        return Err("world-commit cohort does not contain three adapter contracts".to_string());
    }
    validate_cohort_and_negative_source(&root, &config, &first)?;
    write_or_compare(
        &root,
        &config.generated_cohort_path,
        &pretty_json(&first.cohort)?,
        options.update,
    )?;

    let first_receipt = parity_receipt(&first, &corpus)?;
    let second_receipt = parity_receipt(&second, &corpus)?;
    if first_receipt.body != second_receipt.body
        || first_receipt.canonical_bytes != second_receipt.canonical_bytes
        || first_receipt.receipt_identity != second_receipt.receipt_identity
    {
        return Err("fresh parity receipts did not converge byte for byte".to_string());
    }
    write_or_compare(
        &root,
        &config.generated_receipt_path,
        &first_receipt.canonical_bytes,
        options.update,
    )?;
    let receipt_projection = ReceiptProjection {
        schema: RECEIPT_PROJECTION_SCHEMA,
        receipt_identity: typed_wire(&first_receipt.receipt_identity),
        canonical_value_identity: typed_wire(&first_receipt.body.canonical_value_identity),
        molten_world_commit_ref: &first_receipt.body.molten_world_commit_ref,
        cohort_identity: typed_wire(&first_receipt.body.cohort_identity),
        adapter_count: first_receipt.body.adapters.len(),
        verdict: &first_receipt.body.verdict,
        checked_laws: &first_receipt.body.checked_laws,
        non_claims: &first_receipt.body.non_claims,
    };
    write_or_compare(
        &root,
        &config.generated_receipt_projection_path,
        &pretty_json(&receipt_projection)?,
        options.update,
    )?;
    let valence = project_world_commit_receipt_for_valence(&first_receipt);
    write_or_compare(
        &root,
        &config.generated_valence_path,
        &pretty_json(&valence)?,
        options.update,
    )?;
    let summary = RunSummary {
        schema: "kamacite.world-commit-conformance-run.v1",
        cohort_identity: typed_wire(&first_receipt.body.cohort_identity),
        canonical_value_identity: typed_wire(&first_receipt.body.canonical_value_identity),
        receipt_identity: typed_wire(&first_receipt.receipt_identity),
        positive_vector_count: corpus.manifest.positive_vectors.len(),
        negative_vector_count: corpus.manifest.negative_vectors.len(),
        adapter_count: first_receipt.body.adapters.len(),
        verdict: "pass",
    };
    pretty_json(&summary)
}

fn parity_receipt(
    adapters: &AdapterBuild,
    corpus: &VectorCorpus,
) -> Result<kamacite_core::world_commit::WorldParityReceipt, String> {
    evaluate_world_commit_parity(&WorldParityInput {
        cohort: adapters.cohort.clone(),
        authoritative_packed_bytes: corpus.primary_packed.clone(),
        adapters: adapters.observations.clone(),
    })
    .map_err(|issues| format!("world-commit parity denied: {issues:?}"))
}

fn validate_cohort_and_negative_source(
    root: &RootCapability,
    config: &CohortConfig,
    adapters: &AdapterBuild,
) -> Result<(), String> {
    let issues = validate_world_commit_cohort(&adapters.cohort);
    if !issues.is_empty() {
        return Err(format!("world-commit cohort denied: {issues:?}"));
    }
    let vector = config
        .negative_vectors
        .iter()
        .find(|vector| vector.id == ALTERED_SOURCE_VECTOR_ID)
        .ok_or_else(|| "altered-source vector is absent".to_string())?;
    let bytes = read_bounded(root, &vector.path)?;
    let payload: AlteredSourceVector = from_json(&bytes, ALTERED_SOURCE_VECTOR_ID)?;
    let mut altered = adapters.cohort.clone();
    let source = altered
        .source_artifacts
        .iter_mut()
        .find(|source| source.path == payload.source_path)
        .ok_or_else(|| "altered source path is outside the cohort".to_string())?;
    source.identity = SourceIdentity::compute(payload.replacement_text.as_bytes());
    source.byte_length = 0;
    let issues = validate_world_commit_cohort(&altered);
    if !issues
        .iter()
        .any(|issue| matches!(issue, WorldConformanceIssue::SourceIdentity { .. }))
    {
        return Err("altered source vector did not produce a source-identity denial".to_string());
    }
    Ok(())
}

fn load_config(root: &RootCapability) -> Result<CohortConfig, String> {
    let export = sandboxed_nickel_export(
        root,
        COHORT_CONFIG_PATH,
        COHORT_CONTRACT_PATH,
        COHORT_EVALUATOR_PROFILE,
    )?;
    from_json(&export.output, "world-commit cohort config")
}

fn primary_text(root: &RootCapability, config: &CohortConfig) -> Result<String, String> {
    let vector = config
        .positive_vectors
        .iter()
        .find(|vector| vector.id == "logical-v1")
        .ok_or_else(|| "logical-v1 vector is absent".to_string())?;
    let bytes = read_bounded(root, &vector.path)?;
    String::from_utf8(bytes).map_err(|error| format!("primary world vector is not UTF-8: {error}"))
}

fn authorize_update(options: &RunOptions, config: &CohortConfig) -> Result<(), String> {
    if !options.update {
        if options.accepted_revision.is_some() {
            return Err("--accept-revision is valid only with --update".to_string());
        }
        return Ok(());
    }
    if config.product_revision != MOLTEN_REVISION {
        return Err("cohort config names an unsupported Molten revision".to_string());
    }
    if options.accepted_revision.as_deref() != Some(config.product_revision.as_str()) {
        return Err(format!(
            "fixture update requires --accept-revision {}",
            config.product_revision
        ));
    }
    Ok(())
}
