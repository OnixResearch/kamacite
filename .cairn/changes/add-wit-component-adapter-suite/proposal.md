## Why

Kamacite already defines WebAssembly as an optional adapter/validator target and explicitly keeps Preserves authoritative for source envelopes, canonical values, and receipts. It has documentation-runner Wasm facts but no executable WIT/Component Model adapter suite that proves one bounded Kamacite input/output profile can be built, inspected, admitted, executed, and wrapped for Valence.

A dedicated suite is needed before Nickel, Rust, or Steel adapters independently choose manual Wasm ABIs or imply that WIT/component compatibility replaces Kamacite's canonical schemas and non-claims.

## What Changes

- Define a versioned WIT component adapter ABI whose outer interface carries bounded canonical Kamacite Preserves/source-envelope bytes and explicit control fields.
- Preserve WIT source, component bytes, adapter descriptor, runtime profile, input/output, target artifact, and receipt as separate BLAKE3 identities.
- Add adapter descriptors for world, imports/exports, capability/effect requirements, resource limits, toolchain cohort, runtime profile, and non-claims.
- Require executable adapter components, generated bindings, composition, and virtualization outputs to arrive through exact Mantle materialization bundles; Kamacite may validate incomplete descriptors statically but does not build production components.
- Validate components and plans in pure core, then execute only in a thin Wasmtime component shell using the admitted Molten-compatible deterministic profile.
- Require declared capabilities/effects and deny ambient WASI, filesystem, network, process, clock, random, environment, credential, compiler, evaluator, mutation, or device authority.
- Check canonical Preserves input/output and emit deterministic adapter receipts plus bounded Valence exports.
- Add positive and negative fixtures for valid round trips, malformed/stale WIT, wrong worlds, undeclared imports/effects, resource failures, tampered payloads, and overclaims.

## Impact

- **Surfaces**: adapter descriptors, source envelopes, target artifacts, WIT fixtures, component runner shell, validation harness, Preserves schemas/receipts, Valence export, and documentation.
- **Cross-stack boundary**: Mantle materializes executable artifacts, the suite targets the shared Molten component profile, and Kamacite retains adapter/source/value semantics without transferring Molten runtime, Basalt authority, Mantle build, Octet check, or Valence semantics into its core.
- **Claims**: a passing adapter receipt proves only the declared byte-level/profile round trip and execution record. It does not prove arbitrary Nickel/Rust/Steel semantic equivalence, compiler/runtime correctness, sandbox completeness, or authority.
