# Algebraic-effect stack roles

Kamacite owns algebraic-effect artifact schemas and receipt contracts. The current implementation validates algebraic-effect data as canonical artifacts, not as a Kamacite evaluator, programming language, runtime, Rust extension, host-authority grant, or semantic-equivalence proof.

<!-- r[impl kamacite.algebraic_effects.semantic_stack_consumption] -->
<!-- r[impl kamacite.algebraic_effects.semantic_supporting_evidence] -->

Owned Kamacite artifacts:

- effect signatures;
- effect rows and summaries;
- handler descriptors;
- handler-application receipts;
- continuation policies;
- replay profiles;
- effect-log projections.

Host capabilities remain separate from algebraic operations. Declaring an operation such as `ReadConfig`, `CurrentTime`, or `Random` never grants filesystem, clock, randomness, network, process, credential, package-manager, dynamic-library, or mutation authority. Host-backed execution claims need separate adapter/capability admission evidence.

Stack boundaries:

- Trellis may provide proof/reference artifacts for finite signatures, rows, handler discharge, sandbox attenuation, or single-use continuation facts. Those artifacts support review but do not replace Kamacite schemas, identities, receipts, or local validation.
- Valence may wrap Kamacite algebraic-effect receipts as stable evidence identities with roles, spans, hashes, diagnostics, and non-claims. Valence export must not broaden the claim to runtime correctness or host authorization.
- Aspen/Molten may execute handler profiles and consume effect-log projections. Runtime evidence must reference Kamacite handler descriptor and effect summary identities rather than redefining canonical effect artifacts.
- Octet may check Kamacite Rust validators, adapters, fixture runners, policy metadata, and provenance. Octet findings remain Rust source-shape, policy, or provenance evidence only.

Version 2 surfaces carry typed `effect-operation` identities through rows, summaries, handler descriptors, receipts, logs, adapters, target imports, and Wasm imports. `validate_semantic_operation_reference_bundle` rejects stale or undeclared identities before a consumer uses the artifact.

The fixture corpus contains positive declared-discharge, recorded-replay, and single-use continuation cases plus negative fixtures for unsupported versions, missing operation schemas, undeclared operations, open-row purity inference, handler-name discharge claims, unknown effects, continuation mismatches, host-capability confusion, stale identities, and non-replayable evidence misuse.
