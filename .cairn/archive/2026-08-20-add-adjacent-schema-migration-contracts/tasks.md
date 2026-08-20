## Phase 1: Adapter contract

- [x] [serial] [task:contract] Reconcile the prior draft with the pinned Schema Migration Core ownership boundary and record the isolated-pilot MSRV decision. r[kamacite.schema_migration.shared_planner] r[kamacite.schema_migration.pilot_boundary]
- [x] [serial] [task:adjacency] [after:contract] Implement nonzero adjacent-version admission, direction checks, and deterministic schema-version consistency checks around shared migration entries. r[kamacite.schema_migration.adjacency]
- [x] [serial] [task:receipt] [after:adjacency] Project shared terminal planner receipts into canonical Preserves bytes with a Kamacite receipt identity and exact non-claims. r[kamacite.schema_migration.receipt]

## Phase 2: Evidence and completion

- [x] [serial] [task:fixtures] [after:receipt] Add positive and negative adapter tests plus the complete embedded shared conformance rail. r[kamacite.schema_migration.conformance]
- [x] [serial] [task:docs] [after:fixtures] Add the Preserves schema, typed Nickel source contract, and pilot documentation with execution and authority non-claims. r[kamacite.schema_migration.receipt] r[kamacite.schema_migration.pilot_boundary]
- [x] [serial] [task:validate] [after:docs] Run focused tests, strict Clippy, workspace validation, lifecycle gates, and `nix flake check -L`; then sync and archive the accepted requirements. r[kamacite.schema_migration.conformance]
