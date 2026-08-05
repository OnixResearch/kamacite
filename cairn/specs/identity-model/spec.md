# Identity Model Specification

## Purpose

Defines Kamacite's content-addressed identity model, inspired by Unison discipline but owned by Kamacite's canonical bytes and BLAKE3 hashes.

## Requirements

### Requirement: Content-addressed identity model

r[kamacite.lossless_ir.content_addressed_identity] Kamacite MUST use content-addressed identity for canonical values, source envelopes, adapter descriptors, and receipts. Human names, paths, aliases, and labels MUST be treated as metadata rather than identity. Kamacite MUST use BLAKE3 over Kamacite-owned canonical bytes unless an interop surface explicitly requires another hash format.

#### Scenario: Names do not affect canonical identity

r[kamacite.lossless_ir.content_addressed_identity.names_metadata]

- GIVEN a Kamacite canonical value or source envelope has two human-readable names or paths for the same canonical bytes
- WHEN Kamacite computes identity
- THEN the stable identity MUST remain the same and the names or paths MUST be recorded only as metadata or aliases.

#### Scenario: Dependency closure is hash-bound

r[kamacite.lossless_ir.content_addressed_identity.dependency_closure]

- GIVEN a projection, adapter result, or constrained-policy round trip depends on other Kamacite artifacts
- WHEN Kamacite emits the receipt
- THEN the receipt MUST bind the dependency identities and adapter/evaluator identity before claiming the result is reusable.

#### Scenario: Effects are explicit capability labels

r[kamacite.lossless_ir.content_addressed_identity.effect_labels]

- GIVEN an adapter or evaluator requires host effects such as filesystem, process, network, randomness, time, credentials, or mutation
- WHEN Kamacite classifies the artifact
- THEN the receipt MUST carry explicit effect/capability labels and MUST fail closed if an undeclared effect is observed.

#### Scenario: Durable storage envelope validates before decode

r[kamacite.lossless_ir.content_addressed_identity.durable_storage]

- GIVEN a persisted Kamacite artifact is loaded from storage
- WHEN Kamacite prepares to decode it
- THEN validation MUST check schema, codec, content hash, and non-claim boundary before treating the decoded value as trusted input.

### Requirement: Maintained identity domains are nominal Rust types

r[kamacite.lossless_ir.nominal_identity] Kamacite MUST represent each statically known maintained identity domain with a distinct Rust type over private validated BLAKE3 content.

#### Scenario: Same-domain identity is accepted

r[kamacite.lossless_ir.nominal_identity.same_domain]
- GIVEN a canonical-value identity has valid BLAKE3 content
- WHEN a core API requires a canonical-value identity
- THEN the API MUST accept it without another runtime expected-domain argument.

#### Scenario: Cross-domain identity does not compile

r[kamacite.lossless_ir.nominal_identity_compile_time]
- GIVEN source-envelope and receipt identities use distinct marker-domain instantiations
- WHEN source passes a receipt identity to an API that requires a source-envelope identity
- THEN the source MUST fail compilation with a type mismatch.

#### Scenario: Malformed identity fails before core use

r[kamacite.lossless_ir.nominal_identity.validation]
- GIVEN untrusted identity text has an unsupported algorithm, wrong length, unsupported spelling, or non-hex content
- WHEN an adapter constructs a typed identity
- THEN construction MUST fail with deterministic diagnostics
- AND the invalid identity MUST NOT enter the typed core.

### Requirement: Wire identity domains remain explicit

r[kamacite.lossless_ir.nominal_identity_wire] Kamacite MUST preserve domain and algorithm fields in versioned interchange records and MUST admit them into a closed enum of domain-specific typed identities.

#### Scenario: Wire identity admits a known domain

r[kamacite.lossless_ir.nominal_identity_wire.known]
- GIVEN a supported wire domain, algorithm, and digest value
- WHEN identity admission runs
- THEN Kamacite MUST construct the matching typed identity variant.

#### Scenario: Wire identity has conflicting domain data

r[kamacite.lossless_ir.nominal_identity_wire.validation]
- GIVEN a wire identity has an unknown domain, unsupported algorithm, stale version, or conflicting artifact-role metadata
- WHEN identity admission runs
- THEN Kamacite MUST fail closed with a deterministic domain diagnostic.

### Requirement: Identity migration preserves canonical Preserves authority

r[kamacite.lossless_ir.nominal_identity_migration] Kamacite MUST migrate identity families through explicit adapters without changing canonical Preserves bytes or reinterpreting legacy identities.

#### Scenario: Canonical Preserves identity remains stable

r[kamacite.lossless_ir.nominal_identity.compatibility]
- GIVEN a supported artifact is encoded before and after the Rust nominal-type migration
- WHEN canonical Preserves bytes and BLAKE3 refs are compared
- THEN they MUST remain unchanged unless a separate versioned migration approves the new identity.

#### Scenario: Runtime domain checks move to admission

r[kamacite.lossless_ir.nominal_identity_migration.admission]
- GIVEN a heterogeneous manifest contains untrusted identity records
- WHEN the adapter admits each record
- THEN runtime domain checking MUST occur at admission
- AND known-role core APIs MUST receive exact typed identities.

### Requirement: Stack projections preserve typed identity roles

r[kamacite.lossless_ir.nominal_identity_stack] Kamacite Valence-friendly projections and stack adapters MUST preserve source, value, projection, target, receipt, effect, and compatibility identity roles without promoting evidence.

#### Scenario: Valence projection preserves roles

r[kamacite.lossless_ir.nominal_identity_stack.valence]
- GIVEN a Kamacite receipt contains several domain-specific identities
- WHEN Kamacite emits a Valence-friendly projection
- THEN each identity MUST retain its declared domain and evidence role
- AND the projection MUST NOT treat equal digest bytes as equal roles.

### Requirement: Nominal identity claims remain bounded

r[kamacite.lossless_ir.nominal_identity_docs] Kamacite documentation and receipts MUST state that nominal identity types prove local domain separation and checked identity syntax only.

#### Scenario: Boundary remains visible

r[kamacite.lossless_ir.nominal_identity_final_checks]
- GIVEN a typed identity passes construction and domain admission
- WHEN Kamacite states the supported claim
- THEN it MUST NOT claim source fidelity, adapter law satisfaction, semantic equivalence, authority, runtime correctness, or release eligibility.
