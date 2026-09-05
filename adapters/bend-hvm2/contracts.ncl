let StringArray = Array String in
let NumberArray = Array Number in
let SourcePin = {
  repository | String,
  revision | String,
  version | String,
  nix_nar_sha256 | String,
  license | String,
  license_path | String,
  license_blake3 | String,
  attribution | String,
} in
let CommandProfile = {
  id | String,
  executable | String,
  arguments | StringArray,
} in
let CompilerProfile = {
  family | String,
  version | String,
  target | String,
  flags | StringArray,
} in
let Cohort = {
  bend | SourcePin,
  hvm2 | SourcePin,
  bend_hvm_dependency | String,
  compiler | CompilerProfile,
  command_profiles | Array CommandProfile,
  excluded_toolchains | StringArray,
} in
let Workload = {
  id | String,
  algorithm_profile | String,
  leaves | NumberArray,
  leaf_limit | Number,
  numeric_type | String,
  modulus | Number,
  result | Number,
  source_path | String,
  canonical_input_path | String,
  required_annotations | StringArray,
  forbidden_source_tokens | StringArray,
} in
let Resources = {
  timeout_milliseconds | Number,
  cpu_seconds | Number,
  memory_limit_bytes | Number,
  output_limit_bytes | Number,
  diagnostic_limit_bytes | Number,
  artifact_limit_bytes | Number,
  file_limit_bytes | Number,
  process_limit | Number,
} in
let Benchmark = {
  warmup_runs | Number,
  measured_runs | Number,
  statistic | String,
  maximum_spread_basis_points | Number,
  maximum_generated_c_oracle_ratio | Number,
  maximum_run_rs_oracle_ratio | Number,
  maximum_stage_wall_milliseconds | Number,
} in
let Trellis = {
  repository | String,
  revision | String,
  nix_nar_sha256 | String,
  proof_bundle_path | String,
  proof_bundle_blake3 | String,
  scope | String,
  algorithm_profile | String,
  executable_oracle_symbol | String,
  proof_command | String,
  verifier_toolchain | String,
  verifier_provider_revision | String,
  proof_check | String,
  required_requirement_ids | StringArray,
  required_proof_functions | StringArray,
  required_non_claims | StringArray,
} in
let ProfileContract = {
  schema_version | String,
  profile_id | String,
  disposition | String,
  canonical_ir | String,
  cohort | Cohort,
  workload | Workload,
  resources | Resources,
  benchmark | Benchmark,
  trellis | Trellis,
  denied_authority | StringArray,
  required_parent_roles | StringArray,
  checked_laws | StringArray,
  non_claims | StringArray,
} in
{
  Profile = ProfileContract,
}
