use kamacite_core::WireIdentity;
use kamacite_core::bend_hvm2::{
    BendHvm2Diagnostic, BendHvm2PilotReceipt, BendHvm2ValenceProjection, PilotDecision,
    PilotEvaluation,
};
use serde::Serialize;

pub const RUN_SUMMARY_SCHEMA: &str = "kamacite.bend-hvm2-pilot-run.v1";

#[derive(Debug, Serialize)]
pub struct PilotRunSummary {
    pub schema_version: &'static str,
    pub decision: PilotDecision,
    pub blockers: Vec<String>,
    pub diagnostics: Vec<BendHvm2Diagnostic>,
    pub profile_identity: WireIdentity,
    pub bend_source_identity: WireIdentity,
    pub canonical_input_identity: WireIdentity,
    pub proof_bundle_identity: WireIdentity,
    pub proof_validation_identity: WireIdentity,
    pub generated_c_identity: WireIdentity,
    pub compiler_invocation_identity: WireIdentity,
    pub executable_identity: WireIdentity,
    pub receipt_identity: WireIdentity,
    pub valence_projection_identity: WireIdentity,
    pub observation_count: usize,
    pub benchmark_present: bool,
    pub non_claims: Vec<String>,
}

impl PilotRunSummary {
    pub fn new(
        evaluation: &PilotEvaluation,
        receipt: &BendHvm2PilotReceipt,
        projection: &BendHvm2ValenceProjection,
    ) -> Self {
        Self {
            schema_version: RUN_SUMMARY_SCHEMA,
            decision: evaluation.decision,
            blockers: evaluation.blockers.clone(),
            diagnostics: evaluation.diagnostics.clone(),
            profile_identity: evaluation.identities.profile.to_wire(),
            bend_source_identity: evaluation.identities.bend_source.to_wire(),
            canonical_input_identity: evaluation.identities.canonical_input.to_wire(),
            proof_bundle_identity: evaluation.identities.proof_bundle.to_wire(),
            proof_validation_identity: evaluation.identities.proof_validation.to_wire(),
            generated_c_identity: evaluation.identities.generated_c.to_wire(),
            compiler_invocation_identity: evaluation.identities.compiler_invocation.to_wire(),
            executable_identity: evaluation.identities.executable.to_wire(),
            receipt_identity: receipt.receipt_identity.to_wire(),
            valence_projection_identity: projection.projection_identity.to_wire(),
            observation_count: receipt.body.observations.len(),
            benchmark_present: receipt.body.benchmark.is_some(),
            non_claims: evaluation.non_claims.clone(),
        }
    }
}

pub fn pretty_json<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}
