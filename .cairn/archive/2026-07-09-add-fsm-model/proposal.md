## Why

Kamacite already models artifact classes, validation verdicts, adapter execution modes, effect summaries, and golden-update outcomes, but those stateful behaviors are not first-class finite-state-machine artifacts. Without a reviewed FSM model, implementations can accidentally encode hidden control flow in Rust branches, CLI modes, fixture conventions, or downstream runtime receipts without an explicit transition table, totality policy, or trace receipt.

First-class FSM modeling gives Kamacite a bounded way to describe deterministic lifecycle, adapter, validator, and runtime-observation machines while keeping the same lossless-IR boundary: the machine description is canonical data, traces are receipts, and execution remains outside the pure core unless a separate adapter/runtime capability admits it.

## What Changes

- Define Kamacite-owned FSM artifact classes for machine descriptors, states, events, transition relations, coverage matrices, traces, and completeness receipts.
- Require every trusted FSM descriptor to declare finite state/event sets, initial state, terminal/error states, determinism policy, totality policy, transition rows, checked laws, dependencies, and non-claims.
- Define FSM completeness as explicit coverage over declared finite state/event domains: every cell is transitioned, denied, unreachable, or otherwise justified by reviewed metadata rather than inferred by default.
- Require pure validation of reachability, terminal-state behavior, missing cells, conflicting transitions, stale identities, unknown endpoints, host-authority confusion, and unsupported variants.
- Require positive, negative, property-style, and golden receipt fixtures before any FSM claim is trusted.
- Document stack roles so Trellis proofs, Valence evidence, Aspen/Molten runtime traces, Octet source-shape checks, Cairn lifecycle machines, and Mantle release evidence can reference Kamacite FSM artifacts without owning their canonical schema.

## Impact

- **Files**: `cairn/changes/add-fsm-model/**` now; later implementation will touch Preserves schemas, Rust core validators, fixture manifests, golden receipts, docs, and accepted specs.
- **Testing**: Cairn validation plus proposal/design/tasks gates now; later implementation must add Rust unit tests, positive and negative fixture receipts, manifest projection checks, default harness validation, and Cairn validation before FSM claims are accepted.
