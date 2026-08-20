## Phase 1: Envelope and identity contracts

- [ ] [serial] r[kamacite.qpe.declaration] Add quantity-preserving envelopes to the Kamacite schema vocabulary.
- [ ] [parallel] r[kamacite.qpe.declaration.identity] Add a fixture that requires divergent identity for equal bytes with different meaning.
- [ ] [parallel] r[kamacite.qpe.declaration.untyped] Add a fixture that requires an explicit untyped mark for absent specifications.

## Phase 2: Separation and character

- [ ] [serial] r[kamacite.qpe.separation] Add separation checks that reject meaning swaps between equal-dimension values.
- [ ] [parallel] r[kamacite.qpe.separation.swap] Add a fixture that must fail on a silent meaning swap.
- [ ] [serial] r[kamacite.qpe.character] Add the bounded point, absolute, and difference character with explicit absence marks.
- [ ] [parallel] r[kamacite.qpe.character.declared] Add a fixture that must fail when an undeclared character behaves as a signed difference.
- [ ] [parallel] r[kamacite.qpe.character.point] Add a fixture that must fail on unpermitted point arithmetic.

## Phase 3: Provenance and conversion

- [ ] [serial] r[kamacite.qpe.provenance] Add meaning provenance for declared, assumed, and derived envelopes.
- [ ] [parallel] r[kamacite.qpe.provenance.assume] Add a fixture that must fail on silent assumption promotion.
- [ ] [serial] r[kamacite.qpe.conversion] Add explicit unit and frame conversion with recorded identity.
- [ ] [parallel] r[kamacite.qpe.conversion.explicit] Add a fixture that must preserve named meaning across a declared conversion.
- [ ] [parallel] r[kamacite.qpe.conversion.inversion] Add a fixture that must fail on silent inversion.

## Phase 4: Cross-language preservation and verification

- [ ] [serial] r[kamacite.qpe.cross_language] Preserve declared specifications and values across pinned Nickel, Rust, and Steel adapters.
- [ ] [parallel] r[kamacite.qpe.cross_language.pinned] Add a fixture that must bind all three adapter identities in the receipt.
- [ ] [parallel] r[kamacite.qpe.cross_language.drift] Add a fixture that must fail and name the drifting adapter.
- [ ] [serial] r[kamacite.qpe.verification] Add the focused positive, negative, and boundary matrix and run the rails.
