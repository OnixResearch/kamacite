use super::*;
use serde::Deserialize;
use std::collections::BTreeSet;

const NEGATIVE_CASES: &[u8] =
    include_bytes!("../../../../../fixtures/bend-hvm2/negative-cases.json");
const EXPECTED_CASE_COUNT: usize = 20;
const FAILED_EXIT_CODE: i32 = 2;
const UNSTABLE_WALL_NANOSECONDS: u64 = 6_000_000_000;

#[derive(Deserialize)]
struct NegativeCase {
    id: String,
    mutation: String,
    expected_boundary: String,
}

// r[verify kamacite.bend_hvm2_pilot.fixtures]
#[test]
fn declared_negative_cases_fail_at_their_named_boundary() {
    let cases = serde_json::from_slice::<Vec<NegativeCase>>(NEGATIVE_CASES);
    assert!(cases.is_ok());
    let Ok(cases) = cases else {
        return;
    };
    assert_eq!(cases.len(), EXPECTED_CASE_COUNT);
    let ids = cases.iter().map(|case| &case.id).collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), EXPECTED_CASE_COUNT);
    for case in cases {
        let expected = parse_boundary(&case.expected_boundary);
        assert!(expected.is_some(), "unknown boundary for {}", case.id);
        let Some(expected) = expected else {
            continue;
        };
        let observed = execute_case(&case.mutation);
        assert!(
            observed
                .as_ref()
                .is_some_and(|boundaries| boundaries.contains(&expected)),
            "fixture {} did not reach {:?}: {:?}",
            case.id,
            expected,
            observed
        );
    }
}

fn execute_case(mutation: &str) -> Option<Vec<PilotBoundary>> {
    if mutation == "stale-receipt" {
        return stale_receipt_boundary();
    }
    let mut owned = OwnedPilotRequest::valid()?;
    apply_mutation(&mut owned, mutation)?;
    match evaluate_bend_hvm2_pilot(&owned.request()) {
        Ok(evaluation) => Some(
            evaluation
                .diagnostics
                .iter()
                .map(|item| item.boundary)
                .collect(),
        ),
        Err(diagnostics) => Some(diagnostics.iter().map(|item| item.boundary).collect()),
    }
}

fn apply_mutation(owned: &mut OwnedPilotRequest, mutation: &str) -> Option<()> {
    match mutation {
        "bend-version-drift" => owned.toolchain.bend_version = "0.2.39".to_string(),
        "hvm2-version-drift" => owned.toolchain.hvm2_version = "2.0.23".to_string(),
        "license-omission" => owned.bend_license_bytes.clear(),
        "proof-revision-drift" => {
            owned.proof_validation.trellis_revision = "drifted".to_string();
        }
        "proof-rerun-failure" => {
            owned.proof_validation.proof_result_present = false;
            owned.proof_validation.passed = false;
        }
        "input-mapping-drift" => set_leaves(owned, &[1, 1, 1, 1])?,
        "empty-input" => set_leaves(owned, &[])?,
        "non-power-of-two-input" => set_leaves(owned, &[1, 2, 3])?,
        "leaf-limit-exceeded" => set_leaves(owned, &[1, 2, 3, 4, 5, 6, 7, 8])?,
        "u24-domain-violation" => set_leaves(owned, &[U24_MODULUS, 1, 0, 1])?,
        "missing-annotation" => {
            owned.source_bytes = String::from_utf8(owned.source_bytes.clone())
                .ok()?
                .replace("def main() -> u24:", "def main():")
                .into_bytes();
        }
        "explicit-any" => owned.source_bytes.extend_from_slice(b"# Any\n"),
        "forbidden-authority" => owned.source_bytes.extend_from_slice(b"# IO/FS/load\n"),
        "malformed-generated-artifact" => owned.generated_c_bytes.clear(),
        "tool-failure" => {
            mutate_stage_outcome(owned, ProcessOutcome::ExitFailure, Some(FAILED_EXIT_CODE))?
        }
        "resource-exhaustion" => {
            mutate_stage_outcome(owned, ProcessOutcome::ResourceLimit, None)?;
        }
        "backend-disagreement" => {
            for observation in owned
                .observations
                .iter_mut()
                .filter(|observation| observation.stage == PilotStage::GeneratedC)
            {
                observation.result = Some(2);
            }
        }
        "unstable-measurements" => {
            let observation = owned.observations.iter_mut().find(|observation| {
                observation.stage == PilotStage::GeneratedC && !observation.warmup
            })?;
            observation.wall_nanoseconds = UNSTABLE_WALL_NANOSECONDS;
        }
        "semantic-overclaim" => {
            owned.proof_bundle_bytes = String::from_utf8(owned.proof_bundle_bytes.clone())
                .ok()?
                .replace(
                    "records local Trellis u24 arithmetic, refinement, equivalence, and abstract work proofs",
                    "proves Bend correctness",
                )
                .into_bytes();
        }
        _ => return None,
    }
    Some(())
}

fn set_leaves(owned: &mut OwnedPilotRequest, leaves: &[u64]) -> Option<()> {
    let input = BendHvm2LeafInput {
        schema_version: BEND_HVM2_INPUT_SCHEMA.to_string(),
        numeric_type: "u24".to_string(),
        leaves: leaves.to_vec(),
    };
    owned.input_bytes = serde_json::to_vec(&input).ok()?;
    Some(())
}

fn mutate_stage_outcome(
    owned: &mut OwnedPilotRequest,
    outcome: ProcessOutcome,
    exit_code: Option<i32>,
) -> Option<()> {
    let observation = owned
        .observations
        .iter_mut()
        .find(|observation| observation.stage == PilotStage::BendRunRs)?;
    observation.outcome = outcome;
    observation.exit_code = exit_code;
    Some(())
}

fn stale_receipt_boundary() -> Option<Vec<PilotBoundary>> {
    let owned = OwnedPilotRequest::valid()?;
    let evaluation = evaluate_bend_hvm2_pilot(&owned.request()).ok()?;
    let mut receipt = build_pilot_receipt(&evaluation, &owned.observations).ok()?;
    receipt.body.blockers.push("stale".to_string());
    validate_pilot_receipt(&evaluation, &owned.observations, &receipt)
        .err()
        .map(|_| vec![PilotBoundary::Receipt])
}

fn parse_boundary(text: &str) -> Option<PilotBoundary> {
    match text {
        "pre-execution" => Some(PilotBoundary::PreExecution),
        "execution" => Some(PilotBoundary::Execution),
        "post-output" => Some(PilotBoundary::PostOutput),
        "benchmark" => Some(PilotBoundary::Benchmark),
        "receipt" => Some(PilotBoundary::Receipt),
        _ => None,
    }
}
