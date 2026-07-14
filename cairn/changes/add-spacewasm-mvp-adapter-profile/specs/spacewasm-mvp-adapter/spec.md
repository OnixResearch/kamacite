# SpaceWasm MVP Adapter Profile Specification

## Purpose

Define a distinct experimental Kamacite adapter/validator profile for exact WebAssembly 1.0 core modules executed by one reviewed SpaceWasm cohort while keeping Preserves authoritative and all component, authority, semantic, safety, and production claims bounded.

## Requirements

### Requirement: The SpaceWasm profile is disjoint from WIT components

r[kamacite.spacewasm_adapter.boundary] Kamacite MUST classify `kamacite-spacewasm-mvp` as a distinct core-module adapter kind and MUST reject component/WIT descriptors or implicit fallback between this profile and the WIT Component Model adapter suite.

#### Scenario: Core-MVP descriptor is selected explicitly
- GIVEN a descriptor names the SpaceWasm MVP kind and matching ABI/profile
- WHEN adapter-kind admission runs
- THEN Kamacite MAY evaluate the MVP profile.

#### Scenario: Component descriptor reaches the MVP path
- GIVEN a descriptor names a component, WIT world, or component runtime profile
- WHEN MVP admission runs
- THEN Kamacite MUST reject it without translation or fallback.

### Requirement: The core-module byte ABI is explicit and versioned

r[kamacite.spacewasm_adapter.abi] Kamacite MUST define a versioned pointer/length/result ABI with declared memory, allocation, deallocation, entrypoint, host-function signatures, byte limits, and typed result/error classes.

#### Scenario: Module exports the exact ABI
- GIVEN every required export/import name, kind, signature, and memory fact matches
- WHEN ABI validation runs
- THEN an execution plan MAY reference those bindings.

#### Scenario: ABI shape is missing or ambiguous
- GIVEN an export/import is missing, duplicated, wrong-kind, wrong-signature, or incompatible with memory limits
- WHEN ABI validation runs
- THEN execution MUST be denied before runtime construction.

### Requirement: One exact runtime profile owns execution intent

r[kamacite.spacewasm_adapter.profile] The adapter profile MUST bind exact SpaceWasm source/materialization/configuration, module producer, admitted WebAssembly features, ABI, capability/effect/resource policy, external evidence, promotion state, and non-claims through typed reviewable configuration.

#### Scenario: Complete profile matches
- GIVEN every runtime, producer, artifact, ABI, evidence, resource, and policy identity matches
- WHEN profile admission runs
- THEN Kamacite MAY construct a closed execution plan.

#### Scenario: Profile or cohort drifts
- GIVEN any required profile, source, build, support, or generated projection identity differs
- WHEN admission runs
- THEN Kamacite MUST fail closed without widening features or selecting another runtime.

### Requirement: Every layer keeps a separate typed identity

r[kamacite.spacewasm_adapter.identity] Kamacite MUST preserve separate BLAKE3 identity domains for source envelope, canonical input/output, adapter descriptor, module bytes, ABI profile, SpaceWasm runtime/configuration, Mantle bundle, Octet report, optional differential report, hostcall transcript, and receipt.

#### Scenario: Same canonical input reaches different modules
- GIVEN one canonical input is evaluated by different exact module bytes
- WHEN identities are constructed
- THEN input identity remains equal while module and receipt identities differ.

#### Scenario: Artifact or evidence identity substitutes for canonical value
- GIVEN a descriptor uses module/runtime/report identity as canonical Kamacite value identity
- WHEN identity validation runs
- THEN Kamacite MUST reject the role confusion.

### Requirement: Exact materialization and static evidence precede execution

r[kamacite.spacewasm_adapter.materialization] Evidence-bearing execution MUST remeasure a complete Mantle module/runtime bundle and verify matching Octet artifact/profile/cohort evidence; optional ChaosControl differential evidence MUST remain separately identified and missing required evidence MUST block without local production build or floating acquisition.

#### Scenario: Bundle and report match exact bytes
- GIVEN every portable member and Octet fact matches the descriptor and module
- WHEN materialization admission runs
- THEN the shell MAY receive those exact artifacts and evidence refs.

#### Scenario: Bundle or report is stale or incomplete
- GIVEN a required member, digest, role, profile, cohort, or finding differs
- WHEN admission runs
- THEN execution MUST be denied without trusting a store path or build-success label.

### Requirement: Preserves bytes remain canonical input and output

r[kamacite.spacewasm_adapter.preserves_io] The pure core MUST validate bounded canonical Preserves or source-envelope bytes before execution and MUST revalidate returned bytes before constructing output or receipt identities; runtime memory and IR MUST NOT become Kamacite canonical IR.

#### Scenario: Canonical round trip succeeds
- GIVEN canonical input bytes and bounded canonical output bytes
- WHEN adapter validation completes
- THEN input/output identities are computed over those canonical bytes and retained in the receipt.

#### Scenario: Input or output is malformed or non-canonical
- GIVEN bytes fail parsing, schema, canonical encoding, size, or source-envelope checks
- WHEN boundary validation runs
- THEN the adapter fails closed without accepting a value or semantic claim.

### Requirement: Runtime execution exposes only admitted effects

r[kamacite.spacewasm_adapter.execution] The SpaceWasm shell MUST link only host functions present in the closed execution plan, MUST deny WASI and ambient filesystem/network/process/environment/clock/random/credential/compiler/evaluator/mutation/device/package authority, and MUST return explicit runtime observations to the core.

#### Scenario: Declared host function is authorized
- GIVEN import, capability, effect, authority, resource, and input bindings all match
- WHEN the module invokes the host function
- THEN the shell MAY dispatch it and MUST record the ordered bounded observation.

#### Scenario: Undeclared effect or import is requested
- GIVEN the module imports or invokes a binding absent from the plan
- WHEN linking or dispatch runs
- THEN execution MUST fail closed without ambient fallback.

### Requirement: Capabilities and resources are explicit and bounded

r[kamacite.spacewasm_adapter.capabilities] The profile MUST separately declare interpreter allocator/code/stack and guest memory/table/growth/instruction/hostcall limits plus capability/effect requirements, and possession of a descriptor or artifact MUST NOT grant authority.

#### Scenario: All declarations and grants fit
- GIVEN capabilities/effects are admitted and resource demand stays within every named bound
- WHEN execution runs
- THEN observations MAY be classified under that exact resource profile.

#### Scenario: Authority is missing or a bound is exceeded
- GIVEN a required grant is absent or interpreter/guest demand exceeds its bound
- WHEN admission or enforcement runs
- THEN execution MUST deny or stop with a typed result and MUST NOT silently widen another bound.

### Requirement: Adapter decisions have a functional core

r[kamacite.spacewasm_adapter.functional_core] Descriptor/profile/materialization/evidence/ABI/capability/effect/resource/canonical-value/receipt validation, identity material, execution planning, outcome classification, and diagnostic ordering MUST be pure deterministic logic.

#### Scenario: Identical facts are evaluated twice
- GIVEN identical normalized inputs
- WHEN the core evaluates them
- THEN it MUST produce identical identities, plans, decisions, receipts, and diagnostics without filesystem, process, runtime, network, environment, clock, or output effects.

### Requirement: Receipts bind execution without semantic promotion

r[kamacite.spacewasm_adapter.receipt] Execution and denial receipts MUST bind every declared source/value/descriptor/module/ABI/runtime/evidence/capability/effect/resource/hostcall/outcome identity and explicit non-claims.

#### Scenario: Execution completes
- GIVEN admitted execution returns validated output and observations
- WHEN receipt construction runs
- THEN the receipt MUST bind exact parent facts and the bounded result class.

#### Scenario: Receipt claims language equivalence or runtime correctness
- GIVEN a receipt claims arbitrary Nickel/Rust/Steel equivalence, SpaceWasm correctness, or canonical runtime IR
- WHEN receipt validation runs
- THEN Kamacite MUST reject the unsupported claim.

### Requirement: Valence export preserves bounded roles

r[kamacite.spacewasm_adapter.valence_export] Kamacite MUST export SpaceWasm adapter evidence with distinct artifact/value/profile/runtime/report/receipt roles, parent edges, scopes, and non-claims and MUST NOT transfer Kamacite, Mantle, Octet, ChaosControl, SpaceWasm, or Valence semantics across those roles.

#### Scenario: Valid receipt is projected
- GIVEN a verified bounded adapter receipt
- WHEN Valence export runs
- THEN every role and parent identity remains explicit and the export is recorded-only or boundary evidence.

### Requirement: Profile evidence preserves non-claims

r[kamacite.spacewasm_adapter.nonclaims] Kamacite MUST keep the SpaceWasm MVP profile experimental and MUST reject claims of WIT/component compatibility, arbitrary source-language semantics, runtime correctness, sandbox completeness, portable interpreter state, flight qualification, production readiness, or release eligibility.

#### Scenario: Positive fixture passes
- GIVEN an exact adapter fixture passes
- WHEN support status is rendered
- THEN the profile remains experimental until a later accepted promotion change.

#### Scenario: MVP evidence satisfies component or production gate
- GIVEN a gate attempts to use MVP evidence as component, production, or release proof
- WHEN claim validation runs
- THEN the gate MUST reject the unsupported promotion.

### Requirement: The profile has positive and negative fixtures

r[kamacite.spacewasm_adapter.fixtures] The profile MUST include positive exact-evidence, core-module, ABI, canonical round-trip, declared-hostcall, trap, replay, and resource cases plus negative component fallback, proposal, evidence, ABI, pointer/length, canonical-data, import/effect, authority, resource, runtime, receipt, identity, and overclaim cases.

#### Scenario: Adapter boundary changes
- GIVEN descriptor, profile, ABI, materialization, execution, receipt, or export behavior changes
- WHEN fixture validation runs
- THEN both positive and negative cases MUST execute or record exact external blockers.

### Requirement: Profile validation is complete

r[kamacite.spacewasm_adapter.validation] The change MUST run focused pure-core, shell, ABI, resource, receipt, manifest-freshness, positive/negative fixture, Mantle/Octet/optional Chaos evidence, Valence conformance, lifecycle, Cairn gate, and relevant flake checks before archive.

#### Scenario: External runtime evidence is unavailable
- GIVEN an exact bundle or optional differential report cannot run
- WHEN closeout evidence is prepared
- THEN the blocker, affected claim, and next-best deterministic check MUST be recorded without marking the unavailable rail passed.
