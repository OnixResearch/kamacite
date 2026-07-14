# FSM spec and receipt fixtures

Kamacite now has a bounded FSM fixture slice for reviewed finite-state-machine specifications, exhaustiveness receipts, and trace receipts.

The slice is data-only. `kamacite-core` validates parsed in-memory DTOs and canonical bytes; it does not execute a state machine, grant transition authority, model liveness, prove runtime correctness, or replace Octet/Valence/Cairn/Mantle roles.

Validated fixture coverage includes:

- a positive complete deterministic state/event table with explicit denial coverage;
- validation-command and adapter-execution-mode FSM fixtures that make command states, events, denials, and capability gates explicit without claiming CLI branch coverage or host authority;
- negative missing coverage, duplicate/conflicting transition, wildcard row, undeclared guard, unknown endpoint, stale identity, wrong domain, missing non-claim, host-authority confusion, terminal outgoing transition, and unreachable-unmarked state cases;
- descriptor-bound exhaustiveness receipts that recompute state/event universes, transition/denial roots, exclusions, and missing cells before accepting receipt identity;
- positive and negative trace receipts whose initial state, declared events, exact transition identities, guard-decision evidence, and final state are checked against the validated descriptor;
- stack-role fixtures for Trellis, Valence, Aspen/Molten, Octet, Cairn, and Mantle evidence consumers.

Run the slice through the normal harness:

```bash
nix run .#kamacite -- validate --root . --manifest fixtures/manifest.json
```

Schema artifacts live under `schemas/preserves/fsm-spec-v1.preserves`, the legacy exhaustiveness/trace schema references, and the descriptor-bound `schemas/preserves/fsm-receipt-v3.preserves`. Fixture canonical bytes live under `fixtures/core/fsm/`, deterministic golden receipts live under `fixtures/golden/`, and `fixtures/fsm/migration-v2-descriptor-bound.json` records old/new receipt and Valence projection identities.
