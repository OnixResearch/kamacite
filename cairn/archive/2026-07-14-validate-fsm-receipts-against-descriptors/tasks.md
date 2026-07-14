## Phase 1: Descriptor-bound core

- [x] [serial] Introduce a validated FSM descriptor type produced from canonical decode and spec validation, and require it at receipt validation boundaries. r[kamacite.findings.fsm_receipts.descriptor_binding]
- [x] [serial] Canonically recompute state/event universes, transition/denial coverage, exclusions, and missing cells from the descriptor. r[kamacite.findings.fsm_receipts.coverage_recomputation]
- [x] [serial] Extend receipt schema as needed to bind transition and guard-decision evidence without executing host effects. r[kamacite.findings.fsm_receipts.trace_consistency]

## Phase 2: Receipt validation

- [x] [serial] Validate exhaustiveness roots and matrix claims against the bound descriptor before accepting receipt identity. r[kamacite.findings.fsm_receipts.coverage_recomputation]
- [x] [serial] Validate trace start policy, events, each enabled transition, guard evidence, and final state against the descriptor. r[kamacite.findings.fsm_receipts.trace_consistency]
- [x] [serial] Version and migrate affected FSM receipts and Valence projections with explicit old/new identity evidence. r[kamacite.findings.fsm_receipts.descriptor_binding]

## Phase 3: Regression evidence

- [x] [serial] Add positive descriptor-consistent cases and negative substitution, wrong-start, undeclared-event, invented-transition, guard, stale-root, and missing-cell cases. r[kamacite.findings.fsm_receipts.negative_coverage]
- [x] [serial] Run FSM-focused tests, full fixture/export validation, workspace tests, Clippy, Cairn gates, and Nix checks. r[kamacite.findings.fsm_receipts.coverage_recomputation] r[kamacite.findings.fsm_receipts.negative_coverage]
