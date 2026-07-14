## Why

Kamacite owns lossless interchange boundaries across Nickel, Rust, Steel, Preserves, and adapter receipts. ast-grep can cheaply inventory syntax-level adapter boundary violations, source-envelope handling, ambient authority leaks, and migration candidates. Kamacite should use ast-grep as a bounded structural audit source while keeping Preserves/source-envelope identity and adapter receipts authoritative.

## What Changes

- Define ast-grep adapter-boundary audit profiles for Rust, Nix, YAML/JSON, and any registered custom tree-sitter language used by Kamacite.
- Record rule bundle identity, source-envelope scope, findings, candidate rewrites, and non-claims in Kamacite receipts.
- Require positive and negative fixtures before ast-grep rules can gate adapter conformance.
- Keep ast-grep syntax matches separate from losslessness, semantic equivalence, and adapter correctness claims.

## Impact

- **Surfaces**: adapter conformance, validation harness, source envelopes, Steel/Nickel adapter suites, Valence receipt export.
- **Non-claims**: ast-grep does not prove losslessness, Preserves canonical identity, Nickel/Rust/Steel semantic equivalence, or adapter soundness.
- **Validation**: ast-grep rule tests, adapter fixture scans, receipt validation, Cairn gates, and Kamacite validation harness checks.
