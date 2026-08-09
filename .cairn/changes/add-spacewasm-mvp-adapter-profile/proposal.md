## Why

Kamacite's active WIT component adapter suite targets a shared Wasmtime Component Model profile. SpaceWasm cannot execute that profile, but it may be useful for a narrower class of embedded validators and adapters expressed as WebAssembly 1.0 core modules with explicit host functions, bounded memory, and canonical Preserves byte input/output.

Using SpaceWasm as an implicit fallback from a WIT component request would erase ABI and evidence distinctions. A separate MVP adapter profile is needed to preserve Kamacite source/value authority, require exact materialization and static evidence, and keep runtime observations bounded while upstream remains unreleased and incomplete.

## What Changes

- Add a distinct `kamacite-spacewasm-mvp` adapter profile for core modules; never select it for a WIT/component request or as fallback from the component suite.
- Define a Kamacite-owned pointer/length/result byte-carrier ABI for canonical Preserves/source-envelope bytes plus explicit host-function descriptors.
- Preserve source envelope, canonical input/output, adapter descriptor, core module, ABI profile, SpaceWasm cohort/configuration, static report, runtime observation, and receipt as separate BLAKE3 identities.
- Require exact module/runtime artifacts through Mantle bundles and matching Octet `spacewasm-mvp` artifact evidence before execution.
- Validate descriptors, identity edges, capabilities/effects, resource bounds, ABI facts, and receipt plans in pure core.
- Execute only through a thin SpaceWasm shell with declared host functions, no WASI, and no ambient filesystem/network/process/clock/random/environment/credential/device authority.
- Emit deterministic adapter execution/denial receipts and bounded Valence exports.
- Add positive and negative fixtures for round trips, profile separation, evidence drift, malformed bytes/ABI, undeclared effects/imports, resource failure, runtime divergence evidence, and semantic overclaims.

## Impact

- **Surfaces**: adapter descriptors, core-module ABI, target artifacts, Mantle/Octet handoffs, SpaceWasm shell, capability/effect/resource admission, Preserves receipts, Valence export, fixtures, and docs.
- **Relationship to active work**: complements `add-wit-component-adapter-suite` with a separately named core-MVP profile; it does not weaken or replace the component profile.
- **Cross-stack ownership**: Mantle materializes, Octet checks static facts, ChaosControl may supply diagnostic differential evidence, and Kamacite owns adapter/source/value/receipt semantics.
- **Claims**: passing proves only the exact byte-level adapter observation under one profile. It does not prove source-language semantic equivalence, SpaceWasm correctness, sandbox completeness, certification, or production readiness.
