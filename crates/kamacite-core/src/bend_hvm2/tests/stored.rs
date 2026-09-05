use super::*;
use crate::ProjectionIdentity;
use serde::Deserialize;

const GENERATED_C: &[u8] = include_bytes!("../../../../../fixtures/bend-hvm2/golden/generated.c");
const EXECUTABLE: &[u8] =
    include_bytes!("../../../../../fixtures/bend-hvm2/golden/generated-executable.bin");
const COMPILER_INVOCATION: &[u8] =
    include_bytes!("../../../../../fixtures/bend-hvm2/golden/compiler-invocation.json");
const PROOF_VALIDATION: &[u8] =
    include_bytes!("../../../../../fixtures/bend-hvm2/golden/proof-validation.json");
const TOOLCHAIN: &[u8] = include_bytes!("../../../../../fixtures/bend-hvm2/golden/toolchain.json");
const OBSERVATIONS: &[u8] =
    include_bytes!("../../../../../fixtures/bend-hvm2/golden/observations.json");
const STREAMS: &[u8] = include_bytes!("../../../../../fixtures/bend-hvm2/golden/streams.json");
const RECEIPT: &[u8] =
    include_bytes!("../../../../../fixtures/bend-hvm2/golden/pilot-receipt.json");
const VALENCE: &[u8] =
    include_bytes!("../../../../../fixtures/bend-hvm2/golden/valence-projection.json");

#[derive(Deserialize)]
struct StoredCapture {
    stage: PilotStage,
    sample_index: usize,
    warmup: bool,
    stdout: String,
    stderr: String,
}

// r[verify kamacite.bend_hvm2_pilot.evidence]
// r[verify kamacite.bend_hvm2_pilot.validation]
#[test]
fn retained_defer_evidence_revalidates_from_exact_artifacts() {
    let proof = serde_json::from_slice::<ProofValidationObservation>(PROOF_VALIDATION);
    let toolchain = serde_json::from_slice::<ToolchainObservation>(TOOLCHAIN);
    let observations = serde_json::from_slice::<Vec<StageObservation>>(OBSERVATIONS);
    let receipt = serde_json::from_slice::<BendHvm2PilotReceipt>(RECEIPT);
    let valence = serde_json::from_slice::<BendHvm2ValenceProjection>(VALENCE);
    let captures = serde_json::from_slice::<Vec<StoredCapture>>(STREAMS);
    assert!(proof.is_ok());
    assert!(toolchain.is_ok());
    assert!(observations.is_ok());
    assert!(receipt.is_ok());
    assert!(valence.is_ok());
    assert!(captures.is_ok());
    let (Ok(proof), Ok(toolchain), Ok(observations), Ok(receipt), Ok(valence), Ok(captures)) =
        (proof, toolchain, observations, receipt, valence, captures)
    else {
        return;
    };
    assert_stream_identities(&observations, &captures);
    let evaluation = evaluate_bend_hvm2_pilot(&PilotEvaluationRequest {
        profile_bytes: PROFILE_BYTES,
        source_bytes: SOURCE_BYTES,
        canonical_input_bytes: INPUT_BYTES,
        bend_license_bytes: BEND_LICENSE_BYTES,
        hvm2_license_bytes: HVM2_LICENSE_BYTES,
        proof_bundle_bytes: PROOF_BUNDLE_BYTES,
        proof_validation: &proof,
        toolchain: &toolchain,
        observations: &observations,
        generated_c_bytes: GENERATED_C,
        compiler_invocation_bytes: COMPILER_INVOCATION,
        executable_bytes: EXECUTABLE,
    });
    assert!(evaluation.is_ok());
    let Ok(evaluation) = evaluation else {
        return;
    };
    assert_eq!(evaluation.decision, PilotDecision::Defer);
    assert!(evaluation.benchmark.is_none());
    assert!(evaluation.diagnostics.iter().any(|item| {
        item.code == "hvm2-address-space-profile" && item.message.contains("10737418240 bytes")
    }));
    assert!(validate_pilot_receipt(&evaluation, &observations, &receipt).is_ok());
    let expected_valence = project_pilot_receipt_to_valence(&receipt);
    assert_eq!(expected_valence.ok(), Some(valence));
}

fn assert_stream_identities(observations: &[StageObservation], captures: &[StoredCapture]) {
    assert_eq!(observations.len(), captures.len());
    for observation in observations {
        let capture = captures.iter().find(|capture| {
            capture.stage == observation.stage
                && capture.sample_index == observation.sample_index
                && capture.warmup == observation.warmup
        });
        assert!(capture.is_some());
        let Some(capture) = capture else {
            continue;
        };
        assert_eq!(
            ProjectionIdentity::compute(capture.stdout.as_bytes()),
            observation.stdout_identity
        );
        assert_eq!(
            ProjectionIdentity::compute(capture.stderr.as_bytes()),
            observation.stderr_identity
        );
    }
}
