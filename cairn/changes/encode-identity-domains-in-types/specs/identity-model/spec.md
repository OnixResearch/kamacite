# Identity Model Specification Delta

## ADDED Requirements

### Requirement: Maintained identity domains are nominal Rust types

r[kamacite.lossless_ir.nominal_identity] Kamacite MUST represent each statically known maintained identity domain with a distinct Rust type over private validated BLAKE3 content.

#### Scenario: Same-domain identity is accepted

r[kamacite.lossless_ir.nominal_identity.same_domain]
- GIVEN a canonical-value identity has valid BLAKE3 content
- WHEN a core API requires a canonical-value identity
- THEN the API MUST accept it without another runtime expected-domain argument.

#### Scenario: Cross-domain identity does not compile

r[kamacite.lossless_ir.nominal_identity_compile_time]
- GIVEN source-envelope and receipt identities use distinct marker-domain instantiations
- WHEN source passes a receipt identity to an API that requires a source-envelope identity
- THEN the source MUST fail compilation with a type mismatch.

#### Scenario: Malformed identity fails before core use

r[kamacite.lossless_ir.nominal_identity.validation]
- GIVEN untrusted identity text has an unsupported algorithm, wrong length, unsupported spelling, or non-hex content
- WHEN an adapter constructs a typed identity
- THEN construction MUST fail with deterministic diagnostics
- AND the invalid identity MUST NOT enter the typed core.

### Requirement: Wire identity domains remain explicit

r[kamacite.lossless_ir.nominal_identity_wire] Kamacite MUST preserve domain and algorithm fields in versioned interchange records and MUST admit them into a closed enum of domain-specific typed identities.

#### Scenario: Wire identity admits a known domain

r[kamacite.lossless_ir.nominal_identity_wire.known]
- GIVEN a supported wire domain, algorithm, and digest value
- WHEN identity admission runs
- THEN Kamacite MUST construct the matching typed identity variant.

#### Scenario: Wire identity has conflicting domain data

r[kamacite.lossless_ir.nominal_identity_wire.validation]
- GIVEN a wire identity has an unknown domain, unsupported algorithm, stale version, or conflicting artifact-role metadata
- WHEN identity admission runs
- THEN Kamacite MUST fail closed with a deterministic domain diagnostic.

### Requirement: Identity migration preserves canonical Preserves authority

r[kamacite.lossless_ir.nominal_identity_migration] Kamacite MUST migrate identity families through explicit adapters without changing canonical Preserves bytes or reinterpreting legacy identities.

#### Scenario: Canonical Preserves identity remains stable

r[kamacite.lossless_ir.nominal_identity.compatibility]
- GIVEN a supported artifact is encoded before and after the Rust nominal-type migration
- WHEN canonical Preserves bytes and BLAKE3 refs are compared
- THEN they MUST remain unchanged unless a separate versioned migration approves the new identity.

#### Scenario: Runtime domain checks move to admission

r[kamacite.lossless_ir.nominal_identity_migration.admission]
- GIVEN a heterogeneous manifest contains untrusted identity records
- WHEN the adapter admits each record
- THEN runtime domain checking MUST occur at admission
- AND known-role core APIs MUST receive exact typed identities.

### Requirement: Stack projections preserve typed identity roles

r[kamacite.lossless_ir.nominal_identity_stack] Kamacite Valence-friendly projections and stack adapters MUST preserve source, value, projection, target, receipt, effect, and compatibility identity roles without promoting evidence.

#### Scenario: Valence projection preserves roles

r[kamacite.lossless_ir.nominal_identity_stack.valence]
- GIVEN a Kamacite receipt contains several domain-specific identities
- WHEN Kamacite emits a Valence-friendly projection
- THEN each identity MUST retain its declared domain and evidence role
- AND the projection MUST NOT treat equal digest bytes as equal roles.

### Requirement: Nominal identity claims remain bounded

r[kamacite.lossless_ir.nominal_identity_docs] Kamacite documentation and receipts MUST state that nominal identity types prove local domain separation and checked identity syntax only.

#### Scenario: Boundary remains visible

r[kamacite.lossless_ir.nominal_identity_final_checks]
- GIVEN a typed identity passes construction and domain admission
- WHEN Kamacite states the supported claim
- THEN it MUST NOT claim source fidelity, adapter law satisfaction, semantic equivalence, authority, runtime correctness, or release eligibility.
