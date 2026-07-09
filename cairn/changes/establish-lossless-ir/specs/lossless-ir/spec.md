# Lossless IR Specification

## Purpose

Defines Braid's initial format strategy for lossless interchange across Nickel, Rust, and Steel Scheme.

## Requirements

### Requirement: Preserves-backed canonical format strategy

r[braid.lossless_ir.format_strategy] Braid MUST use Preserves as the canonical encoded IR candidate for stack-owned value-level interchange, source envelopes, and receipt hash material. Braid MAY emit JSON as a compatibility projection, but JSON MUST NOT be the authority for losslessness when Preserves bytes are available.

#### Scenario: Canonical Preserves value round-trips

r[braid.lossless_ir.format_strategy.preserves_roundtrip]

- GIVEN a Braid canonical value has been encoded as Preserves bytes with a supported schema label
- WHEN an adapter decodes and re-encodes the value without changing the declared value
- THEN the canonical Preserves identity MUST be preserved or the adapter MUST emit a deterministic mismatch diagnostic.

#### Scenario: JSON projection is not authoritative

r[braid.lossless_ir.format_strategy.json_projection]

- GIVEN a Braid value has both canonical Preserves bytes and a JSON projection
- WHEN the two disagree or the JSON projection cannot represent the full value identity
- THEN validation MUST treat the Preserves value as authoritative and mark the JSON projection as invalid or incomplete.

### Requirement: Language naming boundary

r[braid.lossless_ir.language_boundary] Braid MUST NOT be described as a programming language unless a later accepted change defines a human-authored syntax, grammar, evaluator, type rules, and semantics. Current Braid surfaces SHOULD be described as an interchange IR, source-envelope format, receipt contract, or constrained policy vocabulary.

#### Scenario: Braid is presented as an IR

r[braid.lossless_ir.language_boundary.ir_terms]

- GIVEN Braid documentation describes the current project scope
- WHEN it names the artifact model
- THEN it SHOULD use terms such as interchange IR, source-envelope format, receipt contract, or constrained policy vocabulary rather than programming language.

#### Scenario: DSL claim requires semantics

r[braid.lossless_ir.language_boundary.dsl_requires_semantics]

- GIVEN a Braid surface is proposed as a DSL or language
- WHEN the proposal is reviewed
- THEN it MUST define the syntax, grammar, evaluator, type rules, semantics, and non-claims before the language label is accepted.

### Requirement: Explicit losslessness classes

r[braid.lossless_ir.losslessness_classes] Braid MUST classify each artifact claim as `source-envelope`, `canonical-value`, `constrained-policy`, or `unsupported-source-translation` before accepting a round-trip or conversion claim.

#### Scenario: Arbitrary source translation is denied as lossless

r[braid.lossless_ir.losslessness_classes.arbitrary_source_denied]

- GIVEN arbitrary Nickel, Rust, or Steel source is requested as a lossless translation into another host language
- WHEN no constrained Braid schema and adapter contract is declared
- THEN Braid MUST reject the lossless translation claim and MAY preserve the original source as a source envelope.

#### Scenario: Constrained policy round-trip names the schema

r[braid.lossless_ir.losslessness_classes.constrained_policy]

- GIVEN a policy is authored in a constrained Braid schema
- WHEN adapters render it to Nickel, Rust, or Steel-facing forms and parse it back
- THEN the receipt MUST name the schema and compare the resulting canonical IR instead of claiming arbitrary host-language equivalence.

### Requirement: Source preservation envelope

r[braid.lossless_ir.source_preservation] Braid MUST preserve original source bytes, declared language, parser or normalizer identity, and BLAKE3 source hash for source-bearing Nickel, Rust, and Steel artifacts.

#### Scenario: Normalization does not replace source

r[braid.lossless_ir.source_preservation.normalization]

- GIVEN a Nickel, Rust, or Steel source artifact has a normalized projection
- WHEN Braid stores or validates the artifact
- THEN the original source bytes and source hash MUST remain available as the source identity and the normalized projection MUST be marked as derived evidence.

#### Scenario: Source hash mismatch fails closed

r[braid.lossless_ir.source_preservation.hash_mismatch]

- GIVEN a source envelope declares a BLAKE3 source hash
- WHEN the stored source bytes hash to a different value
- THEN validation MUST fail closed with a deterministic source-hash diagnostic.

### Requirement: Nickel, Rust, and Steel adapter boundaries

r[braid.lossless_ir.adapter_boundaries] Braid adapters for Nickel, Rust, and Steel MUST cross the trust boundary through Braid IR values, source envelopes, and deterministic receipts rather than ambient host-language state.

#### Scenario: Nickel export is a derived projection

r[braid.lossless_ir.adapter_boundaries.nickel_projection]

- GIVEN a Nickel-authored contract exports a runtime value
- WHEN Braid ingests the export
- THEN the receipt MUST distinguish the Nickel source envelope from the exported Braid IR or compatibility JSON projection.

#### Scenario: Steel adapter has no ambient authority

r[braid.lossless_ir.adapter_boundaries.steel_no_ambient_authority]

- GIVEN a Steel adapter requests filesystem, process, network, credential, clock, or mutation authority not declared in the adapter contract
- WHEN Braid validates the evaluation request or receipt
- THEN validation MUST deny the request or receipt deterministically without evaluating the undeclared host action.

### Requirement: WASM target boundary

r[braid.lossless_ir.wasm_target_boundary] Braid MUST treat WebAssembly as an optional adapter, validator, or conformance target rather than the canonical IR. WASM components or modules MAY consume Braid IR/source-envelope bytes and emit deterministic receipts, but they MUST NOT replace Preserves as the authoritative value or source-envelope identity.

#### Scenario: WASM adapter stays receipt-bounded

r[braid.lossless_ir.wasm_target_boundary.adapter_receipt]

- GIVEN a Braid adapter is packaged as a WebAssembly module or component
- WHEN Braid invokes or validates the adapter
- THEN the adapter MUST receive only declared inputs and capabilities and MUST return a deterministic Braid receipt over Preserves-backed identity.

#### Scenario: WASM is rejected as canonical IR

r[braid.lossless_ir.wasm_target_boundary.not_ir]

- GIVEN a tool claims that compiled WebAssembly is the lossless representation of arbitrary Nickel, Rust, or Steel source
- WHEN Braid classifies the claim
- THEN validation MUST reject the canonical-IR claim and MAY accept the WebAssembly artifact only as a target artifact or source envelope with explicit non-claims.

### Requirement: Stack evidence integration boundary

r[braid.lossless_ir.stack_evidence_boundary] Braid MUST integrate with Valence and Octet through bounded evidence surfaces before adopting direct crate coupling. Braid receipts SHOULD expose stable IDs, source spans, BLAKE3 hashes, verification roles, and non-claims in a form that Valence can wrap or consume. Braid Rust code SHOULD run through Octet checks once Rust crates exist, but Octet findings MUST NOT be represented as proof of Nickel, Rust, or Steel semantic equivalence.

#### Scenario: Valence wraps Braid receipt claims

r[braid.lossless_ir.stack_evidence_boundary.valence_receipts]

- GIVEN a Braid round-trip or adapter receipt has stable identity, source span metadata, verification role, canonical hash, and explicit non-claims
- WHEN the receipt is exported for stack evidence processing
- THEN the export SHOULD be mappable to Valence Evidence IR without broadening Braid's bounded identity-preservation claim.

#### Scenario: Octet checks Rust implementation only

r[braid.lossless_ir.stack_evidence_boundary.octet_checks]

- GIVEN Braid has Rust validators, adapters, fixture runners, or receipt code
- WHEN Octet checks or provenance collection run against that Rust scope
- THEN the resulting evidence MUST be classified as Rust source-shape/provenance evidence and MUST NOT be promoted to arbitrary language semantic-equivalence proof.

### Requirement: Round-trip evidence

r[braid.lossless_ir.round_trip_evidence] Braid MUST require positive and negative fixtures for every implemented conversion, adapter, or validation claim before the claim is accepted.

#### Scenario: Positive fixture proves stable identity

r[braid.lossless_ir.round_trip_evidence.positive]

- GIVEN a valid source envelope, canonical value, or constrained policy fixture
- WHEN the declared round-trip check runs
- THEN the fixture MUST pass and preserve the declared canonical identity.

#### Scenario: Negative fixture fails closed

r[braid.lossless_ir.round_trip_evidence.negative]

- GIVEN a fixture with stale hash, malformed schema, unsupported class, lossy projection, missing source bytes, or undeclared authority
- WHEN the declared round-trip check runs
- THEN validation MUST fail closed with deterministic diagnostics naming the rejected claim.

### Requirement: Non-claim boundary

r[braid.lossless_ir.non_claims] Passing Braid validation MUST NOT prove compiler correctness, evaluator soundness, verifier soundness, runtime behavior, authorization, security policy satisfaction, or semantic equivalence for arbitrary Nickel, Rust, or Steel programs.

#### Scenario: Receipt states bounded claim

r[braid.lossless_ir.non_claims.receipt]

- GIVEN a Braid validation or round-trip receipt passes
- WHEN an operator reads the supported claim
- THEN the receipt MUST state only the artifact class, identity preservation claim, adapter boundary, and explicit non-claims.
