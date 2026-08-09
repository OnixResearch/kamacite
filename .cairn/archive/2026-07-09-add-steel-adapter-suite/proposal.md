## Why

Kamacite now has a stable fixture manifest, pure Rust validation core, executable Nickel adapter suite, and artifacts-first Valence receipt export profile. Steel remains the largest deferred surface in Kamacite's original Nickel/Rust/Steel boundary: current Steel coverage is only synthetic adapter-receipt evidence, so it does not execute a bounded Steel evaluator, bind evaluator identity from an execution path, prove typed host-function admission, or reject ambient Steel authority with adapter-specific receipts.

A bounded Steel adapter suite is the next useful integration slice because it exercises the adapter/probe side of Kamacite without expanding the canonical IR. The suite should prove a narrow DTO-only Steel probe path first: declared input/output schemas, declared host functions, declared imports/modules, evaluator identity, output projection identity, checked laws, and non-claims. It must keep Steel bytecode, opaque closures, ambient libraries, and arbitrary scripting authority outside trusted Kamacite values.

## What Changes

- Define the Steel adapter execution boundary: Steel execution stays in an explicit CLI/adapter-runner shell, while `kamacite-core` receives only normalized DTOs, source bytes, evaluator identity, module/import facts, host-function declarations, capabilities, output bytes, diagnostics, and receipt fields.
- Add a Steel adapter receipt/profile DTO for source identity, evaluator identity, module identity, input/output DTO schemas, host-function contracts, declared built-in modules/imports, callable descriptors, capabilities, effect class, output projection identity, checked laws, and non-claims.
- Add positive fixtures for DTO-only Steel evaluation with declared host functions and no ambient authority.
- Add negative fixtures for missing or unsupported evaluator identity, undeclared host functions, undeclared ambient authority, stale output projection identity, malformed output, opaque closures, and Steel bytecode or JSON being claimed as canonical Kamacite IR.
- Add deterministic golden receipts and explicit update evidence for Steel adapter fixtures.
- Document the Steel-capable validation command, declared capabilities, non-claims, deferred-suite relationship, and boundaries around Steel modules, contracts, and callables.

## Impact

- **Files**: `cairn/changes/add-steel-adapter-suite/**`, `cairn/specs/steel-adapter-suite/spec.md`, `cairn/specs/validation-command/spec.md`, `fixtures/manifest.ncl`, `fixtures/manifest.json`, `fixtures/adapters/steel/**` or equivalent fixture paths, `fixtures/golden/**`, `crates/kamacite-core/**`, `crates/kamacite-cli/**`, `schemas/preserves/**` if a profile schema artifact is added, and validation/deferred-suite docs.
- **Testing**: Baseline Rust tests, default harness validation, Nickel adapter validation, Valence export validation, manifest projection freshness check, and Cairn validation before implementation; final Rust tests, Steel adapter positive/negative fixtures, golden receipt validation, explicit Steel command checks, projection freshness check, docs checks, Cairn validation, and proposal/design/tasks gates.
