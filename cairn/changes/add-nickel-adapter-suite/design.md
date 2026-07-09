## Context

Kamacite currently validates source envelopes, canonical values, synthetic adapter receipts, target/provenance artifacts, stack evidence, capability-shell cases, and algebraic-effect artifacts. It intentionally deferred executable adapter suites until the core and manifest authority were stable.

The previous manifest-authority change made `fixtures/manifest.ncl` the full reviewed source for the fixture corpus and made `fixtures/manifest.json` freshness-checked runtime input. That gives the Nickel adapter suite a stable admission path: new adapter fixtures can be reviewed in Nickel source, projected to JSON, and validated by the shell before receipts are trusted.

The suite must not turn Kamacite into a Nickel evaluator or reimplement Nickel semantics. The Nickel CLI or reference crates own evaluation/export semantics. Kamacite owns source preservation, projection classification, adapter descriptors, typed identities, deterministic diagnostics, golden receipts, and explicit non-claims.

## Decisions

### 1. Execute Nickel only in the shell

**Choice:** Nickel execution, if needed for a fixture, happens in `kamacite-cli` or a dedicated adapter-runner shell under an explicit Nickel-capable validation command. The core receives source bytes, export bytes, evaluator identity, projection identity, adapter descriptor facts, diagnostics, and receipt DTOs.

**Rationale:** This keeps `kamacite-core` pure and unit-testable. The shell may read fixture files and invoke a pinned Nickel toolchain, but it must normalize observations before core validation.

### 2. Bind evaluator identity but do not infer authority from it

**Choice:** Every trusted Nickel export receipt must bind the Nickel evaluator/exporter identity, including enough version/profile data for review. Evaluator identity is evidence, not a capability grant and not a semantic-equivalence proof.

**Rationale:** Reproducibility and provenance require knowing which tool produced an export. That fact must not broaden the claim beyond source-envelope preservation and derived projection receipt validation.

### 3. Keep source and export identities separate

**Choice:** A Nickel fixture records at least a source identity and an export projection identity. If a projection is adapted into Kamacite canonical Preserves bytes, that canonical identity remains a third domain. JSON export bytes never become source identity or canonical IR.

**Rationale:** This is the central Kamacite claim boundary for Nickel: source bytes are preserved, exports are derived evidence, and canonical values are Preserves-backed when present.

### 4. Add negative fixtures before trusting execution

**Choice:** The implementation must include negative fixtures for missing source-envelope evidence, stale export projection identity, JSON export as canonical-IR overclaim, missing evaluator identity, unsupported evaluator identity, malformed export output, and undeclared host effects.

**Rationale:** The first executable source-language suite should prove fail-closed behavior, not just happy-path export. These negative cases are the most likely ways a Nickel adapter could overclaim.

### 5. Require explicit execution modes and capabilities

**Choice:** Adapter fixtures that invoke Nickel must declare their execution mode, input paths, output artifact role, and required host capabilities. File access stays root-relative and deterministic; undeclared network, clock, random, credential, package-manager, mutation, and process behavior outside the declared Nickel invocation fails closed.

**Rationale:** Shell execution is still host authority. A deterministic receipt is only trustworthy if the execution boundary and allowed effects are explicit.

### 6. Golden receipts remain non-mutating by default

**Choice:** Default validation compares Nickel adapter receipts against checked golden receipts and does not rewrite evidence. Any update workflow must name changed fixtures, old/new source/export/receipt identities, covered requirements, and non-claims.

**Rationale:** Adapter execution output drift is evidence drift. It must be reviewed rather than silently accepted.

### 7. Keep parser/span evidence separate from export evidence

**Choice:** Tree-sitter Nickel or other parser/span fixtures may share source envelopes, but parser evidence stays syntax/span evidence and does not prove Nickel evaluation or export semantics.

**Rationale:** Syntax parsing and evaluation are different authorities. Mixing them would blur Kamacite's non-claims.

## Risks / Trade-offs

- The suite may require adding a Nickel-capable Nix/dev-shell path before it can run everywhere. Until that exists, executable Nickel behavior remains untrusted outside the explicit suite command.
- Binding exact export bytes can make harmless formatting/tool-version changes visible as receipt drift. This is intentional and should be handled through explicit update evidence.
- Adding broad Nickel payload contracts in the first slice could overfit to current fixture shapes. Start with the smallest typed DTOs needed for source/export/evaluator/projection receipt validation and grow contracts with later fixtures.
