# Documentation Pipeline Specification

## Purpose

Make documentation command success mean that the named checked action actually completed.

## Requirements

### Requirement: Documentation configuration is evaluated and validated

r[kamacite.findings.docs_commands.config_validation] Every documentation action MUST consume a Nickel configuration that has been evaluated against the accepted docs contract or a freshness-checked projection receipt binding the source, contract, evaluator, and projection identities.

#### Scenario: Readable invalid Nickel is supplied

- GIVEN a readable `.ncl` file contains malformed Nickel or violates the docs contract
- WHEN `kamacite docs check` runs
- THEN the command MUST exit unsuccessfully with deterministic config diagnostics and MUST NOT report a checked config identity.

### Requirement: Each action completes its named work

r[kamacite.findings.docs_commands.action_completion] `collect`, `check`, and `render` MUST invoke their pure planning/validation cores and complete the required declared shell work before returning success. `run-cells` and `update` MUST return success only after admitted execution or mutation actually occurs.

#### Scenario: Render backend is not implemented

- GIVEN an operator requests a supported-looking render action whose backend is unavailable
- WHEN the command dispatches the action
- THEN it MUST fail as unsupported or incomplete and MUST NOT treat action admission as successful rendering.

### Requirement: No-op execution and mutation fail closed

r[kamacite.findings.docs_commands.no_op_rejection] A report with `executed = false` MUST NOT satisfy a requested `run-cells` action, and a report with `mutated = false` MUST NOT satisfy a requested `update` action unless an explicit dry-run mode was requested and reported as a plan rather than execution.

#### Scenario: Update action performs no writes

- GIVEN `docs update` is requested without dry-run semantics
- WHEN the shell declines to mutate
- THEN the command MUST exit unsuccessfully and MUST NOT emit a completed update receipt.

### Requirement: Documentation actions emit bounded evidence

r[kamacite.findings.docs_commands.receipts] Successful documentation actions MUST emit deterministic evidence binding checked config, inputs, action/profile, outputs or diagnostics, checked laws, non-claims, and receipt identity.

#### Scenario: Check succeeds

- GIVEN a valid checked config and valid documentation facts/IR inputs
- WHEN `docs check` completes
- THEN the report MUST identify the validated artifacts and receipt rather than only hashing raw config bytes.
