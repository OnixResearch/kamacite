## Phase 1: Path authority core

- [ ] [serial] Preserve the reproduced symlink-write escape as a negative integration fixture and inventory every root-scoped read/write call site. r[kamacite.findings.root_io_confinement.coverage]
- [ ] [serial] Implement a pure relative-path policy and one handle-relative root-capability shell gateway for reads, creates, replacements, and renames. r[kamacite.findings.root_io_confinement.single_gateway]
- [ ] [serial] Enforce no-follow/same-root semantics for source, destination, and parent components without check-then-open races. r[kamacite.findings.root_io_confinement.symlink_safe] r[kamacite.findings.root_io_confinement.mutation_safe]

## Phase 2: Migrate commands

- [ ] [serial] Route manifests, fixtures, evaluator inputs, goldens, receipts, Valence outputs, docs outputs, and all update reports through the gateway. r[kamacite.findings.root_io_confinement.single_gateway]
- [ ] [serial] Implement confined temporary-file and replacement behavior for explicit mutation workflows. r[kamacite.findings.root_io_confinement.mutation_safe]

## Phase 3: Regression evidence

- [ ] [serial] Add positive regular-path tests and negative absolute, parent, source symlink, destination symlink, parent symlink, and replacement-race tests for each command family. r[kamacite.findings.root_io_confinement.coverage]
- [ ] [serial] Run shell integration tests, workspace tests, Clippy, fixture validation, Cairn gates, and Nix checks. r[kamacite.findings.root_io_confinement.single_gateway] r[kamacite.findings.root_io_confinement.coverage]
