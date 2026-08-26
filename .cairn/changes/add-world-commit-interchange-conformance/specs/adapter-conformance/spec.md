# Adapter Conformance Specification Delta

## Purpose

Add independent Nickel, Rust, and Steel value and byte conformance for one immutable Molten world-commit schema cohort.

## ADDED Requirements

### Requirement: World-commit conformance pins one exact product cohort

r[kamacite.world_commit_conformance.cohort] Kamacite MUST bind the Molten repository, immutable revision, schema identity, codec identity, canonical domain tag, supported version, fixture manifest, license, and non-claims. A sibling path, mutable branch, or implicit latest revision MUST NOT satisfy conformance.

#### Scenario: Immutable cohort is available

- GIVEN every pinned source and fixture identity matches
- WHEN world-commit conformance admission runs
- THEN Kamacite MAY evaluate the cohort

#### Scenario: Molten source revision drifts

- GIVEN fixture bytes come from a revision other than the conformance entry
- WHEN admission runs
- THEN Kamacite MUST reject the corpus before adapter comparison

### Requirement: The conformance entry preserves Molten ownership

r[kamacite.world_commit_conformance.entry] The entry MUST classify Molten as owner of world-commit schema and semantics. Kamacite MUST own only source envelopes, admitted canonical values, adapter descriptors, checked laws, parity receipts, and conformance diagnostics.

#### Scenario: Entry claims branch authority

- GIVEN an adapter entry claims that byte parity authorizes a branch transition
- WHEN entry validation runs
- THEN Kamacite MUST reject the entry as a runtime-authority overclaim

### Requirement: Typed root domains remain distinct

r[kamacite.world_commit_conformance.domains] Kamacite MUST validate the pinned world-commit root-domain inventory and MUST reject substitution between artifact, schema, durable-state, task, history, effect, scheduler, time, entropy, runtime-profile, policy, authority-observation, and opaque-snapshot references.

#### Scenario: Artifact digest occupies the task-root field

- GIVEN the digest bytes are valid but their nominal domain is artifact
- WHEN world-commit admission runs
- THEN Kamacite MUST reject the value as domain substitution

### Requirement: Canonical Preserves bytes are the parity boundary

r[kamacite.world_commit_conformance.parity] The authoritative Rust fixture, admitted Nickel projection, and fresh Steel adapter MUST produce or resolve to the same canonical Kamacite IR value and exact packed Preserves bytes for a passing vector. JSON and source text MUST remain noncanonical projections or envelopes.

#### Scenario: Values match but packed bytes are noncanonical

- GIVEN an adapter decodes to the expected value but emits a noncanonical packed representation
- WHEN parity comparison runs
- THEN Kamacite MUST reject exact-byte conformance

### Requirement: Rust fixtures retain product provenance

r[kamacite.world_commit_conformance.rust] Rust vectors MUST bind Molten DTO source, schema, codec, domain tag, source revision, canonical bytes, and fixture identity. Kamacite MUST NOT regenerate missing authoritative bytes from its own implementation during normal validation.

#### Scenario: Authoritative bytes are absent

- GIVEN only a Kamacite-generated expected value exists
- WHEN Rust parity runs
- THEN the vector MUST remain incomplete

### Requirement: Nickel remains a review projection

r[kamacite.world_commit_conformance.nickel] Nickel fixtures MUST preserve source envelopes and evaluator identity separately from exports. Nickel contracts MAY validate product DTO shape, but JSON export MUST NOT replace canonical Preserves identity.

#### Scenario: Nickel JSON is labeled canonical

- GIVEN a fixture identifies exported JSON bytes as the world-commit identity
- WHEN Nickel conformance runs
- THEN Kamacite MUST reject the fixture

### Requirement: Steel runs without ambient authority

r[kamacite.world_commit_conformance.steel] The Steel adapter MUST run fresh in a declared no-kernel engine with bounded inputs, pure registered host functions, declared modules and contracts, and no ambient filesystem, process, network, clock, random, credential, package, dynamic-library, or mutation authority.

#### Scenario: Steel adapter requests file access

- GIVEN the adapter requests an undeclared filesystem capability
- WHEN fresh conformance execution runs
- THEN Kamacite MUST fail closed before accepting output

### Requirement: Source identities remain separate from value identity

r[kamacite.world_commit_conformance.sources] Rust, Nickel, and Steel source bytes MUST remain source envelopes with their own BLAKE3 identities. Source identity MUST NOT substitute for canonical value or adapter receipt identity.

#### Scenario: Steel source hash is supplied as commit value identity

- GIVEN a valid Steel source-envelope hash occupies the canonical-value field
- WHEN receipt validation runs
- THEN Kamacite MUST reject the identity-domain mismatch

### Requirement: Cohort updates are explicit and convergent

r[kamacite.world_commit_conformance.update] Updating schema, codec, or supported version MUST create a new pinned cohort, dry-run difference report, explicit fixture regeneration, convergence check, and review of removed or widened negative vectors. Unknown versions MUST fail closed.

#### Scenario: Golden bytes change without cohort update

- GIVEN expected packed bytes change while the pinned schema and codec identities remain unchanged
- WHEN freshness validation runs
- THEN Kamacite MUST reject the update

### Requirement: Receipts preserve bounded conformance claims

r[kamacite.world_commit_conformance.receipt] Receipts MUST bind source cohort, conformance entry, schemas, codec, adapters, evaluator identities, source envelopes, fixtures, checked laws, exact results, and non-claims. They MUST NOT claim runtime behavior, evaluator equivalence, restore compatibility, authority, or release eligibility.

#### Scenario: Valence export widens parity into runtime proof

- GIVEN a Valence projection states that adapter parity proves deterministic execution
- WHEN export validation runs
- THEN Kamacite MUST reject the widened claim

### Requirement: Verification includes positive and hostile vectors

r[kamacite.world_commit_conformance.verification] Kamacite MUST test all supported root and profile classes plus malformed, noncanonical, unknown-version, domain-substitution, duplicate, ordering, bound, stale-schema, embedded-evidence, secret, ambient-authority, and overclaim cases.

#### Scenario: Corpus has only positive vectors

- GIVEN all supported values pass but no hostile vectors exist
- WHEN conformance coverage runs
- THEN the world-commit profile MUST remain incomplete
