#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::ambiguous_params,
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "Receipt construction is total over admitted data and preserves explicit public identity roles without assertion panics."
    )
)]

use super::model::{
    BEND_HVM2_RECEIPT_ROOT, BEND_HVM2_RECEIPT_SCHEMA, BEND_HVM2_VALENCE_SCHEMA,
    BendHvm2PilotReceipt, BendHvm2PilotReceiptBody, BendHvm2ValenceArtifact, BendHvm2ValenceClass,
    BendHvm2ValenceProjection, PilotEvaluation, StageObservation,
};
use crate::{
    AnyIdentity, PreservesLimits, PreservesSchema, ProjectionIdentity, ReceiptError,
    ReceiptIdentity, encode_typed_preserves,
};
use serde::Serialize;

const STREAM_ARTIFACTS_PER_OBSERVATION: usize = 2;

#[derive(Serialize)]
struct ValenceProjectionBody<'a> {
    schema_version: &'static str,
    subject_receipt_identity: &'a ReceiptIdentity,
    artifacts: &'a [BendHvm2ValenceArtifact],
    checked_laws: &'a [String],
    non_claims: &'a [String],
}

pub fn build_pilot_receipt(
    evaluation: &PilotEvaluation,
    observations: &[StageObservation],
) -> Result<BendHvm2PilotReceipt, ReceiptError> {
    let mut observations = observations.to_vec();
    observations.sort_by_key(|observation| (observation.stage, observation.sample_index));
    let body = BendHvm2PilotReceiptBody {
        schema_version: BEND_HVM2_RECEIPT_SCHEMA.to_string(),
        decision: evaluation.decision,
        blockers: evaluation.blockers.clone(),
        diagnostics: evaluation.diagnostics.clone(),
        profile_identity: evaluation.identities.profile.clone(),
        bend_source_identity: evaluation.identities.bend_source.clone(),
        canonical_input_identity: evaluation.identities.canonical_input.clone(),
        proof_bundle_identity: evaluation.identities.proof_bundle.clone(),
        proof_validation_identity: evaluation.identities.proof_validation.clone(),
        generated_c_identity: evaluation.identities.generated_c.clone(),
        compiler_invocation_identity: evaluation.identities.compiler_invocation.clone(),
        executable_identity: evaluation.identities.executable.clone(),
        observations,
        benchmark: evaluation.benchmark.clone(),
        checked_laws: evaluation.checked_laws.clone(),
        non_claims: evaluation.non_claims.clone(),
    };
    let bytes = receipt_bytes(&body)?;
    Ok(BendHvm2PilotReceipt {
        body,
        receipt_identity: ReceiptIdentity::compute(&bytes),
    })
}

pub fn validate_pilot_receipt(
    evaluation: &PilotEvaluation,
    observations: &[StageObservation],
    receipt: &BendHvm2PilotReceipt,
) -> Result<(), String> {
    let expected =
        build_pilot_receipt(evaluation, observations).map_err(|error| error.to_string())?;
    if receipt == &expected {
        Ok(())
    } else {
        Err("Bend/HVM2 pilot receipt differs from exact evaluated facts".to_string())
    }
}

pub fn project_pilot_receipt_to_valence(
    receipt: &BendHvm2PilotReceipt,
) -> Result<BendHvm2ValenceProjection, ReceiptError> {
    let mut artifacts = fixed_artifacts(receipt);
    artifacts.reserve(
        receipt
            .body
            .observations
            .len()
            .saturating_mul(STREAM_ARTIFACTS_PER_OBSERVATION),
    );
    for observation in &receipt.body.observations {
        artifacts.push(BendHvm2ValenceArtifact {
            role: format!(
                "{:?}-sample-{}-stdout",
                observation.stage, observation.sample_index
            )
            .to_ascii_lowercase(),
            identity: AnyIdentity::Projection(observation.stdout_identity.clone()),
            verification_class: BendHvm2ValenceClass::RecordedOnly,
        });
        artifacts.push(BendHvm2ValenceArtifact {
            role: format!(
                "{:?}-sample-{}-stderr",
                observation.stage, observation.sample_index
            )
            .to_ascii_lowercase(),
            identity: AnyIdentity::Projection(observation.stderr_identity.clone()),
            verification_class: BendHvm2ValenceClass::RecordedOnly,
        });
    }
    artifacts.sort_by(|left, right| left.role.cmp(&right.role));
    let body = ValenceProjectionBody {
        schema_version: BEND_HVM2_VALENCE_SCHEMA,
        subject_receipt_identity: &receipt.receipt_identity,
        artifacts: &artifacts,
        checked_laws: &receipt.body.checked_laws,
        non_claims: &receipt.body.non_claims,
    };
    let projection_bytes =
        serde_json::to_vec(&body).map_err(|error| ReceiptError::Serialize(error.to_string()))?;
    Ok(BendHvm2ValenceProjection {
        schema_version: BEND_HVM2_VALENCE_SCHEMA.to_string(),
        subject_receipt_identity: receipt.receipt_identity.clone(),
        artifacts,
        checked_laws: receipt.body.checked_laws.clone(),
        non_claims: receipt.body.non_claims.clone(),
        projection_identity: ProjectionIdentity::compute(&projection_bytes),
    })
}

fn fixed_artifacts(receipt: &BendHvm2PilotReceipt) -> Vec<BendHvm2ValenceArtifact> {
    vec![
        artifact(
            "adapter-profile",
            AnyIdentity::Adapter(receipt.body.profile_identity.clone()),
            BendHvm2ValenceClass::Boundary,
        ),
        artifact(
            "bend-source-envelope",
            AnyIdentity::Source(receipt.body.bend_source_identity.clone()),
            BendHvm2ValenceClass::RecordedOnly,
        ),
        artifact(
            "canonical-leaf-input",
            AnyIdentity::CanonicalValue(receipt.body.canonical_input_identity.clone()),
            BendHvm2ValenceClass::Boundary,
        ),
        artifact(
            "trellis-proof-bundle",
            AnyIdentity::Source(receipt.body.proof_bundle_identity.clone()),
            BendHvm2ValenceClass::RecordedOnly,
        ),
        artifact(
            "trellis-proof-validation",
            AnyIdentity::EvidenceChain(receipt.body.proof_validation_identity.clone()),
            BendHvm2ValenceClass::VerifiedTrellisLocal,
        ),
        artifact(
            "generated-c-source",
            AnyIdentity::TargetArtifact(receipt.body.generated_c_identity.clone()),
            BendHvm2ValenceClass::RecordedOnly,
        ),
        artifact(
            "compiler-invocation",
            AnyIdentity::Compiler(receipt.body.compiler_invocation_identity.clone()),
            BendHvm2ValenceClass::Boundary,
        ),
        artifact(
            "generated-executable",
            AnyIdentity::TargetArtifact(receipt.body.executable_identity.clone()),
            BendHvm2ValenceClass::RecordedOnly,
        ),
        artifact(
            "pilot-decision",
            AnyIdentity::Receipt(receipt.receipt_identity.clone()),
            BendHvm2ValenceClass::Boundary,
        ),
    ]
}

fn artifact(
    role: &str,
    identity: AnyIdentity,
    verification_class: BendHvm2ValenceClass,
) -> BendHvm2ValenceArtifact {
    BendHvm2ValenceArtifact {
        role: role.to_string(),
        identity,
        verification_class,
    }
}

fn receipt_bytes(body: &BendHvm2PilotReceiptBody) -> Result<Vec<u8>, ReceiptError> {
    encode_typed_preserves(
        body,
        PreservesSchema {
            schema_version: BEND_HVM2_RECEIPT_SCHEMA,
            root_label: BEND_HVM2_RECEIPT_ROOT,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ReceiptError::Serialize(error.to_string()))
}
