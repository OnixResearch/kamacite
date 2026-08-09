# Nickel Adapter Suite Specification

## Purpose

Define the admitted evidence, command, and receipt boundary for adapting a verified Nickel JSON export into canonical Preserves without reevaluating Nickel.

## Requirements

### Requirement: Only admitted Nickel export artifacts enter conversion

r[kamacite.nickel_preserves_adapter.admission] The adapter MUST accept only a selected `format = "json"` export from a manifest admitted by the canonical pinned `nickel-export-core` manifest API. It MUST verify the supplied output bytes, BLAKE3 identity, byte length, destination, evaluator cohort, and manifest/receipt invariants before decoding JSON. A raw manifest, self-authored integrity report, or matching filename MUST NOT establish admission.

#### Scenario: Verified JSON export is selected

- GIVEN a canonical Nickel export manifest is admitted and its selected JSON artifact bytes match the receipt
- WHEN the Kamacite adapter prepares conversion input
- THEN it MUST pass a normalized in-memory admitted-export DTO to the pure conversion core with the manifest, receipt, evaluator, source, and output identities still distinct.

#### Scenario: Output bytes drift after export

- GIVEN a manifest names one JSON output identity but the supplied artifact bytes or byte length differ
- WHEN adapter admission runs
- THEN conversion MUST fail before JSON decoding and MUST NOT emit canonical Preserves bytes or a success receipt.

### Requirement: Nickel export verification stays outside the Kamacite core

r[kamacite.nickel_preserves_adapter.composition] `kamacite-core` MUST NOT invoke Nickel, read a manifest, inspect files, spawn `nickel-export`, or depend on ambient evaluator state. The CLI shell MAY use the pinned pure `nickel-export-core` admission API, MUST normalize the resulting verified fields into Kamacite-owned DTOs, and MUST keep file reads, writes, path authority, argument handling, and presentation outside the core.

#### Scenario: Existing export is adapted

- GIVEN a valid manifest and its exact generated JSON artifact already exist
- WHEN an operator runs the adapter
- THEN the command MUST verify and adapt those artifacts without invoking a Nickel evaluator or claiming that conversion proves export freshness beyond the supplied admitted evidence.

### Requirement: Adapter receipts preserve the full identity chain

r[kamacite.nickel_preserves_adapter.receipt] A successful adapter receipt MUST bind the Nickel export manifest identity, export receipt identity, declared-input identity, source identity, evaluator artifact and plan identities, JSON output identity, mapping-profile identity, canonical-codec identity, canonical packed Preserves identity, output byte length, checked laws, diagnostics, and explicit non-claims. Optional Preserves text output MUST have a separate projection identity and MUST NOT replace packed canonical identity.

#### Scenario: Canonical adaptation succeeds

- GIVEN admitted JSON bytes map successfully under the selected profile and encode through the canonical codec
- WHEN the adapter constructs its receipt
- THEN the receipt MUST make the source, JSON projection, canonical Preserves value, and optional text projection independently auditable without reusing one identity domain for another.

### Requirement: Conversion commands are explicit and fail closed

r[kamacite.nickel_preserves_adapter.command] Kamacite MUST expose an explicit Nickel-export adaptation command with mutually exclusive non-mutating check and explicit write modes. Check mode MUST compare existing packed output and receipt identities without mutation. Write mode MUST use the root-confined output gateway and MUST NOT report success or publish a trusted receipt after a partial, failed, escaped, or no-op materialization.

#### Scenario: Existing output is stale

- GIVEN an admitted Nickel JSON export maps to canonical bytes different from the checked packed output or receipt
- WHEN check mode runs
- THEN the command MUST exit unsuccessfully with deterministic first-mismatch evidence and MUST leave every file unchanged.

#### Scenario: Output path escapes the reviewed root

- GIVEN a write destination uses an absolute path, parent traversal, symlink traversal, or replace-time escape
- WHEN write mode opens or publishes outputs
- THEN the command MUST deny the mutation before trusted output or receipt publication.

### Requirement: Adapter fixtures cover success and failure

r[kamacite.nickel_preserves_adapter.fixtures] The adapter MUST include positive fixtures for admitted conversion, canonical stability, check mode, explicit write planning, and optional text projection. It MUST include negative fixtures for malformed or unknown manifest schemas, non-JSON formats, stale manifest/receipt/artifact identities, malformed JSON, unsupported mapping or codec versions, policy-limit violations, detached canonical bytes, path escapes, partial publication, and arbitrary-Nickel-equivalence overclaims.

#### Scenario: Unsupported input fails deterministically

- GIVEN an input violates one admission, mapping, canonicalization, authority, or non-claim rule
- WHEN the focused adapter suite runs
- THEN it MUST reject the input with a stable diagnostic class and MUST NOT emit a successful canonical artifact receipt.
