#![forbid(unsafe_code)]

// r[verify kamacite.findings.steel_execution.fresh_probe]
// r[verify kamacite.findings.steel_execution.runner_identity]
// r[verify kamacite.findings.steel_execution.capability_enforcement]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const MANIFEST: &str = "fixtures/manifest.json";
const DTO_FIXTURE_ID: &str = "adapter.positive.steel-dto-only";
const EXECUTED_EVIDENCE_LABEL: &str = "steel-executed-probe-evidence";
const FAKE_RUNNER_DESCRIPTOR: &str = "fake-runner";

fn repository_root() -> Result<PathBuf, std::io::Error> {
    std::fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

fn run(root: &Path, extra_arguments: &[&str]) -> Result<Output, std::io::Error> {
    let root = root.to_string_lossy();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kamacite-validate"));
    command.args([
        "validate-steel-adapter",
        "--root",
        &root,
        "--manifest",
        MANIFEST,
    ]);
    command.args(extra_arguments);
    command.output()
}

fn output_text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn explicit_validation_executes_and_records_fresh_probe_evidence()
-> Result<(), Box<dyn std::error::Error>> {
    let root = repository_root()?;
    let output = run(&root, &[])?;
    assert!(output.status.success(), "{}", output_text(&output));
    let report = serde_json::from_slice::<serde_json::Value>(&output.stdout)?;
    let results = report["fixture_report"]["results"]
        .as_array()
        .ok_or("fixture results are missing")?;
    let fixture = results
        .iter()
        .find(|fixture| fixture["id"] == DTO_FIXTURE_ID)
        .ok_or("executed Steel fixture is missing")?;
    assert_eq!(fixture["observed_verdict"], "accepted");
    assert!(
        fixture["receipt"]["body"]["output_identities"]
            .get(EXECUTED_EVIDENCE_LABEL)
            .is_some()
    );
    Ok(())
}

#[test]
fn explicit_validation_rejects_caller_supplied_fake_runner()
-> Result<(), Box<dyn std::error::Error>> {
    let root = repository_root()?;
    let output = run(&root, &["--evaluator-descriptor", FAKE_RUNNER_DESCRIPTOR])?;
    assert!(!output.status.success());
    assert!(
        output_text(&output).contains("does not match the admitted embedded runner"),
        "{}",
        output_text(&output)
    );
    Ok(())
}
