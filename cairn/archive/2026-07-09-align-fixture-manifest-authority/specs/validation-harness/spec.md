# Validation Harness Specification

## Purpose

Defines the executable Kamacite validation harness authority boundary between reviewed Nickel source manifests and runtime JSON projections.

## Requirements

### Requirement: Fixture manifest full corpus authority

r[kamacite.validation_harness.fixture_manifest.full_corpus_authority] Kamacite MUST keep the complete trusted fixture corpus in the human-reviewed Nickel manifest source. The runtime JSON manifest MUST remain a compatibility projection and MUST NOT be the only file that names trusted fixture IDs, fixture classes, artifact classes, expected verdicts, golden receipts, requirement coverage, non-claims, and payload descriptors.

#### Scenario: Nickel source carries every fixture

r[kamacite.validation_harness.fixture_manifest.full_corpus_authority.complete_source]

- GIVEN a fixture is trusted by the runtime validation harness
- WHEN reviewers inspect the manifest authority
- THEN the Nickel source manifest MUST name that fixture ID and its trust metadata rather than requiring reviewers to treat the JSON projection as the only source.

### Requirement: Fixture manifest projection identity

r[kamacite.validation_harness.fixture_manifest.projection_identity] The Nickel fixture manifest MUST bind the runtime projection with a BLAKE3 projection identity. The projection identity MUST be computed over the exact runtime projection bytes that the shell consumes, and normal validation MUST NOT rewrite that projection identity implicitly.

#### Scenario: Source declares projection identity

r[kamacite.validation_harness.fixture_manifest.projection_identity.source_declared]

- GIVEN the CLI is configured with a source manifest and runtime projection
- WHEN it loads the projection for validation
- THEN it MUST compute the projection BLAKE3 identity and compare it with the identity declared by the source manifest before trusting fixture results.

### Requirement: Fixture manifest projection freshness

r[kamacite.validation_harness.fixture_manifest.projection_freshness] Default harness validation MUST fail closed when the runtime JSON projection is stale relative to the source-declared projection identity. The failure MUST be deterministic and MUST NOT mutate the Nickel source, runtime projection, or golden receipts.

#### Scenario: Matching projection is accepted

r[kamacite.validation_harness.fixture_manifest.projection_freshness.matching_projection]

- GIVEN the Nickel source declares the BLAKE3 identity of the current runtime JSON projection
- WHEN default validation loads the manifest
- THEN validation MUST continue to fixture execution and receipt comparison.

#### Scenario: Stale projection is rejected

r[kamacite.validation_harness.fixture_manifest.projection_freshness.stale_projection]

- GIVEN the runtime JSON projection bytes differ from the identity declared by the Nickel source
- WHEN default validation loads the manifest
- THEN validation MUST fail closed with a deterministic projection-stale diagnostic and MUST NOT rewrite evidence.

### Requirement: Fixture manifest ID alignment

r[kamacite.validation_harness.fixture_manifest.fixture_id_alignment] The validation shell MUST compare source-declared fixture IDs with runtime projection fixture IDs before trusting the projection. A missing, duplicate, reordered, or extra projection fixture ID MUST be reported deterministically before fixture receipts are trusted.

#### Scenario: Fixture ID drift is rejected

r[kamacite.validation_harness.fixture_manifest.fixture_id_alignment.drift]

- GIVEN the Nickel source manifest and runtime JSON projection name different fixture IDs or the same fixture IDs in different order
- WHEN default validation loads the projection
- THEN validation MUST fail closed with deterministic fixture-ID drift evidence.
