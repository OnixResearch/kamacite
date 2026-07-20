let Contracts = import "contracts.ncl" in
let BytesPerKiB = 1024 in
let KiB = BytesPerKiB in
let MiB = KiB * KiB in
let ComponentFuel = 100000 in
let MaxComponentBytes = 4 * MiB in
let MaxWitBytes = 64 * KiB in
let MaxMemoryBytes = 16 * MiB in
let MaxTableElements = 1024 in
let MaxInstances = 16 in
let MaxMemories = 4 in
let MaxTables = 4 in
let MaxStackBytes = 512 * KiB in
let MaxHostcallBytes = 64 * KiB in
let MaxResultBytes = 64 * KiB in
let MaxConcurrency = 1 in
let MaxImports = 128 in
let MaxExports = 128 in
let Profile = {
  schema_version = "kamacite.wasm-component-static-profile.v1",
  profile_id = "kamacite.wit-component-adapter.v1",
  source = {
    repository = "https://github.com/OnixResearch/aspen",
    revision = "62b02a3435a7000a85bbb6988a88f8afe5effbbb",
    runtime_profile_path = "docs/wasm-component-runtime/generated/profile.json",
    runtime_profile_blake3 = "c2ef4db1ab1010512b0d7b023e6a64f80c00c11850e041fa5347d91028d8d96f",
    wit_path = "wit/molten-component-runtime/runtime.wit",
    wit_blake3 = "83be349bb27975ada30dbe60817c5404df7862babdf02ae229b399886e76d5e8",
  },
  runtime = {
    schema_id = "molten.wasm-component-profile.v1",
    schema_version = 1,
    source_language = "nickel",
    profile = {
      profile_id = "molten.wasm.component.v1",
    evidence_scope = "production",
    runtime_strategy = "portable-component",
    toolchain = {
      wasmtime = "45.0.0",
      wasm_tools = "1.240.0",
      wasmparser = "0.240.0",
      wit_bindgen = "0.46.0",
      wasmtime_wasi = "45.0.0",
      wasi_package = "0.2.6",
    },
    wit = {
      package = "molten:component-runtime@1.0.0",
      world = "actor",
      source_ref = "blake3:83be349bb27975ada30dbe60817c5404df7862babdf02ae229b399886e76d5e8",
    },
    features = {
      component_model = true,
      multi_value = true,
      bulk_memory = true,
      reference_types = true,
      simd = true,
      relaxed_simd = false,
      threads = false,
      tail_call = false,
      multi_memory = false,
      exceptions = false,
      gc = false,
      memory64 = false,
      extended_const = false,
      function_references = false,
      custom_page_sizes = false,
      wide_arithmetic = false,
      component_async = false,
    },
    determinism = {
      fuel_interruption = true,
      nan_canonicalization = true,
      relaxed_simd_deterministic = true,
      memory_growth = "fixed",
      table_growth = "fixed",
      host_inputs = "recorded-effects-only",
    },
    resources = {
      fuel = ComponentFuel,
      max_component_bytes = MaxComponentBytes,
      max_wit_bytes = MaxWitBytes,
      max_memory_bytes = MaxMemoryBytes,
      max_table_elements = MaxTableElements,
      max_instances = MaxInstances,
      max_memories = MaxMemories,
      max_tables = MaxTables,
      max_stack_bytes = MaxStackBytes,
      max_hostcall_bytes = MaxHostcallBytes,
      max_result_bytes = MaxResultBytes,
      max_concurrency = MaxConcurrency,
      max_imports = MaxImports,
      max_exports = MaxExports,
    },
    allowed_imports = [],
    allowed_wasi_interfaces = [],
      non_claims = [
        "not-behavioral-correctness",
        "not-semantic-equivalence",
        "not-authority",
        "not-release-eligibility",
        "not-whole-system-safety",
        "not-build-trust-transfer",
      ],
    },
  },
  adapter = {
    descriptor_schema = "kamacite.wasm-component-descriptor.v1",
    materialization_schema = "mantle-wasm-component-materialization-bundle-v1",
    artifact_kind = "component",
    invoke_export = "invoke",
    input_carrier = "canonical-preserves-bytes",
    output_carrier = "canonical-preserves-bytes",
    error_carrier = "utf8-string",
    canonical_codec = "kamacite-preserves-packed-canonical-v2-preserves-rs-5.0.0-rc.7",
    production_materialization = "exact-mantle-bundle-required",
    initial_import_policy = "deny-all",
    result_classes = ["denied", "error", "success", "trap"],
    checked_laws = [
      "wit-source-identity-bound",
      "runtime-profile-identity-bound",
      "descriptor-layers-distinct",
      "materialization-remeasured",
      "canonical-input-before-planning",
      "ambient-imports-denied",
      "missing-production-evidence-denies-execution",
    ],
    non_claims = [
      "WIT and WebAssembly are target artifacts, not Kamacite canonical IR",
      "static validation is not component execution or behavioral correctness",
      "profile compatibility is not Wasmtime correctness or sandbox completeness",
      "materialization references do not transfer Mantle build trust",
      "capability declarations do not grant host authority",
      "no arbitrary Nickel, Rust, Steel, or Wasm semantic equivalence",
      "no release eligibility or production readiness",
    ],
  },
} in
Profile | Contracts.Profile
