# Algebraic Effect Model Specification Delta

## Purpose

Strengthen Kamacite effect-operation identity so handlers, logs, caches, runtimes, and compatibility decisions bind exact declared semantics rather than names or signature shape alone.

## ADDED Requirements

### Requirement: Semantic operation descriptors are canonical

r[kamacite.algebraic_effects.semantic_operation_descriptor] Kamacite MUST define a versioned semantic effect-operation descriptor whose BLAKE3 identity is computed over canonical Preserves data binding the nominal effect-family identity, parameter and result schema identities, continuation contract, replay class, dependency identities, and either a default-behavior identity or an explicit abstract-operation marker.

#### Scenario: Complete semantic descriptor is identified
- GIVEN an operation descriptor contains every required semantic field in canonical form
- WHEN Kamacite derives its operation identity
- THEN the BLAKE3 identity over the canonical descriptor MUST be the normative effect-operation identity.

#### Scenario: Behavior identity and abstract marker are both absent
- GIVEN an operation descriptor declares neither a default-behavior identity nor an explicit abstract-operation marker
- WHEN descriptor validation runs
- THEN Kamacite MUST reject it with a deterministic incomplete-semantic-operation diagnostic.

### Requirement: Names remain metadata

r[kamacite.algebraic_effects.semantic_operation_names] Display names, paths, aliases, source locations, and documentation MUST remain metadata outside semantic operation identity.

#### Scenario: Operation is renamed without semantic change
- GIVEN every semantic descriptor field remains identical while a display name, path, alias, source span, or documentation string changes
- WHEN identity is recomputed
- THEN the semantic operation identity MUST remain unchanged and the metadata change MUST remain separately observable.

#### Scenario: Name alone is supplied as identity
- GIVEN a handler or effect row refers only to a trusted-looking operation name
- WHEN normative validation runs
- THEN validation MUST deny until the exact semantic operation identity is supplied.

### Requirement: Semantic changes re-key operations

r[kamacite.algebraic_effects.semantic_operation_rekey] A change to effect-family identity, parameter schema, result schema, continuation contract, replay class, dependency identity, default-behavior identity, or abstract/default status MUST produce a different semantic operation identity.

#### Scenario: Default behavior changes under the same name and signature
- GIVEN an operation keeps its name and input/output schemas but its default-behavior identity changes
- WHEN identity is recomputed
- THEN the new semantic operation identity MUST differ from the prior identity.

#### Scenario: Same-shaped operations belong to different families
- GIVEN two operations have identical schemas and replay classes but different nominal effect-family identities
- WHEN identities are computed
- THEN the operation identities MUST differ.

### Requirement: Handlers and effect artifacts use exact semantic identities

r[kamacite.algebraic_effects.semantic_handler_matching] Effect rows, summaries, handler descriptors, handler receipts, effect-log entries, adapter bindings, and target-artifact imports MUST reference exact semantic operation identities, and a handler MUST NOT discharge a different identity merely because names or schema shapes match.

#### Scenario: Handler covers the exact operation
- GIVEN a handler descriptor lists the exact semantic operation identity performed by an input artifact
- WHEN handler validation runs
- THEN the operation MAY be classified handled subject to existing row, continuation, replay, and capability checks.

#### Scenario: Handler covers an older same-named identity
- GIVEN code requests a new operation identity but the handler lists only an older same-named or same-shaped identity
- WHEN handler validation runs
- THEN validation MUST deny with a semantic-operation-mismatch diagnostic.

### Requirement: Compatibility is explicit and directional

r[kamacite.algebraic_effects.semantic_operation_compatibility] Intentional substitution between different semantic operation identities MUST require a versioned directional compatibility artifact binding exact source and target identities, allowed contexts, supporting evidence refs, diagnostics, and non-claims. Compatibility MUST NOT be inferred from names, schemas, shared behavior refs, or artifact possession.

#### Scenario: Reviewed compatibility permits one bounded context
- GIVEN a compatibility artifact names exact old and new operation identities and admits replay-only substitution
- WHEN a runtime-compatibility projection is validated for replay
- THEN Kamacite MAY preserve that bounded compatibility claim without claiming general semantic equivalence or host authority.

#### Scenario: Compatibility is used in an undeclared direction or context
- GIVEN a compatibility artifact permits old-to-new replay substitution only
- WHEN a caller attempts new-to-old substitution or live host execution
- THEN validation MUST deny the unsupported use.

### Requirement: Identity migration is versioned and non-reinterpreting

r[kamacite.algebraic_effects.semantic_operation_migration] Existing operation identities MUST remain versioned compatibility artifacts and MUST NOT be silently reinterpreted as semantic operation identities. Migration MUST emit explicit old/new identity mappings and changed-field diagnostics.

#### Scenario: Legacy artifact is loaded
- GIVEN an algebraic-effect artifact uses the prior operation identity version
- WHEN the new validator reads it
- THEN the artifact MUST remain legacy-versioned and MUST require explicit migration or compatibility evidence for new semantic-key use.

#### Scenario: Migration changes no semantic fields
- GIVEN a legacy artifact supplies sufficient canonical source facts to construct the new descriptor
- WHEN migration is previewed
- THEN the plan MUST show old and new identities, descriptor fields, and non-claims without mutating fixtures unless update is explicitly requested.

### Requirement: Semantic identity is not authority or correctness

r[kamacite.algebraic_effects.semantic_identity_boundary] A valid semantic operation identity or compatibility artifact MUST NOT grant host capability, runtime authority, handler correctness, default-behavior correctness, semantic equivalence, replay truth, or release eligibility.

#### Scenario: Matching key lacks host admission
- GIVEN a runtime handler matches the exact semantic operation identity but lacks required capability or authority evidence
- WHEN execution admission runs
- THEN the host-backed action MUST remain denied.

### Requirement: Semantic identity has positive and negative fixtures

r[kamacite.algebraic_effects.semantic_identity_fixtures] Kamacite MUST include positive rename-stability, exact-handler, abstract-operation, migration-preview, and bounded-compatibility fixtures plus negative behavior/schema/continuation/replay drift, nominal-family collision, stale identity, missing behavior marker, same-name handler substitution, wrong-direction compatibility, host-authority confusion, and overclaim fixtures.

#### Scenario: Golden semantic descriptor changes
- GIVEN an intentional semantic descriptor field changes
- WHEN fixture validation runs
- THEN the affected operation, handler, log, receipt, adapter, target, and Valence projection identities MUST change through an explicit reviewed golden update rather than silent acceptance.
