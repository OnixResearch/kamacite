## Why

Kamacite now has an executable Rust workspace, deterministic fixture harness, Nickel and Steel adapter validation modes, and Valence export checks, but contributors still run those gates through the sibling Cairn development shell. That keeps the trusted validation surface dependent on copy-pasted commands and a neighboring shell instead of a Kamacite-owned package/check contract.

A repo-local package and flake check surface is the next safe integration slice because it does not broaden Kamacite's IR or adapter claims. It makes the existing evidence rails easier to run, pins the command entry points, and gives later Octet, Valence, WASM, and richer Steel changes a stable baseline.

## What Changes

- Add a repo-owned Nix flake package, app, development shell, and checks for the Kamacite CLI.
- Add deterministic checks for Rust formatting, tests, Clippy, default fixture validation, manifest projection freshness, Nickel adapter validation, Steel adapter validation, Valence receipt export validation, and lifecycle validation.
- Add a negative check that intentionally creates stale manifest evidence and proves default validation rejects it without rewriting artifacts.
- Update README, AGENTS, and validation docs to use Kamacite-owned commands instead of the sibling Cairn shell for normal Rust and harness validation.

## Impact

- **Files**: `flake.nix`, `flake.lock`, `README.md`, `AGENTS.md`, `docs/validation-harness.md`, `docs/valence-receipt-export.md`, `cairn/changes/add-kamacite-package/**`, and accepted Cairn specs after sync.
- **Testing**: Baseline Rust and harness checks before implementation; final `cargo test --workspace`, package build, flake checks, Kamacite app validation commands, negative stale-manifest check, Cairn validation, and proposal/design/tasks gates.
