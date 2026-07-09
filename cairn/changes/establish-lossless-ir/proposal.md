## Why

Nickel, Rust, and Steel Scheme already meet through ad hoc JSON DTOs, generated artifacts, and backend receipts, but that is not a lossless any-direction bridge. JSON drops source identity, Nickel authoring semantics, binary/value distinctions, canonical byte identity, and enough type/schema information that downstream adapters can overclaim what round-tripped.

Braid is needed to make the losslessness boundary explicit before implementation. The repo should preserve original source when source fidelity matters, use a canonical IR for value-level interchange, and require receipts that state exactly what did and did not survive a conversion.

## What Changes

- Create the Braid repository as the OnixResearch home for Nickel/Rust/Steel lossless interchange work.
- Adopt Preserves as the canonical encoded IR candidate for value-level interchange and source envelopes.
- Define losslessness classes so arbitrary source translation is not confused with source preservation or constrained-DSL round trips.
- Specify adapter boundaries for Nickel, Rust, and bounded Steel Scheme evaluation.
- Require positive and negative round-trip fixtures before any conversion claim is accepted.

## Impact

- **Files**: `README.md`, `AGENTS.md`, `.gitignore`, and `cairn/changes/establish-lossless-ir/**`.
- **Testing**: Initial Cairn validation and proposal/design/tasks gates; later implementation must add Preserves, Nickel, Rust, and Steel positive/negative fixtures.
