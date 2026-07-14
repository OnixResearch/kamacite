## Why

The repository declares Clippy as a required validation rail, but `cargo clippy --workspace --all-targets -- -D warnings` currently fails. A focused library run reports nine errors, including production `expect()` calls forbidden by the crate-level `deny(clippy::expect_used)`, plus `double_must_use` and `collapsible_if`; all-targets adds test failures. Passing unit tests and lifecycle validation therefore do not imply the advertised Rust gate is green.

## What Changes

- Remove production panic paths from canonicalization helpers by returning typed errors or using explicitly infallible encoders.
- Resolve all warning-as-error findings in library and test targets without weakening crate lint policy.
- Add a focused negative guard proving forbidden panic macros and newly introduced Clippy warnings fail the repo-owned check.
- Keep local, Nix, and flake Clippy invocations aligned on workspace/all-target coverage.

## Impact

- **Surfaces**: Rust core error plumbing, tests, `flake.nix` checks, and contributor validation instructions.
- **Behavior**: the advertised Clippy gate becomes executable evidence again rather than a failing nominal check.
- **Validation**: focused core Clippy, workspace all-target Clippy, Rust tests, fixture validation, and Cairn gates.
