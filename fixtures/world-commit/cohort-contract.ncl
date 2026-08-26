let Source = {
  role | String,
  path | String,
} in
let Vector = {
  id | String,
  path | String,
} in
let WorldCommitCohort = {
  schema | String,
  product_repository | String,
  product_revision | String,
  product_owner | String,
  product_schema | String,
  canonical_codec | String,
  canonical_domain_tag | String,
  supported_version | String,
  license | String,
  sources | Array Source,
  positive_vectors | Array Vector,
  negative_vectors | Array Vector,
  nickel_projection_path | String,
  nickel_contract_path | String,
  steel_source_path | String,
  steel_module_path | String,
  steel_input_path | String,
  molten_schema_path | String,
  molten_codec_path | String,
  product_sources | Array String,
  generated_product_manifest_path | String,
  implementation_sources | Array String,
  generated_implementation_manifest_path | String,
  generated_manifest_path | String,
  generated_cohort_path | String,
  generated_receipt_path | String,
  generated_receipt_projection_path | String,
  generated_valence_path | String,
  checked_laws | Array String,
  non_claims | Array String,
} in
{
  Cohort = WorldCommitCohort,
}
