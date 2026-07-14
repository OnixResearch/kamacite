## Phase 1: Producer profile

- [x] [serial] Define the content-addressed producer/canonicalizer profile and classify normative fields versus display metadata. r[kamacite.findings.function_address_producer.profile_identity] r[kamacite.findings.function_address_producer.metadata_boundary]
- [x] [serial] Extend the Octet handoff and core DTOs to carry a validated producer profile or enough inputs to construct one. r[kamacite.findings.function_address_producer.profile_identity]
- [x] [serial] Bind producer profile identity into canonical function-address receipt bytes and projection validation. r[kamacite.findings.function_address_producer.receipt_binding]

## Phase 2: Compatibility and migration

- [x] [serial] Preserve display-only rename/path stability while rejecting unbound normative producer-field changes. r[kamacite.findings.function_address_producer.metadata_boundary] r[kamacite.findings.function_address_producer.receipt_binding]
- [x] [serial] Version receipts and classify legacy v1 identities with an explicit tool-version-unbound caveat. r[kamacite.findings.function_address_producer.migration]
- [x] [serial] Migrate CLI fixtures, JSON/Valence projections, and goldens with old/new identity evidence. r[kamacite.findings.function_address_producer.migration]

## Phase 3: Regression evidence

- [x] [serial] Add tests proving tool/canonicalizer/config changes alter identity, display-only changes do not, and projection drift fails closed. r[kamacite.findings.function_address_producer.profile_identity] r[kamacite.findings.function_address_producer.metadata_boundary]
- [x] [serial] Run focused core/CLI tests, fixture/export validation, workspace tests, Clippy, Cairn gates, and Nix checks. r[kamacite.findings.function_address_producer.receipt_binding] r[kamacite.findings.function_address_producer.migration]
