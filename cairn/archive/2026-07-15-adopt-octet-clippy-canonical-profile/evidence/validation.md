# Validation evidence

Date: 2026-07-15

## Baseline

- `nix build .#checks.x86_64-linux.clippy --no-link -L --option secret-key-files ''` — passed before policy changes.
- `nix build .#checks.x86_64-linux.clippy-rejects-forbidden-panic --no-link -L --option secret-key-files ''` — passed before fixture changes.
- Octet `production-safety` and `canonical-determinism` profiled evidence runs — passed against the pre-change Kamacite workspace.

## Post-change checks

- `nix build .#checks.x86_64-linux.clippy --no-link -L --option secret-key-files ''` — passed with all six `canonical-determinism` restrictions denied across workspace/all-targets.
- `nix build .#checks.x86_64-linux.clippy-rejects-forbidden-panic --no-link -L --option secret-key-files ''` — passed by observing both `expect_used` and `dbg_macro` from the violating fixture.
- `nixfmt --check flake.nix` — passed.
- `rustfmt --edition 2024 --check fixtures/clippy-negative/src/lib.rs` — passed in the repo development shell.
- `nix run --option secret-key-files '' .#lifecycle-validate -- --root .` — passed with no issues.

## Octet policy-bound evidence

- `cargo-octet evidence collect --rail clippy-co-reporting --run --profile canonical-determinism --workspace-root .` — passed.
- `cargo-octet artifact verify` — reported the bundle valid.
- Receipt: `evidence/octet-clippy-canonical/evidence-rail-receipt.json`.
- Receipt binds embedded catalog identity `b3:125e578fa37bfcb314108c80992df302b3dd2647df0f0d042c3ac205b82010cd`, selected profile identity `b3:359239a1d9a1f5081fba4515b4b0ca4ab01a3484ae222a85e07754563674d968`, exact lint argv, clean findings, and successful tool status.

Nix attempted the configured remote builder during some checks, reported it unavailable, and completed the derivations locally. `--option secret-key-files ''` bypassed an unrelated missing host signing-key path without changing repository policy.
