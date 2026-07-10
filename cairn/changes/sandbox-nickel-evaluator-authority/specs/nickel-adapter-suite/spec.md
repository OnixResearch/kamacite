# Nickel Adapter Suite Specification

## Purpose

Enforce the declared capability boundary around explicit Nickel reference evaluation.

## Requirements

### Requirement: Nickel execution uses a receipt-bound sandbox profile

r[kamacite.findings.nickel_sandbox.profile] Every explicit Nickel evaluator run MUST use a versioned sandbox profile that declares evaluator identity policy, readable source/import closure, environment policy, network/process policy, writable outputs, and named resource limits.

#### Scenario: Reference export is requested

- GIVEN a reviewed Nickel fixture requests reference export
- WHEN the shell invokes the evaluator
- THEN the evaluator MUST run under the declared sandbox profile and the adapter receipt MUST bind that profile identity.

### Requirement: Imports stay within the reviewed closure

r[kamacite.findings.nickel_sandbox.import_closure] Nickel source and imports MUST be resolved only from a reviewed read-only closure. Absolute imports, parent escapes, undeclared package inputs, and symlink escapes MUST fail closed before export evidence is trusted.

#### Scenario: Source imports outside the fixture root

- GIVEN a root-relative Nickel source imports a file outside the admitted closure
- WHEN explicit adapter validation runs
- THEN the evaluator run MUST be denied or isolated from that file and validation MUST emit deterministic undeclared-import evidence.

### Requirement: Enforced capabilities replace self-reported observations

r[kamacite.findings.nickel_sandbox.enforced_evidence] Nickel receipts MUST distinguish requested, sandbox-admitted, and observed execution facts. Manifest-provided observed-effect fields MUST NOT by themselves prove that ambient filesystem, environment, network, process, credential, or mutation authority was absent.

#### Scenario: Manifest claims no ambient effects

- GIVEN a manifest records only declared evaluator capabilities but the sandbox policy would expose an undeclared host resource
- WHEN Kamacite evaluates trust
- THEN the run MUST be rejected regardless of the manifest's self-reported observations.

### Requirement: Evaluator runs are bounded

r[kamacite.findings.nickel_sandbox.resource_limits] The Nickel shell MUST enforce named execution, memory, and output limits, terminate exceeded runs, bound captured diagnostics, and report the limit outcome deterministically.

#### Scenario: Evaluator does not terminate within policy

- GIVEN a Nickel evaluation exceeds the declared execution budget
- WHEN the shell reaches the budget
- THEN it MUST terminate the run, emit a bounded resource-limit diagnostic, and MUST NOT produce a trusted export receipt.
