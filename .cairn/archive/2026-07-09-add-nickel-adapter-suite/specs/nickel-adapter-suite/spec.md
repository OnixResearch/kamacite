# Nickel Adapter Suite Specification

## Purpose

Defines the executable Nickel adapter suite that proves Kamacite's source-envelope, derived-projection, evaluator-identity, and receipt boundaries for Nickel-authored inputs.

## Requirements

### Requirement: Nickel adapter execution boundary

r[kamacite.nickel_adapter_suite.execution_boundary] Kamacite MUST execute Nickel only in an effectful CLI or adapter-runner shell. The trusted core MUST NOT call Nickel, inspect the environment, spawn processes, discover files, or infer evaluator semantics. The shell MUST normalize Nickel source bytes, export bytes, evaluator identity, declared capabilities, diagnostics, projection identity, and receipt facts into explicit DTOs before invoking the core.

#### Scenario: Nickel execution stays outside the core

r[kamacite.nickel_adapter_suite.execution_boundary.shell_only]

- GIVEN a Nickel fixture requires `nickel export` or equivalent reference evaluation
- WHEN the fixture is admitted for trusted validation
- THEN the Nickel invocation MUST happen outside `kamacite-core` and the core MUST receive only normalized in-memory evidence.

#### Scenario: Nickel-capable command is explicit

r[kamacite.nickel_adapter_suite.execution_boundary.explicit_command]

- GIVEN the normal walking-skeleton harness can validate pure fixtures without external evaluators
- WHEN Nickel adapter execution is required
- THEN Kamacite MUST provide an explicit Nickel-capable command or mode that declares evaluator availability and MUST NOT silently trust stale prerecorded execution evidence as a fresh Nickel run.

### Requirement: Nickel source and export separation

r[kamacite.nickel_adapter_suite.source_export_separation] Nickel adapter receipts MUST keep Nickel source-envelope identity, export projection identity, and optional canonical Preserves identity in distinct typed domains. `nickel export` output MUST be classified as derived projection evidence and MUST NOT replace source identity or canonical Preserves identity.

#### Scenario: Export projection is derived evidence

r[kamacite.nickel_adapter_suite.source_export_separation.derived_projection]

- GIVEN a Nickel source fixture exports a value through the reference Nickel evaluator
- WHEN Kamacite records the adapter receipt
- THEN the receipt MUST bind the source identity and export projection identity separately and MUST state that the export is derived evidence.

#### Scenario: JSON export is not canonical IR

r[kamacite.nickel_adapter_suite.source_export_separation.json_not_canonical]

- GIVEN a Nickel export produces JSON bytes
- WHEN a fixture or receipt claims those JSON bytes are canonical Kamacite IR
- THEN validation MUST reject the claim with deterministic overclaim evidence.

### Requirement: Nickel evaluator identity

r[kamacite.nickel_adapter_suite.evaluator_identity] Every trusted Nickel export receipt MUST bind a Nickel evaluator or exporter identity. The evaluator identity MUST include enough deterministic version/profile data for review, and validation MUST reject missing, malformed, unsupported, or cross-domain evaluator identities.

#### Scenario: Evaluator identity is bound

r[kamacite.nickel_adapter_suite.evaluator_identity.bound]

- GIVEN a Nickel export fixture is accepted
- WHEN Kamacite emits the adapter receipt
- THEN the receipt MUST include a distinct evaluator identity and MUST NOT infer host authority or semantic equivalence from that identity.

#### Scenario: Missing evaluator identity is rejected

r[kamacite.nickel_adapter_suite.evaluator_identity.missing]

- GIVEN a Nickel export projection is present without an evaluator identity
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject it before trusting the projection.

#### Scenario: Unsupported evaluator identity is rejected

r[kamacite.nickel_adapter_suite.evaluator_identity.unsupported]

- GIVEN a Nickel export fixture names an evaluator outside the accepted Nickel reference profile
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject the fixture or classify it as untrusted external evidence.

### Requirement: Nickel adapter positive fixtures

r[kamacite.nickel_adapter_suite.positive_fixtures] The Nickel adapter suite MUST include positive fixtures for at least one valid Nickel source-envelope export, one source/export identity separation case, one evaluator-identity-bound receipt, and one parser/span non-claim case when parser evidence is included.

#### Scenario: Valid Nickel export fixture passes

r[kamacite.nickel_adapter_suite.positive_fixtures.valid_export]

- GIVEN a Nickel source envelope, declared evaluator identity, exported projection bytes, projection identity, checked laws, and non-claims all match
- WHEN the Nickel adapter suite validates the fixture
- THEN validation MUST accept it and emit a deterministic golden receipt.

#### Scenario: Parser evidence remains syntax-only

r[kamacite.nickel_adapter_suite.positive_fixtures.parser_non_claim]

- GIVEN a Nickel parser/span fixture uses tree-sitter Nickel or another syntax parser
- WHEN Kamacite records the parser evidence
- THEN the receipt MUST classify it as syntax/span evidence only and MUST NOT claim Nickel evaluation semantics.

### Requirement: Nickel adapter negative fixtures

r[kamacite.nickel_adapter_suite.negative_fixtures] The Nickel adapter suite MUST include negative fixtures for missing source-envelope evidence, stale export projection identity, JSON export canonical-IR overclaim, missing evaluator identity, unsupported evaluator identity, malformed export output, and undeclared host effects.

#### Scenario: Missing source envelope is rejected

r[kamacite.nickel_adapter_suite.negative_fixtures.missing_source]

- GIVEN a Nickel export projection is present without preserved Nickel source bytes and source identity
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject the fixture and MUST NOT trust the projection as source preservation evidence.

#### Scenario: Stale export projection identity is rejected

r[kamacite.nickel_adapter_suite.negative_fixtures.stale_projection]

- GIVEN Nickel export bytes do not match the declared projection identity
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST fail closed with deterministic stale-projection evidence.

#### Scenario: Malformed export output is rejected

r[kamacite.nickel_adapter_suite.negative_fixtures.malformed_export]

- GIVEN a Nickel export fixture records malformed JSON, malformed Preserves projection bytes, or output bytes that cannot be decoded under the declared projection schema
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject the fixture with deterministic malformed-output evidence.

#### Scenario: Undeclared host effect is rejected

r[kamacite.nickel_adapter_suite.negative_fixtures.undeclared_effect]

- GIVEN a Nickel adapter receipt records filesystem, process, network, clock, random, credential, mutation, package-manager, or other host effects beyond the declared Nickel execution capability
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject the fixture with deterministic undeclared-capability evidence.

### Requirement: Nickel adapter golden receipts

r[kamacite.nickel_adapter_suite.golden_receipts] Nickel adapter validation MUST emit deterministic golden receipts that bind source identity, evaluator identity, export projection identity, checked laws, diagnostics, requirement IDs, and explicit non-claims. Default validation MUST compare receipts without mutating evidence.

#### Scenario: Nickel receipt matches golden evidence

r[kamacite.nickel_adapter_suite.golden_receipts.match]

- GIVEN the Nickel source, evaluator identity, export projection, manifest metadata, and golden receipt are unchanged
- WHEN the Nickel adapter suite runs
- THEN the emitted receipt MUST match the checked golden receipt exactly.

#### Scenario: Nickel receipt update is explicit

r[kamacite.nickel_adapter_suite.golden_receipts.explicit_update]

- GIVEN a Nickel evaluator version, export bytes, fixture metadata, or receipt schema changes
- WHEN a golden receipt update is requested
- THEN the update evidence MUST name changed fixtures, old and new source/export/evaluator/receipt identities where applicable, requirement coverage, and non-claims.

### Requirement: Nickel adapter capability boundary

r[kamacite.nickel_adapter_suite.capability_boundary] Nickel adapter execution MUST declare the exact shell capabilities required to read source fixtures, invoke the Nickel reference evaluator, and capture export bytes. Undeclared network, clock, random, credential, package-manager, dynamic-library, mutation, or ambient filesystem authority MUST fail closed.

#### Scenario: Root-relative fixture reads only

r[kamacite.nickel_adapter_suite.capability_boundary.root_relative]

- GIVEN a Nickel adapter fixture reads source or expected-output files
- WHEN the shell opens those paths
- THEN paths MUST be root-relative fixture paths and MUST NOT escape the reviewed fixture root.

#### Scenario: Evaluator invocation is not ambient process authority

r[kamacite.nickel_adapter_suite.capability_boundary.evaluator_invocation]

- GIVEN the shell invokes the Nickel reference evaluator for a reviewed fixture
- WHEN Kamacite records the receipt
- THEN the receipt MUST classify the process invocation as the declared evaluator capability and MUST NOT grant general ambient process authority to adapters.
