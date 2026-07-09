## Why

Kamacite has a stable core fixture harness and the reviewed Nickel source manifest now owns the full runtime projection. The remaining Nickel coverage is still DTO-only: current adapter fixtures classify Nickel export behavior, but they do not execute the Nickel reference toolchain, bind an evaluator identity, or prove the source/export/projection receipt path end to end.

Nickel is the first source-language adapter Kamacite should make executable because Nickel-authored contracts and configuration are central to Kamacite's interchange boundary. Without a dedicated suite, later Steel, WASM, Valence, and Octet work may build on synthetic adapter receipts instead of a reviewed source-language adapter pattern.

## What Changes

- Define the Nickel adapter execution boundary: Nickel execution stays in the CLI/adapter shell, while `kamacite-core` receives only normalized DTOs, bytes, identities, diagnostics, and receipt fields.
- Add a fixture suite for Nickel source envelopes, Nickel export projections, evaluator identity, projection identity, checked laws, and explicit non-claims.
- Add negative fixtures for missing source envelopes, stale export projection identity, treating JSON export as canonical IR, missing or unsupported evaluator identity, malformed export output, and undeclared host effects.
- Add deterministic golden receipts and explicit update evidence for Nickel adapter fixtures.
- Document the Nickel-capable validation command, required capabilities, and non-claims.

## Impact

- **Files**: `cairn/changes/add-nickel-adapter-suite/**`, `fixtures/manifest.ncl`, `fixtures/manifest.json`, `fixtures/adapters/nickel/**` or equivalent fixture paths, `fixtures/golden/**`, `crates/kamacite-core/**`, `crates/kamacite-cli/**`, and validation/deferred-suite docs.
- **Testing**: Baseline `cargo test --workspace` and current harness validation before implementation; final Rust tests, Nickel adapter positive/negative fixtures, golden receipt validation, projection freshness check, Cairn validation, and proposal/design/tasks gates.
