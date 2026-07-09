## Context

Kamacite's current walking skeleton has several implicit stateful surfaces: validation verdicts are accepted or rejected, fixture classes control trust boundaries, golden outputs are matched or stale, adapter commands switch between stored-evidence and explicit execution modes, and algebraic-effect receipts classify residual and discharged operations. Those surfaces are bounded, but they are not modeled as reusable FSM artifacts.

The missing product decision is not an evaluator. Kamacite needs canonical data for finite machines and deterministic receipts that prove a descriptor is complete for its declared domain. Runtime execution, model checking, proof search, and host authority remain outside the core unless separate adapters admit them.

## Decisions

### 1. Kamacite owns canonical FSM artifacts

**Choice:** Define FSM descriptors, transition relations, trace receipts, and completeness receipts as Kamacite artifact classes with Preserves-backed canonical identity and BLAKE3 hashes.

**Rationale:** FSMs describe bounded control boundaries that can appear in validators, adapters, lifecycle phases, runtime traces, and release evidence. Kamacite owns the interchange and receipt shape; downstream consumers can reference those identities without inventing incompatible schemas.

**Rejected alternative:** Let each consumer encode machines as ad hoc JSON, Rust enums, Markdown diagrams, or runtime-only traces. That would make hidden defaults, identity drift, and transition gaps hard to audit.

### 2. Completeness is finite-domain coverage, not arbitrary behavioral proof

**Choice:** An FSM completeness claim applies only to the descriptor's declared finite state set, declared finite event alphabet, declared guard partitions, and declared totality policy. Every state/event cell must be explicitly covered by a transition, denial, unreachable-state mark, impossible-event mark, or other reviewed reason.

**Rationale:** This gives reviewers an observable completion criterion while avoiding an overclaim that Kamacite proves arbitrary program behavior. Missing cells fail closed instead of silently becoming ignored events or default rejection paths.

**Rejected alternative:** Treat a transition table as complete when tests pass for a few traces. Trace fixtures are necessary evidence, but they cannot replace finite coverage of the declared descriptor.

### 3. Deterministic machines are the default trusted slice

**Choice:** The first trusted FSM profile is deterministic by default. For a given state, event, and guard partition, validation rejects overlapping or conflicting transitions. Nondeterministic machines require an explicit policy, explicit receipts, and separate fixture coverage before they are trusted.

**Rationale:** Determinism makes receipts stable and makes missing or conflicting transitions easier to diagnose. It also matches the current validation and lifecycle command surfaces.

**Rejected alternative:** Admit nondeterminism by default and rely on runtime choices. That would make replay receipts ambiguous and invite hidden control flow.

### 4. Actions and effects stay separate from host authority

**Choice:** Transitions may reference action descriptors, effect summaries, adapter identities, or handler receipts, but the transition itself does not grant filesystem, process, network, clock, random, credential, compiler, evaluator, or mutation authority.

**Rationale:** Kamacite already denies ambient host authority. FSM modeling must not weaken that boundary by making transition labels equivalent to permissions.

**Rejected alternative:** Let transition actions execute during core validation. That would turn a data model into an evaluator and break the functional-core boundary.

### 5. Trace receipts are evidence, not execution semantics

**Choice:** FSM trace receipts bind a descriptor identity, input event sequence, start state, observed transition IDs, final state, diagnostics, checked laws, and non-claims. They can prove that a recorded path is consistent with a validated descriptor, but they do not prove that a runtime correctly implemented the machine unless separate runtime evidence is present.

**Rationale:** This keeps traces useful for Valence exports, Aspen/Molten runtime observations, and Cairn lifecycle evidence while preserving bounded claims.

**Rejected alternative:** Treat a successful trace as proof of runtime correctness. A trace is one recorded path, not a proof that all implementations or executions conform.

### 6. Existing Kamacite control surfaces become candidate FSM fixtures

**Choice:** Later implementation should model the validation command, golden-update workflow, adapter execution-mode gates, and algebraic-effect handler discharge profile as candidate FSM fixtures after the generic FSM descriptor and receipt schema land.

**Rationale:** These are the stateful surfaces already present in the repo. Modeling them as fixtures will exercise the schema on real Kamacite behavior without claiming arbitrary language semantics.

**Rejected alternative:** Start with an abstract toy FSM only. Toy fixtures are useful for seed coverage, but they will not prove the model fits Kamacite's actual control boundaries.

### 7. Stack consumers keep bounded roles

**Choice:** Trellis may prove finite transition and reachability facts; Valence may wrap FSM receipts as evidence; Aspen/Molten may emit runtime trace receipts; Octet may check Rust source-shape and provenance for validators; Cairn may consume lifecycle-machine evidence; Mantle may record release-machine evidence. None of these replaces Kamacite's canonical FSM descriptor or receipt validation.

**Rationale:** This mirrors existing stack boundaries and prevents a proof artifact, runtime trace, lifecycle gate, or build receipt from becoming the canonical FSM model by accident.

**Rejected alternative:** Let a downstream lifecycle/runtime/build artifact define the FSM schema. That would fragment identity and blur ownership.

## Risks / Trade-offs

- **Semantic overclaim:** Mitigate by saying FSM artifacts are data and receipts, not a Kamacite evaluator or proof of arbitrary behavior.
- **Coverage false confidence:** Mitigate by defining completeness over declared finite domains and requiring non-claims for external behavior.
- **Nondeterminism ambiguity:** Mitigate by making deterministic machines the first trusted profile and rejecting implicit conflicts.
- **Host-authority confusion:** Mitigate by separating transition actions/effects from capability admission.
- **Fixture burden:** Mitigate by starting with a small seed machine plus real Kamacite command/update-mode machines, then expanding after evidence passes.
- **Stack drift:** Mitigate by requiring stack consumers to reference Kamacite descriptor and receipt identities rather than defining parallel schemas.
