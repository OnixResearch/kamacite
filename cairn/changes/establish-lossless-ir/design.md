## Context

The stack already uses a practical split: Nickel owns human-authored contract/config review boundaries, Rust owns runtime validators and DTOs, and Steel Scheme appears as bounded predicate/probe logic. Existing JSON handoffs are useful for runtime consumption but are not sufficient for lossless interchange claims.

Braid starts by documenting the invariant rather than pretending arbitrary source-to-source translation is available. The canonical path is source-preserving envelopes plus a constrained value/policy IR with deterministic receipts.

## Decisions

### 1. Name the repo Braid

**Choice:** Use `braid` as the new OnixResearch repository name.

**Rationale:** The project braids three language strands without claiming they become one language. The name fits a boundary repo rather than a compiler or evaluator.

### 2. Do not call Braid a new language yet

**Choice:** Braid is an interchange IR, source-envelope format, receipt contract, and constrained policy vocabulary. It is not a programming language unless a later change defines a human-authored syntax, grammar, evaluator, type rules, and semantics.

**Rationale:** Calling Braid a language would imply semantics and evaluation authority that the project explicitly does not claim. Nickel, Rust, and Steel remain the languages; Braid preserves sources, classifies artifacts, and verifies bounded interchange receipts.

**Rejected alternative:** Market Braid as a universal cross-language language. This would encourage unsupported source-to-source and semantic-equivalence claims.

### 3. Use Preserves as the canonical encoded IR candidate

**Choice:** Braid uses Preserves as the authoritative encoded form for canonical IR values, source envelopes, and receipt hash material where the project controls the format.

**Rationale:** Preserves is a better fit than JSON for canonical identity and typed data. JSON remains useful as compatibility output, but it is not the authority for losslessness because it cannot carry every distinction Braid needs without extra conventions.

**Rejected alternative:** Use JSON as the only IR. This would repeat the current lossy boundary and make source fidelity and typed identity claims fragile.

### 4. Preserve source separately from semantic projections

**Choice:** Ingested Nickel, Rust, and Steel source artifacts keep original bytes, declared language, parser/normalizer identity, and BLAKE3 source hash. Parsed or normalized projections are derived evidence and must not replace the original source.

**Rationale:** Arbitrary source-to-source translation is not lossless in a useful sense. Preserving source plus declaring narrower derived claims is honest and testable.

### 5. Constrain adapter claims to explicit artifact classes

**Choice:** Braid recognizes source envelopes, canonical values, constrained policy DSL values, and unsupported arbitrary source translation as distinct classes.

**Rationale:** The same bytes may support different claims. A constrained DSL can round-trip through adapters, while arbitrary Rust or Steel source can only be preserved unless a specific parser/projection contract exists.

### 6. Keep Rust as the pure validation core

**Choice:** Rust owns pure deterministic validation, canonicalization, hash material construction, adapter DTO validation, and receipt checks. Nickel and Steel adapters stay at the boundary.

**Rationale:** This preserves functional-core / imperative-shell boundaries and gives tests a pure unit under test. Nickel remains the reviewable authoring language for configs/contracts. Steel remains bounded dynamic predicate/probe logic without ambient host authority.

### 7. Use WebAssembly only as a target or sandbox boundary

**Choice:** Braid does not use WebAssembly as the canonical IR. WASM may be used later as a portable target for validators, conformance probes, or untrusted adapters whose inputs and outputs are Braid IR/source-envelope bytes plus deterministic receipts.

**Rationale:** WebAssembly is an execution format and sandbox boundary, not a source-preserving interchange format. Using it as the IR would confuse executable behavior with artifact identity. Keeping Preserves as the canonical IR while allowing WASM adapter targets lets Braid isolate evaluation without weakening losslessness claims.

**Rejected alternative:** Compile Nickel, Rust, or Steel into WASM and call that the lossless interchange. This loses source-level identity and overclaims semantic equivalence.

### 8. Integrate with Valence and Octet through receipts first

**Choice:** Braid uses Valence closely as the evidence/linkage integration target and Octet closely as the checked Rust/provenance rail, but neither repo owns Braid's canonical IR. Initial integration is by exported Braid receipts, source spans, stable IDs, and fixtures rather than direct crate dependencies.

**Rationale:** Valence already owns Evidence IR identity, roles, typed linkage, spans, and non-claim enforcement across the stack. Braid should make its receipts Valence-friendly so downstream repos can reason about Braid evidence without importing Braid semantics into Valence. Octet already owns Rust source checking and provenance for systems code; Braid should be an Octet consumer when Rust validators/adapters exist. Keeping the coupling receipt-first prevents circular ownership and avoids implying that Octet or Valence proves language semantic equivalence.

**Rejected alternative:** Make Braid a Valence module or an Octet rail from the start. That would blur Braid's source/value interchange ownership and make early format decisions harder to validate independently.

## Risks / Trade-offs

- **Preserves dependency uncertainty:** Mitigate by keeping the first change at the format-contract level and adding fixtures before relying on a concrete library.
- **False lossless claims:** Mitigate with explicit classes, non-claims, and negative fixtures.
- **Schema drift:** Mitigate with BLAKE3 hashes over canonical Preserves bytes and generated artifact freshness checks.
- **Over-broad Steel authority:** Mitigate by requiring DTO-only evaluation and deterministic denied receipts for ambient authority requests.
- **Nickel source confusion:** Mitigate by separating Nickel-authored source from exported runtime projections.
