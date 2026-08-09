# Validation Command Specification

## Purpose

Defines the deterministic validation command that will check Kamacite schemas, fixtures, adapters, target evidence, and receipts.

## Requirements

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
