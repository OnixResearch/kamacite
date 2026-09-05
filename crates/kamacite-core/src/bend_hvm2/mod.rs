#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::ambiguous_params,
        tigerstyle::assertion_density,
        tigerstyle::borrowed_argument_types,
        tigerstyle::fragile_exhaustive_enum_match,
        tigerstyle::missing_const_fn,
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        tigerstyle::path_segment_repetition,
        reason = "This component facade composes pure admission and evidence functions that return diagnostics instead of assertion panics."
    )
)]

mod model;
mod observation;
mod profile;
mod proof;
#[path = "proof/record.rs"]
mod proof_record;
mod receipt;
mod source;

pub use model::{
    BEND_HVM2_PROFILE_ID, BEND_HVM2_PROFILE_SCHEMA, BEND_HVM2_RECEIPT_SCHEMA,
    BEND_HVM2_VALENCE_SCHEMA, BEND_REVISION, BEND_VERSION, BenchmarkPolicy, BenchmarkSummary,
    BendHvm2Diagnostic, BendHvm2PilotReceipt, BendHvm2PilotReceiptBody, BendHvm2Profile,
    BendHvm2ValenceArtifact, BendHvm2ValenceClass, BendHvm2ValenceProjection, CohortProfile,
    CommandProfile, CompilerProfile, HVM2_REVISION, HVM2_VERSION, PILOT_LEAF_LIMIT,
    PILOT_MEASURED_RUNS, PILOT_RESULT, PILOT_WARMUP_RUNS, PilotArtifactIdentities, PilotBoundary,
    PilotDecision, PilotEvaluation, PilotEvaluationRequest, PilotStage, ProcessOutcome,
    ProofValidationObservation, ResourceProfile, SourcePin, StageObservation,
    TRELLIS_ALGORITHM_PROFILE, TRELLIS_ORACLE_SYMBOL, TRELLIS_REVISION,
    TRELLIS_VERIFIER_PROVIDER_REVISION, ToolchainObservation, TrellisProfile, U24_MAXIMUM,
    U24_MODULUS, WorkloadProfile,
};
pub use observation::{
    ParsedRouteOutput, command_profile_identity, parse_route_output, validate_observations,
};
pub use receipt::{build_pilot_receipt, project_pilot_receipt_to_valence, validate_pilot_receipt};
pub use source::{
    AdmittedBendHvm2Workload, BEND_HVM2_INPUT_SCHEMA, BendHvm2LeafInput,
    canonical_leaf_input_bytes, decode_leaf_input, render_bend_workload,
};

use crate::{
    AdapterIdentity, CompilerIdentity, EvidenceChainIdentity, SourceIdentity,
    TargetArtifactIdentity,
};
use observation::evaluate_observations;
use profile::{admit_profile, diagnostic};
use proof::admit_proof_handoff;
use source::admit_workload;

pub const HVM2_STATIC_ADDRESS_SPACE_LOWER_BOUND_BYTES: u64 = 10_737_418_240;
const HVM2_STATIC_LAYOUT_TOKENS: &[&[u8]] = &[
    b"#define TPC_L2 5",
    b"#define RLEN (1ul << 24)",
    b"#define G_NODE_LEN (1ul << 29)",
    b"#define G_VARS_LEN (1ul << 29)",
    b"APair node_buf[G_NODE_LEN]",
    b"APort vars_buf[G_VARS_LEN]",
    b"APair rbag_buf[G_RBAG_LEN]",
];

pub struct PilotPreExecutionRequest<'a> {
    pub profile_bytes: &'a [u8],
    pub source_bytes: &'a [u8],
    pub canonical_input_bytes: &'a [u8],
    pub bend_license_bytes: &'a [u8],
    pub hvm2_license_bytes: &'a [u8],
    pub proof_bundle_bytes: &'a [u8],
    pub proof_validation: &'a ProofValidationObservation,
    pub toolchain: &'a ToolchainObservation,
}

pub struct PilotPreExecutionAdmission {
    pub profile: BendHvm2Profile,
    pub workload: AdmittedBendHvm2Workload,
    pub proof_validation_identity: EvidenceChainIdentity,
}

// r[impl kamacite.bend_hvm2_pilot.boundary]
// r[impl kamacite.bend_hvm2_pilot.workload]
// r[impl kamacite.bend_hvm2_pilot.trellis_oracle_evidence]
// r[impl kamacite.bend_hvm2_pilot.functional_core]
pub fn admit_bend_hvm2_pre_execution(
    request: &PilotPreExecutionRequest<'_>,
) -> Result<PilotPreExecutionAdmission, Vec<BendHvm2Diagnostic>> {
    let profile = admit_profile(
        request.profile_bytes,
        request.bend_license_bytes,
        request.hvm2_license_bytes,
        request.toolchain,
    )?;
    let workload = admit_workload(
        &profile,
        request.source_bytes,
        request.canonical_input_bytes,
    )?;
    let proof_validation_identity = admit_proof_handoff(
        &profile,
        request.proof_bundle_bytes,
        request.proof_validation,
    )?;
    Ok(PilotPreExecutionAdmission {
        profile,
        workload,
        proof_validation_identity,
    })
}

// r[impl kamacite.bend_hvm2_pilot.oracle]
// r[impl kamacite.bend_hvm2_pilot.evidence]
// r[impl kamacite.bend_hvm2_pilot.benchmark]
// r[impl kamacite.bend_hvm2_pilot.promotion]
pub fn evaluate_bend_hvm2_pilot(
    request: &PilotEvaluationRequest<'_>,
) -> Result<PilotEvaluation, Vec<BendHvm2Diagnostic>> {
    let admission = admit_bend_hvm2_pre_execution(&PilotPreExecutionRequest {
        profile_bytes: request.profile_bytes,
        source_bytes: request.source_bytes,
        canonical_input_bytes: request.canonical_input_bytes,
        bend_license_bytes: request.bend_license_bytes,
        hvm2_license_bytes: request.hvm2_license_bytes,
        proof_bundle_bytes: request.proof_bundle_bytes,
        proof_validation: request.proof_validation,
        toolchain: request.toolchain,
    })?;
    validate_artifacts(&admission.profile, request)?;
    let identities = PilotArtifactIdentities {
        profile: AdapterIdentity::compute(request.profile_bytes),
        bend_source: SourceIdentity::compute(request.source_bytes),
        canonical_input: admission.workload.canonical_identity,
        proof_bundle: SourceIdentity::compute(request.proof_bundle_bytes),
        proof_validation: admission.proof_validation_identity,
        generated_c: TargetArtifactIdentity::compute(request.generated_c_bytes),
        compiler_invocation: CompilerIdentity::compute(request.compiler_invocation_bytes),
        executable: TargetArtifactIdentity::compute(request.executable_bytes),
    };
    let (benchmark, mut diagnostics) =
        evaluate_observations(&admission.profile, request.observations);
    diagnostics.extend(generated_c_resource_diagnostics(
        &admission.profile,
        request.generated_c_bytes,
    ));
    diagnostics.sort();
    diagnostics.dedup();
    let decision = classify_decision(&diagnostics);
    let blockers = blockers(&diagnostics);
    Ok(PilotEvaluation {
        decision,
        diagnostics,
        blockers,
        identities,
        benchmark,
        checked_laws: admission.profile.checked_laws,
        non_claims: admission.profile.non_claims,
    })
}

fn validate_artifacts(
    profile: &BendHvm2Profile,
    request: &PilotEvaluationRequest<'_>,
) -> Result<(), Vec<BendHvm2Diagnostic>> {
    let mut diagnostics = Vec::new();
    validate_artifact_bytes(
        request.generated_c_bytes,
        profile.resources.artifact_limit_bytes,
        "generated_c",
        &mut diagnostics,
    );
    validate_artifact_bytes(
        request.executable_bytes,
        profile.resources.artifact_limit_bytes,
        "executable",
        &mut diagnostics,
    );
    if request.compiler_invocation_bytes.is_empty() {
        diagnostics.push(diagnostic(
            PilotBoundary::PostOutput,
            "compiler-invocation-missing",
            "compiler_invocation",
            "compiler invocation evidence is empty",
        ));
    }
    diagnostics.sort();
    diagnostics.dedup();
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn validate_artifact_bytes(
    bytes: &[u8],
    limit_bytes: u64,
    field: &str,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let observed = u64::try_from(bytes.len()).ok();
    if bytes.is_empty() || observed.is_none_or(|length_bytes| length_bytes > limit_bytes) {
        diagnostics.push(diagnostic(
            PilotBoundary::PostOutput,
            "artifact-limit",
            field,
            "artifact is empty or exceeds the declared byte limit",
        ));
    }
}

fn generated_c_resource_diagnostics(
    profile: &BendHvm2Profile,
    generated_c: &[u8],
) -> Vec<BendHvm2Diagnostic> {
    let has_exact_layout = HVM2_STATIC_LAYOUT_TOKENS.iter().all(|token| {
        generated_c
            .windows(token.len())
            .any(|window| window == *token)
    });
    if has_exact_layout
        && profile.resources.memory_limit_bytes < HVM2_STATIC_ADDRESS_SPACE_LOWER_BOUND_BYTES
    {
        vec![diagnostic(
            PilotBoundary::Execution,
            "hvm2-address-space-profile",
            "generated_c.static_layout",
            format!(
                "generated HVM2 C reserves at least {HVM2_STATIC_ADDRESS_SPACE_LOWER_BOUND_BYTES} bytes, above the declared address-space limit"
            ),
        )]
    } else {
        Vec::new()
    }
}

fn classify_decision(diagnostics: &[BendHvm2Diagnostic]) -> PilotDecision {
    if diagnostics.is_empty() {
        return PilotDecision::Adopt;
    }
    if diagnostics.iter().any(|item| {
        item.boundary == PilotBoundary::PostOutput
            && matches!(
                item.code.as_str(),
                "backend-disagreement" | "oracle-result" | "bend-result"
            )
    }) {
        PilotDecision::Reject
    } else {
        PilotDecision::Defer
    }
}

fn blockers(diagnostics: &[BendHvm2Diagnostic]) -> Vec<String> {
    let mut blockers = diagnostics
        .iter()
        .map(|item| format!("{}:{}", item.code, item.field))
        .collect::<Vec<_>>();
    blockers.sort();
    blockers.dedup();
    blockers
}

#[cfg(test)]
mod tests;
