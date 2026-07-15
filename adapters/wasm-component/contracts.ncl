let PositiveInteger = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value > 0
) in
let StringArray = Array String in
let Source = {
  repository | String,
  revision | String,
  runtime_profile_path | String,
  runtime_profile_blake3 | String,
  wit_path | String,
  wit_blake3 | String,
} in
let Toolchain = {
  wasmtime | String,
  wasm_tools | String,
  wasmparser | String,
  wit_bindgen | String,
  wasmtime_wasi | String,
  wasi_package | String,
} in
let Wit = {
  package | String,
  world | String,
  source_ref | String,
} in
let Features = {
  component_model | Bool,
  multi_value | Bool,
  bulk_memory | Bool,
  reference_types | Bool,
  simd | Bool,
  relaxed_simd | Bool,
  threads | Bool,
  tail_call | Bool,
  multi_memory | Bool,
  exceptions | Bool,
  gc | Bool,
  memory64 | Bool,
  extended_const | Bool,
  function_references | Bool,
  custom_page_sizes | Bool,
  wide_arithmetic | Bool,
  component_async | Bool,
} in
let Determinism = {
  fuel_interruption | Bool,
  nan_canonicalization | Bool,
  relaxed_simd_deterministic | Bool,
  memory_growth | String,
  table_growth | String,
  host_inputs | String,
} in
let Resources = {
  fuel | PositiveInteger,
  max_component_bytes | PositiveInteger,
  max_wit_bytes | PositiveInteger,
  max_memory_bytes | PositiveInteger,
  max_table_elements | PositiveInteger,
  max_instances | PositiveInteger,
  max_memories | PositiveInteger,
  max_tables | PositiveInteger,
  max_stack_bytes | PositiveInteger,
  max_hostcall_bytes | PositiveInteger,
  max_result_bytes | PositiveInteger,
  max_concurrency | PositiveInteger,
  max_imports | PositiveInteger,
  max_exports | PositiveInteger,
} in
let RuntimeProfile = {
  profile_id | String,
  evidence_scope | String,
  runtime_strategy | String,
  toolchain | Toolchain,
  wit | Wit,
  features | Features,
  determinism | Determinism,
  resources | Resources,
  allowed_imports | StringArray,
  allowed_wasi_interfaces | StringArray,
  non_claims | StringArray,
} in
let Runtime = {
  schema_id | String,
  schema_version | PositiveInteger,
  source_language | String,
  profile | RuntimeProfile,
} in
let Adapter = {
  descriptor_schema | String,
  materialization_schema | String,
  artifact_kind | String,
  invoke_export | String,
  input_carrier | String,
  output_carrier | String,
  error_carrier | String,
  canonical_codec | String,
  production_materialization | String,
  initial_import_policy | String,
  result_classes | StringArray,
  checked_laws | StringArray,
  non_claims | StringArray,
} in
let ProfileContract = {
  schema_version | String,
  profile_id | String,
  source | Source,
  runtime | Runtime,
  adapter | Adapter,
} in
{
  Profile = ProfileContract,
}
