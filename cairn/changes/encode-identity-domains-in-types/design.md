# Design: Encode identity domains in Rust types

## Context

Kamacite owns several canonical identity domains over common BLAKE3 text. The current `Identity` model stores the domain beside the digest and checks expected domains at runtime.

That model is necessary at interchange boundaries. It is weaker than a nominal Rust API because all admitted identities remain exchangeable at compile time.

## Decisions

### Decision: Add a generic nominal identity

**Choice:** Add `Identity<Domain>` with private `Blake3Hex` storage and a zero-sized role marker. Add `IdentityRef<'a, Domain>` for borrowed adapter paths.

Define aliases for source envelope, canonical value, projection, target artifact, adapter, evaluator, compiler, receipt, effect family, effect operation, handler, and compatibility artifact identities.

**Rationale:** The marker prevents cross-domain calls while one implementation owns validation and formatting.

### Decision: Keep wire domains explicit

**Choice:** Retain a versioned wire record with the serialized domain and algorithm fields. Convert it through checked admission into a closed `AnyIdentity` enum.

Known-role core APIs accept exact aliases. Heterogeneous manifests must pattern-match the enum.

**Rationale:** Preserves and JSON need explicit domain tags. Internal APIs need compile-time separation.

### Decision: Preserve canonical authority

**Choice:** Rust newtypes do not enter canonical identity material unless the existing schema already includes their represented domain and digest. Add byte-for-byte golden checks before migration.

**Rationale:** Preserves bytes remain authoritative. Rust type structure is an implementation boundary only.

### Decision: Keep generic identity free of effect semantics

**Choice:** The generic operation-identity alias stores only the validated identity for its domain. The active semantic operation descriptor owns family, behavior, continuation, replay, and dependency meaning.

**Rationale:** Generic identity mechanics must not absorb algebraic-effect semantics.

### Decision: Migrate by artifact family

**Choice:** Migrate source envelopes and canonical values first. Then migrate adapters and targets. Migrate receipts, effects, documentation, and Valence projections after compatibility fixtures pass.

**Rationale:** A staged migration keeps fixture churn reviewable and isolates regressions.

### Decision: Compile-test domain separation

**Choice:** Add compile-fail fixtures for source-to-receipt, adapter-to-target, evaluator-to-compiler, value-to-projection, and effect-family-to-operation substitutions.

Add compile-pass fixtures for same-domain owned, borrowed, parsed, and serialized flows.

**Rationale:** These fixtures demonstrate the new compiler guarantee directly.

## Functional core and shell boundary

Domain parsing, typed conversion, canonical projection, comparison, and diagnostics remain pure over in-memory values.

Manifest file reads, Nickel and Steel execution, Wasm inspection, filesystem confinement, and golden updates remain in `kamacite-cli`.

## Test design

Positive tests cover every domain alias, accepted BLAKE3 spelling, borrowed views, `AnyIdentity` round trips, unchanged canonical bytes, and staged family migrations.

Negative tests cover malformed digests, unknown domains, unsupported algorithms, cross-domain wire records, stale identity versions, role promotion, compile-time substitution, and weakened non-claims.

## Risks and trade-offs

- The domain catalog can grow. Marker types and aliases keep expansion explicit.
- Heterogeneous artifacts retain runtime matching. The closed enum keeps that matching exhaustive.
- Public Rust API changes can affect exact-revision consumers. Compatibility adapters and migration notes reduce disruption.
- Typed identities can look authoritative. Receipts and docs must keep authority non-claims visible.

## Claim boundary

The change prevents selected internal Rust category errors and centralizes domain parsing. It does not prove source fidelity, adapter laws, semantic equivalence, authority, or release eligibility.
