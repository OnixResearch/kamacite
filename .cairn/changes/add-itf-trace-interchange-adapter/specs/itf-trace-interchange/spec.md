# ITF Trace Interchange Adapter Specification

## Purpose

Define a bounded adapter that converts ITF traces from external model checkers into canonical Preserves values with layer-separated BLAKE3 identity, fail-closed validation, and deterministic receipts.

## ADDED Requirements

### Requirement: ITF is an adapter input, not canonical IR

r[kamacite.itf_interchange.adapter]

Kamacite MUST preserve ITF source bytes as a source envelope, MUST convert the trace into canonical Preserves values through the versioned adapter, and MUST NOT treat ITF JSON as canonical Kamacite IR.

#### Scenario: Valid ITF conversion succeeds
- GIVEN ITF source bytes that satisfy the declared ITF version and mapping
- WHEN adapter conversion runs
- THEN Kamacite MUST emit a converted Preserves value and an adapter receipt without relabeling ITF as canonical IR.

#### Scenario: ITF claims canonical authority
- GIVEN an adapter or receipt claims that ITF replaces Preserves identity
- WHEN validation runs
- THEN Kamacite MUST reject the overclaim.

### Requirement: Layers keep independent identity

r[kamacite.itf_interchange.identity]

Kamacite MUST bind ITF version, parser/tool identity, ITF source bytes, converted Preserves value, adapter descriptor, schema references, and receipt as independent BLAKE3 identities with explicit parent edges.

#### Scenario: Identity set is complete
- GIVEN every layer identity and parent edge is present and role-correct
- WHEN identity validation runs
- THEN Kamacite MUST produce one deterministic adapter identity set.

#### Scenario: Identity set drifts
- GIVEN source bytes change while the declared version or descriptor stays fixed
- WHEN freshness validation runs
- THEN Kamacite MUST require a new source identity and MUST reject stale descriptors or receipts.

### Requirement: Admission fails closed

r[kamacite.itf_interchange.validation]

Kamacite MUST reject malformed, stale-version, schema-mismatched, or tampered ITF traces with deterministic denials, and MUST NOT admit a partial conversion as a trace.

#### Scenario: Trace is malformed or stale
- GIVEN ITF bytes fail parsing, use a non-admitted version, or violate a schema
- WHEN admission runs
- THEN Kamacite MUST deny conversion and MUST record the denial.

#### Scenario: Trace is tampered
- GIVEN the receipt or descriptor references a different source, version, or converted value than the supplied bytes
- WHEN admission runs
- THEN Kamacite MUST fail closed.

### Requirement: Receipts preserve every layer

r[kamacite.itf_interchange.receipt]

Kamacite MUST emit deterministic conversion or denial receipts binding ITF version, source envelope, adapter descriptor, converted Preserves value, schema refs, parser/tool identity, result class, parent edges, and non-claims.

#### Scenario: Passing receipt is validated
- GIVEN a successful conversion and a complete receipt
- WHEN receipt validation runs
- THEN every layer identity and parent edge MUST match the admitted bytes and conversion.

#### Scenario: Receipt swaps a layer
- GIVEN a receipt refers to different source, version, descriptor, or converted value
- WHEN receipt validation runs
- THEN Kamacite MUST fail closed.

### Requirement: Non-overclaim is mandatory

r[kamacite.itf_interchange.nonclaim]

Every adapter receipt MUST carry the non-claim that it proves only the declared byte-level conversion and identity binding, and MUST NOT claim semantic equivalence, checker correctness, losslessness, or trace semantic validity.

#### Scenario: Non-claim is recorded
- GIVEN a receipt includes the bounded non-claim
- WHEN receipt validation runs
- THEN the non-claim MUST remain attached.

### Requirement: Suite has positive and negative fixtures

r[kamacite.itf_interchange.fixtures]

The interchange suite MUST include positive fixtures for valid conversions and negative fixtures for malformed, wrong-version, schema-mismatch, tamper, missing-layer, and equivalence-overclaim cases.

#### Scenario: Suite runs
- GIVEN all fixtures are committed with their declared identities
- WHEN the validation harness executes
- THEN positive fixtures MUST pass and each negative fixture MUST fail at its declared boundary.

### Requirement: Adapter changes run full validation

r[kamacite.itf_interchange.validation_rail]

Changes to the adapter descriptor, mapping, schemas, validation, receipts, or fixtures MUST run focused core tests, manifest freshness, positive and negative adapter validation, lifecycle validation, Cairn gates, and relevant flake checks.

#### Scenario: Change is validated
- GIVEN an adapter change passes all focused checks
- WHEN lifecycle validation is assembled
- THEN it MUST include both positive and negative results and MUST identify the exact ITF version, parser/tool identity, and schema refs.
