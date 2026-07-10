# Stack Integration Specification

## Purpose

Define a generic Kamacite canonical evidence-chain envelope for Preserves-backed stack evidence.

## Requirements

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
