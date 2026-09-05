#![forbid(unsafe_code)]
#![allow(
    unexpected_cfgs,
    reason = "The pinned Dylint driver injects cfg(dylint_lib = \"octet\") only during Octet checks."
)]
#![cfg_attr(dylint_lib = "octet", feature(register_tool))]
#![cfg_attr(dylint_lib = "octet", register_tool(tigerstyle))]
#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::ambient_env,
        tigerstyle::assertion_density,
        tigerstyle::function_length,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        tigerstyle::unbounded_loop,
        reason = "This composition root decodes fixed CLI paths before the bounded shell admits them."
    )
)]

use kamacite_cli::bend_hvm2_shell::{DEFAULT_EVIDENCE_DIRECTORY, RunOptions, ToolPaths, run};
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

const DEFAULT_ROOT: &str = ".";
const DEFAULT_BEND: &str = "bend";
const DEFAULT_HVM2: &str = "hvm";
const DEFAULT_COMPILER: &str = "cc";
const DEFAULT_ORACLE: &str = "kamacite-bend-hvm2-oracle";
const DEFAULT_SANDBOX: &str = "bwrap";
const DEFAULT_LIMIT: &str = "prlimit";
const HELP: &str = "usage: check-bend-hvm2-pilot --scratch-root PATH --runtime-closure-file PATH --proof-result PATH --verifier-identity-file PATH [--root PATH] [--bend PATH] [--hvm2 PATH] [--compiler PATH] [--oracle PATH] [--sandbox PATH] [--limit PATH] [--evidence-directory PATH] [--update-evidence]";

#[derive(Default)]
struct RequiredPaths {
    scratch_root: Option<PathBuf>,
    runtime_closure_file: Option<PathBuf>,
    proof_result: Option<PathBuf>,
    verifier_identity_file: Option<PathBuf>,
}

fn main() -> ExitCode {
    let options = parse_options(std::env::args().skip(1));
    match options.and_then(|options| run(&options)) {
        Ok(output) => {
            if let Err(error) = std::io::stdout().write_all(&output) {
                eprintln!("write pilot summary: {error}");
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
        Err(error) if error == HELP => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Bend/HVM2 pilot failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn parse_options(arguments: impl Iterator<Item = String>) -> Result<RunOptions, String> {
    let mut arguments = arguments;
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut bend = PathBuf::from(DEFAULT_BEND);
    let mut hvm2 = PathBuf::from(DEFAULT_HVM2);
    let mut compiler = PathBuf::from(DEFAULT_COMPILER);
    let mut oracle = PathBuf::from(DEFAULT_ORACLE);
    let mut sandbox = PathBuf::from(DEFAULT_SANDBOX);
    let mut limit = PathBuf::from(DEFAULT_LIMIT);
    let mut evidence_directory = PathBuf::from(DEFAULT_EVIDENCE_DIRECTORY);
    let mut required = RequiredPaths::default();
    let mut should_update_evidence = false;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => root = next_path(&mut arguments, "--root")?,
            "--scratch-root" => {
                required.scratch_root = Some(next_path(&mut arguments, "--scratch-root")?);
            }
            "--runtime-closure-file" => {
                required.runtime_closure_file =
                    Some(next_path(&mut arguments, "--runtime-closure-file")?);
            }
            "--proof-result" => {
                required.proof_result = Some(next_path(&mut arguments, "--proof-result")?);
            }
            "--verifier-identity-file" => {
                required.verifier_identity_file =
                    Some(next_path(&mut arguments, "--verifier-identity-file")?);
            }
            "--bend" => bend = next_path(&mut arguments, "--bend")?,
            "--hvm2" => hvm2 = next_path(&mut arguments, "--hvm2")?,
            "--compiler" => compiler = next_path(&mut arguments, "--compiler")?,
            "--oracle" => oracle = next_path(&mut arguments, "--oracle")?,
            "--sandbox" => sandbox = next_path(&mut arguments, "--sandbox")?,
            "--limit" => limit = next_path(&mut arguments, "--limit")?,
            "--evidence-directory" => {
                evidence_directory = next_path(&mut arguments, "--evidence-directory")?;
            }
            "--update-evidence" => should_update_evidence = true,
            "--help" | "-h" => return Err(HELP.to_string()),
            _ => return Err(format!("unknown argument `{argument}`\n{HELP}")),
        }
    }
    Ok(RunOptions {
        root,
        scratch_root: required
            .scratch_root
            .ok_or_else(|| format!("--scratch-root is required\n{HELP}"))?,
        tools: ToolPaths {
            bend,
            hvm2,
            compiler,
            oracle,
            sandbox,
            limit,
            runtime_closure_file: required
                .runtime_closure_file
                .ok_or_else(|| format!("--runtime-closure-file is required\n{HELP}"))?,
        },
        proof_result: required
            .proof_result
            .ok_or_else(|| format!("--proof-result is required\n{HELP}"))?,
        verifier_identity_file: required
            .verifier_identity_file
            .ok_or_else(|| format!("--verifier-identity-file is required\n{HELP}"))?,
        update_evidence: should_update_evidence,
        evidence_directory,
    })
}

fn next_path(
    arguments: &mut impl Iterator<Item = String>,
    option: &str,
) -> Result<PathBuf, String> {
    arguments
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| format!("{option} requires a path\n{HELP}"))
}
