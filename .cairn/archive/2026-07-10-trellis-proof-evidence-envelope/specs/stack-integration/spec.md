# Stack Integration Specification

## Purpose

Canonicalize Trellis proof evidence IR as a Kamacite Preserves evidence-chain profile.

## Requirements

### Requirement: Trellis proof evidence has a canonical envelope profile
r[kamacite.stack_integration.trellis_proof_evidence_envelope.schema] Kamacite MUST define a Trellis proof evidence profile for canonical evidence-chain envelopes.

#### Scenario: Trellis handoff canonicalizes
r[kamacite.stack_integration.trellis_proof_evidence_envelope.positive]
- GIVEN Trellis provides a valid proof evidence-chain handoff
- WHEN Kamacite canonicalizes it
- THEN Kamacite MUST emit a canonical Preserves evidence-chain envelope with the Trellis proof profile.

### Requirement: Source, IR, verifier, assumptions, and properties are linked
r[kamacite.stack_integration.trellis_proof_evidence_envelope.links] Kamacite MUST bind source envelope identity, proof IR identity, verifier receipt identity, assumptions root, dependency root, property IDs, requirement IDs, and non-claim boundary hash in the Trellis proof profile.

#### Scenario: Assumption drift changes validation
r[kamacite.stack_integration.trellis_proof_evidence_envelope.negative]
- GIVEN a Trellis proof profile has stale or missing assumption roots, malformed property IDs, or missing verifier receipt identity
- WHEN Kamacite validates the profile
- THEN validation MUST fail with deterministic profile diagnostics.

### Requirement: Role metadata is bounded
r[kamacite.stack_integration.trellis_proof_evidence_envelope.roles] Kamacite MUST preserve producer role metadata for recorded-only and formal-proof candidate evidence without deciding downstream proof promotion.

#### Scenario: Formal-proof candidate remains a candidate
- GIVEN Trellis proof evidence claims a passed verifier result
- WHEN Kamacite encodes the canonical envelope
- THEN the envelope MAY record a formal-proof candidate role but MUST NOT claim downstream Octet, Mantle, or Cairn proof acceptance.

### Requirement: JSON projection is non-authoritative
r[kamacite.stack_integration.trellis_proof_evidence_envelope.projection] Kamacite MAY emit JSON projections for Trellis proof envelopes only when they bind to canonical Preserves envelope identity and preserve profile roots.

#### Scenario: Projection drift fails closed
r[kamacite.stack_integration.trellis_proof_evidence_envelope.validation]
- GIVEN a JSON projection differs from canonical Preserves content, omits envelope identity, or weakens non-claims
- WHEN Kamacite validates projection binding
- THEN validation MUST fail with deterministic projection-drift diagnostics.
