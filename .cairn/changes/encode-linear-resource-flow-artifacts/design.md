# Design: Encode linear resource-flow artifacts

## Context

Kamacite stores canonical data and receipt contracts. It does not execute algebraic effects or grant host authority.

The existing effect model covers operation signatures, effect rows, summaries, handlers, continuations, replay profiles, and effect logs.

Linear resource flow needs a separate artifact class. An effect row states which operations can occur. A resource row states which symbolic ownership obligations exist.

## Decisions

### Decision: Add an additive versioned profile

**Choice:** Add a `linear-resource-flow-v1` profile without changing the meaning of existing effect or machine artifacts.

**Rationale:** Existing bytes and identities must not gain new guarantees through reinterpretation.

### Decision: Keep artifact classes separate

**Choice:** Define distinct canonical artifacts for:

- resource kind descriptors
- resource occurrence references
- resource row expressions
- closed resource rows
- row substitutions
- resource transition descriptors
- resource-flow admission receipts

**Rationale:** Each class has a separate identity and evidence role. One aggregate document hides stale or mismatched links.

### Decision: Use canonical Preserves bytes as identity authority

**Choice:** Each artifact identity uses domain-separated BLAKE3 over canonical packed Preserves bytes.

Names, display labels, source paths, source spans, and documentation remain outside semantic identity.

**Rationale:** The existing Kamacite identity rule already separates canonical content from metadata.

### Decision: Encode symbolic facts only

**Choice:** A resource occurrence contains exact kind, state, and occurrence identities. It does not contain a live host object.

The schemas reject credential material, host endpoints, raw handles, ambient capability fields, process identifiers, and mutable runtime state.

**Rationale:** A canonical semantic descriptor must not become an authority or runtime transport.

### Decision: Preserve row order without an ordering claim

**Choice:** Canonical row artifacts retain an exact entry sequence. A separate operation must describe any admitted permutation.

The artifact does not claim that two row orders commute or that effects can reorder.

**Rationale:** Encoding order must stay deterministic without inventing semantic independence.

### Decision: Distinguish open expressions from closed rows

**Choice:** A row expression can contain a bounded tail-variable reference. A closed row contains only concrete occurrence references.

Choregraph machine artifacts and Lattice runtime intake use closed rows. Open expressions remain proof and composition data.

**Rationale:** Runtime interchange must not depend on unresolved substitutions.

### Decision: Link operation witnesses exactly

**Choice:** Duplication and disposal transitions carry exact witness artifact identities for the resource kind and operation.

A missing, stale, wrong-kind, or wrong-operation witness fails validation.

**Rationale:** A loose name or boolean flag cannot support durable non-linear operation evidence.

### Decision: Separate shape validation from semantic admission

**Choice:** Kamacite validates schema, canonical encoding, identity, unique occurrence shape, and exact linkage.

Trellis owns proof laws. Choregraph owns graph admission and the semantic decision recorded in the receipt.

**Rationale:** Kamacite must not become a second resource-flow evaluator.

### Decision: Bind receipts to producer evidence

**Choice:** An admission receipt binds:

- artifact profile and schema identities
- resource kind, occurrence, row, substitution, and transition identities
- Trellis model, source, proof-profile, and assumption identities
- Choregraph source, graph, machine, admission-profile, and decision identities
- deterministic diagnostic identities
- decision class and explicit non-claims

**Rationale:** Exact linkage makes stale producer or projection evidence fail closed.

### Decision: Keep authorization and execution outside the receipt

**Choice:** The receipt can state that symbolic graph admission accepted or rejected. It cannot state that authority allowed execution or that an effect occurred.

Lattice records separate authorization, attempt, observation, and recovery evidence.

**Rationale:** Planned effects and observed effects have different owners and evidence.

### Decision: Use explicit migration mappings

**Choice:** Any later artifact version uses adjacent migration plans with old identity, new identity, changed fields, and dry-run-first behavior.

The first profile does not convert existing effect rows into resource rows automatically.

**Rationale:** Automatic conversion invents linearity and occurrence meaning.

## Artifact Relationships

```text
ResourceKindDescriptor
        |
ResourceOccurrenceRef
        |
ResourceRowExpression ---- RowSubstitution
        |                         |
ClosedResourceRow <---------------
        |
ResourceTransitionDescriptor
        |
ResourceFlowAdmissionReceipt
        |
Valence-friendly evidence projection
```

Every arrow represents an exact identity reference. It does not represent runtime ownership transfer.

## Core and Shell Boundary

`kamacite-core` owns pure types, canonicalization, validation, identity framing, diagnostics, and receipt construction.

The CLI shell owns file reads, manifest loading, output writing, evaluator or compiler processes, and explicit golden updates.

No new port is needed for pure artifact validation.

## Validation Behavior

Positive fixtures include:

- one canonical closed row
- one open row expression and matching substitution
- one preserve transition
- one exact duplication witness
- one exact disposal witness
- one accepted Choregraph admission receipt
- canonical text and packed round trips

Negative fixtures include:

- malformed Preserves input
- unsupported profile version
- duplicate occurrence in a closed row
- unresolved variable in a runtime row
- noncanonical row order representation
- stale kind, row, substitution, transition, proof, graph, or schema identity
- wrong-kind duplication or disposal witness
- authority-bearing or live-handle field
- receipt that claims effect execution or authorization
- old effect artifact presented as a linear resource artifact

## Compatibility

Existing effect, FSM, source-envelope, adapter, target, and receipt artifacts remain unchanged.

Consumers must opt into the new profile. Absence of resource artifacts means no linear resource-flow claim.

## Evidence Boundary

Kamacite evidence proves only canonical decoding, validation, identity, linkage, and round-trip facts for the declared profile.

It does not prove Trellis soundness, row-unifier completeness, graph correctness, authority, runtime ownership, cleanup, liveness, effect completion, or release eligibility.

## Risks and Trade-offs

- More artifact classes add identity and migration work. Separate roles prevent broader ambiguous claims.
- Exact row order can create identity churn. A later permutation artifact can address reviewed independence cases.
- Unique occurrence validation does not prove conservation across a graph. Choregraph owns that decision.
- Proof references can become stale. Receipts must bind exact producer source and profile identities.
- Consumers can confuse occurrence references with capabilities. Closed schemas and non-claims must reject that interpretation.
