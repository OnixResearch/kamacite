## Context

Several public APIs either synthesize alleged Preserves bytes from deterministic JSON or accept opaque canonical bytes beside independently supplied typed fields. Hash equality proves only byte equality in those paths; it does not prove schema validity or that validated fields belong to the hashed artifact.

## Decisions

### 1. One canonical value feeds validation and identity

**Choice:** Decode supported Preserves input into a closed typed value, validate that value, and emit canonical Preserves binary from the same value for BLAKE3 identity.

**Rationale:** This removes the split-brain state in which DTO fields and identity bytes can disagree.

### 2. Canonicalization is a pure bounded core

**Choice:** Keep parsing, normalization, schema dispatch, encoding, and hash-material construction in `kamacite-core`. Pass named depth, collection, string, and byte limits as explicit policy inputs.

**Rationale:** Canonical identity remains deterministic and testable without filesystem or evaluator state while hostile inputs cannot force unbounded work.

### 3. Compatibility projections carry canonical identity, never authority labels alone

**Choice:** JSON projections include the canonical artifact identity and versioned codec, and validation requires either canonical bytes or a previously validated canonical artifact object.

**Rationale:** A string claiming `preserves-canonical-binary` is not evidence of the codec used.

### 4. Migration is versioned and reviewable

**Choice:** Introduce new codec/schema versions, generate an old/new identity migration report, and classify legacy prefixed-JSON identities as recorded compatibility evidence only.

**Rationale:** Silently changing hash material would corrupt downstream linkage.

## Risks / Trade-offs

- Real canonicalization changes many golden identities and requires coordinated fixture migration.
- Preserves implementation selection becomes trusted codec surface and needs differential/property tests.
- Bounded decoding may reject previously accepted oversized fixtures; limits must be named policy, not magic numbers.
