let DocsConfig = import "docs-config-contract.ncl" in
{
  schema_version = 1,
  source_roots = [
    { path = "crates/kamacite-core/src", language = "rust", collector_profile = "rust-bounded-docs-v1" },
    { path = "schemas/preserves", language = "nickel", collector_profile = "schema-docs-v1" },
    { path = "fixtures", language = "steel", collector_profile = "fixture-docs-v1" },
  ],
  output_formats = [
    { format = "markdown", path = "target/generated-docs" },
  ],
  page_grouping = [],
  link_policy = "identity-only",
  run_profiles = ["check-only"],
  wasm_runner_profiles = [
    {
      profile = "docs-wasm-check-only",
      engine_identity_policy = "declared-identity-required",
      imports = ["wasi:cli/stdout"],
      capabilities = ["stdout"],
      fuel_limit = 100000,
      memory_limit_bytes = 65536,
      output_limit_bytes = 1048576,
    },
  ],
  update_policy = "explicit-docs-update-only",
} | DocsConfig
