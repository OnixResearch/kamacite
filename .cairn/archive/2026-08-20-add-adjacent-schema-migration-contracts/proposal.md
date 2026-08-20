## Why

Kamacite pins Schema Migration Core for deterministic planning, but its pilot only checks one shared fixture and prints the shared plan identity. It does not yet bind product-owned adjacent-version policy or emit a canonical Kamacite receipt.

The earlier draft proposed a second planner and converter shell. Main now has a richer published component boundary. Kamacite must reuse that planner and retain only its own version policy, Preserves projection, identity, diagnostics, and non-claims.

## What Changes

- Add a pure Kamacite adapter that accepts only nonzero, directional, immediately adjacent schema-version edges before shared planning.
- Keep catalog admission, path selection, terminal decisions, effect requests, freshness, and planner identities in the exact pinned Schema Migration Core revision.
- Project shared terminal receipts into a versioned canonical Preserves artifact with a Kamacite receipt identity.
- Expand positive and negative pilot tests across adjacent upgrades, declared downgrades, bounded chains, skipped versions, direction mismatch, inconsistent version bindings, shared conformance vectors, and unknown scenarios.
- Keep migration execution, storage, transactions, authorization, rollback, and semantic correctness outside this pilot.

## Impact

- **Files**: `.cairn/changes/add-adjacent-schema-migration-contracts/`, `pilots/schema-migration/`, `schemas/preserves/`, and `docs/schema-migration-core-pilot.md`.
- **Testing**: focused pilot tests and Clippy, canonical receipt stability tests, lifecycle gates, workspace validation, and `nix flake check -L`.
- **Compatibility**: the root workspace keeps its Rust 1.88 contract. The isolated pilot can consume the pinned Rust 1.95 component without changing Kamacite core dependencies or canonical fixture identities.
