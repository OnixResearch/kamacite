## Context

Kamacite keeps source envelopes, canonical Preserves values, adapter descriptors, target artifacts, and receipts as separate identities, and treats JSON as a compatibility projection only. ITF traces are a well-defined JSON trace shape used by external model checkers. The adapter must convert them without letting ITF become canonical IR.

## Decisions

### Decision: ITF is an adapter input, never canonical IR

**Choice:** Preserve ITF source bytes as a source envelope and require adapter conversion into canonical Preserves values. ITF JSON authority ends at the adapter boundary.

**Rationale:** Preserves remains the canonical encoded IR candidate; JSON is a compatibility projection only.

### Decision: Layers keep independent identities

**Choice:** Record separate typed identities for ITF version, parser/tool identity, ITF source bytes, converted Preserves value, adapter descriptor, schema refs, and receipt. No derived artifact inherits a losslessness or semantic-equivalence claim.

**Rationale:** Layer separation is Kamacite's central honesty boundary.

### Decision: Conversion runs in pure core

**Choice:** Implement ITF value mapping, schema validation, identity binding, and receipt construction as pure deterministic logic. File reads and hashing of source bytes run in a thin shell.

**Rationale:** Kamacite core must remain testable without files, processes, or ambient authority.

### Decision: Admission fails closed

**Choice:** Reject malformed, stale-version, schema-mismatched, or tampered traces with deterministic denials. No partial conversion is admitted as a trace.

**Rationale:** Partial or guessed conversions silently corrupt evidence.

### Decision: Non-overclaim is explicit

**Choice:** Every adapter receipt carries the non-claim that it proves only the declared byte-level conversion and identity binding, not semantic equivalence, checker correctness, or trace semantic validity.

**Rationale:** The interchange boundary must not mutate meaning silently.

## Functional core / imperative shell split

- **Pure core**: descriptor validation, ITF version admission, value mapping, schema validation, identity binding, receipt construction and validation, and denials.
- **Imperative shell**: read and hash ITF source bytes, persist receipts, and export bounded Valence projections.

## Risks / Trade-offs

- ITF evolves between checker versions. Bind the ITF version and parser/tool identity explicitly and treat upgrades as compatibility changes.
- JSON decoding of foreign traces can be surprising. Fail closed on unknown fields that cross a schema boundary.
- Conversion cannot prove that Preserves meaning equals ITF meaning. Keep the equivalence claim out of the receipt.

## Non-Goals

- No ITF canonical IR and no Preserves-claims-ITF semantics.
- No checker execution, no trace generation, and no model-check authority in Kamacite.
- No general JSON schema engine; only the bounded ITF adapter surface.
- No claim of losslessness beyond the declared schema mapping.
