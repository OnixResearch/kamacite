## Context

Kamacite currently explains its source-envelope, adapter, receipt, Valence export, algebraic-effect, and FSM boundaries in repository docs and Cairn specs. Those documents are useful, but they are not themselves validated artifacts. Examples embedded in prose can become stale, generated publication formats can drift, and documentation commands could accidentally execute Nickel, Steel, Rust, or external tools without an explicit receipt boundary.

The intended product decision is not to make Kamacite a documentation language. Kamacite needs a bounded documentation artifact model that preserves original source bytes, builds a deterministic documentation IR, renders derived publication formats, and optionally checks explicit runnable cells under denied-by-default capabilities.

## Decisions

### 1. Documentation IR is Kamacite-owned data

**Choice:** Define pages, sections, anchors, links, prose blocks, code blocks, runnable cells, diagnostics, source spans, metadata, dependencies, and non-claims as Kamacite documentation IR artifacts with Preserves-backed canonical identity and BLAKE3 hashes.

**Rationale:** A stable intermediate model lets Kamacite render HTML, Markdown, and AsciiDoc without duplicating interpretation logic. It also keeps source envelopes and canonical documentation facts separate from path names, labels, themes, and generated files.

**Rejected alternative:** Treat Markdown files, generated HTML, or AsciiDoc output as canonical. That would make lossy presentation syntax responsible for identity and would blur source preservation with publication output.

### 2. Rendered formats are deterministic projections

**Choice:** HTML, Markdown, and AsciiDoc outputs are renderer projections from documentation IR. Renderers must preserve declared anchors, links, source spans, diagnostics, runnable-cell results, and non-claims, but the rendered files do not replace the source envelope or documentation IR identity.

**Rationale:** This borrows the useful `rustdoc`/`cargo doc` separation between collection and rendering while keeping Kamacite's own identity model. It also permits multiple publication formats without changing the reviewed documentation facts.

**Rejected alternative:** Build separate emitters directly from source files. That would invite inconsistent anchors, link resolution, diagnostics, and runnable-cell result presentation across formats.

### 3. Runnable docs are checked cells, not ambient scripts

**Choice:** Runnable documentation cells are explicit artifacts that declare language or adapter class, source-envelope identity, dependency identities, expected output or expected diagnostic, declared effects, run profile, evaluator or adapter identity, and non-claims. Default documentation generation validates stored evidence only and never executes cells.

**Rationale:** This adapts Unison's useful idea that documentation examples can be checked while preserving Kamacite's denied-by-default adapter boundary. A runnable cell proves only the named example under the declared profile, not arbitrary semantic equivalence between Nickel, Rust, Steel, or Preserves.

**Rejected alternative:** Execute fenced code blocks during rendering. That would hide host authority in documentation generation and make publication depend on ambient filesystem, process, network, clock, random, credential, compiler, or evaluator state.

### 4. Documentation commands have a pure core and thin shell

**Choice:** Documentation collection, link planning, anchor assignment, renderer planning, fixture validation, and receipt construction belong in pure Rust core functions over explicit in-memory inputs. CLI shells own filesystem discovery, file reads and writes, external evaluator invocation, update mode, and output directory mutation.

**Rationale:** Functional-core boundaries make documentation behavior testable without standing up external tools or touching the filesystem. The shell can then remain boring orchestration around explicit inputs and declared effects.

**Rejected alternative:** Let renderer code read files, invoke evaluators, or mutate outputs directly. That would make determinism and negative tests fragile.

### 5. Receipts bind every documentation claim

**Choice:** Documentation receipts bind source-envelope identities, documentation IR identity, dependency closure, renderer identity, target format, output artifact identities, cell-run receipts when present, diagnostics, checked laws, and non-claims. Golden outputs are trusted only when their receipt identities match the current inputs.

**Rationale:** Kamacite's existing receipt discipline should apply to documentation too. Rendered files become reviewable artifacts with explicit identities instead of untracked side effects.

**Rejected alternative:** Trust generated files because the command succeeded. That would miss stale examples, stale anchors, renderer-version drift, and output overwrites.

### 6. Runnable-cell capabilities fail closed

**Choice:** Nickel, Steel, Rust-validator, Preserves-canonicalization, or later adapter cells may run only when the cell descriptor declares required capabilities and the selected command mode admits them. Undeclared effects, unsupported languages, missing evaluator identity, stale dependency identity, mismatched expected output, or unexpected success for an expected-failure cell fail closed with deterministic diagnostics.

**Rationale:** Documentation examples should strengthen evidence, not bypass adapter rules. Failure fixtures are as important as successful rendered pages.

**Rejected alternative:** Treat runnable docs as a developer convenience outside validation. That would let examples drift from the receipt corpus and weaken the docs as evidence.

### 7. Stack consumers keep bounded roles

**Choice:** Valence may wrap documentation and cell-run receipts as evidence; Octet may check Rust renderer and validator source shape; Mantle may package rendered docs and receipts; Cairn may gate lifecycle readiness on doc evidence; Trellis may prove narrow pure-core properties if introduced later. None of these consumers owns Kamacite's documentation IR or broadens its claims.

**Rationale:** The same explicit stack boundary used for other Kamacite artifacts prevents publication, lifecycle, build, or proof evidence from becoming a canonical source by accident.

**Rejected alternative:** Let the publication site, release bundle, lifecycle gate, or downstream evidence layer define documentation semantics. That would fragment identities and make non-claims harder to preserve.

## Risks / Trade-offs

- **Semantic overclaim:** Mitigate by stating rendered docs and runnable-cell receipts are projections/evidence only, not proof of arbitrary Nickel/Rust/Steel semantic equivalence.
- **Hidden execution:** Mitigate by making default documentation generation non-executing and requiring explicit run modes plus declared capabilities.
- **Renderer drift:** Mitigate by using one documentation IR and deterministic receipts across HTML, Markdown, and AsciiDoc outputs.
- **Fixture burden:** Mitigate by starting with a small positive/negative fixture corpus and expanding only after the receipt shape is stable.
- **Anchor instability:** Mitigate by deriving anchors from explicit IDs or canonical identity data instead of renderer-local counters.
- **Stack confusion:** Mitigate by requiring downstream consumers to preserve source documentation receipt identities and non-claims.
