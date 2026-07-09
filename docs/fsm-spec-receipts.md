# FSM spec and receipt fixtures

Kamacite now has a bounded FSM fixture slice for reviewed finite-state-machine specifications, exhaustiveness receipts, and trace receipts.

The slice is data-only. `kamacite-core` validates parsed in-memory DTOs and canonical bytes; it does not execute a state machine, grant transition authority, model liveness, prove runtime correctness, or replace Octet/Valence/Cairn/Mantle roles.

Validated fixture coverage includes:

- a positive complete deterministic state/event table with explicit denial coverage;
- negative missing coverage, duplicate/conflicting transition, wildcard row, undeclared guard, unknown endpoint, stale identity, wrong domain, missing non-claim, host-authority confusion, terminal outgoing transition, and unreachable-unmarked state cases;
- an exhaustiveness receipt that links Octet source-shape evidence and a Valence-friendly projection without broadening claims;
- positive and negative trace receipts for path consistency.

Run the slice through the normal harness:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- validate --root . --manifest fixtures/manifest.json'
```

Schema artifacts live under `schemas/preserves/fsm-spec-v1.preserves`, `schemas/preserves/fsm-exhaustiveness-receipt-v1.preserves`, and `schemas/preserves/fsm-trace-receipt-v1.preserves`. Fixture canonical bytes live under `fixtures/core/fsm/`, while deterministic golden receipts live under `fixtures/golden/`.
