# Semantic effect-operation identity

<!-- r[impl kamacite.algebraic_effects.semantic_operation_descriptor] -->
<!-- r[impl kamacite.algebraic_effects.semantic_operation_names] -->
<!-- r[impl kamacite.algebraic_effects.semantic_operation_rekey] -->
<!-- r[impl kamacite.algebraic_effects.semantic_handler_matching] -->
<!-- r[impl kamacite.algebraic_effects.semantic_operation_compatibility] -->
<!-- r[impl kamacite.algebraic_effects.semantic_identity_boundary] -->

Kamacite owns version 2 semantic effect-operation descriptors as canonical interchange data.

The descriptor identity is BLAKE3 over canonical packed Preserves bytes. The descriptor binds these fields:

- nominal effect-family identity;
- parameter and result schema identities;
- continuation contract;
- replay class;
- normalized dependency identities; and
- exactly one default-behavior identity or abstract-operation marker.

Display names, aliases, paths, source spans, and documentation are separate metadata. A rename does not change the semantic identity.

`semantic_operation_descriptor` rejects a missing implementation choice and a conflicting default-plus-abstract choice. `identify_semantic_operation` validates identity domains, normalizes unordered dependencies, writes canonical Preserves bytes, and derives the `effect-operation` identity.

Handlers and runtime records must use `require_exact_semantic_operation`. Name equality and schema-shape equality do not satisfy this check. Existing version 1 algebraic-effect artifacts keep their declared identity and remain legacy evidence. They do not become version 2 semantic identities by reinterpretation.

## Directional compatibility

<!-- r[impl kamacite.algebraic_effects.semantic_operation_migration] -->

A compatibility artifact binds exact source and target operation identities. It also binds allowed contexts, supporting references, diagnostics, and non-claims. Its identity is BLAKE3 over canonical packed Preserves bytes in the evidence-chain domain.

`admit_semantic_operation_compatibility` checks direction and context. Replay-only compatibility cannot admit reverse substitution or live host execution.

Migration must construct a new descriptor from reviewed source facts. It must record the old identity and new identity as separate values. It must not hash old bytes under the new domain. Mutation of governed fixtures remains an explicit shell action.

## Stack boundary

<!-- r[impl kamacite.algebraic_effects.semantic_stack_consumption] -->
<!-- r[impl kamacite.algebraic_effects.semantic_valence_export] -->
<!-- r[impl kamacite.algebraic_effects.semantic_supporting_evidence] -->

Wasm imports, effect rows, handlers, logs, caches, replay records, and Aspen/Molten runtime evidence can carry the exact Kamacite semantic identity. Each consumer retains its own capability, policy, resource, execution, and lifecycle decisions.

Valence projections must keep descriptor, behavior, compatibility, handler, and runtime-receipt roles separate. Trellis and Octet evidence can support review. They cannot replace the descriptor or transfer evidence from an older identity.

A valid identity does not prove behavior correctness, handler correctness, semantic equivalence, replay truth, host capability, runtime authority, or release eligibility.
