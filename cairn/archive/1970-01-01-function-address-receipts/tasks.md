## Phase 1: Portable function-address receipt

- [x] [serial] Capture the portable function-address receipt requirement delta before implementation. r[kamacite.function_address_receipts.contract]
- [x] [serial] Add typed core models for source envelopes, function records, dependency edges, caveats, receipt identity, and validation diagnostics. r[kamacite.function_address_receipts.contract] r[kamacite.function_address_receipts.validation]
- [x] [serial] Add Preserves schema/canonical identity support and deterministic JSON compatibility projections. r[kamacite.function_address_receipts.canonical_identity]
- [x] [parallel] Add positive fixtures for valid Octet-produced receipts, name/path metadata churn, dependency-edge preservation, and Valence export projection. r[kamacite.function_address_receipts.fixtures.positive] r[kamacite.function_address_receipts.valence_export]
- [x] [parallel] Add negative fixtures for stale source identities, malformed BLAKE3 digests, locator collisions, missing non-claims, unknown identity domains, and overclaim text. r[kamacite.function_address_receipts.fixtures.negative] r[kamacite.function_address_receipts.validation]
- [x] [parallel] Document Octet/Kamacite/Valence/Mantle/Cairn ownership boundaries and artifact-first coupling rules. r[kamacite.function_address_receipts.docs] r[kamacite.function_address_receipts.octet_interop]
- [x] [serial] Run Kamacite validation, focused receipt/export tests, negative fixture checks, lifecycle validation, and proposal/design/tasks gates before sync/archive. r[kamacite.function_address_receipts.final_validation]
