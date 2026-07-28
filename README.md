# Kamacite

Kamacite defines lossless interchange boundaries for Nickel, Rust, and Steel Scheme.

The name refers to a nickel and iron alloy. The project uses that name for a stable boundary between Nickel and Steel data.

Kamacite is not a programming language. It currently provides these surfaces:

- an interchange IR
- a source-envelope format
- receipt contracts
- constrained policy vocabularies

A future human-authored syntax can be a DSL only after it defines a grammar, evaluator, type rules, and semantics.

## Core model

Preserves is the canonical encoded IR for values and source envelopes. Plain JSON cannot preserve enough identity for this boundary.

Nickel exports, Rust Serde DTOs, and Steel data can project through Kamacite IR. When source fidelity matters, the IR retains source bytes and hashes.

### Losslessness classes

| Class | Claim | Boundary |
|---|---|---|
| Source envelope | Preserves original Nickel, Rust, or Steel bytes with BLAKE3 identity and parser metadata | Does not claim arbitrary semantic equivalence |
| Canonical value | Round-trips one Kamacite IR value through Preserves bytes and adapters | Preserves bytes are authoritative, and JSON is a compatibility output |
| Constrained policy DSL | Renders a schema-bound policy to adapters and returns the same IR | Applies only to the constrained schema |
| Unsupported source translation | Translates arbitrary host-language source | Out of scope unless the source remains an envelope with explicit non-claims |

## Canonical authority

Kamacite uses Preserves as its canonical IR and encoding authority. It uses MLIR only as an architecture reference.

Kamacite adapts these MLIR ideas:

- dialect-like vocabularies for artifact and adapter classes
- deterministic verifier-style checks
- schema-bound attributes and types
- source-envelope spans as locations
- receipt-producing transformations as passes

The layers have separate roles:

- **Preserves:** canonical value, source-envelope, and receipt identity
- **MLIR:** architecture reference only
- **Valence Evidence IR:** downstream evidence projection
- **Unison:** content-identity design reference
- **Kamacite pipeline:** envelopes, values, adapters, targets, and receipts
- **Compiler artifacts:** target and provenance evidence only

Compiler artifacts include LLVM IR, WASM, GHC Core, STG, Rust MIR, bytecode, object files, and reports.

Human-authored source remains in source envelopes. Preserves owns canonical machine identity.

Adapter steps produce receipts. Compiler outputs remain target artifacts with explicit non-claims.

## Canonical Preserves codec

Rust-owned values and receipts use the official `preserves` crate. They do not use labeled JSON as canonical data.

The canonical codec is:

```text
kamacite-preserves-packed-canonical-v2-preserves-rs-5.0.0-rc.7
```

Supported text and packed syntaxes decode through a bounded explicit-frame core. Kamacite computes identity from canonical packed bytes.

Typed checks and canonical identity use the same value. Fixture checks reject these inputs:

- malformed data
- unsupported root schemas
- noncanonical packed data
- excessive depth, collection, atom, or value sizes
- canonical bytes that disagree with the typed artifact

Legacy prefixed-JSON identities remain compatibility evidence. Kamacite never reinterprets them as Preserves identities.

Migration evidence is in these files:

- `fixtures/preserves/migration-v1-real-codec.json`
- `fixtures/preserves/golden-update-v1-real-codec.json`

`migrate-preserves` uses dry-run mode by default. `--update` runs bounded convergence passes after canonical dependency identities change.

## Functional design

Kamacite uses a pure functional core and thin effect shells. Shells own I/O, evaluators, compilers, clocks, randomness, credentials, and presentation.

The design uses these rules:

- model artifact classes as explicit alternatives
- represent effects and capabilities as data
- give each identity domain a separate type
- require positive, negative, and property fixtures
- return complete diagnostics instead of panics or silent defaults

Kamacite does not adopt Haskell source, GHC artifacts, lazy evaluation, implicit typeclass resolution, or monad terminology as IR authority.

## Algebraic effects

Kamacite stores algebraic effects as canonical data and receipts. It does not provide an effect runtime.

Owned data includes these items:

- effect signatures and operation identities
- parameter and result schemas
- replay classes
- effect rows and summaries
- handler descriptors and checked laws
- continuation and replay policy
- handler receipts and effect-log projections

An operation declaration does not grant host authority. Every host capability requires separate adapter admission.

Undeclared capabilities fail closed. This rule covers files, processes, networks, clocks, randomness, devices, credentials, compilers, evaluators, and mutation.

Aspen or Molten can consume effect descriptors and logs. Those runtimes do not own the canonical Kamacite effect model.

## Target boundaries

### WebAssembly

WebAssembly is an optional adapter and validator target. It is not the canonical IR.

A WASM adapter accepts Kamacite IR or source-envelope bytes. It runs with declared capabilities and returns a deterministic Kamacite receipt.

WIT components and SpaceWasm core modules are separate fail-closed profiles. Neither profile can fall back to the other.

Their ABIs do not replace Preserves schemas.

### LLVM

LLVM is a compiler target and provenance artifact. It is not a source-preserving interchange format.

An LLVM receipt binds these facts:

- compiler version
- target triple
- optimization level
- pass pipeline
- debug-information availability
- BLAKE3 artifact hashes

The receipt does not prove source losslessness or semantic equivalence. LLVM toolchain and target behavior prevent that broader claim.

## Evidence boundaries

Valence can wrap Kamacite receipts as Evidence IR. Valence owns stack evidence identity and linkage.

Kamacite retains source preservation, value interchange, adapter classes, and round-trip claims.

Octet checks Kamacite Rust code and collects provenance. Octet does not define the IR or prove cross-language equivalence.

Initial integration uses exported receipts and fixtures. Direct crate dependencies require stable receipt schemas and tested API needs.

## Nickel integration

Kamacite treats Nickel as a source language and adapter. It does not reimplement Nickel semantics.

The integration follows these rules:

- use the Nickel CLI or reference crates for evaluation fixtures
- preserve `.ncl` bytes, parser identity, spans, and BLAKE3 hashes
- treat `nickel export` output as a derived projection
- generate contracts only for constrained policy vocabularies
- use tree-sitter Nickel for syntax and spans, not evaluation
- use generated contract corpora as stress fixtures, not schema authority

## Steel integration

Kamacite treats Steel as a bounded adapter and probe language. Steel is not the canonical IR or an ambient scripting environment.

The integration can use these patterns:

- controlled `Engine` fixtures with evaluator identity
- typed Rust host-function registration
- declared built-in modules
- `provide`, `require`, and contract exports
- descriptor-bound and hash-bound higher-order contracts

By default, Steel receives no file, process, TCP, random, clock, package, or dynamic-library authority.

Steel bytecode and opaque closures are not portable Kamacite values. JSON remains a compatibility projection.

A Steel adapter consumes declared values or canonical bytes. It emits a deterministic receipt.

## Identity discipline

Canonical content owns identity. Names, paths, aliases, and labels are metadata.

Kamacite uses BLAKE3 over its own canonical bytes. It does not adopt another language hash format or codebase database.

Receipts bind dependency hashes and adapter identities. They also bind evaluator identities when a projection depends on evaluation.

Schema and name migrations use dry-run-first plans over identities. Persisted values bind their schema, codec, hash, and non-claims.

Kamacite does not promise distributed code execution or serialization of arbitrary functions.

## Current implementation

Kamacite has a validation harness and a Rust core profile.

### `crates/kamacite-core/`

The pure core provides these functions:

- typed identities and source-envelope checks
- canonical, target, and adapter boundary checks
- admitted Nickel JSON-to-Preserves mapping
- fail-closed WIT and SpaceWasm planning
- Mantle component-bundle handoff checks
- Steel DTO adapter checks
- algebraic-effect and FSM artifact checks
- documentation artifact checks
- deterministic diagnostics and receipts

### `crates/kamacite-cli/`

The shell provides manifest and file I/O, evaluator execution, fixture materialization, receipt comparison, and explicit update workflows.

On Linux, each `--root` uses one `openat2` directory capability. Child access stays beneath that root and rejects symlinks.

Mutation uses handle-relative atomic replacement. Unsupported platforms fail closed.

### `schemas/preserves/`

This directory contains Kamacite-owned schemas for source envelopes, values, adapters, targets, receipts, effects, FSM artifacts, and documentation artifacts.

It also contains schemas for function addresses, evidence chains, Trellis profiles, Valence exports, WIT, SpaceWasm, Steel, and WASM documentation artifacts.

### `fixtures/`

This directory contains Nickel sources, contracts, runtime projections, generation receipts, positive fixtures, and negative fixtures.

It also contains Preserves outputs, Steel adapter data, Valence exports, FSM artifacts, documentation data, and checked golden receipts.

### Documentation

Read these documents for the implemented boundaries:

- `docs/validation-harness.md`
- `docs/rust-core-profile.md`
- `docs/nickel-preserves-adapter.md`
- `docs/wasm-component-adapter.md`
- `docs/spacewasm-mvp-adapter.md`
- `docs/deferred-adapter-suites.md`
- `docs/function-address-receipts.md`
- `docs/valence-receipt-export.md`
- `docs/algebraic-effects-stack-roles.md`
- `docs/fsm-spec-receipts.md`
- `docs/runnable-docs-pipeline.md`
- `docs/autogenerated-docs-pipeline.md`

## Checks

Run checks through the repository package and development shell. The Clippy gate adds Octet's selected canonical-determinism restrictions.

It does not enable the full `clippy::restriction` group.

```bash
nix develop . -c cargo test --workspace
nix develop . -c cargo clippy --workspace --all-targets -- -D warnings -D clippy::dbg_macro -D clippy::mem_forget -D clippy::undocumented_unsafe_blocks -D clippy::multiple_unsafe_ops_per_block -D clippy::host_endian_bytes -D clippy::iter_over_hash_type
nix run .#kamacite -- validate --root . --manifest fixtures/manifest.json
nix run .#kamacite -- generate-manifest --root .
nix run .#kamacite -- generate-manifest --root . --update
nix run .#kamacite -- validate-nickel-adapter --root . --manifest fixtures/manifest.json
nix run .#kamacite -- validate-steel-adapter --root . --manifest fixtures/manifest.json
nix run .#kamacite -- adapt nickel-export --root . --manifest fixtures/nickel-export/service-config/generated/manifest.json --export examples/service-config/generated/service.json --artifact fixtures/nickel-export/service-config/generated/service.json --packed-output fixtures/nickel-export/service-config/generated/service.preserves --text-output fixtures/nickel-export/service-config/generated/service.preserves.txt --receipt fixtures/nickel-export/service-config/generated/service.kamacite-receipt.json --check
ast-grep test --config audits/ast-grep/sgconfig.yml
ast-grep scan --rule audits/ast-grep/rules/core-ambient-process.yml --json=compact crates/kamacite-core/src
nix run .#kamacite -- migrate-preserves --root . --manifest fixtures/manifest.json --manifest-source fixtures/manifest.ncl
nix run .#kamacite -- export-valence-receipt --root . --receipt fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json --conformance fixtures/valence-export/conformance.json --golden fixtures/valence-export/golden/source-envelope.valid.nickel-minimal.valence-export.json
nix run .#kamacite -- docs check --root .
nix run .#kamacite -- docs render --root . --format markdown
nix flake check
# Default validation is receipt-only: it binds the exact source, closed contract, evaluator profile, and complete projection bytes before fixture IO or golden trust.
# generate-manifest is the explicit live Nickel check; only --update rewrites the JSON projection and its checked generation receipt.
# The validate-nickel-adapter command runs explicitly declared reference exports under the receipt-bound Linux Bubblewrap/prlimit profile; the Nix package supplies the required tools and Nickel runtime closure.
# The validate-steel-adapter command runs an exact reviewed DTO cohort in a fresh raw no-kernel Steel engine with only its declared empty base module and pure host function; default validation keeps prerecorded Steel outputs explicitly non-fresh.
# The adapt nickel-export command admits pinned nickel-export receipt evidence without reevaluating Nickel, then checks or explicitly writes authoritative packed Preserves bytes and a bounded receipt.
# The migrate-preserves command is dry-run-only unless --update is passed; update reports default under target/.
# The export-valence-receipt command validates deterministic Valence-friendly projections without invoking live Valence APIs.
# Documentation actions evaluate their checked Nickel config and emit completion receipts; run-cells and non-dry-run update fail until actual execution or mutation evidence exists.
# Every --root child path rejects absolute, parent, and symlink traversal; mutation uses confined temporary-file replacement.
# ast-grep audit receipts bind syntax-only rules, parser identities, fixture coverage, spans, dry-run codemods, and boundary-role Valence projections; they do not prove adapter semantics.
# The Clippy restriction profile is compiler source-shape evidence only; passing does not prove canonical semantics, runtime determinism, panic freedom, memory safety, compiler correctness, formal proof, whole-program correctness, or release eligibility.
```

Run lifecycle checks through the packaged Cairn input:

```bash
nix run .#lifecycle-validate -- --root .
```

## Non-claims

Kamacite checks do not prove these properties:

- compiler or evaluator correctness
- verifier correctness
- runtime behavior
- authorization
- semantic equivalence of arbitrary programs

Each result proves only the bounded claim that its artifact class and receipt name.

## License

`kamacite-core` uses `MIT OR Apache-2.0`. The `kamacite-cli` shell uses `AGPL-3.0-or-later`.

Complete texts are in [LICENSES](LICENSES). The package map is in [docs/licensing.md](docs/licensing.md).

The split does not change third-party licenses, generated artifacts, prior grants, or canonical identities. A core receipt does not grant host authority.

## References

- [PsychoLlama/ambient](https://github.com/PsychoLlama/ambient) — nominal ability and immutable identity reference.
- [`../valence/`](../valence/) — evidence identity and linkage layer.
- [`../octet/`](../octet/) — checked Rust and provenance layer.
- [`../aspen/`](../aspen/) — Molten runtime and component cohort.
- [`../mantle/`](../mantle/) — build evidence and SpaceWasm reference bundle.
- [nasa/spacewasm](https://github.com/nasa/spacewasm) at `e24cf09355a90497148eb5029fdb8e3400bd63e3` — experimental core-MVP interpreter cohort.
- [OnixResearch/nickel-export](https://github.com/OnixResearch/nickel-export) — Nickel export manifests and exact artifact evidence.
- [nickel-lang/nickel](https://github.com/nickel-lang/nickel) — Nickel evaluator and CLI.
- [nickel-lang/json-schema-to-nickel](https://github.com/nickel-lang/json-schema-to-nickel) — schema-to-contract conversion reference.
- [nickel-lang/tree-sitter-nickel](https://github.com/nickel-lang/tree-sitter-nickel) — Nickel syntax and span parser.
- [nickel-lang/json-scanner](https://github.com/nickel-lang/json-scanner) — JSON byte-position and number-lexeme reference.
- [nickel-lang/nickel-schemastore](https://github.com/nickel-lang/nickel-schemastore) — generated contract stress corpus.
- [mattwparas/steel](https://github.com/mattwparas/steel) — embedded Steel and typed host-function reference.
- [unisonweb/unison](https://github.com/unisonweb/unison) — content identity and structured migration reference.
- [dhall-lang/dhall-lang](https://github.com/dhall-lang/dhall-lang) — typed configuration and import-integrity reference.
- [cue-lang/cue](https://github.com/cue-lang/cue) — constraint-oriented configuration reference.
- [open-policy-agent/opa](https://github.com/open-policy-agent/opa) — policy and WASM target reference.
- [in-toto/in-toto](https://github.com/in-toto/in-toto) — bounded attestation reference.
- [slsa-framework/slsa](https://github.com/slsa-framework/slsa) — provenance vocabulary reference.
- [sigstore/rekor](https://github.com/sigstore/rekor) — transparency evidence reference.
- [ipld/specs](https://github.com/ipld/specs) — content-addressed data and codec reference.
- [koka-lang/koka](https://github.com/koka-lang/koka) — algebraic effect reference.
- [effekt-lang/effekt](https://github.com/effekt-lang/effekt) — effect-handler and capability reference.
- [OxCaml](https://oxcaml.org/) — explicit typed control reference.
- [WebAssembly/WASI](https://github.com/WebAssembly/WASI) — sandboxed capability-interface reference.
- [WebAssembly/component-model](https://github.com/WebAssembly/component-model) — WIT and component ABI reference.
- [ghc/ghc](https://gitlab.haskell.org/ghc/ghc) — Haskell compiler artifact reference.
- [llvm/llvm-project](https://github.com/llvm/llvm-project) — MLIR architecture and LLVM target reference.
- [joweeba/tMIR](https://github.com/joweeba/tMIR) — dialect round-trip and proof-lineage reference.
- [rust-lang/rust](https://github.com/rust-lang/rust) — Rust MIR and compiler report reference.
- [rust-lang/rustc-dev-guide](https://github.com/rust-lang/rustc-dev-guide/tree/main) — rustc architecture reference.
- [oxidecomputer/typify](https://github.com/oxidecomputer/typify) — schema-to-Rust translation reference.
- [oxidecomputer/openapi-lint](https://github.com/oxidecomputer/openapi-lint) — interchange-shape lint reference.
- [oxidecomputer/drift](https://github.com/oxidecomputer/drift) — compatibility classification reference.
- [oxidecomputer/dropshot-api-manager](https://github.com/oxidecomputer/dropshot-api-manager) — generated artifact freshness reference.
- [oxidecomputer/json-schema-tools](https://github.com/oxidecomputer/json-schema-tools) — shared schema traversal reference.
- [oxidecomputer/daft](https://github.com/oxidecomputer/daft) — typed structural difference reference.
- [oxidecomputer/hif](https://github.com/oxidecomputer/hif) — bounded `no_std` interchange machine reference.
- [oxidecomputer/iddqd](https://github.com/oxidecomputer/iddqd) — value-owned identity-map reference.
- [Ferrocene](https://ferrocene.dev/) and [Sealed Rust](https://github.com/ferrous-systems/sealed-rust) — restricted Rust profile references.
- [verus-lang/verus](https://github.com/verus-lang/verus) — Rust verification reference.
- [bytecodealliance/cap-std](https://github.com/bytecodealliance/cap-std) — capability-oriented Rust API reference.
- [bytecodealliance/wasmtime](https://github.com/bytecodealliance/wasmtime) — WebAssembly execution-control reference.
- [bytecodealliance/WASI-Virt](https://github.com/bytecodealliance/WASI-Virt) — deny-by-default WASI reference.
- [rust-lang/rust-clippy](https://github.com/rust-lang/rust-clippy) — Rust lint reference.
- [trailofbits/dylint](https://github.com/trailofbits/dylint) — project-owned lint reference.
- [geiger-rs/cargo-geiger](https://github.com/geiger-rs/cargo-geiger) and [rust-secure-code/safety-dance](https://github.com/rust-secure-code/safety-dance) — unsafe-code evidence references.
- [mozilla/cargo-vet](https://github.com/mozilla/cargo-vet), [EmbarkStudios/cargo-deny](https://github.com/EmbarkStudios/cargo-deny), and [rust-secure-code/cargo-auditable](https://github.com/rust-secure-code/cargo-auditable) — dependency and provenance references.
- [model-checking/kani](https://github.com/model-checking/kani), [viperproject/prusti-dev](https://github.com/viperproject/prusti-dev), and [creusot-rs/creusot](https://github.com/creusot-rs/creusot) — Rust verification references.
- [zkat/miette](https://github.com/zkat/miette), [mitsuhiko/insta](https://github.com/mitsuhiko/insta), [assert-rs/snapbox](https://github.com/assert-rs/snapbox), and [assert-rs/trycmd](https://github.com/assert-rs/trycmd) — diagnostics and fixture-test references.
- [HigherOrderCO/Bend](https://github.com/HigherOrderCO/Bend) at `814453670d0e0d6777c1313c972764dba0491b7f` — evaluation-only pure parallel-language and source-envelope reference.
- [HigherOrderCO/HVM2](https://github.com/HigherOrderCO/HVM2) at `654276018084b8f44a22b562dd68ab18583bfb5b` — evaluation-only CPU runtime and generated-C target reference.

These references do not transfer runtime, compiler, security, semantic, production, or release claims into Kamacite.
