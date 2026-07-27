## Why

Kamacite already requires effect-operation identity to derive from canonical signature data rather than a human name, but the current model does not require an operation key to commit to its nominal effect family, continuation contract, replay class, and default behavior or explicit abstract marker. A same-named or same-shaped operation can therefore drift semantically without one canonical key forcing handlers, logs, caches, and runtimes to reject the mismatch.

Kamacite is the correct owner because it already owns canonical algebraic-effect artifacts and receipts. Moving this identity into Molten would make the interchange contract runtime-specific; moving it into a generic binding library would confuse effect semantics with artifact binding mechanics.

## What Changes

- Define a versioned semantic effect-operation descriptor whose BLAKE3 identity commits to the effect-family identity, parameter and result schema identities, continuation contract, replay class, dependency identities, and either a default-behavior identity or an explicit abstract-operation marker.
- Keep display names, paths, aliases, and documentation outside semantic operation identity.
- Require handlers, effect rows, summaries, effect logs, receipts, Wasm bindings, and Valence projections to reference exact semantic operation identities.
- Define explicit compatibility evidence for intentional substitution between different operation identities; same name or same schema shape is never sufficient.
- Preserve prior algebraic-effect identities as versioned compatibility artifacts rather than silently reinterpreting them under the new domain.
- Add positive and negative fixtures for rename stability, behavior/schema/continuation/replay re-keying, nominal-family separation, stale identity, missing behavior/abstract markers, and compatibility overclaim.

## Impact

- **Files**: `cairn/changes/bind-effect-operation-semantic-identity/**`; later implementation affects `schemas/preserves/`, `fixtures/`, `crates/kamacite-core/`, `crates/kamacite-cli/`, algebraic-effect and Wasm docs, Valence export fixtures, and manifest/golden identities.
- **Testing**: lifecycle validation and Cairn gates now; implementation requires baseline and post-change workspace tests, positive and negative fixture validation, canonical Preserves migration checks, Valence projection checks, manifest freshness, Clippy, and relevant Nix checks.
