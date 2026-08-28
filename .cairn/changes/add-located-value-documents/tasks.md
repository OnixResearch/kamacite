## Phase 1: Baseline and schemas

- [ ] [serial] Run baseline workspace, adapter, and lifecycle tests before core changes. r[kamacite.located_values.fixtures]
- [ ] [serial] Add versioned Preserves schemas for paths, origins, located documents, and diagnostics. r[kamacite.located_values.schemas]
- [ ] [serial] Add the isolated pure component and named path, span, collection, and diagnostic limits. r[kamacite.located_values.boundaries]

## Phase 2: Paths and replacement

- [ ] [serial] Implement schema-bound path admission and deterministic lookup. r[kamacite.located_values.paths]
- [ ] [parallel] Add negative path tests for wrong schemas, kinds, indexes, key identities, and bounds. r[kamacite.located_values.paths.invalid]
- [ ] [serial] Implement identity-guarded immutable replacement and result identity construction. r[kamacite.located_values.lookup_replace]
- [ ] [parallel] Add stale-root, stale-target, mismatch, and law-based replacement tests. r[kamacite.located_values.lookup_replace.stale]

## Phase 3: Origins and diagnostics

- [ ] [serial] Implement exact, derived, and explicitly absent source origins. r[kamacite.located_values.source_origins]
- [ ] [parallel] Add computed, merged, generated, wrapper, and absent-origin fixtures. r[kamacite.located_values.source_origins.derived]
- [ ] [serial] Add bounded structured diagnostics with paths, origins, locator identities, optional parser identities, and redaction metadata. r[kamacite.located_values.diagnostics]
- [ ] [serial] Add delayed location refinement over one captured byte snapshot. r[kamacite.located_values.diagnostics.snapshot]
- [ ] [parallel] Add negative evidence for changed snapshots, invalid spans, secret leakage, and parser overclaims. r[kamacite.located_values.diagnostics.snapshot]

## Phase 4: Adapters and closeout

- [ ] [serial] Add Nickel and Steel adapter projections without moving evaluator authority into the core. r[kamacite.located_values.boundaries]
- [ ] [parallel] Add Valence projections that preserve span identities, roles, and non-claims. r[kamacite.located_values.boundaries.valence]
- [ ] [serial] Add positive, negative, property, and golden receipt fixtures. r[kamacite.located_values.fixtures]
- [ ] [serial] Document the contracts and record the exact upstream design references and licenses. r[kamacite.located_values.schemas]
- [ ] [serial] Run final workspace tests, adapter checks, Clippy, Octet, Cairn gates, and `nix flake check -L`. r[kamacite.located_values.fixtures]
