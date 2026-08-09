# Function Address Receipts Specification

## Purpose

Defines Kamacite's portable receipt contract for content-addressed function inventories produced by Octet or later source analyzers, preserving function identity evidence without claiming host-language semantic correctness.

## Requirements

### Requirement: Function-address receipt contract

r[kamacite.function_address_receipts.contract] Kamacite MUST define a `function-address-receipt` artifact class that records producer identity, canonicalizer identity, source envelopes, function records, dependency edges, caveats, artifact identities, receipt identity, and explicit non-claims.

#### Scenario: Valid receipt preserves function inventory
r[kamacite.function_address_receipts.contract.valid_inventory]
- GIVEN an Octet-produced function-address inventory with source envelopes, function entries, spans, BLAKE3 addresses, and caveats
- WHEN Kamacite validates the receipt
- THEN validation MUST preserve the function inventory and classify the receipt as identity evidence for the supplied producer facts.

#### Scenario: Receipt keeps domains distinct
r[kamacite.function_address_receipts.contract.identity_domains]
- GIVEN a receipt contains source-envelope identity, local function address, dependency-closure identity, projection identity, and receipt identity
- WHEN Kamacite validates or renders the receipt
- THEN each identity MUST remain in its declared domain and MUST NOT be substituted for another domain.

### Requirement: Canonical identity uses Kamacite bytes

r[kamacite.function_address_receipts.canonical_identity] Kamacite MUST compute function-address receipt identity from Kamacite-owned canonical bytes using BLAKE3. Preserves bytes SHOULD be the canonical identity input, while JSON output MUST remain a compatibility projection bound to the canonical receipt identity.

#### Scenario: Preserves identity is authoritative
r[kamacite.function_address_receipts.canonical_identity.preserves]
- GIVEN a function-address receipt has canonical Preserves bytes and a JSON projection
- WHEN Kamacite computes receipt identity
- THEN the canonical receipt identity MUST be the BLAKE3 digest of the canonical Kamacite bytes and the JSON projection MUST bind to that identity.

#### Scenario: Names and paths are metadata
r[kamacite.function_address_receipts.canonical_identity.names_metadata]
- GIVEN two receipt projections differ only by display names, path aliases, or labels that are declared metadata
- WHEN Kamacite computes canonical receipt identity
- THEN the identity MUST remain unchanged and the metadata differences MUST remain visible as metadata.

### Requirement: Receipt validation fails closed

r[kamacite.function_address_receipts.validation] Kamacite MUST fail closed when function-address receipts contain stale identities, malformed BLAKE3 digests, unknown identity domains, duplicate function locators, dependency edges pointing to absent functions, missing non-claims, or overclaim text.

#### Scenario: Stale source identity fails
r[kamacite.function_address_receipts.validation.stale_source]
- GIVEN a function record references a source-envelope identity that does not match the declared source envelope
- WHEN Kamacite validates the receipt
- THEN validation MUST fail with a stale-source-identity diagnostic.

#### Scenario: Duplicate function locator fails
r[kamacite.function_address_receipts.validation.duplicate_locator]
- GIVEN two function records declare the same producer-local locator within one receipt
- WHEN Kamacite validates the receipt
- THEN validation MUST fail with a duplicate-function-locator diagnostic.

#### Scenario: Overclaim fails
r[kamacite.function_address_receipts.validation.overclaim]
- GIVEN a receipt claims host-language semantic correctness, compiler correctness, runtime behavior, whole-program safety, verifier soundness, release eligibility, or arbitrary source equivalence
- WHEN Kamacite validates the receipt
- THEN validation MUST fail with an overclaim diagnostic.

### Requirement: Positive fixture coverage

r[kamacite.function_address_receipts.fixtures.positive] Kamacite MUST include positive fixtures for valid producer receipts, metadata-only rename/path churn, dependency-edge preservation, caveated unresolved dependencies, canonical identity stability, and Valence-friendly export projection.

#### Scenario: Positive fixtures pass
r[kamacite.function_address_receipts.fixtures.positive.suite]
- GIVEN valid function-address receipt fixtures and expected canonical identities
- WHEN the focused fixture suite runs
- THEN Kamacite MUST accept the fixtures and produce deterministic receipt identities and projections.

### Requirement: Negative fixture coverage

r[kamacite.function_address_receipts.fixtures.negative] Kamacite MUST include negative fixtures for stale source identities, malformed digest text, wrong identity domains, duplicate locators, missing dependency targets, missing non-claims, and overclaim text.

#### Scenario: Negative fixtures fail closed
r[kamacite.function_address_receipts.fixtures.negative.suite]
- GIVEN invalid function-address receipt fixtures cover the rejected classes
- WHEN the focused fixture suite runs
- THEN each fixture MUST fail with deterministic diagnostics naming the invalid field or identity domain.

### Requirement: Octet interop boundary

r[kamacite.function_address_receipts.octet_interop] Kamacite MUST treat Octet function-address evidence as producer-supplied Rust source-shape identity evidence. Kamacite MUST NOT treat Octet findings as proof of Kamacite IR correctness, Rust semantic correctness, compiler correctness, or arbitrary source equivalence.

#### Scenario: Octet evidence remains producer input
r[kamacite.function_address_receipts.octet_interop.producer_input]
- GIVEN a receipt identifies Octet as producer and names an Octet canonicalizer profile
- WHEN Kamacite validates the receipt
- THEN Kamacite MUST validate the receipt shape, identities, roles, and non-claims without claiming to rerun Octet or parse Rust semantics.

### Requirement: Valence export projection

r[kamacite.function_address_receipts.valence_export] Kamacite SHOULD provide an artifacts-first Valence-friendly projection for validated function-address receipts. The projection MUST bind the source Kamacite receipt identity, artifact identities, source spans when present, producer roles, BLAKE3 identities, verification role, and non-claims.

#### Scenario: Projection preserves source receipt identity
r[kamacite.function_address_receipts.valence_export.identity]
- GIVEN a valid function-address receipt
- WHEN Kamacite renders the Valence-friendly projection
- THEN the projection MUST bind the source receipt identity and MUST use a projection identity domain rather than replacing the source receipt identity.

#### Scenario: Projection does not broaden claims
r[kamacite.function_address_receipts.valence_export.non_claims]
- GIVEN a function-address receipt is projected for Valence consumption
- WHEN the projection is inspected
- THEN every non-claim MUST survive and the projection MUST NOT claim behavioral correctness, verifier soundness, or release eligibility.

### Requirement: Documentation and final validation

r[kamacite.function_address_receipts.docs] Kamacite documentation MUST describe function-address receipts as portable identity/interchange evidence with explicit Octet, Valence, Mantle, and Cairn ownership boundaries. r[kamacite.function_address_receipts.final_validation] The change MUST include positive and negative fixtures plus focused validation before archive.

#### Scenario: Boundary is visible
r[kamacite.function_address_receipts.docs.boundary]
- GIVEN an operator reads Kamacite function-address receipt documentation
- WHEN the supported claim is described
- THEN the documentation MUST state that Kamacite preserves receipt identity and interchange facts only, while producers own source analysis and downstream systems own evidence linkage or release gates.

#### Scenario: Validation covers pass and fail cases
r[kamacite.function_address_receipts.final_validation.reviewable]
- GIVEN the implementation is complete
- WHEN Kamacite validation, focused receipt tests, export checks, and lifecycle gates run
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and saved evidence MUST identify the checked receipt identities.
