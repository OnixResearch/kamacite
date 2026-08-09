# Wasm Adapter Suite Specification

## Purpose

Execute bounded Kamacite adapters through a WIT WebAssembly component shell while preserving source envelopes, canonical Preserves values, adapter descriptors, target artifacts, receipts, and non-claims as separate identities.

## Requirements

### Requirement: WebAssembly remains an adapter target

r[kamacite.wasm_adapter.boundary] Kamacite MUST treat WebAssembly components as optional adapter/validator target artifacts and MUST retain Preserves as the canonical authority for source-envelope, value, and receipt identity.

#### Scenario: Component adapter succeeds
- GIVEN a declared component consumes and returns valid canonical Kamacite Preserves bytes
- WHEN the bounded adapter profile executes successfully
- THEN Kamacite MAY emit an adapter receipt without relabeling the component or WIT as canonical Kamacite IR.

#### Scenario: Component claims canonical IR authority
- GIVEN an adapter descriptor or receipt claims that Wasm or WIT replaces Preserves identity
- WHEN validation runs
- THEN Kamacite MUST reject the overclaim.

### Requirement: WIT source and world have independent identity

r[kamacite.wasm_adapter.wit_identity] Kamacite MUST preserve WIT source bytes as a source envelope and MUST bind WIT package, world, import, export, parser/tool, and BLAKE3 identities independently from component and canonical payload identities.

#### Scenario: WIT source matches descriptor
- GIVEN preserved WIT bytes resolve to the descriptor's exact package/world and declared imports/exports
- WHEN descriptor validation runs
- THEN the WIT identity MAY participate in the execution plan.

#### Scenario: World name matches but WIT bytes drift
- GIVEN a WIT source changes while retaining the same displayed package/world name
- WHEN freshness validation runs
- THEN Kamacite MUST require a new WIT source identity and MUST reject stale descriptors or receipts.

### Requirement: Adapter descriptors bind complete execution facts

r[kamacite.wasm_adapter.descriptor] A Wasm adapter descriptor MUST bind adapter id/version, WIT package/world, exact component ref, imports/exports, canonical input/output schema refs, capability/effect declarations, resource profile, deterministic runtime profile, toolchain cohort, parent artifacts, non-claims, and a complete Mantle materialization-bundle ref before it can yield an executable plan.

#### Scenario: Descriptor is complete
- GIVEN every required field and parent identity is valid and role-correct
- WHEN descriptor validation runs
- THEN Kamacite MUST produce one deterministic descriptor identity.

#### Scenario: Descriptor omits effect or schema
- GIVEN an imported operation, canonical schema, resource profile, runtime cohort, or parent artifact is missing or role-confused
- WHEN validation runs
- THEN Kamacite MUST fail closed before execution.

### Requirement: Executable adapters are Mantle-materialized

r[kamacite.wasm_adapter.materialization] WIT bindings, guest compilation, component composition, and any WASI virtualization for executable adapters MUST be supplied through a versioned Mantle materialization bundle with exact WIT/component/cohort/profile/stage identities; Kamacite MUST remeasure and inspect the objects and MUST NOT rebuild missing or mismatched production artifacts.

#### Scenario: Complete adapter bundle is available
- GIVEN the descriptor, WIT, component, build cohort, expected runtime profile, Octet facts, and stage receipts match one verified Mantle bundle
- WHEN execution planning runs
- THEN Kamacite MAY continue to its independent Preserves, effect, authority, resource, and runtime admission.

#### Scenario: Descriptor has no valid materialization bundle
- GIVEN an otherwise valid descriptor has loose bytes, an incomplete bundle, or mismatched component/WIT/profile identities
- WHEN execution planning runs
- THEN Kamacite MUST return a deterministic unresolved or denied result and MUST NOT build or execute the production adapter.

### Requirement: Runtime profile is explicit and shared

r[kamacite.wasm_adapter.profile] Executable adapters MUST use a versioned deterministic Wasmtime component profile compatible with the admitted Molten stack profile and MUST deny execution when the cohort, feature posture, WIT world, or runtime configuration is unknown or stale.

#### Scenario: Compatible profile is available
- GIVEN descriptor, component facts, and runtime profile identify one supported cohort
- WHEN execution planning runs
- THEN Kamacite MAY return a shell execution plan.

#### Scenario: Shared profile evidence is unavailable
- GIVEN static descriptor facts are valid but no supported runtime profile is available
- WHEN execution is requested
- THEN Kamacite MUST keep static validation available but MUST deny live execution.

### Requirement: Capabilities and effects are declared but not self-authorizing

r[kamacite.wasm_adapter.capabilities] Every component import MUST map to a declared capability/effect operation, handler/replay profile, resource rule, and separate authority or deterministic fixture input where required; unknown or ambient imports MUST deny.

#### Scenario: Declared deterministic host function is supplied
- GIVEN an import maps to a declared deterministic fixture or separately admitted capability handle
- WHEN the shell builds its linker
- THEN it MAY expose only that function under the declared bounds.

#### Scenario: Component imports ambient WASI
- GIVEN a component imports undeclared filesystem, network, process, clock, random, environment, credential, compiler, evaluator, mutation, or device functionality
- WHEN import admission runs
- THEN Kamacite MUST deny before instantiation or effect execution.

### Requirement: Preserves input and output are canonical

r[kamacite.wasm_adapter.preserves_io] Kamacite MUST validate exact input bytes before invocation and exact output bytes after invocation against canonical Preserves encoding and declared schema identities, and MUST NOT accept WIT carrier validity alone as adapter success.

#### Scenario: Canonical round trip succeeds
- GIVEN canonical input bytes satisfy the declared input schema and returned bytes satisfy the declared output schema
- WHEN the component executes
- THEN the receipt MUST bind exact input/output BLAKE3 and schema refs.

#### Scenario: Output bytes are noncanonical or wrong-schema
- GIVEN the component returns bytes accepted by WIT but noncanonical under Preserves or invalid under the output schema
- WHEN output validation runs
- THEN Kamacite MUST deny the result and MUST NOT emit a passing receipt.

### Requirement: Adapter receipts preserve every layer

r[kamacite.wasm_adapter.receipt] Kamacite MUST emit deterministic execution or denial receipts binding WIT, source envelope, adapter descriptor, component, runtime profile, capability/effect/resource plan, canonical input/output, hostcall summary, result/trap class, parent artifacts, and explicit non-claims.

#### Scenario: Passing receipt is validated
- GIVEN a successful adapter execution and complete receipt
- WHEN receipt validation runs
- THEN every layer identity and parent link MUST match the admitted plan and measured bytes.

#### Scenario: Receipt swaps component or payload
- GIVEN a receipt refers to different component, WIT, descriptor, runtime, input, output, or authority facts
- WHEN validation runs
- THEN Kamacite MUST fail closed.

### Requirement: Wasm adapter decisions have a functional core

r[kamacite.wasm_adapter.functional_core] Descriptor/profile/Mantle-bundle/identity/edge validation, capability/effect/resource admission, WIT/component fact comparison, canonical Preserves input/output checks, execution-plan construction, receipt construction/validation, and Valence projection MUST be pure deterministic logic.

#### Scenario: Identical adapter facts produce identical plan
- GIVEN identical descriptor, WIT, component, profile, capability, resource, and input facts
- WHEN the core plans execution
- THEN it MUST return the same plan or blockers without filesystem, environment, process, clock, network, Wasmtime, output, or ambient mutable-state effects.

### Requirement: Valence export is a bounded projection

r[kamacite.wasm_adapter.valence_export] Kamacite MUST export validated adapter identities, spans, roles, parent edges, receipts, and non-claims through a deterministic Valence-friendly projection without transferring Kamacite source/value round-trip authority.

#### Scenario: Passing receipt is exported
- GIVEN a Kamacite Wasm adapter receipt passes validation
- WHEN Valence projection runs
- THEN the export MUST retain all layer identities and MUST use only supported recorded-only or boundary roles.

### Requirement: Wasm adapter suite has positive and negative fixtures

r[kamacite.wasm_adapter.fixtures] The suite MUST include positive Mantle-materialized component round-trip/hostcall fixtures and negative materialization-bundle, local-build, WIT, world, component, import/effect, authority, resource, canonical input/output, receipt, and overclaim fixtures.

#### Scenario: Adapter suite runs
- GIVEN all fixtures are materialized from their declared identities
- WHEN the validation harness executes
- THEN positive fixtures MUST pass and each negative fixture MUST fail at its declared pre-side-effect or post-output boundary.

### Requirement: Wasm adapter validation is reviewable

r[kamacite.wasm_adapter.validation] Changes to Wasm adapter schemas, profiles, core validation, shell execution, receipts, fixtures, or Valence export MUST run focused positive/negative tests, manifest freshness, Octet checks, lifecycle validation, Cairn gates, and relevant flake checks.

#### Scenario: Adapter change is reviewed
- GIVEN adapter behavior or evidence changes
- WHEN validation evidence is assembled
- THEN it MUST identify the exact Mantle bundle and WIT/component/runtime cohort and include both positive and negative results without claiming arbitrary semantic equivalence.
