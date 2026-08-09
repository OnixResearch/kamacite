# Algebraic Effect Model Specification

## Purpose

Defines Kamacite's canonical algebraic-effect artifact model: effect signatures, rows, summaries, handler descriptors, handler-application receipts, continuation policy, and host-capability separation.

## Requirements

### Requirement: Algebraic effect IR boundary

r[kamacite.algebraic_effects.ir_boundary] Kamacite MUST model algebraic effects as canonical data artifacts rather than as a Kamacite source language, evaluator, runtime, or Rust language extension. Algebraic-effect artifacts MUST use explicit schema versions, Preserves canonical bytes or supported content references, BLAKE3 identity over Kamacite-owned canonical bytes, and explicit non-claims.

#### Scenario: Algebraic effect artifact identity is canonical

r[kamacite.algebraic_effects.ir_boundary.canonical_identity]

- GIVEN an algebraic-effect artifact has Preserves canonical bytes and metadata aliases
- WHEN Kamacite computes the artifact identity
- THEN the BLAKE3 hash over canonical bytes MUST define identity and aliases MUST remain metadata.

#### Scenario: Unsupported algebraic effect version fails closed

r[kamacite.algebraic_effects.ir_boundary.unsupported_version]

- GIVEN an algebraic-effect artifact declares an unsupported schema version
- WHEN Kamacite validates the artifact
- THEN validation MUST reject it with a deterministic unsupported-algebraic-effect-version diagnostic.

#### Scenario: Kamacite does not become an evaluator

r[kamacite.algebraic_effects.ir_boundary.no_evaluator_claim]

- GIVEN an artifact declares algebraic-effect signatures, rows, handlers, or receipts
- WHEN Kamacite validates the artifact
- THEN validation MUST classify the artifact as data and MUST NOT claim Kamacite executed a program unless a separate bounded runtime receipt is present.

### Requirement: EffectSignature vocabulary

r[kamacite.algebraic_effects.signature_vocabulary] A Kamacite `EffectSignature` MUST bind a schema version, operation identities, operation names as metadata, parameter schemas, result schemas, declared determinism or replay class, dependency identities, optional source spans, and explicit non-claims. Operation identity MUST be derived from canonical signature data rather than from a human name alone.

#### Scenario: Operation identity ignores display name changes

r[kamacite.algebraic_effects.signature_vocabulary.name_metadata]

- GIVEN an operation keeps the same canonical signature data while a display name or alias changes
- WHEN Kamacite computes operation identity
- THEN the operation identity MUST remain stable and the name change MUST be recorded as metadata.

#### Scenario: Missing operation schema fails closed

r[kamacite.algebraic_effects.signature_vocabulary.missing_schema]

- GIVEN an effect signature omits the parameter schema or result schema for an operation
- WHEN Kamacite validates the signature
- THEN validation MUST reject it with a deterministic incomplete-effect-signature diagnostic.

### Requirement: EffectRow and EffectSummary vocabulary

r[kamacite.algebraic_effects.row_summary_vocabulary] Kamacite `EffectRow` and `EffectSummary` artifacts MUST bind required operation identities, provided operation identities, discharged operation identities, residual operation identities, row openness or closure, dependency identities, canonical ordering rules, and non-claims. Unknown or undeclared operations MUST fail closed rather than being treated as pure, handled, or host-authorized.

#### Scenario: Residual effects remain explicit

r[kamacite.algebraic_effects.row_summary_vocabulary.residual_effects]

- GIVEN a handler discharges only part of an input effect row
- WHEN Kamacite emits or validates the resulting effect summary
- THEN every remaining operation identity MUST appear in the residual row and the result MUST NOT be classified as pure.

#### Scenario: Undeclared operation fails closed

r[kamacite.algebraic_effects.row_summary_vocabulary.undeclared_operation]

- GIVEN an effect row references an operation identity missing from the declared signature set
- WHEN Kamacite validates the row
- THEN validation MUST reject it with a deterministic undeclared-effect-operation diagnostic.

#### Scenario: Open rows are not inferred pure

r[kamacite.algebraic_effects.row_summary_vocabulary.open_row_not_pure]

- GIVEN an effect row is explicitly open to additional operations
- WHEN Kamacite classifies the summary
- THEN validation MUST preserve the open-row status and MUST NOT infer purity from the listed operations alone.

### Requirement: HandlerDescriptor vocabulary

r[kamacite.algebraic_effects.handler_descriptor] A Kamacite `HandlerDescriptor` MUST bind schema version, handler identity, accepted input rows, handled operation identities, produced residual rows, adapter or evaluator identity, dependency identities, declared handler laws, continuation policy, replay profile, host-capability requirements, diagnostics policy, and explicit non-claims.

#### Scenario: Handler names do not prove discharge

r[kamacite.algebraic_effects.handler_descriptor.name_not_discharge]

- GIVEN a handler descriptor has a trusted-looking name or path but omits handled operation identities
- WHEN Kamacite validates the handler descriptor
- THEN validation MUST reject discharge claims until handled operations and residual rows are declared explicitly.

#### Scenario: Handler residual row is checked

r[kamacite.algebraic_effects.handler_descriptor.residual_row]

- GIVEN a handler descriptor claims to transform an input row into an output row
- WHEN Kamacite validates the descriptor
- THEN every unhandled input operation MUST appear in the residual row unless a checked law or receipt explicitly discharges it.

### Requirement: Handler application receipts

r[kamacite.algebraic_effects.handler_receipts] A Kamacite handler-application receipt MUST bind input artifact identities, input effect summary, handler descriptor identity, discharged operations, residual effect summary, checked laws, continuation-use evidence when claimed, replay or effect-log references, diagnostics, output identities, and non-claims. Passing receipts MUST NOT grant host authority, runtime correctness, or semantic equivalence beyond the bounded checked claim.

#### Scenario: Passing handler receipt binds exact discharge

r[kamacite.algebraic_effects.handler_receipts.exact_discharge]

- GIVEN a handler application succeeds for a declared input effect row
- WHEN Kamacite emits the receipt
- THEN the receipt MUST name the handler descriptor, discharged operation identities, residual operation identities, checked laws, output identities, and non-claims.

#### Scenario: Handler receipt denies unknown effect

r[kamacite.algebraic_effects.handler_receipts.unknown_effect_denied]

- GIVEN a handler application observes an operation not declared in the input row or handler descriptor
- WHEN Kamacite validates the receipt
- THEN validation MUST reject it with a deterministic unknown-effect-operation diagnostic.

### Requirement: Continuation policy boundary

r[kamacite.algebraic_effects.continuation_policy] Kamacite handler descriptors and handler-application receipts MUST classify continuation behavior explicitly. Supported claims MUST distinguish no-continuation behavior, single-use continuation behavior, and unsupported continuation behavior. A continuation claim without matching descriptor and receipt evidence MUST fail closed.

#### Scenario: Single-use continuation claim requires evidence

r[kamacite.algebraic_effects.continuation_policy.single_use_evidence]

- GIVEN a handler receipt claims single-use continuation behavior
- WHEN Kamacite validates the receipt
- THEN the receipt MUST include matching descriptor policy and continuation-use evidence before the claim is accepted.

#### Scenario: Continuation policy mismatch fails closed

r[kamacite.algebraic_effects.continuation_policy.mismatch]

- GIVEN a handler descriptor declares no-continuation behavior but a receipt records continuation use
- WHEN Kamacite validates the receipt
- THEN validation MUST reject it with a deterministic continuation-policy-mismatch diagnostic.

### Requirement: Host capability separation

r[kamacite.algebraic_effects.host_boundary] Kamacite MUST keep algebraic operations separate from host capabilities. Declaring an algebraic operation MUST NOT grant filesystem, process, network, clock, random, credential, package-manager, dynamic-library, mutation, or other ambient host authority. Host authority MAY be admitted only by separate adapter descriptors, capability gates, and receipts that fail closed by default.

#### Scenario: Algebraic operation does not grant host authority

r[kamacite.algebraic_effects.host_boundary.operation_not_authority]

- GIVEN an effect signature declares an operation that a runtime may later implement using filesystem, network, clock, random, credential, or mutation behavior
- WHEN Kamacite validates the signature or row
- THEN validation MUST treat the operation as a symbolic effect requirement and MUST NOT classify it as host-authorized.

#### Scenario: Host-backed handler requires separate admission

r[kamacite.algebraic_effects.host_boundary.host_backed_handler]

- GIVEN a handler descriptor says an operation is implemented by a host-backed adapter
- WHEN Kamacite validates a handler receipt
- THEN the receipt MUST reference separate host-capability admission evidence before any host-backed execution claim is accepted.
