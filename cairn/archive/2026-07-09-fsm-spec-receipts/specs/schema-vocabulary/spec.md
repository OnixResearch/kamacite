# Schema Vocabulary Delta

## ADDED Requirements

### Requirement: FSM spec vocabulary
r[kamacite.lossless_ir.fsm_spec_vocabulary] Kamacite MUST define a versioned FSM spec vocabulary for closed state universes, closed event universes, guard vocabularies, transition rows, invalid-transition denial rows, side-effect classes, checked laws, and explicit non-claims.

#### Scenario: Complete FSM spec validates
r[kamacite.lossless_ir.fsm_spec_vocabulary.complete]
- GIVEN an FSM spec declares schema version, FSM identity, closed states, closed events, declared guards, transition rows, denial rows, side-effect classes, checked laws, and non-claims
- WHEN Kamacite validates the spec
- THEN validation MUST accept the spec and preserve the canonical FSM spec identity.

#### Scenario: Unsupported FSM spec version fails closed
r[kamacite.lossless_ir.fsm_spec_vocabulary.unsupported_version]
- GIVEN an FSM spec declares an unsupported schema version
- WHEN Kamacite validates the spec
- THEN validation MUST reject it with a deterministic unsupported-schema-version diagnostic.

### Requirement: FSM universes are closed data
r[kamacite.lossless_ir.fsm_spec_vocabulary.closed_universes] Kamacite FSM specs MUST treat state and event universes as closed, explicitly enumerated data for the declared spec version.

#### Scenario: Duplicate state or event fails closed
r[kamacite.lossless_ir.fsm_spec_vocabulary.closed_universes.duplicates]
- GIVEN an FSM spec repeats a state identity or event identity
- WHEN Kamacite validates the spec
- THEN validation MUST reject it with a deterministic duplicate-universe-member diagnostic.

#### Scenario: Transition references declared universe members
r[kamacite.lossless_ir.fsm_spec_vocabulary.closed_universes.references]
- GIVEN a transition or denial row references a state or event not present in the closed universe
- WHEN Kamacite validates the spec
- THEN validation MUST reject it with a deterministic undeclared-state-or-event diagnostic.

### Requirement: FSM transition and denial coverage
r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage] Kamacite FSM specs MUST account for every declared state/event pair as either an admitted transition row or an explicit denial row.

#### Scenario: Every pair is covered
r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage.complete]
- GIVEN an FSM spec has closed state and event universes
- WHEN Kamacite computes the state/event pair coverage
- THEN every pair MUST appear in exactly one admitted transition row or explicit denial row.

#### Scenario: Missing invalid-transition denial fails closed
r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage.missing_denial]
- GIVEN a state/event pair is absent from admitted transition rows and denial rows
- WHEN Kamacite validates the spec
- THEN validation MUST reject it with a deterministic missing-pair-coverage diagnostic.

#### Scenario: Duplicate pair coverage fails closed
r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage.duplicate]
- GIVEN a state/event pair appears in multiple transition rows, multiple denial rows, or both an admitted transition and a denial
- WHEN Kamacite validates the spec
- THEN validation MUST reject it with a deterministic duplicate-pair-coverage diagnostic.

### Requirement: FSM specs reject wildcard/default rows
r[kamacite.lossless_ir.fsm_spec_vocabulary.no_wildcards] Kamacite FSM specs MUST NOT use wildcard or default transition rows to stand in for unenumerated state/event pairs.

#### Scenario: Wildcard row fails closed
r[kamacite.lossless_ir.fsm_spec_vocabulary.no_wildcards.reject]
- GIVEN an FSM spec uses a wildcard, default, otherwise, or catch-all row for states or events
- WHEN Kamacite validates the spec
- THEN validation MUST reject it and require explicit state/event pair coverage.

### Requirement: FSM guards and side effects are declared
r[kamacite.lossless_ir.fsm_spec_vocabulary.guards_side_effects] Kamacite FSM specs MUST declare guard vocabulary and side-effect classes before transition or denial rows reference them.

#### Scenario: Undeclared guard fails closed
r[kamacite.lossless_ir.fsm_spec_vocabulary.guards_side_effects.undeclared_guard]
- GIVEN a transition row references a guard identity not declared in the FSM guard vocabulary
- WHEN Kamacite validates the spec
- THEN validation MUST reject it with a deterministic undeclared-guard diagnostic.

#### Scenario: Denial preserves state and side-effect boundary
r[kamacite.lossless_ir.fsm_spec_vocabulary.guards_side_effects.denial]
- GIVEN a denial row is accepted
- WHEN Kamacite validates the row
- THEN the row MUST name a denial reason, preserve prior state, and use a declared no-side-effect or denial side-effect class.

### Requirement: FSM exhaustiveness receipt vocabulary
r[kamacite.lossless_ir.fsm_receipt_vocabulary] Kamacite MUST define an FSM exhaustiveness receipt vocabulary that binds FSM spec identity, state/event universe identities, transition coverage identity, denial coverage identity, adapter descriptor identity, diagnostics, checked laws, dependency identities, optional Octet and Valence links, non-claims, and BLAKE3 receipt identity.

#### Scenario: Receipt binds checked FSM claim
r[kamacite.lossless_ir.fsm_receipt_vocabulary.checked_claim]
- GIVEN an FSM spec validation or projection passes
- WHEN Kamacite emits the FSM receipt
- THEN the receipt MUST state the exact checked claim, supporting identities, checked laws, diagnostics, dependency closure, and non-claims.

#### Scenario: Receipt identity is separate from spec identity
r[kamacite.lossless_ir.fsm_receipt_vocabulary.identity_domain]
- GIVEN an FSM receipt references a canonical FSM spec identity and adapter identity
- WHEN Kamacite computes receipt identity
- THEN the receipt hash MUST be a separate BLAKE3 identity over canonical receipt bytes and MUST NOT reuse the spec or adapter identity as the receipt identity.

### Requirement: FSM positive and negative fixtures
r[kamacite.lossless_ir.fsm_fixtures] Kamacite MUST include positive and negative fixtures for FSM specs and receipts before accepting implementation claims.

#### Scenario: Positive FSM fixtures pass
r[kamacite.lossless_ir.fsm_fixtures.positive]
- GIVEN fixture entries contain complete FSM specs, guarded transitions, explicit denial coverage, Octet-linked receipts, and Valence export projections
- WHEN the Kamacite validation harness runs
- THEN positive fixtures MUST pass and produce deterministic golden receipts.

#### Scenario: Negative FSM fixtures fail closed
r[kamacite.lossless_ir.fsm_fixtures.negative]
- GIVEN fixture entries contain missing states or events, missing denial rows, duplicate transitions, wildcard rows, undeclared guards, stale identities, wrong identity domains, missing non-claims, or overclaims
- WHEN the Kamacite validation harness runs
- THEN negative fixtures MUST fail with deterministic diagnostics naming the violated FSM rule.

### Requirement: Pure FSM validation core
r[kamacite.lossless_ir.fsm_validation] Kamacite FSM spec and receipt validation MUST be pure deterministic logic over parsed in-memory DTOs.

#### Scenario: Shell owns IO and projection work
r[kamacite.lossless_ir.fsm_validation.pure]
- GIVEN Kamacite validates an FSM spec or receipt
- WHEN the pure core runs
- THEN filesystem access, process execution, evaluator calls, network access, clock/random reads, mutation, stdout/stderr, fixture refresh, and Valence export rendering MUST remain outside the core.

### Requirement: FSM validation evidence
r[kamacite.lossless_ir.fsm_final_validation] The FSM spec/receipt change MUST include Rust tests, fixture validation, manifest freshness, Valence export checks, and Cairn validation evidence before archive.

#### Scenario: Final validation is reviewable
r[kamacite.lossless_ir.fsm_final_validation.reviewable]
- GIVEN implementation is complete
- WHEN the change is prepared for archive
- THEN saved evidence MUST identify positive and negative fixture coverage, manifest projection freshness, export checks, and Cairn proposal/design/tasks validation results.
