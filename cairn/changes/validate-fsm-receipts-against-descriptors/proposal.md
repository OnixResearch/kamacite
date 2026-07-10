## Why

FSM exhaustiveness receipts currently receive no matrix validation, and trace receipts are checked only for internal step continuity. Neither path decodes or receives the bound FSM descriptor to prove that the start state is initial, events are declared, transitions are enabled, or coverage identities describe the descriptor. A self-consistent invented path or arbitrary coverage hashes can therefore be accepted as descriptor-bound evidence.

## What Changes

- Validate FSM receipts against a validated descriptor value decoded from the bound canonical artifact.
- Recompute state/event universes, transition coverage, denial coverage, and receipt identity from descriptor and receipt data.
- Require trace start state, events, transition identities, guards/evidence, and final state to correspond to enabled descriptor transitions.
- Add negative fixtures for invented transitions, wrong initial state, undeclared events, stale coverage roots, missing matrix cells, and descriptor substitution.

## Impact

- **Surfaces**: FSM spec/receipt DTOs, canonical schemas, fixture materialization, golden receipts, and Valence export.
- **Non-claim**: consistency checking remains data validation and does not execute a runtime FSM.
- **Validation**: pure descriptor/receipt tests, positive and negative FSM fixtures, full harness validation, and Cairn gates.
