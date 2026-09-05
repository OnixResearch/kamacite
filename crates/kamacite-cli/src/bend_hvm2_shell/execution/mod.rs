mod routes;
mod scratch;
mod toolchain;

use super::process::{
    BoundedProcessRequest, BoundedProcessRun, load_runtime_closure, resolve_enforcement_commands,
    run_bounded_process,
};
use crate::nickel_sandbox::resolve_command;
use crate::root_io::RootCapability;
use kamacite_core::bend_hvm2::{
    BendHvm2Profile, PilotStage, StageObservation, ToolchainObservation,
};
use kamacite_core::canonical_json_bytes;
use routes::{run_bend, run_generated, run_oracle, stage_capture, stage_observation};
use scratch::prepare_scratch;
use serde::Serialize;
use std::path::{Path, PathBuf};
use toolchain::observe_toolchain;

const SANDBOX_WORKLOAD: &str = "/work/workload.bend";
const SANDBOX_GENERATED_C: &str = "/work/generated.c";
const SANDBOX_GENERATED_EXECUTABLE: &str = "/work/generated";
const SCRATCH_GENERATED_C: &str = "generated.c";
const SCRATCH_EXECUTABLE: &str = "generated";
const SUCCESS_EXIT_CODE: i32 = 0;
const PILOT_OBSERVATION_CAPACITY: usize = 20;

#[derive(Clone, Debug)]
pub struct ToolPaths {
    pub bend: PathBuf,
    pub hvm2: PathBuf,
    pub compiler: PathBuf,
    pub oracle: PathBuf,
    pub sandbox: PathBuf,
    pub limit: PathBuf,
    pub runtime_closure_file: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub struct StageCapture {
    pub stage: PilotStage,
    pub sample_index: usize,
    pub warmup: bool,
    pub stdout: String,
    pub stderr: String,
}

pub struct ExecutionArtifacts {
    pub toolchain: ToolchainObservation,
    pub observations: Vec<StageObservation>,
    pub captures: Vec<StageCapture>,
    pub generated_c: Vec<u8>,
    pub executable: Vec<u8>,
    pub compiler_invocation: Vec<u8>,
}

pub(super) struct ExecutionContext<'a> {
    pub(super) profile: &'a BendHvm2Profile,
    pub(super) scratch_root: &'a Path,
    pub(super) bend: PathBuf,
    pub(super) hvm2: PathBuf,
    pub(super) compiler: PathBuf,
    pub(super) oracle: PathBuf,
    pub(super) sandbox: PathBuf,
    pub(super) limit: PathBuf,
    pub(super) runtime_closure: Vec<PathBuf>,
}

#[derive(Serialize)]
struct CompilerInvocation<'a> {
    executable: &'a str,
    arguments: &'a [String],
    compiler_family: &'a str,
    compiler_version: &'a str,
    target: &'a str,
}

pub fn execute_profile<F>(
    profile: &BendHvm2Profile,
    scratch_root: &Path,
    source: &[u8],
    input: &[u8],
    paths: &ToolPaths,
    pre_execution: F,
) -> Result<ExecutionArtifacts, String>
where
    F: FnOnce(&ToolchainObservation) -> Result<(), String>,
{
    let scratch_root = std::fs::canonicalize(scratch_root)
        .map_err(|error| format!("scratch root is unavailable: {error}"))?;
    prepare_scratch(&scratch_root, source, input)?;
    let context = ExecutionContext::resolve(profile, &scratch_root, paths)?;
    let toolchain = observe_toolchain(&context)?;
    pre_execution(&toolchain)?;
    let mut observations = Vec::with_capacity(PILOT_OBSERVATION_CAPACITY);
    let mut captures = Vec::with_capacity(PILOT_OBSERVATION_CAPACITY);

    let generation_arguments = vec![
        "--hvm-bin".to_string(),
        path_text(&context.hvm2)?,
        "gen-c".to_string(),
        SANDBOX_WORKLOAD.to_string(),
    ];
    let generation = context.run(&context.bend, &generation_arguments)?;
    observations.push(stage_observation(
        profile,
        PilotStage::BendGenC,
        0,
        false,
        &generation,
        None,
    )?);
    captures.push(stage_capture(PilotStage::BendGenC, 0, false, &generation)?);
    require_success(&generation, "bend gen-c")?;
    let scratch = RootCapability::open(&scratch_root)
        .map_err(|error| format!("open scratch capability: {error}"))?;
    scratch
        .write(Path::new(SCRATCH_GENERATED_C), &generation.stdout)
        .map_err(|error| format!("write generated C: {error}"))?;

    let compiler_arguments = vec![
        SANDBOX_GENERATED_C.to_string(),
        "-o".to_string(),
        SANDBOX_GENERATED_EXECUTABLE.to_string(),
        "-O2".to_string(),
        "-lm".to_string(),
        "-lpthread".to_string(),
    ];
    let compilation = context.run(&context.compiler, &compiler_arguments)?;
    observations.push(stage_observation(
        profile,
        PilotStage::CCompile,
        0,
        false,
        &compilation,
        None,
    )?);
    captures.push(stage_capture(PilotStage::CCompile, 0, false, &compilation)?);
    require_success(&compilation, "C compilation")?;
    let executable = scratch
        .read(Path::new(SCRATCH_EXECUTABLE))
        .map_err(|error| format!("read generated executable: {error}"))?;
    let compiler_path = path_text(&context.compiler)?;
    let compiler_invocation = canonical_json_bytes(&CompilerInvocation {
        executable: &compiler_path,
        arguments: &compiler_arguments,
        compiler_family: &toolchain.compiler_family,
        compiler_version: &toolchain.compiler_version,
        target: &toolchain.compiler_target,
    })
    .map_err(|error| format!("encode compiler invocation: {error}"))?;

    let route_count = profile
        .benchmark
        .warmup_runs
        .checked_add(profile.benchmark.measured_runs)
        .ok_or_else(|| "benchmark repetition count overflow".to_string())?;
    for sample_index in 0..route_count {
        let warmup = sample_index < profile.benchmark.warmup_runs;
        for result in [
            run_oracle(&context, sample_index, warmup),
            run_bend(&context, sample_index, warmup),
            run_generated(&context, sample_index, warmup),
        ] {
            let (observation, capture) = result?;
            observations.push(observation);
            captures.push(capture);
        }
    }
    Ok(ExecutionArtifacts {
        toolchain,
        observations,
        captures,
        generated_c: generation.stdout,
        executable,
        compiler_invocation,
    })
}

impl<'a> ExecutionContext<'a> {
    fn resolve(
        profile: &'a BendHvm2Profile,
        scratch_root: &'a Path,
        paths: &ToolPaths,
    ) -> Result<Self, String> {
        let bend = resolve_command(&paths.bend)?;
        let hvm2 = resolve_command(&paths.hvm2)?;
        let compiler = resolve_command(&paths.compiler)?;
        let oracle = resolve_command(&paths.oracle)?;
        let (sandbox, limit) = resolve_enforcement_commands(&paths.sandbox, &paths.limit)?;
        let required = vec![bend.clone(), hvm2.clone(), compiler.clone(), oracle.clone()];
        let runtime_closure = load_runtime_closure(&paths.runtime_closure_file, &required)?;
        Ok(Self {
            profile,
            scratch_root,
            bend,
            hvm2,
            compiler,
            oracle,
            sandbox,
            limit,
            runtime_closure,
        })
    }

    pub(super) fn run(
        &self,
        executable: &Path,
        arguments: &[String],
    ) -> Result<BoundedProcessRun, String> {
        run_bounded_process(&BoundedProcessRequest {
            executable,
            arguments,
            scratch_root: self.scratch_root,
            runtime_closure: &self.runtime_closure,
            sandbox_command: &self.sandbox,
            limit_command: &self.limit,
            resources: &self.profile.resources,
        })
    }
}

pub(super) fn require_success(run: &BoundedProcessRun, label: &str) -> Result<(), String> {
    if run.outcome == kamacite_core::bend_hvm2::ProcessOutcome::Success
        && run.exit_code == Some(SUCCESS_EXIT_CODE)
    {
        Ok(())
    } else {
        Err(format!(
            "{label} failed with {:?}: {}",
            run.outcome,
            String::from_utf8_lossy(&run.stderr).trim()
        ))
    }
}

pub(super) fn path_text(path: &Path) -> Result<String, String> {
    path.to_str()
        .map(str::to_string)
        .ok_or_else(|| format!("tool path {} is not UTF-8", path.display()))
}
