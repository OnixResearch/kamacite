## Context

Kamacite already separates source envelopes, projections, evaluator identities, target artifacts, and receipts. Marigold's analyzer returns JSON-serializable stream and program summaries containing cardinality, time/space classes, exact expression strings, and `collects_input`. The analyzer reasons over recognized pipeline operators; it does not inspect arbitrary called-function bodies or supply Kamacite authority, canonical identity, or replay semantics.

The pilot needs one real upstream invocation to determine whether the report is useful while keeping the dependency and authority surface outside the pure core.

## Decisions

### 1. Treat Marigold analysis as advisory projection evidence

**Choice:** The normalized analyzer report receives a `projection` identity and an `advisory` role. Exact source bytes receive a separate `source` identity, and the pinned analyzer descriptor receives an `evaluator` identity.

**Rationale:** Analyzer output is derived metadata. It is neither source identity nor canonical Kamacite IR and cannot be promoted into behavioral evidence by a successful parse.

### 2. Pin one analyzer cohort

**Choice:** The initial profile accepts exactly `marigold-grammar` version `0.1.17` and a versioned Kamacite evaluator descriptor. Updating either requires reviewed profile and fixture changes.

**Rationale:** Analyzer semantics are pre-1.0 and may change. Exact cohort binding prevents a fresh binary from validating a stale report under the same role.

### 3. Keep Marigold out of the functional core dependency closure

**Choice:** `kamacite-core` defines owned DTOs, validation, identity construction, receipt construction, and tests without depending on Marigold. `kamacite-cli` alone depends on `marigold-grammar`, invokes `marigold_analyze`, and maps the serialized result into the owned DTO.

**Rationale:** Core logic remains deterministic and testable without parser, async runtime, filesystem, process, or network setup. Upstream types cannot silently become Kamacite's public receipt schema.

### 4. Do not execute Marigold programs

**Choice:** `analyze-marigold` only parses/analyzes source. It does not compile generated Rust, invoke Cargo, create `~/.marigold`, access pipeline-declared files, enable Tokio/async-std parallelism, or run stream effects.

**Rationale:** The useful first question is whether source-shape metadata is worth retaining. Runtime execution would add ambient authority, scheduling, and artifact-provenance questions before the analysis value is established.

### 5. Fail closed on malformed or overclaimed evidence

**Choice:** The pure core checks schema/profile/tool version, identity domains and freshness, finite report bounds, required fields, supported complexity spellings, required checked laws, required non-claims, and forbidden semantic claims. Diagnostics are sorted and deduplicated.

**Rationale:** A plausible analyzer report is not sufficient evidence. The receipt must expose exact limitations and reject role promotion.

### 6. Keep the experiment independently removable

**Choice:** The new command, dependency, fixtures, check, docs, and receipt profile form one narrow feature slice. No accepted Kamacite source language list, canonical codec, Steel/Nickel adapter behavior, or downstream gate requires Marigold.

**Rationale:** Failure to demonstrate downstream value should allow removal without migration of canonical artifacts.

## Functional core / imperative shell split

- **Pure core**: owned report DTOs, finite bounds, analyzer/profile admission, identity checks, projection hashing, non-claim/overclaim checks, deterministic diagnostic ordering, and receipt construction.
- **Imperative shell**: root-confined source read, UTF-8 conversion, upstream analyzer invocation, upstream-to-owned DTO serialization mapping, optional receipt write, and operator JSON rendering.

## Risks / Trade-offs

- The analyzer can classify recognized operators while missing expensive arbitrary functions; required non-claims and the advisory role prevent cost-proof promotion.
- Literal finite inputs can produce constant complexity classes that are mathematically scoped to that literal but unsuitable as general resource policy.
- Adding a parser dependency increases CLI closure size. Keeping it exact and shell-only bounds the impact.
- Marigold's tightly coupled multi-consumer runtime backpressure is not adopted; this pilot records analysis output only.
- JSON remains a compatibility projection. A future promoted profile would need an explicit Preserves schema/codec decision rather than inheriting JSON authority.

## Non-Goals

- No Marigold runtime, Cargo invocation, generated-Rust execution, async runtime, file/network pipeline I/O, or multi-consumer integration.
- No replacement for Nickel configuration, Steel probes, Preserves canonical values, Kamacite effects, or downstream runtime policy.
- No whole-program complexity, resource-safety, deterministic-execution, replay, compiler-correctness, semantic-equivalence, production, or release claim.
