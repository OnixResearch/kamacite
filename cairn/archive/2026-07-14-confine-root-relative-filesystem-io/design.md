## Context

Lexical path filtering does not constrain filesystem resolution. Symlink components, symlinked destination parents, and replacement between check and open can escape the root. Some write paths bypass the helper entirely.

## Decisions

### 1. Root capability, not joined paths

**Choice:** Open the reviewed root once as a directory capability and resolve child operations relative to it with no-follow semantics. Keep a pure component-policy core and a thin handle-based shell.

**Rationale:** Security depends on the object opened, not on a pre-open string check.

### 2. One gateway for every root-scoped operation

**Choice:** Manifest reads, fixture reads, evaluator source reads, goldens, receipts, Valence outputs, docs outputs, and update reports use the same read/create/replace APIs.

**Rationale:** A single direct `root.join` reintroduces escape authority.

### 3. Mutation uses safe creation and replacement

**Choice:** Create temporary output in the admitted destination directory, verify destination policy through the directory capability, flush as required by policy, and rename within that directory. Reject symlink destinations and parents.

**Rationale:** Mutation needs confinement and crash consistency, not just lexical admission.

### 4. Diagnostics preserve attempted relative paths without exposing ambient paths

**Choice:** Return deterministic path-policy diagnostics that identify the argument and rejected component/class.

**Rationale:** Operators need actionable failures without turning receipts into host-path disclosures.

## Risks / Trade-offs

- Capability-safe APIs may differ across Unix and non-Unix targets; unsupported guarantees must fail closed.
- Existing workflows that intentionally write outside `--root` need an explicit separate output authority rather than an escape path.
- Race tests require careful platform isolation to remain deterministic.
