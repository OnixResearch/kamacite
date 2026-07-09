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

### Requirement: Valence receipt export profile

r[kamacite.valence_receipt_export.profile] Kamacite MUST define a Valence-friendly receipt export profile as a derived evidence projection from validated Kamacite receipts. The profile MUST preserve the source Kamacite receipt identity, artifact identities, typed identity domains, roles, BLAKE3 hashes, adapter identities, conformance identities when present, checked laws, diagnostic summaries, source spans when present, and explicit non-claims. The export MUST NOT replace the Kamacite receipt, become canonical Kamacite IR, or broaden the source receipt's bounded claim.

#### Scenario: Export preserves bounded receipt facts

r[kamacite.valence_receipt_export.profile.bounded_projection]

- GIVEN a validated Kamacite receipt has stable identity, artifact identities, roles, hashes, checked laws, diagnostics, and non-claims
- WHEN Kamacite exports it using the Valence receipt export profile
- THEN the export MUST preserve those facts and MUST classify itself as a derived evidence projection of the source receipt.

#### Scenario: Non-claims survive wrapping

r[kamacite.valence_receipt_export.profile.non_claims]

- GIVEN a Kamacite receipt states non-claims about semantic equivalence, evaluator soundness, compiler correctness, runtime authorization, or canonical IR authority
- WHEN the receipt is exported for Valence consumption
- THEN every non-claim MUST remain present in the export and the export MUST NOT add a contradictory stronger claim.

#### Scenario: Source spans are explicit optional evidence

r[kamacite.valence_receipt_export.profile.source_spans]

- GIVEN a Kamacite receipt may or may not contain source span metadata
- WHEN the receipt is exported
- THEN present span metadata MUST be preserved, and absent span metadata MUST be represented as absent rather than fabricated from paths, labels, diagnostics, or artifact names.

#### Scenario: Export identity is distinct

r[kamacite.valence_receipt_export.profile.export_identity]

- GIVEN a Valence receipt export projection is emitted
- WHEN Kamacite computes identity for the projection
- THEN the projection identity MUST use the projection/export identity domain and MUST bind the source Kamacite receipt identity without substituting for it.

### Requirement: Valence coupling boundary

r[kamacite.valence_receipt_export.coupling_boundary] Kamacite MUST initially integrate with Valence through deterministic receipt/export artifacts and fixtures rather than requiring direct Valence crate coupling. A direct Valence code dependency MAY be added only after a later accepted change narrows the API, versioning, fixture, and ownership boundary.

#### Scenario: Receipt artifacts precede direct dependency

r[kamacite.valence_receipt_export.coupling_boundary.artifacts_first]

- GIVEN Kamacite needs to expose evidence for Valence wrapping
- WHEN the Valence receipt export profile is implemented
- THEN Kamacite MUST provide deterministic export artifacts or DTOs that can be reviewed independently of a live Valence runtime or crate dependency.

#### Scenario: Direct dependency needs later review

r[kamacite.valence_receipt_export.coupling_boundary.deferred_dependency]

- GIVEN a later implementation wants to depend directly on Valence crates or runtime APIs
- WHEN that dependency would affect Kamacite validation behavior or receipt semantics
- THEN the dependency MUST be introduced by a separate accepted lifecycle change with positive and negative integration fixtures.

### Requirement: FSM Valence export compatibility
r[kamacite.stack_integration.fsm_valence_export] Kamacite MUST provide an artifacts-first Valence export projection for FSM receipts that preserves Kamacite receipt identity, FSM spec identity, checked laws, diagnostics, source spans when available, artifact identities, roles, and non-claims without requiring live Valence runtime APIs.

#### Scenario: FSM receipt exports to Valence-friendly evidence
r[kamacite.stack_integration.fsm_valence_export.projection]
- GIVEN a valid Kamacite FSM exhaustiveness receipt and Valence export conformance entry
- WHEN Kamacite renders the Valence-friendly projection
- THEN the projection MUST preserve Kamacite receipt identity, FSM spec identity, universe identities, coverage identities, checked laws, diagnostics, roles, and non-claims.

#### Scenario: Valence export does not broaden claim
r[kamacite.stack_integration.fsm_valence_export.non_claims]
- GIVEN an FSM receipt is projected for Valence
- WHEN the projection is inspected
- THEN it MUST state that Valence export is derived compatibility evidence and does not prove runtime correctness, protocol completeness, liveness, verifier soundness, or release eligibility.

### Requirement: FSM Octet interop boundary
r[kamacite.stack_integration.fsm_octet_interop] Kamacite MUST document and fixture the boundary for Octet-consuming Rust projections of Kamacite FSM specs without treating Octet findings as proof of Kamacite FSM semantic correctness.

#### Scenario: Octet linked receipt remains source-shape evidence
r[kamacite.stack_integration.fsm_octet_interop.source_shape]
- GIVEN a Kamacite FSM receipt links to an Octet FSM exhaustive-default receipt
- WHEN Kamacite validates the linked receipt metadata
- THEN the link MUST be recorded as Octet source-shape/provenance evidence only.

#### Scenario: Missing Octet evidence does not invalidate portable spec
r[kamacite.stack_integration.fsm_octet_interop.optional]
- GIVEN a Kamacite FSM spec validates without an Octet Rust projection receipt
- WHEN Kamacite validates the portable spec
- THEN the spec MAY pass if its own schema, coverage, receipt identity, and non-claim checks pass, while reporting that no Rust source-shape evidence is attached.

#### Scenario: Octet overclaim fails closed
r[kamacite.stack_integration.fsm_octet_interop.overclaim]
- GIVEN linked Octet evidence claims Kamacite semantic correctness, runtime correctness, protocol completeness, liveness, verifier soundness, release eligibility, or arbitrary source equivalence
- WHEN Kamacite validates the linked receipt metadata
- THEN validation MUST fail with a deterministic stack-evidence-overclaim diagnostic.
