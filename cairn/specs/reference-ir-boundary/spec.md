# Reference IR Boundary Specification

## Purpose

Defines which intermediate representations Kamacite uses directly and which IRs are design references, evidence projections, identity references, or target/provenance artifacts only.

## Requirements

### Requirement: Reference IR hierarchy

r[kamacite.lossless_ir.reference_ir_boundary] Kamacite MUST keep its IR references in explicit roles. Preserves is the canonical Kamacite IR/encoding authority for value identity, source envelopes, and receipts. MLIR MAY be used as an architecture reference for dialect-style organization, verifiers, attributes, locations, and passes. Valence Evidence IR MAY be used as the evidence export shape. Unison MAY be used as an identity-discipline reference. Kamacite MUST own its layered pipeline instead of adopting another language's IR: source envelopes, canonical Preserves values, adapter descriptors, target artifacts, and receipts have separate identities. LLVM IR, WebAssembly, GHC Core, STG, Rust MIR, bytecode, object files, and other compiler IRs MUST be treated only as bounded target/provenance artifacts unless a later accepted change defines a narrower role.

#### Scenario: Preserves remains authority

r[kamacite.lossless_ir.reference_ir_boundary.preserves_authority]

- GIVEN a Kamacite artifact has canonical Preserves bytes and a derived representation in MLIR-like, JSON, Valence, LLVM, WebAssembly, GHC, Rust MIR, or another target form
- WHEN validation computes the artifact identity
- THEN validation MUST use the canonical Preserves bytes and MUST treat the other representation as derived evidence or target material.

#### Scenario: MLIR is architecture reference only

r[kamacite.lossless_ir.reference_ir_boundary.mlir_architecture_only]

- GIVEN Kamacite adopts MLIR concepts such as dialects, verifiers, attributes, locations, or passes
- WHEN those concepts appear in Kamacite schemas, adapters, or validation flows
- THEN they MUST be expressed in Kamacite-owned Preserves schemas and receipts rather than treating MLIR textual or bytecode forms as Kamacite canonical identity.

#### Scenario: Valence export does not broaden claims

r[kamacite.lossless_ir.reference_ir_boundary.valence_evidence_shape]

- GIVEN a Kamacite receipt is projected into a Valence-friendly evidence shape
- WHEN the export is consumed downstream
- THEN the export MUST preserve Kamacite's artifact class, source/canonical hashes, roles, spans, adapter identities, and non-claims without broadening the supported claim.

#### Scenario: Unison identity is adapted, not adopted

r[kamacite.lossless_ir.reference_ir_boundary.unison_identity_reference]

- GIVEN Kamacite adapts content-addressed naming, names-as-metadata, dependency closure, or effect-label ideas from Unison
- WHEN Kamacite computes identity or emits a receipt
- THEN the identity MUST be BLAKE3 over Kamacite-owned canonical bytes and MUST NOT use Unison's codebase, syntax tree hash, runtime, or function serialization as Kamacite authority.

#### Scenario: Kamacite-owned layers do separate jobs

r[kamacite.lossless_ir.reference_ir_boundary.braid_owned_layers]

- GIVEN a Kamacite workflow preserves source, canonicalizes values, runs adapters, records target artifacts, or emits receipts
- WHEN Kamacite computes identities and claims
- THEN source envelopes, canonical Preserves values, adapter descriptors, target artifacts, and receipts MUST remain separate layers with separate BLAKE3 identity domains and explicit derivation receipts between layers.

#### Scenario: Compiler IRs are target evidence only

r[kamacite.lossless_ir.reference_ir_boundary.compiler_irs_not_authority]

- GIVEN a tool provides LLVM IR, WebAssembly, GHC Core, STG, Rust MIR, bytecode, object files, or compiler reports for a Kamacite-related artifact
- WHEN Kamacite classifies the artifact
- THEN Kamacite MUST classify it as target/provenance evidence or a source envelope with explicit non-claims and MUST reject any claim that it is the canonical lossless Kamacite IR for Nickel, Rust, Steel, Haskell, or Kamacite artifacts.
