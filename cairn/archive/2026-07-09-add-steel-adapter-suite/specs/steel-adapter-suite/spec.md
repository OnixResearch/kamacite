# Steel Adapter Suite Specification

## Purpose

Defines the executable Steel adapter suite that proves Kamacite's bounded Steel probe, typed host-function, evaluator-identity, output-projection, and no-ambient-authority boundaries.

## Requirements

### Requirement: Steel adapter execution boundary

r[kamacite.steel_adapter_suite.execution_boundary] Kamacite MUST execute Steel only in an explicit effectful CLI or adapter-runner shell. The trusted core MUST NOT call Steel, inspect the environment, spawn evaluator processes, register host functions, discover files, load modules, or infer Steel semantics. The shell MUST normalize Steel source bytes, evaluator identity, module/import declarations, host-function declarations, declared capabilities, observed effects, output bytes, output identity, diagnostics, and receipt facts into explicit DTOs before invoking the core.

#### Scenario: Steel execution stays outside the core

r[kamacite.steel_adapter_suite.execution_boundary.shell_only]

- GIVEN a Steel fixture requires evaluating Steel source, modules, contracts, or host-function calls
- WHEN the fixture is admitted for trusted validation
- THEN evaluation MUST happen outside `kamacite-core` and the core MUST receive only normalized in-memory evidence.

#### Scenario: Steel-capable command is explicit

r[kamacite.steel_adapter_suite.execution_boundary.explicit_command]

- GIVEN the normal walking-skeleton harness can validate pure and stored adapter evidence without external evaluators
- WHEN Steel adapter execution is required
- THEN Kamacite MUST provide an explicit Steel-capable command or mode and MUST NOT silently treat prerecorded Steel output as a fresh evaluator run.

#### Scenario: Stored evidence mode does not execute Steel

r[kamacite.steel_adapter_suite.execution_boundary.stored_evidence]

- GIVEN default validation loads Steel adapter descriptors, receipts, or golden evidence
- WHEN no explicit Steel-capable command or mode is selected
- THEN validation MUST check stored evidence only and MUST NOT execute Steel code, load Steel modules, or register host functions.

### Requirement: Steel adapter receipt profile

r[kamacite.steel_adapter_suite.receipt_profile] Steel adapter receipts MUST bind evaluator identity, optional source identity, module or descriptor identity, input DTO schema, output DTO schema, declared host functions, declared built-in modules, declared imports, required capabilities, observed effects, effect class, output projection identity, checked laws, diagnostics, and explicit non-claims. Missing, malformed, unsupported, stale, or cross-domain identities MUST fail closed before results are trusted.

#### Scenario: Evaluator identity is bound

r[kamacite.steel_adapter_suite.receipt_profile.evaluator_identity]

- GIVEN a Steel adapter fixture is accepted as executed evidence
- WHEN Kamacite emits the adapter receipt
- THEN the receipt MUST bind a distinct Steel evaluator identity with deterministic profile/version data and MUST NOT infer host authority, evaluator soundness, or semantic equivalence from that identity.

#### Scenario: Source and module identities are separate

r[kamacite.steel_adapter_suite.receipt_profile.source_module_identity]

- GIVEN a Steel fixture has source bytes, module descriptors, adapter descriptors, or declared imports
- WHEN Kamacite validates the receipt
- THEN each artifact identity MUST remain in its declared typed domain and MUST NOT substitute for output projection or canonical Preserves identity.

#### Scenario: Output projection identity is bound

r[kamacite.steel_adapter_suite.receipt_profile.output_projection]

- GIVEN a Steel fixture emits DTO or projection bytes
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST compute the output projection identity, compare it with the declared projection identity, and reject stale or wrong-domain output identities.

### Requirement: Steel host-function and import boundary

r[kamacite.steel_adapter_suite.host_function_boundary] Steel adapter descriptors MUST declare every trusted host function, built-in module, and import before execution evidence is trusted. Host-function declarations MUST include stable id, input DTO schema, output DTO schema, effect class, required capabilities, and checked laws. Built-in modules and imports MUST be declared by descriptor. Missing, extra, or undeclared host functions, modules, imports, effects, or capabilities MUST fail closed.

#### Scenario: Declared host functions are trusted inputs

r[kamacite.steel_adapter_suite.host_function_boundary.declared_host_functions]

- GIVEN a Steel fixture calls a Rust host function registered through the evaluator shell
- WHEN Kamacite validates the adapter receipt
- THEN the host function MUST be declared with input/output DTO contracts, effect class, required capabilities, and checked laws before the receipt is accepted.

#### Scenario: Undeclared host function is rejected

r[kamacite.steel_adapter_suite.host_function_boundary.undeclared_host_function]

- GIVEN a Steel fixture calls or records a host function not declared by the adapter descriptor
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST fail closed with deterministic undeclared-host-function evidence.

#### Scenario: Ambient imports are rejected

r[kamacite.steel_adapter_suite.host_function_boundary.ambient_import]

- GIVEN Steel code imports or requests filesystem, process, TCP, random, time, package, dylib, credential, mutation, or other ambient authority outside the declared adapter contract
- WHEN Kamacite validates the adapter request or receipt
- THEN validation MUST fail closed with deterministic undeclared-authority evidence.

### Requirement: Steel output and callable boundary

r[kamacite.steel_adapter_suite.output_boundary] Steel adapter outputs MUST be declared DTO values, projection bytes, descriptors, diagnostics, or receipts. Steel bytecode, opaque closures, dynamically loaded modules, JSON strings, or evaluator-internal values MUST NOT be treated as canonical Kamacite IR. Callable and contract exports MAY be recorded only as descriptor-bound evidence with explicit non-claims.

#### Scenario: DTO-only output is accepted

r[kamacite.steel_adapter_suite.output_boundary.dto_only]

- GIVEN a Steel adapter fixture consumes declared DTO inputs and emits declared DTO or projection output under the adapter descriptor
- WHEN evaluator identity, host functions, output identity, checked laws, and non-claims all match
- THEN validation MAY accept the receipt as bounded adapter evidence.

#### Scenario: Opaque closure export is rejected

r[kamacite.steel_adapter_suite.output_boundary.opaque_closure]

- GIVEN a Steel module exports an opaque closure, continuation, or evaluator-internal callable as a portable Kamacite value
- WHEN Kamacite validates the receipt
- THEN validation MUST reject the value unless it is represented only as a descriptor with stable id, arity, source or module identity, input/output contracts, effect class, and non-claims.

#### Scenario: Steel bytecode is not canonical IR

r[kamacite.steel_adapter_suite.output_boundary.bytecode_not_ir]

- GIVEN a fixture, adapter descriptor, or receipt claims Steel bytecode, JSON strings, closure serialization, or evaluator-internal values are canonical Kamacite IR
- WHEN Kamacite validates the claim
- THEN validation MUST reject the claim with deterministic canonical-IR overclaim evidence.

### Requirement: Steel adapter positive fixtures

r[kamacite.steel_adapter_suite.positive_fixtures] The Steel adapter suite MUST include positive fixtures for a DTO-only Steel adapter run, evaluator-identity-bound receipt, declared host-function use, declared module/import boundary, output projection identity, checked laws, explicit non-claims, and deterministic golden receipt comparison.

#### Scenario: Valid DTO-only Steel fixture passes

r[kamacite.steel_adapter_suite.positive_fixtures.dto_only]

- GIVEN a Steel fixture has declared DTO input/output schemas, evaluator identity, declared host functions, declared modules/imports, declared capabilities, output projection identity, checked laws, and non-claims
- WHEN the Steel adapter suite validates the fixture
- THEN validation MUST accept it and emit a deterministic golden receipt.

#### Scenario: Declared module fixture passes without ambient authority

r[kamacite.steel_adapter_suite.positive_fixtures.declared_module]

- GIVEN a Steel fixture references only declared built-in modules or declared local module descriptors
- WHEN Kamacite validates the adapter receipt
- THEN validation MAY accept the receipt and MUST preserve the no-ambient-authority non-claim.

### Requirement: Steel adapter negative fixtures

r[kamacite.steel_adapter_suite.negative_fixtures] The Steel adapter suite MUST include negative fixtures for missing evaluator identity, unsupported evaluator profile, undeclared host function, undeclared ambient authority, stale output projection identity, wrong identity domain, malformed output, opaque closure export, and Steel bytecode or JSON canonical-IR overclaim.

#### Scenario: Missing evaluator identity is rejected

r[kamacite.steel_adapter_suite.negative_fixtures.missing_evaluator]

- GIVEN Steel output evidence is present without a Steel evaluator identity
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject it before trusting the output.

#### Scenario: Unsupported evaluator is rejected

r[kamacite.steel_adapter_suite.negative_fixtures.unsupported_evaluator]

- GIVEN a Steel adapter fixture names an evaluator outside the accepted Steel profile for the suite
- WHEN Kamacite validates the receipt
- THEN validation MUST reject it or classify it as untrusted external evidence.

#### Scenario: Undeclared ambient authority is rejected

r[kamacite.steel_adapter_suite.negative_fixtures.ambient_authority]

- GIVEN a Steel fixture requests filesystem, process, TCP, random, time, package, dylib, credential, mutation, or other host authority not declared by the adapter descriptor and command mode
- WHEN Kamacite validates the receipt
- THEN validation MUST fail closed with deterministic undeclared-capability evidence.

#### Scenario: Stale output identity is rejected

r[kamacite.steel_adapter_suite.negative_fixtures.stale_output]

- GIVEN Steel output bytes do not match the declared output projection identity
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST fail closed with deterministic stale-output evidence.

#### Scenario: Malformed output is rejected

r[kamacite.steel_adapter_suite.negative_fixtures.malformed_output]

- GIVEN Steel output bytes cannot be decoded under the declared output DTO or projection schema
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject the fixture with deterministic malformed-output evidence.

### Requirement: Steel adapter golden receipts

r[kamacite.steel_adapter_suite.golden_receipts] Steel adapter validation MUST emit deterministic golden receipts that bind source identity when present, evaluator identity, module or descriptor identity, host-function declarations, output projection identity, checked laws, diagnostics, requirement IDs, and explicit non-claims. Default validation MUST compare receipts without mutating evidence.

#### Scenario: Steel receipt matches golden evidence

r[kamacite.steel_adapter_suite.golden_receipts.match]

- GIVEN the Steel source, evaluator identity, descriptor facts, output projection, manifest metadata, and golden receipt are unchanged
- WHEN the Steel adapter suite runs in the same execution mode
- THEN the emitted receipt MUST match the checked golden receipt exactly.

#### Scenario: Steel receipt update is explicit

r[kamacite.steel_adapter_suite.golden_receipts.explicit_update]

- GIVEN a Steel evaluator version, descriptor, output bytes, fixture metadata, or receipt schema changes
- WHEN a golden receipt update is requested
- THEN the update evidence MUST name changed fixtures, old and new source/evaluator/module/output/receipt identities where applicable, requirement coverage, diagnostics, checked laws, and non-claims.

### Requirement: Steel adapter capability boundary

r[kamacite.steel_adapter_suite.capability_boundary] Steel adapter execution MUST declare the exact shell capabilities required to read fixture inputs, invoke or embed the Steel evaluator, register declared host functions, load declared modules, and capture output bytes. Undeclared network, clock, random, credential, package-manager, dynamic-library, mutation, ambient filesystem, or ambient process authority MUST fail closed.

#### Scenario: Root-relative fixture reads only

r[kamacite.steel_adapter_suite.capability_boundary.root_relative]

- GIVEN a Steel adapter fixture reads source, module, input DTO, or expected-output files
- WHEN the shell opens those paths
- THEN paths MUST be root-relative fixture paths and MUST NOT escape the reviewed fixture root.

#### Scenario: Evaluator invocation is not ambient process authority

r[kamacite.steel_adapter_suite.capability_boundary.evaluator_invocation]

- GIVEN the shell invokes a Steel evaluator or embeds a Steel engine for a reviewed fixture
- WHEN Kamacite records the receipt
- THEN the receipt MUST classify that execution as the declared evaluator capability and MUST NOT grant general ambient process, filesystem, module-loading, or host-authority access to adapters.

#### Scenario: Capability mismatch fails closed

r[kamacite.steel_adapter_suite.capability_boundary.mismatch]

- GIVEN observed Steel adapter effects exceed declared adapter capabilities or command-mode capabilities
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject the receipt before trusting the output projection.
