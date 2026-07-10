let SourceRoot = {
  path | String,
  language | String,
  collector_profile | String,
} in
let OutputPlan = {
  format | String,
  path | String,
} in
let WasmRunnerProfile = {
  profile | String,
  engine_identity_policy | String,
  imports | Array String,
  capabilities | Array String,
  fuel_limit | Number,
  memory_limit_bytes | Number,
  output_limit_bytes | Number,
} in
let DocsConfig = {
  schema_version | Number,
  source_roots | Array SourceRoot,
  output_formats | Array OutputPlan,
  page_grouping | Array { pattern | String, page | String },
  link_policy | String,
  run_profiles | Array String,
  wasm_runner_profiles | Array WasmRunnerProfile,
  update_policy | String,
} in
DocsConfig
