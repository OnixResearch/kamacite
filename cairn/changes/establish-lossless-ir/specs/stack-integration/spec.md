# Stack Integration Specification

## Purpose

Defines Braid's integration boundary with Valence and Octet without transferring ownership of Braid's source/value interchange semantics.

## Requirements

### Requirement: Stack evidence integration boundary

r[braid.lossless_ir.stack_evidence_boundary] Braid MUST integrate with Valence and Octet through bounded evidence surfaces before adopting direct crate coupling. Braid receipts SHOULD expose stable IDs, source spans, BLAKE3 hashes, verification roles, and non-claims in a form that Valence can wrap or consume. Braid Rust code SHOULD run through Octet checks once Rust crates exist, but Octet findings MUST NOT be represented as proof of Nickel, Rust, or Steel semantic equivalence.

#### Scenario: Valence wraps Braid receipt claims

r[braid.lossless_ir.stack_evidence_boundary.valence_receipts]

- GIVEN a Braid round-trip or adapter receipt has stable identity, source span metadata, verification role, canonical hash, and explicit non-claims
- WHEN the receipt is exported for stack evidence processing
- THEN the export SHOULD be mappable to Valence Evidence IR without broadening Braid's bounded identity-preservation claim.

#### Scenario: Octet checks Rust implementation only

r[braid.lossless_ir.stack_evidence_boundary.octet_checks]

- GIVEN Braid has Rust validators, adapters, fixture runners, or receipt code
- WHEN Octet checks or provenance collection run against that Rust scope
- THEN the resulting evidence MUST be classified as Rust source-shape/provenance evidence and MUST NOT be promoted to arbitrary language semantic-equivalence proof.
