## Phase 1: Baseline and canonical codec

- [x] [serial] Record current canonical/golden identities and run baseline Rust, fixture, adapter, export, and lifecycle checks. r[kamacite.findings.preserves_authority.real_codec]
- [x] [serial] Add a pure bounded Preserves decoder and canonical-binary encoder with explicit schema dispatch and named policy limits. r[kamacite.findings.preserves_authority.real_codec] r[kamacite.findings.preserves_authority.validation]

## Phase 2: Bind typed artifacts

- [x] [serial] Refactor canonical artifact constructors so validated typed values and identity bytes come from one decode/encode path. r[kamacite.findings.preserves_authority.typed_binding]
- [x] [serial] Remove or reject detached caller-supplied canonical-byte/DTO combinations across FSM, effects, documentation, function-address, proof, and evidence-chain surfaces. r[kamacite.findings.preserves_authority.typed_binding]
- [x] [serial] Version canonical codecs and emit a reviewed migration report for every changed golden identity without rewriting legacy identity claims. r[kamacite.findings.preserves_authority.legacy_codec]

## Phase 3: Regression evidence

- [x] [serial] Add positive text/binary equivalence and deterministic re-encode properties plus negative malformed, non-canonical, limit, unsupported-version, and DTO-substitution cases. r[kamacite.findings.preserves_authority.validation]
- [x] [serial] Run focused codec tests, workspace tests, Clippy, fixture/adapters/export validation, Cairn gates, and the full Nix check rail. r[kamacite.findings.preserves_authority.real_codec] r[kamacite.findings.preserves_authority.validation]
