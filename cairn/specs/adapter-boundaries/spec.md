# Adapter Boundaries Specification

## Purpose

Defines Kamacite adapter trust boundaries for Nickel, Rust, Steel, WebAssembly targets, and LLVM compiled-artifact evidence.

## Requirements

### Requirement: Nickel, Rust, and Steel adapter boundaries

r[kamacite.lossless_ir.adapter_boundaries] Kamacite adapters for Nickel, Rust, and Steel MUST cross the trust boundary through Kamacite IR values, source envelopes, and deterministic receipts rather than ambient host-language state.

#### Scenario: Nickel export is a derived projection

r[kamacite.lossless_ir.adapter_boundaries.nickel_projection]

- GIVEN a Nickel-authored contract exports a runtime value
- WHEN Kamacite ingests the export
- THEN the receipt MUST distinguish the Nickel source envelope from the exported Kamacite IR or compatibility JSON projection.

#### Scenario: Steel adapter has no ambient authority

r[kamacite.lossless_ir.adapter_boundaries.steel_no_ambient_authority]

- GIVEN a Steel adapter requests filesystem, process, network, credential, clock, or mutation authority not declared in the adapter contract
- WHEN Kamacite validates the evaluation request or receipt
- THEN validation MUST deny the request or receipt deterministically without evaluating the undeclared host action.

### Requirement: Steel adapter integration boundary

r[kamacite.lossless_ir.steel_adapter_boundary] Kamacite MUST treat Steel as a bounded adapter/probe evaluator rather than canonical IR. Steel adapters MUST declare evaluator identity, source identity, input DTO schema, host functions, module imports, required capabilities, effect class, and receipt schema before their results are trusted.

#### Scenario: Steel host functions are declared

r[kamacite.lossless_ir.steel_adapter_boundary.host_functions]

- GIVEN a Steel adapter calls a Rust host function registered through the evaluator
- WHEN Kamacite validates the adapter descriptor or receipt
- THEN the host function MUST be declared with its input/output DTO contract, effect class, and required capabilities before evaluation is trusted.

#### Scenario: Steel contracts stay boundary-local

r[kamacite.lossless_ir.steel_adapter_boundary.contract_exports]

- GIVEN a Steel module exports functions with contracts or higher-order callables
- WHEN Kamacite records the adapter output
- THEN the receipt MUST bind exported callable descriptors by stable id, arity, source hash, input/output contracts, and non-claims instead of exporting opaque closures as canonical Kamacite values.

#### Scenario: Ambient Steel libraries fail closed

r[kamacite.lossless_ir.steel_adapter_boundary.ambient_libraries]

- GIVEN Steel code imports or requests filesystem, process, TCP, random, time, package, dylib, credential, or mutation authority not declared in the adapter contract
- WHEN Kamacite validates the adapter request or receipt
- THEN validation MUST fail closed with a deterministic undeclared-authority diagnostic.

### Requirement: WASM target boundary

r[kamacite.lossless_ir.wasm_target_boundary] Kamacite MUST treat WebAssembly as an optional adapter, validator, or conformance target rather than the canonical IR. WASM components or modules MAY consume Kamacite IR/source-envelope bytes and emit deterministic receipts, but they MUST NOT replace Preserves as the authoritative value or source-envelope identity.

#### Scenario: WASM adapter stays receipt-bounded

r[kamacite.lossless_ir.wasm_target_boundary.adapter_receipt]

- GIVEN a Kamacite adapter is packaged as a WebAssembly module or component
- WHEN Kamacite invokes or validates the adapter
- THEN the adapter MUST receive only declared inputs and capabilities and MUST return a deterministic Kamacite receipt over Preserves-backed identity.

#### Scenario: WASM is rejected as canonical IR

r[kamacite.lossless_ir.wasm_target_boundary.not_ir]

- GIVEN a tool claims that compiled WebAssembly is the lossless representation of arbitrary Nickel, Rust, or Steel source
- WHEN Kamacite classifies the claim
- THEN validation MUST reject the canonical-IR claim and MAY accept the WebAssembly artifact only as a target artifact or source envelope with explicit non-claims.

### Requirement: LLVM target artifact boundary

r[kamacite.lossless_ir.llvm_target_boundary] Kamacite MUST treat LLVM IR, bitcode, object files, and compiler reports as compiled target artifacts or provenance evidence rather than canonical IR. LLVM-backed receipts MUST bind compiler identity, target triple, optimization level, pass pipeline, debug-info availability, source/artifact links, and BLAKE3 artifact hashes before target evidence is trusted.

#### Scenario: LLVM artifact receipt is bounded

r[kamacite.lossless_ir.llvm_target_boundary.artifact_receipt]

- GIVEN Kamacite records LLVM IR, bitcode, object output, or a compiler report for a Rust validator or adapter
- WHEN the target-artifact receipt is emitted
- THEN the receipt MUST include compiler/toolchain identity, target triple, optimization level, pass pipeline or profile, debug-info status, source envelope or input artifact identity, output artifact hash, and explicit non-claims.

#### Scenario: LLVM is rejected as canonical IR

r[kamacite.lossless_ir.llvm_target_boundary.not_ir]

- GIVEN a tool claims LLVM IR is the lossless cross-language representation of arbitrary Nickel, Rust, or Steel source
- WHEN Kamacite classifies the claim
- THEN validation MUST reject the canonical-IR claim and MAY accept the LLVM artifact only as target evidence with explicit toolchain and semantic-equivalence non-claims.
