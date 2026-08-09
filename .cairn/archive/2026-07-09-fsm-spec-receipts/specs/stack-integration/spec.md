# Stack Integration Delta

## ADDED Requirements

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
