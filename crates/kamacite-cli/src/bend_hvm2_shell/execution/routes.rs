use super::{
    ExecutionContext, SANDBOX_GENERATED_EXECUTABLE, SANDBOX_WORKLOAD, StageCapture, path_text,
};
use crate::bend_hvm2_shell::process::BoundedProcessRun;
use kamacite_core::ProjectionIdentity;
use kamacite_core::bend_hvm2::{
    BendHvm2Profile, ParsedRouteOutput, PilotStage, ProcessOutcome, StageObservation,
    command_profile_identity, parse_route_output,
};
use std::path::Path;

pub(super) fn run_oracle(
    context: &ExecutionContext<'_>,
    sample_index: usize,
    warmup: bool,
) -> Result<(StageObservation, StageCapture), String> {
    let arguments = context
        .profile
        .workload
        .leaves
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>();
    let run = context.run(&context.oracle, &arguments)?;
    route_result(
        context.profile,
        PilotStage::TrellisOracle,
        sample_index,
        warmup,
        &run,
    )
}

pub(super) fn run_bend(
    context: &ExecutionContext<'_>,
    sample_index: usize,
    warmup: bool,
) -> Result<(StageObservation, StageCapture), String> {
    let arguments = vec![
        "--hvm-bin".to_string(),
        path_text(&context.hvm2)?,
        "run-rs".to_string(),
        SANDBOX_WORKLOAD.to_string(),
        "-s".to_string(),
    ];
    let run = context.run(&context.bend, &arguments)?;
    route_result(
        context.profile,
        PilotStage::BendRunRs,
        sample_index,
        warmup,
        &run,
    )
}

pub(super) fn run_generated(
    context: &ExecutionContext<'_>,
    sample_index: usize,
    warmup: bool,
) -> Result<(StageObservation, StageCapture), String> {
    let executable = Path::new(SANDBOX_GENERATED_EXECUTABLE);
    let run = context.run(executable, &[])?;
    route_result(
        context.profile,
        PilotStage::GeneratedC,
        sample_index,
        warmup,
        &run,
    )
}

fn route_result(
    profile: &BendHvm2Profile,
    stage: PilotStage,
    sample_index: usize,
    warmup: bool,
    run: &BoundedProcessRun,
) -> Result<(StageObservation, StageCapture), String> {
    let parsed = parse_route_output(stage, &run.stdout);
    Ok((
        stage_observation(profile, stage, sample_index, warmup, run, parsed.ok())?,
        stage_capture(stage, sample_index, warmup, run)?,
    ))
}

pub(super) fn stage_observation(
    profile: &BendHvm2Profile,
    stage: PilotStage,
    sample_index: usize,
    warmup: bool,
    run: &BoundedProcessRun,
    parsed: Option<ParsedRouteOutput>,
) -> Result<StageObservation, String> {
    if !run.sandbox_enforced {
        return Err("stage did not enter the reviewed sandbox".to_string());
    }
    let result = parsed.map(|output| output.result);
    let interaction_count = parsed.and_then(|output| output.interaction_count);
    let combine_count = parsed.and_then(|output| output.combine_count);
    let outcome = if run.outcome != ProcessOutcome::Success {
        run.outcome
    } else if parsed.is_some() || matches!(stage, PilotStage::BendGenC | PilotStage::CCompile) {
        ProcessOutcome::Success
    } else {
        ProcessOutcome::MalformedOutput
    };
    Ok(StageObservation {
        stage,
        sample_index,
        warmup,
        command_profile_identity: command_profile_identity(profile, stage)
            .map_err(|error| error.message)?,
        outcome,
        exit_code: run.exit_code,
        result,
        interaction_count,
        combine_count,
        wall_nanoseconds: run.wall_nanoseconds,
        peak_memory_bytes: None,
        stdout_identity: ProjectionIdentity::compute(&run.stdout),
        stderr_identity: ProjectionIdentity::compute(&run.stderr),
        stdout_bytes: u64::try_from(run.stdout.len())
            .map_err(|_| "stdout length does not fit in u64".to_string())?,
        stderr_bytes: u64::try_from(run.stderr.len())
            .map_err(|_| "stderr length does not fit in u64".to_string())?,
    })
}

pub(super) fn stage_capture(
    stage: PilotStage,
    sample_index: usize,
    warmup: bool,
    run: &BoundedProcessRun,
) -> Result<StageCapture, String> {
    Ok(StageCapture {
        stage,
        sample_index,
        warmup,
        stdout: String::from_utf8(run.stdout.clone())
            .map_err(|error| format!("stage stdout is not UTF-8: {error}"))?,
        stderr: String::from_utf8(run.stderr.clone())
            .map_err(|error| format!("stage stderr is not UTF-8: {error}"))?,
    })
}
