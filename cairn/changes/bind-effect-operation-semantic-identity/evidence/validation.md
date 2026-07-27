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

The final Nix flake result, accepted-spec synchronization receipt, traceability receipt, and archive receipt are recorded after those commands complete.

## Downstream evidence blocker

No Aspen/Molten runtime adoption receipt exists for this producer revision. This absence remains a downstream integration blocker. It does not invalidate the Kamacite-owned canonical identity, fixture, migration, and projection checks.

## Non-claims

These checks do not prove default-behavior correctness, handler correctness, semantic equivalence, replay truth, host authorization, runtime correctness, Valence verifier soundness, downstream integration, or release eligibility.
