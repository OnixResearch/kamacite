# Validation evidence: identity-domain migration

Task V7 closeout evidence for `encode-identity-domains-in-types`.

## Rail results on the drain branch

All commands ran in the drain worktree on top of `origin/main` (`d76fe4a`):

- `cargo test --workspace`: 11 suites pass, including 143 `kamacite-core`
  unit tests and doc tests with compile-pass and compile-fail fixtures.
- `cargo clippy --workspace --all-targets` with the repository restriction
  profile (`-D warnings` plus the Octet canonical-determinism lints):
  clean.
- `cargo fmt`: applied; tree is formatted.
- `kamacite validate --root . --manifest fixtures/manifest.json`: exit 0.
- `kamacite migrate-preserves` (dry-run): exit 0; no convergence needed.
- `kamacite export-valence-receipt` with the golden conformance cohort:
  exit 0.
- `kamacite docs check --root .`: exit 0.
- `ast-grep test --config audits/ast-grep/sgconfig.yml`: pass.
- Cairn lifecycle `validate` plus proposal, design, and tasks gates:
  pass with the pinned policy.
- `nix flake check -L`: exit 0 ("all checks passed").

## Canonical stability (V5)

The function-address migration-evidence test compares the computed receipt
identity against the recorded fixture identity and passes. The Valence
identity-domain-substitution negative fixture keeps its golden diagnostic
through the wire-envelope admission flow. Fixture validation, golden
receipts, and the Preserves migration dry-run all pass without updates,
which proves canonical Preserves bytes, JSON projections, and manifest
identities did not change.

## Claim boundary

These results prove the bounded checks named by each receipt and gate.
They do not prove compiler correctness, runtime behavior, semantic
equivalence, or release eligibility.
