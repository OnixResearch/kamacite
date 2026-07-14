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

### Requirement: Receipt validation fails closed

r[kamacite.function_address_receipts.validation] Kamacite MUST fail closed when function-address receipts contain stale identities, malformed BLAKE3 digests, unknown identity domains, duplicate function locators, dependency edges pointing to absent functions, missing non-claims, invalid source spans, projection drift, or overclaim text.

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

### Requirement: CLI emits portable function-address receipts

r[kamacite.function_address_receipts.cli_adapter.command] Kamacite MUST provide a CLI command that ingests an Octet function-address report and emits a validated Kamacite function-address receipt.

#### Scenario: Valid Octet report becomes Kamacite receipt
r[kamacite.function_address_receipts.cli_adapter.positive]
- GIVEN an Octet function-address report has supported producer metadata, source envelopes, function records, BLAKE3 identities, and required non-claims
- WHEN the Kamacite CLI validates it with matching source envelope bytes
- THEN the CLI MUST emit a valid Kamacite function-address receipt and receipt identity.

### Requirement: CLI shell owns I/O

r[kamacite.function_address_receipts.cli_adapter.shell] Kamacite CLI filesystem reads, path resolution, stdout/stderr, and file writes MUST remain outside `kamacite-core` receipt validation.

#### Scenario: Core receives in-memory input
r[kamacite.function_address_receipts.cli_adapter.validation]
- GIVEN the CLI has loaded and parsed an Octet function-address report
- WHEN it invokes receipt validation
- THEN `kamacite-core` MUST receive in-memory typed input without reading files or environment state.

### Requirement: Valence projection output is explicit

r[kamacite.function_address_receipts.cli_adapter.valence_projection] Kamacite SHOULD provide an explicit CLI output for the function-address Valence projection derived from a valid Kamacite receipt.

#### Scenario: Projection is emitted with receipt identity
r[kamacite.function_address_receipts.cli_adapter.negative]
- GIVEN a valid Kamacite function-address receipt is emitted
- WHEN the operator requests Valence projection output
- THEN the projection MUST bind the source Kamacite receipt identity and MUST fail closed if receipt validation failed.

### Requirement: Documentation states ownership boundaries

r[kamacite.function_address_receipts.docs] Kamacite documentation MUST describe function-address receipts as portable identity/interchange evidence with explicit Octet, Valence, Mantle, and Cairn ownership boundaries.

#### Scenario: Boundary is visible
r[kamacite.function_address_receipts.docs.boundary]
- GIVEN an operator reads Kamacite function-address receipt documentation
- WHEN the supported claim is described
- THEN the documentation MUST state that Kamacite preserves receipt identity and interchange facts only, while producers own source analysis and downstream systems own evidence linkage or release gates.

### Requirement: Final validation evidence is reviewable

r[kamacite.function_address_receipts.final_validation] The change MUST include positive and negative fixtures plus focused validation before archive.

#### Scenario: Validation covers pass and fail cases
r[kamacite.function_address_receipts.final_validation.reviewable]
- GIVEN the implementation is complete
- WHEN Kamacite validation, focused receipt tests, export checks, and lifecycle gates run
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and saved evidence MUST identify the checked receipt identities.

### Requirement: Producer profile is content-addressed

r[kamacite.findings.function_address_producer.profile_identity] Function-address receipts MUST use a content-addressed producer profile covering producer kind, tool version/build identity, canonicalizer profile/version, normative options, and producer non-claims.

#### Scenario: Producer tool version changes

- GIVEN two otherwise identical function inventories were produced by different tool versions
- WHEN Kamacite constructs producer profiles
- THEN the producer profile identities MUST differ.

### Requirement: Receipt identity binds producer profile

r[kamacite.findings.function_address_producer.receipt_binding] Canonical function-address receipt bytes MUST include the producer profile identity. Every normative producer field displayed by a compatibility projection MUST resolve to that bound profile.

#### Scenario: JSON displays an unbound version

- GIVEN a JSON receipt projection changes the displayed tool version while retaining the old producer profile and receipt identities
- WHEN projection validation runs
- THEN validation MUST fail with deterministic producer-profile drift evidence.

### Requirement: Display metadata remains non-normative

r[kamacite.findings.function_address_producer.metadata_boundary] Path aliases, display names, and labels MAY remain outside canonical identity only when source, function, dependency, producer-profile, and receipt identities are unchanged and the fields are explicitly classified as metadata.

#### Scenario: Display-only rename occurs

- GIVEN two receipts differ only in admitted display names or path aliases
- WHEN canonical receipt identities are computed
- THEN the identities MUST remain equal while both projections retain their visible metadata.

### Requirement: Legacy unbound versions are caveated

r[kamacite.findings.function_address_producer.migration] Kamacite MUST version the producer-bound receipt profile and MUST NOT claim legacy receipt identities bind tool version. Legacy inputs MAY remain recorded evidence only with an explicit tool-version-unbound caveat.

#### Scenario: Legacy v1 receipt is loaded

- GIVEN a v1 receipt omits producer profile identity from canonical hash material
- WHEN a producer-bound validator reads it
- THEN the validator MUST reject current-profile trust or preserve it only under the legacy caveat without rewriting its identity.
