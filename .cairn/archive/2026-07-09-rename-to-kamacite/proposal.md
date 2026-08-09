## Why

The project name should be Kamacite before the Rust crates, receipt schema IDs, requirement namespaces, and documentation become more widely consumed. Renaming now keeps the source-envelope, validation harness, and algebraic-effect evidence boundary under one project identity.

## What Changes

- Rename user-facing documentation, crate names, CLI binary, receipt schema IDs, Preserves schema IDs, requirement namespaces, fixture metadata, and golden receipts to Kamacite/`kamacite`.
- Preserve the same product scope: lossless interchange boundaries, source envelopes, Preserves-backed canonical identity, Rust validation core, bounded Steel/Nickel/WASM/LLVM adapter evidence, algebraic-effect data artifacts, and explicit non-claims.
- Keep the checkout path outside lifecycle authority; repository-directory moves can happen separately from the content rename.

## Impact

- **Files**: README, AGENTS, Cairn specs/change archive, Rust crate/package names, fixture manifests/goldens, docs, schemas, and source code identifiers.
- **Testing**: Cargo format/tests/Clippy, default non-mutating fixture validation, Cairn validation, proposal/design/tasks gates, sync dry-run/execute, and archive dry-run/execute.
