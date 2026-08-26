use std::path::Path;

use kamacite_cli::nickel_sandbox::{
    NickelSandboxFile, NickelSandboxRequest, inheritable_sandbox_data_file, resolve_command,
    run_nickel_sandbox, runtime_closure_paths, validate_enforcement_command_paths,
};
use kamacite_cli::root_io::RootCapability;
use kamacite_core::{
    AdapterIdentity, EvaluatorIdentity, IdentityDomain, NICKEL_SANDBOX_SOURCE_PATH,
    NickelSandboxOutcome, ProjectionIdentity, SourceIdentity, compute_identity,
    nickel_sandbox_evidence_identity, nickel_sandbox_profile, validate_nickel_sandbox,
};

use crate::world_commit_shell::io::read_bounded;

const NICKEL_COMMAND: &str = "nickel";
const SANDBOX_COMMAND: &str = "bwrap";
const LIMIT_COMMAND: &str = "prlimit";
const VERSION_ARGUMENT: &str = "--version";
const EXPORT_ARGUMENT: &str = "export";
const SOURCE_MEMFD_NAME: &str = "kamacite-world-commit-nickel-source";
const IMPORT_MEMFD_NAME: &str = "kamacite-world-commit-nickel-import";

#[derive(Clone, Debug)]
pub struct NickelExport {
    pub output: Vec<u8>,
    pub evaluator_identity: EvaluatorIdentity,
    pub enforcement_identity: AdapterIdentity,
    pub observed_effects: Vec<String>,
}

pub fn sandboxed_nickel_export(
    root: &RootCapability,
    source_path: &str,
    import_path: &str,
    evaluator_profile: &str,
) -> Result<NickelExport, String> {
    let evaluator_command = resolve_command(Path::new(NICKEL_COMMAND))?;
    let sandbox_command = resolve_command(Path::new(SANDBOX_COMMAND))?;
    let limit_command = resolve_command(Path::new(LIMIT_COMMAND))?;
    validate_enforcement_command_paths(&sandbox_command, &limit_command)?;
    let runtime_closure_paths = runtime_closure_paths(&evaluator_command)?;
    let profile = nickel_sandbox_profile()
        .map_err(|error| format!("construct Nickel sandbox profile: {error}"))?;
    let capabilities = profile.admitted_capabilities.clone();
    let source_bytes = read_bounded(root, source_path)?;
    let import_bytes = read_bounded(root, import_path)?;
    let bootstrap_identity = compute_identity(
        IdentityDomain::Evaluator,
        evaluator_command.to_string_lossy().as_bytes(),
    );
    let version = run_nickel_sandbox(NickelSandboxRequest {
        evaluator_command: evaluator_command.clone(),
        evaluator_arguments: vec![VERSION_ARGUMENT.to_string()],
        evaluator_identity: bootstrap_identity,
        sandbox_command: sandbox_command.clone(),
        limit_command: limit_command.clone(),
        runtime_closure_paths: runtime_closure_paths.clone(),
        source_file: inheritable_sandbox_data_file(SOURCE_MEMFD_NAME, &source_bytes)?,
        source_identity: SourceIdentity::compute(&source_bytes).to_wire(),
        imports: vec![sandbox_import(import_path, &import_bytes)?],
        requested_capabilities: capabilities.clone(),
        expected_effects: capabilities.clone(),
        profile: profile.clone(),
    })?;
    require_success("version", &version.evidence.outcome, &version.stderr)?;
    let version_text = std::str::from_utf8(&version.stdout)
        .map_err(|error| format!("Nickel version is not UTF-8: {error}"))?
        .trim();
    let evaluator_descriptor = format!("{evaluator_profile};version={version_text}");
    let evaluator_identity = EvaluatorIdentity::compute(evaluator_descriptor.as_bytes());
    let run = run_nickel_sandbox(NickelSandboxRequest {
        evaluator_command,
        evaluator_arguments: vec![
            EXPORT_ARGUMENT.to_string(),
            NICKEL_SANDBOX_SOURCE_PATH.to_string(),
        ],
        evaluator_identity: evaluator_identity.to_wire(),
        sandbox_command,
        limit_command,
        runtime_closure_paths,
        source_file: inheritable_sandbox_data_file(SOURCE_MEMFD_NAME, &source_bytes)?,
        source_identity: SourceIdentity::compute(&source_bytes).to_wire(),
        imports: vec![sandbox_import(import_path, &import_bytes)?],
        requested_capabilities: capabilities.clone(),
        expected_effects: capabilities.clone(),
        profile: profile.clone(),
    })?;
    require_success("export", &run.evidence.outcome, &run.stderr)?;
    let projection_identity = ProjectionIdentity::compute(&run.stdout);
    let projection_wire = projection_identity.to_wire();
    let evaluator_wire = evaluator_identity.to_wire();
    let diagnostics = validate_nickel_sandbox(
        &profile,
        &run.evidence,
        &capabilities,
        Some(&projection_wire),
        Some(&evaluator_wire),
    );
    if !diagnostics.is_empty() {
        return Err(format!(
            "Nickel sandbox evidence failed validation: {diagnostics:?}"
        ));
    }
    let enforcement_identity = nickel_sandbox_evidence_identity(&run.evidence)
        .map_err(|error| format!("identify Nickel sandbox evidence: {error}"))?;
    Ok(NickelExport {
        output: run.stdout,
        evaluator_identity,
        enforcement_identity,
        observed_effects: capabilities,
    })
}

fn sandbox_import(path: &str, bytes: &[u8]) -> Result<NickelSandboxFile, String> {
    let sandbox_path = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("Nickel import has no UTF-8 file name: {path}"))?
        .to_string();
    Ok(NickelSandboxFile {
        sandbox_path,
        file: inheritable_sandbox_data_file(IMPORT_MEMFD_NAME, bytes)?,
        identity: SourceIdentity::compute(bytes).to_wire(),
    })
}

fn require_success(
    operation: &str,
    outcome: &NickelSandboxOutcome,
    stderr: &[u8],
) -> Result<(), String> {
    if outcome != &NickelSandboxOutcome::Success {
        return Err(format!(
            "sandboxed Nickel {operation} failed with {outcome:?}: {}",
            String::from_utf8_lossy(stderr).trim()
        ));
    }
    Ok(())
}
