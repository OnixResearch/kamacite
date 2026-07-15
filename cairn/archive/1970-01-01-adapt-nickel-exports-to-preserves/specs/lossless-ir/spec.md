# Lossless IR Specification

## Purpose

Define the constrained value-preservation claim and canonical encoding rules for Nickel JSON export adaptation.

## Requirements

### Requirement: JSON-to-Preserves mapping is versioned, closed, and bounded

r[kamacite.nickel_preserves_adapter.mapping_profile] Kamacite MUST define a versioned `nickel-json-to-preserves-v1` profile that maps every admitted JSON null, boolean, string, number, array, and object case into one declared Preserves value shape. The profile MUST define object-key ordering, numeric classification, Unicode handling, and named depth, collection, string, numeric, input-byte, and output-byte limits. Inputs outside the admitted profile MUST fail deterministically rather than being coerced, truncated, or represented through debug text.

#### Scenario: Equivalent JSON values use one canonical value identity

- GIVEN two separately admitted JSON artifacts differ only in insignificant whitespace or object member order and decode to the same admitted JSON value
- WHEN each value is mapped and canonically encoded under the same profile
- THEN their JSON projection identities MAY differ while their canonical packed Preserves bytes and canonical-value identities MUST match.

#### Scenario: Mapping policy cannot represent a value safely

- GIVEN a decoded JSON value exceeds a named policy limit or has a numeric representation the selected profile does not admit unambiguously
- WHEN the pure mapping core processes it
- THEN it MUST return a deterministic profile diagnostic and MUST NOT emit canonical bytes or an identity.

### Requirement: Canonical output comes from one validated Preserves value

r[kamacite.nickel_preserves_adapter.canonical_output] The adapter MUST construct one typed Preserves value from the admitted JSON value, validate that value against the selected schema/profile, and derive canonical packed bytes and BLAKE3 identity from that same value through the codec established by `make-preserves-authority-real`. Caller-supplied canonical bytes or parallel DTO fields MUST NOT substitute for this construction.

#### Scenario: Detached output bytes are supplied

- GIVEN caller-supplied packed bytes encode a value different from the admitted JSON mapping
- WHEN Kamacite checks or writes the adaptation result
- THEN it MUST reject the detached bytes with canonical-content-mismatch evidence before emitting a success receipt.

### Requirement: Adaptation claim is limited to the exported JSON value

r[kamacite.nickel_preserves_adapter.non_claim] A passing adapter receipt MUST claim preservation only of the decoded JSON-compatible value selected from the admitted Nickel export under the named mapping and codec profiles. It MUST NOT claim arbitrary Nickel source losslessness, import-closure completeness beyond the Nickel export receipt, semantic equivalence, evaluator correctness, authorization, runtime behavior, or release eligibility.

#### Scenario: Caller requests arbitrary Nickel equivalence

- GIVEN canonical Preserves bytes were produced from a valid Nickel JSON export
- WHEN a caller labels the receipt as proof of arbitrary Nickel source or evaluator semantic equivalence
- THEN Kamacite MUST reject the claim or preserve it only as unsupported external metadata outside the trusted receipt.

### Requirement: Rendered text remains a non-authoritative projection

r[kamacite.nickel_preserves_adapter.text_projection] Kamacite MAY emit deterministic Preserves text for review, but the text artifact MUST be derived from the validated canonical value, MUST carry a distinct projection identity, and MUST NOT replace canonical packed bytes as artifact identity. This adapter MUST NOT introduce Markdown or AsciiDoc conversion behavior.

#### Scenario: Review text is requested

- GIVEN canonical packed Preserves output has been constructed successfully
- WHEN the operator requests a review-text projection
- THEN Kamacite MUST render it from the same validated value, bind both identities in the receipt, and identify the text as non-authoritative.
