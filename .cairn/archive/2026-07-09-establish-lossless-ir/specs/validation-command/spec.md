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
