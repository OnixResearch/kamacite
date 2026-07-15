let Natural = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value >= 0
) in
let StringArray = Array String in
let Limits = {
  max_input_bytes | Natural,
  max_output_bytes | Natural,
  max_depth | Natural,
  max_collection_items | Natural,
  max_total_values | Natural,
  max_string_bytes | Natural,
  max_numeric_lexeme_bytes | Natural,
} in
let ProfileContract = {
  schema_version | String,
  profile_id | String,
  accepted_manifest_schemas | StringArray,
  accepted_receipt_schemas | StringArray,
  mapping_profile | String,
  preserves_schema_version | String,
  preserves_root_label | String,
  canonical_codec | String,
  output_classes | StringArray,
  limits | Limits,
  numeric_policy | String,
  object_order | String,
  unicode_policy | String,
  checked_laws | StringArray,
  non_claims | StringArray,
} in
{
  Profile = ProfileContract,
}
