# Stack Integration Specification

## Purpose

Defines Kamacite's integration boundary with Valence and Octet without transferring ownership of Kamacite's source/value interchange semantics.

## Requirements

### Requirement: Stack evidence integration boundary

r[kamacite.lossless_ir.stack_evidence_boundary] Kamacite MUST integrate with Valence and Octet through bounded evidence surfaces before adopting direct crate coupling. Kamacite receipts SHOULD expose stable IDs, source spans, BLAKE3 hashes, verification roles, and non-claims in a form that Valence can wrap or consume. Kamacite Rust code SHOULD run through Octet checks once Rust crates exist, but Octet findings MUST NOT be represented as proof of Nickel, Rust, or Steel semantic equivalence.

#### Scenario: Valence wraps Kamacite receipt claims

r[kamacite.lossless_ir.stack_evidence_boundary.valence_receipts]

- GIVEN a Kamacite round-trip or adapter receipt has stable identity, source span metadata, verification role, canonical hash, and explicit non-claims
- WHEN the receipt is exported for stack evidence processing
- THEN the export SHOULD be mappable to Valence Evidence IR without broadening Kamacite's bounded identity-preservation claim.

#### Scenario: Octet checks Rust implementation only

r[kamacite.lossless_ir.stack_evidence_boundary.octet_checks]

- GIVEN Kamacite has Rust validators, adapters, fixture runners, or receipt code
- WHEN Octet checks or provenance collection run against that Rust scope
- THEN the resulting evidence MUST be classified as Rust source-shape/provenance evidence and MUST NOT be promoted to arbitrary language semantic-equivalence proof.
