## Context

The JSON conformance object contains normative fields and an identity, but validation treats the identity as an unrelated well-typed hash. The checked-in Preserves-shaped conformance entry is not consumed by the CLI.

## Decisions

### 1. Canonical conformance body excludes only its identity

**Choice:** Define a versioned body containing profile/source/output schemas, identity-domain policy, checked laws, dependency profile, fixture coverage, and non-claims. Compute the adapter identity over canonical body bytes.

**Rationale:** Every normative field must be covered while avoiding recursive hashing.

### 2. JSON is a bound projection

**Choice:** The runtime JSON carries the computed canonical conformance identity and codec/schema versions. Validation canonicalizes the body and compares the identity before export.

**Rationale:** Domain correctness alone does not establish content identity.

### 3. CLI input is one content-addressed artifact

**Choice:** Load canonical conformance bytes plus an optional JSON projection, or a validated in-memory artifact derived from them. Do not accept a self-contained JSON assertion as authoritative.

**Rationale:** Export should consume the reviewed entry it claims to identify.

### 4. Ordering semantics are explicit

**Choice:** Canonicalize set-like fields by sorted unique values and preserve sequence-like fields only where order is normative.

**Rationale:** Harmless presentation reorder should not change identity, but field mutation must.

## Risks / Trade-offs

- Existing conformance and export golden identities will change.
- This change should share the real Preserves codec once available; an interim canonical abstraction must not invent another pseudo-codec.
- Removing duplicate set entries may expose previously ambiguous conformance content.
