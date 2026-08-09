# Reference IR Boundary Specification

## Purpose

Add a tMIR-inspired dialect roundtrip reference profile while keeping Preserves as Kamacite's canonical authority.

## Requirements

### Requirement: Dialect roundtrip reference profile
r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.profile] Kamacite MUST define a dialect-roundtrip reference profile for opaque adapter operation vocabularies with namespace, operation name, payload version, canonical payload hash, source envelope ref, source spans, lowering receipt refs, compatibility projection refs, and non-claim boundaries.

#### Scenario: Known dialect payload roundtrips
r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.positive]
- GIVEN a fixture contains a known adapter operation payload encoded as canonical Kamacite Preserves bytes
- WHEN Kamacite validates the dialect-roundtrip profile
- THEN validation MUST preserve namespace, operation name, payload version, payload hash, source envelope identity, source spans, lowering receipt refs, and non-claims.

### Requirement: Preserves remains canonical
r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.canonical_preserves] Kamacite MUST use Preserves canonical bytes as the authoritative identity for dialect-roundtrip payloads and MUST treat JSON as a compatibility projection only.

#### Scenario: Lossy JSON projection is rejected
r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.negative]
- GIVEN a dialect-roundtrip fixture only preserves a JSON projection or changes canonical payload identity during JSON export/import
- WHEN validation runs
- THEN validation MUST fail with deterministic diagnostics and MUST NOT treat the JSON projection as canonical.

### Requirement: Unknown dialects remain opaque
r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.opaque_unknown] Kamacite MAY preserve unknown dialect identities and payload hashes without interpreting operation semantics.

#### Scenario: Unknown dialect is preserved but not interpreted
- GIVEN a fixture contains an unknown dialect namespace with a supported payload envelope and valid hash
- WHEN Kamacite validates the fixture
- THEN validation MAY accept the opaque identity as roundtrip evidence and MUST NOT infer lowering semantics, proof obligations, or runtime behavior.

### Requirement: Dialect receipts keep non-claim boundaries
r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.valence_export] Kamacite SHOULD export dialect-roundtrip receipts to Valence only as bounded source-envelope, payload-identity, and adapter-linkage evidence.

#### Scenario: Export preserves bounded role
- GIVEN a dialect-roundtrip receipt validates locally
- WHEN Kamacite exports a Valence-friendly projection
- THEN the projection SHOULD preserve recorded-only role labels and MUST NOT claim source-language semantic equivalence, proof discharge, compiler correctness, verifier soundness, or runtime behavior.

### Requirement: Dialect roundtrip validation is fixture-backed
r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.validation] Kamacite MUST include positive and negative fixtures before treating dialect-roundtrip behavior as implemented.

#### Scenario: Fixture suite covers failure modes
- GIVEN dialect-roundtrip validation is run
- WHEN fixtures include stale payload hash, unsupported payload version, missing source envelope, lossy JSON projection, cross-dialect payload substitution, and semantic overclaim cases
- THEN each invalid fixture MUST fail with deterministic diagnostics.