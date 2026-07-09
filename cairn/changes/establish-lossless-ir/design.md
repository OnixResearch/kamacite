## Context

The stack already uses a practical split: Nickel owns human-authored contract/config review boundaries, Rust owns runtime validators and DTOs, and Steel Scheme appears as bounded predicate/probe logic. Existing JSON handoffs are useful for runtime consumption but are not sufficient for lossless interchange claims.

Braid starts by documenting the invariant rather than pretending arbitrary source-to-source translation is available. The canonical path is source-preserving envelopes plus a constrained value/policy IR with deterministic receipts.

## Decisions

### 1. Name the repo Braid

**Choice:** Use `braid` as the new OnixResearch repository name.

**Rationale:** The project braids three language strands without claiming they become one language. The name fits a boundary repo rather than a compiler or evaluator.

### 2. Use Preserves as the canonical encoded IR candidate

**Choice:** Braid uses Preserves as the authoritative encoded form for canonical IR values, source envelopes, and receipt hash material where the project controls the format.

**Rationale:** Preserves is a better fit than JSON for canonical identity and typed data. JSON remains useful as compatibility output, but it is not the authority for losslessness because it cannot carry every distinction Braid needs without extra conventions.

**Rejected alternative:** Use JSON as the only IR. This would repeat the current lossy boundary and make source fidelity and typed identity claims fragile.

### 3. Preserve source separately from semantic projections

**Choice:** Ingested Nickel, Rust, and Steel source artifacts keep original bytes, declared language, parser/normalizer identity, and BLAKE3 source hash. Parsed or normalized projections are derived evidence and must not replace the original source.

**Rationale:** Arbitrary source-to-source translation is not lossless in a useful sense. Preserving source plus declaring narrower derived claims is honest and testable.

### 4. Constrain adapter claims to explicit artifact classes

**Choice:** Braid recognizes source envelopes, canonical values, constrained policy DSL values, and unsupported arbitrary source translation as distinct classes.

**Rationale:** The same bytes may support different claims. A constrained DSL can round-trip through adapters, while arbitrary Rust or Steel source can only be preserved unless a specific parser/projection contract exists.

### 5. Keep Rust as the pure validation core

**Choice:** Rust owns pure deterministic validation, canonicalization, hash material construction, adapter DTO validation, and receipt checks. Nickel and Steel adapters stay at the boundary.

**Rationale:** This preserves functional-core / imperative-shell boundaries and gives tests a pure unit under test. Nickel remains the reviewable authoring language for configs/contracts. Steel remains bounded dynamic predicate/probe logic without ambient host authority.

## Risks / Trade-offs

- **Preserves dependency uncertainty:** Mitigate by keeping the first change at the format-contract level and adding fixtures before relying on a concrete library.
- **False lossless claims:** Mitigate with explicit classes, non-claims, and negative fixtures.
- **Schema drift:** Mitigate with BLAKE3 hashes over canonical Preserves bytes and generated artifact freshness checks.
- **Over-broad Steel authority:** Mitigate by requiring DTO-only evaluation and deterministic denied receipts for ambient authority requests.
- **Nickel source confusion:** Mitigate by separating Nickel-authored source from exported runtime projections.
