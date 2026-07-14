# Lossless IR Specification

## Purpose

Close the gap between Kamacite's claimed Preserves authority and the bytes actually decoded, validated, encoded, and hashed.

## Requirements

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
