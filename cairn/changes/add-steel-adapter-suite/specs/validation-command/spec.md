# Validation Command Specification

## Purpose

Defines the deterministic validation command that will check Kamacite schemas, fixtures, adapters, target evidence, and receipts.

## Requirements

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
