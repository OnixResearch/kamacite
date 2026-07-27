# Semantic effect-operation identity validation

<!-- r[verify kamacite.algebraic_effects.semantic_identity_fixtures] -->

Date: 2026-07-27

## Baseline

Before semantic identity implementation, 115 `kamacite-core` tests passed. The existing fixture manifest and packaged lifecycle validator also passed. The original checkout remained unchanged in its separate jj workspace.

## Implemented boundary

Kamacite now derives version 2 operation identities from canonical packed Preserves descriptors. The descriptor binds nominal effect family, parameter and result schemas, continuation contract, replay class, normalized dependencies, and one default behavior or abstract marker.

The implementation keeps names and source metadata outside identity. Typed reference bundles cover rows, summaries, handlers, receipts, logs, adapters, target imports, and Wasm imports. Directional compatibility and Valence projections preserve exact roles and non-claims.

The capability-rooted `kamacite-semantic-operation` CLI supports identification and dry-run migration. Mutation requires both `--update` and a bounded output path. Legacy operation IDs remain separate values and are never reinterpreted.

## Checks

- Task 814 passed workspace formatting and strict Clippy, including the canonical determinism restriction set.
- Task 817 passed workspace tests, canonical fixture validation, and the dry-run Preserves migration rail.
- Task 818 passed Cairn validation and proposal, design, and tasks gates.
- Task 819 passed `nix flake check -L`; Nix reported `all checks passed` for `x86_64-linux`.
- The focused semantic CLI suite passed positive identification and migration cases plus negative incomplete descriptor and unbounded update cases.
- The core suite passed semantic re-keying, exact matching, abstract operation, malformed identity, wrong role, stale reference, compatibility direction, and host-context boundary cases.

Accepted specification synchronization passed with receipt `e47c07b5c82e9d37a23afe8e27a6b58f84410de554501dc0b14d56fa7f37d49e`.

The focused profile covers 11 of 11 requirements. Its receipt is `2b8ba3897da2d6c572a18e82e85917a2900920d1559308166da7f27acdaa17a9`.

The repository-wide default traceability profile remains unavailable evidence because Kamacite adopted a native policy during this change, while historical accepted specifications do not yet have complete default-profile evidence markers. This change does not claim that migration.

Archive execution passed with receipt `5967df03814cf575d262a3be0280e502d17693ef9b9f405c0ed0a2b4a1f35e2c`.

## Downstream evidence blocker

No Aspen/Molten runtime adoption receipt exists for this producer revision. This absence remains a downstream integration blocker. It does not invalidate the Kamacite-owned canonical identity, fixture, migration, and projection checks.

## Non-claims

These checks do not prove default-behavior correctness, handler correctness, semantic equivalence, replay truth, host authorization, runtime correctness, Valence verifier soundness, downstream integration, or release eligibility.
