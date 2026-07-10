## Context

Kamacite's layered model already separates source envelopes, canonical Preserves values, adapter descriptors, target artifacts, and receipts. The Component Model can improve the executable adapter ABI, but only if those layers remain separate and generated WIT bindings do not become a competing canonical IR.

The first suite should be deliberately narrow: a component consumes bounded canonical bytes, may call only declared host functions, and returns bounded canonical bytes plus explicit error information.

## Decisions

### 1. WIT describes the executable shell only

**Choice:** Define a versioned WIT package/world for adapter invocation, declared host functions, and explicit result/error classes. Domain values remain canonical Preserves bytes validated against Kamacite schemas. WIT source is preserved as its own source envelope with parser/tool identity and BLAKE3.

**Rationale:** WIT supplies generated canonical-ABI bindings without replacing Kamacite value identity or source preservation.

### 2. Adapter layers keep independent identities

**Choice:** Record separate typed identities for WIT source/package/world, source envelope, canonical input, adapter descriptor, portable component, runtime profile, execution plan, canonical output, target artifact, and receipt. Parent edges are explicit and no derived artifact inherits a source-losslessness or semantic-equivalence claim.

**Rationale:** Layer separation is Kamacite's central honesty boundary.

### 3. Descriptors bind the complete execution contract

**Choice:** A Wasm adapter descriptor declares adapter id/version, WIT package/world, exact component ref, imports/exports, canonical input/output schema refs, capability/effect row, resource profile, deterministic runtime profile, toolchain cohort, optional build/provenance refs, and non-claims.

**Rationale:** A component filename plus world cannot explain what effects or limits are expected.

### 4. Planning and validation stay in pure core

**Choice:** Pure core validates descriptor shape, WIT/component facts supplied by an inspection shell, identity links, schema refs, capability/effect declarations, resource bounds, runtime compatibility, canonical input/output bytes, and receipt payloads. It returns an execution plan or ordered blockers.

**Rationale:** Kamacite core remains testable without Wasmtime, files, subprocesses, or ambient authority.

### 5. The shell consumes the shared deterministic component profile

**Choice:** The imperative adapter runner uses the admitted Molten-compatible Wasmtime component cohort and configuration rather than inventing another runtime profile. It instantiates only from the pure plan and exposes only declared host functions. Missing sibling profile evidence keeps execution unavailable while static descriptor validation remains possible.

**Rationale:** One stack component profile reduces incompatible Wasm assumptions, while each repo retains its own claim owner.

### 6. Capabilities and algebraic effects are explicit data

**Choice:** Map each component import to a declared Kamacite capability/effect operation, handler profile, policy/authority reference where required, and replay class. Unknown or ambient imports deny. Declaring the operation does not grant host authority; the shell must receive separate admitted capability handles or deterministic fixtures.

**Rationale:** Kamacite models effects as data and receipts, not as ambient runtime powers.

### 7. Preserves input/output is checked before success

**Choice:** Validate exact input bytes before invocation and exact output bytes after invocation against canonical Preserves and declared schema identities. The receipt binds input/output BLAKE3, component/WIT/profile, hostcall/effect summary, resources, result class, and non-claims. Invalid output cannot become a successful adapter result.

**Rationale:** WIT byte-carrier validity alone is weaker than Kamacite canonical-value validity.

### 8. Valence export remains a projection

**Choice:** Export adapter-stage identities, spans, roles, edges, and non-claims through a deterministic Valence-friendly projection after the Kamacite receipt validates. Valence owns Evidence IR linkage; the export does not transfer Kamacite round-trip semantics.

## Functional core / imperative shell split

- **Pure core**: descriptor/profile validation, identity and parent-edge checks, capability/effect/resource admission, WIT/component fact comparison, canonical Preserves input/output validation, execution-plan construction, receipt construction/validation, and Valence projection DTOs.
- **Imperative shell**: read WIT/component/input bytes, inspect artifacts, invoke Wasmtime component APIs, supply admitted host functions, perform approved effects, measure resources, and persist outputs/receipts.

## Risks / Trade-offs

- A byte-carrier ABI gives weaker generated domain typing but avoids duplicate canonical schemas.
- Component tooling drift can invalidate bindings or world resolution. Pin one stack cohort and treat upgrades as explicit compatibility changes.
- Wasmtime isolation is not complete authority or sandbox proof. Keep host functions deny-by-default and authority external.
- Direct sibling crate dependencies can couple release cadence. Begin with profile/receipt fixtures and add direct dependencies only when stable APIs are required.

## Non-Goals

- No Wasm canonical IR, arbitrary source translation, or Nickel/Rust/Steel semantic-equivalence claim.
- No general component package manager, composer, build system, remote RPC, or runtime service in Kamacite.
- No ambient WASI or authority derived from WIT/component validity.
- No JavaScript, Python, WAMR, WASIp3, Wizer, or AOT optimization in the first adapter suite.
