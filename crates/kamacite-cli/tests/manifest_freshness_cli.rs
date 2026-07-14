#![forbid(unsafe_code)]

// r[verify kamacite.findings.manifest_semantic_freshness.complete_projection]
// r[verify kamacite.findings.manifest_semantic_freshness.typed_contract]
// r[verify kamacite.findings.manifest_semantic_freshness.generation_receipt]
// r[verify kamacite.findings.manifest_semantic_freshness.negative_cases]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(target_os = "linux")]
use std::os::unix::fs::{PermissionsExt, symlink};

const MANIFEST_SOURCE: &str = "fixtures/manifest.ncl";
const MANIFEST_CONTRACT: &str = "fixtures/manifest-contract.ncl";
const MANIFEST_PROJECTION: &str = "fixtures/manifest.json";
const MANIFEST_RECEIPT: &str = "fixtures/manifest-generation-receipt.json";
const MANIFEST_FILES: &[&str] = &[
    MANIFEST_SOURCE,
    MANIFEST_CONTRACT,
    MANIFEST_PROJECTION,
    MANIFEST_RECEIPT,
];
const EMPTY_PATH: &str = "";
const FAKE_EVALUATOR_MODE: u32 = 0o755;
const OUTSIDE_BYTES: &[u8] = b"outside manifest evidence must remain unchanged";
const ZERO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const CONTRACT_TEST_POSITIVE: &str = "fixtures/manifest-contract-tests/positive.ncl";
const CONTRACT_TEST_NEGATIVE: &[&str] = &[
    "fixtures/manifest-contract-tests/missing-required-field.ncl",
    "fixtures/manifest-contract-tests/unsupported-kind.ncl",
    "fixtures/manifest-contract-tests/wrong-field-type.ncl",
    "fixtures/manifest-contract-tests/unknown-field.ncl",
];

static TEST_DIRECTORY_COUNTER: AtomicU64 = AtomicU64::new(1);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn create(label: &str) -> Result<Self, std::io::Error> {
        let sequence = TEST_DIRECTORY_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kamacite-manifest-freshness-{label}-{}-{sequence}",
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

fn copy_manifest_files(repository: &Path, root: &Path) -> Result<(), std::io::Error> {
    for relative in MANIFEST_FILES {
        let destination = root.join(relative);
        let Some(parent) = destination.parent() else {
            return Err(std::io::Error::other(
                "manifest destination has no parent directory",
            ));
        };
        fs::create_dir_all(parent)?;
        fs::copy(repository.join(relative), destination)?;
    }
    Ok(())
}

fn run(arguments: &[&str]) -> Result<Output, std::io::Error> {
    Command::new(env!("CARGO_BIN_EXE_kamacite-validate"))
        .args(arguments)
        .output()
}

fn validate(root: &Path) -> Result<Output, std::io::Error> {
    let root = root.to_string_lossy();
    run(&[
        "validate",
        "--root",
        &root,
        "--manifest",
        MANIFEST_PROJECTION,
    ])
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
        "missing diagnostic `{diagnostic}` in output: {}",
        output_text(output)
    );
}

fn read_json(path: &Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn write_json(path: &Path, value: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn default_validation_is_receipt_only_and_does_not_launch_nickel()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let root = repository.to_string_lossy();
    let output = Command::new(env!("CARGO_BIN_EXE_kamacite-validate"))
        .env("PATH", EMPTY_PATH)
        .args([
            "validate",
            "--root",
            &root,
            "--manifest",
            MANIFEST_PROJECTION,
        ])
        .output()?;

    assert!(output.status.success(), "{}", output_text(&output));
    assert!(output_text(&output).contains("\"manifest_generation\""));
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn closed_manifest_contract_accepts_supported_shapes_and_rejects_invalid_shapes()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("contract")?;
    fs::copy(
        repository.join(MANIFEST_CONTRACT),
        test_root.path.join("contract.ncl"),
    )?;
    let root = test_root.path.to_string_lossy();

    fs::copy(
        repository.join(CONTRACT_TEST_POSITIVE),
        test_root.path.join("source.ncl"),
    )?;
    let accepted = run(&[
        "generate-manifest",
        "--root",
        &root,
        "--source",
        "source.ncl",
        "--contract",
        "contract.ncl",
        "--projection",
        "projection.json",
        "--receipt",
        "receipt.json",
        "--update",
    ])?;
    assert!(accepted.status.success(), "{}", output_text(&accepted));

    for negative in CONTRACT_TEST_NEGATIVE {
        fs::copy(repository.join(negative), test_root.path.join("source.ncl"))?;
        let rejected = run(&[
            "generate-manifest",
            "--root",
            &root,
            "--source",
            "source.ncl",
            "--contract",
            "contract.ncl",
            "--projection",
            "projection.json",
            "--receipt",
            "receipt.json",
            "--update",
        ])?;
        assert_rejected(&rejected, "manifest Nickel export failed");
    }
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn explicit_generation_updates_projection_and_checked_receipt()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("generation")?;
    copy_manifest_files(&repository, &test_root.path)?;
    fs::remove_file(test_root.path.join(MANIFEST_PROJECTION))?;
    fs::remove_file(test_root.path.join(MANIFEST_RECEIPT))?;
    let root = test_root.path.to_string_lossy();

    let update = run(&["generate-manifest", "--root", &root, "--update"])?;
    assert!(update.status.success(), "{}", output_text(&update));
    assert!(test_root.path.join(MANIFEST_PROJECTION).is_file());
    assert!(test_root.path.join(MANIFEST_RECEIPT).is_file());

    let check = run(&["generate-manifest", "--root", &root])?;
    assert!(check.status.success(), "{}", output_text(&check));
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn rejects_source_contract_projection_and_receipt_drift_before_fixture_io()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("drift")?;
    copy_manifest_files(&repository, &test_root.path)?;

    let source_path = test_root.path.join(MANIFEST_SOURCE);
    let original_source = fs::read(&source_path)?;
    let mut spoofed_source = original_source.clone();
    spoofed_source
        .extend_from_slice(b"\n# projection_identity_hex = \"spoof\"; id = \"spoofed-fixture\"\n");
    fs::write(&source_path, spoofed_source)?;
    assert_rejected(&validate(&test_root.path)?, "source_identity");

    let mut string_spoof =
        b"let spoof = \"id = \\\"spoofed-fixture\\\"; projection_identity_hex = stale\" in\n"
            .to_vec();
    string_spoof.extend_from_slice(&original_source);
    fs::write(&source_path, string_spoof)?;
    assert_rejected(&validate(&test_root.path)?, "source_identity");
    fs::write(&source_path, original_source)?;

    let contract_path = test_root.path.join(MANIFEST_CONTRACT);
    let original_contract = fs::read(&contract_path)?;
    let mut stale_contract = original_contract.clone();
    stale_contract.extend_from_slice(b"\n# stale contract metadata\n");
    fs::write(&contract_path, stale_contract)?;
    assert_rejected(&validate(&test_root.path)?, "contract_identity");
    fs::write(&contract_path, original_contract)?;

    let projection_path = test_root.path.join(MANIFEST_PROJECTION);
    let mut projection = read_json(&projection_path)?;
    let Some(non_claims) = projection["authority_non_claims"].as_array_mut() else {
        panic!("authority_non_claims must be an array");
    };
    non_claims.push(serde_json::Value::String("stale metadata".to_string()));
    write_json(&projection_path, &projection)?;
    assert_rejected(&validate(&test_root.path)?, "projection_identity");
    fs::copy(repository.join(MANIFEST_PROJECTION), &projection_path)?;

    let receipt_path = test_root.path.join(MANIFEST_RECEIPT);
    let mut receipt = read_json(&receipt_path)?;
    receipt["receipt_identity"]["hex"] = serde_json::Value::String(ZERO_HEX.to_string());
    write_json(&receipt_path, &receipt)?;
    assert_rejected(&validate(&test_root.path)?, "receipt_identity");
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn rejects_duplicate_reordered_and_extra_fixture_projections()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("fixture-drift")?;
    copy_manifest_files(&repository, &test_root.path)?;
    let projection_path = test_root.path.join(MANIFEST_PROJECTION);
    let original = read_json(&projection_path)?;

    let mut reordered = original.clone();
    let Some(fixtures) = reordered["fixtures"].as_array_mut() else {
        panic!("fixtures must be an array");
    };
    fixtures.swap(0, 1);
    write_json(&projection_path, &reordered)?;
    assert_rejected(&validate(&test_root.path)?, "projection_identity");

    let mut duplicate = original.clone();
    let duplicate_fixture = duplicate["fixtures"][0].clone();
    let Some(fixtures) = duplicate["fixtures"].as_array_mut() else {
        panic!("fixtures must be an array");
    };
    fixtures.push(duplicate_fixture);
    write_json(&projection_path, &duplicate)?;
    assert_rejected(&validate(&test_root.path)?, "projection_identity");

    let mut extra = original;
    let mut extra_fixture = extra["fixtures"][0].clone();
    extra_fixture["id"] = serde_json::Value::String("extra-fixture".to_string());
    let Some(fixtures) = extra["fixtures"].as_array_mut() else {
        panic!("fixtures must be an array");
    };
    fixtures.push(extra_fixture);
    write_json(&projection_path, &extra)?;
    assert_rejected(&validate(&test_root.path)?, "projection_identity");
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn manifest_generation_rejects_symlink_sources_and_destinations()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("symlink-policy")?;
    let outside = TestDirectory::create("symlink-policy-outside")?;
    copy_manifest_files(&repository, &test_root.path)?;
    let root = test_root.path.to_string_lossy();

    let source_path = test_root.path.join(MANIFEST_SOURCE);
    fs::remove_file(&source_path)?;
    symlink(repository.join(MANIFEST_SOURCE), &source_path)?;
    let source = run(&["generate-manifest", "--root", &root, "--update"])?;
    assert_rejected(&source, "root path rejected");
    fs::remove_file(&source_path)?;
    fs::copy(repository.join(MANIFEST_SOURCE), &source_path)?;

    let contract_path = test_root.path.join(MANIFEST_CONTRACT);
    fs::remove_file(&contract_path)?;
    symlink(repository.join(MANIFEST_CONTRACT), &contract_path)?;
    let contract = run(&["generate-manifest", "--root", &root, "--update"])?;
    assert_rejected(&contract, "root path rejected");
    fs::remove_file(&contract_path)?;
    fs::copy(repository.join(MANIFEST_CONTRACT), &contract_path)?;

    let outside_projection = outside.path.join("projection.json");
    fs::write(&outside_projection, OUTSIDE_BYTES)?;
    let projection_path = test_root.path.join(MANIFEST_PROJECTION);
    fs::remove_file(&projection_path)?;
    symlink(&outside_projection, &projection_path)?;
    let projection = run(&["generate-manifest", "--root", &root, "--update"])?;
    assert_rejected(&projection, "symlink-destination");
    assert_eq!(fs::read(&outside_projection)?, OUTSIDE_BYTES);
    fs::remove_file(&projection_path)?;
    fs::copy(repository.join(MANIFEST_PROJECTION), &projection_path)?;

    let outside_receipt = outside.path.join("receipt.json");
    fs::write(&outside_receipt, OUTSIDE_BYTES)?;
    let receipt_path = test_root.path.join(MANIFEST_RECEIPT);
    fs::remove_file(&receipt_path)?;
    symlink(&outside_receipt, &receipt_path)?;
    let receipt = run(&["generate-manifest", "--root", &root, "--update"])?;
    assert_rejected(&receipt, "symlink-destination");
    assert_eq!(fs::read(&outside_receipt)?, OUTSIDE_BYTES);
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn explicit_generation_rejects_malformed_source_and_stale_evaluator()
-> Result<(), Box<dyn std::error::Error>> {
    let repository = repository_root()?;
    let test_root = TestDirectory::create("generation-negative")?;
    copy_manifest_files(&repository, &test_root.path)?;
    let root = test_root.path.to_string_lossy();

    fs::write(test_root.path.join(MANIFEST_SOURCE), b"{ fixtures = [")?;
    let malformed = run(&["generate-manifest", "--root", &root])?;
    assert_rejected(&malformed, "manifest Nickel export failed");

    let fake_evaluator = test_root.path.join("fake-nickel");
    fs::write(&fake_evaluator, b"#!/bin/sh\necho 'nickel 9.9.9'\n")?;
    fs::set_permissions(
        &fake_evaluator,
        fs::Permissions::from_mode(FAKE_EVALUATOR_MODE),
    )?;
    let evaluator = fake_evaluator.to_string_lossy();
    let stale = run(&[
        "generate-manifest",
        "--root",
        &root,
        "--evaluator-command",
        &evaluator,
    ])?;
    assert_rejected(&stale, "manifest evaluator descriptor drift");
    Ok(())
}
