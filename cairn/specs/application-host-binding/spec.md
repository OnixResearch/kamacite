# Application Host Binding Specification

## Purpose

Defines the `application-host-binding` capability.

## Requirements

### Requirement: Application-host bindings are canonical artifacts

r[kamacite.application_host_binding.artifact] Kamacite MUST define a versioned canonical Preserves `ApplicationHostBinding` artifact whose BLAKE3 identity binds one application artifact, one executable entrypoint, input and output schema identities, one closed effect-summary identity, one host-profile identity, one adapter-cohort identity, one resource-profile identity, required policy or compatibility identities, and explicit non-claims.

#### Scenario: Complete binding has stable identity

r[kamacite.application_host_binding.artifact.stable]
- GIVEN a binding contains every required semantic field in canonical form
- WHEN Kamacite derives the binding identity
- THEN the BLAKE3 identity over canonical Preserves bytes MUST be the normative binding identity
- AND changes to display names, aliases, help text, or source spans MUST NOT change that identity.

#### Scenario: Semantic binding field changes

r[kamacite.application_host_binding.artifact.rekey]
- GIVEN an application, entrypoint, schema, effect summary, host profile, adapter cohort, resource profile, policy, or compatibility identity changes
- WHEN Kamacite derives the binding identity
- THEN the new binding identity MUST differ from the prior identity.

### Requirement: One executable unit selects exactly one host

r[kamacite.application_host_binding.host_selection] An executable application-host binding MUST contain exactly one host-profile identity and MUST NOT contain missing, alternative, repeated, implicit, or fallback host selections.

#### Scenario: One host profile is selected

r[kamacite.application_host_binding.host_selection.valid]
- GIVEN one executable application unit names one supported host-profile identity
- WHEN binding admission runs
- THEN the host selection MUST pass subject to all other binding checks.

#### Scenario: Host selection is ambiguous

r[kamacite.application_host_binding.host_selection.ambiguous]
- GIVEN a binding omits the host profile or supplies multiple, alternative, repeated, implicit, or fallback host choices
- WHEN binding admission runs
- THEN Kamacite MUST reject the binding with a deterministic host-selection diagnostic.

### Requirement: Executable bindings use closed effect summaries

r[kamacite.application_host_binding.effects] An executable application-host binding MUST reference one closed Kamacite effect summary with exact semantic operation identities. Open, unknown, undeclared, or stale operation sets MUST NOT pass executable binding admission.

#### Scenario: Closed declared effects pass

r[kamacite.application_host_binding.effects.closed]
- GIVEN the binding references a closed effect summary whose operations are declared by exact semantic identities
- WHEN effect admission runs
- THEN the effect field MUST pass subject to host, adapter, capability, and policy checks owned outside the binding.

#### Scenario: Unknown effects deny executable use

r[kamacite.application_host_binding.effects.unknown]
- GIVEN an effect summary is open, unknown, stale, or contains an undeclared operation
- WHEN executable binding admission runs
- THEN Kamacite MUST reject executable use
- AND it MAY preserve the artifact as non-executable review data with explicit diagnostics.

### Requirement: Binding cohorts remain exact

r[kamacite.application_host_binding.cohorts] Application-host binding validation MUST require exact compatible application, entrypoint, schema, effect, host, adapter, resource, policy, and compatibility identities and MUST reject stale or role-mismatched substitutions.

#### Scenario: Exact cohort is admitted

r[kamacite.application_host_binding.cohorts.valid]
- GIVEN every linked identity belongs to the reviewed compatible cohort
- WHEN binding validation runs
- THEN Kamacite MUST preserve each identity in its declared role and MAY emit a passing identity receipt.

#### Scenario: Stale adapter or resource profile is supplied

r[kamacite.application_host_binding.cohorts.stale]
- GIVEN the application or host profile requires a different adapter cohort or resource profile
- WHEN binding validation runs
- THEN validation MUST fail with a deterministic cohort-mismatch diagnostic.

### Requirement: Binding identity does not grant host authority

r[kamacite.application_host_binding.boundary] A valid application-host binding MUST NOT grant capability, authorization, credential, filesystem, process, network, clock, random, device, mutation, sandbox, resource-enforcement, runtime-correctness, application-correctness, or release authority.

#### Scenario: Exact binding lacks current host admission

r[kamacite.application_host_binding.boundary.no_authority]
- GIVEN an application, effect summary, host profile, adapter cohort, and resource profile match exactly
- AND current authorization or runtime enforcement evidence is missing or failing
- WHEN execution admission runs
- THEN execution MUST remain denied by the product-owned host.

### Requirement: Binding receipts support bounded evidence projection

r[kamacite.application_host_binding.receipt] Kamacite MUST emit deterministic binding validation receipts that bind the input artifact identities, binding identity, verdict, diagnostics, checked rules, policy identity, and explicit non-claims.

#### Scenario: Passing receipt is projected

r[kamacite.application_host_binding.projection]
- GIVEN a binding passes canonical validation
- WHEN Kamacite emits its receipt and Valence-friendly projection
- THEN application, entrypoint, host, effect, adapter, resource, binding, diagnostic, and non-claim roles MUST remain distinct
- AND the projection MUST NOT claim execution, authorization, semantic correctness, or release eligibility.

### Requirement: Binding validation includes positive and negative fixtures

r[kamacite.application_host_binding.fixtures] Kamacite MUST maintain positive, negative, and property-style fixtures for application-host binding schemas, canonical identity, host selection, effect closure, cohort compatibility, projections, diagnostics, and non-claims.

#### Scenario: Fixture corpus covers accepted and denied bindings

r[kamacite.application_host_binding.fixtures.coverage]
- GIVEN the application-host binding fixture corpus
- WHEN focused validation runs
- THEN valid capability-free and closed-effect bindings MUST pass with stable identities
- AND missing-host, multiple-host, unknown-effect, undeclared-effect, stale-cohort, unsafe-diagnostic, and authority-overclaim fixtures MUST fail with expected issue codes.
