## Context

Kamacite envelopes preserve source bytes, BLAKE3 identities, and canonical Preserves values. Adapters move Nickel-authored contracts, Rust validators, and Steel probes across one interchange. Kamacite owns source preservation, value interchange, adapter classes, and round-trip claims. It does not claim arbitrary source-to-source translation.

The `mbrock/luv` wiki page `quantities-and-measurement.org` records a semantic arithmetic discipline. Its core claim transfers directly: dimension, named meaning, unit, affine character, and representation are separate questions, and operations must not succeed merely because values share a representation. Kamacite's job is interchange, not evaluation, so this change adapts the semantic separation rather than the arithmetic machinery.

## Goals

- Carry quantity specifications as canonical envelope data.
- Make specifications part of the envelope identity.
- Record meaning provenance without silent promotion.
- Make conversion explicit and recorded.
- Preserve declared meaning across pinned Nickel, Rust, and Steel adapters.

## Non-goals

- Unit conversion algebra, signaling, or physics evaluation inside Kamacite.
- Canonical unit systems or SI vocabulary ownership.
- Arbitrary cross-language semantic equivalence claims.
- Changes to the Kamacite identity scheme or to Nickel evaluation semantics.

## Decisions

### Decision: Specification is canonical identity

**Choice:** A quantity-preserving envelope carries a declared quantity specification as canonical Preserves data. The specification participates in the BLAKE3 identity. An absent specification is an explicit untyped mark, never an invented default.

**Rationale:** Two values that share bytes but answer different questions must not collide in identity.

### Decision: Separation over conflation

**Choice:** Named meaning, dimension, unit, and representation remain separate fields. Equal dimension or equal representation does not imply equal meaning.

**Rationale:** This is the direct adaptation of luv's rejection of shared-representation arithmetic.

### Decision: Bounded character

**Choice:** A specification may declare one character: point, absolute, or difference. Character and non-negativity are explicit marks. Non-negativity is never derived from a defining equation.

**Rationale:** Silent defaults hide the operations a value may lawfully join.

### Decision: Provenance is recorded

**Choice:** Meaning provenance is one of declared, assumed, or derived. Promotion from assumed to derived happens only through an explicit interpretation.

**Rationale:** Interpretation names meaning. It does not silently upgrade an assumption.

### Decision: Conversion is explicit adapter work

**Choice:** Unit and frame conversion are explicit adapter operations that bind a conversion identity. Silent inversion or silent unit change is refused.

**Rationale:** Wrong-space and wrong-unit bugs are classic interchange defects. Explicit projection keeps them visible.

### Decision: Cross-language claim is bounded

**Choice:** The preservation claim covers the declared quantity specification and raw value across the pinned adapter set. The receipt binds adapter identities. Arbitrary semantic equivalence is not claimed.

**Rationale:** Kamacite's non-claim boundary already forbids arbitrary source-to-source equivalence.

## Dependency Direction

```text
quantity-contracts (Nickel authoring vocabulary)
      ^
      |
kamacite adapters and envelopes
```

No new project dependencies. The sibling `quantity-contracts` component provides the Nickel authoring side. This change preserves and interchanges its declared values.

## Decision Flow

```text
Nickel-authored contract via quantity-contracts export
  -> source envelope with declared specification
  -> Rust or Steel adapter preserves specification and value
  -> target artifact with receipt binding all adapter identities
  -> fixture matrix rejects conflation, silent conversion, and overclaim
```

## Adversarial Audit

The audit must try to show that Kamacite:

- creates divergent identities for equal bytes with different meaning;
- silently defaults a character or a unit;
- promotes an assumption into a derivation without an interpretation;
- converts units or frames without recording the projection;
- claims semantic equivalence beyond the pinned adapter set;
- drops any declared specification across an interchange.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- Adding specification data to identity changes envelope hashes. Adapter descriptors must pin the schema version.
- Strict conversion rules raise the cost of legitimate boundary crossings. Explicit projection keeps that cost visible.
- The preservation claim is bounded by the pinned adapter set. Wider claims must be added deliberately.

## Non-Claims

This change does not prove unit algebra correctness, physics-law validity, evaluator equivalence, or arbitrary Nickel, Rust, or Steel semantic equivalence across all possible input.
