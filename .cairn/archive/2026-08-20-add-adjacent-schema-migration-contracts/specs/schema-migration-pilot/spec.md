# Schema Migration Pilot Specification Delta

## ADDED Requirements

### Requirement: Kamacite reuses the pinned shared planner

r[kamacite.schema_migration.shared_planner] The Kamacite schema-migration pilot MUST consume the reviewed immutable Schema Migration Core revision for catalog admission, bounded path selection, plan identity, terminal decisions, effect requests, freshness, and planner receipts. Kamacite MUST NOT duplicate those planning mechanics.

#### Scenario: Admitted catalog enters shared planning

r[kamacite.schema_migration.shared_planner.delegate]
- GIVEN Kamacite admits every product-owned version binding
- WHEN the pilot requests migration planning
- THEN the exact pinned shared planner MUST produce the terminal planning outcome
- AND Kamacite MUST NOT execute any requested effect.

### Requirement: Adjacent version policy is directional and fail-closed

r[kamacite.schema_migration.adjacency] Each Kamacite pilot entry MUST bind nonzero source and target version values to one shared migration entry. A forward entry MUST move from `n` to `n + 1`, and a reverse entry MUST move from `n + 1` to `n`. The adapter MUST reject skipped versions, overflow, direction mismatch, and inconsistent schema-to-version bindings before shared planning.

#### Scenario: Adjacent forward entry passes admission

r[kamacite.schema_migration.adjacency.forward]
- GIVEN a forward shared entry binds version one to version two
- WHEN Kamacite admits the versioned catalog
- THEN the entry MUST remain eligible for shared planning.

#### Scenario: Declared reverse entry passes admission

r[kamacite.schema_migration.adjacency.reverse]
- GIVEN a reverse shared entry binds version two to version one
- WHEN Kamacite admits the versioned catalog
- THEN the reverse entry MUST remain eligible under a policy that allows reverse planning.

#### Scenario: Skipped version fails admission

r[kamacite.schema_migration.adjacency.skipped]
- GIVEN one entry binds version one directly to version three
- WHEN Kamacite admits the versioned catalog
- THEN admission MUST fail before shared planning.

#### Scenario: One schema has conflicting versions

r[kamacite.schema_migration.adjacency.consistency]
- GIVEN the catalog binds one exact schema identity to two version values
- WHEN Kamacite admits the versioned catalog
- THEN admission MUST fail with a deterministic version-binding diagnostic.

### Requirement: Planner receipts have canonical Kamacite projections

r[kamacite.schema_migration.receipt] The pilot MUST project every shared terminal `PlanReceipt` into the versioned `kamacite.schema-migration-plan-receipt.v1` Preserves artifact. The projection MUST bind the exact planner revision, decision, reason, available plan and route identities, selected path facts, bounded ambiguity candidates, and mandatory non-claims. Its Kamacite identity MUST use BLAKE3 over canonical packed Preserves bytes.

#### Scenario: Selected plan receipt is stable

r[kamacite.schema_migration.receipt.stable]
- GIVEN the same selected shared planning outcome and adapter revision
- WHEN Kamacite identifies the receipt twice
- THEN the canonical bytes and Kamacite receipt identity MUST be equal.

#### Scenario: Denial receipt preserves terminal context

r[kamacite.schema_migration.receipt.denial]
- GIVEN shared planning returns ambiguity, bound exhaustion, precondition denial, no path, or invalid catalog
- WHEN Kamacite projects the terminal receipt
- THEN the projection MUST preserve the available exact bindings and stable reason
- AND it MUST NOT fabricate a plan identity.

### Requirement: Planning effects do not grant execution authority

r[kamacite.schema_migration.pilot_boundary] The pilot MUST treat shared effect requests as non-authorizing plans. It MUST NOT execute migration code, read or mutate product data, open transactions, authorize changes, perform rollback or compensation, publish artifacts, or claim semantic correctness. The adapter MUST remain isolated from the Rust 1.88 root workspace while its pinned component requires Rust 1.95.

#### Scenario: Selected plan remains effect-free

r[kamacite.schema_migration.pilot_boundary.effect_free]
- GIVEN shared planning returns ordered effect requests
- WHEN the Kamacite pilot reports the plan and receipt
- THEN no migration, storage, transaction, rollback, compensation, or publication effect MUST occur.

#### Scenario: Root MSRV remains unchanged

r[kamacite.schema_migration.pilot_boundary.msrv]
- GIVEN the pinned shared component requires Rust 1.95
- WHEN the pilot adapter is built and tested
- THEN the root Kamacite workspace MUST retain its Rust 1.88 package contract.

### Requirement: The pilot has positive and negative conformance evidence

r[kamacite.schema_migration.conformance] The focused rail MUST cover adjacent forward planning, declared reverse planning, bounded multi-step planning, canonical receipt stability, all embedded shared conformance vectors, skipped versions, direction mismatch, inconsistent version bindings, stale planning facts, ambiguity, bound failures, precondition failures, invalid catalogs, and unknown scenarios.

#### Scenario: Focused migration rail passes

r[kamacite.schema_migration.conformance.focused]
- GIVEN the pinned source, adapter, schemas, tests, and documentation are current
- WHEN the focused pilot rail runs
- THEN every positive and negative case MUST pass with exact non-claims.
