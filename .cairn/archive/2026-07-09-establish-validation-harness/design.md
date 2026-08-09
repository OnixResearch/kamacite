## Context

Kamacite currently has decision Cairns for the lossless IR boundary, algebraic-effect artifact model, and Rust deterministic-admission profile. Those Cairns require fixture coverage and deterministic receipts, but the repo still has no concrete harness, no Rust workspace, no fixture manifest, and no seed corpus. If implementation starts before the harness exists, the first validators may accidentally define test shape, receipt ordering, manifest fields, or fixture update behavior through convenience rather than reviewed lifecycle decisions.

The harness should be intentionally small. It should not try to implement every adapter, target, or export path up front. Instead it should make the evidence contract executable with a few source-envelope and identity-domain cases, then let later implementation slices add fixtures under the same contract.

## Decisions

### 1. Build a walking skeleton before trusting implementation behavior

**Choice:** Kamacite introduces the validation harness before the first trusted Rust core behavior is accepted. The initial harness must run at least one positive source-envelope case, one stale-hash negative case, one wrong-identity-domain negative case, and one deterministic golden receipt check.

**Rationale:** A small executable contract is better than a large speculative harness. These cases exercise Kamacite's core risks: source identity, fail-closed hashing, typed identity separation, and deterministic evidence.

**Rejected alternative:** Implement validators first and backfill tests later. That would let implementation convenience shape the evidence boundary.

### 2. Keep pure core fixtures separate from adapter/probe fixtures

**Choice:** Fixture layout separates pure in-memory core validation fixtures from adapter/probe fixtures that may eventually involve Nickel, Steel, WASM, compilers, or evidence exports.

**Rationale:** The first trusted claims should be testable without evaluators, process execution, network access, clocks, random sources, credentials, or mutation. Adapter execution can be added later behind declared capabilities and receipt-only modes.

**Rejected alternative:** Put all fixtures in one flat directory and rely on names to imply execution mode. That would make it too easy for effectful behavior to become part of the core gate.

### 3. Use Nickel for the human-authored manifest

**Choice:** The fixture manifest is human-authored in Nickel with a typed contract. Runtime test code may consume a deterministic exported projection, but the reviewed manifest source remains the Nickel file.

**Rationale:** Kamacite already prefers Nickel for human-authored contracts/config. The manifest is policy-like metadata: fixture IDs, artifact classes, expected verdicts, expected diagnostics, non-claims, dependencies, requirement coverage, and update behavior.

**Rejected alternative:** Start with untyped JSON or ad hoc file discovery as the manifest authority. That would weaken review and make duplicate or incomplete metadata easier to miss.

### 4. Use Rust as pure core plus thin runner shell

**Choice:** The minimal Rust workspace must separate pure validation core from the fixture runner shell. The shell discovers files, reads bytes, loads the exported manifest projection, and renders test failures. The core receives normalized DTOs and bytes and returns deterministic results, diagnostics, and receipt data.

**Rationale:** This is the smallest implementation shape that satisfies the Rust deterministic-admission profile while still making the fixture contract executable.

**Rejected alternative:** Put filesystem traversal, manifest parsing, validation, and receipt construction in one test helper. That would hide nondeterminism and make the core hard to unit test.

### 5. Golden receipts are checked, not rewritten by default

**Choice:** The walking-skeleton runner compares emitted receipts to checked-in golden receipts and fails on mismatch by default. Any future update workflow must be explicit and must record changed fixture and receipt identities.

**Rationale:** Golden receipts are evidence, not disposable snapshots. Default no-rewrite behavior protects deterministic identity and makes receipt changes reviewable.

**Rejected alternative:** Auto-update golden receipts during normal validation. That would turn evidence drift into a formatter side effect.

### 6. Full adapter suites are deferred

**Choice:** Nickel, Steel, WASM, LLVM, Valence, Octet, and algebraic-effect suites remain out of the walking skeleton except as preserved-source or explicit non-claim fixtures if needed.

**Rationale:** The initial harness proves the contract; it should not block on external evaluator availability or broad integration design. Adapter-specific suites can land after the harness is stable.

**Rejected alternative:** Build the entire adapter matrix before any Rust implementation. That would delay the first feedback loop and increase the chance of speculative fixture structure.

## Risks / Trade-offs

- **Harness too small to prove adapter behavior:** Mitigate by treating it as an admission skeleton, not adapter conformance. Adapter-specific fixtures still land with their implementation slices.
- **Manifest projection drift:** Mitigate by keeping Nickel as the reviewed source and requiring deterministic export freshness or checked-in generated projection evidence before runtime use.
- **Golden receipt churn:** Mitigate with stable ordering, stable diagnostic codes, and explicit update receipts.
- **Duplicate requirements with existing Cairns:** Mitigate by making this change about execution order and harness structure while existing lossless-IR and Rust-determinism Cairns keep owning the broader semantics.
