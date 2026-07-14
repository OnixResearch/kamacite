let DocsConfig = import "docs-config-contract.ncl" in
{
  schema_version = 1,
  source_roots = [
    { path = "crates/kamacite-core/src", language = "rust", collector_profile = "rust-bounded-docs-v1" },
  ],
  output_formats = [
    { format = "markdown", path = "target/generated-docs" },
  ],
  page_grouping = [],
  link_policy = "identity-only",
  run_profiles = ["check-only"],
  wasm_runner_profiles = [],
  update_policy = "explicit-docs-update-only",
  unknown_field = true,
} | DocsConfig
