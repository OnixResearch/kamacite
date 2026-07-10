# Finite State Machine Model Specification

## Purpose

Bind FSM exhaustiveness and trace receipts to the descriptor whose identity they cite.

## Requirements

### Requirement: Receipt validation consumes a validated descriptor

r[kamacite.findings.fsm_receipts.descriptor_binding] Kamacite MUST validate an FSM receipt against a schema-valid descriptor value whose canonical identity equals the receipt's descriptor identity. A receipt-supplied hash without the corresponding validated descriptor MUST NOT establish consistency.

#### Scenario: Descriptor substitution is attempted

- GIVEN a receipt cites one spec identity while validation receives a different descriptor value
- WHEN receipt validation runs
- THEN validation MUST fail with deterministic descriptor-identity mismatch evidence.

### Requirement: Exhaustiveness identities are recomputed

r[kamacite.findings.fsm_receipts.coverage_recomputation] Exhaustiveness receipt validation MUST recompute state universe, event universe, transition coverage, denial coverage, exclusions, and missing cells from the bound descriptor and compare every declared coverage identity.

#### Scenario: Arbitrary coverage hashes are supplied

- GIVEN an exhaustiveness receipt uses well-formed coverage-domain hashes that were not derived from the descriptor matrix
- WHEN Kamacite validates the receipt
- THEN validation MUST reject the stale coverage roots even if the receipt's own byte hash matches.

### Requirement: Trace steps are enabled by the descriptor

r[kamacite.findings.fsm_receipts.trace_consistency] Trace receipt validation MUST verify the admitted start policy, declared events, transition identities, source and destination states, guard-decision evidence when required, and final state against enabled transitions in the bound descriptor.

#### Scenario: Internally continuous invented transition

- GIVEN a trace step continues from the previous state but no matching descriptor transition exists for its event and destination
- WHEN trace validation runs
- THEN validation MUST fail with an impossible-transition diagnostic.

### Requirement: FSM receipt fixtures cover descriptor disagreement

r[kamacite.findings.fsm_receipts.negative_coverage] The FSM receipt suite MUST include positive descriptor-consistent exhaustiveness/trace cases and negative wrong-initial-state, undeclared-event, invented-transition, guard-evidence, missing-cell, stale-root, and descriptor-substitution cases.

#### Scenario: Wrong initial state is supplied

- GIVEN a trace begins at a declared state that is not the descriptor initial state and no alternate-start profile is admitted
- WHEN validation runs
- THEN the trace MUST be rejected before final-state consistency is considered.
