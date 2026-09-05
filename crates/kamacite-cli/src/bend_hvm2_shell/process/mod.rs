#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::ambient_clock,
        tigerstyle::ambient_env,
        tigerstyle::ambiguous_params,
        tigerstyle::function_length,
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        tigerstyle::path_segment_repetition,
        tigerstyle::too_many_parameters,
        tigerstyle::unbounded_collection_growth,
        reason = "This bounded process adapter records explicit shell-owned clock, environment, stream, and resource observations."
    )
)]

mod streams;

use crate::nickel_sandbox::{resolve_command, validate_enforcement_command_paths};
use kamacite_core::bend_hvm2::{ProcessOutcome, ResourceProfile};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use streams::{capture_file, classify_outcome, elapsed_nanoseconds, read_bounded, stream_length};

const SANDBOX_WORK_PATH: &str = "/work";
const SANDBOX_TEMPORARY_PATH: &str = "/tmp";
const SANDBOX_DEVICE_PATH: &str = "/dev";
const SANDBOX_PROCESS_PATH: &str = "/proc";
const SANDBOX_HOME_PATH: &str = "/homeless";
const POLL_INTERVAL_MILLISECONDS: u64 = 10;
const NIX_STORE_ROOT: &str = "/nix/store";
const GENERATED_EXECUTABLE: &str = "/work/generated";

pub struct BoundedProcessRequest<'a> {
    pub executable: &'a Path,
    pub arguments: &'a [String],
    pub scratch_root: &'a Path,
    pub runtime_closure: &'a [PathBuf],
    pub sandbox_command: &'a Path,
    pub limit_command: &'a Path,
    pub resources: &'a ResourceProfile,
}

pub struct BoundedProcessRun {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub outcome: ProcessOutcome,
    pub exit_code: Option<i32>,
    pub wall_nanoseconds: u64,
    pub sandbox_enforced: bool,
}

pub fn resolve_enforcement_commands(
    sandbox: &Path,
    limit: &Path,
) -> Result<(PathBuf, PathBuf), String> {
    let sandbox = resolve_command(sandbox)?;
    let limit = resolve_command(limit)?;
    validate_enforcement_command_paths(&sandbox, &limit)?;
    Ok((sandbox, limit))
}

pub fn load_runtime_closure(
    closure_file: &Path,
    required_commands: &[PathBuf],
) -> Result<Vec<PathBuf>, String> {
    let text = std::fs::read_to_string(closure_file).map_err(|error| {
        format!(
            "Bend/HVM2 runtime closure file {} is unavailable: {error}",
            closure_file.display()
        )
    })?;
    let mut paths = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    if paths.is_empty() {
        return Err("Bend/HVM2 runtime closure is empty".to_string());
    }
    for path in &paths {
        if path.parent() != Some(Path::new(NIX_STORE_ROOT)) {
            return Err(format!(
                "runtime closure path {} is not a direct Nix store object",
                path.display()
            ));
        }
        let canonical = std::fs::canonicalize(path).map_err(|error| {
            format!(
                "runtime closure path {} is unavailable: {error}",
                path.display()
            )
        })?;
        if canonical != *path {
            return Err(format!(
                "runtime closure path {} is not canonical",
                path.display()
            ));
        }
    }
    for command in required_commands {
        if command != Path::new(GENERATED_EXECUTABLE)
            && !paths.iter().any(|path| command.starts_with(path))
        {
            return Err(format!(
                "runtime closure does not contain command {}",
                command.display()
            ));
        }
    }
    Ok(paths)
}

pub fn run_bounded_process(
    request: &BoundedProcessRequest<'_>,
) -> Result<BoundedProcessRun, String> {
    validate_scratch(request.scratch_root)?;
    let mut stdout_file = capture_file("kamacite-bend-hvm2-stdout")?;
    let mut stderr_file = capture_file("kamacite-bend-hvm2-stderr")?;
    let stdout_child = stdout_file
        .try_clone()
        .map_err(|error| format!("stdout capture clone failed: {error}"))?;
    let stderr_child = stderr_file
        .try_clone()
        .map_err(|error| format!("stderr capture clone failed: {error}"))?;
    let maximum_file_bytes = request
        .resources
        .file_limit_bytes
        .max(request.resources.output_limit_bytes)
        .max(request.resources.diagnostic_limit_bytes);
    let mut command = Command::new(request.limit_command);
    command
        .arg(format!("--as={}", request.resources.memory_limit_bytes))
        .arg(format!("--cpu={}", request.resources.cpu_seconds))
        .arg(format!("--fsize={maximum_file_bytes}"))
        .arg(format!("--nproc={}", request.resources.process_limit))
        .arg("--")
        .arg(request.sandbox_command);
    append_sandbox_arguments(&mut command, request);
    command
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout_child))
        .stderr(Stdio::from(stderr_child));
    let start = Instant::now();
    let launch = command.spawn();
    let mut child = match launch {
        Ok(child) => child,
        Err(error) => {
            return Ok(BoundedProcessRun {
                stdout: Vec::new(),
                stderr: format!("bounded process launch failed: {error}").into_bytes(),
                outcome: ProcessOutcome::LaunchFailure,
                exit_code: None,
                wall_nanoseconds: elapsed_nanoseconds(start)?,
                sandbox_enforced: false,
            });
        }
    };
    let timeout = Duration::from_millis(request.resources.timeout_milliseconds);
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
                    .map_err(|error| format!("bounded process termination failed: {error}"))?;
                break child
                    .wait()
                    .map_err(|error| format!("bounded process reap failed: {error}"))?;
            }
            Err(error) => return Err(format!("bounded process wait failed: {error}")),
        }
    };
    let wall_nanoseconds = elapsed_nanoseconds(start)?;
    let stdout_length = stream_length(&stdout_file, "stdout")?;
    let stderr_length = stream_length(&stderr_file, "stderr")?;
    let stdout = read_bounded(
        &mut stdout_file,
        request.resources.output_limit_bytes,
        "stdout",
    )?;
    let stderr = read_bounded(
        &mut stderr_file,
        request.resources.diagnostic_limit_bytes,
        "stderr",
    )?;
    let outcome = classify_outcome(
        timed_out,
        status.success(),
        status.code(),
        stdout_length,
        stderr_length,
        request.resources,
    );
    Ok(BoundedProcessRun {
        stdout,
        stderr,
        outcome,
        exit_code: status.code(),
        wall_nanoseconds,
        sandbox_enforced: true,
    })
}

fn append_sandbox_arguments(command: &mut Command, request: &BoundedProcessRequest<'_>) {
    command
        .arg("--unshare-all")
        .arg("--die-with-parent")
        .arg("--new-session")
        .arg("--clearenv");
    for path in request.runtime_closure {
        command.arg("--ro-bind").arg(path).arg(path);
    }
    command
        .arg("--dev")
        .arg(SANDBOX_DEVICE_PATH)
        .arg("--proc")
        .arg(SANDBOX_PROCESS_PATH)
        .arg("--tmpfs")
        .arg(SANDBOX_TEMPORARY_PATH)
        .arg("--dir")
        .arg(SANDBOX_HOME_PATH)
        .arg("--bind")
        .arg(request.scratch_root)
        .arg(SANDBOX_WORK_PATH)
        .arg("--setenv")
        .arg("HOME")
        .arg(SANDBOX_HOME_PATH)
        .arg("--setenv")
        .arg("LANG")
        .arg("C")
        .arg("--setenv")
        .arg("LC_ALL")
        .arg("C")
        .arg("--setenv")
        .arg("TZ")
        .arg("UTC")
        .arg("--chdir")
        .arg(SANDBOX_WORK_PATH)
        .arg("--")
        .arg(request.executable)
        .args(request.arguments);
}

fn validate_scratch(path: &Path) -> Result<(), String> {
    let canonical = std::fs::canonicalize(path)
        .map_err(|error| format!("scratch root {} is unavailable: {error}", path.display()))?;
    if canonical != path {
        return Err("scratch root must be an absolute canonical path".to_string());
    }
    let metadata = std::fs::metadata(path)
        .map_err(|error| format!("scratch root metadata failed: {error}"))?;
    if metadata.is_dir() {
        Ok(())
    } else {
        Err("scratch root is not a directory".to_string())
    }
}
