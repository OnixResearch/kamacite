## Context

Kamacite already pins Schema Migration Core revision `4fe90e130f2871cf69a6febcdc70785adca98aea`. That component owns pure catalog admission, bounded path selection, plan identity, terminal decisions, typed effect requests, freshness, and metadata-only planner receipts.

Kamacite owns schema-version policy at its interchange boundary. It also owns Preserves encoding, Kamacite receipt identity, adapter diagnostics, and claim limits. The component requires Rust 1.95, while the root Kamacite workspace retains Rust 1.88 compatibility. The current separate pilot package preserves that boundary.

## Decisions

### Decision: Reuse the published planner

**Choice:** The pilot adapter delegates planning to the exact pinned `schema-migration-core` revision. It does not copy catalog, graph, path, freshness, or effect-request logic into Kamacite.

**Rationale:** The published component already owns these product-neutral mechanics and tests them across independent consumers.

### Decision: Keep adjacent-version policy in Kamacite

**Choice:** A versioned catalog entry binds one shared migration entry to nonzero source and target version values. The adapter accepts only a forward edge from `n` to `n + 1` or a reverse edge from `n + 1` to `n`. It rejects skipped versions, direction mismatch, overflow, and inconsistent schema-to-version bindings before shared planning.

**Rationale:** Exact schema identities do not define product version order. Kamacite must supply that policy without changing shared planning meaning.

### Decision: Emit a canonical Kamacite receipt projection

**Choice:** The adapter projects the complete public shared `PlanReceipt` fields into `kamacite.schema-migration-plan-receipt.v1`. It encodes the projection with the existing typed Preserves codec and derives a normal Kamacite `ReceiptIdentity` from the packed bytes.

**Rationale:** The shared plan identity remains authoritative for planner meaning. The Kamacite identity names the exact Preserves projection and its non-claims.

### Decision: Keep effect requests non-authorizing

**Choice:** The pilot can preserve shared typed effect requests and freshness decisions as planning facts. It executes no migration artifact and performs no storage, transaction, authorization, rollback, compensation, or publication effect.

**Rationale:** Effect requests describe work. They do not grant authority or prove execution.

### Decision: Keep the adapter in the isolated pilot

**Choice:** The root `kamacite-core` workspace does not gain a Schema Migration Core dependency in this change. The adapter remains in `pilots/schema-migration/` until the dependency cohort and Kamacite MSRV can be reviewed together.

**Rationale:** Product adoption must not silently raise the root Rust 1.88 contract to Rust 1.95.

## Risks / Trade-offs

- The pilot validates version adjacency from caller-supplied version facts. It does not discover schema order.
- A valid plan or canonical receipt can still name an incorrect converter or produce semantically wrong data.
- The isolated pilot does not expose a production Kamacite CLI command or execution shell.
- A later graduation change must review MSRV, dependency publication, product fixture adoption, and execution authority separately.
