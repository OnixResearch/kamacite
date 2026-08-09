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
