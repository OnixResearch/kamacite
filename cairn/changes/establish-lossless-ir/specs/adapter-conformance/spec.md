# Adapter Conformance Specification

## Purpose

Defines the conformance matrix for Braid adapters and target/provenance producers across Nickel, Steel, WebAssembly, LLVM, Haskell/GHC, Rust MIR, Valence exports, and Octet checks.

## Requirements

### Requirement: Adapter conformance matrix

r[braid.lossless_ir.adapter_conformance] Every Braid adapter, projection, target producer, evidence export, or checker integration MUST have a conformance entry that names its adapter descriptor, input schemas, output schemas, declared effects, checked laws, positive fixtures, negative fixtures, target/provenance role when applicable, and explicit non-claims.

#### Scenario: Adapter without conformance entry is untrusted

r[braid.lossless_ir.adapter_conformance.untrusted_without_entry]

- GIVEN a tool emits Braid-like artifacts or receipts but has no conformance entry
- WHEN Braid validates the result
- THEN validation MUST treat the result as untrusted external input and MUST NOT accept adapter-specific claims.

#### Scenario: Conformance entry names non-claims

r[braid.lossless_ir.adapter_conformance.non_claims]

- GIVEN an adapter conformance entry is accepted
- WHEN its receipts are emitted
- THEN each receipt MUST carry the conformance entry identity and MUST preserve the entry's non-claims.

### Requirement: Nickel adapter conformance

r[braid.lossless_ir.nickel_adapter_conformance] Nickel conformance entries MUST preserve Nickel source envelopes, identify the Nickel CLI or crate evaluator used for exports, classify `nickel export` output as derived projection evidence, bind syntax/span parser identity when used, and reject claims that Nickel exports replace source identity.

#### Scenario: Nickel source and export are separate

r[braid.lossless_ir.nickel_adapter_conformance.source_export_separation]

- GIVEN a Nickel fixture includes source bytes and an exported value
- WHEN Braid validates Nickel conformance
- THEN the source envelope identity MUST be distinct from the exported projection identity and the receipt MUST identify the Nickel evaluator or exporter.

#### Scenario: Nickel parser does not prove evaluation

r[braid.lossless_ir.nickel_adapter_conformance.parser_non_claim]

- GIVEN a Nickel span fixture uses tree-sitter Nickel or another parser
- WHEN Braid validates the parser evidence
- THEN the receipt MUST classify parser output as syntax/span evidence and MUST NOT claim Nickel evaluation semantics.

### Requirement: Steel adapter conformance

r[braid.lossless_ir.steel_adapter_conformance] Steel conformance entries MUST declare evaluator identity, DTO input/output schemas, registered host functions, built-in modules, module imports, callable descriptors, contracts, capabilities, effect class, and receipt schema. Ambient authority MUST be denied by default.

#### Scenario: Steel DTO conformance passes without ambient authority

r[braid.lossless_ir.steel_adapter_conformance.dto_only]

- GIVEN a Steel adapter fixture consumes only declared DTO inputs and declared Braid host functions
- WHEN Braid validates the adapter receipt
- THEN validation MAY accept the receipt if evaluator identity, host functions, effects, outputs, and checked laws match the conformance entry.

#### Scenario: Steel ambient authority fails conformance

r[braid.lossless_ir.steel_adapter_conformance.ambient_authority]

- GIVEN a Steel fixture requests filesystem, process, TCP, random, time, package, dylib, credential, mutation, or other undeclared authority
- WHEN Braid validates Steel conformance
- THEN validation MUST fail closed with an undeclared-authority diagnostic.

### Requirement: WASM adapter conformance

r[braid.lossless_ir.wasm_adapter_conformance] WebAssembly conformance entries MUST classify WASM modules or components as sandboxed adapters, validators, or target artifacts. WASM inputs and outputs MUST be Braid IR/source-envelope bytes or declared DTOs plus deterministic receipts, and WASM artifacts MUST NOT be accepted as canonical Braid IR.

#### Scenario: WASM adapter returns Braid receipt

r[braid.lossless_ir.wasm_adapter_conformance.receipt]

- GIVEN a WASM adapter consumes declared Braid inputs under declared capabilities
- WHEN Braid validates its output
- THEN the output MUST include a deterministic Braid receipt over Preserves-backed identities.

#### Scenario: WASM canonical IR claim fails conformance

r[braid.lossless_ir.wasm_adapter_conformance.not_ir]

- GIVEN a WASM conformance entry or receipt claims compiled WASM is Braid canonical IR
- WHEN Braid validates the claim
- THEN validation MUST reject the conformance entry or receipt with a canonical-IR overclaim diagnostic.

### Requirement: Compiler target conformance

r[braid.lossless_ir.compiler_target_conformance] LLVM, GHC, Rust MIR, object, bytecode, and other compiler target conformance entries MUST classify outputs as target/provenance evidence only. Receipts MUST bind source or input identities, compiler/toolchain identity, target metadata, output artifact hashes, reproducibility limits, and semantic-equivalence non-claims.

#### Scenario: LLVM target conformance binds compiler metadata

r[braid.lossless_ir.compiler_target_conformance.llvm_metadata]

- GIVEN an LLVM target artifact is recorded for a Rust validator or adapter
- WHEN Braid validates compiler target conformance
- THEN the receipt MUST bind compiler version, target triple, optimization profile, pass pipeline or profile, debug-info status, input identities, output hashes, and non-claims.

#### Scenario: GHC and Rust MIR are provenance only

r[braid.lossless_ir.compiler_target_conformance.ghc_mir_provenance]

- GIVEN GHC Core, STG, bytecode, interface files, Rust MIR, or rustc compiler reports are recorded for a fixture
- WHEN Braid validates compiler target conformance
- THEN validation MUST classify them as source envelopes or target/provenance artifacts only and MUST reject canonical Braid IR claims.

### Requirement: Evidence integration conformance

r[braid.lossless_ir.evidence_integration_conformance] Valence exports and Octet checks MUST have conformance entries before their results are used in Braid evidence flows. Valence conformance MUST preserve Braid receipt claims and non-claims. Octet conformance MUST classify results as Rust source-shape/provenance evidence only.

#### Scenario: Valence export preserves Braid claim

r[braid.lossless_ir.evidence_integration_conformance.valence]

- GIVEN a Braid receipt is exported to a Valence-friendly evidence bundle
- WHEN Braid validates evidence integration conformance
- THEN the export MUST preserve Braid stable IDs, source spans, roles, hashes, adapter identities, checked laws, and non-claims.

#### Scenario: Octet check does not prove cross-language semantics

r[braid.lossless_ir.evidence_integration_conformance.octet]

- GIVEN Octet checks pass for Braid Rust code
- WHEN Braid records the Octet result
- THEN the receipt MUST classify it as Rust source-shape/provenance evidence and MUST NOT claim Nickel, Steel, Haskell, WASM, LLVM, or arbitrary program semantic equivalence.
