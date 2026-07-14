# Kamacite License Boundary Specification

## Purpose

Keep the canonical interchange core permissive while applying AGPL reciprocity to the operator CLI.

## Requirements

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
