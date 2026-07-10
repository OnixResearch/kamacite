# Function Address Receipts Specification

## Purpose

Clarify dependency identity vocabulary for portable function-address receipts.

## Requirements

### Requirement: Dependency identity kind is preserved
r[kamacite.function_address_receipts.dependency_vocabulary.kind] Kamacite MUST preserve an explicit dependency identity kind for each function-address dependency identity supplied by a producer.

#### Scenario: Octet token-set dependency is represented
r[kamacite.function_address_receipts.dependency_vocabulary.positive]
- GIVEN Octet supplies a source-token dependency-set hash for a function-address record
- WHEN Kamacite builds the portable receipt
- THEN the receipt MUST preserve that identity as source-token/source-shape dependency evidence, not semantic dependency closure.

### Requirement: Existing dependency fields remain bounded
r[kamacite.function_address_receipts.dependency_vocabulary.compat] Kamacite MAY continue reading existing `dependency_closure` fields as compatibility inputs, but MUST NOT treat them as stronger than their declared dependency identity kind.

#### Scenario: Compatibility field does not overclaim
r[kamacite.function_address_receipts.dependency_vocabulary.validation]
- GIVEN a v1 receipt has an existing dependency identity field and an explicit token-set kind
- WHEN validation runs
- THEN Kamacite MUST preserve the field while reporting only the bounded token-set claim.

### Requirement: Closure overclaims fail closed
r[kamacite.function_address_receipts.dependency_vocabulary.negative] Kamacite MUST reject function-address receipts that claim token dependency identity proves Rust semantic dependency closure, compiler-resolved call graphs, whole-program safety, or release eligibility.

#### Scenario: Projection preserves dependency kind
r[kamacite.function_address_receipts.dependency_vocabulary.projection]
- GIVEN a valid receipt includes dependency identity kind metadata
- WHEN Kamacite exports a Valence projection
- THEN the projection MUST carry enough dependency-kind metadata for Valence to avoid inferring semantic closure from a hash alone.
