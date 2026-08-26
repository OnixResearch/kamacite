use kamacite_cli::root_io::RootCapability;
use kamacite_core::SourceIdentity;
use kamacite_core::world_commit::MOLTEN_PRODUCT_SOURCE_MANIFEST_HEX;
use std::collections::BTreeSet;

use crate::world_commit_shell::io::{pretty_json, read_bounded, typed_wire, write_or_compare};
use crate::world_commit_shell::model::{
    CohortConfig, ImplementationSourceEntry, ProductSourceManifest,
};

const PRODUCT_MANIFEST_SCHEMA: &str = "kamacite.molten-world-commit-product-sources.v1";

pub fn materialize_product_source_manifest(
    root: &RootCapability,
    config: &CohortConfig,
    update: bool,
) -> Result<(), String> {
    if config.product_sources.is_empty() {
        return Err("Molten product source closure is empty".to_string());
    }
    let mut paths = BTreeSet::new();
    let mut sources = Vec::new();
    for path in &config.product_sources {
        if path == &config.generated_product_manifest_path || !paths.insert(path) {
            return Err(format!("invalid Molten product source path: {path}"));
        }
        let bytes = read_bounded(root, path)?;
        let byte_length = u64::try_from(bytes.len())
            .map_err(|error| format!("product source length conversion: {error}"))?;
        sources.push(ImplementationSourceEntry {
            path: path.clone(),
            source_identity: typed_wire(&SourceIdentity::compute(&bytes)),
            byte_length,
        });
    }
    sources.sort_by(|left, right| left.path.cmp(&right.path));
    let manifest = ProductSourceManifest {
        schema: PRODUCT_MANIFEST_SCHEMA.to_string(),
        product_repository: config.product_repository.clone(),
        product_revision: config.product_revision.clone(),
        license: config.license.clone(),
        sources,
    };
    let bytes = pretty_json(&manifest)?;
    let identity = SourceIdentity::compute(&bytes);
    if identity.hex().as_str() != MOLTEN_PRODUCT_SOURCE_MANIFEST_HEX {
        return Err(format!(
            "stale or altered Molten source snapshot differs from the reviewed immutable product cohort: {}",
            identity.hex().as_str()
        ));
    }
    write_or_compare(
        root,
        &config.generated_product_manifest_path,
        &bytes,
        update,
    )
}
