# Function Address Receipts Specification

## Purpose

Add a first-class Kamacite CLI adapter for Octet function-address reports.

## Requirements

### Requirement: CLI emits portable function-address receipts
r[kamacite.function_address_receipts.cli_adapter.command] Kamacite MUST provide a CLI command that ingests an Octet function-address report and emits a validated Kamacite function-address receipt.

#### Scenario: Valid Octet report becomes Kamacite receipt
r[kamacite.function_address_receipts.cli_adapter.positive]
- GIVEN an Octet function-address report has supported producer metadata, source envelopes, function records, BLAKE3 identities, and required non-claims
- WHEN the Kamacite CLI validates it with matching source envelope bytes
- THEN the CLI MUST emit a valid Kamacite function-address receipt and receipt identity.

### Requirement: CLI shell owns I/O
r[kamacite.function_address_receipts.cli_adapter.shell] Kamacite CLI filesystem reads, path resolution, stdout/stderr, and file writes MUST remain outside `kamacite-core` receipt validation.

#### Scenario: Core receives in-memory input
r[kamacite.function_address_receipts.cli_adapter.validation]
- GIVEN the CLI has loaded and parsed an Octet function-address report
- WHEN it invokes receipt validation
- THEN `kamacite-core` MUST receive in-memory typed input without reading files or environment state.

### Requirement: Valence projection output is explicit
r[kamacite.function_address_receipts.cli_adapter.valence_projection] Kamacite SHOULD provide an explicit CLI output for the function-address Valence projection derived from a valid Kamacite receipt.

#### Scenario: Projection is emitted with receipt identity
r[kamacite.function_address_receipts.cli_adapter.negative]
- GIVEN a valid Kamacite function-address receipt is emitted
- WHEN the operator requests Valence projection output
- THEN the projection MUST bind the source Kamacite receipt identity and MUST fail closed if receipt validation failed.
