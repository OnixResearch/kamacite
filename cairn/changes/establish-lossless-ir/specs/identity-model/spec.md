# Identity Model Specification

## Purpose

Defines Braid's content-addressed identity model, inspired by Unison discipline but owned by Braid's canonical bytes and BLAKE3 hashes.

## Requirements

### Requirement: Content-addressed identity model

r[braid.lossless_ir.content_addressed_identity] Braid MUST use content-addressed identity for canonical values, source envelopes, adapter descriptors, and receipts. Human names, paths, aliases, and labels MUST be treated as metadata rather than identity. Braid MUST use BLAKE3 over Braid-owned canonical bytes unless an interop surface explicitly requires another hash format.

#### Scenario: Names do not affect canonical identity

r[braid.lossless_ir.content_addressed_identity.names_metadata]

- GIVEN a Braid canonical value or source envelope has two human-readable names or paths for the same canonical bytes
- WHEN Braid computes identity
- THEN the stable identity MUST remain the same and the names or paths MUST be recorded only as metadata or aliases.

#### Scenario: Dependency closure is hash-bound

r[braid.lossless_ir.content_addressed_identity.dependency_closure]

- GIVEN a projection, adapter result, or constrained-policy round trip depends on other Braid artifacts
- WHEN Braid emits the receipt
- THEN the receipt MUST bind the dependency identities and adapter/evaluator identity before claiming the result is reusable.

#### Scenario: Effects are explicit capability labels

r[braid.lossless_ir.content_addressed_identity.effect_labels]

- GIVEN an adapter or evaluator requires host effects such as filesystem, process, network, randomness, time, credentials, or mutation
- WHEN Braid classifies the artifact
- THEN the receipt MUST carry explicit effect/capability labels and MUST fail closed if an undeclared effect is observed.

#### Scenario: Durable storage envelope validates before decode

r[braid.lossless_ir.content_addressed_identity.durable_storage]

- GIVEN a persisted Braid artifact is loaded from storage
- WHEN Braid prepares to decode it
- THEN validation MUST check schema, codec, content hash, and non-claim boundary before treating the decoded value as trusted input.
