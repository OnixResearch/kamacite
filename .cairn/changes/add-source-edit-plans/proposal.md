## Why

Kamacite preserves original source bytes, parser identity, and BLAKE3 source identity. It has no canonical plan for a narrow source update.

A whole-tree renderer can change comments, quoting, line endings, or trailing whitespace outside the requested edit. That behavior would weaken source-envelope claims.

Jane Street `Sexp.With_layout` shows how syntax trees can retain comments, positions, and original quoted forms. Its tests normalize some whitespace, so Kamacite needs a stricter byte-patch design.

## What Changes

- Add a pure source-edit component with bounded, ordered byte patches.
- Bind each plan to the exact source envelope and parser identity.
- Bind each patch to its original range BLAKE3 and replacement identity.
- Preserve every untouched byte exactly.
- Make no-op edits preserve the original source identity.
- Add syntax-only Nickel and Steel pilot adapters.
- Publish changed source through the shared Durable File Publication component.
- Emit a new source envelope and bounded edit receipt after a successful edit.

## Impact

- **Files**: `crates/kamacite-source-edit/`, workspace manifests, `schemas/preserves/`, `fixtures/`, Nickel and Steel adapters, documentation, and this lifecycle package.
- **Local dependency**: `add-located-value-documents` supplies structural paths and source-origin contracts.
- **Shared component**: pin `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM` revision `8e05e74e24b45f752d77145c4455385daaf6d6ab` for one-file publication.
- **Reference**: MIT-licensed `sexplib` revision `e5a4df62c104c8f7c2e8a11ce36e88808649fce4` informs layout retention only.
- **Testing**: baseline and final workspace tests, byte-preservation properties, positive and negative adapter fixtures, publication failures, Cairn gates, Clippy, Octet, and `nix flake check -L`.

## Non-goals

- Do not add a general formatter or pretty-printer.
- Do not claim whole-program semantic preservation.
- Do not execute Nickel or Steel in the source-edit core.
- Do not make syntax parsers evaluation authorities.
- Do not add multi-file transaction, rollback, retention, or deletion authority.
