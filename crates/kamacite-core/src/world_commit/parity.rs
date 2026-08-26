use crate::identity::ReceiptIdentity;
use crate::preserves_codec::{PreservesLimits, PreservesSchema, encode_typed_preserves};
use crate::world_commit::cohort::{
    compute_world_commit_cohort_identity, require_strings, validate_world_commit_cohort,
};
use crate::world_commit::model::{
    CONFORMANCE_RECEIPT_SCHEMA, MAX_ADAPTERS, REQUIRED_CHECKED_LAWS, REQUIRED_NON_CLAIMS,
    VALENCE_PROJECTION_SCHEMA, WORLD_COMMIT_VERSION, WorldAdapterContract, WorldAdapterKind,
    WorldAdapterObservation, WorldAdapterSummary, WorldCommitValenceProjection,
    WorldConformanceIssue, WorldParityInput, WorldParityReceipt, WorldParityReceiptBody,
};
use crate::world_commit::parse::admit_world_commit_packed;

const RECEIPT_ROOT_LABEL: &str = "kamacite-world-commit-conformance-receipt-v1";
const RECEIPT_MAX_DEPTH: usize = 24;
const RECEIPT_MAX_NODES: usize = 4_096;
const RECEIPT_MAX_BYTES: usize = 1_048_576;

// r[impl kamacite.world_commit_conformance.parity]
// r[impl kamacite.world_commit_conformance.receipt]
pub fn evaluate_world_commit_parity(
    input: &WorldParityInput,
) -> Result<WorldParityReceipt, Vec<WorldConformanceIssue>> {
    let mut issues = validate_world_commit_cohort(&input.cohort);
    let admitted = match admit_world_commit_packed(&input.authoritative_packed_bytes) {
        Ok(admitted) => admitted,
        Err(mut errors) => {
            issues.append(&mut errors);
            issues.sort();
            issues.dedup();
            return Err(issues);
        }
    };
    let adapters = validate_adapter_observations(input, &admitted, &mut issues);
    if !issues.is_empty() {
        issues.sort();
        issues.dedup();
        return Err(issues);
    }
    let cohort_identity =
        compute_world_commit_cohort_identity(&input.cohort).map_err(|issue| vec![issue])?;
    let body = WorldParityReceiptBody {
        schema: CONFORMANCE_RECEIPT_SCHEMA.to_string(),
        cohort_identity,
        canonical_value_identity: admitted.value_identity,
        molten_world_commit_ref: admitted.molten_world_commit_ref,
        adapters,
        checked_laws: REQUIRED_CHECKED_LAWS
            .iter()
            .map(ToString::to_string)
            .collect(),
        non_claims: REQUIRED_NON_CLAIMS
            .iter()
            .map(ToString::to_string)
            .collect(),
        verdict: "pass".to_string(),
    };
    let canonical_artifact = encode_typed_preserves(
        &body,
        PreservesSchema {
            schema_version: WORLD_COMMIT_VERSION,
            root_label: RECEIPT_ROOT_LABEL,
        },
        receipt_limits(),
    )
    .map_err(|error| {
        vec![WorldConformanceIssue::ReceiptEncoding {
            detail: error.to_string(),
        }]
    })?;
    let canonical_bytes = canonical_artifact.packed_bytes;
    let receipt_identity = ReceiptIdentity::compute(&canonical_bytes);
    Ok(WorldParityReceipt {
        body,
        canonical_bytes,
        receipt_identity,
    })
}

pub fn project_world_commit_receipt_for_valence(
    receipt: &WorldParityReceipt,
) -> WorldCommitValenceProjection {
    WorldCommitValenceProjection {
        schema: VALENCE_PROJECTION_SCHEMA.to_string(),
        role: "recorded-only".to_string(),
        receipt_identity: receipt.receipt_identity.clone(),
        cohort_identity: receipt.body.cohort_identity.clone(),
        canonical_value_identity: receipt.body.canonical_value_identity.clone(),
        adapter_identities: receipt
            .body
            .adapters
            .iter()
            .map(|adapter| adapter.adapter_identity.clone())
            .collect(),
        non_claims: REQUIRED_NON_CLAIMS
            .iter()
            .map(ToString::to_string)
            .collect(),
    }
}

fn validate_adapter_observations(
    input: &WorldParityInput,
    admitted: &crate::world_commit::model::AdmittedWorldCommit,
    issues: &mut Vec<WorldConformanceIssue>,
) -> Vec<WorldAdapterSummary> {
    if input.adapters.len() != MAX_ADAPTERS {
        issues.push(WorldConformanceIssue::CohortField {
            field: "adapters".to_string(),
            detail: format!("expected {MAX_ADAPTERS} observations"),
        });
    }
    let mut summaries = Vec::new();
    for contract in &input.cohort.adapter_contracts {
        let matches = input
            .adapters
            .iter()
            .filter(|observation| observation.adapter == contract.adapter)
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            continue;
        }
        let observation = matches[0];
        validate_adapter_observation(contract, observation, admitted, issues);
        summaries.push(WorldAdapterSummary {
            adapter: observation.adapter,
            adapter_identity: observation.adapter_identity.clone(),
            source_identity: observation.source_identity.clone(),
            implementation_identity: observation.implementation_identity.clone(),
            evaluator_identity: observation.evaluator_identity.clone(),
            enforcement_identity: observation.enforcement_identity.clone(),
            projection_identity: observation.projection_identity.clone(),
            exact_value_match: observation.resolved_value_identity == admitted.value_identity,
            exact_byte_match: observation.resolved_packed_bytes == admitted.packed_bytes,
            fresh_execution: observation.fresh_execution,
        });
    }
    summaries.sort_by_key(|summary| summary.adapter);
    summaries
}

fn validate_adapter_observation(
    contract: &WorldAdapterContract,
    observation: &WorldAdapterObservation,
    admitted: &crate::world_commit::model::AdmittedWorldCommit,
    issues: &mut Vec<WorldConformanceIssue>,
) {
    let adapter_name = format!("{:?}", observation.adapter);
    if observation.source_identity != contract.source_identity
        || observation.implementation_identity != contract.implementation_identity
        || observation.adapter_identity != contract.adapter_identity
        || observation.evaluator_identity != contract.evaluator_identity
        || observation.enforcement_identity != contract.enforcement_identity
        || observation.projection_identity != contract.projection_identity
        || observation.declared_capabilities != contract.allowed_capabilities
    {
        issues.push(WorldConformanceIssue::AdapterBoundary {
            adapter: adapter_name.clone(),
            detail: "observation is outside the pinned adapter cohort".to_string(),
        });
    }
    if !observation.fresh_execution {
        issues.push(WorldConformanceIssue::AdapterBoundary {
            adapter: adapter_name.clone(),
            detail: "adapter observation is not from a fresh check".to_string(),
        });
    }
    let effects_admitted = match observation.adapter {
        WorldAdapterKind::Nickel => observation.observed_effects == contract.allowed_capabilities,
        WorldAdapterKind::Rust | WorldAdapterKind::Steel => observation.observed_effects.is_empty(),
    };
    if !effects_admitted {
        issues.push(WorldConformanceIssue::AdapterBoundary {
            adapter: adapter_name.clone(),
            detail: "adapter observed effects outside its declared boundary".to_string(),
        });
    }
    if observation.claims_json_canonical {
        issues.push(WorldConformanceIssue::SemanticOverclaim {
            claim: format!("{adapter_name} claims JSON is canonical"),
        });
    }
    for claim in &observation.semantic_claims {
        issues.push(WorldConformanceIssue::SemanticOverclaim {
            claim: claim.clone(),
        });
    }
    require_strings(
        &observation.checked_laws,
        &REQUIRED_CHECKED_LAWS,
        "adapter checked-law",
        issues,
    );
    if observation.resolved_value_identity != admitted.value_identity {
        issues.push(WorldConformanceIssue::ValueMismatch {
            adapter: adapter_name.clone(),
        });
    }
    if observation.resolved_packed_bytes != admitted.packed_bytes {
        issues.push(WorldConformanceIssue::ByteMismatch {
            adapter: adapter_name,
        });
    }
}

fn receipt_limits() -> PreservesLimits {
    PreservesLimits {
        max_input_bytes: RECEIPT_MAX_BYTES,
        max_output_bytes: RECEIPT_MAX_BYTES,
        max_depth: RECEIPT_MAX_DEPTH,
        max_collection_items: RECEIPT_MAX_NODES,
        max_atom_bytes: RECEIPT_MAX_BYTES,
        max_values: RECEIPT_MAX_NODES,
    }
}
