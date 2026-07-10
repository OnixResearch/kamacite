## Why

Octet can produce Rust function-address evidence, but the stack needs a source-neutral receipt contract before Valence, Mantle, or Cairn consume it. If each producer invents its own JSON shape, function identity will drift across Rust, Nickel, Steel, and later adapter surfaces.

Kamacite should own the portable receipt layer: source envelopes, canonical Preserves values, adapter descriptors, identity domains, spans, dependency identities, and non-claims. That lets Octet remain the Rust analyzer while Kamacite preserves the function-address inventory as a lossless interchange artifact.

## What Changes

- Define a `function-address-receipt` artifact class with source envelopes, function entries, local function addresses, optional dependency-closure identities, caveats, producer roles, and non-claims.
- Add a Preserves-backed canonical identity contract for function-address receipts, with JSON only as a compatibility projection.
- Validate Octet-produced function-address receipts as producer input without treating Octet findings as Kamacite semantic proof.
- Add Valence-friendly export projections that preserve Kamacite receipt identity, source spans, artifact roles, BLAKE3 identities, and non-claims.
- Add positive and negative fixtures for valid receipts, stale identities, malformed BLAKE3 digests, locator collisions, weakened non-claims, and overclaims.

## Impact

- **Files**: Kamacite core receipt models, Preserves schema artifacts, fixture manifest entries, golden receipts/projections, docs, and Cairn lifecycle artifacts.
- **Testing**: default validation, focused function-address receipt validation, Octet interop fixtures, Valence export fixtures, negative stale/malformed/overclaim cases, and Cairn validation/gates before archive.
