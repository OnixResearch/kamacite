# Typed Core Discipline Specification

## Purpose

Defines the Haskell-inspired principles Kamacite adapts as implementation discipline: pure cores, algebraic artifact modeling, explicit effects, law-based adapters, typed identities, total diagnostics, and clear non-claims about Haskell/GHC artifacts.

## Requirements

### Requirement: Pure typed core boundary

r[kamacite.lossless_ir.typed_core_discipline] Kamacite MUST keep deterministic validation, canonicalization, hash-material construction, artifact classification, and receipt construction in a pure typed core over explicit inputs. Filesystem access, environment reads, process execution, evaluator calls, compiler calls, clocks, randomness, credentials, network access, mutation, and presentation MUST stay in an imperative shell.

#### Scenario: Core validation depends only on explicit inputs

r[kamacite.lossless_ir.typed_core_discipline.explicit_inputs]

- GIVEN a Kamacite core validator receives source-envelope bytes, declared schemas, adapter descriptors, canonical bytes, and receipt fields as in-memory inputs
- WHEN it classifies and validates the artifact
- THEN the validation result MUST be determined only by those explicit inputs and MUST NOT depend on filesystem, environment, process, evaluator, compiler, clock, random, credential, network, or mutable host state.

#### Scenario: Artifact classes are modeled as closed alternatives

r[kamacite.lossless_ir.typed_core_discipline.artifact_alternatives]

- GIVEN the core model represents Kamacite artifact claims
- WHEN a validator matches on the artifact class
- THEN supported classes MUST be explicit alternatives such as source envelope, canonical value, constrained policy, target artifact, and unsupported claim, and unknown classes MUST fail closed instead of defaulting to a supported class.

### Requirement: Distinct identity domains

r[kamacite.lossless_ir.typed_identity_domains] Kamacite MUST keep identity domains distinct in schemas, Rust DTOs, fixtures, and receipts. Source identity, canonical value identity, projection identity, target-artifact identity, adapter identity, evaluator identity, compiler identity, and receipt identity MUST NOT be interchangeable unless an explicit projection or derivation receipt links them.

#### Scenario: Projection hash cannot replace canonical hash

r[kamacite.lossless_ir.typed_identity_domains.projection_not_canonical]

- GIVEN a JSON projection hash is supplied where a Preserves canonical value hash is required
- WHEN Kamacite validates the artifact or receipt
- THEN validation MUST reject the substitution with a deterministic identity-domain diagnostic.

#### Scenario: Source hash cannot replace receipt hash

r[kamacite.lossless_ir.typed_identity_domains.source_not_receipt]

- GIVEN a source-envelope BLAKE3 hash is supplied where a receipt identity is required
- WHEN Kamacite validates the receipt
- THEN validation MUST reject the substitution unless a receipt explicitly derives and binds the source hash as an input field.

### Requirement: Lawful adapter interfaces

r[kamacite.lossless_ir.adapter_laws] Kamacite adapters and projections MUST declare the laws they are expected to satisfy before their results are trusted. Required laws SHOULD include source-envelope preservation, Preserves decode/encode stability, JSON projection derivation, names-as-metadata identity, dependency closure binding, explicit effect declaration, and deterministic failure for unsupported claims.

#### Scenario: Adapter receipt names checked laws

r[kamacite.lossless_ir.adapter_laws.receipt_laws]

- GIVEN an adapter emits a Kamacite receipt after converting, projecting, evaluating, compiling, or validating an artifact
- WHEN the receipt is accepted
- THEN it MUST name the adapter laws checked, the fixtures or property checks used as evidence, and the explicit laws that were not claimed.

#### Scenario: Law violation fails closed

r[kamacite.lossless_ir.adapter_laws.violation]

- GIVEN an adapter round trip changes canonical Preserves identity, drops source-envelope bytes, treats names as identity, omits dependency hashes, or observes undeclared effects
- WHEN Kamacite validates the adapter receipt
- THEN validation MUST reject the receipt with a deterministic law-violation diagnostic.

### Requirement: Total diagnostic behavior

r[kamacite.lossless_ir.total_diagnostics] Kamacite validators and adapter receipt checks MUST return explicit deterministic diagnostics for malformed, unsupported, stale, lossy, undeclared, or ambiguous inputs. Kamacite MUST NOT panic, silently default, truncate, coerce across identity domains, or accept partial success for rejected artifacts.

#### Scenario: Malformed artifact returns diagnostic

r[kamacite.lossless_ir.total_diagnostics.malformed]

- GIVEN a malformed Kamacite artifact is presented to validation
- WHEN decoding, classification, or schema validation fails
- THEN validation MUST return a deterministic diagnostic that names the failing field, schema, artifact class, or claim boundary without accepting a partial artifact.

#### Scenario: Unsupported variant fails closed

r[kamacite.lossless_ir.total_diagnostics.unsupported_variant]

- GIVEN a validly encoded artifact names an unsupported Kamacite class, codec, schema version, adapter capability, or target kind
- WHEN Kamacite validates the artifact
- THEN validation MUST fail closed with an unsupported-variant diagnostic rather than defaulting to the nearest supported behavior.

### Requirement: Haskell/GHC non-claim boundary

r[kamacite.lossless_ir.haskell_boundary] Kamacite MAY adapt Haskell design principles such as algebraic data modeling, parametricity, lawful interfaces, pure/effect separation, property-testing discipline, total error returns, and newtype-style identity separation. Kamacite MUST NOT treat Haskell source, GHC Core, STG, laziness, typeclass resolution, monad terminology, or GHC build artifacts as Kamacite canonical IR, receipt authority, or proof of Nickel, Rust, or Steel semantic equivalence.

#### Scenario: Haskell source is preserved as source only

r[kamacite.lossless_ir.haskell_boundary.source_envelope]

- GIVEN a Haskell source file is included in a Kamacite fixture or adapter reference
- WHEN Kamacite records the artifact
- THEN it MAY preserve the original source bytes as a source envelope with explicit non-claims, but MUST NOT treat the source as Kamacite canonical IR.

#### Scenario: GHC IR is rejected as canonical Kamacite IR

r[kamacite.lossless_ir.haskell_boundary.ghc_ir_not_kamacite_ir]

- GIVEN a tool claims GHC Core, STG, bytecode, interface files, or generated Haskell artifacts are the lossless representation of Nickel, Rust, Steel, or Kamacite artifacts
- WHEN Kamacite classifies the claim
- THEN validation MUST reject the canonical-IR claim and MAY accept those artifacts only as bounded target evidence or source envelopes with explicit non-claims.
