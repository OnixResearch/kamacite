# Source Envelopes Specification

## Purpose

Defines Kamacite source-envelope preservation for Nickel, Rust, and Steel source-bearing artifacts, including Nickel reference-tool integration.

## Requirements

### Requirement: Source preservation envelope

r[kamacite.lossless_ir.source_preservation] Kamacite MUST preserve original source bytes, declared language, parser or normalizer identity, and BLAKE3 source hash for source-bearing Nickel, Rust, and Steel artifacts.

#### Scenario: Normalization does not replace source

r[kamacite.lossless_ir.source_preservation.normalization]

- GIVEN a Nickel, Rust, or Steel source artifact has a normalized projection
- WHEN Kamacite stores or validates the artifact
- THEN the original source bytes and source hash MUST remain available as the source identity and the normalized projection MUST be marked as derived evidence.

#### Scenario: Source hash mismatch fails closed

r[kamacite.lossless_ir.source_preservation.hash_mismatch]

- GIVEN a source envelope declares a BLAKE3 source hash
- WHEN the stored source bytes hash to a different value
- THEN validation MUST fail closed with a deterministic source-hash diagnostic.

### Requirement: Nickel reference integration boundary

r[kamacite.lossless_ir.nickel_reference_boundary] Kamacite MUST treat upstream Nickel tooling as the authority for Nickel evaluation/export semantics while Kamacite owns source-envelope preservation, projection classification, and receipt identity. Nickel syntax parsers, schema-to-contract generators, and generated contract corpora MAY be used as adapters or fixtures, but they MUST NOT replace Kamacite's Preserves-backed canonical identity.

#### Scenario: Nickel evaluation stays external

r[kamacite.lossless_ir.nickel_reference_boundary.reference_eval]

- GIVEN a Nickel source envelope is evaluated or exported for a Kamacite fixture
- WHEN Kamacite records the projection
- THEN the receipt MUST identify the Nickel evaluator or CLI used and MUST classify the exported value as derived from the preserved source envelope.

#### Scenario: Nickel parser is span evidence only

r[kamacite.lossless_ir.nickel_reference_boundary.parser_spans]

- GIVEN tree-sitter Nickel or another syntax parser records spans for a Nickel source envelope
- WHEN Kamacite validates the envelope
- THEN the parser output MAY support source indexing and diagnostics but MUST NOT be treated as proof of Nickel evaluation semantics.

#### Scenario: Schema-generated contracts are fixtures

r[kamacite.lossless_ir.nickel_reference_boundary.generated_contracts]

- GIVEN a Nickel contract is generated from JSON Schema or SchemaStore inputs
- WHEN Kamacite uses it in tests
- THEN the generated contract MUST be classified as fixture or adapter input and MUST NOT become Kamacite's schema authority unless a Kamacite schema contract explicitly adopts it.
