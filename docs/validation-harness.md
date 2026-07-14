# Kamacite validation harness

Kamacite now has a walking-skeleton validation harness for the first trusted source-envelope and receipt slice.

Run it through Kamacite's repo-owned package:

```bash
nix run .#kamacite -- validate --root . --manifest fixtures/manifest.json
```

The default mode is non-mutating. Before fixture execution is trusted, the CLI checks `fixtures/manifest.ncl` for the source-declared BLAKE3 projection identity and fixture ID list, compares them to `fixtures/manifest.json`, and fails closed on stale projection bytes or fixture-ID drift. Golden receipts are then compared exactly and mismatches fail validation.

A focused source/projection freshness check is available with:

```bash
nix run .#kamacite -- check-manifest-projection --source fixtures/manifest.ncl --projection fixtures/manifest.json
```

The explicit Nickel adapter execution mode is:

```bash
nix run .#kamacite -- validate-nickel-adapter --root . --manifest fixtures/manifest.json
```

Only fixtures that declare `execution_mode = "reference-export"` run the Nickel reference CLI. The shell records `fixture-read`, `nickel-reference-export`, and `capture-stdout` as declared capabilities, captures `nickel --version` as evaluator identity material, runs `nickel export` outside `kamacite-core`, and passes source bytes, export bytes, evaluator identity, projection identity, diagnostics, checked laws, and non-claims to the pure core. Default `validate` remains stored-evidence-only and does not silently treat prerecorded export bytes as a fresh Nickel run.

The explicit Steel adapter validation mode is:

```bash
nix run .#kamacite -- validate-steel-adapter --root . --manifest fixtures/manifest.json
```

Default `validate` remains receipt-only for Steel fixtures. It reads reviewed source, module descriptor, output projection, evaluator descriptor, host-function declarations, declared imports, capabilities, and checked laws from the manifest and fixture files, then passes normalized DTOs into `kamacite-core`. The explicit Steel command is the only mode that refreshes DTO-probe evaluator identity for fixtures declaring `execution_mode = "dto-probe"`; it still stays non-mutating unless `--update-golden` is provided. This first Steel slice does not claim sandboxing, Steel evaluator soundness, arbitrary Scheme module semantics, bytecode portability, opaque closure serialization, or canonical IR authority for JSON/Steel outputs.

The explicit Valence receipt export command is:

```bash
nix run .#kamacite -- export-valence-receipt --root . --receipt fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json --conformance fixtures/valence-export/conformance.json --golden fixtures/valence-export/golden/source-envelope.valid.nickel-minimal.valence-export.json
```

The command reads explicit source receipt, conformance, optional source-span, and optional golden-projection inputs. It calls pure core export logic, reports deterministic JSON, and remains non-mutating by default. `--update-golden` is the only mode that writes a Valence export projection and emits `target/kamacite-valence-export-update-receipt.json` with old/new export identities, source receipt identity, checked laws, and non-claims.

The explicit golden receipt update workflow is:

```bash
nix run .#kamacite -- validate --root . --manifest fixtures/manifest.json --update-golden --update-report target/kamacite-golden-update-receipt.json
```

That mode writes checked golden receipt projections and emits `target/kamacite-golden-update-receipt.json` with old/new receipt identities, input/output identities, covered requirement IDs, and non-claims. It is the only harness mode that rewrites golden fixture evidence; manifest projection changes remain reviewed source/projection file changes and are not rewritten implicitly by validation.

The Preserves migration workflow is dry-run-first:

```bash
nix run .#kamacite -- migrate-preserves --root . --manifest fixtures/manifest.json --manifest-source fixtures/manifest.ncl
nix run .#kamacite -- migrate-preserves --root . --manifest fixtures/manifest.json --manifest-source fixtures/manifest.ncl --update --report target/kamacite-preserves-migration-report.json
```

The pure core plans canonical packed bytes and typed identity bindings. The update shell performs at most four named convergence passes so receipts that depend on migrated canonical identities are regenerated before the report is accepted. It updates only canonical fixture projections and their explicit manifest identity fields, preserves intentionally stale negative identities, refreshes the manifest projection identity, and records whether convergence occurred. Legacy labeled-JSON identities are retained as compatibility evidence rather than rewritten as Preserves identities.

## Boundary

- `crates/kamacite-core` is the pure deterministic core. It receives in-memory DTOs and bytes, returns deterministic diagnostics and receipts, and does not read files, inspect environment, spawn processes, print, access clocks, access randomness, or mutate state.
- `crates/kamacite-cli` is the thin imperative shell. It reads the manifest projection and fixture bytes, optionally invokes the Nickel reference evaluator for explicitly declared Nickel fixtures, runs the explicit Steel DTO-probe admission mode without broad ambient authority, compares or writes golden receipts, and renders JSON reports.
- `fixtures/manifest.ncl` is the reviewed Nickel manifest source for the fixture corpus. `fixtures/manifest.json` is the runtime projection consumed by the shell after source-declared projection identity and fixture-ID freshness checks pass.
- `fixtures/core/**` contains pure in-memory fixtures, including `fixtures/core/fsm/**` for FSM spec, exhaustiveness-receipt, and trace-receipt canonical bytes, plus `fixtures/core/docs/**` for documentation IR, renderer-output, runnable-cell, and documentation-receipt evidence. `fixtures/adapters/nickel/**` contains Nickel export projection bytes for the executable Nickel adapter suite. `fixtures/adapters/steel/**` contains Steel source-envelope, module-descriptor, DTO output, and negative projection fixtures for the bounded Steel adapter suite. `fixtures/valence-export/**` contains reviewed Valence export conformance data, source-span fixtures, negative receipt inputs, and golden export projections. Other adapter/probe execution suites remain deferred until their own changes add declared capabilities and receipt-only modes.
- Documentation validation remains non-executing by default: `documentation-cell` fixtures carry stored outputs or diagnostics, explicit evaluator identities, declared capabilities, and non-claims rather than hidden script execution. See `docs/runnable-docs-pipeline.md`.

## Current seed evidence

The seed corpus includes a positive source-envelope fixture, stale source-hash rejection, wrong identity-domain rejection, unsupported schema rejection, missing source-bytes rejection, positive and negative canonical value cases, Nickel adapter positive/negative execution-boundary fixtures, Steel adapter positive/negative DTO/module/host-function/evaluator/output-boundary fixtures, Valence export positive/negative profile fixtures, shell capability escape cases, stack role cases, algebraic-effect positive/negative fixtures, FSM spec/exhaustiveness/trace positive and negative fixtures, and documentation IR/output/cell/receipt positive and negative fixtures. Each fixture has a checked golden receipt under `fixtures/golden/`.
