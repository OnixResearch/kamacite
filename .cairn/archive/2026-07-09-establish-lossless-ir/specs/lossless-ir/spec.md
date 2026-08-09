# Lossless IR Specification

## Purpose

Defines Kamacite's top-level format strategy, naming boundary, losslessness classes, fixture evidence, and non-claims.

## Requirements

### Requirement: Preserves-backed canonical format strategy

r[kamacite.lossless_ir.format_strategy] Kamacite MUST use Preserves as the canonical encoded IR candidate for stack-owned value-level interchange, source envelopes, and receipt hash material. Kamacite MAY emit JSON as a compatibility projection, but JSON MUST NOT be the authority for losslessness when Preserves bytes are available.

#### Scenario: Canonical Preserves value round-trips

r[kamacite.lossless_ir.format_strategy.preserves_roundtrip]

- GIVEN a Kamacite canonical value has been encoded as Preserves bytes with a supported schema label
- WHEN an adapter decodes and re-encodes the value without changing the declared value
- THEN the canonical Preserves identity MUST be preserved or the adapter MUST emit a deterministic mismatch diagnostic.

#### Scenario: JSON projection is not authoritative

r[kamacite.lossless_ir.format_strategy.json_projection]

- GIVEN a Kamacite value has both canonical Preserves bytes and a JSON projection
- WHEN the two disagree or the JSON projection cannot represent the full value identity
- THEN validation MUST treat the Preserves value as authoritative and mark the JSON projection as invalid or incomplete.

### Requirement: Language naming boundary

r[kamacite.lossless_ir.language_boundary] Kamacite MUST NOT be described as a programming language unless a later accepted change defines a human-authored syntax, grammar, evaluator, type rules, and semantics. Current Kamacite surfaces SHOULD be described as an interchange IR, source-envelope format, receipt contract, or constrained policy vocabulary.

#### Scenario: Kamacite is presented as an IR

r[kamacite.lossless_ir.language_boundary.ir_terms]

- GIVEN Kamacite documentation describes the current project scope
- WHEN it names the artifact model
- THEN it SHOULD use terms such as interchange IR, source-envelope format, receipt contract, or constrained policy vocabulary rather than programming language.

#### Scenario: DSL claim requires semantics

r[kamacite.lossless_ir.language_boundary.dsl_requires_semantics]

- GIVEN a Kamacite surface is proposed as a DSL or language
- WHEN the proposal is reviewed
- THEN it MUST define the syntax, grammar, evaluator, type rules, semantics, and non-claims before the language label is accepted.

### Requirement: Explicit losslessness classes

r[kamacite.lossless_ir.losslessness_classes] Kamacite MUST classify each artifact claim as `source-envelope`, `canonical-value`, `constrained-policy`, or `unsupported-source-translation` before accepting a round-trip or conversion claim.

#### Scenario: Arbitrary source translation is denied as lossless

r[kamacite.lossless_ir.losslessness_classes.arbitrary_source_denied]

- GIVEN arbitrary Nickel, Rust, or Steel source is requested as a lossless translation into another host language
- WHEN no constrained Kamacite schema and adapter contract is declared
- THEN Kamacite MUST reject the lossless translation claim and MAY preserve the original source as a source envelope.

#### Scenario: Constrained policy round-trip names the schema

r[kamacite.lossless_ir.losslessness_classes.constrained_policy]

- GIVEN a policy is authored in a constrained Kamacite schema
- WHEN adapters render it to Nickel, Rust, or Steel-facing forms and parse it back
- THEN the receipt MUST name the schema and compare the resulting canonical IR instead of claiming arbitrary host-language equivalence.

### Requirement: Round-trip evidence

r[kamacite.lossless_ir.round_trip_evidence] Kamacite MUST require positive and negative fixtures for every implemented conversion, adapter, or validation claim before the claim is accepted.

#### Scenario: Positive fixture proves stable identity

r[kamacite.lossless_ir.round_trip_evidence.positive]

- GIVEN a valid source envelope, canonical value, or constrained policy fixture
- WHEN the declared round-trip check runs
- THEN the fixture MUST pass and preserve the declared canonical identity.

#### Scenario: Negative fixture fails closed

r[kamacite.lossless_ir.round_trip_evidence.negative]

- GIVEN a fixture with stale hash, malformed schema, unsupported class, lossy projection, missing source bytes, or undeclared authority
- WHEN the declared round-trip check runs
- THEN validation MUST fail closed with deterministic diagnostics naming the rejected claim.

### Requirement: Non-claim boundary

r[kamacite.lossless_ir.non_claims] Passing Kamacite validation MUST NOT prove compiler correctness, evaluator soundness, verifier soundness, runtime behavior, authorization, security policy satisfaction, or semantic equivalence for arbitrary Nickel, Rust, or Steel programs.

#### Scenario: Receipt states bounded claim

r[kamacite.lossless_ir.non_claims.receipt]

- GIVEN a Kamacite validation or round-trip receipt passes
- WHEN an operator reads the supported claim
- THEN the receipt MUST state only the artifact class, identity preservation claim, adapter boundary, and explicit non-claims.
