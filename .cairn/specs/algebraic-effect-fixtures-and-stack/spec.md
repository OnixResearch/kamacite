# Algebraic Effect Fixtures and Stack Boundary Specification

## Purpose

Defines fixture requirements and stack integration boundaries for Kamacite algebraic-effect artifacts. The goal is to prove the model with positive and negative evidence while keeping Trellis, Valence, Aspen/Molten, and Octet roles explicit.

## Requirements

### Requirement: Algebraic effect fixture corpus

r[kamacite.algebraic_effects.fixture_corpus] Kamacite MUST provide positive and negative fixtures before accepting any implementation claim for algebraic-effect signatures, rows, summaries, handler descriptors, handler-application receipts, continuation policies, replay profiles, or host-capability separation. Fixtures MUST cover both valid artifacts and deterministic rejection cases.

#### Scenario: Positive fixture proves declared discharge

r[kamacite.algebraic_effects.fixture_corpus.positive_discharge]

- GIVEN a fixture declares an effect signature, an input row, a handler descriptor, and matching handler receipt
- WHEN Kamacite validates the fixture
- THEN validation MUST accept the declared discharge and preserve any residual row in the emitted summary.

#### Scenario: Negative fixture rejects undeclared operation

r[kamacite.algebraic_effects.fixture_corpus.negative_undeclared_operation]

- GIVEN a fixture references an operation not present in the declared signature set
- WHEN Kamacite validates the fixture
- THEN validation MUST reject it with the undeclared-effect-operation diagnostic.

#### Scenario: Negative fixture rejects host-capability confusion

r[kamacite.algebraic_effects.fixture_corpus.negative_host_confusion]

- GIVEN a fixture declares an algebraic operation and then claims filesystem, process, network, clock, random, credential, package-manager, dynamic-library, or mutation authority without separate admission evidence
- WHEN Kamacite validates the fixture
- THEN validation MUST reject the authority claim with a host-capability-confusion diagnostic.

#### Scenario: Negative fixture rejects stale canonical identity

r[kamacite.algebraic_effects.fixture_corpus.negative_stale_identity]

- GIVEN an algebraic-effect artifact declares a BLAKE3 identity that does not match its canonical bytes
- WHEN Kamacite validates the fixture
- THEN validation MUST reject it with a deterministic algebraic-effect-hash-mismatch diagnostic.

### Requirement: Algebraic effect replay profile

r[kamacite.algebraic_effects.replay_profile] Kamacite effect summaries and handler receipts MUST classify replay behavior as deterministic without external observations, recorded-replay through explicit effect-log entries, denied before execution, or non-replayable with explicit non-claims. Recorded replay artifacts MUST bind the effect summary identity, handler descriptor identity when present, canonical log bytes or supported references, and BLAKE3 log identity.

#### Scenario: Recorded replay binds effect log identity

r[kamacite.algebraic_effects.replay_profile.recorded_log]

- GIVEN a handler receipt claims recorded-replay behavior
- WHEN Kamacite validates the receipt
- THEN the receipt MUST bind canonical effect-log identity and MUST state that the log supports replay of the bounded run rather than host authorization or semantic correctness.

#### Scenario: Non-replayable effect stays non-claiming

r[kamacite.algebraic_effects.replay_profile.non_replayable]

- GIVEN an effect summary classifies an operation or handler result as non-replayable
- WHEN Kamacite exports or validates the summary
- THEN the summary MUST retain the non-replayable classification and MUST NOT be accepted as deterministic replay evidence.

### Requirement: Trellis proof-reference boundary

r[kamacite.algebraic_effects.stack_roles.trellis] Kamacite MAY reference Trellis algebraic-effect or ability-effect proof artifacts as proof/reference evidence for finite signatures, rows, summaries, handler discharge, sandbox attenuation, and single-use continuation facts. Trellis artifacts MUST NOT replace Kamacite canonical artifact identities, schemas, receipts, or local validation results.

#### Scenario: Trellis proof artifact is supporting evidence only

r[kamacite.algebraic_effects.stack_roles.trellis.supporting_evidence]

- GIVEN a Kamacite handler receipt references a Trellis proof artifact
- WHEN Kamacite validates or exports the receipt
- THEN the proof artifact MAY be recorded as supporting evidence but MUST NOT replace the handler descriptor, effect summary, residual row, or Kamacite receipt identity.

### Requirement: Valence evidence export boundary

r[kamacite.algebraic_effects.stack_roles.valence] Kamacite algebraic-effect receipts SHOULD export stable IDs, spans, roles, BLAKE3 hashes, checked claims, diagnostics, and non-claims in a form Valence can wrap or consume. Valence export MUST NOT broaden Kamacite's claim from identity and receipt validation into runtime correctness, host authorization, or arbitrary semantic equivalence.

#### Scenario: Valence wraps effect receipt without broadening claim

r[kamacite.algebraic_effects.stack_roles.valence.wrap_receipt]

- GIVEN a Kamacite algebraic-effect receipt has stable IDs, spans, roles, hashes, diagnostics, and non-claims
- WHEN the receipt is exported for Valence
- THEN the export MUST preserve the bounded claim and MUST NOT classify it as runtime correctness or host authorization evidence.

### Requirement: Aspen/Molten runtime consumption boundary

r[kamacite.algebraic_effects.stack_roles.aspen] Aspen/Molten or another runtime MAY consume Kamacite algebraic-effect signatures, rows, handler descriptors, replay profiles, and effect-log projections. Runtime execution receipts MUST reference Kamacite artifact identities and MUST remain runtime evidence rather than redefining the canonical Kamacite effect model.

#### Scenario: Runtime handler profile references Kamacite identity

r[kamacite.algebraic_effects.stack_roles.aspen.handler_profile]

- GIVEN Aspen/Molten executes a handler profile derived from Kamacite algebraic-effect artifacts
- WHEN Aspen/Molten emits runtime evidence
- THEN the evidence MUST reference the Kamacite handler descriptor and effect summary identities instead of redefining those artifacts by runtime-local names.

### Requirement: Octet policy-check boundary

r[kamacite.algebraic_effects.stack_roles.octet] Octet MAY check Rust validators, adapters, fixture runners, provenance, and effect/capability policy metadata for Kamacite algebraic-effect implementation code. Octet evidence MUST remain Rust source-shape, policy, and provenance evidence and MUST NOT be represented as a Rust language-level algebraic-effect system or proof of arbitrary semantic equivalence.

#### Scenario: Octet finding stays source-shape evidence

r[kamacite.algebraic_effects.stack_roles.octet.source_shape_only]

- GIVEN Octet checks Kamacite Rust code that validates algebraic-effect artifacts
- WHEN Kamacite records Octet output
- THEN the result MUST be classified as Rust source-shape, policy, or provenance evidence only and MUST NOT replace Kamacite algebraic-effect receipts.

### Requirement: Algebraic effect stack role inventory

r[kamacite.algebraic_effects.stack_roles] Kamacite MUST document the stack role split for algebraic effects: Kamacite owns canonical effect artifacts and receipt contracts, Trellis owns proof/reference logic, Valence owns evidence identity/linkage, Aspen/Molten owns runtime handler execution and replay consumption, and Octet owns Rust source-shape/provenance checking.

#### Scenario: Stack role conflict fails review

r[kamacite.algebraic_effects.stack_roles.conflict]

- GIVEN a change claims Aspen/Molten, Valence, Octet, or Trellis owns the canonical Kamacite algebraic-effect schema
- WHEN Kamacite reviews the change
- THEN the change MUST be rejected or revised until canonical schema ownership remains in Kamacite.
