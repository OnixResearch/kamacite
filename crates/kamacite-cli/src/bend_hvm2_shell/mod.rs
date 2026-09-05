#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::ambiguous_params,
        tigerstyle::assertion_density,
        tigerstyle::bool_naming,
        tigerstyle::function_length,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        tigerstyle::too_many_parameters,
        reason = "This imperative shell composes one fixed bounded process workflow and returns typed errors instead of assertion panics."
    )
)]

mod execution;
mod io;
mod process;
mod report;

pub use execution::ToolPaths;
pub use io::DEFAULT_EVIDENCE_DIRECTORY;

use crate::root_io::RootCapability;
use execution::{ExecutionArtifacts, execute_profile};
use io::{EvidenceWriteRequest, RepositoryInputs, write_evidence};
use kamacite_core::bend_hvm2::{
    BendHvm2Profile, PilotEvaluationRequest, PilotPreExecutionRequest, ProofValidationObservation,
    TRELLIS_REVISION, TRELLIS_VERIFIER_PROVIDER_REVISION, admit_bend_hvm2_pre_execution,
    build_pilot_receipt, evaluate_bend_hvm2_pilot, project_pilot_receipt_to_valence,
};
use kamacite_core::{CompilerIdentity, EvidenceChainIdentity};
use report::{PilotRunSummary, pretty_json};
use std::path::{Path, PathBuf};

const REVIEWED_PROFILE: &[u8] =
    include_bytes!("../../../../adapters/bend-hvm2/generated/profile.json");
const MAXIMUM_VERIFIER_IDENTITY_BYTES: u64 = 65_536;

#[derive(Clone, Debug)]
pub struct RunOptions {
    pub root: PathBuf,
    pub scratch_root: PathBuf,
    pub tools: ToolPaths,
    pub proof_result: PathBuf,
    pub verifier_identity_file: PathBuf,
    pub update_evidence: bool,
    pub evidence_directory: PathBuf,
}

// r[impl kamacite.bend_hvm2_pilot.functional_core]
// r[impl kamacite.bend_hvm2_pilot.evidence]
// r[impl kamacite.bend_hvm2_pilot.promotion]
pub fn run(options: &RunOptions) -> Result<Vec<u8>, String> {
    let root = RootCapability::open(&options.root)
        .map_err(|error| format!("open repository root: {error}"))?;
    let inputs = RepositoryInputs::load(&root)?;
    if inputs.profile != REVIEWED_PROFILE {
        return Err("Bend/HVM2 profile differs from the compiled reviewed profile".to_string());
    }
    let profile = serde_json::from_slice::<BendHvm2Profile>(&inputs.profile)
        .map_err(|error| format!("decode Bend/HVM2 profile: {error}"))?;
    let proof_validation = proof_validation_observation(
        &profile,
        &options.proof_result,
        &options.verifier_identity_file,
    )?;
    let execution = execute_profile(
        &profile,
        &options.scratch_root,
        &inputs.source,
        &inputs.input,
        &options.tools,
        |toolchain| {
            admit_bend_hvm2_pre_execution(&PilotPreExecutionRequest {
                profile_bytes: &inputs.profile,
                source_bytes: &inputs.source,
                canonical_input_bytes: &inputs.input,
                bend_license_bytes: &inputs.bend_license,
                hvm2_license_bytes: &inputs.hvm2_license,
                proof_bundle_bytes: &inputs.proof_bundle,
                proof_validation: &proof_validation,
                toolchain,
            })
            .map(|_| ())
            .map_err(|diagnostics| format_diagnostics("pre-execution denied", &diagnostics))
        },
    )?;
    finish_run(options, &root, &inputs, &proof_validation, &execution)
}

fn finish_run(
    options: &RunOptions,
    root: &RootCapability,
    inputs: &RepositoryInputs,
    proof_validation: &ProofValidationObservation,
    execution: &ExecutionArtifacts,
) -> Result<Vec<u8>, String> {
    let evaluation = evaluate_bend_hvm2_pilot(&PilotEvaluationRequest {
        profile_bytes: &inputs.profile,
        source_bytes: &inputs.source,
        canonical_input_bytes: &inputs.input,
        bend_license_bytes: &inputs.bend_license,
        hvm2_license_bytes: &inputs.hvm2_license,
        proof_bundle_bytes: &inputs.proof_bundle,
        proof_validation,
        toolchain: &execution.toolchain,
        observations: &execution.observations,
        generated_c_bytes: &execution.generated_c,
        compiler_invocation_bytes: &execution.compiler_invocation,
        executable_bytes: &execution.executable,
    })
    .map_err(|diagnostics| format_diagnostics("pilot evaluation denied", &diagnostics))?;
    let receipt = build_pilot_receipt(&evaluation, &execution.observations)
        .map_err(|error| format!("build pilot receipt: {error}"))?;
    let valence = project_pilot_receipt_to_valence(&receipt)
        .map_err(|error| format!("project pilot receipt: {error}"))?;
    let summary = PilotRunSummary::new(&evaluation, &receipt, &valence);
    if options.update_evidence {
        write_evidence(
            root,
            &EvidenceWriteRequest {
                directory: &options.evidence_directory,
                execution,
                proof_validation,
                receipt: &receipt,
                valence: &valence,
                summary: &summary,
            },
        )?;
    }
    pretty_json(&summary)
}

fn proof_validation_observation(
    profile: &BendHvm2Profile,
    proof_result: &Path,
    verifier_identity_file: &Path,
) -> Result<ProofValidationObservation, String> {
    let proof_result = std::fs::canonicalize(proof_result)
        .map_err(|error| format!("Trellis proof result is unavailable: {error}"))?;
    let metadata = std::fs::metadata(&proof_result)
        .map_err(|error| format!("Trellis proof result metadata failed: {error}"))?;
    let proof_result_present = metadata.is_file();
    let verifier_metadata = std::fs::metadata(verifier_identity_file)
        .map_err(|error| format!("verifier identity metadata failed: {error}"))?;
    if !verifier_metadata.is_file() || verifier_metadata.len() > MAXIMUM_VERIFIER_IDENTITY_BYTES {
        return Err("verifier identity file is absent or exceeds its byte limit".to_string());
    }
    let verifier_bytes = std::fs::read(verifier_identity_file)
        .map_err(|error| format!("read verifier identity: {error}"))?;
    if verifier_bytes.is_empty() {
        return Err("verifier identity file is empty".to_string());
    }
    let proof_result_text = proof_result
        .to_str()
        .ok_or_else(|| "Trellis proof result path is not UTF-8".to_string())?;
    let mut diagnostics = Vec::new();
    if !proof_result_present {
        diagnostics.push("Trellis proof result is not a regular file".to_string());
    }
    Ok(ProofValidationObservation {
        trellis_revision: TRELLIS_REVISION.to_string(),
        verifier_provider_revision: TRELLIS_VERIFIER_PROVIDER_REVISION.to_string(),
        proof_check: profile.trellis.proof_check.clone(),
        proof_result_present,
        proof_result_identity: EvidenceChainIdentity::compute(proof_result_text.as_bytes())
            .to_wire(),
        verifier_identity: CompilerIdentity::compute(&verifier_bytes).to_wire(),
        passed: proof_result_present && diagnostics.is_empty(),
        diagnostics,
    })
}

fn format_diagnostics<T: serde::Serialize>(label: &str, diagnostics: &T) -> String {
    match serde_json::to_string(diagnostics) {
        Ok(json) => format!("{label}: {json}"),
        Err(error) => format!("{label}: diagnostic encoding failed: {error}"),
    }
}
