## Why

Octet can make Rust FSM transition cores exhaustive by default, and Valence can preserve the resulting evidence identity. Kamacite should own the portable interchange vocabulary for FSM specifications and receipts so state/event universes, transition rows, invalid-transition denials, guard vocabularies, and receipt identities can be authored and exchanged as canonical data rather than inferred from one Rust implementation.

This keeps the stack boundaries clean: Kamacite defines source-preserving Preserves schemas and validation receipts, Octet checks Rust projections and source shape, and Valence records linked evidence identities and non-claims.

## What Changes

- Add a Kamacite FSM spec vocabulary for closed state universes, closed event universes, guard vocabularies, transition rows, invalid-transition denial rows, side-effect classes, and non-claims.
- Add an FSM exhaustiveness receipt vocabulary that binds spec identity, coverage identity, denial coverage identity, adapter identity, diagnostics, checked laws, dependency identities, and BLAKE3 receipt identity.
- Add positive and negative fixtures for complete FSM specs, missing state/event rows, duplicate transitions, wildcard/default transitions, undeclared guards, stale identities, and overclaims.
- Add Valence export compatibility for FSM receipts without making Valence a Kamacite runtime dependency.
- Add Octet interop guidance for Rust projections without treating Octet findings as proof of Kamacite FSM semantic correctness.

## Impact

Kamacite becomes the portable data home for reviewed FSM specifications while preserving its existing non-claims. A passing Kamacite FSM receipt proves only the bounded schema, identity, coverage-record, and non-claim checks named by the receipt. It does not prove arbitrary host-language equivalence, runtime correctness, liveness, verifier soundness, release eligibility, or protocol completeness.
