use super::execution::ExecutionArtifacts;
use super::report::{PilotRunSummary, pretty_json};
use crate::root_io::RootCapability;
use kamacite_core::bend_hvm2::{
    BendHvm2PilotReceipt, BendHvm2ValenceProjection, ProofValidationObservation,
};
use std::path::{Path, PathBuf};

pub const DEFAULT_EVIDENCE_DIRECTORY: &str = "fixtures/bend-hvm2/golden";
const PROFILE_PATH: &str = "adapters/bend-hvm2/generated/profile.json";
const SOURCE_PATH: &str = "fixtures/bend-hvm2/workload.bend";
const INPUT_PATH: &str = "fixtures/bend-hvm2/input.json";
const BEND_LICENSE_PATH: &str = "adapters/bend-hvm2/upstream/bend/LICENSE-APACHE";
const HVM2_LICENSE_PATH: &str = "adapters/bend-hvm2/upstream/hvm2/LICENSE";
const PROOF_BUNDLE_PATH: &str = "adapters/bend-hvm2/upstream/trellis/proof-evidence-bundle.json";
const GENERATED_C_FILE: &str = "generated.c";
const EXECUTABLE_FILE: &str = "generated-executable.bin";
const COMPILER_INVOCATION_FILE: &str = "compiler-invocation.json";
const PROOF_VALIDATION_FILE: &str = "proof-validation.json";
const TOOLCHAIN_FILE: &str = "toolchain.json";
const OBSERVATIONS_FILE: &str = "observations.json";
const STREAMS_FILE: &str = "streams.json";
const RECEIPT_FILE: &str = "pilot-receipt.json";
const VALENCE_FILE: &str = "valence-projection.json";
const SUMMARY_FILE: &str = "summary.json";

pub struct RepositoryInputs {
    pub profile: Vec<u8>,
    pub source: Vec<u8>,
    pub input: Vec<u8>,
    pub bend_license: Vec<u8>,
    pub hvm2_license: Vec<u8>,
    pub proof_bundle: Vec<u8>,
}

pub struct EvidenceWriteRequest<'a> {
    pub directory: &'a Path,
    pub execution: &'a ExecutionArtifacts,
    pub proof_validation: &'a ProofValidationObservation,
    pub receipt: &'a BendHvm2PilotReceipt,
    pub valence: &'a BendHvm2ValenceProjection,
    pub summary: &'a PilotRunSummary,
}

impl RepositoryInputs {
    pub fn load(root: &RootCapability) -> Result<Self, String> {
        Ok(Self {
            profile: read(root, PROFILE_PATH)?,
            source: read(root, SOURCE_PATH)?,
            input: read(root, INPUT_PATH)?,
            bend_license: read(root, BEND_LICENSE_PATH)?,
            hvm2_license: read(root, HVM2_LICENSE_PATH)?,
            proof_bundle: read(root, PROOF_BUNDLE_PATH)?,
        })
    }
}

pub fn write_evidence(
    root: &RootCapability,
    request: &EvidenceWriteRequest<'_>,
) -> Result<(), String> {
    write_member(
        root,
        request.directory,
        GENERATED_C_FILE,
        &request.execution.generated_c,
    )?;
    write_member(
        root,
        request.directory,
        EXECUTABLE_FILE,
        &request.execution.executable,
    )?;
    write_member(
        root,
        request.directory,
        COMPILER_INVOCATION_FILE,
        &request.execution.compiler_invocation,
    )?;
    write_member(
        root,
        request.directory,
        PROOF_VALIDATION_FILE,
        &pretty_json(request.proof_validation)?,
    )?;
    write_member(
        root,
        request.directory,
        TOOLCHAIN_FILE,
        &pretty_json(&request.execution.toolchain)?,
    )?;
    write_member(
        root,
        request.directory,
        OBSERVATIONS_FILE,
        &pretty_json(&request.execution.observations)?,
    )?;
    write_member(
        root,
        request.directory,
        STREAMS_FILE,
        &pretty_json(&request.execution.captures)?,
    )?;
    write_member(
        root,
        request.directory,
        RECEIPT_FILE,
        &pretty_json(request.receipt)?,
    )?;
    write_member(
        root,
        request.directory,
        VALENCE_FILE,
        &pretty_json(request.valence)?,
    )?;
    write_member(
        root,
        request.directory,
        SUMMARY_FILE,
        &pretty_json(request.summary)?,
    )
}

fn read(root: &RootCapability, path: &str) -> Result<Vec<u8>, String> {
    root.read(Path::new(path))
        .map_err(|error| format!("read {path}: {error}"))
}

fn write_member(
    root: &RootCapability,
    directory: &Path,
    file: &str,
    bytes: &[u8],
) -> Result<(), String> {
    let path = relative_member(directory, file)?;
    root.write(&path, bytes)
        .map_err(|error| format!("write {}: {error}", path.display()))
}

fn relative_member(directory: &Path, file: &str) -> Result<PathBuf, String> {
    if directory.as_os_str().is_empty() || directory.is_absolute() {
        return Err("evidence directory must be a non-empty relative path".to_string());
    }
    Ok(directory.join(file))
}
