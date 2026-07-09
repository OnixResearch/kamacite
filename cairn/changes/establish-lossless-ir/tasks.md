## Phase 1: Foundation Cairns

- [x] [serial] Bootstrap the Braid repository with native Cairn layout and repo-local guidance. r[braid.lossless_ir.format_strategy]
- [x] [serial] Document the language naming boundary, Preserves-versus-JSON format decision, and losslessness classes. r[braid.lossless_ir.language_boundary] r[braid.lossless_ir.losslessness_classes]
- [x] [serial] Add the initial lossless IR spec with adapter boundaries, evidence requirements, and non-claims. r[braid.lossless_ir.adapter_boundaries] r[braid.lossless_ir.non_claims]
- [x] [serial] Split the initial Cairn delta into focused specs for core IR, source envelopes, adapter boundaries, identity model, and stack integration. r[braid.lossless_ir.format_strategy] r[braid.lossless_ir.source_preservation] r[braid.lossless_ir.steel_adapter_boundary] r[braid.lossless_ir.content_addressed_identity] r[braid.lossless_ir.stack_evidence_boundary]
- [x] [serial] Validate the initial Cairn package with proposal, design, tasks, and repo validation gates. r[braid.lossless_ir.round_trip_evidence]

## Phase 2: First implementation slice

- [ ] [serial] Define the first Preserves schema or equivalent fixture vocabulary for source envelopes and canonical values. r[braid.lossless_ir.format_strategy]
- [ ] [serial] Implement a pure Rust core model for source envelopes, BLAKE3 source identity, artifact classes, and bounded non-claim receipts. r[braid.lossless_ir.source_preservation]
- [ ] [parallel] Add Nickel positive and negative fixtures that distinguish source envelopes from exported projections and record the Nickel evaluator identity. r[braid.lossless_ir.adapter_boundaries.nickel_projection] r[braid.lossless_ir.nickel_reference_boundary.reference_eval]
- [ ] [parallel] Add tree-sitter Nickel span fixtures that support source indexing without claiming evaluation semantics. r[braid.lossless_ir.nickel_reference_boundary.parser_spans]
- [ ] [parallel] Add schema-generated Nickel contract fixtures, including generated-corpus stress cases, that remain fixture inputs rather than Braid schema authority. r[braid.lossless_ir.nickel_reference_boundary.generated_contracts]
- [ ] [parallel] Add Steel positive and negative fixtures that prove DTO-only evaluation and fail closed for undeclared host authority. r[braid.lossless_ir.adapter_boundaries.steel_no_ambient_authority]
- [ ] [parallel] Add Steel host-function and module-contract fixtures that bind evaluator identity, registered functions, callable descriptors, effect classes, and non-claims. r[braid.lossless_ir.steel_adapter_boundary.host_functions] r[braid.lossless_ir.steel_adapter_boundary.contract_exports]
- [ ] [parallel] Add Steel ambient-library negative fixtures for filesystem, process, TCP, random, time, package, dylib, credential, and mutation requests. r[braid.lossless_ir.steel_adapter_boundary.ambient_libraries]
- [ ] [parallel] Add canonical value fixtures that round-trip through Preserves and reject stale hashes or lossy JSON projections. r[braid.lossless_ir.round_trip_evidence]
- [ ] [parallel] Add WASM target fixtures that prove adapter receipts stay Preserves-backed and reject compiled WASM as canonical IR. r[braid.lossless_ir.wasm_target_boundary]
- [ ] [parallel] Add LLVM target-artifact fixtures that bind compiler identity, target triple, optimization profile, debug-info status, and BLAKE3 artifact hashes while rejecting LLVM as canonical IR. r[braid.lossless_ir.llvm_target_boundary]
- [ ] [parallel] Add content-addressed identity fixtures proving names/paths are metadata and canonical bytes own identity. r[braid.lossless_ir.content_addressed_identity.names_metadata]
- [ ] [parallel] Add dependency-closure receipt fixtures that bind adapter/evaluator identities and dependency hashes. r[braid.lossless_ir.content_addressed_identity.dependency_closure]
- [ ] [parallel] Add effect/capability label fixtures that fail closed on undeclared host effects. r[braid.lossless_ir.content_addressed_identity.effect_labels]
- [ ] [parallel] Add durable-storage envelope fixtures that validate schema, codec, content hash, and non-claims before decode. r[braid.lossless_ir.content_addressed_identity.durable_storage]
- [ ] [parallel] Add Valence export fixtures for Braid receipts with stable IDs, spans, roles, hashes, and non-claims. r[braid.lossless_ir.stack_evidence_boundary.valence_receipts]
- [ ] [parallel] Add an Octet check plan for future Rust crates that classifies Octet output as source-shape/provenance evidence only. r[braid.lossless_ir.stack_evidence_boundary.octet_checks]
- [ ] [serial] Add a focused validation command that runs all positive and negative fixtures and emits a deterministic receipt. r[braid.lossless_ir.round_trip_evidence]
