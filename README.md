# Kamacite

Kamacite is an OnixResearch repo for lossless interchange boundaries across Nickel, Rust, and Steel Scheme.

The name comes from kamacite, a naturally occurring NiFe alloy found in meteoric iron: a compact metaphor for Nickel/Steel-adjacent materials sharing one stable interchange boundary.

Kamacite is not a new programming language at this stage. Call it an interchange IR, source-envelope format, receipt contract, and constrained policy vocabulary. If Kamacite later grows a human-authored syntax with its own grammar, evaluator, type rules, and semantics, that bounded surface can be named a DSL; until then, calling Kamacite a language would overclaim what it proves.

Yes: Kamacite should use Preserves too. Preserves is the canonical encoded IR candidate for value-level interchange and source envelopes because plain JSON cannot preserve enough identity for this problem. Nickel exports, Rust Serde DTOs, and Steel data can all project through the Kamacite IR, but the IR must keep original source bytes and hashes when source fidelity matters.

## Losslessness classes

| Class | Claim | Boundary |
|---|---|---|
| Source envelope | Original Nickel/Rust/Steel bytes are preserved with BLAKE3 identity and parser metadata. | No arbitrary semantic equivalence claim. |
| Canonical value | A Kamacite IR value round-trips through Preserves bytes and adapters. | Preserves bytes are authoritative; JSON is compatibility output. |
| Constrained policy DSL | A policy authored in the Kamacite schema can render to Nickel/Rust/Steel adapters and round-trip back to the same IR. | Only for the constrained schema, not arbitrary host-language code. |
| Unsupported source translation | Arbitrary Nickel, Rust, or Steel source is translated into another language. | Out of scope unless represented as preserved source plus explicit non-claims. |

## Reference IR hierarchy

Kamacite should use Preserves as the actual canonical IR/encoding authority and MLIR only as an architecture reference.

Adopt or adapt these MLIR ideas:

- dialect-like vocabularies for artifact classes and adapter surfaces;
- verifier-style deterministic validation;
- attributes/types as schema-bound metadata, not ambient state;
- locations as source-envelope spans;
- passes as receipt-producing transformations with explicit checked laws.

Keep the roles explicit:

- Preserves: canonical Kamacite value, source-envelope, and receipt identity.
- MLIR: architecture reference only; no MLIR textual or bytecode authority.
- Valence Evidence IR: downstream evidence/export shape.
- Unison: content-addressed identity discipline.
- Kamacite-owned layered pipeline: source envelopes, canonical Preserves values, adapter descriptors, target artifacts, and receipts are separate layers with separate identities.
- LLVM IR, WASM, GHC Core/STG, Rust MIR, bytecode, object files, and compiler reports: target/provenance evidence only.

Kamacite should use that layered pattern directly rather than borrowing another language's IR. Human-authored sources stay source envelopes; canonical machine identity is Preserves; transformations are receipt-producing adapter steps; backend/compiler outputs are target artifacts with non-claims.

## Haskell-inspired discipline

Kamacite should adapt Haskell's design discipline, not Haskell as a runtime or IR.

Adopt or adapt these principles:

- Pure functional cores with thin effectful shells for IO, evaluators, compilers, clocks, randomness, credentials, and presentation.
- Algebraic artifact modeling: source envelopes, canonical values, constrained policies, target artifacts, and unsupported claims are explicit alternatives.
- Explicit effect/capability data instead of ambient authority.
- Lawful adapter interfaces with positive, negative, and property-style fixtures.
- Newtype-style identity separation for source, canonical value, projection, target artifact, adapter, evaluator, compiler, and receipt identities.
- Total diagnostics instead of panics, silent defaults, truncation, or cross-domain coercions.

Do not adopt these by default:

- Haskell source, GHC Core, STG, bytecode, interface files, or generated Haskell artifacts as Kamacite canonical IR.
- Lazy evaluation as a Kamacite semantic promise.
- Typeclass-style implicit resolution across adapter boundaries.
- Monad terminology in receipts unless it maps to explicit effect/capability claims.

## Algebraic effect boundary

Kamacite should own algebraic effects as canonical data and receipts, not as a runtime.

Owned surfaces:

- effect signatures with operation identities, parameter/result schemas, and replay class;
- effect rows and summaries with discharged and residual operations;
- handler descriptors with continuation policy, checked laws, replay profile, and non-claims;
- handler-application receipts and effect-log projections with BLAKE3 identity.

Declaring an algebraic operation does not grant host authority. Filesystem, process, network, clock, random, device, credential, compiler, evaluator, and mutation access still require separate adapter/capability admission and fail closed when undeclared. Runtime systems such as the Aspen/Molten sibling may consume Kamacite effect descriptors and logs, but they do not own Kamacite's canonical effect model.

## WASM boundary

Kamacite should not use WebAssembly as its canonical IR. Preserves remains the authority for canonical values, source envelopes, and receipt identity.

WASM is a good later target for sandboxed adapters and portable validators. A WASM adapter should accept Kamacite IR or source-envelope bytes, run with declared capabilities only, and return a deterministic Kamacite receipt. WIT or the component model may describe the adapter ABI, but they must not replace the Preserves schema as the source of truth.

## LLVM boundary

Kamacite should not use LLVM IR as its canonical IR. LLVM is a compiler target/provenance artifact, not a source-preserving interchange format.

LLVM can be useful later for compiled Rust or adapter evidence: record LLVM IR, bitcode, target triple, optimization level, compiler version, pass pipeline, debug-info availability, and BLAKE3 hashes as target-artifact receipts. Those receipts can help explain lowering/provenance, but they must not claim Nickel/Rust/Steel semantic equivalence or source losslessness. LLVM `poison`, `undef`, target-specific behavior, optimization instability, and toolchain-version sensitivity make it unsuitable as Kamacite's cross-language identity layer.

## Octet and Valence boundary

Kamacite should use Valence closely at the evidence layer. Kamacite receipts should be easy to wrap as Valence Evidence IR artifacts with stable IDs, spans, roles, BLAKE3 hashes, and explicit non-claims. Valence remains the stack evidence/linkage authority; Kamacite owns source preservation, value interchange, adapter classification, and round-trip receipts.

Kamacite should use Octet closely as the checked Rust path once Rust crates exist. Octet should lint and collect provenance for Kamacite's Rust validators, adapters, fixture runners, and receipt code. Octet does not define Kamacite's IR, and Octet findings do not prove Nickel/Rust/Steel semantic equivalence.

Initial coupling should be by exported receipts and fixtures, not hard dependencies. Add direct crate dependencies only after the receipt schemas stabilize and the integration needs tested APIs rather than reference behavior.

## Nickel integration boundary

Kamacite should integrate Nickel as a source language and adapter, not fork or reimplement Nickel semantics.

- Use the Nickel CLI or reference crates for checked export/evaluation fixtures.
- Use Nickel source envelopes to preserve original `.ncl` bytes, parser identity, spans, and BLAKE3 source hashes.
- Treat `nickel export` results as derived projections into Kamacite IR or compatibility JSON, not as source identity.
- Adapt schema-to-Nickel-contract generation ideas for Kamacite's constrained policy vocabulary, while preserving non-claims when strict predicates are required.
- Use tree-sitter Nickel for syntax/spans and editor-grade source indexing; do not treat syntax parsing as evaluation.
- Use generated contract corpora, such as SchemaStore-derived contracts, as stress fixtures rather than as Kamacite-owned schema authority.

## Steel integration boundary

Kamacite should integrate Steel as a bounded adapter/probe language, not as the canonical IR and not as an ambient scripting environment.

Adopt or adapt these Steel patterns:

- `Engine` embedding for controlled evaluation fixtures, with evaluator/version identity recorded in receipts.
- Rust `register_fn` / `IntoSteelVal` / `FromSteelVal` style typed host-function boundaries for Kamacite adapter ABIs.
- Built-in module registration for explicitly declared Kamacite host functions.
- Steel `provide`/`require` and contract exports as the model for module-boundary claims.
- Higher-order contracts only when callables are descriptor-bound, hash-bound, and receipt-bound.

Do not adopt these by default:

- Ambient filesystem, process, TCP, random, time, package, or dylib loading authority.
- Steel bytecode as Kamacite IR.
- Opaque Steel closures as portable values across Kamacite boundaries.
- JSON serialization as anything more than a compatibility projection.

A Steel adapter should consume Kamacite IR/source-envelope bytes or declared DTO values and emit a deterministic Kamacite receipt. Any host capability must be declared before evaluation and denied by default.

## Unison-inspired identity boundary

Kamacite should adapt Unison's content-addressed codebase ideas without becoming Unison or storing executable code as the source of truth.

Adopt or adapt these Unison patterns:

- Content-addressed definitions: canonical Kamacite IR values and source envelopes get stable BLAKE3 identities over canonical content.
- Names as metadata: human names, paths, aliases, and labels are review metadata, not identity.
- Dependency-closure identity: receipts should bind dependency hashes and adapter/evaluator identities when a projection depends on other artifacts.
- Ability/effect labels: adapter receipts should classify required effects/capabilities explicitly, like Unison abilities classify effectful computations.
- Structured refactoring sessions: alias/name/schema migrations should be dry-run-first plans over identities, not text rewrites that imply semantic equivalence.
- Typed durable storage envelopes: persisted Kamacite values should bind schema, codec, content hash, and non-claims before decoding.

Do not adopt these by default:

- Unison's syntax tree hash as Kamacite's hash algorithm; Kamacite defaults to BLAKE3 over its own canonical bytes.
- Unison's codebase database as Kamacite's storage model.
- Unison Cloud/distributed execution or arbitrary computation shipping.
- Typed serialization of arbitrary functions as a Kamacite promise.

## Current status

Kamacite has a first walking-skeleton validation harness and Rust core profile implementation:

- `crates/kamacite-core/` is the pure deterministic core for typed identities, source-envelope validation, canonical/target/adapter boundary checks, algebraic-effect artifact checks, deterministic diagnostics, and receipt construction.
- `crates/kamacite-cli/` is the thin fixture runner shell for manifest/file IO, golden receipt comparison, and explicit update workflows.
- `schemas/preserves/` contains Kamacite-owned Preserves boundary schema artifacts for source envelopes, canonical values, adapter descriptors, target artifacts, receipts, and algebraic-effect artifacts.
- `fixtures/` contains the Nickel seed manifest, runtime JSON projection, positive and negative fixture materials, and checked golden receipts.
- `docs/validation-harness.md`, `docs/rust-core-profile.md`, `docs/deferred-adapter-suites.md`, and `docs/algebraic-effects-stack-roles.md` describe the current executable boundary and deferred integration suites.

Run harness validation through the sibling Cairn dev shell until Kamacite grows its own package:

```bash
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo test --workspace'
nix develop path:/home/brittonr/git/OnixResearch/cairn -c sh -lc 'cd /home/brittonr/git/OnixResearch/kamacite && cargo run -q -p kamacite-cli -- validate --root . --manifest fixtures/manifest.json'
```

Run lifecycle validation with the sibling Cairn repo and its generated policy bundle:

```bash
POLICY=/home/brittonr/git/OnixResearch/cairn/cairn-policy/generated/cairn-policy.json
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- validate --root . --policy "$POLICY"
```

## Non-claims

Kamacite validation does not prove compiler correctness, evaluator soundness, verifier soundness, runtime behavior, authorization, or semantic equivalence of arbitrary programs. It proves only the bounded claim named by the artifact class and receipt.

## References

- [`../valence/`](../valence/) — sibling evidence identity layer that should consume or wrap Kamacite receipts without owning Kamacite's source/value interchange semantics.
- [`../octet/`](../octet/) — sibling Rust checked path and provenance rail for future Kamacite Rust validators and adapters.
- [`../aspen/`](../aspen/) — sibling Molten runtime that may consume Kamacite algebraic-effect descriptors, handler profiles, and effect logs without owning Kamacite's canonical effect schema.
- [nickel-lang/nickel](https://github.com/nickel-lang/nickel) — reference implementation and CLI for Nickel evaluation/export, formatting, contracts, and source-language semantics.
- [nickel-lang/json-schema-to-nickel](https://github.com/nickel-lang/json-schema-to-nickel) — reference for converting schema predicates into Nickel contracts and distinguishing lazy/inspectable contracts from strict predicate fallbacks.
- [nickel-lang/tree-sitter-nickel](https://github.com/nickel-lang/tree-sitter-nickel) — reference parser for Nickel syntax, spans, and source-envelope indexing without claiming evaluation semantics.
- [nickel-lang/json-scanner](https://github.com/nickel-lang/json-scanner) — reference for JSON projection parsing with byte positions and number lexeme preservation.
- [nickel-lang/nickel-schemastore](https://github.com/nickel-lang/nickel-schemastore) — generated Nickel contract corpus suitable for stress fixtures and schema-conversion regression cases.
- [mattwparas/steel](https://github.com/mattwparas/steel) — reference implementation for embedded Steel evaluation, typed Rust host-function registration, module boundaries, and higher-order contract patterns used only as bounded adapter/probe surfaces.
- [unisonweb/unison](https://github.com/unisonweb/unison) — reference for content-addressed definitions, names-as-metadata, dependency-closure identity, abilities/effect labels, structured refactoring, and typed durable storage boundaries.
- [dhall-lang/dhall-lang](https://github.com/dhall-lang/dhall-lang) — reference for typed configuration, import integrity, and semantic hashing; Kamacite preserves source envelopes separately rather than treating normalized configuration as source identity.
- [cue-lang/cue](https://github.com/cue-lang/cue) — reference for constraint-oriented configuration and validation; Kamacite may adapt constraint vocabulary patterns without adopting CUE as its source or canonical IR.
- [open-policy-agent/opa](https://github.com/open-policy-agent/opa) — reference for policy evaluation, partial evaluation, and WASM policy targets; Kamacite receipts can classify policy-adapter behavior without becoming a policy runtime.
- [in-toto/in-toto](https://github.com/in-toto/in-toto) — reference for supply-chain attestations and link metadata; Kamacite receipts should make similarly bounded provenance claims over source envelopes, adapters, and target artifacts.
- [slsa-framework/slsa](https://github.com/slsa-framework/slsa) — reference for supply-chain provenance levels and build attestation vocabulary; Kamacite uses it only as receipt/provenance prior art, not as its artifact schema.
- [sigstore/rekor](https://github.com/sigstore/rekor) — reference for transparency-log-backed attestations and artifact inclusion evidence; Kamacite may export compatible evidence without making transparency logging part of canonical IR.
- [ipld/specs](https://github.com/ipld/specs) — reference for content-addressed structured data, codecs such as DAG-CBOR, and typed data-model boundaries; Kamacite still uses BLAKE3 over Kamacite-owned canonical Preserves bytes for identity.
- [koka-lang/koka](https://github.com/koka-lang/koka) — reference for algebraic effects, effect rows, and handlers as language features; Kamacite models effect signatures and receipts as data rather than adopting Koka semantics.
- [effekt-lang/effekt](https://github.com/effekt-lang/effekt) — reference for effect handlers and capability-style control boundaries; Kamacite uses it only to inform effect vocabulary and continuation-policy receipts.
- [OxCaml](https://oxcaml.org/) — reference for evolving an existing typed language with pay-as-you-go, type-level control over performance, concurrency, layout, and allocation; Kamacite may adapt the explicit-control discipline without treating OCaml/OxCaml compiler artifacts or extensions as Kamacite IR.
- [WebAssembly/WASI](https://github.com/WebAssembly/WASI) — reference for sandboxed capability interfaces; Kamacite may target WASI adapters while keeping Preserves source/value identity authoritative.
- [WebAssembly/component-model](https://github.com/WebAssembly/component-model) — reference for WIT/component adapter ABIs; Kamacite may use component interfaces for adapter shells without replacing Kamacite schemas or receipts.
- [ghc/ghc](https://gitlab.haskell.org/ghc/ghc) — reference for Haskell/GHC concepts used only to bound what Kamacite does not adopt as IR: Core, STG, bytecode, interface files, laziness, and typeclass resolution.
- [llvm/llvm-project](https://github.com/llvm/llvm-project) — reference for MLIR architecture concepts and LLVM compiled target artifacts; neither MLIR nor LLVM IR is Kamacite's canonical IR.
- [rust-lang/rust](https://github.com/rust-lang/rust) — reference for Rust MIR and rustc compiler reports as Rust provenance evidence only, not Kamacite IR.
- [rust-lang/rustc-dev-guide](https://github.com/rust-lang/rustc-dev-guide/tree/main) — primary reference for rustc architecture, especially [HIR](https://github.com/rust-lang/rustc-dev-guide/tree/main/src/hir), [MIR](https://github.com/rust-lang/rustc-dev-guide/tree/main/src/mir), and [backend lowering](https://github.com/rust-lang/rustc-dev-guide/tree/main/src/backend); Kamacite uses it to shape Rust adapter/provenance receipts while keeping rustc internals as target/provenance evidence, not Kamacite IR.
- [oxidecomputer/typify](https://github.com/oxidecomputer/typify) — reference for schema-to-Rust generation limits, `x-rust-type` identity, and honest non-lossless schema translation language.
- [oxidecomputer/openapi-lint](https://github.com/oxidecomputer/openapi-lint) — reference for rejecting schema shapes that are valid but poor interchange surfaces.
- [oxidecomputer/drift](https://github.com/oxidecomputer/drift) — reference for classifying compatibility changes and explicitly naming unhandled comparisons.
- [oxidecomputer/dropshot-api-manager](https://github.com/oxidecomputer/dropshot-api-manager) — reference for generated artifact freshness, versioned boundary management, and jj/git-aware validation workflows.
- [oxidecomputer/json-schema-tools](https://github.com/oxidecomputer/json-schema-tools) — reference for shared schema traversal used by multiple renderers without duplicating interpretation logic.
- [oxidecomputer/daft](https://github.com/oxidecomputer/daft) — reference for typed structural diffs of Rust values and leaf-vs-recursive diff boundaries.
- [oxidecomputer/hif](https://github.com/oxidecomputer/hif) — reference for a bounded no-std interchange machine with explicit host-provided functions and runtime limits.
- [oxidecomputer/iddqd](https://github.com/oxidecomputer/iddqd) — reference for Rust identity maps where keys are borrowed from values, duplicate keys fail closed during serde, and multi-key bijective/trijective indexes preserve value-owned identity.
- [Ferrocene](https://ferrocene.dev/) and [Sealed Rust](https://github.com/ferrous-systems/sealed-rust) — reference for naming and governing restricted/qualified Rust profiles; Kamacite adapts profile discipline only and does not claim compiler qualification or certification.
- [verus-lang/verus](https://github.com/verus-lang/verus) — reference for a verification-oriented Rust subset and specification discipline; Kamacite may use verifier evidence as bounded support without treating Verus syntax or proof artifacts as Kamacite IR.
- [bytecodealliance/cap-std](https://github.com/bytecodealliance/cap-std) — reference for capability-oriented Rust APIs and avoiding ambient filesystem/network authority in shells and adapters, not in Kamacite's pure core.
- [bytecodealliance/wasmtime](https://github.com/bytecodealliance/wasmtime) — reference for deterministic WebAssembly execution controls and explicit import/capability boundaries used only for adapter-target guidance.
- [bytecodealliance/WASI-Virt](https://github.com/bytecodealliance/WASI-Virt) — reference for deny-by-default WASI subsystem virtualization and explicit allow/deny configuration for adapter shells.
- [rust-lang/rust-clippy](https://github.com/rust-lang/rust-clippy) — reference for configurable Rust lints and restriction-lint practice used as source-shape evidence, not receipt validation.
- [trailofbits/dylint](https://github.com/trailofbits/dylint) — reference for project-owned Rust lint libraries suitable for Kamacite/Octet-specific implementation-profile checks.
- [geiger-rs/cargo-geiger](https://github.com/geiger-rs/cargo-geiger) and [rust-secure-code/safety-dance](https://github.com/rust-secure-code/safety-dance) — references for measuring and reducing unsafe Rust usage; Kamacite treats these as implementation evidence only.
- [mozilla/cargo-vet](https://github.com/mozilla/cargo-vet), [EmbarkStudios/cargo-deny](https://github.com/EmbarkStudios/cargo-deny), and [rust-secure-code/cargo-auditable](https://github.com/rust-secure-code/cargo-auditable) — references for Rust dependency review, policy checks, and auditable build metadata; these support provenance but do not prove Kamacite semantics.
- [model-checking/kani](https://github.com/model-checking/kani), [viperproject/prusti-dev](https://github.com/viperproject/prusti-dev), and [creusot-rs/creusot](https://github.com/creusot-rs/creusot) — references for Rust model-checking and deductive verification rails that may support bounded core properties without replacing Kamacite fixtures or receipts.
- [zkat/miette](https://github.com/zkat/miette), [mitsuhiko/insta](https://github.com/mitsuhiko/insta), [assert-rs/snapbox](https://github.com/assert-rs/snapbox), and [assert-rs/trycmd](https://github.com/assert-rs/trycmd) — references for human diagnostics and deterministic snapshot/CLI testing; Kamacite canonical diagnostics and receipt identity remain data-first and presentation-independent.
