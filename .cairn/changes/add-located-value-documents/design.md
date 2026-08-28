## Context

Kamacite preserves source bytes and canonical Preserves values under separate identities. The source-envelope vocabulary already permits an optional span index.

The current model lacks one portable path vocabulary. It also lacks a typed relation from canonical values to source origins.

`sexplib` revision `e5a4df62c104c8f7c2e8a11ce36e88808649fce4` supplies three design references. They are typed structural paths, annotated parse trees, and delayed location-aware conversion.

The adaptation uses Kamacite schemas and Rust types. It does not adopt S-expression syntax or OCaml runtime behavior.

## Goals

- Locate one canonical value without display-string parsing.
- Replace one subtree without mutating the admitted root.
- Bind every replacement to exact pre-state identities.
- Distinguish exact, derived, and unavailable source origins.
- Preserve bounded, redacted, stable diagnostics across adapters.

## Non-goals

- A general editor framework.
- A Nickel evaluator or Steel evaluator.
- Exact source claims for computed values.
- Valence parsing or source-map ownership.
- Ambient file access from the pure component.

## Decisions

### Decision: Use one narrow pure component

**Choice:** Add `crates/kamacite-located-value/` as the owner of paths, origins, lookup, replacement, and diagnostic location facts.

The component accepts admitted in-memory values and source facts. Shell adapters own parsing, evaluation, file reads, and presentation.

**Rationale:** These operations share one coherent capability. The boundary also prevents parser and filesystem authority from entering the core.

### Decision: Bind paths to schemas

**Choice:** A path binds its schema identity and named path-limit profile. Segments distinguish record fields, variant arguments, sequence indexes, and dictionary key identities.

A dictionary segment uses canonical key identity. It does not depend on display text or collection iteration order.

A text path parser remains a presentation adapter. The pure core receives validated path segments.

**Rationale:** Positional list paths depend on one encoding convention. Schema-bound segments retain meaning across review projections.

### Decision: Guard immutable replacement with BLAKE3 identities

**Choice:** A replacement plan binds the root identity, target path, target identity, and replacement identity.

Application validates every pre-state fact before it constructs a new root. The original root remains unchanged.

**Rationale:** A path alone cannot detect stale input or substitution. Exact identities make the precondition explicit.

### Decision: Model source origin as an explicit alternative

**Choice:** `Exact` carries one source-envelope identity and a bounded ordered span set. `Derived` carries bounded parent origins and a derivation class.

`Absent` carries a stable reason. It is not an empty exact span.

Every exact origin binds a locator identity. It also binds parser identity when a parser supplied the spans.

Byte ranges are authoritative. Line and column ranges are validated projections.

**Rationale:** Nickel merges, contract wrappers, generated values, and deserialization can break one-to-one source correspondence.

### Decision: Refine locations from one captured snapshot

**Choice:** A fast adapter path can omit a source index. After a conversion failure, the shell can run a location-aware parser over the same captured bytes.

The shell must not reopen a path or reevaluate source during refinement. The core receives both attempts as in-memory evidence.

**Rationale:** Delayed refinement reduces normal-path cost. One immutable snapshot prevents time-of-check and time-of-use drift.

### Decision: Version every canonical location surface

**Choice:** Paths, origins, located documents, diagnostics, and receipts use versioned Preserves schemas.

Existing source envelopes with no index retain their original meaning. Any canonical shape change receives a new schema version.

Adapter descriptors bind the schema identities, locator identity, optional parser identity, and limit profile.

**Rationale:** Optional implementation fields must not silently reinterpret prior canonical bytes.

### Decision: Keep parser and evidence roles narrow

**Choice:** Tree-sitter Nickel can produce syntax spans. It cannot establish Nickel evaluation meaning.

Kamacite exports admitted spans to Valence. Valence retains evidence identity and linkage ownership.

**Rationale:** This preserves the existing stack authority flow.

### Decision: Use the upstream code only as a design reference

**Choice:** The implementation is Rust-owned and clean-room at the mechanism level.

Any copied upstream expression must retain the MIT notice. The preferred implementation copies no OCaml source text.

**Rationale:** The value model, error model, and authority boundaries differ from `sexplib`.

## Adversarial Audit

The audit must try these failures:

- a path uses the wrong schema;
- a path exceeds its segment bound;
- a dictionary key display string hides another key identity;
- a replacement uses a stale root or target identity;
- an exact span exceeds source bytes;
- a derived value is mislabeled as exact;
- a fallback pass reads changed bytes;
- a parser span is promoted to evaluation evidence;
- a diagnostic leaks source text or secret-like values;
- a Valence projection weakens the Kamacite non-claim.

Any successful counterexample blocks closeout.

## Risks / Trade-offs

- New canonical schemas create migration work for fixtures and receipts.
- Source-origin precision varies by adapter. Explicit absence is safer than fabricated precision.
- Guarded replacement costs extra hashing. The cost buys stale-input detection.
- Delayed refinement adds a second parse only after failure. It does not add a second evaluation.
