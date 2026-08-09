## Context

Kamacite treats WebAssembly as an optional adapter/validator target, never canonical IR. The Component Model suite will use WIT as an outer executable ABI while retaining Preserves authority. SpaceWasm currently supports only core WebAssembly 1.0 modules, so any use requires a manual core-module ABI and a distinct runtime/profile identity.

The useful scope is a bounded embedded validator/adapter experiment: exact canonical bytes enter a declared module function, optional declared host functions exchange bounded canonical bytes, and exact canonical bytes return. The shell cannot decide canonical identity, grant authority, or infer source-language equivalence.

## Decisions

### 1. Core-MVP and WIT component profiles are disjoint

Adapter descriptors use a closed runtime/artifact kind. `kamacite-spacewasm-mvp` accepts only core modules and its own ABI profile. Component/WIT descriptors are rejected rather than translated or silently routed to SpaceWasm.

### 2. Kamacite owns a minimal byte-carrier ABI

The profile will define explicit exported memory/alloc/dealloc/entry functions, pointer/length/result encoding, error classes, maximum byte fields, and host-function signatures. A compatibility mapping to Molten's core ABI may be recorded separately, but matching shapes do not merge schema or authority ownership.

### 3. Preserves remains canonical authority

The shell transports bytes only. The pure core validates canonical Preserves/source-envelope input and output, computes typed BLAKE3 identities, and rejects malformed/non-canonical/tampered values. Module bytes and runtime IR never become Kamacite canonical IR.

### 4. Exact external artifacts precede execution

A production-like fixture requires a complete remeasured Mantle SpaceWasm/module bundle, matching Octet artifact/profile/cohort report, and optionally matching ChaosControl diagnostic evidence. Local loose bytes may be admitted only in an explicitly test-only class and cannot produce promoted receipts.

### 5. Descriptors and execution plans are pure

The core will validate adapter kind/profile, source/module/runtime/report identities, imports/exports, ABI, capabilities/effects, resource bounds, canonical inputs, parent edges, and non-claims and will return a closed execution plan or ordered denials.

### 6. The SpaceWasm shell has no ambient authority

The shell links only declared host functions from an admitted plan and denies WASI and ambient filesystem, network, process, environment, clock, random, credential, compiler, evaluator, mutation, device, or package authority. A capability descriptor is a claim input; existing Kamacite/consumer authority gates still decide whether effects are allowed.

### 7. Runtime and guest memory bounds are separate

Profiles separately identify interpreter allocator/code/stack limits and guest linear-memory/table/growth/instruction/hostcall limits. SpaceWasm's allocation model does not prove the module's linear memory is bounded.

### 8. Receipts preserve every layer and non-claim

Execution/denial receipts bind source envelope, canonical input/output, adapter descriptor, module, ABI, SpaceWasm runtime/profile/configuration, Mantle/Octet/optional Chaos evidence, capabilities/effects/resources, hostcall transcript, outcome, and non-claims. Valence projections preserve those roles without importing trust.

### 9. Profile promotion requires separate evidence

The first profile is experimental. Passing fixtures cannot make SpaceWasm the default docs runner, component shell, or general Kamacite adapter runtime. Promotion requires a later accepted change and reviewed runtime maturity/security evidence.

## Functional core / imperative shell split

- **Pure core**: descriptor/profile/materialization/static-evidence/ABI/capability/effect/resource/canonical-input/output/receipt validation, identity material, execution planning, outcome classification, and deterministic diagnostics.
- **Imperative shell**: bundle/artifact/report reads, SpaceWasm store/module construction, memory transfer, host-function linkage, runtime execution, observation capture, and receipt persistence/rendering.

## Risks / Trade-offs

- A manual core ABI duplicates some shape represented more richly by WIT. Keeping profiles disjoint prevents accidental equivalence but adds maintenance.
- Rust-produced modules may use unsupported proposals. The producer must emit the conservative profile or fail; opaque lowering is excluded.
- Unsafe upstream containers remain part of the runtime trust boundary.
- Runtime success cannot prove canonical source transformation or arbitrary adapter semantics.
- Optional cross-stack evidence may be unavailable initially; missing evidence must remain a typed blocker, not a fabricated pass.

## Current implementation boundary

Mantle commit `a141fcbaafe41f9a413a81275a33fe915bfca370` now provides a complete, independently verifiable SpaceWasm reference bundle, and Octet commit `4367300e10740ecc99ba4b2171ace561b4787327` now provides a matching execution-free `spacewasm-mvp` report over the exact Mantle fixture module. Those external prerequisites are real but narrower than Kamacite's ABI: the module exports only hostless `run`, has no memory, and Octet's profile permits no imports or Kamacite allocation/invocation exports.

Kamacite therefore admits the exact evidence only as static denial evidence. The pure core validates profile, descriptor, source, canonical input/output, typed parent identities, external bytes, canonical denial receipts, and bounded Valence projections, but it constructs no execution plan. The remaining materialization, runtime shell, execution-receipt, execution-fixture, and closeout tasks are blocked until Mantle materializes a Kamacite ABI guest module and Octet emits a matching exact-byte report under a profile that admits that ABI without weakening its reviewed SpaceWasm boundary.

## Non-Goals

- No replacement for the WIT Component Model adapter suite.
- No WIT, components, WASI, or ambient authority in SpaceWasm.
- No canonical-IR, source-losslessness, semantic-equivalence, runtime-correctness, sandbox-completeness, certification, production, or release claim.
- No local production build path or floating upstream acquisition.
- No portable interpreter-state serialization or migration.
