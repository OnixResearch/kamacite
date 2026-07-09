## Context

The stack already uses a practical split: Nickel owns human-authored contract/config review boundaries, Rust owns runtime validators and DTOs, and Steel Scheme appears as bounded predicate/probe logic. Existing JSON handoffs are useful for runtime consumption but are not sufficient for lossless interchange claims.

Braid starts by documenting the invariant rather than pretending arbitrary source-to-source translation is available. The canonical path is source-preserving envelopes plus a constrained value/policy IR with deterministic receipts.

The Cairn delta is split into focused specs:

- `lossless-ir`: top-level format strategy, naming, losslessness classes, evidence, and non-claims.
- `reference-ir-boundary`: Preserves authority, MLIR architecture-reference role, Valence evidence shape, Unison identity reference, and compiler-IR non-claims.
- `schema-vocabulary`: versioned source envelope, canonical value, adapter descriptor, target artifact, and receipt vocabulary.
- `fixture-corpus`: positive, negative, property-style, and golden-receipt fixture requirements.
- `validation-command`: deterministic validation command, discovery, execution, diagnostic, and mutation boundaries.
- `source-envelopes`: source byte preservation and Nickel reference-tool boundaries.
- `adapter-boundaries`: Nickel/Rust/Steel/WASM/LLVM adapter and target-artifact trust boundaries.
- `adapter-conformance`: conformance matrix for adapters, target producers, Valence exports, and Octet checks.
- `identity-model`: content-addressed identity, metadata aliases, dependency closure, effects, and durable storage.
- `typed-core-discipline`: Haskell-inspired pure-core, typed-identity, lawful-adapter, total-diagnostic, and GHC non-claim boundaries.
- `stack-integration`: Valence and Octet receipt/provenance integration boundaries.

## Decisions

### 1. Name the repo Braid

**Choice:** Use `braid` as the new OnixResearch repository name.

**Rationale:** The project braids three language strands without claiming they become one language. The name fits a boundary repo rather than a compiler or evaluator.

### 2. Do not call Braid a new language yet

**Choice:** Braid is an interchange IR, source-envelope format, receipt contract, and constrained policy vocabulary. It is not a programming language unless a later change defines a human-authored syntax, grammar, evaluator, type rules, and semantics.

**Rationale:** Calling Braid a language would imply semantics and evaluation authority that the project explicitly does not claim. Nickel, Rust, and Steel remain the languages; Braid preserves sources, classifies artifacts, and verifies bounded interchange receipts.

**Rejected alternative:** Market Braid as a universal cross-language language. This would encourage unsupported source-to-source and semantic-equivalence claims.

### 3. Use Preserves as the canonical encoded IR candidate

**Choice:** Braid uses Preserves as the authoritative encoded form for canonical IR values, source envelopes, and receipt hash material where the project controls the format.

**Rationale:** Preserves is a better fit than JSON for canonical identity and typed data. JSON remains useful as compatibility output, but it is not the authority for losslessness because it cannot carry every distinction Braid needs without extra conventions.

**Rejected alternative:** Use JSON as the only IR. This would repeat the current lossy boundary and make source fidelity and typed identity claims fragile.

### 4. Keep the reference IR hierarchy explicit

**Choice:** Braid uses Preserves as the actual canonical IR/encoding authority. MLIR is an architecture reference for dialect-style vocabularies, verifier organization, attributes, locations, and pass pipelines. Valence Evidence IR is the evidence export shape. Unison is an identity-discipline reference. LLVM IR, WebAssembly, GHC Core, STG, Rust MIR, bytecode, object files, and compiler reports are target/provenance artifacts only.

**Rationale:** Braid needs one authority for identity and several bounded references for design, evidence, identity, and target provenance. Keeping those roles separate prevents compiler artifacts or evidence projections from being mistaken for source-preserving interchange.

**Rejected alternative:** Use MLIR, LLVM IR, WebAssembly, GHC Core/STG, Rust MIR, or another compiler IR as Braid's canonical cross-language IR. These forms do not preserve the original source boundary and would overclaim semantic equivalence across Nickel, Rust, Steel, and future adapters.

### 5. Preserve source separately from semantic projections

**Choice:** Ingested Nickel, Rust, and Steel source artifacts keep original bytes, declared language, parser/normalizer identity, and BLAKE3 source hash. Parsed or normalized projections are derived evidence and must not replace the original source.

**Rationale:** Arbitrary source-to-source translation is not lossless in a useful sense. Preserving source plus declaring narrower derived claims is honest and testable.

### 6. Integrate Nickel through reference semantics and fixtures

**Choice:** Braid uses nickel-lang components as Nickel-facing adapter references: the Nickel CLI or reference crates for evaluation/export semantics, tree-sitter Nickel for syntax and span indexing, json-schema-to-nickel for schema-to-contract design patterns, json-scanner for JSON projection spans, and generated contract corpora such as nickel-schemastore as stress fixtures. Braid does not fork Nickel semantics.

**Rationale:** Nickel is already the source language for human-authored contracts/config. Braid needs faithful source preservation and bounded projections, not a competing evaluator. Using upstream Nickel tooling as references keeps Nickel semantics external while giving Braid enough fixture coverage to test source envelopes, exports, spans, and contract-generation boundaries.

**Rejected alternative:** Reimplement Nickel evaluation inside Braid. This would create semantic drift and make Braid responsible for a language it is meant to preserve and adapt.

### 7. Constrain adapter claims to explicit artifact classes

**Choice:** Braid recognizes source envelopes, canonical values, constrained policy DSL values, and unsupported arbitrary source translation as distinct classes.

**Rationale:** The same bytes may support different claims. A constrained DSL can round-trip through adapters, while arbitrary Rust or Steel source can only be preserved unless a specific parser/projection contract exists.

### 8. Keep Rust as the pure validation core

**Choice:** Rust owns pure deterministic validation, canonicalization, hash material construction, adapter DTO validation, and receipt checks. Nickel and Steel adapters stay at the boundary.

**Rationale:** This preserves functional-core / imperative-shell boundaries and gives tests a pure unit under test. Nickel remains the reviewable authoring language for configs/contracts. Steel remains bounded dynamic predicate/probe logic without ambient host authority.

### 9. Adapt Haskell's type, law, and effect discipline without adopting Haskell as IR

**Choice:** Braid adapts Haskell-inspired principles as implementation discipline: pure functional cores with effectful shells, algebraic artifact alternatives, distinct identity domains, explicit effect/capability data, law-based adapter interfaces, property-style fixture thinking, total diagnostics, and newtype-style separation for source, canonical, projection, target, adapter, evaluator, compiler, and receipt identities.

**Rationale:** These principles match Braid's bounded-claim model. They make unsupported states harder to represent, keep validation deterministic, and let adapters state laws that can be checked by positive, negative, and property-style fixtures. They also fit Rust implementation well without making Braid depend on Haskell runtime semantics.

**Rejected alternative:** Treat Haskell source, GHC Core, STG, laziness, typeclass resolution, monad terminology, or GHC artifacts as Braid's canonical IR or proof authority. That would confuse a useful design discipline with a new interchange format and would overclaim semantic equivalence.

### 10. Integrate Steel as a bounded adapter/probe evaluator

**Choice:** Braid uses Steel as a bounded adapter/probe language through explicit engine identity, typed Rust host-function registration, declared built-in modules, module contracts, and deterministic receipts. Braid denies ambient filesystem, process, TCP, random, time, package, dylib, credential, and mutation authority by default. Steel bytecode, opaque closures, and JSON strings are not Braid canonical IR.

**Rationale:** Steel is designed as an embeddable Scheme implemented in Rust, with a bytecode VM, modules, Rust function registration, value conversions, and higher-order contracts. Those features fit Braid's adapter/probe layer, especially for dynamic predicates over declared DTO inputs. They do not replace Braid's Preserves-backed value/source identity or source-envelope non-claims.

**Rejected alternative:** Treat Steel as an unrestricted scripting host or as the canonical representation for Braid values. This would reintroduce ambient authority and confuse evaluation behavior with artifact identity.

### 11. Use WebAssembly only as a target or sandbox boundary

**Choice:** Braid does not use WebAssembly as the canonical IR. WASM may be used later as a portable target for validators, conformance probes, or untrusted adapters whose inputs and outputs are Braid IR/source-envelope bytes plus deterministic receipts.

**Rationale:** WebAssembly is an execution format and sandbox boundary, not a source-preserving interchange format. Using it as the IR would confuse executable behavior with artifact identity. Keeping Preserves as the canonical IR while allowing WASM adapter targets lets Braid isolate evaluation without weakening losslessness claims.

**Rejected alternative:** Compile Nickel, Rust, or Steel into WASM and call that the lossless interchange. This loses source-level identity and overclaims semantic equivalence.

### 12. Use LLVM only as a compiled-target provenance artifact

**Choice:** Braid does not use LLVM IR as the canonical IR. LLVM may be used later for compiled Rust or adapter target evidence, with receipts binding compiler version, target triple, optimization level, pass pipeline, debug-info availability, source/artifact links, and BLAKE3 hashes.

**Rationale:** LLVM IR is not source-preserving and is intentionally tied to compiler lowering, target behavior, optimization choices, and unstable semantic details such as poison and undef. It can provide useful provenance for compiled artifacts, but using it as Braid's cross-language identity layer would overclaim source losslessness and semantic equivalence.

**Rejected alternative:** Normalize Nickel, Rust, or Steel through LLVM IR and treat that as the lossless bridge. This would erase source identity and conflate compiler artifacts with Braid's Preserves-backed interchange contract.

### 13. Adapt Unison's identity model, not its language runtime

**Choice:** Braid adapts Unison's content-addressed codebase ideas: canonical content owns identity, names are metadata, dependencies are hash-bound, effect needs are explicit labels, refactoring is structured over identities, and durable storage envelopes bind type/schema/codec/hash before decode. Braid still uses BLAKE3 over Braid-owned canonical bytes and does not adopt Unison syntax, runtime, codebase database, cloud/distributed execution, or arbitrary function persistence.

**Rationale:** Unison's big idea maps directly to Braid's losslessness boundary: source names and paths are not identity, while canonical syntax/value/dependency content can be. The adaptation gives Braid a clean model for aliases, stable IDs, dependency closure, cache keys, and migration receipts without claiming to be a programming language or executable codebase.

**Rejected alternative:** Treat Unison itself as Braid's host language or storage format. That would conflict with Braid's Nickel/Rust/Steel adapter scope and Preserves-backed canonical value identity.

### 14. Integrate with Valence and Octet through receipts first

**Choice:** Braid uses Valence closely as the evidence/linkage integration target and Octet closely as the checked Rust/provenance rail, but neither repo owns Braid's canonical IR. Initial integration is by exported Braid receipts, source spans, stable IDs, and fixtures rather than direct crate dependencies.

**Rationale:** Valence already owns Evidence IR identity, roles, typed linkage, spans, and non-claim enforcement across the stack. Braid should make its receipts Valence-friendly so downstream repos can reason about Braid evidence without importing Braid semantics into Valence. Octet already owns Rust source checking and provenance for systems code; Braid should be an Octet consumer when Rust validators/adapters exist. Keeping the coupling receipt-first prevents circular ownership and avoids implying that Octet or Valence proves language semantic equivalence.

**Rejected alternative:** Make Braid a Valence module or an Octet rail from the start. That would blur Braid's source/value interchange ownership and make early format decisions harder to validate independently.

## Risks / Trade-offs

- **Preserves dependency uncertainty:** Mitigate by keeping the first change at the format-contract level and adding fixtures before relying on a concrete library.
- **Reference role confusion:** Mitigate by specifying Preserves as the only canonical authority while MLIR, Valence, Unison, LLVM, WASM, GHC, and Rust MIR keep bounded reference/evidence roles.
- **False lossless claims:** Mitigate with explicit classes, non-claims, and negative fixtures.
- **Schema drift:** Mitigate with BLAKE3 hashes over canonical Preserves bytes and generated artifact freshness checks.
- **Fixture drift:** Mitigate with positive, negative, property-style, and golden-receipt fixtures plus explicit update receipts.
- **Validation command overreach:** Mitigate by making validation non-mutating by default and separating pure validation from effectful adapter execution.
- **Over-broad Steel authority:** Mitigate by requiring DTO-only evaluation and deterministic denied receipts for ambient authority requests.
- **Nickel source confusion:** Mitigate by separating Nickel-authored source from exported runtime projections.
- **Haskell principle overreach:** Mitigate by adapting purity, types, laws, explicit effects, and property-test discipline only as implementation guidance while rejecting Haskell/GHC artifacts as canonical IR.
