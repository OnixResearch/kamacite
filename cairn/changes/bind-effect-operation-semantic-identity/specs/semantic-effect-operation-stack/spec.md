# Algebraic Effect Fixtures and Stack Boundary Specification Delta

## Purpose

Define how strengthened semantic effect-operation identities cross Kamacite, Valence, Molten, Trellis, and Octet boundaries without transferring canonical ownership or authority.

## ADDED Requirements

### Requirement: Runtime consumers bind Kamacite semantic identities

r[kamacite.algebraic_effects.semantic_stack_consumption] Aspen/Molten and other runtime consumers SHOULD bind exact Kamacite semantic operation descriptor identities in effect manifests, handler profiles, requests, responses, logs, replay/cache records, remote-execution evidence, and upgrade diagnostics rather than reconstructing identity from runtime-local names.

#### Scenario: Molten emits runtime effect evidence
- GIVEN Molten admits and executes an operation derived from a Kamacite descriptor
- WHEN it emits runtime evidence
- THEN the evidence SHOULD reference the exact Kamacite semantic operation identity while retaining Molten-owned capability, policy, resource, and execution decisions.

#### Scenario: Runtime-local name attempts to replace canonical identity
- GIVEN a runtime knows an operation only by local effect id and operation name
- WHEN it claims Kamacite semantic compatibility
- THEN Kamacite stack validation MUST reject the claim until the exact descriptor identity or explicit compatibility artifact is bound.

### Requirement: Valence export preserves semantic-key roles

r[kamacite.algebraic_effects.semantic_valence_export] Valence-friendly projections MUST distinguish semantic operation descriptors, behavior refs, compatibility artifacts, handler descriptors, and runtime receipts by role and MUST preserve their identities, parent edges, checked claims, diagnostics, and non-claims.

#### Scenario: Compatibility artifact is exported
- GIVEN a directional semantic-operation compatibility artifact passes Kamacite validation
- WHEN it is projected for Valence
- THEN the projection MUST remain bounded compatibility or boundary evidence and MUST NOT become behavioral-correctness or host-authorization evidence.

### Requirement: Supporting proof and source checks do not replace semantic identity

r[kamacite.algebraic_effects.semantic_supporting_evidence] Trellis proof artifacts and Octet source-shape or provenance findings MAY support semantic operation review but MUST NOT replace the canonical Kamacite descriptor, behavior ref, compatibility artifact, or local validation result.

#### Scenario: Supporting evidence references a stale descriptor
- GIVEN a Trellis or Octet artifact references an older semantic operation identity
- WHEN a new descriptor is validated
- THEN the supporting evidence MUST be classified stale or inapplicable rather than transferred to the new identity.
