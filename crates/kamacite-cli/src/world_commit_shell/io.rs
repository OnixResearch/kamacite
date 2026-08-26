use std::path::Path;

use kamacite_cli::root_io::RootCapability;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub const MAX_FIXTURE_BYTES: usize = 2_097_152;

pub fn read_bounded(root: &RootCapability, relative: &str) -> Result<Vec<u8>, String> {
    let bytes = root
        .read(Path::new(relative))
        .map_err(|error| format!("read {relative}: {error}"))?;
    if bytes.len() > MAX_FIXTURE_BYTES {
        return Err(format!("{relative} exceeds the fixture byte limit"));
    }
    Ok(bytes)
}

pub fn write_or_compare(
    root: &RootCapability,
    relative: &str,
    expected: &[u8],
    update: bool,
) -> Result<(), String> {
    if expected.len() > MAX_FIXTURE_BYTES {
        return Err(format!("{relative} exceeds the fixture byte limit"));
    }
    if update {
        return root
            .write(Path::new(relative), expected)
            .map_err(|error| format!("write {relative}: {error}"));
    }
    let actual = root
        .read(Path::new(relative))
        .map_err(|error| format!("read {relative}: {error}"))?;
    if actual != expected {
        return Err(format!("generated artifact is stale: {relative}"));
    }
    Ok(())
}

pub fn from_json<T: DeserializeOwned>(bytes: &[u8], label: &str) -> Result<T, String> {
    serde_json::from_slice(bytes).map_err(|error| format!("decode {label}: {error}"))
}

pub fn pretty_json<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("encode deterministic JSON: {error}"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn wire(identity: &kamacite_core::WireIdentity) -> String {
    format!("{}:{}", identity.domain, identity.hex.as_str())
}

pub fn typed_wire<D: kamacite_core::IdentityDomainMarker>(
    identity: &kamacite_core::Identity<D>,
) -> String {
    wire(&identity.to_wire())
}
