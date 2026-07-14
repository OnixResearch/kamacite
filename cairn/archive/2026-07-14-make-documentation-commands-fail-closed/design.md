## Context

The CLI has documentation action names and core documentation helpers, but `run_docs` only reads and hashes raw config bytes. It marks admitted-but-unimplemented execution and mutation messages as passing.

## Decisions

### 1. Success is action completion

**Choice:** Each action returns a typed outcome whose success predicate names required completed work. Admission, planning, or a no-op cannot satisfy execution or mutation actions.

**Rationale:** Exit status must be trustworthy automation evidence.

### 2. Nickel config is evaluated before dispatch

**Choice:** The shell obtains a checked typed config projection and passes it to a pure action planner. Malformed source, contract violations, unsupported schema versions, and stale projections fail before action work.

**Rationale:** File extension and readability are not configuration validation.

### 3. Reuse the functional core

**Choice:** Collection normalization, IR assembly, link checks, render planning, output comparison, and receipt construction remain pure. The CLI performs declared reads, evaluator/runner calls, and writes from an explicit plan.

**Rationale:** Existing core logic should become the testable source of command behavior.

### 4. Execution and update remain separately gated

**Choice:** `run-cells` requires a supported runner profile and actual bounded execution. `update` requires an explicit mutation plan and emits old/new identity evidence. Unsupported profiles fail closed.

**Rationale:** Capability admission is not proof that an action occurred.

## Risks / Trade-offs

- Some current commands will begin failing until their actual backend is implemented.
- Config projection generation must coordinate with the manifest freshness fix.
- Renderer output changes require explicit golden migration rather than silent rewrites.
