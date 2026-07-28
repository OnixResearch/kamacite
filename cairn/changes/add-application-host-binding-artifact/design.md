## Context

Kamacite has separate canonical identities for source envelopes, values, adapters, targets, effects, handlers, resources, and receipts. This separation prevents one artifact role from silently replacing another.

The missing layer is a small composition artifact for one executable unit. Product repositories currently own that composition through different profile and admission records. A common binding can improve inspection and interchange if it links existing identities instead of redefining product policy.

Roc provides the design reference for making application/platform composition explicit. This change adapts that presentation model only. Roc compiler and platform interfaces remain unstable and outside the trust base.

## Decisions

### Decision: Kamacite owns the portable binding artifact

**Choice:** Add a canonical Preserves `ApplicationHostBinding` artifact with its own version and identity domain.

**Rationale:** The binding is portable interchange data across application, adapter, effect, and host-profile identities. It is not runtime policy or evidence linkage authority.

**Rejected alternatives:**

- Put the schema in Animus. That would make the cross-stack contract agent-specific.
- Put it in Valence. Valence links evidence but does not define application or host semantics.
- Put it in Aspen. That would make the contract Wasmtime-specific.

### Decision: Bind existing identities instead of copying policy

**Choice:** Canonical identity material includes:

- binding schema and identity domain;
- application artifact identity;
- executable entrypoint identity;
- input and output schema identities;
- one closed effect-summary identity;
- exactly one host-profile identity;
- one adapter-cohort identity;
- one resource-profile identity;
- compatibility and policy identities when required; and
- explicit non-claims.

Display names, source paths, source spans, help text, and aliases remain review metadata.

**Rationale:** Product owners retain policy and enforcement. The binding only commits to the exact reviewed composition.

### Decision: Exactly one host applies to one executable unit

**Choice:** An executable binding contains one scalar host-profile identity. Missing, repeated, alternative, or fallback host selections fail admission.

**Rationale:** A caller must know which host contract applies before dispatch. Multi-runtime workflows can contain many executable units, each with its own binding.

### Decision: Executable effect requirements are closed

**Choice:** An executable binding references one closed Kamacite effect summary. Open, unknown, undeclared, or stale operation sets can remain review artifacts but cannot pass executable binding admission.

**Rationale:** A binding cannot explain host compatibility when required effects remain unknown.

The summary remains symbolic requirement data. It does not grant host capabilities.

### Decision: Runtime and authority evidence stay separate

**Choice:** Matching application, effect, host, adapter, and resource identities never replace current authorization, capability checks, sandbox admission, resource enforcement, adapter readiness, or runtime observations.

**Rationale:** An exact composition record proves identity and declared compatibility only.

### Decision: Validation uses the existing core and shell split

**Choice:** `kamacite-core` validates loaded DTOs, canonicalizes Preserves values, computes BLAKE3 identity, and returns deterministic diagnostics. The CLI owns bounded file reads, manifest discovery, compatibility JSON, and receipt output.

**Rationale:** The change must not add filesystem, process, network, clock, credential, or execution authority to the core.

### Decision: Cross-repository adoption starts with frozen fixtures

**Choice:** The first Animus producer is admitted through an exact frozen fixture, schema identity, producer revision, and conformance receipt. Direct crate dependencies remain deferred.

**Rationale:** Fixture coupling lets the schema and claim boundary stabilize before release coupling begins.

### Decision: Projection roles remain bounded

**Choice:** A Valence-friendly projection identifies the application, host, binding, effect, adapter, and resource roles. It preserves source spans and non-claims when present.

**Rationale:** Valence can wrap and link the receipt without taking ownership of binding semantics.

## Test Design

Positive fixtures cover a capability-free application, a closed symbolic-effect application, stable metadata renames, exact Animus projection, and Valence projection.

Negative fixtures cover missing or multiple hosts, open or unknown effects, undeclared operations, stale application or profile identity, wrong adapter cohort, wrong resource profile, unsupported schema version, unsafe diagnostics, and authority overclaims.

Property tests cover canonical field order, metadata exclusion, deterministic BLAKE3 identity, and single-field semantic re-keying.

## Risks and Trade-offs

- A universal binding can become a policy dumping ground. The schema links exact owner-defined identities and rejects product-local policy copies.
- Closed effect rows can exclude programs with dynamic behavior. Such programs remain non-executable review artifacts until a bounded profile closes the row.
- Consumer profiles can drift. Exact identities and negative stale-cohort fixtures make drift visible.
- The name can imply runtime enforcement. Receipts and documentation require explicit identity-only and no-authority non-claims.

## Claim Boundary

A passing binding proves canonical identity, required-field validity, one selected host profile, closed declared effects, and exact linked cohort identities. It does not prove application correctness, effect completeness beyond supplied evidence, authorization, host behavior, sandboxing, resource enforcement, runtime success, or release eligibility.
