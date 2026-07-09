# Kamacite validation harness

Kamacite now has a walking-skeleton validation harness for the first trusted source-envelope and receipt slice.

Run it through the sibling Cairn dev shell until Kamacite grows its own package:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- validate --root . --manifest fixtures/manifest.json'
```

The default mode is non-mutating. Before fixture execution is trusted, the CLI checks `fixtures/manifest.ncl` for the source-declared BLAKE3 projection identity and fixture ID list, compares them to `fixtures/manifest.json`, and fails closed on stale projection bytes or fixture-ID drift. Golden receipts are then compared exactly and mismatches fail validation.

A focused source/projection freshness check is available with:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- check-manifest-projection --source fixtures/manifest.ncl --projection fixtures/manifest.json'
```

The explicit Nickel adapter execution mode is:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- validate-nickel-adapter --root . --manifest fixtures/manifest.json'
```

Only fixtures that declare `execution_mode = "reference-export"` run the Nickel reference CLI. The shell records `fixture-read`, `nickel-reference-export`, and `capture-stdout` as declared capabilities, captures `nickel --version` as evaluator identity material, runs `nickel export` outside `kamacite-core`, and passes source bytes, export bytes, evaluator identity, projection identity, diagnostics, checked laws, and non-claims to the pure core. Default `validate` remains stored-evidence-only and does not silently treat prerecorded export bytes as a fresh Nickel run.

The explicit Valence receipt export command is:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- export-valence-receipt --root . --receipt fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json --conformance fixtures/valence-export/conformance.json --golden fixtures/valence-export/golden/source-envelope.valid.nickel-minimal.valence-export.json'
```

The command reads explicit source receipt, conformance, optional source-span, and optional golden-projection inputs. It calls pure core export logic, reports deterministic JSON, and remains non-mutating by default. `--update-golden` is the only mode that writes a Valence export projection and emits `target/kamacite-valence-export-update-receipt.json` with old/new export identities, source receipt identity, checked laws, and non-claims.

The explicit golden receipt update workflow is:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- validate --root . --manifest fixtures/manifest.json --update-golden --update-report target/kamacite-golden-update-receipt.json'
```

That mode writes checked golden receipt projections and emits `target/kamacite-golden-update-receipt.json` with old/new receipt identities, input/output identities, covered requirement IDs, and non-claims. It is the only harness mode that rewrites golden fixture evidence; manifest projection changes remain reviewed source/projection file changes and are not rewritten implicitly by validation.

## Boundary

- `crates/kamacite-core` is the pure deterministic core. It receives in-memory DTOs and bytes, returns deterministic diagnostics and receipts, and does not read files, inspect environment, spawn processes, print, access clocks, access randomness, or mutate state.
- `crates/kamacite-cli` is the thin imperative shell. It reads the manifest projection and fixture bytes, optionally invokes the Nickel reference evaluator for explicitly declared Nickel fixtures, compares or writes golden receipts, and renders JSON reports.
- `fixtures/manifest.ncl` is the reviewed Nickel manifest source for the fixture corpus. `fixtures/manifest.json` is the runtime projection consumed by the shell after source-declared projection identity and fixture-ID freshness checks pass.
- `fixtures/core/**` contains pure in-memory fixtures. `fixtures/adapters/nickel/**` contains Nickel export projection bytes for the executable Nickel adapter suite. `fixtures/valence-export/**` contains reviewed Valence export conformance data, source-span fixtures, negative receipt inputs, and golden export projections. Other adapter/probe execution suites remain deferred until their own changes add declared capabilities and receipt-only modes.

## Current seed evidence

The seed corpus includes a positive source-envelope fixture, stale source-hash rejection, wrong identity-domain rejection, unsupported schema rejection, missing source-bytes rejection, positive and negative canonical value cases, Nickel adapter positive/negative execution-boundary fixtures, Valence export positive/negative profile fixtures, shell capability escape cases, stack role cases, and algebraic-effect positive/negative fixtures. Each fixture has a checked golden receipt under `fixtures/golden/`.
