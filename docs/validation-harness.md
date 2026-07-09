# Kamacite validation harness

Kamacite now has a walking-skeleton validation harness for the first trusted source-envelope and receipt slice.

Run it through the sibling Cairn dev shell until Kamacite grows its own package:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- validate --root . --manifest fixtures/manifest.json'
```

The default mode is non-mutating. Golden receipts are compared exactly and mismatches fail validation.

The explicit update workflow is:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- validate --root . --manifest fixtures/manifest.json --update-golden --update-report target/kamacite-golden-update-receipt.json'
```

That mode writes checked golden receipt projections and emits `target/kamacite-golden-update-receipt.json` with old and new receipt identities. It is the only harness mode that rewrites fixture evidence.

## Boundary

- `crates/kamacite-core` is the pure deterministic core. It receives in-memory DTOs and bytes, returns deterministic diagnostics and receipts, and does not read files, inspect environment, spawn processes, print, access clocks, access randomness, or mutate state.
- `crates/kamacite-cli` is the thin imperative shell. It reads the manifest projection and fixture bytes, compares or writes golden receipts, and renders JSON reports.
- `fixtures/manifest.ncl` is the reviewed Nickel manifest source for the seed gate. `fixtures/manifest.json` is the runtime projection consumed by the shell.
- `fixtures/core/**` contains pure in-memory fixtures. Adapter/probe execution suites are deferred until their own changes add declared capabilities and receipt-only modes.

## Current seed evidence

The seed corpus includes a positive source-envelope fixture, stale source-hash rejection, wrong identity-domain rejection, unsupported schema rejection, missing source-bytes rejection, positive and negative canonical value cases, shell capability escape cases, stack role cases, and algebraic-effect positive/negative fixtures. Each fixture has a checked golden receipt under `fixtures/golden/`.
