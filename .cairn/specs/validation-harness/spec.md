# Validation Harness Specification

## Purpose

Defines the first executable Kamacite validation harness: a walking-skeleton fixture and receipt runner that must exist before trusted implementation behavior is admitted.

## Requirements

### Requirement: Walking-skeleton validation harness

r[kamacite.validation_harness.walking_skeleton] Kamacite MUST establish a walking-skeleton validation harness before the first trusted implementation behavior is accepted. The harness MUST exercise at least one valid source-envelope fixture, one stale-hash rejection fixture, one wrong-identity-domain rejection fixture, and one deterministic golden receipt comparison.

#### Scenario: Harness exists before implementation admission

r[kamacite.validation_harness.walking_skeleton.precedes_admission]

- GIVEN a new Kamacite validator or receipt constructor is proposed for trusted claim admission
- WHEN the walking-skeleton harness has not passed for source identity, stale hash rejection, wrong identity-domain rejection, and golden receipt comparison
- THEN the behavior MUST remain untrusted for Kamacite claim admission.

#### Scenario: Walking skeleton remains minimal

r[kamacite.validation_harness.walking_skeleton.minimal_scope]

- GIVEN the walking-skeleton harness is implemented
- WHEN Kamacite evaluates its initial scope
- THEN it SHOULD include only the minimal source-envelope, identity-domain, hash, diagnostic, and receipt cases needed to prove the harness contract before broader adapter suites are added.

### Requirement: Fixture layout boundary

r[kamacite.validation_harness.fixture_layout] Kamacite MUST use a deterministic fixture layout that separates pure core validation fixtures from adapter, evaluator, compiler, target, export, or probe fixtures. Fixture classes MUST include positive, negative, property-style, and golden-receipt evidence where those classes apply.

#### Scenario: Pure fixtures do not require host authority

r[kamacite.validation_harness.fixture_layout.pure_without_host_authority]

- GIVEN a fixture is classified as a pure core validation fixture
- WHEN the harness runs the fixture
- THEN the result MUST depend only on explicit fixture bytes and manifest metadata and MUST NOT require filesystem traversal inside the core, process execution, evaluator calls, network access, clocks, random sources, credentials, or mutation.

#### Scenario: Adapter fixtures are isolated

r[kamacite.validation_harness.fixture_layout.adapter_isolation]

- GIVEN a future fixture requires Nickel, Steel, WASM, compiler, Valence, Octet, algebraic-effect runtime, or other adapter/probe behavior
- WHEN the fixture is added to the corpus
- THEN it MUST be placed in an adapter/probe class or equivalent manifest section and MUST NOT become part of the pure core gate unless represented only as explicit in-memory evidence or non-claim data.

### Requirement: Nickel fixture manifest

r[kamacite.validation_harness.fixture_manifest] Kamacite MUST keep the human-authored fixture manifest in Nickel with a typed contract. The manifest MUST declare fixture IDs, artifact classes, fixture classes, input artifacts, expected verdicts, expected diagnostics, expected receipt identities or golden receipt paths, non-claims, dependency identities, and covered requirement IDs.

#### Scenario: Manifest rejects incomplete fixture metadata

r[kamacite.validation_harness.fixture_manifest.incomplete_metadata]

- GIVEN a fixture manifest entry omits a required fixture ID, artifact class, expected verdict, covered requirement ID, or non-claim field
- WHEN the manifest is checked or exported for harness use
- THEN the manifest MUST fail closed before the fixture can be accepted as evidence.

#### Scenario: Duplicate fixture identity fails closed

r[kamacite.validation_harness.fixture_manifest.duplicate_identity]

- GIVEN two manifest entries declare the same fixture identity
- WHEN the harness loads the manifest projection
- THEN validation MUST fail closed with a deterministic duplicate-fixture diagnostic before trusting either fixture result.

#### Scenario: Manifest source remains reviewed authority

r[kamacite.validation_harness.fixture_manifest.nickel_authority]

- GIVEN runtime tests consume a generated manifest projection
- WHEN Kamacite records harness evidence
- THEN the evidence MUST bind the Nickel manifest source identity or a freshness-checked projection identity and MUST NOT treat an unchecked JSON projection as the human-authored authority.

### Requirement: Fixture manifest full corpus authority

r[kamacite.validation_harness.fixture_manifest.full_corpus_authority] Kamacite MUST keep the complete trusted fixture corpus in the human-reviewed Nickel manifest source. The runtime JSON manifest MUST remain a compatibility projection and MUST NOT be the only file that names trusted fixture IDs, fixture classes, artifact classes, expected verdicts, golden receipts, requirement coverage, non-claims, and payload descriptors.

#### Scenario: Nickel source carries every fixture

r[kamacite.validation_harness.fixture_manifest.full_corpus_authority.complete_source]

- GIVEN a fixture is trusted by the runtime validation harness
- WHEN reviewers inspect the manifest authority
- THEN the Nickel source manifest MUST name that fixture ID and its trust metadata rather than requiring reviewers to treat the JSON projection as the only source.

### Requirement: Fixture manifest projection identity

r[kamacite.validation_harness.fixture_manifest.projection_identity] Kamacite MUST bind the runtime projection with a BLAKE3 projection identity in checked generation evidence. The projection identity MUST be computed over the exact runtime projection bytes that the shell consumes, and normal validation MUST NOT rewrite the source, projection, or generation receipt implicitly.

#### Scenario: Generation receipt binds projection identity

r[kamacite.validation_harness.fixture_manifest.projection_identity.source_declared]

- GIVEN the CLI is configured with a Nickel source manifest, closed contract, generation receipt, and runtime projection
- WHEN it loads the projection for validation
- THEN it MUST compute the projection BLAKE3 identity and compare it with the identity bound by the generation receipt before trusting fixture results.

### Requirement: Fixture manifest projection freshness

r[kamacite.validation_harness.fixture_manifest.projection_freshness] Default harness validation MUST fail closed when the runtime JSON projection is stale relative to checked generation evidence binding the exact source, contract, evaluator profile, and projection bytes. The failure MUST be deterministic and MUST NOT mutate the Nickel source, runtime projection, generation receipt, or golden receipts.

#### Scenario: Matching projection is accepted

r[kamacite.validation_harness.fixture_manifest.projection_freshness.matching_projection]

- GIVEN checked generation evidence binds the current source, contract, evaluator profile, and runtime JSON projection
- WHEN default validation loads the manifest
- THEN validation MUST continue to fixture execution and receipt comparison without executing Nickel.

#### Scenario: Stale projection is rejected

r[kamacite.validation_harness.fixture_manifest.projection_freshness.stale_projection]

- GIVEN the runtime JSON projection bytes differ from the identity bound by checked generation evidence
- WHEN default validation loads the manifest
- THEN validation MUST fail closed with deterministic manifest-generation mismatch evidence and MUST NOT rewrite evidence.

### Requirement: Fixture manifest ID alignment

r[kamacite.validation_harness.fixture_manifest.fixture_id_alignment] The validation shell MUST bind the complete ordered fixture projection before trusting it. Missing, duplicate, reordered, or extra fixtures MUST change the checked projection identity or fail structural validation before fixture receipts are trusted.

#### Scenario: Fixture ID drift is rejected

r[kamacite.validation_harness.fixture_manifest.fixture_id_alignment.drift]

- GIVEN the Nickel source manifest and runtime JSON projection name different fixture IDs or the same fixture IDs in different order
- WHEN default validation loads the projection
- THEN validation MUST fail closed with deterministic manifest-generation mismatch evidence.

### Requirement: Freshness covers the complete manifest projection

r[kamacite.findings.manifest_semantic_freshness.complete_projection] Before fixture materialization, Kamacite MUST compare the complete checked Nickel manifest projection with the exact runtime projection bytes. Fixture metadata, payload descriptors, verdicts, classes, requirements, non-claims, laws, golden paths, and identities MUST all participate.

#### Scenario: Verdict changes while IDs remain stable

- GIVEN the Nickel source changes one or more fixture verdicts without changing fixture IDs or the old embedded projection hash
- WHEN default validation checks freshness
- THEN validation MUST fail before fixture execution with deterministic source/projection drift evidence.

### Requirement: Manifest payloads use a closed typed contract

r[kamacite.findings.manifest_semantic_freshness.typed_contract] The Nickel manifest contract MUST define closed payload alternatives for every admitted fixture kind and MUST reject missing required fields, unsupported kinds, and invalid field types before projection trust.

#### Scenario: Dynamic payload omits required identity

- GIVEN a fixture payload kind requires a declared identity but the Nickel source omits it
- WHEN the manifest contract is evaluated
- THEN export MUST fail and no runtime projection or generation receipt MUST be accepted.

### Requirement: Default trust uses checked generation evidence

r[kamacite.findings.manifest_semantic_freshness.generation_receipt] Default validation MUST either obtain a checked Nickel export or verify a generation receipt binding exact Nickel source identity, contract identity, evaluator/profile identity, and runtime projection identity. Lexical assignment or fixture-ID extraction MUST NOT establish freshness.

#### Scenario: Source changed after projection generation

- GIVEN a generation receipt binds an earlier Nickel source identity and unchanged runtime JSON
- WHEN the source bytes change
- THEN default validation MUST reject the stale generation receipt without rewriting source or projection files.

### Requirement: Textual spoofing fails closed

r[kamacite.findings.manifest_semantic_freshness.negative_cases] Manifest freshness tests MUST cover malformed Nickel, comments or strings containing assignment-like text, duplicate fixture IDs, reordered or extra fixtures, metadata-only drift, and stale evaluator or contract identity.

#### Scenario: Comment contains a matching hash

- GIVEN malformed or divergent Nickel text contains a comment or string that resembles the expected projection assignment and fixture IDs
- WHEN freshness validation runs
- THEN the text MUST NOT satisfy projection trust and validation MUST fail deterministically.

### Requirement: Harness core and runner boundary

r[kamacite.validation_harness.core_runner_boundary] Kamacite MUST separate the pure validation core from the effectful harness runner. The runner MAY discover fixture files, read bytes, load exported manifest projections, compare golden receipts, and render test output. The core MUST receive normalized in-memory DTOs and bytes and return deterministic validation results, diagnostics, and receipt data.

#### Scenario: Runner normalizes fixture inputs

r[kamacite.validation_harness.core_runner_boundary.normalized_inputs]

- GIVEN the harness runner discovers fixture files and manifest entries
- WHEN it invokes the validation core
- THEN it MUST pass normalized artifact bytes, DTO values, declared identities, and expected metadata rather than paths, file handles, environment variables, clocks, random sources, process handles, or presentation sinks.

#### Scenario: Core can be unit tested without the runner

r[kamacite.validation_harness.core_runner_boundary.unit_testable_core]

- GIVEN the same normalized fixture DTOs and bytes are provided directly to the core and through the harness runner
- WHEN validation completes
- THEN the core result, diagnostics, and receipt fields MUST be the same without requiring filesystem, process, evaluator, network, clock, random, credential, or mutation mocks.

### Requirement: Seed fixture set

r[kamacite.validation_harness.seed_fixtures] The first harness implementation MUST seed positive, negative, and golden receipt coverage for the source-envelope slice. The seed set MUST include a valid source-envelope fixture, a stale-source-hash fixture, a wrong-identity-domain fixture, and a golden validation receipt fixture.

#### Scenario: Valid source envelope is accepted

r[kamacite.validation_harness.seed_fixtures.valid_source_envelope]

- GIVEN a seed fixture declares source bytes, language label, parser identity, schema version, non-claims, and a matching BLAKE3 source identity
- WHEN the harness validates the fixture
- THEN validation MUST accept the source envelope and emit a deterministic receipt that binds the source identity and non-claims.

#### Scenario: Stale source hash is rejected

r[kamacite.validation_harness.seed_fixtures.stale_source_hash]

- GIVEN a seed fixture declares source bytes and a mismatched source identity
- WHEN the harness validates the fixture
- THEN validation MUST reject it with a deterministic stale-identity diagnostic and MUST NOT emit a trusted acceptance receipt.

#### Scenario: Wrong identity domain is rejected

r[kamacite.validation_harness.seed_fixtures.wrong_identity_domain]

- GIVEN a seed fixture supplies a projection, canonical value, target artifact, adapter, evaluator, compiler, or receipt identity where a source identity is required
- WHEN the harness validates the fixture
- THEN validation MUST reject it with a deterministic identity-domain diagnostic and MUST NOT coerce the identity into the source domain.

#### Scenario: Golden receipt comparison is deterministic

r[kamacite.validation_harness.seed_fixtures.golden_receipt]

- GIVEN a seed fixture has a checked-in golden receipt
- WHEN the harness validates the same explicit inputs repeatedly
- THEN the emitted receipt MUST match the golden receipt exactly except for fields explicitly declared outside canonical receipt identity.

### Requirement: Default non-mutating receipt updates

r[kamacite.validation_harness.receipt_update_boundary] The harness MUST be non-mutating by default. Golden receipt updates, generated manifest projections, generated schemas, or other evidence artifacts MUST require an explicit update workflow and MUST record old and new identities for changed artifacts.

#### Scenario: Default run does not rewrite evidence

r[kamacite.validation_harness.receipt_update_boundary.default_no_rewrite]

- GIVEN a golden receipt, manifest projection, or generated evidence artifact is stale
- WHEN the harness runs in default validation mode
- THEN it MUST fail with deterministic diagnostics and MUST NOT rewrite the stale artifact.

#### Scenario: Explicit update records changed identities

r[kamacite.validation_harness.receipt_update_boundary.explicit_update]

- GIVEN a future explicit update workflow changes a golden receipt, manifest projection, generated schema, or evidence artifact
- WHEN the workflow completes
- THEN it MUST emit evidence that names each changed artifact, old identity, new identity, covered requirements, and non-claims.

### Requirement: Adapter suite deferral

r[kamacite.validation_harness.adapter_suite_deferral] The walking-skeleton harness MUST NOT require complete Nickel, Steel, WASM, LLVM, Valence, Octet, or algebraic-effect execution suites before the first core source-envelope slice lands. Adapter-specific execution suites SHOULD be added with their own positive, negative, and golden fixtures after the pure core fixture gate passes.

#### Scenario: External adapters are not required for skeleton pass

r[kamacite.validation_harness.adapter_suite_deferral.no_external_adapter_requirement]

- GIVEN the walking-skeleton harness is validating the first pure source-envelope slice
- WHEN Nickel, Steel, WASM, compiler, Valence, Octet, or algebraic-effect runtime tools are absent or intentionally disabled
- THEN the skeleton harness MAY still pass if the pure fixture contract, non-claims, and receipt checks pass without executing those external tools.

#### Scenario: Adapter behavior requires later fixture expansion

r[kamacite.validation_harness.adapter_suite_deferral.later_expansion]

- GIVEN a later implementation claims Nickel, Steel, WASM, LLVM, Valence, Octet, algebraic-effect, or other adapter behavior
- WHEN that behavior is proposed for trusted claim admission
- THEN the fixture corpus MUST expand with adapter-specific positive fixtures, negative fixtures, declared capabilities, diagnostics, and golden receipts before the behavior is trusted.
