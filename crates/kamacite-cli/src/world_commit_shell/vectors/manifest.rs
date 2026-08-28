#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::assertion_density,
        tigerstyle::compound_condition,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        tigerstyle::unbounded_collection_growth,
        reason = "Manifest validators keep closed hostile-vector fields visible and return typed errors."
    )
)]

use crate::root_io::RootCapability;
use kamacite_core::SourceIdentity;
use kamacite_core::world_commit::{
    MAX_WORLD_COMMIT_PARENTS, ROOT_DOMAIN_INVENTORY, WorldCohortUpdateRequest,
    WorldConformanceIssue, WorldProfileKind, admit_world_commit_packed,
    canonicalize_world_commit_text, plan_world_commit_cohort_update,
};
use std::collections::BTreeSet;

use crate::steel_runner::validate_world_vector_cohort;
use crate::world_commit_shell::io::{
    from_json, pretty_json, read_bounded, typed_wire, write_or_compare,
};
use crate::world_commit_shell::model::{
    AlteredSourceVector, CohortConfig, HostileClaimVector, NegativeVectorManifestEntry,
    PositiveVectorManifestEntry, VectorCorpus, WorldVectorManifest,
};

const MANIFEST_SCHEMA: &str = "kamacite.world-commit-vector-manifest.v1";
const PRIMARY_VECTOR_ID: &str = "logical-v1";
const ORDERED_VECTOR_ID: &str = "ordered-parents-v1";
const BOUNDED_VECTOR_ID: &str = "bounded-parents-v1";
const ORDERED_PARENT_COUNT: usize = 2;
const ALTERED_SOURCE_VECTOR_ID: &str = "altered-source";
const UPDATE_VECTOR_ID: &str = "unapproved-cohort-update";
const NONCANONICAL_VECTOR_ID: &str = "noncanonical-packed";
const UNDECLARED_STEEL_VECTOR_ID: &str = "undeclared-steel-authority";
const CODEC_DRIFT_VECTOR_ID: &str = "codec-drift";
const JSON_AUTHORITY_VECTOR_ID: &str = "json-as-authority";
const SEMANTIC_OVERCLAIM_VECTOR_ID: &str = "semantic-equivalence-overclaim";

// r[impl kamacite.world_commit_conformance.verification]
pub fn build_vector_corpus(
    root: &RootCapability,
    config: &CohortConfig,
    update: bool,
) -> Result<VectorCorpus, String> {
    validate_unique_ids(config)?;
    let mut positive_vectors = Vec::new();
    let mut primary = None;
    for vector in &config.positive_vectors {
        let source = read_bounded(root, &vector.path)?;
        let admitted = canonicalize_world_commit_text(&source)
            .map_err(|issues| format!("positive vector {} denied: {issues:?}", vector.id))?;
        let packed_path = packed_path(&vector.id);
        write_or_compare(root, &packed_path, &admitted.packed_bytes, update)?;
        if vector.id == PRIMARY_VECTOR_ID {
            primary = Some(admitted.clone());
        }
        positive_vectors.push(PositiveVectorManifestEntry {
            id: vector.id.clone(),
            source_path: vector.path.clone(),
            packed_path,
            source_identity: typed_wire(&SourceIdentity::compute(&source)),
            packed_identity: typed_wire(&SourceIdentity::compute(&admitted.packed_bytes)),
            canonical_value_identity: typed_wire(&admitted.value_identity),
            molten_world_commit_ref: admitted.molten_world_commit_ref,
            profile: admitted.value.profile.kind,
            root_domains: admitted
                .value
                .roots
                .iter()
                .map(|root| root.domain)
                .collect(),
            parent_count: admitted.value.parents.len(),
            root_count: admitted.value.roots.len(),
            completeness_count: admitted.value.completeness.len(),
        });
    }
    verify_positive_coverage(&positive_vectors)?;
    let negative_vectors = build_negative_manifest(root, config)?;
    let manifest = WorldVectorManifest {
        schema: MANIFEST_SCHEMA.to_string(),
        product_repository: config.product_repository.clone(),
        product_revision: config.product_revision.clone(),
        product_schema: config.product_schema.clone(),
        canonical_codec: config.canonical_codec.clone(),
        positive_vectors,
        negative_vectors,
        non_claims: config.non_claims.clone(),
    };
    write_or_compare(
        root,
        &config.generated_manifest_path,
        &pretty_json(&manifest)?,
        update,
    )?;
    let primary = primary.ok_or_else(|| "primary logical vector is absent".to_string())?;
    Ok(VectorCorpus {
        manifest,
        primary: primary.value,
        primary_packed: primary.packed_bytes,
    })
}

fn build_negative_manifest(
    root: &RootCapability,
    config: &CohortConfig,
) -> Result<Vec<NegativeVectorManifestEntry>, String> {
    let mut entries = Vec::new();
    for vector in &config.negative_vectors {
        let source = read_bounded(root, &vector.path)?;
        let diagnostic_classes = match vector.id.as_str() {
            ALTERED_SOURCE_VECTOR_ID => {
                let altered: AlteredSourceVector = from_json(&source, &vector.id)?;
                if altered.expected_verdict != "deny" || altered.replacement_text.is_empty() {
                    return Err("altered-source vector does not require denial".to_string());
                }
                vec!["world-source-identity".to_string()]
            }
            UPDATE_VECTOR_ID => {
                let request: WorldCohortUpdateRequest = from_json(&source, &vector.id)?;
                if plan_world_commit_cohort_update(&request).admitted {
                    return Err("unapproved cohort update vector was admitted".to_string());
                }
                vec!["world-cohort-update-denied".to_string()]
            }
            NONCANONICAL_VECTOR_ID => match admit_world_commit_packed(&source) {
                Ok(_) => return Err("noncanonical packed vector was admitted".to_string()),
                Err(issues) => issue_codes(&issues),
            },
            UNDECLARED_STEEL_VECTOR_ID => {
                let module = read_bounded(root, &config.steel_module_path)?;
                let input = read_bounded(root, &config.steel_input_path)?;
                if validate_world_vector_cohort(&source, &module, &input).is_ok() {
                    return Err("undeclared Steel authority vector was admitted".to_string());
                }
                vec!["world-adapter-boundary".to_string()]
            }
            CODEC_DRIFT_VECTOR_ID | JSON_AUTHORITY_VECTOR_ID | SEMANTIC_OVERCLAIM_VECTOR_ID => {
                validate_hostile_claim(&source, &vector.id)?
            }
            _ => {
                let issues = match canonicalize_world_commit_text(&source) {
                    Ok(_) => {
                        return Err(format!(
                            "negative Preserves vector was admitted: {}",
                            vector.id
                        ));
                    }
                    Err(issues) => issues,
                };
                issue_codes(&issues)
            }
        };
        entries.push(NegativeVectorManifestEntry {
            id: vector.id.clone(),
            source_path: vector.path.clone(),
            source_identity: typed_wire(&SourceIdentity::compute(&source)),
            diagnostic_classes,
            verdict: "deny".to_string(),
        });
    }
    Ok(entries)
}

fn validate_hostile_claim(source: &[u8], id: &str) -> Result<Vec<String>, String> {
    let vector: HostileClaimVector = from_json(source, id)?;
    if vector.schema != "kamacite.world-commit-hostile-claim.v1"
        || vector.class != id
        || vector.claim.is_empty()
        || vector.expected_verdict != "deny"
    {
        return Err(format!("hostile claim vector is malformed: {id}"));
    }
    if id == CODEC_DRIFT_VECTOR_ID {
        if vector.current_identity.is_none()
            || vector.proposed_identity.is_none()
            || vector.current_identity == vector.proposed_identity
        {
            return Err("codec-drift vector does not describe identity drift".to_string());
        }
        return Ok(vec!["world-cohort-field".to_string()]);
    }
    Ok(vec!["world-semantic-overclaim".to_string()])
}

fn issue_codes(issues: &[WorldConformanceIssue]) -> Vec<String> {
    let mut codes = issues
        .iter()
        .map(|issue| issue.code().to_string())
        .collect::<Vec<_>>();
    codes.sort();
    codes.dedup();
    codes
}

fn verify_positive_coverage(entries: &[PositiveVectorManifestEntry]) -> Result<(), String> {
    let profiles = entries
        .iter()
        .map(|entry| entry.profile)
        .collect::<BTreeSet<_>>();
    let expected_profiles = [
        WorldProfileKind::Logical,
        WorldProfileKind::Opaque,
        WorldProfileKind::Mixed,
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    if profiles != expected_profiles {
        return Err("positive vectors do not cover every snapshot profile".to_string());
    }
    let domains = entries
        .iter()
        .flat_map(|entry| entry.root_domains.iter().copied())
        .collect::<BTreeSet<_>>();
    let expected_domains = ROOT_DOMAIN_INVENTORY.into_iter().collect::<BTreeSet<_>>();
    if domains != expected_domains {
        return Err("positive vectors do not cover every nominal root domain".to_string());
    }
    let ordered = entry(entries, ORDERED_VECTOR_ID)?;
    if ordered.parent_count != ORDERED_PARENT_COUNT {
        return Err("ordered parent vector has the wrong parent count".to_string());
    }
    let bounded = entry(entries, BOUNDED_VECTOR_ID)?;
    if bounded.parent_count != MAX_WORLD_COMMIT_PARENTS {
        return Err("bounded parent vector does not reach the admitted limit".to_string());
    }
    let primary = entry(entries, PRIMARY_VECTOR_ID)?;
    if primary.parent_count != 0 {
        return Err("primary vector does not cover the empty parent collection".to_string());
    }
    Ok(())
}

fn validate_unique_ids(config: &CohortConfig) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    for vector in config
        .positive_vectors
        .iter()
        .chain(&config.negative_vectors)
    {
        if vector.id.is_empty() || !ids.insert(&vector.id) {
            return Err(format!(
                "fixture vector id is empty or duplicated: {}",
                vector.id
            ));
        }
    }
    Ok(())
}

fn entry<'a>(
    entries: &'a [PositiveVectorManifestEntry],
    id: &str,
) -> Result<&'a PositiveVectorManifestEntry, String> {
    entries
        .iter()
        .find(|entry| entry.id == id)
        .ok_or_else(|| format!("positive vector is absent: {id}"))
}

fn packed_path(id: &str) -> String {
    format!("fixtures/world-commit/vectors/packed/{id}.preserves.bin")
}
