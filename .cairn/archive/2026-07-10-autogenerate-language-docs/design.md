## Design

The existing documentation IR remains the canonical documentation artifact. This change inserts a bounded language documentation facts layer between source envelopes and documentation IR:

```text
Rust / Nickel / Steel source envelopes
  -> language-specific collectors
  -> language documentation facts
  -> documentation IR planner
  -> runnable-cell descriptors
  -> optional WASM runner artifacts and cell receipts
  -> HTML / Markdown / AsciiDoc projections
  -> documentation receipts and Valence-friendly exports
```

### Artifact model

Add a `language-documentation-facts` artifact class with Preserves-backed canonical identity. Each artifact records one collector run over explicit source envelopes and contains only bounded documentation facts:

- source envelope identity and source spans;
- language label: `rust`, `nickel`, or `steel`;
- collector identity and collector capability profile;
- stable item identities, symbol paths, titles, signatures or contract summaries;
- prose/doc blocks and code/example blocks;
- runnable-cell descriptors or references;
- optional WASM runner targets for executable examples;
- explicit links and dependency identities;
- diagnostics, checked laws, and non-claims.

The facts artifact is not a substitute for source identity, compiler output, evaluator output, WASM module identity, or semantic proof. It is an input to documentation IR assembly.

### Collectors

Collectors are imperative shells around pure normalizers:

- The shell reads source files, invokes optional language tooling, captures tool output, and records collector identity.
- The pure core receives in-memory source envelope records and collector DTOs, then produces deterministic facts, diagnostics, and identities.
- Missing, stale, unsupported, or overclaiming collector inputs fail closed.

Rust collection may use rustdoc JSON, rust-analyzer-compatible syntax facts, or a bounded parser, but any compiler/tooling output remains collector evidence rather than proof of Rust semantics.

Nickel collection may use Nickel source envelopes, comments, contracts, field metadata, examples, and optional reference CLI output. The collector must not reimplement Nickel evaluation semantics.

Steel collection may use module declarations, exported names, contracts, `define` forms, adapter descriptors, host-function descriptors, and examples. The collector must not treat Steel bytecode, opaque closures, or JSON strings as canonical Kamacite IR.

### Documentation IR assembly

A pure planner converts facts into existing `documentation-ir` pages and sections. It owns deterministic anchor assignment, duplicate detection, cross-language link resolution, runnable-cell reference wiring, dependency closure construction, and receipt input construction. Filesystem layout and output directory writes stay in the CLI shell.

Generated anchors use explicit item identities and configured page grouping. A generated rendered document never replaces a language facts identity, source-envelope identity, runnable-cell receipt identity, WASM runner receipt identity, or documentation IR identity.

### WASM runnable-cell execution

WASM is an execution target for runnable documentation examples, not Kamacite's canonical IR. A runnable documentation cell may declare a WASM target that binds:

- source envelope identity;
- compiled module or component identity;
- compiler, adapter, or packager identity when compilation is part of the evidence chain;
- engine and WASI/profile identity;
- declared imports and host capabilities;
- resource limits such as fuel and memory;
- expected stdout, stderr, structured output, or diagnostic identities;
- observed effects, diagnostics, checked laws, and non-claims.

`docs run-cells` may execute WASM-backed cells only in an explicit run mode. The CLI shell owns engine invocation, sandbox setup, import wiring, resource-limit enforcement, and output capture. Pure core validation receives in-memory execution records and checks identities, declared imports, observed effects, outputs, diagnostics, and non-claims before trusting the cell receipt.

Rust examples may compile to a WASI-compatible module or component through a declared compiler profile. Nickel and Steel examples may run through declared WASM adapters or packaged evaluators, but those adapters remain bounded evaluator evidence and must not claim source-language semantic equivalence. Browser-demo use is a downstream projection only; release evidence must still bind the same Kamacite receipts.

### CLI boundary

Add explicit subcommands under a documentation command group:

```text
kamacite docs collect --config docs/site.ncl
kamacite docs check --config docs/site.ncl
kamacite docs render --config docs/site.ncl --format html,markdown,asciidoc
kamacite docs run-cells --config docs/site.ncl --profile PROFILE [--target wasm]
kamacite docs update --config docs/site.ncl
```

Defaults remain non-mutating and non-executing. `docs run-cells` is the only mode that may execute runnable cells, and it may run only cells with declared capabilities, evaluator identities, admitted profiles, and declared execution targets such as WASM. `docs update` is the only mode that writes generated artifacts or golden receipts, and it must emit an update receipt.

### Nickel configuration

Docs configuration is human-authored Nickel with typed contracts, then exported to deterministic runtime input for the CLI. The config declares source roots, language labels, collector profiles, output formats, page grouping, link policy, run profile, WASM runner profiles, output directories, and update policy. JSON/TOML/YAML are compatibility outputs only if a downstream tool requires them.

### Validation

Implementation admission requires:

- Preserves schemas for language documentation facts, collection receipts, WASM runner profiles, and WASM cell execution receipts;
- pure Rust validators/planners for facts, WASM execution records, and documentation IR assembly;
- CLI shells for collect/check/render/run-cells/update;
- positive and negative fixtures for each language, WASM-backed examples, and cross-language link behavior;
- generated projection fixtures and documentation receipts;
- lifecycle validation, cargo tests, and manifest validation.
