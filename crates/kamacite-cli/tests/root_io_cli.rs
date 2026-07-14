#![forbid(unsafe_code)]

// r[verify kamacite.findings.root_io_confinement.single_gateway]
// r[verify kamacite.findings.root_io_confinement.symlink_safe]
// r[verify kamacite.findings.root_io_confinement.mutation_safe]
// r[verify kamacite.findings.root_io_confinement.coverage]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(target_os = "linux")]
use std::os::unix::fs::symlink;

const OUTSIDE_BYTES: &[u8] = b"outside must remain unchanged";
const ROOT_IO_DIAGNOSTIC: &str = "root path rejected";
const FUNCTION_REPORT: &str = "fixtures/function-address/octet-valid-report.json";
const FUNCTION_SOURCE: &str = "fixtures/function-address/source.rs";
const VALENCE_RECEIPT: &str = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json";
const VALENCE_CONFORMANCE: &str = "fixtures/valence-export/conformance.json";

static TEST_DIRECTORY_COUNTER: AtomicU64 = AtomicU64::new(1);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn create(label: &str) -> Result<Self, std::io::Error> {
        let sequence = TEST_DIRECTORY_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kamacite-root-io-cli-{label}-{}-{sequence}",
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

fn run(arguments: &[&str]) -> Result<Output, std::io::Error> {
    Command::new(env!("CARGO_BIN_EXE_kamacite-validate"))
        .args(arguments)
        .output()
}

fn copy_fixture(repository: &Path, root: &Path, relative: &str) -> Result<(), std::io::Error> {
    let destination = root.join(relative);
    let Some(parent) = destination.parent() else {
        return Err(std::io::Error::other(
            "fixture destination has no parent directory",
        ));
    };
    fs::create_dir_all(parent)?;
    fs::copy(repository.join(relative), destination)?;
    Ok(())
}

fn output_text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn assert_rejected(output: &Output, diagnostic: &str) {
    assert!(!output.status.success(), "command unexpectedly succeeded");
    assert!(
        output_text(output).contains(diagnostic),
        "missing diagnostic `{diagnostic}` in command output: {}",
        output_text(output)
    );
}

#[cfg(target_os = "linux")]
#[test]
fn regular_root_scoped_command_families_still_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let root = repository.to_string_lossy().into_owned();

    let validate = run(&[
        "validate",
        "--root",
        &root,
        "--manifest",
        "fixtures/manifest.json",
    ])?;
    assert!(validate.status.success(), "{}", output_text(&validate));

    let steel = run(&[
        "validate-steel-adapter",
        "--root",
        &root,
        "--manifest",
        "fixtures/manifest.json",
    ])?;
    assert!(steel.status.success(), "{}", output_text(&steel));

    let migration = run(&[
        "migrate-preserves",
        "--root",
        &root,
        "--manifest",
        "fixtures/manifest.json",
        "--manifest-source",
        "fixtures/manifest.ncl",
    ])?;
    assert!(migration.status.success(), "{}", output_text(&migration));

    let valence = run(&[
        "export-valence-receipt",
        "--root",
        &root,
        "--receipt",
        VALENCE_RECEIPT,
        "--conformance",
        VALENCE_CONFORMANCE,
        "--golden",
        "fixtures/valence-export/golden/source-envelope.valid.nickel-minimal.valence-export.json",
    ])?;
    assert!(valence.status.success(), "{}", output_text(&valence));
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn root_scoped_commands_reject_absolute_parent_and_symlink_sources()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("source-policy")?;
    let root = test_root.path.to_string_lossy().into_owned();
    let absolute_manifest = repository.join("fixtures/manifest.json");
    let absolute_manifest = absolute_manifest.to_string_lossy().into_owned();

    let absolute = run(&[
        "validate",
        "--root",
        &root,
        "--manifest",
        &absolute_manifest,
    ])?;
    assert_rejected(&absolute, "root path rejected (absolute)");

    let parent = run(&[
        "validate",
        "--root",
        &root,
        "--manifest",
        "../manifest.json",
    ])?;
    assert_rejected(&parent, "root path rejected (parent-traversal)");

    symlink(
        repository.join("fixtures/manifest.json"),
        test_root.path.join("manifest-link.json"),
    )?;
    for command in [
        "validate",
        "validate-nickel-adapter",
        "validate-steel-adapter",
        "migrate-preserves",
    ] {
        let output = run(&[command, "--root", &root, "--manifest", "manifest-link.json"])?;
        assert_rejected(&output, ROOT_IO_DIAGNOSTIC);
    }
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn function_address_outputs_reject_symlink_destinations_and_parents()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("function-output")?;
    let outside = TestDirectory::create("function-output-outside")?;
    copy_fixture(&repository, &test_root.path, FUNCTION_REPORT)?;
    copy_fixture(&repository, &test_root.path, FUNCTION_SOURCE)?;
    let root = test_root.path.to_string_lossy().into_owned();
    let source_argument = format!("{FUNCTION_SOURCE}={FUNCTION_SOURCE}");

    let regular = run(&[
        "emit-function-address-receipt",
        "--root",
        &root,
        "--report",
        FUNCTION_REPORT,
        "--source",
        &source_argument,
        "--out",
        "regular/receipt.json",
    ])?;
    assert!(regular.status.success(), "{}", output_text(&regular));
    assert!(test_root.path.join("regular/receipt.json").is_file());

    let outside_file = outside.path.join("outside.json");
    fs::write(&outside_file, OUTSIDE_BYTES)?;
    symlink(&outside_file, test_root.path.join("destination-link.json"))?;
    let destination = run(&[
        "emit-function-address-receipt",
        "--root",
        &root,
        "--report",
        FUNCTION_REPORT,
        "--source",
        &source_argument,
        "--out",
        "destination-link.json",
    ])?;
    assert_rejected(&destination, "symlink-destination");
    assert_eq!(fs::read(&outside_file)?, OUTSIDE_BYTES);

    symlink(&outside.path, test_root.path.join("parent-link"))?;
    let parent = run(&[
        "emit-function-address-receipt",
        "--root",
        &root,
        "--report",
        FUNCTION_REPORT,
        "--source",
        &source_argument,
        "--out",
        "parent-link/receipt.json",
    ])?;
    assert_rejected(&parent, ROOT_IO_DIAGNOSTIC);
    assert!(!outside.path.join("receipt.json").exists());
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn valence_update_rejects_outward_symlink_parent() -> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("valence-output")?;
    let outside = TestDirectory::create("valence-output-outside")?;
    copy_fixture(&repository, &test_root.path, VALENCE_RECEIPT)?;
    copy_fixture(&repository, &test_root.path, VALENCE_CONFORMANCE)?;
    symlink(&outside.path, test_root.path.join("escape"))?;
    let root = test_root.path.to_string_lossy().into_owned();

    let output = run(&[
        "export-valence-receipt",
        "--root",
        &root,
        "--receipt",
        VALENCE_RECEIPT,
        "--conformance",
        VALENCE_CONFORMANCE,
        "--golden",
        "escape/golden.json",
        "--update-golden",
        "--update-report",
        "reports/update.json",
    ])?;
    assert_rejected(&output, ROOT_IO_DIAGNOSTIC);
    assert!(!outside.path.join("golden.json").exists());
    Ok(())
}
