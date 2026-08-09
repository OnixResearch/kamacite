## Context

The active lossless-IR work establishes Kamacite as the source-envelope, canonical-value, adapter, identity, and receipt boundary for Nickel, Rust, Steel, and future adapter targets. It already requires explicit effect/capability labels, but those labels are not enough to model algebraic effects as portable data.

Algebraic effects need operation signatures, effect rows, summaries, handlers, continuation policies, and replay/discharge receipts. If those are defined first in Aspen/Molten, they become runtime-specific. If they are defined first in Octet, they risk looking like a Rust language extension. If they remain only in Trellis, they are proof artifacts rather than the product-facing receipt contract. Kamacite is the correct place to define the canonical data model and bounded claims.

## Decisions

### 1. Kamacite owns the canonical algebraic-effect model

**Choice:** Kamacite defines effect signatures, rows, summaries, handler descriptors, handler-application receipts, and effect-log projections as canonical artifact classes.

**Rationale:** Kamacite owns portable interchange semantics and receipt contracts. Algebraic effects are part of that semantic boundary because they describe what an adapter or constrained policy value may request and how those requests are discharged or left residual.

**Rejected alternative:** Define algebraic effects in Aspen/Molten first. Aspen/Molten should consume and execute handler profiles, not own the canonical interchange model.

### 2. Algebraic effects are data, not a new Kamacite evaluator

**Choice:** The first algebraic-effect boundary is a canonical data and validation model. It does not introduce a Kamacite programming language, source syntax, evaluator, type checker, or ambient runtime.

**Rationale:** Kamacite can validate signatures, rows, handler descriptors, receipts, and non-claims without claiming to execute arbitrary programs. That keeps the existing source-envelope and constrained-policy boundaries intact.

**Rejected alternative:** Treat algebraic-effect support as permission to call Kamacite a programming language. That would overclaim semantics that are not defined by this change.

### 3. Operation signatures stay separate from host capabilities

**Choice:** Algebraic operations are declared symbolic operations with input/output schemas and effect classes. Host capabilities are separate shell permissions that must be admitted through adapter descriptors and receipts.

**Rationale:** An operation such as `ReadConfig` or `CurrentTime` may appear in an algebraic-effect row, but declaring that operation must not grant filesystem or clock authority. Runtime shells can map operations to host actions only through explicit, denied-by-default capability gates.

**Rejected alternative:** Equate algebraic operations with host permissions. That would weaken Kamacite's ambient-authority denial model and confuse semantic requirements with side-effect authority.

### 4. Handler descriptors declare discharge and residual effects

**Choice:** Handler descriptors must bind handled operation identities, accepted input rows, produced residual rows, continuation policy, replay profile, adapter/evaluator identity, dependency identities, checked laws, and non-claims.

**Rationale:** A handler is only trustworthy when Kamacite can state exactly which operations it discharges, what remains, and what obligations were checked. Residual effects must be explicit so downstream runtimes and evidence consumers do not infer purity from partial handling.

**Rejected alternative:** Accept handler names or paths as evidence. Names are metadata and cannot prove discharge, residual row shape, or continuation safety.

### 5. Continuation policy is explicit and fail-closed

**Choice:** Handler descriptors and receipts must classify continuation use with explicit policies such as no-continuation, single-use continuation, or unsupported-continuation. A receipt that claims a policy but lacks matching evidence fails closed.

**Rationale:** Single-use continuation discipline is a key algebraic-effect safety claim. Kamacite should record and validate the claim shape even if Trellis or a runtime later supplies deeper proof or execution evidence.

**Rejected alternative:** Treat continuation behavior as an implementation detail of a runtime handler. That would make receipts unable to distinguish safe discharge from ambiguous control behavior.

### 6. Replay and determinism are receipt claims

**Choice:** Effect summaries and handler-application receipts must classify deterministic, recorded-replay, denied, and non-replayable effect behavior. Effect logs are evidence projections, not canonical proof of host authority.

**Rationale:** Aspen/Molten and other runtimes need replayable effect logs, but Kamacite should define the identity and non-claim boundary. A recorded response can support deterministic replay of a bounded run without proving the original host action was authorized or semantically correct.

**Rejected alternative:** Let runtimes attach ad hoc effect logs without Kamacite schemas. That would make replay evidence hard to compare, hash, export, or deny consistently.

### 7. Trellis, Valence, Aspen/Molten, and Octet keep bounded roles

**Choice:** Trellis provides proof/reference artifacts for finite algebraic-effect facts. Valence wraps Kamacite effect receipts as evidence identities and roles. Aspen/Molten executes handler profiles and consumes logs. Octet checks Kamacite Rust implementation surfaces and policy metadata only.

**Rationale:** This keeps ownership aligned with the stack: Kamacite defines canonical effect data, Trellis proves bounded logic, Valence links evidence, Aspen/Molten runs effects, and Octet checks Rust without adding a Rust algebraic-effect language system.

**Rejected alternative:** Promote any downstream consumer's artifact into Kamacite canonical identity. That would blur proof, runtime, evidence, and source-checking boundaries.

## Risks / Trade-offs

- **Semantic overclaim:** Mitigate by saying this change defines data and receipt validation, not a Kamacite evaluator or language.
- **Host-authority confusion:** Mitigate by keeping algebraic operations separate from shell capabilities and requiring denied-by-default adapter admission.
- **Partial handler confusion:** Mitigate with residual effect rows and deterministic diagnostics for undeclared or unhandled operations.
- **Continuation ambiguity:** Mitigate by requiring explicit continuation policies and fail-closed receipts.
- **Runtime-specific drift:** Mitigate by making Aspen/Molten consume Kamacite-defined schemas instead of inventing runtime-only handler logs.
- **Proof-artifact overreach:** Mitigate by treating Trellis artifacts as reference/proof evidence that cannot replace Kamacite canonical artifacts or Kamacite receipt checks.
