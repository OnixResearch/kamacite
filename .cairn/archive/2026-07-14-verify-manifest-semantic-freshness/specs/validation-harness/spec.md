# Validation Harness Specification

## Purpose

Require complete semantic freshness between the reviewed Nickel manifest and the runtime projection consumed by the harness.

## Requirements

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
