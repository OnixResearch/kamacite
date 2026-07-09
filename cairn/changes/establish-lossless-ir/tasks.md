## Phase 1: Foundation Cairns

- [x] [serial] Bootstrap the Braid repository with native Cairn layout and repo-local guidance. r[braid.lossless_ir.format_strategy]
- [x] [serial] Document the Preserves-versus-JSON format decision and losslessness classes. r[braid.lossless_ir.losslessness_classes]
- [x] [serial] Add the initial lossless IR spec with adapter boundaries, evidence requirements, and non-claims. r[braid.lossless_ir.adapter_boundaries] r[braid.lossless_ir.non_claims]
- [x] [serial] Validate the initial Cairn package with proposal, design, tasks, and repo validation gates. r[braid.lossless_ir.round_trip_evidence]

## Phase 2: First implementation slice

- [ ] [serial] Define the first Preserves schema or equivalent fixture vocabulary for source envelopes and canonical values. r[braid.lossless_ir.format_strategy]
- [ ] [serial] Implement a pure Rust core model for source envelopes, BLAKE3 source identity, artifact classes, and bounded non-claim receipts. r[braid.lossless_ir.source_preservation]
- [ ] [parallel] Add Nickel positive and negative fixtures that distinguish source envelopes from exported projections. r[braid.lossless_ir.adapter_boundaries.nickel_projection]
- [ ] [parallel] Add Steel positive and negative fixtures that prove DTO-only evaluation and fail closed for undeclared host authority. r[braid.lossless_ir.adapter_boundaries.steel_no_ambient_authority]
- [ ] [parallel] Add canonical value fixtures that round-trip through Preserves and reject stale hashes or lossy JSON projections. r[braid.lossless_ir.round_trip_evidence]
- [ ] [parallel] Add WASM target fixtures that prove adapter receipts stay Preserves-backed and reject compiled WASM as canonical IR. r[braid.lossless_ir.wasm_target_boundary]
- [ ] [parallel] Add Valence export fixtures for Braid receipts with stable IDs, spans, roles, hashes, and non-claims. r[braid.lossless_ir.stack_evidence_boundary.valence_receipts]
- [ ] [parallel] Add an Octet check plan for future Rust crates that classifies Octet output as source-shape/provenance evidence only. r[braid.lossless_ir.stack_evidence_boundary.octet_checks]
- [ ] [serial] Add a focused validation command that runs all positive and negative fixtures and emits a deterministic receipt. r[braid.lossless_ir.round_trip_evidence]
