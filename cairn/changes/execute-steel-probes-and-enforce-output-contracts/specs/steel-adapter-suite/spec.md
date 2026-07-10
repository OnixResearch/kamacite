# Steel Adapter Suite Specification

## Purpose

Ensure explicit Steel probe evidence comes from an actual bounded execution and schema-valid DTO values.

## Requirements

### Requirement: Stored and executed Steel evidence are distinct

r[kamacite.findings.steel_execution.fresh_probe] Kamacite MUST model stored Steel evidence and freshly executed probe evidence as distinct alternatives. An executed receipt MUST require an actual admitted evaluator run and MUST NOT be constructed by changing evaluator descriptor text while reusing prerecorded output.

#### Scenario: DTO probe mode is selected

- GIVEN a Steel fixture declares executed DTO-probe mode
- WHEN `validate-steel-adapter` runs
- THEN the shell MUST execute the admitted source/module/input under the declared engine or fail without emitting executed evidence.

### Requirement: Evaluator identity comes from the runner

r[kamacite.findings.steel_execution.runner_identity] Executed Steel receipts MUST bind evaluator/engine version and profile derived from the actual runner, plus source, module, input, host-function, capability, output, and exit-result identities.

#### Scenario: Caller supplies an arbitrary descriptor label

- GIVEN a caller-provided evaluator label does not match the actual admitted runner identity
- WHEN executed evidence is constructed
- THEN validation MUST reject the mismatch and MUST NOT refresh golden receipts under the caller label.

### Requirement: DTO schemas are enforced

r[kamacite.findings.steel_execution.schema_conformance] Steel input and output schema references MUST resolve to content-addressed admitted contracts, and the pure core MUST validate decoded values against those contracts. Nonempty schema strings and generic JSON syntax MUST NOT establish DTO conformance.

#### Scenario: JSON has the wrong shape

- GIVEN an output schema requires an integer DTO but execution returns a syntactically valid object
- WHEN Kamacite validates the output
- THEN validation MUST fail with deterministic output-schema evidence even when the projection hash matches the object bytes.

### Requirement: Executed probes enforce declared authority

r[kamacite.findings.steel_execution.capability_enforcement] The Steel runner MUST register only declared host functions and modules, deny undeclared ambient imports and capabilities, and record enforced observations in the execution receipt.

#### Scenario: Steel requests an undeclared host function

- GIVEN Steel code calls a host function absent from the admitted descriptor
- WHEN the explicit probe runs
- THEN the runner MUST deny the call and validation MUST reject the execution before trusting output bytes.
