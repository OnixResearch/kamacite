use serde_json::Value;
use std::path::PathBuf;
use std::process::{Command, Output};

const IDENTIFY_DEFAULT: &str = "fixtures/semantic-operation/identify-default.json";
const IDENTIFY_DEFAULT_GOLDEN: &str = "fixtures/semantic-operation/identify-default.golden.json";
const IDENTIFY_MISSING: &str = "fixtures/semantic-operation/identify-missing-implementation.json";
const MIGRATION_PREVIEW: &str = "fixtures/semantic-operation/migration-preview.json";
const MIGRATION_PREVIEW_GOLDEN: &str = "fixtures/semantic-operation/migration-preview.golden.json";

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kamacite-semantic-operation"))
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("semantic operation CLI failed to start: {error}"))
}

#[test]
fn identifies_default_operation_from_capability_root() {
    let root = repository_root();
    let output = run(&[
        "identify",
        "--root",
        &root.to_string_lossy(),
        "--input",
        IDENTIFY_DEFAULT,
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = std::fs::read_to_string(root.join(IDENTIFY_DEFAULT_GOLDEN));
    assert!(expected.is_ok());
    if let Ok(expected) = expected {
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), expected);
    }
    let parsed: Result<Value, _> = serde_json::from_slice(&output.stdout);
    assert!(parsed.is_ok());
    if let Ok(parsed) = parsed {
        assert_eq!(
            parsed["semantic_operation_identity"]["domain"],
            "effect-operation"
        );
        assert!(
            parsed["canonical_bytes"]
                .as_array()
                .is_some_and(|bytes| !bytes.is_empty())
        );
    }
}

#[test]
fn rejects_operation_without_behavior_or_abstract_marker() {
    let root = repository_root();
    let output = run(&[
        "identify",
        "--root",
        &root.to_string_lossy(),
        "--input",
        IDENTIFY_MISSING,
    ]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("requires a default behavior identity or an abstract marker")
    );
}

#[test]
fn migration_is_dry_run_without_update() {
    let root = repository_root();
    let output = run(&[
        "migrate",
        "--root",
        &root.to_string_lossy(),
        "--input",
        MIGRATION_PREVIEW,
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = std::fs::read_to_string(root.join(MIGRATION_PREVIEW_GOLDEN));
    assert!(expected.is_ok());
    if let Ok(expected) = expected {
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), expected);
    }
    let parsed: Result<Value, _> = serde_json::from_slice(&output.stdout);
    assert!(parsed.is_ok());
    if let Ok(parsed) = parsed {
        assert_eq!(parsed["legacy"]["operation_id"], "op.read-config");
        assert_eq!(parsed["mutation_performed"], false);
        assert_eq!(
            parsed["semantic_operation_identity"]["domain"],
            "effect-operation"
        );
    }
}

#[test]
fn rejects_update_without_bounded_output() {
    let root = repository_root();
    let output = run(&[
        "migrate",
        "--root",
        &root.to_string_lossy(),
        "--input",
        MIGRATION_PREVIEW,
        "--update",
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--update requires --output"));
}
