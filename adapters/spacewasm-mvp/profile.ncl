let Contracts = import "contracts.ncl" in
let kibibyte = 1024 in
let mebibyte = kibibyte * kibibyte in
let maximum_artifact_bytes = mebibyte in
let maximum_transport_bytes = mebibyte in
let maximum_hostcall_bytes = 64 * kibibyte in
let maximum_exports = 16 in
let maximum_functions = 4096 in
let maximum_memories = 1 in
let maximum_tables = 1 in
let maximum_guest_memory_pages = 256 in
let maximum_guest_table_elements = 1024 in
let maximum_interpreter_code_pages = 64 in
let maximum_interpreter_control_frames = 64 in
let maximum_interpreter_stack_words = 1024 in
let default_interpreter_fuel = 1024 in
let maximum_interpreter_stream_chunks = 4096 in
let mantle_revision = "a141fcbaafe41f9a413a81275a33fe915bfca370" in
let octet_revision = "4367300e10740ecc99ba4b2171ace561b4787327" in
let spacewasm_revision = "e24cf09355a90497148eb5029fdb8e3400bd63e3" in
let Profile = {
  schema_version = "kamacite.spacewasm-mvp-profile.v1",
  profile_id = "kamacite-spacewasm-mvp",
  adapter_kind = "spacewasm-core-module",
  artifact_kind = "webassembly-1.0-core-module",
  promotion_state = "experimental-blocked",
  canonical_ir = "packed-preserves-bytes",
  mantle = {
    source = {
      repository = "https://github.com/OnixResearch/mantle",
      revision = mantle_revision,
    },
    bundle_schema = "mantle-spacewasm-reference-bundle-v1",
    bundle_identity_blake3 = "7775e57a4d07da89ca57ff282ab0d321f1adf70e9d650f3de4e3a329c92a71e6",
    bundle_manifest = {
      path = "upstream/mantle/bundle-manifest.json",
      blake3 = "e3f4afcee53cf9bf0401a2aedc5ca47c5d53b92e01dc3b72f2dadc657abd5562",
    },
    materialization_report = {
      path = "upstream/mantle/materialization-report.json",
      blake3 = "4b9a91e0ef9ac2774ae053f70e690485f516b52b42e1dbcc1c96cdae6b66bece",
    },
    profile = {
      path = "upstream/mantle/profile.json",
      blake3 = "742ab4c8f4eb81f49de04f8eb74d5b7a87d43638215bd0a71d2ccd8e493d0348",
    },
    module = {
      path = "upstream/mantle/mvp-positive.wasm",
      blake3 = "b2568cffcad3c442a90dd65ccb82d6b7e285bf71c2c9908ce539b6b515217bde",
    },
    verifier_summary = {
      path = "upstream/mantle/bundle-verification.json",
      blake3 = "969095deb65f9ca7bfcfe1193a696f8d4d3d35632679fc7dca7d3292739416d5",
    },
    materialization_profile_id = "nasa-spacewasm-e24cf093-reference-v1",
    materialization_profile_identity = "cceb1bd03f90dd382d4c7ff6e79266650830a7c87e9fae07db2e48773469eb67",
    materialization_cohort_identity = "fb2c9e84459271828ad6840093c80af359508f47b06125f6741b14a31c9cc103",
  },
  octet = {
    source = {
      repository = "https://github.com/OnixResearch/octet",
      revision = octet_revision,
    },
    profile_id = "spacewasm-mvp",
    profile_identity = "5d6d9f57001dbc698e017c6b2fa59412131596712722d42c718f7493ff497d9e",
    registry_identity = "f58715eb73f91d8a6fffd565cc3b030e7f1f8296ec8e91851155d1bb5e52bafd",
    cohort_identity = "c50e2d7f0e8c49de4a1d44afae196bdf96bb14e67e7de0a153de146a6207449a",
    runtime_cohort_identity = "7c537eef7bff66d0a90508715de08de79c9aaac23352e1007a6d47e7fb4674a2",
    registry = {
      path = "upstream/octet/profiles.json",
      blake3 = "f58715eb73f91d8a6fffd565cc3b030e7f1f8296ec8e91851155d1bb5e52bafd",
    },
    support_projection = {
      path = "upstream/octet/support-projection.json",
      blake3 = "7f3c40650c576bf9340d07df5371da28fad42ef4937d1d9ce9c2c8bc80a5b638",
    },
    report = {
      path = "upstream/octet/evidence-rail-receipt.json",
      blake3 = "0362dec1532a9a138f27e1b70d49cd68bf5d6e27653f11cc3af081f0a69a77ab",
    },
    module = {
      path = "upstream/octet/wasm-artifact.bin",
      blake3 = "b2568cffcad3c442a90dd65ccb82d6b7e285bf71c2c9908ce539b6b515217bde",
    },
    verification_role = "boundary",
  },
  abi = {
    profile_id = "kamacite.spacewasm.bytes.v1",
    memory_export = "memory",
    allocation_export = "kamacite_alloc",
    deallocation_export = "kamacite_dealloc",
    entry_export = "kamacite_invoke",
    pointer_type = "i32",
    length_type = "i32",
    result_type = "i64",
    result_encoding = "low-u32-pointer high-u32-length; zero pointer with nonzero length is invalid",
    input_encoding = "bounded canonical packed Preserves or source-envelope bytes",
    output_encoding = "bounded canonical packed Preserves bytes",
    max_input_bytes = maximum_transport_bytes,
    max_output_bytes = maximum_transport_bytes,
    max_hostcall_bytes = maximum_hostcall_bytes,
    result_classes = ["success", "guest-error", "invalid-pointer-length", "malformed-output", "trap", "out-of-fuel", "resource-denial", "harness-failure"],
    allowed_host_functions = [],
    denied_ambient_authority = ["wasi", "filesystem", "network", "process", "environment", "clock", "random", "credential", "device", "compiler", "evaluator", "package-manager", "mutation"],
  },
  admitted_proposals = ["wasm1"],
  denied_proposals = ["bulk-memory", "component-model", "custom-page-sizes", "memory64", "multi-memory", "multi-value", "non-trapping-float-to-int", "sign-extension-ops", "threads", "wit", "wasi"],
  resources = {
    max_artifact_bytes = maximum_artifact_bytes,
    max_exports = maximum_exports,
    max_functions = maximum_functions,
    max_memories = maximum_memories,
    max_tables = maximum_tables,
    max_guest_memory_pages = maximum_guest_memory_pages,
    max_guest_table_elements = maximum_guest_table_elements,
    allow_memory_grow = false,
    allow_start = false,
    interpreter_code_pages = maximum_interpreter_code_pages,
    interpreter_control_frames = maximum_interpreter_control_frames,
    interpreter_stack_words = maximum_interpreter_stack_words,
    interpreter_default_fuel = default_interpreter_fuel,
    interpreter_stream_chunks = maximum_interpreter_stream_chunks,
  },
  observed_external_module = {
    artifact_kind = "module",
    proposals = ["wasm1"],
    imports = [],
    exports = ["module::run::Func"],
    has_memory = false,
    abi_compatible = false,
  },
  required_parent_roles = ["source-envelope", "canonical-input", "input-schema", "output-schema", "adapter-profile", "abi-profile", "module", "spacewasm-runtime", "mantle-bundle", "octet-static-report", "capability-plan", "effect-plan", "resource-plan"],
  execution_blockers = [
    "mantle-reference-module-does-not-implement-kamacite-byte-abi",
    "octet-spacewasm-mvp-profile-permits-only-hostless-run-export",
    "kamacite-guest-materialization-bundle-unavailable",
    "consumer-spacewasm-shell-unavailable",
  ],
  checked_laws = ["profile-selection-is-explicit", "component-and-wit-fallback-rejected", "exact-external-bytes-bound", "module-and-canonical-value-identities-remain-distinct", "external-static-pass-cannot-create-execution-plan", "packed-preserves-remains-authoritative"],
  non_claims = ["not WIT or Component Model compatibility", "not source-language semantic equivalence", "not SpaceWasm correctness or memory safety", "not WebAssembly conformance", "not sandbox completeness or host authority", "not portable interpreter state", "not flight qualification or certification", "not production readiness", "not release eligibility"],
} in
Profile | Contracts.Profile
