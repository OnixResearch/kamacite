use super::*;

// r[verify kamacite.bend_hvm2_pilot.boundary]
// r[verify kamacite.bend_hvm2_pilot.workload]
// r[verify kamacite.bend_hvm2_pilot.trellis_oracle_evidence]
// r[verify kamacite.bend_hvm2_pilot.functional_core]
// r[verify kamacite.bend_hvm2_pilot.oracle]
// r[verify kamacite.bend_hvm2_pilot.benchmark]
// r[verify kamacite.bend_hvm2_pilot.promotion]
#[test]
fn exact_facts_reach_an_adopt_decision() {
    let Some(owned) = OwnedPilotRequest::valid() else {
        return;
    };
    let evaluation = evaluate_bend_hvm2_pilot(&owned.request());
    assert!(evaluation.is_ok(), "{evaluation:?}");
    let Ok(evaluation) = evaluation else {
        return;
    };
    assert_eq!(evaluation.decision, PilotDecision::Adopt);
    assert!(evaluation.diagnostics.is_empty());
    assert!(evaluation.blockers.is_empty());
    assert!(evaluation.benchmark.is_some());
}

// r[verify kamacite.bend_hvm2_pilot.workload]
#[test]
fn canonical_leaves_render_the_checked_source() {
    let input = decode_leaf_input(INPUT_BYTES);
    assert!(input.is_ok());
    let Ok(input) = input else {
        return;
    };
    let rendered = render_bend_workload(&input.leaves);
    assert!(rendered.is_ok());
    assert_eq!(rendered.ok().as_deref(), Some(SOURCE_BYTES));
    assert_eq!(input.leaves, vec![U24_MAXIMUM, 1, 0, 1]);
}

// r[verify kamacite.bend_hvm2_pilot.evidence]
#[test]
fn route_output_parser_separates_hvm_interactions_from_oracle_combines() {
    let oracle = parse_route_output(PilotStage::TrellisOracle, b"Result: 1\n- COMBINES: 3\n");
    let run_rs = parse_route_output(PilotStage::BendRunRs, b"Result: 1\n- ITRS: 114\n");
    let generated = parse_route_output(PilotStage::GeneratedC, b"Result: 1\n- ITRS: 113\n");
    assert_eq!(
        oracle.ok(),
        Some(ParsedRouteOutput {
            result: PILOT_RESULT,
            interaction_count: None,
            combine_count: Some(3),
        })
    );
    assert_eq!(
        run_rs.ok().and_then(|output| output.interaction_count),
        Some(114)
    );
    assert_eq!(
        generated.ok().and_then(|output| output.interaction_count),
        Some(113)
    );
}

// r[verify kamacite.bend_hvm2_pilot.evidence]
#[test]
fn receipt_and_valence_projection_are_deterministic_and_role_scoped() {
    let Some(owned) = OwnedPilotRequest::valid() else {
        return;
    };
    let evaluation = evaluate_bend_hvm2_pilot(&owned.request());
    assert!(evaluation.is_ok());
    let Ok(evaluation) = evaluation else {
        return;
    };
    let first = build_pilot_receipt(&evaluation, &owned.observations);
    let second = build_pilot_receipt(&evaluation, &owned.observations);
    assert!(first.is_ok());
    assert_eq!(first, second);
    let Ok(first) = first else {
        return;
    };
    assert!(validate_pilot_receipt(&evaluation, &owned.observations, &first).is_ok());
    let projection = project_pilot_receipt_to_valence(&first);
    assert!(projection.is_ok());
    let Ok(projection) = projection else {
        return;
    };
    assert_eq!(projection.subject_receipt_identity, first.receipt_identity);
    assert!(projection.artifacts.iter().any(|artifact| {
        artifact.role == "trellis-proof-validation"
            && artifact.verification_class == BendHvm2ValenceClass::VerifiedTrellisLocal
    }));
    assert!(projection.artifacts.iter().all(|artifact| {
        artifact.role != "generated-executable"
            || artifact.verification_class == BendHvm2ValenceClass::RecordedOnly
    }));
}
