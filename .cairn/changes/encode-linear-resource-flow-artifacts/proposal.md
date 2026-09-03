# Proposal: Encode linear resource-flow artifacts

## Why

Kamacite already owns canonical effect rows, summaries, operation identities, handler descriptors, and receipts.

The stack lacks canonical artifacts for linear resource occurrences, resource rows, row substitutions, resource transitions, and graph-admission results.

Without one interchange contract, Trellis proof evidence and Choregraph graph admission can use incompatible identity or field meanings.

## What Changes

- Add a versioned Preserves profile for linear resource-flow artifacts.
- Define canonical resource kind, occurrence, row, substitution, transition, and admission-receipt shapes.
- Derive domain-separated BLAKE3 identities from canonical packed Preserves bytes.
- Bind Trellis proof references and Choregraph graph identities without importing their authority.
- Reject live handles, credentials, host endpoints, ambient capabilities, and runtime observations in semantic descriptors.
- Add positive round-trip fixtures and negative malformed, stale, duplicate, mismatched, and authority-confusion fixtures.
- Preserve existing algebraic-effect and machine artifacts without reinterpretation.

## Outcome and Durable Capability

The immediate outcome is one canonical artifact profile for the Choregraph resource-flow consumer.

The durable capability is deterministic interchange and linkage for linear resource-flow evidence.

Kamacite maintains artifact schemas, identities, validators, fixtures, and receipts. It does not maintain graph semantics or runtime behavior.

## Impact

The change affects `kamacite-core`, schemas, fixtures, manifests, documentation, validation commands, and Valence-friendly receipt export.

No evaluator, compiler, graph admission engine, or host adapter gains authority.

## Dependencies

Trellis change `add-linear-resource-flow-primitives` defines the independent verified semantic profile.

Choregraph change `admit-linear-resource-flow` is the first artifact consumer.

The stack coordinator is `adopt-linear-resource-flow-contracts`.

## Non-goals

- Do not implement row unification, graph admission, effect execution, authorization, or runtime recovery.
- Do not encode live file, process, network, credential, key, or token handles.
- Do not reinterpret existing algebraic-effect rows as linear resource rows.
- Do not infer resource independence, commutativity, duplication, disposal, or effect success.
- Do not claim Trellis proof validity or Choregraph projection correctness from receipt linkage.
- Do not add a Mirth adapter, evaluator, compiler, or runtime.
