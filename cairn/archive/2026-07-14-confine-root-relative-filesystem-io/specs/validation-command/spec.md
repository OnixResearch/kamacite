# Validation Command Specification

## Purpose

Make every root-relative file operation an enforced filesystem capability rather than a lexical path convention.

## Requirements

### Requirement: Root-scoped I/O uses one confinement gateway

r[kamacite.findings.root_io_confinement.single_gateway] Every read, create, replace, and rename performed under a CLI `--root` MUST use one audited root-capability gateway. Direct joins or ambient absolute opens MUST NOT bypass the gateway.

#### Scenario: Update report path is supplied

- GIVEN an explicit update command receives a relative update-report path
- WHEN the shell creates the report
- THEN it MUST resolve and create the report through the same root capability used for fixture and golden paths.

### Requirement: Symlinks cannot escape the root

r[kamacite.findings.root_io_confinement.symlink_safe] Root-scoped I/O MUST reject symlink components or use handle-relative no-follow operations that prove the opened object remains beneath the reviewed root. Lexical rejection of absolute paths and parent components alone MUST NOT be considered sufficient.

#### Scenario: Destination parent is an outward symlink

- GIVEN `root/escape` is a symlink to a directory outside the root
- WHEN a command writes `escape/receipt.json`
- THEN the command MUST fail with a deterministic path-escape diagnostic and MUST NOT create or modify the outside file.

### Requirement: Mutations resist replacement races

r[kamacite.findings.root_io_confinement.mutation_safe] Root-scoped mutation MUST create and replace files through admitted directory handles and MUST fail closed if a destination or parent is replaced, becomes a symlink, or leaves the root during the operation.

#### Scenario: Destination changes after admission

- GIVEN an attacker replaces an admitted destination component before file creation or rename
- WHEN the update shell commits output
- THEN the operation MUST fail or remain confined to the originally admitted directory handle without writing outside the root.

### Requirement: Confinement has positive and negative coverage

r[kamacite.findings.root_io_confinement.coverage] Kamacite MUST test normal relative reads/writes and negative absolute, parent traversal, symlink source, symlink destination, symlink parent, and replacement-race cases for every root-scoped command family.

#### Scenario: Normal relative output succeeds

- GIVEN a regular destination directory beneath the reviewed root with no symlink components
- WHEN an explicit mutation command writes an admitted artifact
- THEN the write MUST succeed and the receipt MUST report the root-relative path without ambient host-path authority.
