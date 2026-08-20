# Validation evidence

## Baseline

- The isolated pilot passed its original three tests before implementation.
- Native lifecycle validation passed before implementation.

## Focused adapter evidence

- The final pilot rail passed 15 tests.
- Positive cases cover forward, reverse, multi-step, and stable canonical receipt behavior.
- Negative cases cover zero, overflow, skip, direction mismatch, schema-version conflicts, ambiguity, bounds, no path, stale facts, invalid catalogs, preconditions, and unknown scenarios.
- The complete embedded Schema Migration Core conformance set passed.
- Strict pilot Clippy passed with the Kamacite canonical-determinism flags.
- Nickel type checking passed for `pilots/schema-migration/source.ncl`.
- The Nix `schema-migration-core-pilot-source` check passed.

## Repository evidence

- Workspace formatting, tests, and strict Clippy passed.
- Canonical fixture validation passed with all golden receipts matched.
- The dry-run Preserves migration report passed without mutation.
- Native lifecycle validation and proposal, design, and task gates passed.
- `nix flake check path:$PWD -L` reported `all checks passed` for `x86_64-linux`.

## Traceability limitation

The extra repository-wide default traceability command remains unavailable evidence. It reports 703 accepted requirements, 47 referenced requirements, and a historical missing-marker backlog across many existing specifications. The current profile scans `crates/` and `tools/`, not the isolated `pilots/` tree.

This change does not weaken that profile or claim repository-wide traceability coverage. The change-owned implementation and test markers remain in the pilot sources for a later focused profile or policy migration.

## Non-claims

These checks do not prove migration implementation correctness, execution, atomicity, rollback success, semantic equivalence, authorization, storage safety, or release eligibility.
