# Quantity-Preserving Envelopes

## ADDED Requirements

### Requirement: Quantity declarations are canonical envelope data

r[kamacite.qpe.declaration] A quantity-preserving envelope MUST record the declared quantity specification as canonical data. The specification MUST bind named meaning, dimension, unit, and optional character. The specification MUST participate in the envelope BLAKE3 identity.

#### Scenario: Declared specification binds the identity

r[kamacite.qpe.declaration.identity]
- GIVEN two envelopes carry identical raw values but different quantity specifications
- WHEN identity is computed
- THEN the envelopes MUST have distinct BLAKE3 identities

#### Scenario: Missing specification is explicit

r[kamacite.qpe.declaration.untyped]
- GIVEN an envelope carries no quantity specification
- WHEN validation runs
- THEN the envelope MUST declare the absence explicitly
- AND adapters MUST NOT invent a specification

### Requirement: Meaning, dimension, unit, and representation stay separate

r[kamacite.qpe.separation] The envelope MUST keep named meaning, physical dimension, unit, and byte representation as separate fields. An adapter MUST NOT treat equal dimensions or equal representations as equal meaning.

#### Scenario: Equal dimension rejects meaning swap

r[kamacite.qpe.separation.swap]
- GIVEN two values share one dimension but answer different domain questions
- WHEN an adapter substitutes one for the other
- THEN the substitution MUST fail validation unless an explicit interpretation records the meaning

### Requirement: Character is bounded and explicit

r[kamacite.qpe.character] A quantity specification MAY declare one character: point, absolute, or difference. The character MUST NOT default silently. Non-negativity MUST be an explicit mark and MUST NOT be derived from a defining equation.

#### Scenario: Character is declared, not defaulted

r[kamacite.qpe.character.declared]
- GIVEN an envelope with no declared character
- WHEN validation runs
- THEN the absence MUST be visible
- AND the envelope MUST NOT behave as an implied signed difference

#### Scenario: Point and point do not add without permission

r[kamacite.qpe.character.point]
- GIVEN two point values from the same space
- WHEN the arithmetic boundary evaluates their sum
- THEN the boundary MUST refuse the sum unless the declared operation permits it

### Requirement: Meaning provenance is recorded

r[kamacite.qpe.provenance] The envelope MUST record how meaning was bound: declared construction, auditable assumption, or derived interpretation. The envelope MUST NOT silently promote an assumed meaning into a derived meaning.

#### Scenario: Assumption boundary is auditable

r[kamacite.qpe.provenance.assume]
- GIVEN an adapter asserts a meaning over raw bytes
- WHEN the envelope is constructed
- THEN the envelope MUST mark the assertion as an auditable assumption
- AND a later claim of derivation MUST fail unless an explicit interpretation exists

### Requirement: Conversion is explicit at boundaries

r[kamacite.qpe.conversion] Unit and frame conversion MUST be explicit adapter operations. The envelope MUST record the conversion identity and MUST NOT silently change unit, character, or named meaning. Silent inversion MUST be refused.

#### Scenario: Explicit conversion records identity

r[kamacite.qpe.conversion.explicit]
- GIVEN an envelope crosses a unit or frame boundary
- WHEN the conversion runs
- THEN the result MUST bind the conversion identity
- AND the named meaning MUST remain unchanged

#### Scenario: Silent inversion fails

r[kamacite.qpe.conversion.inversion]
- GIVEN an adapter changes the frame without recording a projection
- WHEN validation runs
- THEN validation MUST refuse the change

### Requirement: Cross-language preservation is a bounded claim

r[kamacite.qpe.cross_language] Nickel, Rust, and Steel adapters MUST preserve the declared quantity specification and raw value exactly across the interchange. The receipt MUST bind every adapter, evaluator, and target artifact identity. The claim MUST NOT extend to arbitrary semantic equivalence.

#### Scenario: Pinned adapters preserve the projection

r[kamacite.qpe.cross_language.pinned]
- GIVEN a pinned Nickel source envelope, a Rust adapter, and a Steel probe
- WHEN the interchange executes
- THEN the target artifact MUST preserve the declared specification and value
- AND the receipt MUST bind all three identities

#### Scenario: Adapter drift fails

r[kamacite.qpe.cross_language.drift]
- GIVEN an adapter changes the specification or value across the interchange
- WHEN validation runs
- THEN validation MUST fail and name the drifting adapter

### Requirement: Fixture coverage

r[kamacite.qpe.verification] Positive, negative, and boundary fixtures MUST cover declaration, separation, character, provenance, conversion, cross-language preservation, and overclaims.

#### Scenario: Focused quantity matrix passes

r[kamacite.qpe.verification.matrix]
- GIVEN envelopes, fixtures, adapter evidence, and documentation are complete
- WHEN the focused rail runs
- THEN in-bounds inputs MUST behave as declared
- AND every meaning-swap, silent-default, silent-conversion, promoted-assumption, inverted-frame, or overclaim input MUST fail as declared
