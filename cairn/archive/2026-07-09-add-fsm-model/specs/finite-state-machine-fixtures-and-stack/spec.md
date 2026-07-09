# Finite State Machine Fixtures and Stack Roles Specification

## Purpose

Defines the fixture corpus and stack integration rules required before Kamacite treats finite-state-machine descriptors, completeness receipts, and trace receipts as trusted evidence.

## Requirements

### Requirement: FSM fixture corpus

r[kamacite.fsms.fixture_corpus] Kamacite MUST add positive, negative, property-style, and golden receipt fixtures before FSM descriptors, completeness receipts, or trace receipts are accepted as trusted evidence. Fixtures MUST cover descriptor identity, state and event vocabulary, transition coverage, determinism conflicts, reachability, terminal-state behavior, trace consistency, non-claims, stale hashes, wrong identity domains, unsupported versions, and host-authority confusion.

#### Scenario: Positive complete deterministic fixture is required

r[kamacite.fsms.fixture_corpus.positive_complete]

- GIVEN Kamacite claims support for the deterministic FSM profile
- WHEN the FSM fixture corpus is reviewed
- THEN the corpus MUST include a positive descriptor fixture whose reachable state/event coverage matrix is complete and whose golden receipt binds descriptor identity, checked laws, and non-claims.

#### Scenario: Negative missing-cell fixture is required

r[kamacite.fsms.fixture_corpus.negative_missing_cell]

- GIVEN Kamacite validates FSM completeness receipts
- WHEN the FSM fixture corpus is reviewed
- THEN the corpus MUST include a negative fixture with at least one reachable state/event matrix cell lacking a transition, denial, or reviewed coverage reason.

#### Scenario: Negative conflict fixture is required

r[kamacite.fsms.fixture_corpus.negative_conflict]

- GIVEN Kamacite validates deterministic FSM descriptors
- WHEN the FSM fixture corpus is reviewed
- THEN the corpus MUST include a negative fixture with conflicting transitions for the same source state, event, and guard partition.

#### Scenario: Negative stale identity fixture is required

r[kamacite.fsms.fixture_corpus.negative_stale_identity]

- GIVEN Kamacite computes FSM descriptor, transition, completeness, or trace identities
- WHEN the FSM fixture corpus is reviewed
- THEN the corpus MUST include a negative fixture whose declared BLAKE3 identity does not match the canonical bytes.

#### Scenario: Negative host-authority confusion fixture is required

r[kamacite.fsms.fixture_corpus.negative_host_confusion]

- GIVEN a transition references an action, effect, adapter, runtime handler, or host-backed behavior
- WHEN the FSM fixture corpus is reviewed
- THEN the corpus MUST include a negative fixture proving that the transition reference does not grant undeclared filesystem, process, network, clock, random, credential, compiler, evaluator, or mutation authority.

### Requirement: FSM command and adapter usage

r[kamacite.fsms.command_adapter_usage] Kamacite command workflows, fixture harness modes, adapter execution modes, golden update workflows, and algebraic-effect handler profiles MAY be modeled as FSM fixtures only when their descriptors and receipts preserve the existing functional-core and capability boundaries. The pure core MUST validate normalized FSM DTOs and bytes in memory; shells MAY read files, invoke explicitly admitted tools, compare goldens, or emit update receipts.

#### Scenario: Validation command workflow can be modeled

r[kamacite.fsms.command_adapter_usage.validation_workflow]

- GIVEN the validation command has states such as input discovery, manifest freshness check, pure fixture validation, golden comparison, report rendering, and failure
- WHEN Kamacite models that workflow as an FSM fixture
- THEN the descriptor MUST make each state, event, terminal outcome, error outcome, and update-mode transition explicit rather than relying on CLI branch structure alone.

#### Scenario: Adapter execution mode gates can be modeled

r[kamacite.fsms.command_adapter_usage.adapter_modes]

- GIVEN an adapter command distinguishes stored-evidence, explicit evaluator execution, DTO probe, update-golden, or denied execution modes
- WHEN Kamacite models that command as an FSM fixture
- THEN each effectful transition MUST require declared capability and execution-mode evidence before the trace receipt is trusted.

#### Scenario: Pure core remains evaluator-free

r[kamacite.fsms.command_adapter_usage.pure_core]

- GIVEN an FSM descriptor references validators, adapters, evaluators, compilers, runtimes, or host effects
- WHEN the pure Kamacite core validates the descriptor or receipt
- THEN validation MUST operate over normalized in-memory DTOs and MUST NOT read files, inspect environment, spawn processes, access clocks, access randomness, open network connections, read credentials, or mutate state.

### Requirement: FSM stack roles

r[kamacite.fsms.stack_roles] Kamacite MUST keep stack roles explicit for finite-state-machine evidence. Trellis proof artifacts, Valence evidence exports, Aspen/Molten runtime traces, Octet Rust checks, Cairn lifecycle gates, and Mantle release evidence MAY support or consume Kamacite FSM artifacts, but they MUST NOT replace Kamacite canonical descriptor identity or Kamacite receipt validation.

#### Scenario: Trellis proofs are supporting evidence

r[kamacite.fsms.stack_roles.trellis]

- GIVEN a Trellis proof artifact proves reachability, determinism, completeness, or trace consistency for a finite machine
- WHEN Kamacite records that proof as FSM evidence
- THEN the proof MAY be referenced as supporting evidence and MUST NOT replace the Kamacite FSM descriptor, completeness receipt, trace receipt, or fixture checks.

#### Scenario: Valence wraps FSM receipts without broadening claims

r[kamacite.fsms.stack_roles.valence]

- GIVEN a Kamacite FSM descriptor, completeness receipt, or trace receipt is exported for Valence consumption
- WHEN Valence-friendly evidence is produced
- THEN the export MUST preserve Kamacite stable IDs, source spans when present, roles, BLAKE3 hashes, diagnostics, checked laws, and non-claims without adding stronger runtime or semantic-equivalence claims.

#### Scenario: Aspen and Molten consume runtime traces

r[kamacite.fsms.stack_roles.aspen_molten]

- GIVEN Aspen or Molten emits runtime observations for a state machine
- WHEN Kamacite records those observations
- THEN they MUST be modeled as trace or runtime evidence referencing a Kamacite FSM descriptor and MUST NOT become canonical FSM schema authority.

#### Scenario: Octet checks implementation surfaces only

r[kamacite.fsms.stack_roles.octet]

- GIVEN Octet checks pass for Rust code that validates or emits FSM artifacts
- WHEN Kamacite records Octet results
- THEN the receipt MUST classify them as Rust source-shape, policy, or provenance evidence and MUST NOT claim whole-machine semantic correctness by itself.

#### Scenario: Cairn lifecycle machines are external consumers

r[kamacite.fsms.stack_roles.cairn]

- GIVEN Cairn lifecycle phases, gates, sync flows, archive flows, or release-readiness checks are modeled as FSMs
- WHEN Kamacite records those models
- THEN the models MUST be Kamacite descriptor and receipt artifacts consumed by Cairn evidence and MUST NOT override Cairn's repo-local lifecycle authority.

#### Scenario: Mantle release machines remain release evidence

r[kamacite.fsms.stack_roles.mantle]

- GIVEN Mantle records build, package, or release workflow state as finite-machine evidence
- WHEN Kamacite records that evidence
- THEN it MUST remain release/build evidence referencing Kamacite FSM identities and MUST NOT imply source losslessness, evaluator soundness, or semantic equivalence.

### Requirement: FSM implementation admission

r[kamacite.fsms.implementation_admission] Kamacite MUST NOT treat FSM support as implemented until Preserves schemas, pure Rust DTO validators, positive fixtures, negative fixtures, golden receipts, manifest entries, docs, and Cairn validation pass together. Default validation MUST remain non-mutating, and any generated FSM golden updates MUST require an explicit update mode that records old and new identities.

#### Scenario: FSM support remains untrusted before fixtures pass

r[kamacite.fsms.implementation_admission.before_fixtures]

- GIVEN FSM schema files or Rust DTOs exist but positive, negative, and golden fixture evidence has not passed
- WHEN Kamacite reports implementation status
- THEN FSM support MUST remain untrusted for Kamacite claim admission.

#### Scenario: Explicit update records FSM identity changes

r[kamacite.fsms.implementation_admission.update_receipt]

- GIVEN an operator intentionally refreshes FSM descriptor, completeness, trace, manifest, or golden receipt evidence
- WHEN the update workflow writes changed artifacts
- THEN it MUST emit an update receipt listing changed fixture IDs, old identities, new identities, checked laws, diagnostics, and non-claims.
