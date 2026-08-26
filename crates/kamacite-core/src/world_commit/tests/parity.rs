use crate::identity::SourceIdentity;
use crate::world_commit::{
    REQUIRED_NON_CLAIMS, WorldAdapterKind, WorldAdapterObservation, WorldConformanceIssue,
    WorldParityReceipt, WorldSourceRole, compute_world_commit_cohort_identity,
    evaluate_world_commit_parity, project_world_commit_receipt_for_valence,
    validate_world_commit_cohort,
};

use super::support::{cohort_entry, parity_input};

const EXPECTED_ADAPTER_COUNT: usize = 3;
type TestResult = Result<(), String>;

// r[verify kamacite.world_commit_conformance.cohort]
// r[verify kamacite.world_commit_conformance.entry]
// r[verify kamacite.world_commit_conformance.parity]
// r[verify kamacite.world_commit_conformance.nickel]
// r[verify kamacite.world_commit_conformance.steel]
// r[verify kamacite.world_commit_conformance.sources]
// r[verify kamacite.world_commit_conformance.receipt]
#[test]
fn three_independent_adapters_converge_on_one_canonical_value() -> TestResult {
    let input = parity_input()?;
    let first = admitted(evaluate_world_commit_parity(&input))?;
    let second = admitted(evaluate_world_commit_parity(&input))?;
    assert_eq!(first.body, second.body);
    assert_eq!(first.canonical_bytes, second.canonical_bytes);
    assert_eq!(first.receipt_identity, second.receipt_identity);
    assert_eq!(first.body.adapters.len(), EXPECTED_ADAPTER_COUNT);
    assert!(
        first
            .body
            .adapters
            .iter()
            .all(|adapter| adapter.exact_byte_match && adapter.exact_value_match)
    );
    Ok(())
}

#[test]
fn valence_projection_is_recorded_only_and_preserves_non_claims() -> TestResult {
    let receipt = admitted(evaluate_world_commit_parity(&parity_input()?))?;
    let projection = project_world_commit_receipt_for_valence(&receipt);
    assert_eq!(projection.role, "recorded-only");
    assert_eq!(projection.receipt_identity, receipt.receipt_identity);
    assert_eq!(
        projection.non_claims,
        REQUIRED_NON_CLAIMS
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn missing_adapter_and_byte_drift_fail_closed() -> TestResult {
    let mut missing = parity_input()?;
    missing
        .adapters
        .retain(|adapter| adapter.adapter != WorldAdapterKind::Steel);
    assert!(evaluate_world_commit_parity(&missing).is_err());

    let mut drifted = parity_input()?;
    adapter_mut(&mut drifted.adapters, WorldAdapterKind::Nickel)?
        .resolved_packed_bytes
        .push(0);
    let issues = denied(evaluate_world_commit_parity(&drifted))?;
    assert!(issues.iter().any(|issue| matches!(
        issue,
        WorldConformanceIssue::ByteMismatch { adapter } if adapter == "Nickel"
    )));
    Ok(())
}

#[test]
fn ambient_steel_effects_and_semantic_overclaims_are_denied() -> TestResult {
    let mut effected = parity_input()?;
    adapter_mut(&mut effected.adapters, WorldAdapterKind::Steel)?
        .observed_effects
        .push("filesystem-read".to_string());
    let issues = denied(evaluate_world_commit_parity(&effected))?;
    assert!(issues.iter().any(|issue| matches!(
        issue,
        WorldConformanceIssue::AdapterBoundary { adapter, .. } if adapter == "Steel"
    )));

    let mut claimed = parity_input()?;
    let first = claimed
        .adapters
        .first_mut()
        .ok_or_else(|| "parity input has no adapters".to_string())?;
    first
        .semantic_claims
        .push("proves Molten restore compatibility".to_string());
    assert!(evaluate_world_commit_parity(&claimed).is_err());
    Ok(())
}

#[test]
fn altered_sources_and_unknown_cohorts_are_rejected() -> TestResult {
    let mut altered = cohort_entry()?;
    let first = altered
        .source_artifacts
        .first_mut()
        .ok_or_else(|| "cohort has no source artifacts".to_string())?;
    first.identity = SourceIdentity::compute(b"altered-source");
    let issues = validate_world_commit_cohort(&altered);
    assert!(!issues.is_empty());

    let mut product_drift = cohort_entry()?;
    let product_manifest = product_drift
        .source_artifacts
        .iter_mut()
        .find(|artifact| artifact.role == WorldSourceRole::MoltenProductManifest)
        .ok_or_else(|| "product manifest source is absent".to_string())?;
    product_manifest.identity = SourceIdentity::compute(b"altered-product-manifest");
    product_drift.declared_entry_identity = compute_world_commit_cohort_identity(&product_drift)
        .map_err(|issue| format!("altered cohort identity failed: {issue:?}"))?
        .to_wire();
    let issues = validate_world_commit_cohort(&product_drift);
    assert!(issues.iter().any(|issue| matches!(
        issue,
        WorldConformanceIssue::SourceIdentity { role, .. }
            if role == "MoltenProductManifest"
    )));

    let mut future = cohort_entry()?;
    future.product_revision = "ffffffffffffffffffffffffffffffffffffffff".to_string();
    future.declared_entry_identity = compute_world_commit_cohort_identity(&future)
        .map_err(|issue| format!("future cohort identity failed: {issue:?}"))?
        .to_wire();
    let issues = validate_world_commit_cohort(&future);
    assert!(issues.iter().any(|issue| matches!(
        issue,
        WorldConformanceIssue::CohortField { field, .. } if field == "product-revision"
    )));
    Ok(())
}

#[test]
fn json_canonicality_claim_is_denied_even_when_bytes_match() -> TestResult {
    let mut input = parity_input()?;
    let first = input
        .adapters
        .first_mut()
        .ok_or_else(|| "parity input has no adapters".to_string())?;
    first.claims_json_canonical = true;
    let issues = denied(evaluate_world_commit_parity(&input))?;
    assert!(issues.iter().any(|issue| matches!(
        issue,
        WorldConformanceIssue::SemanticOverclaim { claim }
            if claim.contains("claims JSON is canonical")
    )));
    Ok(())
}

fn adapter_mut(
    adapters: &mut [WorldAdapterObservation],
    kind: WorldAdapterKind,
) -> Result<&mut WorldAdapterObservation, String> {
    adapters
        .iter_mut()
        .find(|adapter| adapter.adapter == kind)
        .ok_or_else(|| format!("adapter is absent: {kind:?}"))
}

fn admitted(
    result: Result<WorldParityReceipt, Vec<WorldConformanceIssue>>,
) -> Result<WorldParityReceipt, String> {
    result.map_err(|issues| format!("parity denied: {issues:?}"))
}

fn denied(
    result: Result<WorldParityReceipt, Vec<WorldConformanceIssue>>,
) -> Result<Vec<WorldConformanceIssue>, String> {
    match result {
        Ok(_) => Err("parity was unexpectedly admitted".to_string()),
        Err(issues) => Ok(issues),
    }
}
