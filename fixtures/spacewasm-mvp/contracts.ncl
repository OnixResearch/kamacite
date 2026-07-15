let StringArray = Array String in
let FixtureContract = {
  fixture_id | String,
  class | String,
  phase | String,
  expected_state | String,
  expected_blockers | StringArray,
  expected_diagnostic_codes | StringArray,
  non_claims | StringArray,
} in
let ManifestContract = {
  schema_version | String,
  profile_id | String,
  mantle_revision | String,
  octet_revision | String,
  spacewasm_revision | String,
  fixtures | Array FixtureContract,
} in
{
  Manifest = ManifestContract,
}
