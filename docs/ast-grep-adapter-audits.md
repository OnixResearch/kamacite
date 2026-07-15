# ast-grep adapter-boundary audits

Kamacite uses ast-grep only as syntax-level adapter-boundary evidence. The typed source of truth is `audits/ast-grep/profile.ncl`; `audits/ast-grep/generated/profile.json` is its freshness-checked runtime projection. The profile records built-in Rust, Nix, YAML, and JSON parser identities, scan scopes, rule coverage, promotion posture, and non-claims. A custom tree-sitter language must bind its parser identity and extension scope before findings are admitted.

The initial gate-eligible rule rejects direct `std::process::Command::new(...)` construction in `kamacite-core`. It has matching and non-matching fixtures plus ast-grep rule-test snapshots. Rules without both fixture classes remain advisory.

The ordered rule bundle, exact source-envelope scope, normalized finding set, profile, codemod recipe, and Valence-friendly projection have distinct BLAKE3 identities. The projection uses only the `boundary` role and retains exact source spans and syntax-only non-claims.

Codemods are dry-run migration artifacts. `audits/ast-grep/codemods/core-ambient-process.json` binds before/after source identities and required post-check commands; the audit rail does not apply the rewrite or claim semantic equivalence.

Validate the rail with:

```console
ast-grep test --config audits/ast-grep/sgconfig.yml
ast-grep scan --warning --rule audits/ast-grep/rules/core-ambient-process.yml --json=compact audits/ast-grep/fixtures/positive
ast-grep scan --warning --rule audits/ast-grep/rules/core-ambient-process.yml --json=compact audits/ast-grep/fixtures/negative
ast-grep scan --rule audits/ast-grep/rules/core-ambient-process.yml --json=compact crates/kamacite-core/src
cargo run -q -p kamacite-cli -- validate --root . --manifest fixtures/manifest.json
```

A clean scan does not prove losslessness, type correctness, adapter correctness, semantic equivalence, runtime behavior, or release eligibility.
