# Stack Integration Specification

## Purpose

Defines Kamacite's integration boundary with Valence and Octet without transferring ownership of Kamacite's source/value interchange semantics.

## Requirements

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
