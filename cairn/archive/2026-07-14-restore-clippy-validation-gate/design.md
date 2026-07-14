## Context

The crate denies panic-oriented Clippy lints, but production canonicalization helpers call `expect`. The all-target gate also evaluates test code under those denies. The flake defines a Clippy check, yet the current source cannot satisfy it.

## Decisions

### 1. Canonicalization errors are values

**Choice:** Thread typed serialization/canonicalization errors through public fallible APIs. For genuinely infallible encodings, implement an infallible byte builder that cannot return a serialization error rather than asserting success.

**Rationale:** A pure core promised total diagnostics and should not panic on a nominally impossible library result.

### 2. Tests follow the same no-hidden-panic posture

**Choice:** Replace test `expect`/panic helpers with explicit matches and assertions that preserve useful failure messages without weakening crate lint levels.

**Rationale:** `--all-targets` is part of the advertised gate, and tests should model accepted error handling.

### 3. Fix warnings, do not allow them globally

**Choice:** Resolve `double_must_use`, `collapsible_if`, and subsequent warnings structurally. Narrow local allows require written justification and are not the default.

**Rationale:** Suppressing the gate would hide future regressions.

### 4. Keep command parity

**Choice:** Document and execute one canonical workspace/all-target Clippy command in local validation and Nix checks.

**Rationale:** Contributors and release checks must observe the same result.

## Risks / Trade-offs

- Making helper APIs fallible can propagate signature changes through receipt constructors.
- Golden behavior must remain unchanged except where error handling exposes previously hidden failure states.
- Fixing current errors may reveal additional warnings that must also be resolved before completion.
