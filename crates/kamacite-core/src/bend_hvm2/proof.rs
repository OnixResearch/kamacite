use super::model::{
    BendHvm2Diagnostic, BendHvm2Profile, PilotBoundary, ProofValidationObservation,
    TRELLIS_BUNDLE_BLAKE3, TRELLIS_REVISION, TRELLIS_VERIFIER_PROVIDER_REVISION,
};
use super::profile::diagnostic;
use super::proof_record::{self, TrellisBundle};
use crate::{EvidenceChainIdentity, IdentityDomain, canonical_json_bytes};

pub fn admit_proof_handoff(
    profile: &BendHvm2Profile,
    bundle_bytes: &[u8],
    observation: &ProofValidationObservation,
) -> Result<EvidenceChainIdentity, Vec<BendHvm2Diagnostic>> {
    let mut diagnostics = Vec::new();
    let observed_bundle = blake3::hash(bundle_bytes).to_hex().to_string();
    if observed_bundle != TRELLIS_BUNDLE_BLAKE3
        || profile.trellis.proof_bundle_blake3 != TRELLIS_BUNDLE_BLAKE3
    {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "proof-bundle-drift",
            "trellis.proof_bundle",
            "Trellis proof bundle identity differs from the reviewed artifact",
        ));
    }
    let bundle = match serde_json::from_slice::<TrellisBundle>(bundle_bytes) {
        Ok(bundle) => bundle,
        Err(error) => {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "proof-bundle-decode",
                "trellis.proof_bundle",
                format!("Trellis proof bundle decoding failed: {error}"),
            ));
            return Err(diagnostics);
        }
    };
    proof_record::validate(profile, &bundle, &mut diagnostics);
    validate_observation(profile, observation, &mut diagnostics);
    let identity = canonical_json_bytes(observation)
        .map(|bytes| EvidenceChainIdentity::compute(&bytes))
        .map_err(|error| {
            vec![diagnostic(
                PilotBoundary::PreExecution,
                "proof-observation-encoding",
                "trellis.proof_validation",
                error.to_string(),
            )]
        })?;
    diagnostics.sort();
    diagnostics.dedup();
    if diagnostics.is_empty() {
        Ok(identity)
    } else {
        Err(diagnostics)
    }
}

fn validate_observation(
    profile: &BendHvm2Profile,
    observation: &ProofValidationObservation,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    for (field, actual, expected) in [
        (
            "trellis_revision",
            observation.trellis_revision.as_str(),
            TRELLIS_REVISION,
        ),
        (
            "verifier_provider_revision",
            observation.verifier_provider_revision.as_str(),
            TRELLIS_VERIFIER_PROVIDER_REVISION,
        ),
        (
            "proof_check",
            observation.proof_check.as_str(),
            profile.trellis.proof_check.as_str(),
        ),
    ] {
        if actual != expected {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "proof-rerun-drift",
                format!("trellis.proof_validation.{field}"),
                format!("expected `{expected}`, observed `{actual}`"),
            ));
        }
    }
    if !observation.proof_result_present
        || !observation.passed
        || !observation.diagnostics.is_empty()
    {
        diagnostics.push(proof_diagnostic(
            "proof-rerun-failed",
            "pinned Trellis proof validation did not produce a clean passing observation",
        ));
    }
    if observation
        .proof_result_identity
        .validate_domain(IdentityDomain::EvidenceChain)
        .is_err()
    {
        diagnostics.push(proof_diagnostic(
            "proof-result-identity",
            "proof result identity uses the wrong domain",
        ));
    }
    if observation
        .verifier_identity
        .validate_domain(IdentityDomain::Compiler)
        .is_err()
    {
        diagnostics.push(proof_diagnostic(
            "verifier-identity",
            "verifier identity uses the wrong domain",
        ));
    }
}

fn proof_diagnostic(code: &str, message: &str) -> BendHvm2Diagnostic {
    diagnostic(
        PilotBoundary::PreExecution,
        code,
        "trellis.proof_handoff",
        message,
    )
}
