# Validation Command Specification

## Purpose

Defines the deterministic validation command that will check Kamacite schemas, fixtures, adapters, target evidence, and receipts.

## Requirements

### Requirement: Deterministic validation command

r[kamacite.lossless_ir.validation_command] Kamacite MUST provide a deterministic validation command before implementation claims are accepted. The command MUST validate schema vocabulary, source envelopes, canonical values, adapter descriptors, target artifacts, fixtures, adapter laws, diagnostics, and receipts using explicit inputs and MUST emit a deterministic validation receipt.

#### Scenario: Validation command emits receipt

r[kamacite.lossless_ir.validation_command.receipt]

- GIVEN a repository or fixture root contains Kamacite schema and fixture inputs
- WHEN the validation command runs successfully
- THEN it MUST emit a deterministic receipt that binds input identities, validated requirement coverage, checked laws, diagnostics summary, non-claims, and receipt identity.

#### Scenario: Validation command is deterministic

r[kamacite.lossless_ir.validation_command.deterministic]

- GIVEN the same explicit inputs and tool versions are provided to the validation command
- WHEN the command runs more than once
- THEN it MUST produce the same validation verdict and receipt identity regardless of filesystem iteration order, locale, wall-clock time, random values, network state, or unrelated environment variables.

### Requirement: Validation command input discovery

r[kamacite.lossless_ir.validation_input_discovery] The validation command MUST discover inputs deterministically from explicit roots, manifests, or arguments. It MUST ignore unsupported files unless the manifest requires them, and it MUST fail closed for duplicate fixture IDs, duplicate artifact IDs, ambiguous manifests, unsupported schema versions, or missing required inputs.

#### Scenario: Duplicate fixture ID fails closed

r[kamacite.lossless_ir.validation_input_discovery.duplicate_fixture]

- GIVEN two fixtures declare the same fixture identity in a validation root
- WHEN the validation command discovers inputs
- THEN validation MUST fail closed with a duplicate-fixture diagnostic before trusting either fixture result.

#### Scenario: Missing required input fails closed

r[kamacite.lossless_ir.validation_input_discovery.missing_input]

- GIVEN a manifest declares a source envelope, canonical value, adapter descriptor, target artifact, or golden receipt that is absent
- WHEN the validation command discovers inputs
- THEN validation MUST fail closed with a deterministic missing-input diagnostic.

### Requirement: Validation command execution boundary

r[kamacite.lossless_ir.validation_execution_boundary] The validation command MUST separate pure validation from effectful fixture execution. Pure checks MUST run over in-memory artifacts. Effectful adapters, evaluators, compilers, or target runners MAY run only when their descriptors declare the required capabilities and the command mode permits those effects.

#### Scenario: Pure validation does not execute adapters

r[kamacite.lossless_ir.validation_execution_boundary.pure_mode]

- GIVEN validation is run in pure or receipt-only mode
- WHEN fixtures reference Nickel, Steel, WebAssembly, LLVM, GHC, Rust MIR, or other external tools
- THEN the command MUST validate existing descriptors and receipts without executing external tools.

#### Scenario: Effectful validation requires declared capabilities

r[kamacite.lossless_ir.validation_execution_boundary.effectful_mode]

- GIVEN validation is run in a mode that permits adapter, evaluator, compiler, or target execution
- WHEN a fixture requests an effect
- THEN the effect MUST be declared by the adapter descriptor and allowed by the command mode before execution is trusted.

### Requirement: Validation command diagnostics

r[kamacite.lossless_ir.validation_diagnostics] The validation command MUST report deterministic diagnostics for schema errors, identity-domain substitutions, hash mismatches, lossy projections, undeclared effects, unsupported variants, duplicate IDs, stale golden receipts, target-artifact overclaims, and non-claim violations.

#### Scenario: Diagnostics name rejected claim

r[kamacite.lossless_ir.validation_diagnostics.rejected_claim]

- GIVEN validation rejects an artifact, fixture, adapter receipt, target artifact, or projection
- WHEN the diagnostic is emitted
- THEN it MUST name the rejected claim boundary, relevant artifact identity when available, and requirement ID or rule family that caused the rejection.

#### Scenario: Diagnostics are stable

r[kamacite.lossless_ir.validation_diagnostics.stable]

- GIVEN the same invalid inputs are validated repeatedly
- WHEN diagnostics are emitted
- THEN diagnostic codes, requirement IDs, artifact identities, and ordering MUST remain stable across runs.

### Requirement: Validation command mutation boundary

r[kamacite.lossless_ir.validation_mutation_boundary] The validation command MUST be non-mutating by default. Any workflow that updates generated fixtures, schemas, projections, or golden receipts MUST require an explicit update mode and MUST emit a receipt describing every changed identity.

#### Scenario: Default validation does not rewrite golden receipts

r[kamacite.lossless_ir.validation_mutation_boundary.default_no_rewrite]

- GIVEN a golden receipt differs from validation output
- WHEN the validation command runs in default mode
- THEN it MUST fail with a receipt-mismatch diagnostic and MUST NOT rewrite the golden receipt.

#### Scenario: Explicit update records changed identities

r[kamacite.lossless_ir.validation_mutation_boundary.explicit_update]

- GIVEN an operator runs an accepted update workflow for generated artifacts or golden receipts
- WHEN the workflow writes changes
- THEN it MUST emit a receipt listing changed fixture IDs, old identities, new identities, checked laws, and non-claims.

### Requirement: Valence receipt export command

r[kamacite.valence_receipt_export.command] Kamacite MUST provide an explicit command or mode for producing or validating Valence receipt export projections. The command MUST read explicit receipt inputs, call pure core export logic, emit deterministic reports, and remain non-mutating by default. It MUST NOT invoke Valence, run external evaluators, inspect ambient repository state beyond declared inputs, or silently upgrade Kamacite receipt claims.

#### Scenario: Export command uses explicit inputs

r[kamacite.valence_receipt_export.command.explicit_inputs]

- GIVEN an operator requests a Valence receipt export
- WHEN the export command runs
- THEN it MUST consume explicit receipt, manifest, or fixture inputs and MUST NOT discover unrelated repository files as evidence.

#### Scenario: Export command is non-mutating by default

r[kamacite.valence_receipt_export.command.non_mutating]

- GIVEN an export output or golden projection differs from newly computed output
- WHEN the Valence export command runs in default mode
- THEN it MUST report the mismatch and MUST NOT rewrite the source receipt, manifest, export projection, or golden evidence.

#### Scenario: Export command fails closed

r[kamacite.valence_receipt_export.command.fail_closed]

- GIVEN receipt input is malformed, unsupported, stale, missing required identity domains, missing non-claims, or carries unsupported semantic-equivalence claims
- WHEN the Valence export command validates the input
- THEN it MUST fail closed with deterministic diagnostics before emitting a trusted export projection.

#### Scenario: Explicit update records changed export identities

r[kamacite.valence_receipt_export.command.explicit_update]

- GIVEN an operator intentionally refreshes Valence export golden projections
- WHEN the command writes updated projection evidence
- THEN it MUST emit an update receipt listing changed fixture IDs, source receipt identities, old export identities, new export identities, checked laws, diagnostics, and non-claims.

### Requirement: Steel adapter validation command

r[kamacite.steel_adapter_suite.command] Kamacite MUST provide an explicit command or mode for producing or validating Steel adapter execution receipts. The command MUST read explicit manifest, source, module, input DTO, conformance, and golden inputs; run Steel only when the command mode permits it; call pure core validation logic with normalized DTOs; emit deterministic reports; and remain non-mutating by default. It MUST NOT silently execute Steel during default validation, discover unrelated repository files as evidence, grant ambient host authority, or upgrade Steel outputs into canonical Kamacite IR.

#### Scenario: Steel command uses explicit inputs

r[kamacite.steel_adapter_suite.command.explicit_inputs]

- GIVEN an operator requests Steel adapter validation
- WHEN the Steel-capable command runs
- THEN it MUST consume explicit manifest, source, module, DTO, conformance, golden, evaluator, and capability inputs and MUST NOT discover unrelated repository files as evidence.

#### Scenario: Steel command is non-mutating by default

r[kamacite.steel_adapter_suite.command.non_mutating]

- GIVEN a Steel output, adapter receipt, manifest projection, or golden receipt differs from newly computed output
- WHEN the Steel-capable command runs in default mode
- THEN it MUST report the mismatch and MUST NOT rewrite source fixtures, module descriptors, manifest projections, output projections, or golden evidence.

#### Scenario: Steel command fails closed

r[kamacite.steel_adapter_suite.command.fail_closed]

- GIVEN Steel input is malformed, unsupported, stale, missing required identity domains, missing evaluator identity, missing non-claims, observes undeclared capabilities, or carries canonical-IR or semantic-equivalence overclaims
- WHEN the Steel-capable command validates the input or execution receipt
- THEN it MUST fail closed with deterministic diagnostics before emitting a trusted adapter receipt.

#### Scenario: Default validation remains receipt-only

r[kamacite.steel_adapter_suite.command.default_receipt_only]

- GIVEN default validation encounters Steel adapter fixture entries
- WHEN no Steel-capable command or mode is selected
- THEN validation MUST validate stored descriptor and receipt evidence only and MUST NOT evaluate Steel code, load Steel modules, register host functions, invoke external processes, or treat stored output as a fresh run.

#### Scenario: Explicit update records changed Steel identities

r[kamacite.steel_adapter_suite.command.explicit_update]

- GIVEN an operator intentionally refreshes Steel adapter golden receipts or output projections
- WHEN the command writes updated evidence
- THEN it MUST emit an update receipt listing changed fixture IDs, source identities when present, evaluator identities, module or descriptor identities, old output identities, new output identities, old receipt identities, new receipt identities, checked laws, diagnostics, and non-claims.

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
