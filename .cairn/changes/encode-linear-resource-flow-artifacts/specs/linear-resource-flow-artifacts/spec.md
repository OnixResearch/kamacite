# Linear Resource Flow Artifacts Delta

## ADDED Requirements

### Requirement: Kamacite defines an additive resource-flow profile

r[kamacite.linear_resource_flow.profile] Kamacite MUST define a versioned profile for linear resource-flow artifacts without changing existing effect or machine artifact meaning.

#### Scenario: A consumer selects the new profile

- GIVEN a consumer supplies a supported linear resource-flow profile identity
- WHEN Kamacite validates the bundle
- THEN Kamacite MUST apply only that profile schema and identity contract

#### Scenario: A consumer presents an old effect artifact

- GIVEN an existing algebraic-effect artifact has no linear resource-flow profile
- WHEN a consumer presents it as a resource-flow artifact
- THEN Kamacite MUST reject the reinterpretation

### Requirement: Resource-flow artifact classes have separate schemas

r[kamacite.linear_resource_flow.schema] Kamacite MUST define separate schemas for resource kinds, occurrences, rows, substitutions, transitions, witnesses, and admission receipts.

#### Scenario: A complete bundle uses exact class identities

- GIVEN every artifact uses its declared schema and profile
- WHEN bundle validation runs
- THEN each cross-artifact reference MUST match the exact target identity

#### Scenario: One artifact uses the wrong class

- GIVEN a transition field references a row identity where a witness identity is required
- WHEN bundle validation runs
- THEN validation MUST reject the bundle

### Requirement: Canonical bytes own artifact identity

r[kamacite.linear_resource_flow.identity] Every resource-flow identity MUST use domain-separated BLAKE3 over canonical packed Preserves bytes.

#### Scenario: Equivalent supported input decodes to one canonical value

- GIVEN supported text or packed input decodes to the same typed artifact
- WHEN Kamacite computes identity
- THEN the identity MUST derive from the same canonical packed bytes

#### Scenario: Supplied bytes disagree with the claimed identity

- GIVEN an artifact carries an identity for different canonical bytes
- WHEN identity validation runs
- THEN validation MUST reject the artifact

### Requirement: Runtime rows are closed and occurrence-safe

r[kamacite.linear_resource_flow.validation] A runtime resource row MUST contain only concrete unique occurrence references and exact kind and state identities.

#### Scenario: A closed row contains unique occurrences

- GIVEN every row entry has concrete kind, state, and occurrence identities
- AND every occurrence identity is unique
- WHEN closed-row validation runs
- THEN validation MUST accept the row shape

#### Scenario: A closed row repeats an occurrence

- GIVEN one occurrence identity appears more than once
- WHEN closed-row validation runs
- THEN validation MUST reject the row

#### Scenario: A runtime row contains an unresolved variable

- GIVEN a runtime row contains a tail-variable reference
- WHEN closed-row validation runs
- THEN validation MUST reject the row

### Requirement: Open rows and substitutions remain explicit

r[kamacite.linear_resource_flow.validation] An open row expression and its substitution MUST have separate identities and explicit variable bindings.

#### Scenario: A substitution closes an open expression

- GIVEN an open expression references a declared variable
- AND a substitution binds that variable to a finite row expression
- WHEN linkage validation runs
- THEN validation MUST accept the shape and exact identities

#### Scenario: A substitution references an undeclared variable

- GIVEN a substitution binds a variable absent from the source expression
- WHEN linkage validation runs
- THEN validation MUST reject the substitution bundle

### Requirement: Non-linear transitions bind exact witnesses

r[kamacite.linear_resource_flow.validation] Duplication and disposal transitions MUST reference an exact witness for the resource kind and operation identity.

#### Scenario: A duplication transition has an exact witness

- GIVEN the witness matches the transition resource kind and operation identity
- WHEN witness linkage validation runs
- THEN validation MUST accept the linkage shape

#### Scenario: A witness has the wrong kind or operation

- GIVEN a witness identity resolves to different kind or operation facts
- WHEN witness linkage validation runs
- THEN validation MUST reject the transition

### Requirement: Semantic artifacts exclude host authority

r[kamacite.linear_resource_flow.boundary] Resource-flow descriptors MUST NOT contain live handles, credentials, keys, tokens, host endpoints, process identifiers, or authorization decisions.

#### Scenario: A descriptor contains only symbolic identities

- GIVEN a descriptor contains canonical symbolic resource and operation identities
- WHEN boundary validation runs
- THEN validation MUST accept the authority-free shape

#### Scenario: A descriptor contains a credential or live handle

- GIVEN a descriptor includes an authority-bearing or runtime-object field
- WHEN boundary validation runs
- THEN validation MUST reject the descriptor

### Requirement: Admission receipts bind producer and consumer evidence

r[kamacite.linear_resource_flow.receipt] A resource-flow admission receipt MUST bind exact Trellis, Choregraph, schema, artifact, decision, diagnostic, and non-claim identities.

#### Scenario: An accepted receipt has complete linkage

- GIVEN the receipt references one complete compatible artifact bundle
- AND it binds exact producer and consumer profile identities
- WHEN receipt validation runs
- THEN validation MUST accept the receipt structure

#### Scenario: A receipt has stale proof or graph linkage

- GIVEN a receipt references a different Trellis profile or Choregraph graph identity
- WHEN receipt validation runs
- THEN validation MUST reject the receipt

#### Scenario: A receipt claims effect execution

- GIVEN a receipt contains only resource-flow admission evidence
- WHEN it claims authorization or external effect success
- THEN validation MUST reject the claim shape

### Requirement: Later versions use explicit migration

r[kamacite.linear_resource_flow.migration] A later resource-flow profile MUST use an adjacent migration plan. Migration MUST remain dry-run by default.

#### Scenario: A reviewed adjacent migration exists

- GIVEN source and target profile identities are adjacent and supported
- WHEN migration preview runs
- THEN it MUST report old identity, new identity, changed fields, and non-claims

#### Scenario: A caller requests implicit effect-row conversion

- GIVEN an old effect row lacks resource occurrence semantics
- WHEN a caller requests automatic linear-resource conversion
- THEN migration MUST reject the request

### Requirement: Fixtures cover acceptance and rejection

r[kamacite.linear_resource_flow.fixtures] The profile MUST include positive round-trip fixtures and negative malformed, stale, duplicate, mismatch, and authority-confusion fixtures.

#### Scenario: The fixture cohort is complete

- GIVEN every required positive and negative class is present
- WHEN the fixture harness runs
- THEN valid fixtures MUST pass
- AND invalid fixtures MUST fail with stable diagnostic identities

#### Scenario: Negative fixtures are missing

- GIVEN the profile lacks duplicate, stale, unresolved, wrong-witness, or authority-confusion cases
- WHEN readiness review runs
- THEN readiness MUST remain blocked

### Requirement: Consumers bind the exact profile

r[kamacite.linear_resource_flow.consumer] Kamacite MUST publish exact schema, codec, identity-domain, fixture, and receipt identities for Choregraph.

#### Scenario: Choregraph adopts the profile

- GIVEN Kamacite has published an immutable compatible profile
- WHEN Choregraph emits resource-flow artifacts
- THEN Choregraph MUST bind the exact Kamacite revision and profile identities

#### Scenario: Choregraph supplies a partial profile

- GIVEN a consumer omits a required schema, identity domain, fixture cohort, or receipt contract
- WHEN consumer admission runs
- THEN admission MUST reject the partial profile

### Requirement: Evidence preserves bounded claims

r[kamacite.linear_resource_flow.evidence] Kamacite evidence MUST report canonicalization, validation, identity, linkage, fixture, and round-trip facts only.

#### Scenario: All Kamacite validation passes

- GIVEN the profile and fixtures pass local validation
- WHEN evidence is published
- THEN it MUST record exact source, schema, codec, manifest, and validation identities
- AND it MUST retain explicit non-claims

#### Scenario: Evidence promotes linkage into semantic proof

- GIVEN a receipt links Trellis and Choregraph identities
- WHEN evidence claims proof soundness or graph correctness
- THEN evidence review MUST reject the broader claim
