## Context

The repository has accepted specs, archived changes, Rust crates, fixture manifests, golden receipts, and Preserves schema artifacts under a pre-rename project identity. The user now wants the project named Kamacite. This is primarily an identity and naming change, not a semantic expansion.

## Decisions

### 1. Kamacite is the project identity

**Choice:** Rename the project to Kamacite and use `kamacite` as the machine namespace for requirement IDs, schema IDs, crate names, command names, receipt schema versions, and fixture metadata.

**Rationale:** One consistent name prevents future evidence consumers from mixing pre-rename and Kamacite identities or treating the rename as a semantic fork.

### 2. Scope remains unchanged

**Choice:** Keep all existing claim boundaries: Kamacite is an interchange IR/source-envelope/receipt contract/constrained policy vocabulary, not a programming language, evaluator, runtime, compiler, verifier, authorization system, or semantic-equivalence proof.

**Rationale:** The rename should not broaden accepted claims or reset the current evidence boundary.

### 3. Checkout path is not lifecycle identity

**Choice:** Update content and package identity now, while not requiring the local checkout directory to be moved as part of Cairn lifecycle mutation.

**Rationale:** Filesystem repo moves are workspace operations outside the product evidence boundary. Documentation should avoid hard-coding the old checkout path where possible.

## Risks / Trade-offs

- **Receipt identity churn:** Mitigated by regenerating checked golden receipts through the explicit update workflow.
- **Namespace drift:** Mitigated by grep checks for stale pre-rename or legacy project references after the rename.
- **External path mismatch:** Mitigated by documenting that the content rename is complete while the local checkout path can be renamed separately if desired.
