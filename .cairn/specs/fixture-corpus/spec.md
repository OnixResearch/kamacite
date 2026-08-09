# Fixture Corpus Specification

## Purpose

Defines the Kamacite fixture corpus requirements for proving bounded losslessness, adapter conformance, fail-closed behavior, and deterministic receipts.

## Requirements

### Requirement: Fixture corpus layout

r[kamacite.lossless_ir.fixture_corpus] Kamacite MUST maintain a deterministic fixture corpus for every implemented schema, adapter, projection, target artifact, validation rule, and receipt kind. Fixtures MUST be classified as positive, negative, or property-style, and each fixture MUST name the requirement IDs, artifact class, adapter descriptor when applicable, input identities, expected diagnostics, and expected receipt claim.

#### Scenario: Fixture declares requirement coverage

r[kamacite.lossless_ir.fixture_corpus.requirement_coverage]

- GIVEN a fixture is added to the corpus
- WHEN Kamacite validates the fixture metadata
- THEN the fixture MUST name the requirement IDs it covers and MUST state whether it is positive, negative, or property-style evidence.

#### Scenario: Fixture discovery is deterministic

r[kamacite.lossless_ir.fixture_corpus.deterministic_discovery]

- GIVEN multiple fixtures exist in the corpus
- WHEN Kamacite discovers them for validation
- THEN discovery order MUST be deterministic and MUST NOT depend on filesystem iteration order, locale, wall-clock time, random values, network state, or environment variables.

### Requirement: Positive fixture evidence

r[kamacite.lossless_ir.positive_fixtures] Positive fixtures MUST prove that supported source envelopes, canonical values, constrained policies, adapter descriptors, target artifacts, and receipts preserve the declared identities and checked laws.

#### Scenario: Positive round-trip fixture preserves identity

r[kamacite.lossless_ir.positive_fixtures.round_trip_identity]

- GIVEN a valid canonical value or constrained policy fixture declares a round-trip adapter law
- WHEN Kamacite runs the fixture
- THEN the resulting canonical identity MUST match the expected identity and the receipt MUST name the checked law.

#### Scenario: Positive source-envelope fixture preserves source bytes

r[kamacite.lossless_ir.positive_fixtures.source_bytes]

- GIVEN a valid source-envelope fixture includes source bytes, language label, parser identity, and BLAKE3 source hash
- WHEN Kamacite validates the fixture
- THEN the source hash MUST match the bytes and all projections MUST remain derived evidence.

### Requirement: Negative fixture evidence

r[kamacite.lossless_ir.negative_fixtures] Negative fixtures MUST prove that malformed, stale, lossy, unsupported, ambiguous, unauthorized, or cross-domain artifacts fail closed with deterministic diagnostics. Every implemented validation behavior MUST include at least one negative fixture for its primary failure mode.

#### Scenario: Negative stale hash fixture fails closed

r[kamacite.lossless_ir.negative_fixtures.stale_hash]

- GIVEN a fixture declares a source, canonical, projection, target, adapter, compiler, evaluator, or receipt hash that does not match its canonical bytes
- WHEN Kamacite validates the fixture
- THEN validation MUST fail closed with a deterministic hash-domain diagnostic.

#### Scenario: Negative lossy projection fixture fails closed

r[kamacite.lossless_ir.negative_fixtures.lossy_projection]

- GIVEN a JSON or other compatibility projection drops distinctions required by the canonical Preserves value
- WHEN Kamacite validates the projection claim
- THEN validation MUST reject any claim that the projection is authoritative and MUST report the lossy projection as derived, invalid, or incomplete.

#### Scenario: Negative undeclared effect fixture fails closed

r[kamacite.lossless_ir.negative_fixtures.undeclared_effect]

- GIVEN an adapter fixture observes filesystem, process, network, clock, random, credential, mutation, compiler, evaluator, or package-manager effects not declared in its descriptor
- WHEN Kamacite validates the fixture receipt
- THEN validation MUST fail closed with an undeclared-effect diagnostic.

#### Scenario: Negative unsupported translation fixture fails closed

r[kamacite.lossless_ir.negative_fixtures.unsupported_translation]

- GIVEN a fixture claims arbitrary Nickel, Rust, Steel, Haskell, LLVM, WebAssembly, GHC, Rust MIR, or other source/target translation is lossless without a constrained Kamacite schema
- WHEN Kamacite classifies the claim
- THEN validation MUST reject the claim and MAY preserve the source or target as a bounded envelope with explicit non-claims.

### Requirement: Property-style fixture evidence

r[kamacite.lossless_ir.property_fixtures] Kamacite MAY include generated or enumerated property-style fixtures, but each property-style fixture MUST record generator identity, seed or enumeration identity when applicable, input domain, shrink or minimization result when applicable, and deterministic expected diagnostics or receipts.

#### Scenario: Property fixture is replayable

r[kamacite.lossless_ir.property_fixtures.replayable]

- GIVEN a property-style fixture discovers or records a passing or failing case
- WHEN Kamacite stores the fixture evidence
- THEN the stored metadata MUST be sufficient to replay the exact case deterministically without depending on hidden randomness or ambient host state.

#### Scenario: Property failure becomes negative fixture

r[kamacite.lossless_ir.property_fixtures.failure_promotion]

- GIVEN property-style validation finds a minimal failing case for a supported rule
- WHEN the failure is accepted into the corpus
- THEN Kamacite SHOULD promote it to a stable negative fixture with explicit expected diagnostics.

### Requirement: Golden receipt evidence

r[kamacite.lossless_ir.golden_receipts] Kamacite fixture validation MUST produce deterministic golden receipts for accepted fixture outcomes. Golden receipts MUST include input identities, expected output identities, diagnostics, checked laws, non-claims, and receipt identity. Updating golden receipts MUST be a deliberate fixture change rather than an incidental formatter or tool ordering change.

#### Scenario: Golden receipt mismatch is explicit

r[kamacite.lossless_ir.golden_receipts.mismatch]

- GIVEN a fixture has a stored golden receipt
- WHEN the validation command emits a different receipt
- THEN validation MUST fail with a receipt-mismatch diagnostic unless the operator explicitly runs an accepted update workflow.
