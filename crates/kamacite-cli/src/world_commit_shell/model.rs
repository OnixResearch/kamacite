use kamacite_core::world_commit::{
    WorldCommitValue, WorldProfileKind, WorldRootDomain, WorldSourceRole,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct CohortConfig {
    pub schema: String,
    pub product_repository: String,
    pub product_revision: String,
    pub product_owner: String,
    pub product_schema: String,
    pub canonical_codec: String,
    pub canonical_domain_tag: String,
    pub supported_version: String,
    pub license: String,
    pub sources: Vec<SourceConfig>,
    pub positive_vectors: Vec<VectorConfig>,
    pub negative_vectors: Vec<VectorConfig>,
    pub nickel_projection_path: String,
    pub nickel_contract_path: String,
    pub steel_source_path: String,
    pub steel_module_path: String,
    pub steel_input_path: String,
    pub molten_schema_path: String,
    pub molten_codec_path: String,
    pub product_sources: Vec<String>,
    pub generated_product_manifest_path: String,
    pub implementation_sources: Vec<String>,
    pub generated_implementation_manifest_path: String,
    pub generated_manifest_path: String,
    pub generated_cohort_path: String,
    pub generated_receipt_path: String,
    pub generated_receipt_projection_path: String,
    pub generated_valence_path: String,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SourceConfig {
    pub role: WorldSourceRole,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct VectorConfig {
    pub id: String,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProductSourceManifest {
    pub schema: String,
    pub product_repository: String,
    pub product_revision: String,
    pub license: String,
    pub sources: Vec<ImplementationSourceEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ImplementationManifest {
    pub schema: String,
    pub sources: Vec<ImplementationSourceEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ImplementationSourceEntry {
    pub path: String,
    pub source_identity: String,
    pub byte_length: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WorldVectorManifest {
    pub schema: String,
    pub product_repository: String,
    pub product_revision: String,
    pub product_schema: String,
    pub canonical_codec: String,
    pub positive_vectors: Vec<PositiveVectorManifestEntry>,
    pub negative_vectors: Vec<NegativeVectorManifestEntry>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PositiveVectorManifestEntry {
    pub id: String,
    pub source_path: String,
    pub packed_path: String,
    pub source_identity: String,
    pub packed_identity: String,
    pub canonical_value_identity: String,
    pub molten_world_commit_ref: String,
    pub profile: WorldProfileKind,
    pub root_domains: Vec<WorldRootDomain>,
    pub parent_count: usize,
    pub root_count: usize,
    pub completeness_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NegativeVectorManifestEntry {
    pub id: String,
    pub source_path: String,
    pub source_identity: String,
    pub diagnostic_classes: Vec<String>,
    pub verdict: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReceiptProjection<'a> {
    pub schema: &'a str,
    pub receipt_identity: String,
    pub canonical_value_identity: String,
    pub molten_world_commit_ref: &'a str,
    pub cohort_identity: String,
    pub adapter_count: usize,
    pub verdict: &'a str,
    pub checked_laws: &'a [String],
    pub non_claims: &'a [String],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostileClaimVector {
    pub schema: String,
    pub class: String,
    pub claim: String,
    pub current_identity: Option<String>,
    pub proposed_identity: Option<String>,
    pub expected_verdict: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AlteredSourceVector {
    pub schema: String,
    pub source_path: String,
    pub replacement_text: String,
    pub expected_verdict: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VectorCorpus {
    pub manifest: WorldVectorManifest,
    pub primary: WorldCommitValue,
    pub primary_packed: Vec<u8>,
}
