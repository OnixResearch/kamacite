# Braid

Braid is an OnixResearch repo for lossless interchange boundaries across Nickel, Rust, and Steel Scheme.

Braid is not a new programming language at this stage. Call it an interchange IR, source-envelope format, receipt contract, and constrained policy vocabulary. If Braid later grows a human-authored syntax with its own grammar, evaluator, type rules, and semantics, that bounded surface can be named a DSL; until then, calling Braid a language would overclaim what it proves.

Yes: Braid should use Preserves too. Preserves is the canonical encoded IR candidate for value-level interchange and source envelopes because plain JSON cannot preserve enough identity for this problem. Nickel exports, Rust Serde DTOs, and Steel data can all project through the Braid IR, but the IR must keep original source bytes and hashes when source fidelity matters.

## Losslessness classes

| Class | Claim | Boundary |
|---|---|---|
| Source envelope | Original Nickel/Rust/Steel bytes are preserved with BLAKE3 identity and parser metadata. | No arbitrary semantic equivalence claim. |
| Canonical value | A Braid IR value round-trips through Preserves bytes and adapters. | Preserves bytes are authoritative; JSON is compatibility output. |
| Constrained policy DSL | A policy authored in the Braid schema can render to Nickel/Rust/Steel adapters and round-trip back to the same IR. | Only for the constrained schema, not arbitrary host-language code. |
| Unsupported source translation | Arbitrary Nickel, Rust, or Steel source is translated into another language. | Out of scope unless represented as preserved source plus explicit non-claims. |

## Reference IR hierarchy

Braid should use Preserves as the actual canonical IR/encoding authority and MLIR only as an architecture reference.

Adopt or adapt these MLIR ideas:

- dialect-like vocabularies for artifact classes and adapter surfaces;
- verifier-style deterministic validation;
- attributes/types as schema-bound metadata, not ambient state;
- locations as source-envelope spans;
- passes as receipt-producing transformations with explicit checked laws.

Keep the roles explicit:

- Preserves: canonical Braid value, source-envelope, and receipt identity.
- MLIR: architecture reference only; no MLIR textual or bytecode authority.
- Valence Evidence IR: downstream evidence/export shape.
- Unison: content-addressed identity discipline.
- Chelis: layered language-pipeline reference for human syntax, canonical machine syntax, typed/effectful validation, DAG lowering, backend conformance, and CI-validated examples.
- LLVM IR, WASM, GHC Core/STG, Rust MIR, bytecode, object files, and compiler reports: target/provenance evidence only.

Chelis is especially useful because it separates Surf as human syntax, Deep as canonical machine syntax, RISC DAG lowering, backend source emission, and `chelis validate` conformance checks. Braid should adapt that layering and validation discipline without adopting Chelis Deep, its RISC DAG, tensor language semantics, numerical oracle, or backends as Braid canonical IR.

## Haskell-inspired discipline

Braid should adapt Haskell's design discipline, not Haskell as a runtime or IR.

Adopt or adapt these principles:

- Pure functional cores with thin effectful shells for IO, evaluators, compilers, clocks, randomness, credentials, and presentation.
- Algebraic artifact modeling: source envelopes, canonical values, constrained policies, target artifacts, and unsupported claims are explicit alternatives.
- Explicit effect/capability data instead of ambient authority.
- Lawful adapter interfaces with positive, negative, and property-style fixtures.
- Newtype-style identity separation for source, canonical value, projection, target artifact, adapter, evaluator, compiler, and receipt identities.
- Total diagnostics instead of panics, silent defaults, truncation, or cross-domain coercions.

Do not adopt these by default:

- Haskell source, GHC Core, STG, bytecode, interface files, or generated Haskell artifacts as Braid canonical IR.
- Lazy evaluation as a Braid semantic promise.
- Typeclass-style implicit resolution across adapter boundaries.
- Monad terminology in receipts unless it maps to explicit effect/capability claims.

## WASM boundary

Braid should not use WebAssembly as its canonical IR. Preserves remains the authority for canonical values, source envelopes, and receipt identity.

WASM is a good later target for sandboxed adapters and portable validators. A WASM adapter should accept Braid IR or source-envelope bytes, run with declared capabilities only, and return a deterministic Braid receipt. WIT or the component model may describe the adapter ABI, but they must not replace the Preserves schema as the source of truth.

## LLVM boundary

Braid should not use LLVM IR as its canonical IR. LLVM is a compiler target/provenance artifact, not a source-preserving interchange format.

LLVM can be useful later for compiled Rust or adapter evidence: record LLVM IR, bitcode, target triple, optimization level, compiler version, pass pipeline, debug-info availability, and BLAKE3 hashes as target-artifact receipts. Those receipts can help explain lowering/provenance, but they must not claim Nickel/Rust/Steel semantic equivalence or source losslessness. LLVM `poison`, `undef`, target-specific behavior, optimization instability, and toolchain-version sensitivity make it unsuitable as Braid's cross-language identity layer.

## Octet and Valence boundary

Braid should use Valence closely at the evidence layer. Braid receipts should be easy to wrap as Valence Evidence IR artifacts with stable IDs, spans, roles, BLAKE3 hashes, and explicit non-claims. Valence remains the stack evidence/linkage authority; Braid owns source preservation, value interchange, adapter classification, and round-trip receipts.

Braid should use Octet closely as the checked Rust path once Rust crates exist. Octet should lint and collect provenance for Braid's Rust validators, adapters, fixture runners, and receipt code. Octet does not define Braid's IR, and Octet findings do not prove Nickel/Rust/Steel semantic equivalence.

Initial coupling should be by exported receipts and fixtures, not hard dependencies. Add direct crate dependencies only after the receipt schemas stabilize and the integration needs tested APIs rather than reference behavior.

## Nickel integration boundary

Braid should integrate Nickel as a source language and adapter, not fork or reimplement Nickel semantics.

- Use the Nickel CLI or reference crates for checked export/evaluation fixtures.
- Use Nickel source envelopes to preserve original `.ncl` bytes, parser identity, spans, and BLAKE3 source hashes.
- Treat `nickel export` results as derived projections into Braid IR or compatibility JSON, not as source identity.
- Adapt schema-to-Nickel-contract generation ideas for Braid's constrained policy vocabulary, while preserving non-claims when strict predicates are required.
- Use tree-sitter Nickel for syntax/spans and editor-grade source indexing; do not treat syntax parsing as evaluation.
- Use generated contract corpora, such as SchemaStore-derived contracts, as stress fixtures rather than as Braid-owned schema authority.

## Steel integration boundary

Braid should integrate Steel as a bounded adapter/probe language, not as the canonical IR and not as an ambient scripting environment.

Adopt or adapt these Steel patterns:

- `Engine` embedding for controlled evaluation fixtures, with evaluator/version identity recorded in receipts.
- Rust `register_fn` / `IntoSteelVal` / `FromSteelVal` style typed host-function boundaries for Braid adapter ABIs.
- Built-in module registration for explicitly declared Braid host functions.
- Steel `provide`/`require` and contract exports as the model for module-boundary claims.
- Higher-order contracts only when callables are descriptor-bound, hash-bound, and receipt-bound.

Do not adopt these by default:

- Ambient filesystem, process, TCP, random, time, package, or dylib loading authority.
- Steel bytecode as Braid IR.
- Opaque Steel closures as portable values across Braid boundaries.
- JSON serialization as anything more than a compatibility projection.

A Steel adapter should consume Braid IR/source-envelope bytes or declared DTO values and emit a deterministic Braid receipt. Any host capability must be declared before evaluation and denied by default.

## Unison-inspired identity boundary

Braid should adapt Unison's content-addressed codebase ideas without becoming Unison or storing executable code as the source of truth.

Adopt or adapt these Unison patterns:

- Content-addressed definitions: canonical Braid IR values and source envelopes get stable BLAKE3 identities over canonical content.
- Names as metadata: human names, paths, aliases, and labels are review metadata, not identity.
- Dependency-closure identity: receipts should bind dependency hashes and adapter/evaluator identities when a projection depends on other artifacts.
- Ability/effect labels: adapter receipts should classify required effects/capabilities explicitly, like Unison abilities classify effectful computations.
- Structured refactoring sessions: alias/name/schema migrations should be dry-run-first plans over identities, not text rewrites that imply semantic equivalence.
- Typed durable storage envelopes: persisted Braid values should bind schema, codec, content hash, and non-claims before decoding.

Do not adopt these by default:

- Unison's syntax tree hash as Braid's hash algorithm; Braid defaults to BLAKE3 over its own canonical bytes.
- Unison's codebase database as Braid's storage model.
- Unison Cloud/distributed execution or arbitrary computation shipping.
- Typed serialization of arbitrary functions as a Braid promise.

## Current status

This repo is bootstrapped with native Cairn lifecycle artifacts. The first active change is:

```text
cairn/changes/establish-lossless-ir/
```

Run lifecycle validation with the sibling Cairn repo:

```bash
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- validate --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate proposal establish-lossless-ir --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate design establish-lossless-ir --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate tasks establish-lossless-ir --root .
```

## Non-claims

Braid validation does not prove compiler correctness, evaluator soundness, verifier soundness, runtime behavior, authorization, or semantic equivalence of arbitrary programs. It proves only the bounded claim named by the artifact class and receipt.

## References

- [`../valence/`](../valence/) — sibling evidence identity layer that should consume or wrap Braid receipts without owning Braid's source/value interchange semantics.
- [`../octet/`](../octet/) — sibling Rust checked path and provenance rail for future Braid Rust validators and adapters.
- [nickel-lang/nickel](https://github.com/nickel-lang/nickel) — reference implementation and CLI for Nickel evaluation/export, formatting, contracts, and source-language semantics.
- [nickel-lang/json-schema-to-nickel](https://github.com/nickel-lang/json-schema-to-nickel) — reference for converting schema predicates into Nickel contracts and distinguishing lazy/inspectable contracts from strict predicate fallbacks.
- [nickel-lang/tree-sitter-nickel](https://github.com/nickel-lang/tree-sitter-nickel) — reference parser for Nickel syntax, spans, and source-envelope indexing without claiming evaluation semantics.
- [nickel-lang/json-scanner](https://github.com/nickel-lang/json-scanner) — reference for JSON projection parsing with byte positions and number lexeme preservation.
- [nickel-lang/nickel-schemastore](https://github.com/nickel-lang/nickel-schemastore) — generated Nickel contract corpus suitable for stress fixtures and schema-conversion regression cases.
- [mattwparas/steel](https://github.com/mattwparas/steel) — reference implementation for embedded Steel evaluation, typed Rust host-function registration, module boundaries, and higher-order contract patterns used only as bounded adapter/probe surfaces.
- [unisonweb/unison](https://github.com/unisonweb/unison) — reference for content-addressed definitions, names-as-metadata, dependency-closure identity, abilities/effect labels, structured refactoring, and typed durable storage boundaries.
- [ghc/ghc](https://gitlab.haskell.org/ghc/ghc) — reference for Haskell/GHC concepts used only to bound what Braid does not adopt as IR: Core, STG, bytecode, interface files, laziness, and typeclass resolution.
- [llvm/llvm-project](https://github.com/llvm/llvm-project) — reference for MLIR architecture concepts and LLVM compiled target artifacts; neither MLIR nor LLVM IR is Braid's canonical IR.
- [rust-lang/rust](https://github.com/rust-lang/rust) — reference for Rust MIR and rustc compiler reports as Rust provenance evidence only, not Braid IR.
- [Chelis Developer Book](https://chelis.ch/docs/chelis/) — reference for a layered Surf/Deep/RISC-DAG/backend pipeline, explicit tensor/effect typing, validation command discipline, and CI-validated examples; Chelis Deep and RISC DAG are not Braid IR.
- [oxidecomputer/typify](https://github.com/oxidecomputer/typify) — reference for schema-to-Rust generation limits, `x-rust-type` identity, and honest non-lossless schema translation language.
- [oxidecomputer/openapi-lint](https://github.com/oxidecomputer/openapi-lint) — reference for rejecting schema shapes that are valid but poor interchange surfaces.
- [oxidecomputer/drift](https://github.com/oxidecomputer/drift) — reference for classifying compatibility changes and explicitly naming unhandled comparisons.
- [oxidecomputer/dropshot-api-manager](https://github.com/oxidecomputer/dropshot-api-manager) — reference for generated artifact freshness, versioned boundary management, and jj/git-aware validation workflows.
- [oxidecomputer/json-schema-tools](https://github.com/oxidecomputer/json-schema-tools) — reference for shared schema traversal used by multiple renderers without duplicating interpretation logic.
- [oxidecomputer/daft](https://github.com/oxidecomputer/daft) — reference for typed structural diffs of Rust values and leaf-vs-recursive diff boundaries.
- [oxidecomputer/hif](https://github.com/oxidecomputer/hif) — reference for a bounded no-std interchange machine with explicit host-provided functions and runtime limits.
- [oxidecomputer/iddqd](https://github.com/oxidecomputer/iddqd) — reference for Rust identity maps where keys are borrowed from values, duplicate keys fail closed during serde, and multi-key bijective/trijective indexes preserve value-owned identity.
