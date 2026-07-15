let StringArray = Array String in
let Fixture = {
  fixture_id | String,
  class | String,
  expected_state | String,
  expected_blockers | StringArray,
  expected_diagnostic_codes | StringArray,
  non_claims | StringArray,
} in
let ManifestContract = {
  schema_version | String,
  adapter_profile | String,
  source_revision | String,
  fixtures | Array Fixture,
} in
{
  Manifest = ManifestContract,
}
