## Design

1. Add ast-grep audit profiles for Kamacite adapter boundaries: source-envelope preservation, ambient authority denial, receipt construction, and target-artifact non-claim hygiene.
2. Keep each profile tied to a declared language and scan scope. Built-in ast-grep languages cover Rust, Nix, YAML, and JSON; custom languages require explicit tree-sitter dynamic-language registration and recorded parser identity.
3. Bind rule bundles and scan receipts with BLAKE3 identities and include the source-envelope or fixture scope being audited.
4. Require positive and negative fixtures before a rule can participate in adapter-conformance gates.
5. Treat findings as candidate boundary evidence only. A clean scan cannot prove losslessness or semantic equivalence.
6. Preserve candidate rewrite recipes as dry-run migration artifacts with explicit before/after source-envelope identity and post-check commands.
7. Export findings to Valence only through recorded-only or boundary roles.

## Non-Goals

- No ast-grep evaluation of Nickel or Steel semantics.
- No replacement for Kamacite Preserves canonicalization, adapter DTO validation, or receipt checks.
- No automatic source rewrites across adapter fixtures without a scoped migration package.
