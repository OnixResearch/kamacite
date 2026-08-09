## Phase 1: Lifecycle and package surface

- [x] [serial] Capture the self-hosted package/check requirement delta before implementation. r[kamacite.packaging.self_hosted_package]
- [x] [serial] Add the repo-local flake package, app aliases, dev shell, lifecycle tool input, and positive/negative checks. r[kamacite.packaging.self_hosted_package] r[kamacite.packaging.validation_checks] r[kamacite.packaging.negative_stale_manifest_check]
- [x] [parallel] Update README, AGENTS, and validation/export docs to use Kamacite-owned package/check commands and preserve adapter non-claims. r[kamacite.packaging.docs]
- [x] [serial] Run Rust tests, packaged validation commands, flake checks, Cairn validation, and proposal/design/tasks gates. r[kamacite.packaging.validation_checks]
- [x] [serial] Sync accepted specs and archive the completed change after evidence is green. r[kamacite.packaging.self_hosted_package]
