use serde_json::Value;
use std::path::PathBuf;
use std::process::{Command, Output};

const VALID_BINDING: &str = "fixtures/application-host-binding/valid-closed-effect.json";
const MISSING_HOST: &str = "fixtures/application-host-binding/missing-host.json";

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kamacite-application-host-binding"))
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("application-host binding CLI failed to start: {error}"))
}

#[test]
fn validates_binding_and_projects_distinct_roles() {
    let root = repository_root();
    let output = run(&[
        "validate",
        "--root",
        &root.to_string_lossy(),
        "--input",
        VALID_BINDING,
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let parsed: Result<Value, _> = serde_json::from_slice(&output.stdout);
    assert!(parsed.is_ok());
    if let Ok(parsed) = parsed {
        assert_eq!(parsed["passed"], true);
        assert_eq!(parsed["validation"]["verdict"], "accepted");
        assert_eq!(
            parsed["validation"]["binding"]["identity"]["domain"],
            "application-host-binding"
        );
        assert_eq!(parsed["receipt"]["receipt_identity"]["domain"], "receipt");
        assert_eq!(
            parsed["valence_projection"]["projection_identity"]["domain"],
            "projection"
        );
        assert_eq!(parsed["mutation_performed"], false);
    }
}

#[test]
fn rejects_missing_host_with_safe_diagnostic() {
    let root = repository_root();
    let output = run(&[
        "validate",
        "--root",
        &root.to_string_lossy(),
        "--input",
        MISSING_HOST,
    ]);
    assert!(!output.status.success());
    let parsed: Result<Value, _> = serde_json::from_slice(&output.stdout);
    assert!(parsed.is_ok());
    if let Ok(parsed) = parsed {
        assert_eq!(parsed["passed"], false);
        assert_eq!(parsed["validation"]["verdict"], "rejected");
        assert_eq!(parsed["valence_projection"], Value::Null);
        let diagnostics = parsed["validation"]["diagnostics"].as_array();
        assert!(diagnostics.is_some());
        if let Some(diagnostics) = diagnostics {
            assert!(diagnostics.iter().any(|diagnostic| {
                diagnostic["code"] == "host-selection-missing"
                    && diagnostic["field"] == "binding.host_profile_identities"
            }));
        }
    }
}

#[test]
fn rejects_mutation_without_bounded_receipt_output() {
    let root = repository_root();
    let output = run(&[
        "validate",
        "--root",
        &root.to_string_lossy(),
        "--input",
        VALID_BINDING,
        "--update",
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--update requires --receipt-output"));
}

#[test]
fn rejects_input_path_outside_capability_root() {
    let root = repository_root();
    let output = run(&[
        "validate",
        "--root",
        &root.to_string_lossy(),
        "--input",
        "../README.md",
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("parent-traversal"));
}
