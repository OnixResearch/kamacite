## Why

Kamacite now has a much larger runtime fixture projection than the original Nickel source manifest. The validation harness still documents `fixtures/manifest.ncl` as the reviewed authority and `fixtures/manifest.json` as a compatibility projection, but only the JSON file currently contains the full corpus. That makes the compatibility projection look authoritative and weakens the Nickel-first boundary Kamacite is meant to prove.

## What Changes

- Restore `fixtures/manifest.ncl` as the full reviewed fixture-corpus authority.
- Add a deterministic projection freshness check that binds `fixtures/manifest.json` to the source manifest before normal validation trusts it.
- Add positive and negative checks for matching source/projection state and stale projection rejection.
- Document the default validation flow so unchecked JSON does not become an evidence authority.

## Impact

- **Files**: `fixtures/manifest.ncl`, `crates/kamacite-cli/src/main.rs`, validation docs, and `cairn/changes/align-fixture-manifest-authority/**`.
- **Testing**: Baseline and final `cargo test --workspace`; final harness validation; Cairn validation plus proposal/design/tasks gates, sync, and archive.
