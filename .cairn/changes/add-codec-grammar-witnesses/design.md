## Context

Kamacite owns versioned Preserves schemas and adapter descriptors. The descriptors list supported schemas, effects, laws, dependencies, and non-claims.

No shared artifact describes the runtime value shape accepted by one codec. Documentation and fixture tools must inspect separate sources or use handwritten knowledge.

`ppx_sexp_conv` revision `fd48b5f67a8149d0b3546a4a3b3c852e8da06398` generates converter and grammar functions together. Kamacite adapts this paired-witness mechanism over its own schemas.

## Goals

- Make accepted structural shape reviewable as canonical data.
- Detect codec and witness drift before release.
- Preserve closed-field and explicit-default behavior.
- Support bounded derived tools without transferring authority.

## Non-goals

- A replacement for Nickel contracts.
- A proof of evaluator or adapter correctness.
- A general Rust reflection system.
- An unbounded recursive grammar engine.

## Decisions

### Decision: Add one pure codec-witness component

**Choice:** Add `crates/kamacite-codec-witness/` for witness DTOs, validation, admission, canonical identity, and bounded sample classification.

The component consumes admitted in-memory schema facts. File reads, code generation, documentation rendering, and fixture writes remain in shell tools.

**Rationale:** Witness meaning is one coherent capability. It does not need evaluator or filesystem authority.

### Decision: Use a closed grammar vocabulary

**Choice:** A witness declares its schema identity, codec identity, root rule, named rules, alternatives, collections, fields, variants, scalar constraints, references, and limit profile.

Field rules state required, optional, or explicit-default behavior. Record rules state their unknown-field policy.

References are named and bounded. Validation rejects missing, duplicate, cyclic-without-policy, or excessive rule graphs.

**Rationale:** An open grammar vocabulary can hide decoder behavior. Closed rules make acceptance claims reviewable.

### Decision: Produce codec and witness from one source

**Choice:** One checked schema definition or generated descriptor produces the codec plan and witness artifact.

A hand-maintained reminder is not sufficient. Generation receipts bind source, codec, witness, tool, and limit-profile identities.

**Rationale:** Paired generation prevents ordinary copy drift. Receipts expose generator drift.

### Decision: State witness exactness

**Choice:** Every witness is `exact` or `conservative` for a named structural acceptance claim.

An exact witness claims structural agreement with the paired decoder under one profile. A conservative witness identifies every omitted custom predicate or semantic condition.

A conservative witness cannot authorize input admission.

**Rationale:** Custom validators, evaluation, and policy can narrow accepted values beyond a structural grammar.

### Decision: Bind witnesses in adapter descriptors

**Choice:** An adapter descriptor that claims grammar-backed tooling binds the witness identity, codec identity, schema identity, generator identity, and limit profile.

A missing, stale, unsupported, or cross-schema witness fails closed for that claim. Existing adapters without the new claim retain their prior meaning.

**Rationale:** A witness has value only when a consumer can link it to the exact codec.

### Decision: Keep derived uses non-authoritative

**Choice:** Documentation, completion, sample generation, negative fixture generation, and fuzz seeding can consume admitted witnesses.

Generated values remain untrusted until the paired codec admits them. Generated documentation remains a projection.

**Rationale:** A witness supports tooling. It does not replace canonical codec admission.

### Decision: Validate with bounded differential corpora

**Choice:** Exact witnesses use bounded generated and hand-authored corpora to compare witness classification with codec admission.

Negative fixtures cover unknown fields, missing required fields, implicit defaults, wrong variants, malformed references, excessive depth, and stale identities.

**Rationale:** Corpus agreement is bounded evidence. It is not a universal proof of decoder equivalence.

### Decision: Use the upstream code only as a design reference

**Choice:** The implementation uses Kamacite Rust and Preserves types. It copies no PPX or OCaml source text.

Any copied upstream expression must retain the MIT notice.

**Rationale:** Kamacite does not share the OCaml type or code-generation model.

## Adversarial Audit

The audit must try these failures:

- a witness and codec use different schemas;
- a generator changes only one output;
- a witness omits unknown-field rejection;
- an optional field hides an implicit default;
- a recursive graph exceeds its bound;
- a conservative witness claims exact admission;
- generated documentation becomes schema authority;
- a generated sample bypasses codec admission;
- a receipt binds a stale witness identity.

Any successful counterexample blocks closeout.

## Risks / Trade-offs

- Paired generation adds one reviewed artifact and identity per codec.
- Exactness can be unavailable for custom validators. Conservative classification keeps that limit visible.
- Differential corpora can miss behavior. The receipts must state their bounded claim.
