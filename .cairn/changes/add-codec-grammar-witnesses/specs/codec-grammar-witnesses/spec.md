# Codec Grammar Witnesses

## ADDED Requirements

### Requirement: Grammar witnesses use a closed versioned vocabulary

r[kamacite.codec_witness.vocabulary] A grammar witness MUST use a versioned Preserves schema. It MUST bind schema, codec, root rule, named rules, alternatives, fields, variants, collections, scalar constraints, references, exactness, and limit-profile identities. Unsupported rules or versions MUST fail closed.

#### Scenario: A valid witness graph is admitted

r[kamacite.codec_witness.vocabulary.valid]
- GIVEN a witness has one root and a bounded closed rule graph
- WHEN witness admission runs
- THEN every reference MUST resolve exactly once
- AND the admitted witness MUST receive a distinct canonical BLAKE3 identity

#### Scenario: A malformed graph is rejected

r[kamacite.codec_witness.vocabulary.invalid]
- GIVEN a witness has a missing rule, duplicate rule, invalid reference, unsupported cycle, or excessive graph
- WHEN witness admission runs
- THEN it MUST return a stable witness diagnostic
- AND it MUST NOT emit an admitted identity

### Requirement: Codec and witness share one source

r[kamacite.codec_witness.source_of_truth] One checked schema definition or generated descriptor MUST produce the paired codec plan and grammar witness. A generation receipt MUST bind the source, schema, codec, witness, generator, and limit-profile identities. A handwritten reminder MUST NOT satisfy this requirement.

#### Scenario: One generated output drifts

r[kamacite.codec_witness.source_of_truth.drift]
- GIVEN the codec or witness differs from the paired generation receipt
- WHEN freshness validation runs
- THEN validation MUST fail before an adapter can claim grammar-backed conformance

### Requirement: Witness exactness is explicit

r[kamacite.codec_witness.exactness] Every witness MUST declare `exact` or `conservative` structural coverage under one named profile. An exact witness MUST name the paired decoder claim. A conservative witness MUST list omitted custom predicates or semantic conditions and MUST NOT authorize admission.

#### Scenario: An exact witness matches the bounded corpus

r[kamacite.codec_witness.exactness.exact]
- GIVEN an exact witness and its paired codec
- WHEN the bounded positive and negative differential corpus runs
- THEN witness classification and codec admission MUST agree for every corpus value

#### Scenario: A custom predicate is not represented

r[kamacite.codec_witness.exactness.conservative]
- GIVEN a decoder applies a custom policy or semantic predicate outside the grammar vocabulary
- WHEN its witness is produced
- THEN the witness MUST be conservative
- AND it MUST identify the omitted condition

### Requirement: Adapter descriptors bind grammar witnesses

r[kamacite.codec_witness.adapter_binding] An adapter descriptor that claims grammar-backed behavior MUST bind the witness, codec, schema, generator, and limit-profile identities. A missing, stale, unsupported, or cross-schema witness MUST fail closed for that claim.

#### Scenario: An existing adapter makes no grammar claim

r[kamacite.codec_witness.adapter_binding.compatibility]
- GIVEN an existing adapter descriptor has no grammar-backed claim
- WHEN the new validator admits it under its original version
- THEN its prior meaning MUST remain unchanged
- AND no witness-backed capability MUST be inferred

### Requirement: Derived witness uses remain non-authoritative

r[kamacite.codec_witness.derived_uses] Admitted witnesses MAY drive documentation, completion, samples, negative fixtures, and fuzz seeds. Every generated value MUST still pass the paired codec. Generated text or values MUST NOT replace schema, contract, policy, or canonical admission authority.

#### Scenario: A generated sample reaches a consumer

r[kamacite.codec_witness.derived_uses.admission]
- GIVEN a sample was produced from an admitted witness
- WHEN an adapter uses the sample as input
- THEN the paired codec MUST admit the sample before trusted use

### Requirement: Witness evidence covers drift and malformed values

r[kamacite.codec_witness.fixtures] Kamacite MUST include positive, negative, property, differential, and golden receipt fixtures for witness admission, paired generation, exactness, conservative omissions, adapter binding, bounds, and derived uses.

#### Scenario: The focused witness matrix runs

r[kamacite.codec_witness.fixtures.matrix]
- GIVEN the witness schemas, component, generators, and adapters are complete
- WHEN the focused validation matrix runs
- THEN every valid fixture MUST produce the expected deterministic result
- AND every stale, detached, malformed, excessive, or overclaiming fixture MUST fail as declared
