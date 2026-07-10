## Context

FSM spec validation and receipt validation operate on disconnected inputs. A receipt carries a spec hash and opaque canonical bytes, but not a validated descriptor value that can be used to check coverage or trace transitions.

## Decisions

### 1. Receipt validation consumes a validated descriptor

**Choice:** Introduce a validated FSM descriptor type produced only by canonical decode plus spec validation. Exhaustiveness and trace receipt cores require that value and its identity.

**Rationale:** A hash label alone cannot answer whether a transition exists.

### 2. Coverage roots are recomputed

**Choice:** Canonically derive state universe, event universe, transition coverage, denial coverage, and excluded-cell roots from the descriptor and compare every declared receipt identity.

**Rationale:** Self-asserted coverage identities do not establish completeness.

### 3. Trace consistency uses transition identity

**Choice:** Require the descriptor initial state unless an explicit admitted alternate-start profile exists; resolve every event/step to a unique descriptor transition or denial. Guarded transitions bind explicit guard-decision evidence rather than evaluating host semantics.

**Rationale:** Internal chain continuity is weaker than descriptor consistency.

### 4. Preserve the data-only boundary

**Choice:** The core validates a recorded path and matrix. It does not execute effects or claim runtime behavior.

**Rationale:** Stronger receipt linkage must not turn Kamacite into an FSM runtime.

## Risks / Trade-offs

- Existing FSM receipt schemas lack enough transition/guard identity detail and need a version bump.
- Recomputed roots will change golden receipts.
- Alternate-start and nondeterministic profiles require explicit policy rather than silent acceptance.
