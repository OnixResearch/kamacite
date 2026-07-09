## Why

Kamacite already preserves source envelopes, canonical Preserves values, adapter descriptors, target evidence, and deterministic receipts, but it has no reviewed documentation artifact boundary. If documentation is added as ad hoc Markdown, generated HTML, or CLI text, examples can drift from fixtures, rendered anchors can become unstable, and documentation output can accidentally imply canonical authority.

Rust's `rustdoc`/`cargo doc` pipeline is useful as a publishing architecture: collect structured facts, resolve links and anchors, then render format-specific outputs. Unison's runnable documentation model is useful as an evidence architecture: examples are structured cells that can be checked and surfaced as part of the documentation. Kamacite should combine those ideas without adopting either tool's IR, storage model, or execution semantics.

## What Changes

- Define a Kamacite-owned documentation IR artifact class for pages, sections, anchors, links, prose, code blocks, runnable cells, diagnostics, source spans, dependencies, metadata, and non-claims.
- Treat HTML, Markdown, and AsciiDoc as deterministic projections from documentation IR rather than canonical Kamacite identity.
- Define runnable documentation cells as bounded checked examples with explicit language, source-envelope identity, declared effects, expected output or diagnostic, run profile, and receipt identity.
- Require documentation rendering and runnable-cell execution receipts to bind source envelopes, canonical documentation IR, renderer identity, target format, output artifact hashes, diagnostics, checked laws, and non-claims.
- Keep default documentation generation non-executing; runnable cells execute only through an explicit command mode and declared capabilities.
- Require positive and negative fixtures before any documentation projection, runnable-cell check, or generated output claim is trusted.

## Impact

- **Files**: `cairn/changes/add-runnable-docs-pipeline/**` now; later implementation will touch Preserves schemas, Rust core validators/renderers, CLI command surfaces, fixture manifests, golden receipts, docs, README guidance, and accepted specs.
- **Testing**: Cairn validation plus proposal/design/tasks gates now; later implementation must add Rust unit tests, positive and negative fixture receipts, deterministic renderer golden outputs for HTML/Markdown/AsciiDoc, runnable-cell success and failure fixtures, default non-execution checks, and Cairn validation before documentation claims are accepted.
