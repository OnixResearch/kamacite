## Phase 1: Executed evidence model

- [x] [serial] Split stored and executed Steel evidence into closed DTO alternatives and preserve current stored-evidence behavior as explicitly non-fresh. r[kamacite.findings.steel_execution.fresh_probe]
- [x] [serial] Implement an admitted embedded Steel engine or equivalent bounded runner that executes DTO-probe fixtures with declared modules and host functions only. r[kamacite.findings.steel_execution.fresh_probe] r[kamacite.findings.steel_execution.capability_enforcement]
- [x] [serial] Derive evaluator identity and execution receipt facts from the actual runner rather than caller descriptor text. r[kamacite.findings.steel_execution.runner_identity]

## Phase 2: DTO conformance

- [x] [serial] Define content-addressed input/output DTO contracts and pure schema validation for every admitted Steel fixture profile. r[kamacite.findings.steel_execution.schema_conformance]
- [x] [serial] Gate executed golden updates on successful runner, authority, and schema-conformance evidence. r[kamacite.findings.steel_execution.fresh_probe] r[kamacite.findings.steel_execution.schema_conformance]

## Phase 3: Regression evidence

- [x] [serial] Add positive executed DTO/module/host-function fixtures and negative stale-output, fake-runner, wrong-shape JSON, undeclared function/import, failed-run, and malformed-output cases. r[kamacite.findings.steel_execution.capability_enforcement] r[kamacite.findings.steel_execution.schema_conformance]
- [x] [serial] Run Steel integration tests, default and explicit adapter validation, workspace tests, Clippy, Cairn gates, and Nix checks. r[kamacite.findings.steel_execution.fresh_probe] r[kamacite.findings.steel_execution.runner_identity]
