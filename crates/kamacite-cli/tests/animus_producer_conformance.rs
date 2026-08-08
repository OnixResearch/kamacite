#![forbid(unsafe_code)]

// r[verify kamacite.application_host_binding.artifact]
// r[verify kamacite.application_host_binding.effects]
// r[verify kamacite.application_host_binding.cohorts]
// r[verify kamacite.application_host_binding.fixtures]

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

const MANIFEST_PATH: &str = "fixtures/manifest.json";
const PRODUCER_PATH: &str = "fixtures/application-host-binding/animus-producer.json";
const CONFORMANCE_PATH: &str = "fixtures/application-host-binding/animus-conformance.json";
const FIXTURE_ID: &str = "application-host-binding.positive.animus-producer";
const PRODUCER_SCHEMA: &str = "animus.application-host-binding-producer.v1";
const PRODUCER_REVISION: &str = "c09991fa299f1f74b04d066bf31382e34108fa82";
const SOURCE_IDENTITY: &str = "1290a95e48b531cc15386490844178a6ed49f25d6c1355e9a29dfea972bfcc62";
const ACCEPTED_VERDICT: &str = "accepted";

fn repository_root() -> Result<PathBuf, std::io::Error> {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

fn read_json(root: &Path, relative_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let bytes = fs::read(root.join(relative_path))?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn fixture_payload<'a>(manifest: &'a Value, fixture_id: &str) -> Result<&'a Value, &'static str> {
    let fixtures = manifest
        .get("fixtures")
        .and_then(Value::as_array)
        .ok_or("manifest fixtures are missing")?;
    let matching: Vec<&Value> = fixtures
        .iter()
        .filter(|fixture| fixture.get("id").and_then(Value::as_str) == Some(fixture_id))
        .collect();
    if matching.len() != 1 {
        return Err("manifest must contain exactly one Animus producer fixture");
    }
    matching[0]
        .get("payload")
        .ok_or("Animus producer fixture payload is missing")
}

fn validate_exact_producer(producer: &Value, manifest: &Value) -> Result<(), &'static str> {
    if producer.get("schema").and_then(Value::as_str) != Some(PRODUCER_SCHEMA) {
        return Err("Animus producer schema differs from the frozen schema");
    }
    if producer.get("producer_revision").and_then(Value::as_str) != Some(PRODUCER_REVISION) {
        return Err("Animus producer revision differs from the frozen revision");
    }
    if producer.get("source_identity").and_then(Value::as_str) != Some(SOURCE_IDENTITY) {
        return Err("Animus source identity differs from the frozen identity");
    }
    let binding = producer
        .get("binding")
        .ok_or("Animus producer binding is missing")?;
    if fixture_payload(manifest, FIXTURE_ID)? != binding {
        return Err("Kamacite fixture differs from the exact Animus producer binding");
    }
    Ok(())
}

#[test]
fn exact_animus_producer_revision_and_fixture_are_frozen() -> Result<(), Box<dyn std::error::Error>>
{
    let root = repository_root()?;
    let producer = read_json(&root, PRODUCER_PATH)?;
    let manifest = read_json(&root, MANIFEST_PATH)?;

    assert_eq!(validate_exact_producer(&producer, &manifest), Ok(()));

    let golden = read_json(
        &root,
        "fixtures/golden/application-host-binding.positive.animus-producer.receipt.json",
    )?;
    assert_eq!(
        golden.pointer("/body/verdict").and_then(Value::as_str),
        Some(ACCEPTED_VERDICT)
    );
    assert_eq!(
        golden.pointer("/body/fixture_id").and_then(Value::as_str),
        Some(FIXTURE_ID)
    );

    let conformance = read_json(&root, CONFORMANCE_PATH)?;
    assert_eq!(
        conformance.get("verdict").and_then(Value::as_str),
        Some(ACCEPTED_VERDICT)
    );
    assert_eq!(
        conformance
            .pointer("/producer/revision")
            .and_then(Value::as_str),
        Some(PRODUCER_REVISION)
    );
    assert_eq!(
        conformance
            .pointer("/producer/source_identity")
            .and_then(Value::as_str),
        Some(SOURCE_IDENTITY)
    );
    assert_eq!(
        conformance
            .pointer("/kamacite/golden_receipt_identity/hex")
            .and_then(Value::as_str),
        golden
            .get("receipt_identity")
            .and_then(|identity| identity.get("hex"))
            .and_then(Value::as_str)
    );
    Ok(())
}

#[test]
fn changed_animus_producer_binding_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let root = repository_root()?;
    let mut producer = read_json(&root, PRODUCER_PATH)?;
    let manifest = read_json(&root, MANIFEST_PATH)?;
    producer["binding"]["effect_summary_closed"] = Value::Bool(false);

    assert_eq!(
        validate_exact_producer(&producer, &manifest),
        Err("Kamacite fixture differs from the exact Animus producer binding")
    );
    Ok(())
}

#[test]
fn changed_animus_producer_revision_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let root = repository_root()?;
    let mut producer = read_json(&root, PRODUCER_PATH)?;
    let manifest = read_json(&root, MANIFEST_PATH)?;
    producer["producer_revision"] = Value::String(String::from("stale"));

    assert_eq!(
        validate_exact_producer(&producer, &manifest),
        Err("Animus producer revision differs from the frozen revision")
    );
    Ok(())
}
