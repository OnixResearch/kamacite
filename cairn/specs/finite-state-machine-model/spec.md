# Finite State Machine Model Specification

## Purpose

Defines Kamacite's canonical finite-state-machine artifact model: machine descriptors, state and event vocabularies, transition relations, completeness matrices, determinism policy, reachability rules, terminal-state rules, and trace receipts.

## Requirements

### Requirement: FSM IR boundary

r[kamacite.fsms.ir_boundary] Kamacite MUST model finite-state machines as canonical data artifacts and receipts rather than as a Kamacite source language, evaluator, runtime, or proof engine. FSM artifacts MUST use explicit schema versions, Preserves canonical bytes or supported content references, BLAKE3 identity over Kamacite-owned canonical bytes, and explicit non-claims.

#### Scenario: FSM descriptor identity is canonical

r[kamacite.fsms.ir_boundary.canonical_identity]

- GIVEN an FSM descriptor has canonical bytes and metadata aliases
- WHEN Kamacite computes the descriptor identity
- THEN the BLAKE3 hash over canonical bytes MUST define identity and aliases MUST remain metadata.

#### Scenario: Unsupported FSM version fails closed

r[kamacite.fsms.ir_boundary.unsupported_version]

- GIVEN an FSM artifact declares an unsupported schema version
- WHEN Kamacite validates the artifact
- THEN validation MUST reject it with a deterministic unsupported-fsm-version diagnostic.

#### Scenario: Kamacite does not become an FSM evaluator

r[kamacite.fsms.ir_boundary.no_evaluator_claim]

- GIVEN an artifact declares FSM states, events, transitions, traces, or completeness receipts
- WHEN Kamacite validates the artifact
- THEN validation MUST classify the artifact as data and MUST NOT claim Kamacite executed a runtime state machine unless a separate bounded runtime receipt is present.

### Requirement: FSM descriptor vocabulary

r[kamacite.fsms.descriptor_vocabulary] A Kamacite `FsmDescriptor` MUST bind a schema version, machine identity, state identities, event identities, initial state identity, terminal state identities, error or rejection state identities when present, transition relation identity, determinism policy, totality policy, dependency identities, optional source spans, and explicit non-claims. State and event identity MUST be derived from canonical state or event entries rather than human names alone.

#### Scenario: Missing initial state fails closed

r[kamacite.fsms.descriptor_vocabulary.initial_state_required]

- GIVEN an FSM descriptor omits the initial state or names an initial state outside the declared state set
- WHEN Kamacite validates the descriptor
- THEN validation MUST reject it with a deterministic invalid-initial-state diagnostic.

#### Scenario: State and event names are metadata

r[kamacite.fsms.descriptor_vocabulary.names_metadata]

- GIVEN a state or event keeps the same canonical entry while a display name or alias changes
- WHEN Kamacite computes state, event, or machine identity
- THEN the identity MUST remain stable and the display change MUST be recorded as metadata.

#### Scenario: Duplicate state or event identity fails closed

r[kamacite.fsms.descriptor_vocabulary.duplicate_identity]

- GIVEN two states or events in the same descriptor declare the same identity
- WHEN Kamacite validates the descriptor
- THEN validation MUST reject it with a deterministic duplicate-fsm-identity diagnostic.

### Requirement: FSM transition relation vocabulary

r[kamacite.fsms.transition_relation] A Kamacite transition relation MUST bind transition identities, source state identities, event identities, optional guard descriptor identities, target state identities, action or effect summary identities when present, checked laws, diagnostics policy, dependency identities, and explicit non-claims. A transition MUST NOT name a state, event, guard, action, or effect outside declared descriptor dependencies.

#### Scenario: Unknown transition endpoint is rejected

r[kamacite.fsms.transition_relation.unknown_endpoint]

- GIVEN a transition names a source or target state missing from the descriptor state set
- WHEN Kamacite validates the transition relation
- THEN validation MUST reject it with a deterministic unknown-fsm-state diagnostic.

#### Scenario: Unknown event is rejected

r[kamacite.fsms.transition_relation.unknown_event]

- GIVEN a transition names an event missing from the descriptor event set
- WHEN Kamacite validates the transition relation
- THEN validation MUST reject it with a deterministic unknown-fsm-event diagnostic.

#### Scenario: Transition action is not host authority

r[kamacite.fsms.transition_relation.action_not_authority]

- GIVEN a transition references an action, adapter, effect summary, handler descriptor, or host-backed behavior
- WHEN Kamacite validates the transition relation
- THEN validation MUST treat the reference as symbolic data and MUST NOT classify it as filesystem, process, network, clock, random, credential, compiler, evaluator, or mutation authority without separate capability admission evidence.

### Requirement: FSM completeness matrix

r[kamacite.fsms.completeness_matrix] A Kamacite FSM completeness receipt MUST bind the descriptor identity, declared state set, declared event set, guard partition profile, totality policy, coverage matrix, missing-cell diagnostics, excluded unreachable cells, checked laws, and non-claims. For every covered state and event pair, validation MUST record an explicit transition, explicit denial, unreachable-state reason, impossible-event reason, or other reviewed coverage reason; undefined behavior MUST NOT be inferred.

#### Scenario: Complete deterministic table is accepted

r[kamacite.fsms.completeness_matrix.complete_table]

- GIVEN every declared reachable state and declared event pair has a transition, denial, or reviewed coverage reason
- WHEN Kamacite validates the completeness receipt
- THEN validation MAY accept the FSM completeness claim if all other descriptor checks pass.

#### Scenario: Missing matrix cell fails closed

r[kamacite.fsms.completeness_matrix.missing_cell]

- GIVEN a reachable state and declared event pair has no transition, denial, or reviewed coverage reason
- WHEN Kamacite validates the completeness receipt
- THEN validation MUST reject it with a deterministic missing-fsm-transition-cell diagnostic.

#### Scenario: Explicit denial is coverage, not execution

r[kamacite.fsms.completeness_matrix.explicit_denial]

- GIVEN a state and event pair is explicitly denied by descriptor policy
- WHEN Kamacite validates the completeness matrix
- THEN validation MAY count the denial as matrix coverage and MUST NOT claim any runtime event was executed.

### Requirement: FSM determinism policy

r[kamacite.fsms.determinism_policy] A trusted Kamacite FSM descriptor MUST declare its determinism policy. The default trusted profile MUST be deterministic and MUST reject overlapping or conflicting transitions for the same source state, event, and guard partition. Nondeterministic, probabilistic, concurrent, or timed-machine variants MUST fail closed unless a later accepted change defines their policy and fixture coverage.

#### Scenario: Conflicting deterministic transitions are rejected

r[kamacite.fsms.determinism_policy.conflict]

- GIVEN a deterministic FSM descriptor contains two enabled transitions for the same source state, event, and guard partition with different targets or action summaries
- WHEN Kamacite validates the descriptor
- THEN validation MUST reject it with a deterministic conflicting-fsm-transition diagnostic.

#### Scenario: Nondeterminism is not silently coerced

r[kamacite.fsms.determinism_policy.nondeterminism_explicit]

- GIVEN an FSM descriptor requests nondeterministic, probabilistic, concurrent, or timed behavior
- WHEN no accepted profile defines that behavior
- THEN validation MUST reject the descriptor rather than coercing it into the deterministic profile.

### Requirement: FSM reachability and terminal-state policy

r[kamacite.fsms.reachability_terminal_policy] A Kamacite FSM descriptor MUST classify state reachability and terminal-state behavior explicitly. Every declared state MUST be reachable from an initial state or marked as intentionally unreachable metadata. Terminal, accepting, rejected, or error states MUST NOT have outgoing transitions unless the descriptor declares a restart, cleanup, or escalation policy with explicit checked laws and non-claims.

#### Scenario: Unmarked unreachable state fails closed

r[kamacite.fsms.reachability_terminal_policy.unreachable_unmarked]

- GIVEN an FSM descriptor declares a state that is not reachable from the initial state and is not marked intentionally unreachable
- WHEN Kamacite validates reachability
- THEN validation MUST reject it with a deterministic unreachable-fsm-state diagnostic.

#### Scenario: Terminal outgoing transition fails closed

r[kamacite.fsms.reachability_terminal_policy.terminal_outgoing]

- GIVEN a terminal, accepting, rejected, or error state has an outgoing transition without a declared restart, cleanup, or escalation policy
- WHEN Kamacite validates terminal-state behavior
- THEN validation MUST reject it with a deterministic terminal-fsm-transition diagnostic.

### Requirement: FSM trace receipts

r[kamacite.fsms.trace_receipts] A Kamacite FSM trace receipt MUST bind descriptor identity, input event sequence identity, start state, transition identities, final state, diagnostics, checked laws, output identities when present, and non-claims. A trace receipt MAY prove that a recorded path is consistent with a descriptor, but MUST NOT prove runtime correctness, authorization, timing behavior, fairness, liveness, concurrency safety, or arbitrary program semantic equivalence.

#### Scenario: Valid trace path is accepted

r[kamacite.fsms.trace_receipts.valid_path]

- GIVEN a trace starts at the descriptor initial state and every recorded transition is enabled by the descriptor for the current state and event
- WHEN Kamacite validates the trace receipt
- THEN validation MAY accept the trace consistency claim and MUST bind the descriptor identity, input sequence identity, transition identities, final state, checked laws, and non-claims.

#### Scenario: Impossible trace transition is rejected

r[kamacite.fsms.trace_receipts.impossible_transition]

- GIVEN a trace records an event or transition that is not enabled by the descriptor from the current state
- WHEN Kamacite validates the trace receipt
- THEN validation MUST reject it with a deterministic impossible-fsm-transition diagnostic.

#### Scenario: Trace receipt preserves non-claims

r[kamacite.fsms.trace_receipts.non_claims]

- GIVEN an FSM descriptor or completeness receipt states non-claims about runtime correctness, authorization, liveness, fairness, or semantic equivalence
- WHEN Kamacite emits or validates a trace receipt
- THEN every non-claim MUST remain present and the trace receipt MUST NOT add a contradictory stronger claim.

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
