## Context

The accepted algebraic-effect model already says operation identity derives from canonical signature data and names remain metadata. The implementation currently carries string operation ids beside schemas and validates whole algebraic-effect artifacts, but it does not yet define one independently canonical descriptor whose identity necessarily changes when nominal family, continuation behavior, replay class, dependencies, or default behavior changes.

That gap matters across handlers, target imports, effect logs, replay, caches, remote execution, and live upgrades. Matching by a local name or same-shaped schema can silently connect callers and handlers compiled against different behavior assumptions.

Ambient's method-key construction is a useful reference because it includes nominal ability identity, canonical signature, and default implementation identity while excluding the method name. Kamacite adapts that identity discipline as canonical data only; it does not adopt Ambient's runtime dispatch or VM.

## Decisions

### Decision: Kamacite owns semantic operation descriptors

**Choice:** Add a versioned canonical Preserves descriptor for one semantic effect operation and derive its BLAKE3 identity under the existing effect-operation identity domain or an explicitly versioned successor domain.

**Rationale:** Kamacite owns effect signatures, rows, handlers, logs, and receipt contracts. The descriptor is portable interchange data, not runtime policy.

**Rejected alternatives:**

- Put the key in Molten: runtime-local identity would fragment the stack contract.
- Put it in `artifact-binding-core`: generic binding mechanics must not interpret algebraic-effect semantics.
- Use Valence stable ids: Valence links evidence but does not define operation semantics.

### Decision: Commit to nominal family and canonical behavior inputs

**Choice:** Identity material includes:

- descriptor schema/version and identity domain;
- nominal effect-family identity;
- parameter and result schema identities;
- continuation contract;
- replay class;
- ordered dependency identities; and
- exactly one of a default-behavior identity or explicit abstract-operation marker.

**Rationale:** Same-shaped operations in different families are distinct, and a change to behavior-relevant inputs re-keys loudly. The behavior ref identifies supplied canonical behavior material; Kamacite validation does not prove that behavior is correct.

### Decision: Exclude review metadata

**Choice:** Display name, aliases, path, source span, docs, and presentation fields remain outside semantic identity while still appearing in enclosing artifacts and receipts.

**Rationale:** Renames and moves should not break handler compatibility when semantics are unchanged.

### Decision: Require exact handler matching

**Choice:** Rows, summaries, handler descriptors, receipts, logs, target imports, and adapter bindings use exact semantic operation identities. Name or shape equality cannot discharge an operation.

**Rationale:** This makes drift fail before a runtime silently chooses an incompatible handler.

### Decision: Compatibility is a separate directional artifact

**Choice:** Define a versioned compatibility artifact that binds exact source and target operation identities, direction, allowed contexts, supporting refs, diagnostics, and non-claims.

**Rationale:** Compatibility can be narrower than equivalence. Replay substitution may be acceptable while live host execution is not. Direction and context must remain explicit.

The compatibility artifact records a reviewed claim shape; it does not prove semantic equivalence or grant authority.

### Decision: Preserve prior identities without reinterpretation

**Choice:** Existing algebraic-effect artifacts remain under their declared versions. Migration constructs new descriptors and emits explicit old/new mappings and changed-field diagnostics. Dry-run is the default; golden mutation remains explicit.

**Rationale:** Reinterpreting old bytes under new identity rules would corrupt evidence continuity.

### Decision: Keep host authority separate

**Choice:** Semantic identity agreement is checked before or alongside existing handler/capability evidence, but it never substitutes for adapter, capability, authority, policy, resource, lifecycle, or runtime receipts.

**Rationale:** An exact symbolic operation match does not authorize the host action that may implement it.

## Risks / Trade-offs

- **Identity churn:** Existing fixtures and consumers will re-key. Mitigate through a versioned migration preview, explicit golden updates, and compatibility artifacts rather than in-place reinterpretation.
- **Behavior-ref overclaim:** A content ref may be mistaken for semantic correctness. Mitigate with explicit non-claims and role-separated Valence export.
- **Abstract operation ambiguity:** Operations without defaults need a canonical marker. Require an explicit abstract marker and reject absent/both-set states.
- **Compatibility sprawl:** Broad compatibility artifacts could recreate name matching. Require directional contexts and negative wrong-direction/live-use fixtures.
- **Consumer lag:** Molten and Wasm adapters may still use local names. Keep legacy artifacts versioned and require exact identity before new semantic-key claims.
