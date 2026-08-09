let Upstream = import "upstream-default.ncl" in
let CairnTraceabilityProfile = {
  id = "cairn-default",
  requirement_sources = [
    { root = ".cairn/specs", source_kind = "accepted_specs", extensions = ["md"] },
  ],
  evidence_sources = [
    { root = "crates", source_kind = "rust_source", extensions = ["rs"] },
    { root = "crates", source_kind = "nickel_source", extensions = ["ncl"] },
    { root = "tools", source_kind = "tool_source", extensions = ["rs", "ncl"] },
  ],
  marker_verbs = ["impl", "verify", "depends", "related"],
} in
let SemanticOperationTraceabilityProfile = {
  id = "bind-effect-operation-semantic-identity",
  requirement_sources = [
    { root = ".cairn/specs/semantic-effect-operation-identity", source_kind = "accepted_specs", extensions = ["md"] },
    { root = ".cairn/specs/semantic-effect-operation-stack", source_kind = "accepted_specs", extensions = ["md"] },
  ],
  evidence_sources = [
    { root = "crates/kamacite-core/src/semantic_operation.rs", source_kind = "rust_source", extensions = ["rs"] },
    { root = "crates/kamacite-core/src/semantic_operation_valence.rs", source_kind = "rust_source", extensions = ["rs"] },
    { root = "crates/kamacite-cli/src/semantic_operation_cli.rs", source_kind = "rust_source", extensions = ["rs"] },
    { root = "crates/kamacite-cli/tests/semantic_operation_cli.rs", source_kind = "rust_source", extensions = ["rs"] },
    { root = "schemas/preserves", source_kind = "tool_source", extensions = ["preserves"] },
    { root = "docs/semantic-effect-operation-identity.md", source_kind = "tool_source", extensions = ["md"] },
    { root = "docs/algebraic-effects-stack-roles.md", source_kind = "tool_source", extensions = ["md"] },
    { root = "docs/wasm-component-adapter.md", source_kind = "tool_source", extensions = ["md"] },
    { root = "docs/valence-receipt-export.md", source_kind = "tool_source", extensions = ["md"] },
  ],
  marker_verbs = ["impl", "verify", "depends", "related"],
} in
Upstream & {
  name | force = "kamacite-default",
  traceability_policy | force = {
    default_profile = "cairn-default",
    profiles = [
      CairnTraceabilityProfile,
      SemanticOperationTraceabilityProfile,
    ],
  },
}
