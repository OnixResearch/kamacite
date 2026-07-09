## Why

Nickel, Rust, and Steel Scheme already meet through ad hoc JSON DTOs, generated artifacts, and backend receipts, but that is not a lossless any-direction bridge. JSON drops source identity, Nickel authoring semantics, binary/value distinctions, canonical byte identity, and enough type/schema information that downstream adapters can overclaim what round-tripped.

Kamacite is needed to make the losslessness boundary explicit before implementation. The repo should preserve original source when source fidelity matters, use a canonical IR for value-level interchange, and require receipts that state exactly what did and did not survive a conversion.

## What Changes

- Create the Kamacite repository as the OnixResearch home for Nickel/Rust/Steel lossless interchange work.
- Adopt Preserves as the canonical encoded IR candidate for value-level interchange and source envelopes.
- Define a reference IR hierarchy: Preserves is canonical, MLIR is architecture reference, Valence is evidence shape, Unison is identity discipline, Kamacite owns its layered pipeline, and compiler IRs are target/provenance only.
- Define losslessness classes so arbitrary source translation is not confused with source preservation or constrained-DSL round trips.
- Specify adapter boundaries for Nickel, Rust, bounded Steel Scheme evaluation, future WASM targets, LLVM compiled-target provenance artifacts, GHC artifacts, and Rust MIR provenance.
- Split the Cairn delta into focused specs for core IR, source envelopes, adapters, adapter conformance, identity, typed-core discipline, schema vocabulary, fixture corpus, validation command, reference IR boundaries, and stack integration.
- Adapt Haskell-inspired purity, algebraic modeling, explicit effects, lawful interfaces, property-test discipline, and newtype-style identity separation without adopting Haskell/GHC artifacts as Kamacite IR.
- Require positive and negative round-trip fixtures before any conversion claim is accepted.

## Impact

- **Files**: `README.md`, `AGENTS.md`, `.gitignore`, and `cairn/changes/establish-lossless-ir/**`.
- **Testing**: Initial Cairn validation and proposal/design/tasks gates; later implementation must add Preserves, Nickel, Rust, and Steel positive/negative fixtures.
