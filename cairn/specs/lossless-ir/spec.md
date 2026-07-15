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

### Requirement: Canonical identity uses a real Preserves codec

r[kamacite.findings.preserves_authority.real_codec] Kamacite MUST decode supported Preserves transfer syntax into a typed value and MUST compute canonical artifact identity from the corresponding canonical Preserves binary encoding. Prefixing JSON or another projection with a Preserves label MUST NOT qualify as canonical Preserves bytes.

#### Scenario: Text and binary transfer syntax converge

- GIVEN equivalent schema-valid values in supported Preserves text and binary transfer syntax
- WHEN Kamacite canonicalizes both values
- THEN it MUST emit identical canonical binary bytes and identical BLAKE3 identities.

### Requirement: Validated fields are bound to canonical bytes

r[kamacite.findings.preserves_authority.typed_binding] Every trusted typed artifact MUST be derived from the canonical decoded value or encoded from the same validated typed value. Kamacite MUST reject a parallel DTO whose normative fields disagree with the supplied canonical bytes.

#### Scenario: Detached DTO substitution is rejected

- GIVEN canonical bytes encode one FSM, effect model, receipt, or documentation artifact while a parallel DTO describes another
- WHEN Kamacite validates the artifact
- THEN validation MUST fail with deterministic canonical-content-mismatch evidence before emitting a trusted identity.

### Requirement: Legacy pseudo-codecs remain non-authoritative

r[kamacite.findings.preserves_authority.legacy_codec] Kamacite MUST version the real canonical codec and MUST NOT silently reinterpret existing prefixed-JSON identities as canonical Preserves identities.

#### Scenario: Legacy identity is encountered

- GIVEN an artifact declares a legacy codec that used labeled deterministic JSON
- WHEN the new validator loads it
- THEN the artifact MUST be rejected for canonical authority or retained only as explicitly caveated compatibility evidence with its original identity.

### Requirement: Canonical decoding is bounded and tested

r[kamacite.findings.preserves_authority.validation] Preserves decoding and canonical encoding MUST apply named policy limits and MUST include positive and negative fixtures for transfer-syntax equivalence, malformed input, non-canonical input, unsupported schema versions, and DTO/bytes substitution.

#### Scenario: Hostile encoded input exceeds policy

- GIVEN Preserves input exceeds a declared depth, collection, string, or byte limit
- WHEN the pure canonicalization core decodes it
- THEN validation MUST return a deterministic limit diagnostic and MUST NOT panic, allocate without bound, or emit an identity.

### Requirement: JSON-to-Preserves mapping is versioned, closed, and bounded

r[kamacite.nickel_preserves_adapter.mapping_profile] Kamacite MUST define a versioned `nickel-json-to-preserves-v1` profile that maps every admitted JSON null, boolean, string, number, array, and object case into one declared Preserves value shape. The profile MUST define object-key ordering, numeric classification, Unicode handling, and named depth, collection, string, numeric, input-byte, and output-byte limits. Inputs outside the admitted profile MUST fail deterministically rather than being coerced, truncated, or represented through debug text.

#### Scenario: Equivalent JSON values use one canonical value identity

- GIVEN two separately admitted JSON artifacts differ only in insignificant whitespace or object member order and decode to the same admitted JSON value
- WHEN each value is mapped and canonically encoded under the same profile
- THEN their JSON projection identities MAY differ while their canonical packed Preserves bytes and canonical-value identities MUST match.

#### Scenario: Mapping policy cannot represent a value safely

- GIVEN a decoded JSON value exceeds a named policy limit or has a numeric representation the selected profile does not admit unambiguously
- WHEN the pure mapping core processes it
- THEN it MUST return a deterministic profile diagnostic and MUST NOT emit canonical bytes or an identity.

### Requirement: Canonical output comes from one validated Preserves value

r[kamacite.nickel_preserves_adapter.canonical_output] The adapter MUST construct one typed Preserves value from the admitted JSON value, validate that value against the selected schema/profile, and derive canonical packed bytes and BLAKE3 identity from that same value through the codec established by `make-preserves-authority-real`. Caller-supplied canonical bytes or parallel DTO fields MUST NOT substitute for this construction.

#### Scenario: Detached output bytes are supplied

- GIVEN caller-supplied packed bytes encode a value different from the admitted JSON mapping
- WHEN Kamacite checks or writes the adaptation result
- THEN it MUST reject the detached bytes with canonical-content-mismatch evidence before emitting a success receipt.

### Requirement: Adaptation claim is limited to the exported JSON value

r[kamacite.nickel_preserves_adapter.non_claim] A passing adapter receipt MUST claim preservation only of the decoded JSON-compatible value selected from the admitted Nickel export under the named mapping and codec profiles. It MUST NOT claim arbitrary Nickel source losslessness, import-closure completeness beyond the Nickel export receipt, semantic equivalence, evaluator correctness, authorization, runtime behavior, or release eligibility.

#### Scenario: Caller requests arbitrary Nickel equivalence

- GIVEN canonical Preserves bytes were produced from a valid Nickel JSON export
- WHEN a caller labels the receipt as proof of arbitrary Nickel source or evaluator semantic equivalence
- THEN Kamacite MUST reject the claim or preserve it only as unsupported external metadata outside the trusted receipt.

### Requirement: Rendered text remains a non-authoritative projection

r[kamacite.nickel_preserves_adapter.text_projection] Kamacite MAY emit deterministic Preserves text for review, but the text artifact MUST be derived from the validated canonical value, MUST carry a distinct projection identity, and MUST NOT replace canonical packed bytes as artifact identity. This adapter MUST NOT introduce Markdown or AsciiDoc conversion behavior.

#### Scenario: Review text is requested

- GIVEN canonical packed Preserves output has been constructed successfully
- WHEN the operator requests a review-text projection
- THEN Kamacite MUST render it from the same validated value, bind both identities in the receipt, and identify the text as non-authoritative.
