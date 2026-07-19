# Shared Wasm Component Boundary Feasibility

## Decision

**Classification: incubate / blocked for extraction.**

A product-neutral deterministic static boundary is technically plausible, but no independent repository should be created yet. The portfolio does not currently prove two committed direct consumers of one lossless API, independent release value, compatible licensing, or a complete cross-consumer fixture corpus. Executable materialization, runtime authority, interchange semantics, checked-source policy, and adversarial evidence must remain with their current owners.

## Smallest plausible boundary

A future `no_std` plus `alloc` core may own only:

- artifact kind (`core-module` or `component`) and opaque BLAKE3 artifact identity;
- WIT package/world/source identities as opaque strings and digests;
- product-neutral static artifact facts: proposals, imports, exports, worlds, memory/table/function counts and declarations, growth/start/custom-section facts;
- deterministic feature, import/export, structural, and resource-profile DTOs;
- pure admission over supplied facts with ordered diagnostics;
- deterministic profile identity over one documented canonical encoding;
- normalized static and execution-outcome vocabulary that does not interpret product receipts;
- optional `wasmparser` inspection behind a separate feature, returning the same static facts;
- positive and negative conformance fixtures shared by every adopter.

The core must not depend on Mantle, Kamacite, Octet, Aspen/Molten, ChaosControl, Valence, Cairn, UCAN, Basalt, Preserves, Wasmtime, WAC, Wizer, or a package registry.

## Ownership matrix

| Existing owner | Reusable facts observed | Semantics that must remain with the owner |
|---|---|---|
| Kamacite | WIT/component/profile DTO shape, parent-role distinctions, deterministic static denial | Preserves canonical values, source envelopes, adapter descriptors, capability/effect admission, receipt construction, Valence projection, and losslessness claims |
| Octet | wasmparser inspection, artifact facts, profile checks, finding classes | checked Rust policy, source/provenance roles, SARIF, cohort and handoff evidence, standards, and verification claims |
| Mantle | `no_std` component model DTOs, BLAKE3 values, deterministic admission helpers | package resolution, WIT binding generation, composition, virtualization, store objects, tool invocation, sandbox/build evidence, materialization bundles, cache and release claims |
| Aspen/Molten | artifact kind, feature/resource profile, static resource inspection, denial vocabulary | runtime-profile selection, Wasmtime execution, host functions, Basalt/UCAN authority, Preserves actor ABI, resource enforcement, runtime receipts, migration, and product support |
| ChaosControl | normalized parser/profile/compile/link/trap/result classes and bounded case metadata | generation, mutation, differential execution, replay, shrinking, retention, exploration evidence, and VM/snapshot claim separation |

Passing a shared static check would prove only that supplied bytes/facts satisfy the exact admitted static profile. It would not prove build provenance, materialization trust, runtime correctness, sandbox completeness, authority, semantic equivalence, deterministic execution, evidence validity, or release readiness.

## Consumer sufficiency

The portfolio has multiple implementations but not two committed adopters of one API:

- Kamacite is committed to static planning and is blocked on a verified Mantle materialization handoff.
- Mantle owns an implemented component materialization core, not a commitment to consume an external static vocabulary.
- Aspen/Molten owns a runtime profile and execution path, not a commitment to replace its product model with an external crate.
- Octet owns a checked-path analyzer whose current internal types include Octet-specific findings, handoffs, SARIF, and evidence.
- ChaosControl's adversarial rail consumes cohort evidence and normalized observations, but it is not an adopter of a proposed shared crate.

Extraction requires two accepted consumer changes naming the same external package, exact revision, mapping, rollback path, and positive/negative fixtures. Similar local types or a cross-repo proposal that exchanges receipts do not satisfy this gate.

## Cycle analysis

A dependency-acyclic shape is possible only if the shared crate is below every stack owner and has no stack dependencies:

```text
static-wasm-component-core
  -> Kamacite adapter mapping
  -> Octet analyzer mapping
  -> Mantle materialization mapping
  -> Aspen/Molten runtime mapping
  -> ChaosControl exploration mapping
```

The candidate must not import Valence identity/evidence types, Kamacite Preserves types, Mantle bundle types, Aspen runtime profiles, Octet findings, or ChaosControl evidence. Doing so would either create release-order pressure and likely cycles or transfer owner-specific claims into a generic package.

## Losslessness gates

Before extraction, cross-consumer fixtures must prove:

1. exact mapping of artifact kind, WIT package/world, features, imports/exports, resource declarations, and profile identity;
2. deterministic rejection of malformed bytes, unsupported proposals, undeclared imports/exports, ambiguous worlds, unbounded resources, growth/start policy violations, and unknown fields;
3. explicit preservation or rejection of every owner-specific field—never silent dropping;
4. exact distinction between static rejection, profile rejection, compile rejection, link rejection, deterministic trap, normalized success/error, bound skip, divergence, and harness error;
5. no interpretation of Mantle materialization references, Kamacite canonical values, Octet evidence roles, Aspen authority/runtime receipts, or ChaosControl replay evidence;
6. no fallback between core modules and components or between runtime profiles.

## Licensing and release gate

The candidate material spans differently licensed owners: `kamacite-core` is `MIT OR Apache-2.0`, while the reviewed Octet, Mantle, and Aspen component implementations are `AGPL-3.0-or-later`. No new repository may copy or relicense implementation code across this boundary without explicit copyright authorization and a documented provenance map. A clean-room DTO specification still requires review to ensure it does not copy protected implementation expression.

Independent release value is not yet proven. A release must remain useful as a static facts/admission/conformance package without any stack repository, and upgrades must be independently adoptable by at least two consumers. Until those gates pass, shared fixtures and receipt exchange are safer than a new repository.

## Promotion checklist

Extraction may be reconsidered only when all are true:

- [ ] two accepted consumer changes pin the proposed package and map every field losslessly;
- [ ] the package has no stack dependencies and cycle analysis is recorded for every consumer;
- [ ] one product-neutral canonical profile encoding and BLAKE3 identity algorithm are accepted;
- [ ] positive and negative cross-consumer fixtures pass from the same corpus;
- [ ] optional `wasmparser` versions and feature drift have an explicit compatibility policy;
- [ ] owner-specific authority, evidence, materialization, runtime, and interchange semantics remain outside;
- [ ] copyright holders authorize the selected license and provenance is documented;
- [ ] the package demonstrates an independent release cadence and rollback path.

Until then, the correct result is **incubate**, with an exact blocker rather than a repository extraction claim.
