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

### Requirement: Kamacite owns a canonical evidence-chain envelope

r[kamacite.stack_integration.canonical_evidence_chain_envelope.schema] Kamacite MUST define a versioned Preserves schema for a canonical evidence-chain envelope that carries typed evidence kind, parent links, profile payload roots, policy/profile hashes, projections, and non-claim boundary hashes.

#### Scenario: Function-address uses the generic envelope
r[kamacite.stack_integration.canonical_evidence_chain_envelope.function_address]
- GIVEN Octet provides a valid function-address producer handoff
- WHEN Kamacite canonicalizes it
- THEN Kamacite MUST encode it as a function-address profile within the generic evidence-chain envelope.

### Requirement: Canonical bytes own envelope identity

r[kamacite.stack_integration.canonical_evidence_chain_envelope.hash] Kamacite MUST compute evidence-chain envelope identity as BLAKE3 over canonical Preserves binary bytes.

#### Scenario: Canonical identity is stable
r[kamacite.stack_integration.canonical_evidence_chain_envelope.positive]
- GIVEN the same typed evidence-chain envelope is represented in supported Preserves transfer syntaxes
- WHEN Kamacite canonicalizes the value
- THEN canonical binary bytes and envelope identity MUST be stable.

### Requirement: Parent links and roots are typed

r[kamacite.stack_integration.canonical_evidence_chain_envelope.links] Kamacite MUST domain-separate evidence-chain parent links and collection roots so source, producer, receipt, validation, release, and readiness identities are not interchangeable.

#### Scenario: Wrong link domain is rejected
r[kamacite.stack_integration.canonical_evidence_chain_envelope.negative]
- GIVEN an evidence-chain envelope places a syntactically valid hash under the wrong parent-link domain or omits required policy identity
- WHEN Kamacite validates the envelope
- THEN validation MUST fail with deterministic domain diagnostics.

### Requirement: Profile collections use deterministic roots

r[kamacite.stack_integration.canonical_evidence_chain_envelope.roots] Kamacite SHOULD support deterministic Merkle-style roots over sorted typed profile leaves for evidence profiles that contain collections.

#### Scenario: Collection root survives projection
- GIVEN an evidence-chain profile has a set of typed leaves
- WHEN Kamacite renders a compatibility projection
- THEN the projection MUST bind to the same deterministic collection root and canonical envelope hash.

### Requirement: JSON projection is non-authoritative

r[kamacite.stack_integration.canonical_evidence_chain_envelope.validation] Kamacite MAY emit JSON projections of evidence-chain envelopes, but each projection MUST bind to the canonical Preserves envelope hash and MUST preserve the non-claim boundary.

#### Scenario: Projection drift fails closed
- GIVEN a JSON projection diverges from canonical Preserves content, omits canonical identity, or weakens non-claims
- WHEN validation compares projection and envelope
- THEN validation MUST fail with deterministic projection-drift diagnostics.

### Requirement: Trellis proof evidence has a canonical envelope profile

r[kamacite.stack_integration.trellis_proof_evidence_envelope.schema] Kamacite MUST define a Trellis proof evidence profile for canonical evidence-chain envelopes that preserves source, proof IR, verifier, assumptions, dependencies, property IDs, requirement IDs, role metadata, and non-claim roots without deciding downstream promotion.

#### Scenario: Trellis handoff canonicalizes
r[kamacite.stack_integration.trellis_proof_evidence_envelope.positive]
- GIVEN Trellis provides a valid proof evidence-chain handoff
- WHEN Kamacite canonicalizes it
- THEN Kamacite MUST emit a canonical Preserves evidence-chain envelope with the Trellis proof profile.

### Requirement: Trellis proof links are bounded

r[kamacite.stack_integration.trellis_proof_evidence_envelope.links] Kamacite MUST bind source envelope identity, proof IR identity, verifier receipt identity, assumptions root, dependency root, property IDs, requirement IDs, and non-claim boundary hash in the Trellis proof profile.

#### Scenario: Assumption drift changes validation
r[kamacite.stack_integration.trellis_proof_evidence_envelope.negative]
- GIVEN a Trellis proof profile has stale or missing assumption roots, malformed property IDs, wrong identity domains, overclaims, or missing verifier receipt identity
- WHEN Kamacite validates the profile
- THEN validation MUST fail with deterministic profile diagnostics.

### Requirement: Trellis proof role metadata is bounded

r[kamacite.stack_integration.trellis_proof_evidence_envelope.roles] Kamacite MUST preserve producer role metadata for recorded-only and formal-proof candidate evidence without deciding downstream proof promotion.

#### Scenario: Formal-proof candidate remains a candidate
- GIVEN Trellis proof evidence claims a passed verifier result
- WHEN Kamacite encodes the canonical envelope
- THEN the envelope MAY record a formal-proof candidate role but MUST NOT claim downstream Octet, Mantle, or Cairn proof acceptance.

### Requirement: Trellis proof JSON projection is non-authoritative

r[kamacite.stack_integration.trellis_proof_evidence_envelope.projection] Kamacite MAY emit JSON projections for Trellis proof envelopes only when they bind to canonical Preserves envelope identity and preserve profile roots.

#### Scenario: Projection drift fails closed
r[kamacite.stack_integration.trellis_proof_evidence_envelope.validation]
- GIVEN a JSON projection differs from canonical Preserves content, omits envelope identity, or weakens non-claims
- WHEN Kamacite validates projection binding
- THEN validation MUST fail with deterministic projection-drift diagnostics.

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

### Requirement: Conformance identity covers normative content

r[kamacite.findings.valence_conformance.content_identity] Kamacite MUST compute the adapter-domain conformance identity from canonical bytes covering profile version, source/output schemas, identity-domain policy, checked laws, dependency profile, fixture coverage, and non-claims. The identity field itself MUST be excluded from recursive hash material.

#### Scenario: Checked law changes

- GIVEN a reviewed conformance entry adds, removes, or changes a checked law
- WHEN Kamacite canonicalizes the entry
- THEN the conformance identity MUST change and the old identity MUST be rejected as stale.

### Requirement: JSON conformance is a bound projection

r[kamacite.findings.valence_conformance.bound_projection] A JSON conformance projection MUST identify its canonical conformance codec and identity, and validation MUST recompute or resolve that identity before export. Domain validation alone MUST NOT establish conformance trust.

#### Scenario: JSON retains an old identity

- GIVEN a JSON conformance projection changes a normative field while retaining its previous adapter-domain identity
- WHEN Valence export validation runs
- THEN export MUST fail before carrying any conformance laws into derived evidence.

### Requirement: CLI consumes the reviewed artifact

r[kamacite.findings.valence_conformance.cli_input] The Valence export command MUST consume canonical conformance bytes or a previously validated canonical conformance artifact, with any JSON input treated only as a compatibility projection.

#### Scenario: Self-asserted JSON is the only input

- GIVEN a caller supplies a structurally valid JSON object with an arbitrary adapter-domain hash but no bound canonical conformance artifact
- WHEN export is requested
- THEN the command MUST reject it as untrusted conformance input.

### Requirement: Conformance migration is explicit

r[kamacite.findings.valence_conformance.migration] Conformance identity changes MUST update export goldens and emit old/new identity evidence, and tests MUST cover field mutation, set reordering, duplicates, malformed entries, and wrong identity domains.

#### Scenario: Set-like fields reorder only

- GIVEN two conformance entries differ only in the presentation order of fields declared set-like
- WHEN canonical identities are computed
- THEN the identities MUST remain equal after sorted-unique canonicalization.

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
