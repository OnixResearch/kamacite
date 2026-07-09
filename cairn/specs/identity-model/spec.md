# Identity Model Specification

## Purpose

Defines Taenite's content-addressed identity model, inspired by Unison discipline but owned by Taenite's canonical bytes and BLAKE3 hashes.

## Requirements

### Requirement: Content-addressed identity model

r[taenite.lossless_ir.content_addressed_identity] Taenite MUST use content-addressed identity for canonical values, source envelopes, adapter descriptors, and receipts. Human names, paths, aliases, and labels MUST be treated as metadata rather than identity. Taenite MUST use BLAKE3 over Taenite-owned canonical bytes unless an interop surface explicitly requires another hash format.

#### Scenario: Names do not affect canonical identity

r[taenite.lossless_ir.content_addressed_identity.names_metadata]

- GIVEN a Taenite canonical value or source envelope has two human-readable names or paths for the same canonical bytes
- WHEN Taenite computes identity
- THEN the stable identity MUST remain the same and the names or paths MUST be recorded only as metadata or aliases.

#### Scenario: Dependency closure is hash-bound

r[taenite.lossless_ir.content_addressed_identity.dependency_closure]

- GIVEN a projection, adapter result, or constrained-policy round trip depends on other Taenite artifacts
- WHEN Taenite emits the receipt
- THEN the receipt MUST bind the dependency identities and adapter/evaluator identity before claiming the result is reusable.

#### Scenario: Effects are explicit capability labels

r[taenite.lossless_ir.content_addressed_identity.effect_labels]

- GIVEN an adapter or evaluator requires host effects such as filesystem, process, network, randomness, time, credentials, or mutation
- WHEN Taenite classifies the artifact
- THEN the receipt MUST carry explicit effect/capability labels and MUST fail closed if an undeclared effect is observed.

#### Scenario: Durable storage envelope validates before decode

r[taenite.lossless_ir.content_addressed_identity.durable_storage]

- GIVEN a persisted Taenite artifact is loaded from storage
- WHEN Taenite prepares to decode it
- THEN validation MUST check schema, codec, content hash, and non-claim boundary before treating the decoded value as trusted input.
