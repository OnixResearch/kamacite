## Phase 1: Baseline and total error plumbing

- [x] [serial] Save the current focused and workspace/all-target Clippy failures as baseline evidence without treating passing unit tests as gate success. r[kamacite.findings.clippy_gate.workspace_clean]
- [x] [serial] Replace production serialization, canonicalization, and length-conversion `expect` calls with typed errors or explicitly infallible encoders. r[kamacite.findings.clippy_gate.total_core]
- [x] [serial] Resolve structural library warnings such as redundant `must_use` and collapsible control flow without broad allows. r[kamacite.findings.clippy_gate.no_policy_weakening]

## Phase 2: All-target cleanup and guard

- [x] [serial] Refactor test helpers to satisfy the crate panic lint posture under all-target Clippy. r[kamacite.findings.clippy_gate.workspace_clean] r[kamacite.findings.clippy_gate.total_core]
- [x] [serial] Add a negative guard fixture/check proving forbidden panic calls or warning regressions fail the repo-owned Clippy rail. r[kamacite.findings.clippy_gate.no_policy_weakening]
- [x] [serial] Align documented local, Nix, and release Clippy commands on workspace/all-target warning-as-error scope. r[kamacite.findings.clippy_gate.command_parity]

## Phase 3: Validation

- [x] [serial] Run focused core Clippy, workspace/all-target Clippy, Rust tests, fixture validation, Cairn gates, and Nix checks; record any unrelated pre-existing blocker separately. r[kamacite.findings.clippy_gate.workspace_clean] r[kamacite.findings.clippy_gate.command_parity]
