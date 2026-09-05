use super::*;

// r[verify kamacite.bend_hvm2_pilot.fixtures]
#[test]
fn rejects_cohort_license_source_and_input_drift_before_execution() {
    let Some(mut cohort) = OwnedPilotRequest::valid() else {
        return;
    };
    cohort.toolchain.bend_version = "0.2.39".to_string();
    assert_error_boundary(&cohort, PilotBoundary::PreExecution, "cohort-drift");

    let Some(mut license) = OwnedPilotRequest::valid() else {
        return;
    };
    license.bend_license_bytes.push(b'\n');
    assert_error_boundary(&license, PilotBoundary::PreExecution, "license-drift");

    let Some(mut source) = OwnedPilotRequest::valid() else {
        return;
    };
    source.source_bytes = b"def main() -> Any:\n  return 1\n".to_vec();
    let errors = evaluate_bend_hvm2_pilot(&source.request()).err();
    assert!(errors.is_some_and(|items| {
        items.iter().any(|item| item.code == "missing-annotation")
            && items.iter().any(|item| item.code == "forbidden-authority")
    }));

    let Some(mut input) = OwnedPilotRequest::valid() else {
        return;
    };
    input.input_bytes = br#"{
      "schema_version":"kamacite.bend-hvm2-leaf-input.v1",
      "numeric_type":"u24",
      "leaves":[1,2,3]
    }"#
    .to_vec();
    assert_error_boundary(&input, PilotBoundary::PreExecution, "leaf-shape");
}

// r[verify kamacite.bend_hvm2_pilot.trellis_oracle_evidence]
#[test]
fn rejects_missing_or_drifted_trellis_proof_evidence() {
    let Some(mut missing) = OwnedPilotRequest::valid() else {
        return;
    };
    missing.proof_validation.proof_result_present = false;
    missing.proof_validation.passed = false;
    missing
        .proof_validation
        .diagnostics
        .push("proof command failed".to_string());
    assert_error_boundary(&missing, PilotBoundary::PreExecution, "proof-rerun-failed");

    let Some(mut bundle) = OwnedPilotRequest::valid() else {
        return;
    };
    bundle.proof_bundle_bytes.push(b'\n');
    assert_error_boundary(&bundle, PilotBoundary::PreExecution, "proof-bundle-drift");
}

// r[verify kamacite.bend_hvm2_pilot.functional_core]
// r[verify kamacite.bend_hvm2_pilot.evidence]
#[test]
fn tool_failures_defer_and_backend_disagreement_rejects() {
    let Some(mut failed) = OwnedPilotRequest::valid() else {
        return;
    };
    if let Some(observation) = failed
        .observations
        .iter_mut()
        .find(|observation| observation.stage == PilotStage::BendRunRs)
    {
        observation.outcome = ProcessOutcome::Timeout;
        observation.exit_code = None;
    }
    let evaluation = evaluate_bend_hvm2_pilot(&failed.request());
    assert!(evaluation.is_ok());
    assert_eq!(
        evaluation.ok().map(|result| result.decision),
        Some(PilotDecision::Defer)
    );

    let Some(mut disagreement) = OwnedPilotRequest::valid() else {
        return;
    };
    for observation in disagreement
        .observations
        .iter_mut()
        .filter(|observation| observation.stage == PilotStage::GeneratedC)
    {
        observation.result = Some(2);
    }
    let evaluation = evaluate_bend_hvm2_pilot(&disagreement.request());
    assert!(evaluation.is_ok());
    assert_eq!(
        evaluation.ok().map(|result| result.decision),
        Some(PilotDecision::Reject)
    );
}

// r[verify kamacite.bend_hvm2_pilot.benchmark]
#[test]
fn unstable_or_slow_measurements_defer_with_exact_benchmark_boundary() {
    const SLOW_WALL_NANOSECONDS: u64 = 6_000_000_000;

    let Some(mut owned) = OwnedPilotRequest::valid() else {
        return;
    };
    if let Some(observation) = owned
        .observations
        .iter_mut()
        .find(|observation| observation.stage == PilotStage::GeneratedC && !observation.warmup)
    {
        observation.wall_nanoseconds = SLOW_WALL_NANOSECONDS;
    }
    let evaluation = evaluate_bend_hvm2_pilot(&owned.request());
    assert!(evaluation.is_ok());
    let Some(evaluation) = evaluation.ok() else {
        return;
    };
    assert_eq!(evaluation.decision, PilotDecision::Defer);
    assert!(
        evaluation
            .diagnostics
            .iter()
            .any(|item| item.boundary == PilotBoundary::Benchmark)
    );
}

// r[verify kamacite.bend_hvm2_pilot.benchmark]
#[test]
fn fixed_hvm2_global_layout_defers_the_one_gibibyte_profile() {
    let Some(mut owned) = OwnedPilotRequest::valid() else {
        return;
    };
    owned.generated_c_bytes = br#"
#define TPC_L2 5
#define RLEN (1ul << 24)
#define G_NODE_LEN (1ul << 29)
#define G_VARS_LEN (1ul << 29)
APair node_buf[G_NODE_LEN];
APort vars_buf[G_VARS_LEN];
APair rbag_buf[G_RBAG_LEN];
"#
    .to_vec();
    let evaluation = evaluate_bend_hvm2_pilot(&owned.request());
    assert!(evaluation.is_ok());
    let Some(evaluation) = evaluation.ok() else {
        return;
    };
    assert_eq!(evaluation.decision, PilotDecision::Defer);
    assert!(evaluation.diagnostics.iter().any(|item| {
        item.code == "hvm2-address-space-profile" && item.boundary == PilotBoundary::Execution
    }));
}

// r[verify kamacite.bend_hvm2_pilot.evidence]
#[test]
fn malformed_outputs_missing_artifacts_and_stale_receipts_fail_closed() {
    assert!(parse_route_output(PilotStage::BendRunRs, b"Result: 1\n").is_err());
    assert!(parse_route_output(PilotStage::TrellisOracle, b"Result: nope\n").is_err());

    let Some(mut missing_artifact) = OwnedPilotRequest::valid() else {
        return;
    };
    missing_artifact.generated_c_bytes.clear();
    assert_error_boundary(
        &missing_artifact,
        PilotBoundary::PostOutput,
        "artifact-limit",
    );

    let Some(owned) = OwnedPilotRequest::valid() else {
        return;
    };
    let evaluation = evaluate_bend_hvm2_pilot(&owned.request());
    let Some(evaluation) = evaluation.ok() else {
        return;
    };
    let receipt = build_pilot_receipt(&evaluation, &owned.observations);
    let Some(mut receipt) = receipt.ok() else {
        return;
    };
    receipt.body.blockers.push("stale".to_string());
    assert!(validate_pilot_receipt(&evaluation, &owned.observations, &receipt).is_err());
}

fn assert_error_boundary(owned: &OwnedPilotRequest, boundary: PilotBoundary, code: &str) {
    let errors = evaluate_bend_hvm2_pilot(&owned.request()).err();
    assert!(errors.is_some_and(|items| {
        items
            .iter()
            .any(|item| item.boundary == boundary && item.code == code)
    }));
}
