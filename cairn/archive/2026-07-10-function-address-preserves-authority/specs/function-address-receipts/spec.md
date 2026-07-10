# Function Address Receipts Specification

## Purpose

Make Preserves canonical binary receipts the authoritative portable form for Kamacite function-address evidence.

## Requirements

### Requirement: Function-address receipts have a versioned Preserves schema
r[kamacite.function_address_receipts.preserves_authority.schema] Kamacite MUST define a versioned Preserves schema for function-address receipts covering producer metadata, source envelope identity, repo-relative paths, byte spans, function addresses, dependency identity kind, digest domains, and non-claims.

#### Scenario: Valid Octet handoff encodes as Preserves
r[kamacite.function_address_receipts.preserves_authority.positive]
- GIVEN Octet supplies a complete function-address handoff envelope
- WHEN Kamacite validates and encodes it
- THEN Kamacite MUST produce a schema-valid Preserves function-address receipt.

### Requirement: Canonical Preserves binary bytes own receipt identity
r[kamacite.function_address_receipts.preserves_authority.hash] Kamacite MUST compute function-address receipt identity as BLAKE3 over canonical Preserves binary bytes.

#### Scenario: Canonical hash is stable across transfer syntax
r[kamacite.function_address_receipts.preserves_authority.validation]
- GIVEN the same typed function-address receipt is represented in Preserves text and binary transfer syntaxes
- WHEN Kamacite canonicalizes the receipt
- THEN the canonical binary bytes and BLAKE3 receipt identity MUST be stable.

### Requirement: JSON projection is non-authoritative and bound
r[kamacite.function_address_receipts.preserves_authority.json_projection] Kamacite MAY emit JSON compatibility projections, but each projection MUST identify the canonical Preserves receipt hash and MUST NOT be treated as the lossless authority.

#### Scenario: Projection drift fails closed
r[kamacite.function_address_receipts.preserves_authority.negative]
- GIVEN a JSON projection disagrees with the canonical Preserves receipt or omits the canonical receipt hash
- WHEN Kamacite or downstream validators compare the projection to the canonical receipt
- THEN validation MUST fail with a deterministic projection-drift diagnostic.
