#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        tigerstyle::path_segment_repetition,
        tigerstyle::unbounded_collection_growth,
        reason = "Implementation-manifest failures return typed errors; source identities require a separate API migration."
    )
)]

use crate::root_io::RootCapability;
use kamacite_core::SourceIdentity;
use std::collections::BTreeSet;

use crate::world_commit_shell::io::{pretty_json, read_bounded, typed_wire, write_or_compare};
use crate::world_commit_shell::model::{
    CohortConfig, ImplementationManifest, ImplementationSourceEntry,
};

const IMPLEMENTATION_MANIFEST_SCHEMA: &str = "kamacite.world-commit-implementation-manifest.v1";

pub fn materialize_implementation_manifest(
    root: &RootCapability,
    config: &CohortConfig,
    update: bool,
) -> Result<(), String> {
    if config.implementation_sources.is_empty() {
        return Err("world-commit implementation source closure is empty".to_string());
    }
    let mut paths = BTreeSet::new();
    let mut sources = Vec::new();
    for path in &config.implementation_sources {
        if path == &config.generated_implementation_manifest_path || !paths.insert(path) {
            return Err(format!("invalid implementation source path: {path}"));
        }
        let bytes = read_bounded(root, path)?;
        let byte_length = u64::try_from(bytes.len())
            .map_err(|error| format!("implementation source length conversion: {error}"))?;
        sources.push(ImplementationSourceEntry {
            path: path.clone(),
            source_identity: typed_wire(&SourceIdentity::compute(&bytes)),
            byte_length,
        });
    }
    sources.sort_by(|left, right| left.path.cmp(&right.path));
    let manifest = ImplementationManifest {
        schema: IMPLEMENTATION_MANIFEST_SCHEMA.to_string(),
        sources,
    };
    write_or_compare(
        root,
        &config.generated_implementation_manifest_path,
        &pretty_json(&manifest)?,
        update,
    )
}
