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
