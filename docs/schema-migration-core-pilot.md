# Schema Migration Core pilot

Kamacite pins Schema Migration Core revision `4fe90e130f2871cf69a6febcdc70785adca98aea` through its canonical Radicle adapter.

The shared component owns immutable catalog admission, bounded path selection, plan identity, terminal decisions, typed effect requests, freshness, and metadata-only planner receipts. Kamacite does not copy that planning logic.

The pilot adds product-owned admission for nonzero adjacent version edges. A forward edge moves from `n` to `n + 1`. A reverse edge moves from `n + 1` to `n`. Each direction is an independent declared capability. The adapter rejects skipped versions, direction mismatch, overflow, and conflicting schema-version bindings before shared planning.

The pilot projects each shared terminal receipt into `kamacite.schema-migration-plan-receipt.v1`. Preserves packed bytes are canonical for this projection. A Kamacite receipt identity uses BLAKE3 over those bytes. The shared plan identity remains the identity of planner meaning.

The pilot stays outside the root Rust workspace. Schema Migration Core requires Rust 1.95, while the root Kamacite package contract remains Rust 1.88. A later graduation change must review this cohort difference before product adoption.

## Evidence boundary

The focused rail covers the complete Nickel-generated shared conformance set. It also covers adjacent upgrades, declared downgrades, bounded chains, canonical receipt stability, skipped versions, direction mismatch, inconsistent version bindings, denials, and unknown scenarios.

The pilot executes no adapter, migration, file, process, storage, transaction, rollback, compensation, publication, or authorization effect.

A valid plan or receipt does not prove converter correctness, successful execution, atomicity, restored external effects, semantic equivalence, release eligibility, or authority.

Kamacite retains source envelopes, Preserves identity, schema vocabulary, adapter diagnostics, and interchange receipts.
