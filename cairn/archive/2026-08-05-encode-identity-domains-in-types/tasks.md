# Tasks: Encode identity domains in Rust types

## Phase 1: Baseline and identity foundation

- [x] [serial] V1 Capture current canonical Preserves bytes, JSON projections, identity refs, manifests, and golden receipts for every maintained identity family. r[kamacite.lossless_ir.nominal_identity.compatibility]
- [x] [serial] I1 Add private owned and borrowed generic identity types with marker domains, checked construction, explicit accessors, and typed diagnostics. r[kamacite.lossless_ir.nominal_identity]
- [x] [parallel] V2 Add positive and negative parser tests for supported BLAKE3 input, malformed text, unsupported algorithms, and unknown domains. r[kamacite.lossless_ir.nominal_identity.validation]
- [x] [serial] I2 Add the versioned wire identity adapter and closed `AnyIdentity` enum. r[kamacite.lossless_ir.nominal_identity_wire]

## Phase 2: Staged core migration

- [x] [serial] I3 Migrate source-envelope and canonical-value APIs to exact identity aliases. r[kamacite.lossless_ir.nominal_identity_migration]
- [x] [serial] I4 Migrate projection, adapter, evaluator, compiler, and target-artifact APIs. r[kamacite.lossless_ir.nominal_identity_migration]
- [x] [serial] I5 Migrate receipt, effect-family, effect-operation, handler, documentation, and compatibility identity APIs. r[kamacite.lossless_ir.nominal_identity_migration]
- [x] [serial] I6 Update Valence-friendly projections and stack adapters without role promotion or direct live dependencies. r[kamacite.lossless_ir.nominal_identity_stack]

## Phase 3: Compiler and compatibility evidence

- [x] [parallel] V3 Add compile-pass fixtures for same-domain flows and compile-fail fixtures for cross-domain substitutions. r[kamacite.lossless_ir.nominal_identity_compile_time]
- [x] [parallel] V4 Add `AnyIdentity` positive and negative fixtures for unsupported domains, conflicting metadata, stale versions, and wrong artifact roles. r[kamacite.lossless_ir.nominal_identity_wire.validation]
- [x] [parallel] V5 Prove canonical Preserves bytes, JSON projections, manifest identities, and existing golden receipts remain stable. r[kamacite.lossless_ir.nominal_identity.compatibility]
- [x] [parallel] V6 Run source, Nickel, Steel, Wasm, effect, FSM, documentation, Valence export, and migration fixture suites. r[kamacite.lossless_ir.nominal_identity_migration]

## Phase 4: Closeout

- [x] [serial] I7 Document domain aliases, wire admission, semantic-operation ownership, consumer migration, and non-claims. r[kamacite.lossless_ir.nominal_identity_docs]
- [x] [serial] V7 Run workspace tests, Clippy, manifest freshness, canonical migration checks, lifecycle validation, Cairn gates, and relevant Nix checks. r[kamacite.lossless_ir.nominal_identity_final_checks]

## Verification coverage

- `Scenario: Same-domain identity is accepted` -> I1, V2, V3
- `Scenario: Cross-domain identity does not compile` -> I1, V3
- `Scenario: Wire identity admits a known domain` -> I2, V4
- `Scenario: Wire identity has conflicting domain data` -> I2, V4
- `Scenario: Source and canonical identities remain distinct` -> I3, V3
- `Scenario: Adapter and target identities remain distinct` -> I4, V3
- `Scenario: Effect family and operation identities remain distinct` -> I5, V3
- `Scenario: Valence projection preserves roles` -> I6, V6
- `Scenario: Canonical Preserves identity remains stable` -> V1, V5
- `Scenario: Existing artifact families retain behavior` -> I3, I4, I5, V6
- `Scenario: Nominal identity claims remain bounded` -> I7, V7
