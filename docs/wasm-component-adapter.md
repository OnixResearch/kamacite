# WIT component adapter boundary

<!-- r[impl kamacite.algebraic_effects.semantic_handler_matching] -->

Version 2 imports use exact Kamacite semantic operation identities. A local WIT name cannot replace the typed `effect-operation` identity.

Kamacite's Component Model adapter is a layered, fail-closed target boundary. WIT and WebAssembly remain target artifacts; canonical packed Preserves bytes remain authoritative for values and receipts.

## Reviewed static cohort

The static profile is authored at `adapters/wasm-component/profile.ncl` and checked into `adapters/wasm-component/generated/profile.json`. It binds exact copies of the shared Molten cohort from `OnixResearch/aspen` revision `62b02a3435a7000a85bbb6988a88f8afe5effbbb`:

- `adapters/wasm-component/upstream/runtime.wit` — package `molten:component-runtime@1.0.0`, world `actor`, BLAKE3 `83be349bb27975ada30dbe60817c5404df7862babdf02ae229b399886e76d5e8`;
- `adapters/wasm-component/upstream/molten-profile.json` — exact production runtime profile, BLAKE3 `c2ef4db1ab1010512b0d7b023e6a64f80c00c11850e041fa5347d91028d8d96f`.

The WIT world carries canonical Preserves bytes through `list<u8>` and returns an explicit `result<list<u8>, string>`. The carrier does not establish canonicality by itself. Kamacite validates packed Preserves input before planning and must validate returned bytes after execution before issuing a passing receipt.

## Static planning

`kamacite_core::validate_wasm_component_static` is pure deterministic logic over in-memory bytes and DTOs. It binds and validates:

- exact profile and WIT source bytes;
- descriptor, WIT, component, runtime, toolchain, resource, schema, and optional materialization identities;
- complete typed parent edges;
- sorted imports/exports and capability/effect/authority/resource bindings;
- fixed resource declarations and the reviewed Wasm feature cohort;
- canonical packed Preserves input under the descriptor's input schema; and
- the canonical Mantle `mantle-wasm-component-materialization-bundle-v1` schema, bundle identity, required stage-receipt links, WIT inputs, runtime-profile expectation, final portable component, object sizes, and required non-claims when supplied.

`kamacite_core::validate_wasm_component_mantle_handoff` is the smaller product-boundary core. It validates Mantle's canonical bundle serialization, remeasures one selected WIT input and the final portable component, and returns typed Kamacite identities without interpreting Mantle build correctness. Missing materialization returns a successful `static-only` decision with an explicit blocker, never an executable plan. Until an independent Mantle file/verifier API and a matching Molten adapter bundle are integrated, even a structurally valid, remeasured bundle carries `external-materialization-verifier-unavailable` and cannot yield a plan. Ambient WASI and all imports are denied by the initial shared profile.

## Current external blocker

Live execution is intentionally unavailable. Mantle now emits real WIT binding, guest compilation, composition, virtualization, validation, and Octet evidence through `mantle-wasm-component-materialization-bundle-v1`, and Kamacite can consume that exact schema and remeasure its selected WIT and final portable bytes. The checked Mantle hello fixture proves only this producer/consumer schema-and-byte handoff: its `demo:app/app` world and runtime-profile identity do not match Kamacite's pinned `molten:component-runtime@1.0.0/actor` cohort, so it is not executable adapter evidence. The initial Molten profile also admits no host imports, so a positive hostcall fixture requires a later reviewed profile cohort.

A static-only decision can be encoded as `kamacite.wasm-component-static-receipt.v1`. Receipt construction reruns validation over the exact request rather than trusting a caller-assembled decision, binds optional component and inspection identities when materialization bytes exist, requires nonempty execution blockers, and cannot contain an execution plan or passing output claim. Component or inspection bytes without a materialization manifest fail as orphan artifacts.

The pure core can validate candidate output bytes against the exact static request's output schema and byte bound, but that check does not imply execution and cannot remove a blocker. The deterministic Valence-friendly projection assigns only `recorded-only` or `boundary` verification classes, preserves typed descriptor parent edges and non-claims, records exact byte spans for byte-owning artifacts, and uses explicit absent-span reasons for identity-only layers. Projection linkage does not transfer Kamacite, Mantle, Aspen/Molten, Wasmtime, or Valence trust.

Until a Mantle bundle for the exact pinned Molten WIT/runtime cohort, independently verified stage/object files, and the reviewed runtime shell exist, Kamacite must not add a production component build fallback, relabel loose test bytes as executable evidence, instantiate Wasmtime, or issue a passing execution receipt.

## Non-claims

Static validation does not prove component execution, behavioral correctness, Wasmtime correctness, sandbox completeness, build trust, host authority, arbitrary Nickel/Rust/Steel/Wasm semantic equivalence, release eligibility, or production readiness.
