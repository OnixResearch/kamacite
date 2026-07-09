# Project Naming Specification

## Purpose

Defines Kamacite as the project identity and `kamacite` as the machine namespace after renaming from the pre-rename identity.

## Requirements

### Requirement: Kamacite project identity

r[kamacite.naming.project_identity] The project MUST use Kamacite as its human-facing name. Documentation, accepted specs, archived lifecycle artifacts, fixture metadata, schema descriptions, and non-claim text MUST NOT continue to describe the active project by a pre-rename name.

#### Scenario: Human-facing name is Kamacite

r[kamacite.naming.project_identity.human_name]

- GIVEN a user reads current project documentation or lifecycle specs
- WHEN the project is named
- THEN the text MUST use Kamacite for the active project identity.

### Requirement: Kamacite machine namespace

r[kamacite.naming.machine_namespace] The project MUST use `kamacite` as the machine namespace for crate names, command names, Rust imports, receipt schema versions, receipt codecs, Preserves schema IDs, record labels, requirement IDs, fixture metadata, and checked golden receipt content.

#### Scenario: Machine namespace is consistent

r[kamacite.naming.machine_namespace.consistent]

- GIVEN generated receipts, manifests, code, schemas, and specs are inspected after the rename
- WHEN a project namespace appears
- THEN it MUST use `kamacite` for current Kamacite-owned identities.

### Requirement: Scope preservation during rename

r[kamacite.naming.scope_preservation] The rename MUST preserve the existing bounded claims. Kamacite MUST remain an interchange IR, source-envelope format, receipt contract, and constrained policy vocabulary; the rename MUST NOT introduce a programming language, evaluator, runtime, compiler, verifier, authorization claim, or arbitrary semantic-equivalence claim.

#### Scenario: Rename does not broaden claims

r[kamacite.naming.scope_preservation.no_broadening]

- GIVEN a receipt, schema, adapter fixture, source envelope, algebraic-effect artifact, or stack-evidence export is renamed to Kamacite
- WHEN the renamed artifact is validated
- THEN the artifact MUST retain the same bounded claim and explicit non-claims it had before the rename.

### Requirement: Receipt identity update

r[kamacite.naming.receipt_identity_update] Checked golden receipts MUST be regenerated through the explicit update workflow after project namespace, schema ID, requirement ID, or receipt codec changes. Default validation MUST remain non-mutating and MUST fail on stale golden receipt content.

#### Scenario: Explicit update records namespace-driven identity churn

r[kamacite.naming.receipt_identity_update.explicit_update]

- GIVEN receipt schema IDs, requirement IDs, or non-claim text change to Kamacite
- WHEN golden receipts are updated
- THEN the update MUST use the explicit update workflow and MUST produce evidence of changed receipt identities.

### Requirement: Rename validation

r[kamacite.naming.validation] The rename MUST pass Rust format checks, Rust tests, Clippy, non-mutating fixture validation, Cairn validation, proposal/design/tasks gates, sync, archive, and remaining-reference searches for stale pre-rename or legacy project identifiers.

#### Scenario: Stale project references are rejected

r[kamacite.naming.validation.no_stale_references]

- GIVEN the rename implementation is complete
- WHEN the repository text is searched for active project references
- THEN no current code, docs, specs, schemas, manifests, or golden receipts SHOULD retain stale pre-rename or legacy project identifiers except historical third-party names or intentionally documented migration notes.
