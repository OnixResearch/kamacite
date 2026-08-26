## Context

Molten's world-commit core is a product schema. Kamacite can validate its interchange shape and cross-adapter value agreement, but it must not become the world runtime or reinterpret product semantics.

The profile depends on an immutable implemented Molten revision. Until that exists, this change remains blocked and cannot publish passing conformance claims.

## Decisions

### Decision: Pin one exact product schema cohort

**Choice:** A conformance entry binds the Molten source repository, immutable revision, schema identity, codec identity, canonical domain tag, supported version, fixture manifest, and explicit non-claims.

No sibling path, mutable branch, or implicit latest revision can satisfy the profile.

**Rationale:** Adapter agreement is meaningful only for one exact schema and codec cohort.

### Decision: Keep Molten semantics outside Kamacite

**Choice:** Kamacite validates typed field domains, bounds, canonical Preserves shape, exact bytes, and declared adapter laws. It does not decide capture coherence, ancestry, restore compatibility, branch authority, merge meaning, or effect release.

**Rationale:** Interchange conformance must not transfer runtime ownership.

### Decision: Use canonical Preserves bytes as the shared value boundary

**Choice:** The authoritative Rust fixture provides canonical packed Preserves bytes under the pinned Molten codec. Kamacite admits those bytes into a world-commit dialect descriptor and compares all projections with that value.

JSON remains a review or compatibility projection. Source envelopes preserve Nickel and Steel source identity separately.

**Rationale:** Text and JSON formatting cannot preserve the complete identity boundary.

### Decision: Give each language adapter a narrow role

**Choice:**

- Rust supplies product-owned DTO serialization and canonical fixture bytes.
- Nickel supplies a contract-backed review projection and invalid-shape corpus.
- Steel supplies a fresh bounded value adapter in a no-kernel engine with only declared pure host functions.

The adapters do not execute a world or prove one another's evaluator semantics.

**Rationale:** Independent implementations reduce shared-code risk while preserving honest claim limits.

### Decision: Require exact parity and hostile negative vectors

**Choice:** Positive vectors cover every typed root, parent shape, logical profile, opaque profile reference, detached-evidence exclusion, and collection boundary.

Negative vectors cover malformed and noncanonical Preserves, unknown versions, domain substitution, duplicate and reordered canonical sets, bound overflow, stale schema, embedded signatures, secret values, undeclared Steel authority, and semantic overclaims.

**Rationale:** A codec rail is incomplete if it tests only successful values produced by one implementation.

### Decision: Emit conformance receipts, not compatibility authority

**Choice:** Receipts bind source, schema, codec, conformance entry, adapters, evaluator identities, source envelopes, fixture identities, laws, results, and non-claims.

A passing receipt proves bounded byte and value agreement only. Consumers still pin and validate the Molten contract themselves.

**Rationale:** Kamacite evidence supports review but cannot approve product adoption or runtime use.

### Decision: Make updates explicit and convergent

**Choice:** Default validation is read-only. A schema update requires a new pinned cohort, dry-run difference report, explicit fixture regeneration, convergence check, and review of removed or widened negative cases.

Unknown future versions fail closed.

**Rationale:** Golden updates must not hide breaking product-schema changes.

## Rollout

1. Wait for the implemented Molten core and pin its immutable revision.
2. Import authoritative schemas and fixtures as source envelopes and exact bytes.
3. Add the world-commit dialect and pure validation.
4. Add Nickel review projections and negative contracts.
5. Add the fresh bounded Steel adapter.
6. Publish golden receipts and a consumer verification guide.

## Risks / Trade-offs

- Adapter code can share dependencies and reduce independence. Record dependency identities and shared-code limits.
- Preserves library updates can change bytes. Treat codec identity as part of the cohort.
- A product schema can evolve faster than the fixture corpus. Unknown versions remain blocked until reviewed.
- Cross-language parity can be mistaken for runtime equivalence. Every receipt must preserve the narrower claim.
