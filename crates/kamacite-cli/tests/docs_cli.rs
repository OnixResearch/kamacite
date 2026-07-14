#![forbid(unsafe_code)]

// r[verify kamacite.findings.docs_commands.config_validation]
// r[verify kamacite.findings.docs_commands.action_completion]
// r[verify kamacite.findings.docs_commands.no_op_rejection]
// r[verify kamacite.findings.docs_commands.receipts]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const INVALID_CONFIG: &str = "fixtures/docs-config/invalid-readable.ncl";
const CONTRACT_INVALID_CONFIG: &str = "fixtures/docs-config/contract-invalid.ncl";

fn repository_root() -> Result<PathBuf, std::io::Error> {
    std::fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

fn run(root: &Path, arguments: &[&str]) -> Result<Output, std::io::Error> {
    Command::new(env!("CARGO_BIN_EXE_kamacite-validate"))
        .current_dir(root)
        .args(arguments)
        .output()
}

fn output_text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn report(output: &Output) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_slice(&output.stdout)
}

#[test]
fn collect_check_and_render_complete_with_receipts() -> Result<(), Box<dyn std::error::Error>> {
    let root = repository_root()?;
    for arguments in [
        vec!["docs", "collect"],
        vec!["docs", "check"],
        vec!["docs", "render", "--format", "markdown"],
    ] {
        let output = run(&root, &arguments)?;
        assert!(output.status.success(), "{}", output_text(&output));
        let report = report(&output)?;
        assert_eq!(report["passed"], true);
        assert_eq!(report["state"], "completed");
        assert!(
            report["completed_work_items"]
                .as_u64()
                .is_some_and(|count| count > 0)
        );
        assert!(report["documentation_ir_identity"].is_object());
        assert!(report["receipt"].is_object());
    }
    Ok(())
}

#[test]
fn malformed_and_contract_invalid_configs_fail_before_dispatch()
-> Result<(), Box<dyn std::error::Error>> {
    let root = repository_root()?;
    for config in [INVALID_CONFIG, CONTRACT_INVALID_CONFIG] {
        let output = run(&root, &["docs", "check", "--config", config])?;
        assert!(!output.status.success(), "command unexpectedly succeeded");
        let text = output_text(&output);
        assert!(
            text.contains("docs Nickel export")
                || text.contains("docs config projection is malformed or contract-invalid")
                || text.contains("docs config must import the checked sibling"),
            "unexpected config diagnostic: {text}"
        );
        assert!(!text.contains("config_source_identity"));
    }
    Ok(())
}

#[test]
fn unsupported_and_no_op_actions_fail_closed_while_dry_run_is_only_a_plan()
-> Result<(), Box<dyn std::error::Error>> {
    let root = repository_root()?;
    for arguments in [
        vec!["docs", "render", "--format", "pdf"],
        vec!["docs", "check", "--profile", "production"],
        vec!["docs", "run-cells", "--target", "wasm"],
        vec!["docs", "update"],
    ] {
        let output = run(&root, &arguments)?;
        assert!(!output.status.success(), "command unexpectedly succeeded");
        let report = report(&output)?;
        assert_eq!(report["passed"], false);
        assert!(report["receipt"].is_null());
    }

    let dry_run = run(&root, &["docs", "update", "--dry-run"])?;
    assert!(dry_run.status.success(), "{}", output_text(&dry_run));
    let report = report(&dry_run)?;
    assert_eq!(report["state"], "planned");
    assert_eq!(report["mutated"], false);
    assert!(report["receipt"].is_null());
    Ok(())
}
