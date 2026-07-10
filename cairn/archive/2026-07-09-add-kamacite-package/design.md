## Context

Kamacite's trusted behavior is now executable, but the repo is still packaged as a bare Cargo workspace. README and validation docs instruct users to enter the sibling Cairn dev shell before running `cargo test` or `cargo run -p kamacite-cli`. That was appropriate while the harness was a walking skeleton, but it is now friction: package identity, Nickel availability, app names, and check composition are not owned by Kamacite.

The change should keep the existing functional-core / imperative-shell split. It should not change receipt semantics, fixture contents, canonical IR boundaries, or adapter authority. Packaging is an execution and operator surface around existing commands.

## Decisions

### 1. Provide a Kamacite-owned flake package and app

**Choice:** Add `flake.nix` with a `kamacite` package that builds `kamacite-cli`, installs a `kamacite` wrapper alias for the existing `kamacite-validate` binary, and wraps Nickel into the runtime `PATH` for the explicit Nickel adapter command. Expose `nix run .#kamacite -- ...` and default app/package aliases.

**Rationale:** The command surface should be repo-local and stable. Wrapping Nickel makes `validate-nickel-adapter` reproducible through the packaged app while preserving the rule that only the explicit Nickel adapter mode runs Nickel.

### 2. Keep validation checks deterministic and non-mutating

**Choice:** Add flake checks that copy the source to a writable build directory, run non-mutating commands, and touch `$out` only after success. Checks must not run update modes, rewrite golden receipts, or infer evidence from ambient files outside declared roots.

**Rationale:** Kamacite receipts and golden projections are evidence. A check suite that mutates them would weaken the no-rewrite boundary already accepted for the harness.

### 3. Include positive and negative package-gate evidence

**Choice:** Positive checks cover Rust formatting, tests, Clippy, package build, default fixture validation, manifest projection freshness, Nickel adapter validation, Steel adapter validation, Valence export validation, and Cairn lifecycle validation. A negative check intentionally corrupts the source-declared manifest projection identity and asserts default validation fails with the stale-projection diagnostic.

**Rationale:** The package rail should prove both that supported workflows run and that stale evidence still fails closed under the packaged app. This avoids a convenience wrapper that accidentally bypasses source/projection authority.

### 4. Use Cairn as an explicit lifecycle tool dependency, not as Kamacite's dev shell

**Choice:** The flake may use the sibling Cairn flake as a lifecycle-validation tool input for the lifecycle check, but Kamacite's Rust package, dev shell, and fixture checks must be owned by Kamacite and must not require entering Cairn's dev shell.

**Rationale:** Cairn owns lifecycle validation. Kamacite should still validate its Cairn artifacts with the reviewed tool while removing the unrelated dependency on Cairn's Rust/Nickel development shell for normal harness execution.

### 5. Do not broaden adapter or IR claims

**Choice:** Packaging docs must continue to state that default validation is receipt-only for effectful adapters, Nickel execution is explicit, Steel remains DTO/probe receipt validation, and Valence export is artifacts-first.

**Rationale:** A better command surface must not imply sandboxing, evaluator soundness, compiler correctness, Valence runtime ingestion, or arbitrary semantic equivalence.

## Risks / Trade-offs

- A local Cairn flake input keeps lifecycle validation convenient in this sibling workspace but remains a tool dependency rather than a Kamacite semantic dependency.
- Wrapping Nickel into the CLI runtime makes the Nickel adapter check easier to run, but evaluator-version drift will still surface as receipt drift by design.
- Flake checks increase validation time, but they provide the baseline needed before adding higher-risk adapter/runtime integrations.
