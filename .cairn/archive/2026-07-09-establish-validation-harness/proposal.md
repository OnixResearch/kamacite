## Why

Kamacite should establish its validation harness before the first trusted Rust implementation slice lands. The active lossless-IR and Rust-determinism work already requires positive fixtures, negative fixtures, golden receipts, deterministic diagnostics, and a pure core/effectful shell split, but the repository still needs a concrete walking skeleton that turns those requirements into an executable evidence contract.

Doing the harness first prevents early implementation code from defining the acceptance boundary by accident. A small red-green fixture runner can prove that source-envelope identity, wrong-domain rejection, stale-hash rejection, and deterministic receipt emission are observable before broader Nickel, Steel, WASM, LLVM, Valence, Octet, or algebraic-effect integrations are attempted.

## What Changes

- Define a walking-skeleton validation harness as the first implementation gate for Kamacite claim admission.
- Require a deterministic fixture layout that separates pure core fixtures from adapter/probe fixtures and classifies fixtures as positive, negative, property-style, or golden-receipt evidence.
- Require a human-authored Nickel fixture manifest with typed fixture metadata, expected verdicts, diagnostics, non-claims, dependencies, and requirement coverage.
- Require a minimal Rust workspace boundary with a pure core crate/module and a thin runner shell that performs discovery and IO before passing normalized DTOs to the core.
- Seed the harness with only the minimum red-green cases: valid source envelope, stale hash rejection, wrong identity-domain rejection, and deterministic golden receipt comparison.
- Defer full adapter execution suites until the walking skeleton is passing and each adapter adds its own positive and negative fixtures.

## Impact

- **Files**: `cairn/changes/establish-validation-harness/**`, README active-change guidance.
- **Testing**: Cairn validation plus proposal/design/tasks gates now; later implementation must add the minimal fixture tree, Nickel manifest, Rust workspace/test runner, seed fixtures, and deterministic receipt checks before implementation behavior is treated as trusted.
