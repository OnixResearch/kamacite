#![forbid(unsafe_code)]

// r[verify kamacite.nickel_preserves_adapter.admission]
// r[verify kamacite.nickel_preserves_adapter.composition]
// r[verify kamacite.nickel_preserves_adapter.receipt]
// r[verify kamacite.nickel_preserves_adapter.command]
// r[verify kamacite.nickel_preserves_adapter.fixtures]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

const MANIFEST_PATH: &str = "fixtures/nickel-export/service-config/generated/manifest.json";
const ARTIFACT_PATH: &str = "fixtures/nickel-export/service-config/generated/service.json";
const PROFILE_PATH: &str = "adapters/nickel-preserves/generated/profile.json";
const SELECTED_EXPORT: &str = "examples/service-config/generated/service.json";
const PACKED_OUTPUT: &str = "out/service.preserves";
const TEXT_OUTPUT: &str = "out/service.preserves.txt";
const RECEIPT_OUTPUT: &str = "out/service.receipt.json";
const STALE_ARTIFACT: &[u8] = b"{\"service\":\"tampered\"}";
const VALID_BUT_WRONG_B3: &str =
    "b3:0000000000000000000000000000000000000000000000000000000000000000";

static TEST_DIRECTORY_COUNTER: AtomicU64 = AtomicU64::new(1);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn create(label: &str) -> Result<Self, std::io::Error> {
        let sequence = TEST_DIRECTORY_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kamacite-nickel-preserves-{label}-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&path)?;
        Ok(Self { path })
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn repository_root() -> Result<PathBuf, std::io::Error> {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

fn prepare_root(repository: &Path, label: &str) -> Result<TestDirectory, std::io::Error> {
    let root = TestDirectory::create(label)?;
    for relative in [MANIFEST_PATH, ARTIFACT_PATH, PROFILE_PATH] {
        let destination = root.path.join(relative);
        let parent = destination
            .parent()
            .ok_or_else(|| std::io::Error::other("fixture destination has no parent"))?;
        fs::create_dir_all(parent)?;
        fs::copy(repository.join(relative), destination)?;
    }
    Ok(root)
}

fn run(root: &Path, mode: &str, packed_output: &str) -> Result<Output, std::io::Error> {
    run_selected(root, mode, packed_output, SELECTED_EXPORT)
}

fn run_selected(
    root: &Path,
    mode: &str,
    packed_output: &str,
    selected_export: &str,
) -> Result<Output, std::io::Error> {
    run_modes(root, &[mode], packed_output, selected_export)
}

fn run_modes(
    root: &Path,
    modes: &[&str],
    packed_output: &str,
    selected_export: &str,
) -> Result<Output, std::io::Error> {
    let root = root.to_string_lossy();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kamacite-validate"));
    command.args([
        "adapt",
        "nickel-export",
        "--root",
        &root,
        "--manifest",
        MANIFEST_PATH,
        "--export",
        selected_export,
        "--artifact",
        ARTIFACT_PATH,
        "--packed-output",
        packed_output,
        "--text-output",
        TEXT_OUTPUT,
        "--receipt",
        RECEIPT_OUTPUT,
        "--profile",
        PROFILE_PATH,
    ]);
    command.args(modes).output()
}

fn output_text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn write_check_and_noop_modes_are_fail_closed() -> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let root = prepare_root(&repository, "positive")?;
    let write = run(&root.path, "--write", PACKED_OUTPUT)?;
    assert!(write.status.success(), "{}", output_text(&write));
    assert!(root.path.join(PACKED_OUTPUT).is_file());
    assert!(root.path.join(TEXT_OUTPUT).is_file());
    assert!(root.path.join(RECEIPT_OUTPUT).is_file());

    let check = run(&root.path, "--check", PACKED_OUTPUT)?;
    assert!(check.status.success(), "{}", output_text(&check));

    let noop = run(&root.path, "--write", PACKED_OUTPUT)?;
    assert!(!noop.status.success());
    assert!(output_text(&noop).contains("would be a no-op"));

    fs::write(root.path.join(TEXT_OUTPUT), b"stale-review-text")?;
    let stale_text = run(&root.path, "--check", PACKED_OUTPUT)?;
    assert!(!stale_text.status.success());
    assert!(output_text(&stale_text).contains("Preserves review text output is stale"));
    let repair_text = run(&root.path, "--write", PACKED_OUTPUT)?;
    assert!(
        repair_text.status.success(),
        "{}",
        output_text(&repair_text)
    );

    fs::write(root.path.join(RECEIPT_OUTPUT), b"stale-receipt")?;
    let stale_receipt = run(&root.path, "--check", PACKED_OUTPUT)?;
    assert!(!stale_receipt.status.success());
    assert!(output_text(&stale_receipt).contains("adapter receipt output is stale"));
    let repair_receipt = run(&root.path, "--write", PACKED_OUTPUT)?;
    assert!(
        repair_receipt.status.success(),
        "{}",
        output_text(&repair_receipt)
    );

    fs::write(root.path.join(PACKED_OUTPUT), b"detached-canonical-bytes")?;
    let stale_packed = run(&root.path, "--check", PACKED_OUTPUT)?;
    assert!(!stale_packed.status.success());
    assert!(output_text(&stale_packed).contains("packed Preserves output is stale"));
    Ok(())
}

#[test]
fn check_and_write_modes_are_mutually_exclusive() -> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let root = prepare_root(&repository, "mode-selection")?;
    for modes in [&[][..], &["--check", "--write"][..]] {
        let output = run_modes(&root.path, modes, PACKED_OUTPUT, SELECTED_EXPORT)?;
        assert!(!output.status.success());
        assert!(output_text(&output).contains("requires exactly one of --check or --write"));
    }
    Ok(())
}

#[test]
fn stale_artifact_is_rejected_before_output() -> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let root = prepare_root(&repository, "stale-artifact")?;
    fs::write(root.path.join(ARTIFACT_PATH), STALE_ARTIFACT)?;
    let output = run(&root.path, "--check", PACKED_OUTPUT)?;
    assert!(!output.status.success());
    assert!(
        output_text(&output).contains("artifact verification failed"),
        "{}",
        output_text(&output)
    );
    assert!(!root.path.join(PACKED_OUTPUT).exists());

    let missing = prepare_root(&repository, "missing-artifact")?;
    fs::remove_file(missing.path.join(ARTIFACT_PATH))?;
    let missing_output = run(&missing.path, "--check", PACKED_OUTPUT)?;
    assert!(!missing_output.status.success());
    assert!(
        output_text(&missing_output).contains("root I/O read failed (not-found)"),
        "{}",
        output_text(&missing_output)
    );
    Ok(())
}

#[test]
fn malformed_manifest_wrong_destination_and_parent_escape_are_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let malformed = prepare_root(&repository, "malformed-manifest")?;
    fs::write(malformed.path.join(MANIFEST_PATH), b"{}")?;
    let malformed_output = run(&malformed.path, "--check", PACKED_OUTPUT)?;
    assert!(!malformed_output.status.success());
    assert!(output_text(&malformed_output).contains("manifest parse failed"));

    let wrong_destination = prepare_root(&repository, "wrong-destination")?;
    let wrong_destination_output = run_selected(
        &wrong_destination.path,
        "--check",
        PACKED_OUTPUT,
        "generated/missing.json",
    )?;
    assert!(!wrong_destination_output.status.success());
    assert!(output_text(&wrong_destination_output).contains("absent from the manifest"));

    let absolute = prepare_root(&repository, "absolute-escape")?;
    let absolute_output = run(&absolute.path, "--check", "/tmp/escaped.preserves")?;
    assert!(!absolute_output.status.success());
    assert!(output_text(&absolute_output).contains("root path rejected (absolute)"));

    let escaped = prepare_root(&repository, "parent-escape")?;
    let escape_output = run(&escaped.path, "--check", "../escaped.preserves")?;
    assert!(!escape_output.status.success());
    assert!(
        output_text(&escape_output).contains("parent-traversal"),
        "{}",
        output_text(&escape_output)
    );
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn unsafe_projection_destination_is_rejected_before_publication()
-> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::symlink;

    let repository = repository_root()?;
    let root = prepare_root(&repository, "partial-publication")?;
    let outside = TestDirectory::create("partial-publication-outside")?;
    fs::create_dir_all(root.path.join("out"))?;
    symlink(outside.path.join("review.txt"), root.path.join(TEXT_OUTPUT))?;
    let output = run(&root.path, "--write", PACKED_OUTPUT)?;
    assert!(!output.status.success());
    assert!(
        output_text(&output).contains("root path"),
        "{}",
        output_text(&output)
    );
    assert!(!root.path.join(PACKED_OUTPUT).exists());
    assert!(!root.path.join(RECEIPT_OUTPUT).exists());
    Ok(())
}

#[test]
fn stale_source_evaluator_and_dependency_evidence_is_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    for (label, pointer, replacement) in [
        ("unknown-schema", "/schema", "unknown-manifest-schema"),
        (
            "stale-receipt",
            "/exports/0/receipt_identity",
            VALID_BUT_WRONG_B3,
        ),
        (
            "stale-source",
            "/exports/0/source/identity",
            VALID_BUT_WRONG_B3,
        ),
        (
            "wrong-evaluator",
            "/exports/0/evaluator/version",
            "substituted-evaluator",
        ),
        (
            "stale-dependency",
            "/exports/0/dependencies/0/identity",
            VALID_BUT_WRONG_B3,
        ),
    ] {
        let root = prepare_root(&repository, label)?;
        let manifest_path = root.path.join(MANIFEST_PATH);
        let mut manifest = serde_json::from_slice::<serde_json::Value>(&fs::read(&manifest_path)?)?;
        let field = manifest
            .pointer_mut(pointer)
            .ok_or_else(|| std::io::Error::other(format!("fixture field is absent: {pointer}")))?;
        *field = serde_json::Value::String(replacement.to_string());
        fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
        let output = run(&root.path, "--check", PACKED_OUTPUT)?;
        assert!(!output.status.success());
        assert!(
            output_text(&output).contains("manifest admission failed"),
            "{}",
            output_text(&output)
        );
    }
    Ok(())
}
