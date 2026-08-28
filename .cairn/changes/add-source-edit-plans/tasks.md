## Phase 1: Dependency and baseline

- [ ] [serial] Complete and admit the located-value path and source-origin contracts. r[kamacite.source_edits.plan]
- [ ] [serial] Run baseline workspace, adapter, publication, and lifecycle tests before core changes. r[kamacite.source_edits.fixtures]
- [ ] [serial] Pin Durable File Publication revision `8e05e74e24b45f752d77145c4455385daaf6d6ab` in Cargo and Nix. r[kamacite.source_edits.publication]

## Phase 2: Plan and application core

- [ ] [serial] Add versioned Preserves schemas for plans, patches, outcomes, and receipts. r[kamacite.source_edits.plan]
- [ ] [serial] Add the isolated pure component with named patch, range, and replacement limits. r[kamacite.source_edits.plan]
- [ ] [serial] Implement exact pre-state admission and byte-preserving patch application. r[kamacite.source_edits.snapshot]
- [ ] [parallel] Add overlap, order, duplicate, stale hash, range, overflow, and size failures. r[kamacite.source_edits.plan.invalid]
- [ ] [parallel] Add no-op identity and unchanged-range property tests. r[kamacite.source_edits.no_op]

## Phase 3: Language adapters

- [ ] [serial] Add a syntax-only Nickel adapter with parser-bound target checks. r[kamacite.source_edits.language_boundary]
- [ ] [parallel] Add Nickel comment, quote, line-ending, stale-input, and parser-drift fixtures. r[kamacite.source_edits.fixtures.nickel]
- [ ] [serial] Add a syntax-only Steel adapter over the same generic patch core. r[kamacite.source_edits.language_boundary]
- [ ] [parallel] Add Steel comment, quote, line-ending, stale-input, and parser-drift fixtures. r[kamacite.source_edits.fixtures.steel]

## Phase 4: Publication and receipts

- [ ] [serial] Publish changed bytes through the pinned one-file publication shell. r[kamacite.source_edits.publication]
- [ ] [parallel] Add collision, cleanup, committed-unknown, unsupported-platform, and no-op publication tests. r[kamacite.source_edits.publication.failure]
- [ ] [serial] Emit new source envelopes and edit receipts with bounded claims. r[kamacite.source_edits.receipt]
- [ ] [parallel] Add overclaim and weakened-non-claim failures. r[kamacite.source_edits.receipt.non_claim]

## Phase 5: Closeout

- [ ] [serial] Add positive, negative, property, and golden receipt evidence for both pilots. r[kamacite.source_edits.fixtures]
- [ ] [serial] Document patch semantics, publication ownership, references, and rollback limits. r[kamacite.source_edits.preservation]
- [ ] [serial] Run final workspace tests, adapter checks, Clippy, Octet, Cairn gates, and `nix flake check -L`. r[kamacite.source_edits.fixtures]
