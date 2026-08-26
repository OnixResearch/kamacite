#![forbid(unsafe_code)]

#[path = "../world_commit_shell/mod.rs"]
mod world_commit_shell;

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use world_commit_shell::{PlanUpdateOptions, RunOptions, plan_update, run};

const USAGE: &str = "usage: check-world-commit-conformance [--root PATH] [--update --accept-revision REVISION] [--plan-update REVISION [--proposed-schema PATH] [--proposed-codec PATH]]";

enum Command {
    Check(RunOptions),
    Plan(PlanUpdateOptions),
}

fn main() -> ExitCode {
    let result = parse_command().and_then(|command| match command {
        Command::Check(options) => run(&options),
        Command::Plan(options) => plan_update(&options),
    });
    match result {
        Ok(summary) => match io::stdout().write_all(&summary) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("write conformance summary: {error}");
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("world-commit conformance denied: {error}");
            ExitCode::FAILURE
        }
    }
}

fn parse_command() -> Result<Command, String> {
    let mut root = PathBuf::from(".");
    let mut update = false;
    let mut accepted_revision = None;
    let mut planned_revision = None;
    let mut proposed_schema_path = None;
    let mut proposed_codec_path = None;
    let mut arguments = std::env::args_os().skip(1);
    while let Some(argument) = arguments.next() {
        if argument == "--root" {
            root = arguments
                .next()
                .map(PathBuf::from)
                .ok_or_else(|| missing_value("--root"))?;
        } else if argument == "--update" {
            update = true;
        } else if argument == "--accept-revision" {
            accepted_revision = Some(next_string(&mut arguments, "--accept-revision")?);
        } else if argument == "--plan-update" {
            planned_revision = Some(next_string(&mut arguments, "--plan-update")?);
        } else if argument == "--proposed-schema" {
            proposed_schema_path = Some(next_string(&mut arguments, "--proposed-schema")?);
        } else if argument == "--proposed-codec" {
            proposed_codec_path = Some(next_string(&mut arguments, "--proposed-codec")?);
        } else if argument == "--help" {
            return Err(USAGE.to_string());
        } else {
            return Err(format!(
                "unknown argument: {}\n{USAGE}",
                argument.to_string_lossy()
            ));
        }
    }
    if let Some(proposed_revision) = planned_revision {
        if update || accepted_revision.is_some() {
            return Err(format!(
                "--plan-update cannot be combined with mutation options\n{USAGE}"
            ));
        }
        return Ok(Command::Plan(PlanUpdateOptions {
            root,
            proposed_revision,
            proposed_schema_path,
            proposed_codec_path,
        }));
    }
    if proposed_schema_path.is_some() || proposed_codec_path.is_some() {
        return Err(format!(
            "proposed source paths require --plan-update\n{USAGE}"
        ));
    }
    Ok(Command::Check(RunOptions {
        root,
        update,
        accepted_revision,
    }))
}

fn next_string(
    arguments: &mut impl Iterator<Item = std::ffi::OsString>,
    option: &str,
) -> Result<String, String> {
    arguments
        .next()
        .ok_or_else(|| missing_value(option))?
        .into_string()
        .map_err(|_| format!("{option} value is not UTF-8"))
}

fn missing_value(option: &str) -> String {
    format!("{option} requires a value\n{USAGE}")
}
