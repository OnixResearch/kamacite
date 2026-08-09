# Validation evidence

Date: 2026-08-09

## Admitted input

- Cairn revision: `695124d459574ba7aeba6097310d237f393c243c`
- The targeted Nix lock update was generated with `nix flake lock --update-input cairn`.
- The resulting root `cairn` lock node resolves to the admitted revision.

## Lifecycle migration

- Preview plan hash: `3663d3a65504746700f6abeed3b5b9c79f75ed3f74b50fda0f76548711ed1fa5`
- The migration receipt is retained in `evidence/migration-receipt.json`.
- A sorted BLAKE3 manifest comparison found byte parity between the legacy `cairn/` tree and native `.cairn/` tree before legacy removal.
- Maintained repository paths now select `.cairn/`. Historical receipt text retains its original paths.

## Checks

- `nix run github:OnixResearch/cairn/695124d459574ba7aeba6097310d237f393c243c#cairn -- policy export --source cairn-policy/default.ncl --output cairn-policy/generated/cairn-policy.json --check`: pass.
- `nix develop . -c cargo test --workspace`: pass.
- `nix run .#kamacite -- validate --root . --manifest fixtures/manifest.json`: pass.
- `nix run .#lifecycle-validate -- --root .`: pass.
- `nix flake check path:$PWD -L`: pass.
- `git diff --check`: pass.

## Boundary

This evidence covers the Cairn input update, the lifecycle-store migration, and the listed checks. It does not expand Kamacite product claims.
