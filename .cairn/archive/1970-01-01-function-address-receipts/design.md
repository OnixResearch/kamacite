## Context

Kamacite already uses Unison-inspired identity discipline: canonical content owns identity and names are metadata. Function-address receipts fit that model, but the receipt must not become Rust-specific. Octet should contribute Rust source-shape facts; Kamacite should preserve those facts as a typed portable artifact; Valence should wrap them as Evidence IR; Mantle should bind the resulting evidence to release artifacts.

The first receipt should distinguish the local function address from stronger dependency-closure identity. This keeps the portable format useful for every supported function while making unresolved dependencies explicit.

## Decisions

### 1. Make Preserves canonical and JSON a projection

**Choice:** Define the function-address receipt as a Kamacite canonical value with Preserves bytes as the authoritative identity input. JSON fixtures and exports remain deterministic compatibility projections and must bind back to the Preserves/canonical receipt identity.

**Rationale:** Kamacite's losslessness boundary depends on canonical bytes, not lossy JSON projection shape.

### 2. Separate source envelopes, functions, and dependency closure

**Choice:** A receipt contains source-envelope records, function records, dependency-edge records, and caveat records as separate identity domains. `function_address` identifies local canonical function content; `closure_identity` is optional and may be absent when dependency identities are incomplete.

**Rationale:** This mirrors Unison discipline without claiming Rust or any other language is fully content-addressed by Kamacite.

### 3. Treat producer names and locators as metadata

**Choice:** Human symbols, paths, labels, and aliases are searchable metadata. Receipt identity is computed from canonical fields and explicitly included identity inputs, not display names alone.

**Rationale:** Names change during refactors. The function-address receipt must remain stable when only metadata changes.

### 4. Keep Octet interop artifacts-first

**Choice:** Kamacite validates Octet-produced function-address receipt fixtures and producer metadata without taking a direct Octet crate dependency in the first slice. Direct API coupling requires a later accepted change.

**Rationale:** Receipt artifacts are enough to stabilize the interchange contract while keeping repo ownership boundaries clean.

### 5. Export Valence-friendly evidence without broadening claims

**Choice:** Kamacite provides a derived Valence-friendly projection that binds the Kamacite receipt identity and preserves non-claims. It does not call live Valence APIs or become canonical Evidence IR by itself.

**Rationale:** Valence can consume the projection while Kamacite continues to own the source/value interchange contract.

## Risks / Trade-offs

- Function canonicalizers may change. The receipt must bind producer and canonicalizer identities so drift is visible.
- Dependency closure is useful but can be incomplete. The receipt must report unknown or unresolved dependencies instead of hiding them.
- A portable function-address receipt is identity evidence, not a language semantics layer. Documentation and validation must reject stronger claims.
