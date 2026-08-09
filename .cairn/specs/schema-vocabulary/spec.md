# Schema Vocabulary Specification

## Purpose

Defines the first Kamacite vocabulary surfaces that future Preserves schemas, Rust DTOs, fixtures, and receipts must implement.

## Requirements

### Requirement: Versioned schema vocabulary

r[kamacite.lossless_ir.schema_vocabulary] Kamacite MUST define a versioned schema vocabulary for source envelopes, canonical values, adapter descriptors, target artifacts, and receipts before accepting implementation claims. Schema versions MUST be explicit in canonical bytes and receipts, and unsupported versions MUST fail closed.

#### Scenario: Unsupported schema version fails closed

r[kamacite.lossless_ir.schema_vocabulary.unsupported_version]

- GIVEN an artifact declares a schema version the validator does not support
- WHEN Kamacite decodes or validates the artifact
- THEN validation MUST reject it with a deterministic unsupported-schema-version diagnostic.

### Requirement: SourceEnvelope vocabulary

r[kamacite.lossless_ir.source_envelope_vocabulary] A Kamacite `SourceEnvelope` MUST bind original source bytes, source language label, media type or codec when known, declared parser or normalizer identity, optional source span index, BLAKE3 source hash, metadata aliases, and explicit non-claims. Derived projections MUST reference the source envelope identity rather than replacing it.

#### Scenario: SourceEnvelope keeps bytes authoritative

r[kamacite.lossless_ir.source_envelope_vocabulary.bytes_authoritative]

- GIVEN a source envelope includes original source bytes and a normalized or parsed projection
- WHEN Kamacite validates the source envelope
- THEN the BLAKE3 hash of the original source bytes MUST define source identity and the projection MUST be classified as derived evidence.

#### Scenario: SourceEnvelope missing bytes is invalid

r[kamacite.lossless_ir.source_envelope_vocabulary.missing_bytes]

- GIVEN a source envelope declares a source hash but omits the original source bytes without a supported external-content binding
- WHEN Kamacite validates the envelope
- THEN validation MUST reject the envelope with a deterministic missing-source-bytes diagnostic.

### Requirement: CanonicalValue vocabulary

r[kamacite.lossless_ir.canonical_value_vocabulary] A Kamacite `CanonicalValue` MUST bind schema version, Preserves codec identity, canonical Preserves bytes or a supported content-addressed reference, BLAKE3 canonical hash, optional compatibility projections, and projection non-claims. Compatibility projections MUST NOT be accepted as canonical identity.

#### Scenario: CanonicalValue projection is derived

r[kamacite.lossless_ir.canonical_value_vocabulary.projection_derived]

- GIVEN a canonical value has Preserves bytes and a JSON projection
- WHEN Kamacite validates the value
- THEN the Preserves bytes and canonical hash MUST be authoritative and the JSON projection MUST be marked as derived compatibility output.

#### Scenario: CanonicalValue stale hash fails closed

r[kamacite.lossless_ir.canonical_value_vocabulary.stale_hash]

- GIVEN a canonical value declares a BLAKE3 canonical hash
- WHEN the canonical Preserves bytes hash to a different value
- THEN validation MUST reject the value with a deterministic canonical-hash-mismatch diagnostic.

### Requirement: AdapterDescriptor vocabulary

r[kamacite.lossless_ir.adapter_descriptor_vocabulary] A Kamacite `AdapterDescriptor` MUST bind adapter kind, adapter version, evaluator or toolchain identity, supported input and output schemas, declared capabilities/effects, declared adapter laws, dependency identities, target kind when applicable, and explicit non-claims.

#### Scenario: AdapterDescriptor declares effects before use

r[kamacite.lossless_ir.adapter_descriptor_vocabulary.declared_effects]

- GIVEN an adapter descriptor declares its input schema, output schema, and capabilities
- WHEN Kamacite validates an adapter receipt
- THEN every observed effect MUST appear in the descriptor and undeclared effects MUST fail closed.

#### Scenario: AdapterDescriptor names law coverage

r[kamacite.lossless_ir.adapter_descriptor_vocabulary.law_coverage]

- GIVEN an adapter descriptor is used for round-trip, projection, evaluation, compilation, or validation work
- WHEN Kamacite accepts the descriptor
- THEN it MUST name the adapter laws it claims to satisfy and MUST leave unclaimed laws explicit.

### Requirement: TargetArtifact vocabulary

r[kamacite.lossless_ir.target_artifact_vocabulary] A Kamacite `TargetArtifact` MUST bind target kind, producing adapter or toolchain descriptor, input artifact identities, output artifact hashes, target-specific metadata, reproducibility limits, and explicit non-claims. Target artifacts MUST NOT be accepted as canonical Kamacite IR.

#### Scenario: TargetArtifact records toolchain provenance

r[kamacite.lossless_ir.target_artifact_vocabulary.toolchain_provenance]

- GIVEN Kamacite records a WebAssembly, LLVM, GHC, Rust MIR, object, or other compiled target artifact
- WHEN Kamacite emits the target artifact receipt
- THEN the receipt MUST bind producer identity, input identities, output hashes, target metadata, reproducibility limits, and non-claims.

#### Scenario: TargetArtifact cannot replace canonical value

r[kamacite.lossless_ir.target_artifact_vocabulary.not_canonical]

- GIVEN a target artifact hash is supplied where a canonical Preserves value hash is required
- WHEN Kamacite validates the artifact or receipt
- THEN validation MUST reject the substitution with a deterministic identity-domain diagnostic.

### Requirement: Receipt vocabulary

r[kamacite.lossless_ir.receipt_vocabulary] A Kamacite `Receipt` MUST bind receipt schema version, receipt kind, artifact class, input identities, output identities, adapter descriptors, checked laws, diagnostics, dependency closure, effect/capability summary, non-claims, and BLAKE3 receipt identity over canonical receipt bytes.

#### Scenario: Receipt binds checked claim

r[kamacite.lossless_ir.receipt_vocabulary.checked_claim]

- GIVEN a validation, round-trip, projection, adapter, target, or export receipt passes
- WHEN Kamacite emits the receipt
- THEN the receipt MUST state the exact checked claim, supporting identities, checked laws, diagnostics, dependency closure, and non-claims.

#### Scenario: Receipt identity is separate domain

r[kamacite.lossless_ir.receipt_vocabulary.identity_domain]

- GIVEN a receipt has inputs with source, canonical value, projection, target, adapter, evaluator, or compiler identities
- WHEN Kamacite computes receipt identity
- THEN the receipt hash MUST be a separate BLAKE3 identity over canonical receipt bytes and MUST NOT reuse any input identity as the receipt identity.

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
