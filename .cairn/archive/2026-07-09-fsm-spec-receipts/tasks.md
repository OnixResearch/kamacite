## Tasks

- [x] [serial] Define Kamacite FSM spec DTOs and Preserves schema artifacts for closed state universes, closed event universes, guard vocabularies, transition rows, denial rows, side-effect classes, checked laws, and non-claims. r[kamacite.lossless_ir.fsm_spec_vocabulary]
- [x] [serial] Define Kamacite FSM exhaustiveness receipt DTOs and schema artifacts with spec identity, coverage identities, adapter identity, dependency identities, diagnostics, checked laws, optional Octet/Valence links, and BLAKE3 receipt identity. r[kamacite.lossless_ir.fsm_receipt_vocabulary]
- [x] [parallel] Add positive fixtures for complete FSM specs, guarded transitions, denial coverage, Octet-linked receipts, and Valence export projections. r[kamacite.lossless_ir.fsm_fixtures.positive]
- [x] [parallel] Add negative fixtures for missing states/events, missing denial rows, duplicate transitions, wildcard/default rows, undeclared guards, stale identities, wrong identity domains, missing non-claims, and overclaims. r[kamacite.lossless_ir.fsm_fixtures.negative]
- [x] [serial] Implement pure validation over parsed FSM spec and receipt values without filesystem, process, network, evaluator, clock, random, mutation, or stdout access. r[kamacite.lossless_ir.fsm_validation]
- [x] [parallel] Add artifacts-first Valence export compatibility for FSM receipts without requiring live Valence APIs. r[kamacite.stack_integration.fsm_valence_export]
- [x] [parallel] Document Octet Rust-projection interoperability and preserve the boundary that Octet source-shape findings do not prove Kamacite FSM semantic correctness. r[kamacite.stack_integration.fsm_octet_interop]
- [x] [serial] Run Rust tests, default fixture validation, manifest projection freshness, Valence export checks, Cairn validation, and proposal/design/tasks gates before archive. r[kamacite.lossless_ir.fsm_final_validation]
