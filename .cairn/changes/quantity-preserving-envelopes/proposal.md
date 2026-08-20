## Why

Kamacite defines the lossless interchange boundary for Nickel-authored configuration and contracts, Rust-owned adapters and validators, and bounded Steel predicates. Its envelopes preserve source bytes, canonical values, and deterministic receipts.

The `mbrock/luv` workshop keeps a source study of semantic arithmetic at `wiki/quantities-and-measurement.org`. It keeps physical dimension, named meaning, unit, affine character, and representation as separate questions. It refuses operations whose values only share a representation. It treats derivation and interpretation as separate steps and marks assumptions as auditable boundaries.

Kamacite can adopt this discipline at its interchange boundary. Quantity specifications become canonical envelope data that participates in the BLAKE3 identity, and adapters preserve meaning across the Nickel, Rust, and Steel interchange without claiming semantic equivalence.

## What Changes

- Add quantity-preserving envelopes as canonical Preserves data with declared quantity specifications.
- Bind the specification into the envelope identity.
- Keep named meaning, dimension, unit, character, and representation separate.
- Record meaning provenance: declared, assumed, or derived.
- Make unit and frame conversion explicit adapter operations with recorded identity.
- Preserve the declared specification and value across pinned Nickel, Rust, and Steel adapters with deterministic receipts.
- Add positive, negative, and boundary fixtures and explicit non-claims.

## Impact

- **Files**: envelope and receipt contracts in Kamacite schemas, preservation and validation logic, fixtures, and this lifecycle package.
- **Testing**: preservation and validation fixtures, workspace tests, Clippy, Octet, Nickel policy checks, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `mbrock/luv` `wiki/quantities-and-measurement.org` at `main`.

## Non-goals

- No runtime quantity algebra, unit evaluation, or physics engine inside Kamacite.
- No silent unit normalization or automatic conversion.
- No claims of arbitrary Nickel, Rust, or Steel semantic equivalence.
- The Nickel quantity-contract authoring vocabulary lives in the sibling `quantity-contracts` component.
