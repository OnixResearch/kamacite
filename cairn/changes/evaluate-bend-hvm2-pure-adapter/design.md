## Context

Kamacite separates source envelopes, canonical Preserves values, adapter descriptors, target artifacts, and receipts. A Bend/HVM2 pilot must use these layers instead of adding Bend or HVM as canonical IR.

The reviewed upstream cohort is:

- Bend 0.2.38, Apache-2.0, source revision `814453670d0e0d6777c1313c972764dba0491b7f`.
- HVM2 2.0.22, Apache-2.0, source revision `654276018084b8f44a22b562dd68ab18583bfb5b`.
- Bend declares the exact Cargo dependency `hvm = "=2.0.22"`.

The first pilot excludes CUDA, HVM3, HVM4, Kind, network access, runtime I/O, and production consumer integration. HVM4 remains pre-launch and had no repository license file during the review.

## Decisions

### 1. The pilot is evaluation-only

**Choice:** Add one non-default Kamacite adapter-evaluation profile. A successful run can produce evidence, but it cannot make Bend or HVM2 a production runtime dependency.

**Rationale:** The toolchain has promising parallel semantics and material maturity limits. Evidence must precede adoption.

### 2. The cohort is exact and CPU-only

**Choice:** Pin the reviewed Bend and HVM2 revisions, package versions, licenses, host compiler, target triple, and command profiles. Run `bend run-rs` as the Bend reference route. Run `bend gen-c`, compile the emitted C with the pinned host compiler, and run the resulting executable as the optimized Bend route.

**Rationale:** Exact inputs make the result reviewable. CPU execution avoids the NVIDIA-only CUDA support and its separate stability surface.

### 3. Every artifact layer keeps its identity

**Choice:** Give Bend source, source envelope, evaluator profile, generated C, host-compiler invocation, executable, declared input, observed output, run observation, decision, and receipt separate typed identities. Use BLAKE3 over Kamacite-owned canonical bytes.

**Rationale:** Generated artifacts and matching outputs do not inherit source losslessness or semantic correctness.

### 4. The first workload is pure and deliberately narrow

**Choice:** Admit one finite recursive tree reduction or bounded state-search workload. Require public type annotations, reject explicit `Any`, use only declared `u24` modulo semantics, and expose no I/O, host functions, filesystem, process, network, clock, random, credential, mutation, or device authority to the Bend program.

**Rationale:** This shape exercises Bend's intended parallel model without importing an effect runtime or hiding number semantics.

### 5. An independent optimized Rust path is the oracle

**Choice:** Implement the same bounded workload in a pure Rust oracle that does not call Bend or HVM2. Compare canonical results from the Rust oracle, `bend run-rs`, and the generated-C executable. Any disagreement denies the passing receipt.

**Rationale:** The two Bend routes share a frontend. Their agreement alone cannot detect a shared frontend error.

### 6. Planning and decisions stay pure

**Choice:** The Kamacite core validates cohort facts, profile limits, source metadata, declared numeric semantics, artifact links, explicit observations, backend equality, benchmark policy, decision state, and receipts. The core returns ordered diagnostics or one deterministic decision from supplied facts.

**Rationale:** File access, process execution, clocks, and measurement remain outside the functional core.

### 7. The shell reuses bounded process controls

**Choice:** The CLI shell reads reviewed files, invokes the pinned tools, applies the existing cleared-environment and resource-limit profile, captures bounded output, and records explicit observations. It does not grant network access or treat process limits as sandbox proof.

**Rationale:** The pilot needs real execution without adding ambient authority or a second hidden process policy.

### 8. Measurements are version-bound evidence

**Choice:** Record interaction counts, wall duration, peak-memory observations when available, output size, exit class, and repeated-run summaries. A typed Nickel policy declares repetitions, warm-up handling, summary statistic, variance limit, resource limits, and the performance target against optimized Rust.

**Rationale:** Interaction counts and timing can change with source, compiler, backend, host, and version. They are evidence for one cohort, not universal units.

### 9. The final result is an explicit decision

**Choice:** The pilot ends with `adopt`, `defer`, or `reject`. `Adopt` requires backend equality, positive and negative evidence, resource compliance, and the declared performance target. `Defer` requires an exact blocker. `Reject` records the failed criterion without inventing success.

**Rationale:** A completed evaluation does not require a favorable result.

### 10. Consumer work requires separate authority

**Choice:** This change does not edit Lattice, Animus, ChaosControl, Trellis, Choregraph, Aspen, or another consumer. A passing decision can become input to a separate consumer-owned Cairn change.

**Rationale:** Kamacite owns adapter and evidence boundaries. Each consumer retains runtime, policy, effect, and release authority.

## Functional core / imperative shell split

- **Pure core**: cohort and license facts, profile admission, source and artifact identity checks, workload and numeric-policy admission, observation comparison, benchmark decision, receipt construction and validation, and Valence projection DTOs.
- **Imperative shell**: file reads, Nix-provided tool discovery, Bend invocation, C compilation, executable invocation, resource controls, observation capture, and confined receipt writes.

## Risks / Trade-offs

- Bend's type system does not make the fully annotated profile a type-safety proof.
- The Rust oracle can duplicate a specification error. Positive and negative fixtures must target independent boundary cases.
- Benchmark noise can produce unstable decisions. The policy must bind repetition and variance rules before execution.
- Generated C can vary with the host compiler. The receipt must bind compiler and target identities.
- Interaction counts can drift between toolchain revisions. Cross-version comparisons remain unsupported.
- Process limits and namespace observations do not prove complete sandboxing.
- Apache-2.0 permits the pilot, but packaged notices and source attribution remain required.

## Promotion boundary

Kamacite can complete and archive this change after it records a validated decision and evidence bundle. Only an `adopt` decision can support a later consumer proposal. That later proposal must rerun the relevant workload and retain all Kamacite non-claims.
