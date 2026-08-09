## Why

Kamacite currently labels several identity inputs as canonical Preserves binary while constructing them as a text prefix followed by `serde_json` bytes. Other validators hash caller-supplied `canonical_bytes` but validate a separate DTO without proving that the DTO decodes from those bytes. The core has no Preserves codec dependency, so the claimed transfer-syntax stability and canonical authority are not implemented.

## What Changes

- Replace pseudo-Preserves prefixes and opaque byte assertions with a real, bounded Preserves decode/canonical-binary encode path.
- Derive typed artifact fields and identity from the same decoded canonical value instead of trusting parallel DTO and byte inputs.
- Version canonical codecs and migrate affected golden identities explicitly; legacy pseudo-codecs remain untrusted compatibility inputs.
- Add positive text/binary equivalence fixtures and negative malformed, non-canonical, and DTO/bytes-substitution fixtures.

## Impact

- **Surfaces**: canonical values, function-address receipts, evidence-chain envelopes, Trellis proof profiles, FSMs, algebraic effects, documentation artifacts, and golden receipts.
- **Risk reduced**: two materially different artifacts can no longer share an identity by reusing detached `canonical_bytes`.
- **Validation**: focused codec properties, cross-transfer-syntax fixtures, full Rust tests, fixture validation, and Cairn gates.
