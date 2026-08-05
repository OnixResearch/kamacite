// r[impl kamacite.findings.nickel_sandbox.profile]
// r[impl kamacite.findings.nickel_sandbox.import_closure]
// r[impl kamacite.findings.nickel_sandbox.enforced_evidence]
// r[impl kamacite.findings.nickel_sandbox.resource_limits]
use kamacite_core::{
    IdentityDomain, NICKEL_SANDBOX_IMPORT_ROOT, NICKEL_SANDBOX_SOURCE_PATH, NickelSandboxEvidence,
    NickelSandboxOutcome, NickelSandboxProfile, WireIdentity, compute_identity,
    nickel_sandbox_observed_facts, validate_nickel_sandbox_profile,
};
use rustix::fs::{MemfdFlags, memfd_create};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::AsRawFd;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const SANDBOX_EVALUATOR_PATH: &str = "/evaluator";
const SANDBOX_DEV_PATH: &str = "/dev";
const SANDBOX_TMP_PATH: &str = "/tmp";
const POLL_INTERVAL_MILLISECONDS: u64 = 10;
const READ_CHUNK_BYTES: usize = 8_192;
const SIGNAL_EXIT_CODE_BASE: i32 = 128;
const RUNTIME_CLOSURE_ENVIRONMENT: &str = "KAMACITE_NICKEL_RUNTIME_CLOSURE_FILE";
const NIX_STORE_ROOT: &str = "/nix/store";
const NIX_STORE_QUERY_COMMAND: &str = "nix-store";
const NIX_STORE_QUERY_REQUISITES: &str = "-qR";
const SANDBOX_EXECUTABLE_NAME: &str = "bwrap";
const SANDBOX_STORE_OBJECT_MARKER: &str = "-bubblewrap-";
const LIMIT_EXECUTABLE_NAME: &str = "prlimit";
const LIMIT_STORE_OBJECT_MARKER: &str = "-util-linux-";
const NIX_BINARY_DIRECTORY_NAME: &str = "bin";
const TIMEOUT_DIAGNOSTIC: &[u8] = b"timeout";
const RESOURCE_LIMIT_DIAGNOSTIC: &[u8] = b"resource-limit";
const OUTPUT_LIMIT_DIAGNOSTIC: &[u8] = b"output-limit";
const DIAGNOSTIC_LIMIT_DIAGNOSTIC: &[u8] = b"diagnostic-limit";

pub struct NickelSandboxFile {
    pub sandbox_path: String,
    pub file: File,
    pub identity: WireIdentity,
}

pub struct NickelSandboxRequest {
    pub evaluator_command: PathBuf,
    pub evaluator_arguments: Vec<String>,
    pub evaluator_identity: WireIdentity,
    pub sandbox_command: PathBuf,
    pub limit_command: PathBuf,
    pub runtime_closure_paths: Vec<PathBuf>,
    pub source_file: File,
    pub source_identity: WireIdentity,
    pub imports: Vec<NickelSandboxFile>,
    pub requested_capabilities: Vec<String>,
    pub expected_effects: Vec<String>,
    pub profile: NickelSandboxProfile,
}

pub struct NickelSandboxRun {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub evidence: NickelSandboxEvidence,
}

pub fn resolve_command(command: &Path) -> Result<PathBuf, String> {
    if command.components().count() > 1 {
        return std::fs::canonicalize(command)
            .map_err(|error| format!("command {} is unavailable: {error}", command.display()));
    }
    let path = std::env::var_os("PATH").unwrap_or_default();
    for directory in std::env::split_paths(&path) {
        let candidate = directory.join(command);
        if candidate.is_file() {
            return std::fs::canonicalize(&candidate).map_err(|error| {
                format!(
                    "command {} cannot be resolved: {error}",
                    candidate.display()
                )
            });
        }
    }
    Err(format!("command {} is unavailable", command.display()))
}

pub fn validate_enforcement_command_paths(
    sandbox_command: &Path,
    limit_command: &Path,
) -> Result<(), String> {
    validate_nix_store_command(
        sandbox_command,
        SANDBOX_EXECUTABLE_NAME,
        SANDBOX_STORE_OBJECT_MARKER,
    )?;
    validate_nix_store_command(
        limit_command,
        LIMIT_EXECUTABLE_NAME,
        LIMIT_STORE_OBJECT_MARKER,
    )
}

pub fn runtime_closure_paths(evaluator_command: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = if let Some(path) = std::env::var_os(RUNTIME_CLOSURE_ENVIRONMENT) {
        read_runtime_closure_file(Path::new(&path))?
    } else {
        query_runtime_closure(evaluator_command)?
    };
    paths.sort();
    paths.dedup();
    validate_runtime_closure_paths(evaluator_command, &paths)?;
    Ok(paths)
}

pub fn inheritable_sandbox_data_file(name: &str, bytes: &[u8]) -> Result<File, String> {
    let mut file = memfd_create(name, MemfdFlags::empty())
        .map(File::from)
        .map_err(|error| format!("Nickel sandbox data memfd creation failed: {error}"))?;
    file.write_all(bytes)
        .map_err(|error| format!("Nickel sandbox data memfd write failed: {error}"))?;
    file.seek(SeekFrom::Start(0))
        .map_err(|error| format!("Nickel sandbox data memfd seek failed: {error}"))?;
    Ok(file)
}

pub fn run_nickel_sandbox(request: NickelSandboxRequest) -> Result<NickelSandboxRun, String> {
    let profile_diagnostics = validate_nickel_sandbox_profile(&request.profile);
    if !profile_diagnostics.is_empty() {
        return Err("Nickel sandbox request uses an unsupported profile".to_string());
    }
    validate_sandbox_import_paths(&request.imports)?;
    let mut stdout_file = capture_file("kamacite-nickel-stdout")?;
    let mut stderr_file = capture_file("kamacite-nickel-stderr")?;
    let stdout_child = stdout_file
        .try_clone()
        .map_err(|error| format!("Nickel stdout capture clone failed: {error}"))?;
    let stderr_child = stderr_file
        .try_clone()
        .map_err(|error| format!("Nickel stderr capture clone failed: {error}"))?;

    let maximum_file_bytes = request
        .profile
        .limits
        .output_limit_bytes
        .max(request.profile.limits.diagnostic_limit_bytes);
    let mut command = Command::new(&request.limit_command);
    command
        .arg(format!(
            "--as={}",
            request.profile.limits.memory_limit_bytes
        ))
        .arg(format!("--cpu={}", request.profile.limits.cpu_seconds))
        .arg(format!("--fsize={maximum_file_bytes}"))
        .arg("--")
        .arg(&request.sandbox_command);
    append_sandbox_arguments(&mut command, &request)?;
    command
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout_child))
        .stderr(Stdio::from(stderr_child));

    let launch = command.spawn();
    let mut child = match launch {
        Ok(child) => child,
        Err(error) => {
            let diagnostic = format!("Nickel sandbox launch failed: {error}").into_bytes();
            let evidence = sandbox_evidence(
                &request,
                NickelSandboxOutcome::LaunchFailure,
                None,
                &[],
                &diagnostic,
            );
            return Ok(NickelSandboxRun {
                stdout: Vec::new(),
                stderr: diagnostic,
                evidence,
            });
        }
    };

    let timeout = Duration::from_millis(request.profile.limits.execution_timeout_milliseconds);
    let deadline = Instant::now() + timeout;
    let mut timed_out = false;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(POLL_INTERVAL_MILLISECONDS));
            }
            Ok(None) => {
                timed_out = true;
                child
                    .kill()
                    .map_err(|error| format!("Nickel sandbox termination failed: {error}"))?;
                break child
                    .wait()
                    .map_err(|error| format!("Nickel sandbox reap failed: {error}"))?;
            }
            Err(error) => return Err(format!("Nickel sandbox wait failed: {error}")),
        }
    };

    let stdout_length = stdout_file
        .metadata()
        .map_err(|error| format!("Nickel stdout metadata failed: {error}"))?
        .len();
    let stderr_length = stderr_file
        .metadata()
        .map_err(|error| format!("Nickel stderr metadata failed: {error}"))?
        .len();
    let stdout = read_bounded(
        &mut stdout_file,
        request.profile.limits.output_limit_bytes,
        "stdout",
    )?;
    let stderr = read_bounded(
        &mut stderr_file,
        request.profile.limits.diagnostic_limit_bytes,
        "stderr",
    )?;
    let outcome = classify_outcome(
        timed_out,
        status.success(),
        status.code(),
        stdout_length,
        stderr_length,
        &request.profile,
    );
    let evidence = sandbox_evidence(&request, outcome, status.code(), &stdout, &stderr);
    Ok(NickelSandboxRun {
        stdout,
        stderr,
        evidence,
    })
}

fn validate_nix_store_command(
    command: &Path,
    executable_name: &str,
    store_object_marker: &str,
) -> Result<(), String> {
    let Some(file_name) = command.file_name().and_then(|name| name.to_str()) else {
        return Err(format!(
            "sandbox enforcement command {} has no executable name",
            command.display()
        ));
    };
    let binary_directory = command.parent().filter(|path| {
        path.file_name().and_then(|name| name.to_str()) == Some(NIX_BINARY_DIRECTORY_NAME)
    });
    let store_object = binary_directory
        .and_then(Path::parent)
        .filter(|path| path.parent() == Some(Path::new(NIX_STORE_ROOT)));
    let Some(store_object_name) = store_object
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
    else {
        return Err(format!(
            "sandbox enforcement command {} is not in a direct Nix store object",
            command.display()
        ));
    };
    if file_name != executable_name || !store_object_name.contains(store_object_marker) {
        return Err(format!(
            "sandbox enforcement command {} does not satisfy the admitted tool policy",
            command.display()
        ));
    }
    Ok(())
}

fn validate_runtime_closure_paths(
    evaluator_command: &Path,
    paths: &[PathBuf],
) -> Result<(), String> {
    validate_runtime_closure_shape(evaluator_command, paths)?;
    for path in paths {
        let canonical = std::fs::canonicalize(path).map_err(|error| {
            format!(
                "Nickel runtime closure path {} is unavailable: {error}",
                path.display()
            )
        })?;
        if canonical != *path {
            return Err(format!(
                "Nickel runtime closure path {} is not canonical",
                path.display()
            ));
        }
    }
    Ok(())
}

fn validate_runtime_closure_shape(
    evaluator_command: &Path,
    paths: &[PathBuf],
) -> Result<(), String> {
    if paths.is_empty() {
        return Err("Nickel sandbox runtime closure is empty".to_string());
    }
    let store_root = Path::new(NIX_STORE_ROOT);
    for path in paths {
        if path.parent() != Some(store_root) {
            return Err(format!(
                "Nickel runtime closure path {} is not a direct Nix store object",
                path.display()
            ));
        }
    }
    if !paths.iter().any(|path| evaluator_command.starts_with(path)) {
        return Err("Nickel runtime closure does not contain the evaluator".to_string());
    }
    Ok(())
}

fn read_runtime_closure_file(path: &Path) -> Result<Vec<PathBuf>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| {
        format!(
            "Nickel runtime closure file {} is unavailable: {error}",
            path.display()
        )
    })?;
    Ok(text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(PathBuf::from)
        .collect())
}

fn query_runtime_closure(evaluator_command: &Path) -> Result<Vec<PathBuf>, String> {
    let output = Command::new(NIX_STORE_QUERY_COMMAND)
        .arg(NIX_STORE_QUERY_REQUISITES)
        .arg(evaluator_command)
        .env_clear()
        .output()
        .map_err(|error| format!("Nickel runtime closure query failed: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "Nickel runtime closure query exited unsuccessfully: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|error| format!("Nickel runtime closure output is not UTF-8: {error}"))?;
    Ok(text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(PathBuf::from)
        .collect())
}

fn append_sandbox_arguments(
    command: &mut Command,
    request: &NickelSandboxRequest,
) -> Result<(), String> {
    command
        .arg("--unshare-all")
        .arg("--die-with-parent")
        .arg("--new-session")
        .arg("--clearenv");
    for path in &request.runtime_closure_paths {
        command.arg("--ro-bind").arg(path).arg(path);
    }
    command
        .arg("--dev")
        .arg(SANDBOX_DEV_PATH)
        .arg("--tmpfs")
        .arg(SANDBOX_TMP_PATH)
        .arg("--dir")
        .arg(NICKEL_SANDBOX_IMPORT_ROOT);
    for directory in sandbox_parent_directories(&request.imports)? {
        command.arg("--dir").arg(directory);
    }
    command
        .arg("--ro-bind")
        .arg(&request.evaluator_command)
        .arg(SANDBOX_EVALUATOR_PATH)
        .arg("--ro-bind-data")
        .arg(request.source_file.as_raw_fd().to_string())
        .arg(NICKEL_SANDBOX_SOURCE_PATH);
    for import in &request.imports {
        command
            .arg("--ro-bind-data")
            .arg(import.file.as_raw_fd().to_string())
            .arg(format!(
                "{NICKEL_SANDBOX_IMPORT_ROOT}/{}",
                import.sandbox_path
            ));
    }
    for (name, value) in &request.profile.environment {
        command.arg("--setenv").arg(name).arg(value);
    }
    command
        .arg("--chdir")
        .arg(NICKEL_SANDBOX_IMPORT_ROOT)
        .arg("--")
        .arg(SANDBOX_EVALUATOR_PATH)
        .args(&request.evaluator_arguments);
    Ok(())
}

fn validate_sandbox_import_paths(imports: &[NickelSandboxFile]) -> Result<(), String> {
    let mut paths = BTreeSet::new();
    for import in imports {
        let path = Path::new(&import.sandbox_path);
        if import.sandbox_path.is_empty()
            || path.components().any(|component| {
                matches!(
                    component,
                    Component::RootDir | Component::ParentDir | Component::Prefix(_)
                )
            })
        {
            return Err(format!(
                "Nickel import path {} escapes the reviewed closure",
                import.sandbox_path
            ));
        }
        if !paths.insert(import.sandbox_path.clone()) {
            return Err(format!(
                "Nickel import path {} is duplicated",
                import.sandbox_path
            ));
        }
    }
    Ok(())
}

fn sandbox_parent_directories(imports: &[NickelSandboxFile]) -> Result<Vec<OsString>, String> {
    let mut directories = BTreeSet::new();
    for import in imports {
        let mut current = PathBuf::from(NICKEL_SANDBOX_IMPORT_ROOT);
        let path = Path::new(&import.sandbox_path);
        if let Some(parent) = path.parent() {
            for component in parent.components() {
                let Component::Normal(component) = component else {
                    return Err(format!(
                        "Nickel import path {} has an invalid parent",
                        import.sandbox_path
                    ));
                };
                current.push(component);
                directories.insert(current.as_os_str().to_os_string());
            }
        }
    }
    Ok(directories.into_iter().collect())
}

fn capture_file(name: &str) -> Result<File, String> {
    memfd_create(name, MemfdFlags::empty())
        .map(File::from)
        .map_err(|error| format!("Nickel capture memfd creation failed: {error}"))
}

fn read_bounded(file: &mut File, limit: u64, label: &str) -> Result<Vec<u8>, String> {
    file.seek(SeekFrom::Start(0))
        .map_err(|error| format!("Nickel {label} capture seek failed: {error}"))?;
    let mut output = Vec::new();
    let mut bounded = file.take(limit);
    let mut chunk = [0_u8; READ_CHUNK_BYTES];
    loop {
        let read = bounded
            .read(&mut chunk)
            .map_err(|error| format!("Nickel {label} capture read failed: {error}"))?;
        if read == 0 {
            break;
        }
        output.extend_from_slice(&chunk[..read]);
    }
    Ok(output)
}

fn classify_outcome(
    timed_out: bool,
    success: bool,
    exit_code: Option<i32>,
    stdout_length: u64,
    stderr_length: u64,
    profile: &NickelSandboxProfile,
) -> NickelSandboxOutcome {
    if timed_out {
        return NickelSandboxOutcome::Timeout;
    }
    if stdout_length > profile.limits.output_limit_bytes
        || (!success && stdout_length == profile.limits.output_limit_bytes)
    {
        return NickelSandboxOutcome::OutputLimit;
    }
    if stderr_length > profile.limits.diagnostic_limit_bytes
        || (!success && stderr_length == profile.limits.diagnostic_limit_bytes)
    {
        return NickelSandboxOutcome::DiagnosticLimit;
    }
    if success {
        return NickelSandboxOutcome::Success;
    }
    if exit_code.is_none() || exit_code.is_some_and(|code| code >= SIGNAL_EXIT_CODE_BASE) {
        return NickelSandboxOutcome::ResourceLimit;
    }
    NickelSandboxOutcome::ExitFailure
}

fn sandbox_evidence(
    request: &NickelSandboxRequest,
    outcome: NickelSandboxOutcome,
    exit_code: Option<i32>,
    stdout: &[u8],
    stderr: &[u8],
) -> NickelSandboxEvidence {
    let mut import_identities = BTreeMap::new();
    import_identities.insert("source.ncl".to_string(), request.source_identity.clone());
    for import in &request.imports {
        import_identities.insert(import.sandbox_path.clone(), import.identity.clone());
    }
    let stdout_identity =
        (!stdout.is_empty()).then(|| compute_identity(IdentityDomain::Projection, stdout));
    let diagnostic_bytes = normalized_diagnostic_bytes(&outcome, stderr);
    NickelSandboxEvidence {
        profile_identity: request.profile.profile_identity.to_wire(),
        evaluator_identity: request.evaluator_identity.clone(),
        import_identities,
        requested_capabilities: request.requested_capabilities.clone(),
        admitted_capabilities: request.profile.admitted_capabilities.clone(),
        expected_effects: request.expected_effects.clone(),
        observed_execution_facts: nickel_sandbox_observed_facts(),
        outcome,
        exit_code,
        stdout_identity,
        stderr_identity: compute_identity(IdentityDomain::Projection, diagnostic_bytes),
        sandbox_enforced: true,
    }
}

fn normalized_diagnostic_bytes<'a>(outcome: &NickelSandboxOutcome, stderr: &'a [u8]) -> &'a [u8] {
    match outcome {
        NickelSandboxOutcome::Timeout => TIMEOUT_DIAGNOSTIC,
        NickelSandboxOutcome::ResourceLimit => RESOURCE_LIMIT_DIAGNOSTIC,
        NickelSandboxOutcome::OutputLimit => OUTPUT_LIMIT_DIAGNOSTIC,
        NickelSandboxOutcome::DiagnosticLimit => DIAGNOSTIC_LIMIT_DIAGNOSTIC,
        NickelSandboxOutcome::Success
        | NickelSandboxOutcome::ExitFailure
        | NickelSandboxOutcome::LaunchFailure => stderr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kamacite_core::nickel_sandbox_profile;

    #[test]
    fn admitted_bytes_materialize_as_rewound_inheritable_data() {
        const ADMITTED_BYTES: &[u8] = b"{ admitted = true }";

        let file = inheritable_sandbox_data_file("kamacite-test-source", ADMITTED_BYTES);
        assert!(file.is_ok());
        let Ok(mut file) = file else {
            return;
        };
        let mut observed = Vec::new();
        assert!(file.read_to_end(&mut observed).is_ok());
        assert_eq!(observed, ADMITTED_BYTES);
    }

    #[test]
    fn import_path_policy_accepts_relative_paths_and_rejects_escape() {
        let source = File::open("Cargo.toml");
        assert!(source.is_ok());
        let Ok(source) = source else {
            return;
        };
        let accepted = NickelSandboxFile {
            sandbox_path: "imports/accepted.ncl".to_string(),
            file: source,
            identity: compute_identity(IdentityDomain::Source, b"accepted"),
        };
        assert!(validate_sandbox_import_paths(&[accepted]).is_ok());

        let source = File::open("Cargo.toml");
        assert!(source.is_ok());
        let Ok(source) = source else {
            return;
        };
        let escaped = NickelSandboxFile {
            sandbox_path: "../escaped.ncl".to_string(),
            file: source,
            identity: compute_identity(IdentityDomain::Source, b"escaped"),
        };
        assert!(validate_sandbox_import_paths(&[escaped]).is_err());
    }

    #[test]
    fn enforcement_command_policy_accepts_nix_tools_and_rejects_substitution() {
        const SANDBOX_COMMAND: &str =
            "/nix/store/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-bubblewrap-0.11.2/bin/bwrap";
        const LIMIT_COMMAND: &str =
            "/nix/store/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb-util-linux-2.42.2-bin/bin/prlimit";
        const AMBIENT_SANDBOX_COMMAND: &str = "/tmp/bwrap";

        assert!(
            validate_enforcement_command_paths(
                Path::new(SANDBOX_COMMAND),
                Path::new(LIMIT_COMMAND),
            )
            .is_ok()
        );
        assert!(
            validate_enforcement_command_paths(
                Path::new(AMBIENT_SANDBOX_COMMAND),
                Path::new(LIMIT_COMMAND),
            )
            .is_err()
        );
        assert!(
            validate_enforcement_command_paths(
                Path::new(LIMIT_COMMAND),
                Path::new(SANDBOX_COMMAND),
            )
            .is_err()
        );
    }

    #[test]
    fn runtime_closure_shape_accepts_store_objects_and_rejects_ambient_paths() {
        const STORE_OBJECT: &str = "/nix/store/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-nickel";
        const STORE_EVALUATOR: &str =
            "/nix/store/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-nickel/bin/nickel";
        const AMBIENT_PATH: &str = "/tmp/ambient-runtime";

        let accepted = vec![PathBuf::from(STORE_OBJECT)];
        assert!(validate_runtime_closure_shape(Path::new(STORE_EVALUATOR), &accepted).is_ok());

        let ambient = vec![PathBuf::from(AMBIENT_PATH)];
        assert!(validate_runtime_closure_shape(Path::new(STORE_EVALUATOR), &ambient).is_err());
        assert!(validate_runtime_closure_shape(Path::new(STORE_EVALUATOR), &[]).is_err());
    }

    #[test]
    fn resource_diagnostics_normalize_nondeterministic_process_text() {
        const FIRST_PROCESS_DIAGNOSTIC: &[u8] = b"allocation failed at address one";
        const SECOND_PROCESS_DIAGNOSTIC: &[u8] = b"allocation failed at address two";

        assert_eq!(
            normalized_diagnostic_bytes(
                &NickelSandboxOutcome::ResourceLimit,
                FIRST_PROCESS_DIAGNOSTIC,
            ),
            normalized_diagnostic_bytes(
                &NickelSandboxOutcome::ResourceLimit,
                SECOND_PROCESS_DIAGNOSTIC,
            )
        );
        assert_ne!(
            normalized_diagnostic_bytes(
                &NickelSandboxOutcome::ExitFailure,
                FIRST_PROCESS_DIAGNOSTIC,
            ),
            normalized_diagnostic_bytes(
                &NickelSandboxOutcome::ExitFailure,
                SECOND_PROCESS_DIAGNOSTIC,
            )
        );
    }

    #[test]
    fn outcome_classification_is_fail_closed() {
        let profile = nickel_sandbox_profile();
        assert!(profile.is_ok());
        let Ok(profile) = profile else {
            return;
        };
        assert_eq!(
            classify_outcome(true, false, None, 0, 0, &profile),
            NickelSandboxOutcome::Timeout
        );
        assert_eq!(
            classify_outcome(
                false,
                false,
                None,
                profile.limits.output_limit_bytes + 1,
                0,
                &profile,
            ),
            NickelSandboxOutcome::OutputLimit
        );
        assert_eq!(
            classify_outcome(
                false,
                false,
                None,
                0,
                profile.limits.diagnostic_limit_bytes + 1,
                &profile,
            ),
            NickelSandboxOutcome::DiagnosticLimit
        );
        assert_eq!(
            classify_outcome(false, false, None, 0, 0, &profile),
            NickelSandboxOutcome::ResourceLimit
        );
        assert_eq!(
            classify_outcome(false, false, Some(SIGNAL_EXIT_CODE_BASE), 0, 0, &profile,),
            NickelSandboxOutcome::ResourceLimit
        );
    }
}
