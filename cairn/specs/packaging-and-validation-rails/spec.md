# Packaging and Validation Rails Specification

## Purpose

Defines Kamacite's repo-owned package, application, development-shell, and validation-check surface for running the existing deterministic harness without borrowing the sibling Cairn development shell.

## Requirements

### Requirement: Self-hosted Kamacite package

r[kamacite.packaging.self_hosted_package] Kamacite MUST provide a repo-local package and app entry point for the Kamacite CLI. The packaged CLI MUST expose the same validation commands as the Cargo runner, MUST make explicitly declared Nickel adapter execution available through a packaged Nickel evaluator, and MUST NOT require entering a sibling repository development shell for normal Rust or fixture validation.

#### Scenario: Packaged CLI runs default validation

r[kamacite.packaging.self_hosted_package.default_validation]

- GIVEN a contributor is in the Kamacite repository with fixture inputs present
- WHEN the contributor runs the repo-owned packaged CLI in default validation mode
- THEN it MUST validate the default manifest and golden receipts without requiring the sibling Cairn development shell.

#### Scenario: Packaged CLI keeps Nickel execution explicit

r[kamacite.packaging.self_hosted_package.nickel_explicit]

- GIVEN Nickel adapter fixtures require reference-export execution
- WHEN the packaged CLI runs the explicit Nickel adapter validation command
- THEN the command MUST use a packaged Nickel evaluator and MUST NOT make default validation execute Nickel implicitly.

#### Scenario: Package does not broaden claims

r[kamacite.packaging.self_hosted_package.non_claims]

- GIVEN a packaged app or check validates Kamacite fixtures
- WHEN the result is reported
- THEN the package surface MUST NOT claim evaluator soundness, compiler correctness, runtime authorization, sandboxing, Valence runtime ingestion, or arbitrary semantic equivalence beyond the receipts it validates.

### Requirement: Repo-owned validation checks

r[kamacite.packaging.validation_checks] Kamacite MUST provide repo-owned validation checks for the supported implementation and fixture rails. The checks MUST include Rust formatting, Rust tests, Clippy, package build, default fixture validation, manifest projection freshness, Nickel adapter validation, Steel adapter validation, Valence receipt export validation, and Cairn lifecycle validation.

#### Scenario: Positive checks cover supported rails

r[kamacite.packaging.validation_checks.positive]

- GIVEN a clean Kamacite checkout with current fixtures and receipts
- WHEN the repo-owned check suite runs
- THEN it MUST exercise the Rust, fixture, adapter, export, package, and lifecycle rails and report success only when all supported positive gates pass.

#### Scenario: Checks are non-mutating

r[kamacite.packaging.validation_checks.non_mutating]

- GIVEN a golden receipt, manifest projection, Valence export projection, or generated evidence artifact is stale
- WHEN a default repo-owned check runs
- THEN it MUST fail deterministically and MUST NOT rewrite source fixtures, runtime projections, golden receipts, export projections, or update reports.

#### Scenario: Lifecycle validation remains a tool boundary

r[kamacite.packaging.validation_checks.lifecycle_tool_boundary]

- GIVEN Kamacite validates its Cairn lifecycle artifacts as part of the check suite
- WHEN the lifecycle check invokes Cairn
- THEN Cairn MUST be treated as an external lifecycle validation tool and MUST NOT become a Kamacite canonical IR, adapter, evaluator, or receipt authority.

### Requirement: Negative stale-manifest package check

r[kamacite.packaging.negative_stale_manifest_check] Kamacite MUST include a repo-owned negative check proving the packaged validation command rejects stale manifest projection evidence. The negative check MUST mutate only a temporary copy of the source, MUST run the default packaged validation command, and MUST require a deterministic stale-projection diagnostic before passing.

#### Scenario: Stale manifest identity is rejected

r[kamacite.packaging.negative_stale_manifest_check.rejects_stale_identity]

- GIVEN the source-declared runtime manifest projection identity is intentionally changed in a temporary check copy
- WHEN the packaged default validation command runs against that copy
- THEN validation MUST fail closed with the stale-projection diagnostic and MUST NOT rewrite the manifest source, runtime projection, or golden receipts.

### Requirement: Package documentation

r[kamacite.packaging.docs] Kamacite documentation MUST describe the repo-owned package, app, development shell, and check commands for normal validation. Documentation MUST preserve existing adapter execution boundaries and MUST keep lifecycle validation, Nickel evaluation, Steel DTO/probe validation, and Valence export non-claims explicit.

#### Scenario: Docs prefer repo-owned commands

r[kamacite.packaging.docs.repo_owned_commands]

- GIVEN a contributor reads Kamacite validation instructions
- WHEN the instructions describe Rust tests, fixture validation, Nickel adapter validation, Steel adapter validation, or Valence export validation
- THEN they MUST prefer `nix develop .`, `nix run .#kamacite`, or `nix flake check` over entering the sibling Cairn development shell.

#### Scenario: Docs retain non-claim boundaries

r[kamacite.packaging.docs.non_claims]

- GIVEN package documentation describes packaged validation
- WHEN it mentions adapter or export checks
- THEN it MUST state that default validation remains receipt-only for effectful adapters, Nickel execution remains explicit, Steel remains bounded DTO/probe evidence, and Valence export remains artifacts-first.

### Requirement: Package licensing follows the core-shell split

r[kamacite.packaging.license_boundary.package_split] `kamacite-core` MUST remain `MIT OR Apache-2.0`, and `kamacite-cli` MUST declare `AGPL-3.0-or-later`.

#### Scenario: Package metadata is consumed

- GIVEN an external project inspects Kamacite package metadata
- WHEN it chooses the core or CLI package
- THEN the package MUST report the license assigned to that architectural role.

### Requirement: Complete license artifacts accompany source

r[kamacite.packaging.license_boundary.artifacts] Kamacite MUST include complete AGPL, MIT, and Apache texts and MUST map each repository-owned package to its selected terms.

#### Scenario: A source archive is inspected

- GIVEN the archive contains both core and CLI source
- WHEN the license boundary is reviewed
- THEN every selected license MUST be available without relying on an external URL.

### Requirement: License metadata is outside canonical IR identity

r[kamacite.packaging.license_boundary.documentation] Kamacite documentation MUST state that package licensing does not alter existing canonical source-envelope, Preserves value, adapter, target-artifact, or receipt identity rules and does not relicense third-party material.

#### Scenario: A package license changes

- GIVEN repository packaging metadata changes without changing a canonical artifact's bytes or profile
- WHEN existing artifact identity is interpreted
- THEN Kamacite MUST NOT claim that the package metadata change rewrites that artifact identity.

### Requirement: The split is validated in both directions

r[kamacite.packaging.license_boundary.validation] Focused validation MUST accept the intended core/CLI mapping and MUST reject fixtures or repository states that assign AGPL to the core or a permissive license to the CLI.

#### Scenario: Core and CLI declarations are reversed

- GIVEN a malformed package mapping reverses the selected licenses
- WHEN license-boundary validation runs
- THEN validation MUST fail with package-specific diagnostics.

### Requirement: Focused verification is recorded

r[kamacite.packaging.license_boundary.final_validation] The change MUST verify Cargo metadata, checked-in license artifacts, current documentation, and lifecycle conformance.

#### Scenario: The implemented boundary is reviewed

- GIVEN all package declarations and artifacts are present
- WHEN focused checks and lifecycle validation run
- THEN the boundary MUST pass without making semantic-equivalence or release-readiness claims.

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

### Requirement: Canonical Clippy restriction profile

r[kamacite.packaging.clippy_restriction_profile] Kamacite's repo-owned Clippy gate MUST deny the explicit restriction lints selected by Octet's reviewed `canonical-determinism` profile in addition to ordinary warnings, and MUST NOT enable the blanket `clippy::restriction` group.

#### Scenario: Clean workspace passes the selected profile

r[kamacite.packaging.clippy_restriction_profile.positive]

- GIVEN all tracked Kamacite Rust library, binary, and test targets
- WHEN the repo-owned workspace/all-target Clippy gate runs
- THEN it MUST deny `dbg_macro`, `mem_forget`, `undocumented_unsafe_blocks`, `multiple_unsafe_ops_per_block`, `host_endian_bytes`, and `iter_over_hash_type`.
- AND it MUST report success only when no ordinary warning or selected restriction finding remains.

#### Scenario: Violating fixture fails closed

r[kamacite.packaging.clippy_restriction_profile.negative]

- GIVEN a negative fixture contains an existing forbidden hidden panic and a `dbg!` macro selected by the adopted profile
- WHEN the repo-owned negative Clippy check runs with the same restriction flags
- THEN the command MUST fail.
- AND the check MUST observe both the `expect_used` and `dbg_macro` lint identities.

#### Scenario: Compiler lint evidence remains bounded

r[kamacite.packaging.clippy_restriction_profile.boundaries]

- GIVEN the canonical Clippy gate passes
- WHEN Kamacite reports the result
- THEN it MUST treat the result as compiler source-shape evidence only.
- AND it MUST NOT claim canonical semantic correctness, runtime determinism, panic freedom, memory safety, compiler correctness, formal proof, whole-program correctness, or release eligibility.
