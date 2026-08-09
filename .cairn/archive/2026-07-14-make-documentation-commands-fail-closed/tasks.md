## Phase 1: Checked configuration and outcomes

- [x] [serial] Preserve the invalid-readable-Nickel false-success reproduction as a negative CLI fixture. r[kamacite.findings.docs_commands.config_validation]
- [x] [serial] Add checked Nickel docs projection/generation evidence and reject malformed, stale, or contract-invalid configs before dispatch. r[kamacite.findings.docs_commands.config_validation]
- [x] [serial] Define typed action outcomes and success predicates that distinguish plan, execution, mutation, unsupported, and failed states. r[kamacite.findings.docs_commands.action_completion] r[kamacite.findings.docs_commands.no_op_rejection]

## Phase 2: Wire action shells

- [x] [serial] Connect collect, check, and render actions to pure facts, IR, link, renderer, comparison, and receipt cores. r[kamacite.findings.docs_commands.action_completion]
- [x] [serial] Require actual bounded runner execution for run-cells and actual receipt-producing mutation for update; expose dry-run only as an explicit plan. r[kamacite.findings.docs_commands.no_op_rejection]
- [x] [serial] Emit deterministic action receipts binding config, inputs, action/profile, outputs, diagnostics, laws, and non-claims. r[kamacite.findings.docs_commands.receipts]

## Phase 3: Regression evidence

- [x] [serial] Add positive action tests and negative malformed config, unsupported backend/profile/format, no-op execution, no-op mutation, stale output, and failed-cell cases. r[kamacite.findings.docs_commands.config_validation] r[kamacite.findings.docs_commands.no_op_rejection]
- [x] [serial] Run docs CLI/core tests, fixture validation, workspace tests, Clippy, Cairn gates, and Nix checks. r[kamacite.findings.docs_commands.action_completion] r[kamacite.findings.docs_commands.receipts]
