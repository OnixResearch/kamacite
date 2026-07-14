# Packaging and Validation Rails Specification

## Purpose

Restore the repo-owned Clippy rail as a passing, panic-free workspace/all-target gate.

## Requirements

### Requirement: The canonical Clippy command passes

r[kamacite.findings.clippy_gate.workspace_clean] A clean supported checkout MUST pass `cargo clippy --workspace --all-targets -- -D warnings` in the repo-owned development environment before Rust validation rails are reported green.

#### Scenario: Current workspace is checked

- GIVEN all tracked Rust library, binary, example, and test targets
- WHEN the canonical Clippy command runs
- THEN it MUST exit successfully with no warnings or denied-lint errors.

### Requirement: Core canonicalization does not panic

r[kamacite.findings.clippy_gate.total_core] Production core code MUST NOT use `expect`, `unwrap`, panic macros, unreachable assertions, or equivalent hidden panic paths for serialization, canonicalization, length conversion, or malformed input handling.

#### Scenario: Serialization or conversion fails

- GIVEN a fallible canonicalization or conversion operation returns an error
- WHEN the pure core handles the result
- THEN it MUST return a typed deterministic error or diagnostic and MUST NOT panic.

### Requirement: Fixes preserve lint policy

r[kamacite.findings.clippy_gate.no_policy_weakening] Clippy recovery MUST resolve findings structurally and MUST NOT globally allow denied panic lints or remove warning-as-error coverage. Any narrow exception MUST be justified and tested.

#### Scenario: New forbidden panic call is introduced

- GIVEN a production change adds an `expect` call under the core crate lint policy
- WHEN the repo-owned Clippy check runs
- THEN the check MUST fail before package validation can report success.

### Requirement: Local and Nix gates are equivalent

r[kamacite.findings.clippy_gate.command_parity] Contributor documentation, the flake Clippy check, and release validation MUST use equivalent workspace/all-target scope and warning policy.

#### Scenario: Library-only Clippy passes but a test target fails

- GIVEN a warning exists only in a test or binary target
- WHEN the repo-owned Nix Clippy check runs
- THEN the check MUST fail just as the documented canonical local command does.
