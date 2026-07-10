## Why

Kamacite owns lossless source envelopes, canonical Preserves values, adapter descriptors, and receipt-producing transformations. The tMIR repository is a useful reference for dialect-style operation vocabularies that round-trip through formats even when consumers do not understand them. Kamacite should adapt that pattern for adapter descriptor and receipt fixture design without using tMIR as the canonical IR.

Reference source: `https://github.com/joweeba/tMIR`.

## What Changes

- Define a dialect-roundtrip reference profile for opaque adapter operation vocabularies in Kamacite fixtures.
- Preserve unknown operation namespace, payload version, payload hash, source spans, lowering receipt refs, and non-claims through Preserves canonical bytes.
- Add positive/negative fixtures for known and unknown dialect payloads, lowering receipts, stale payload hashes, lossy JSON projection, and semantic-equivalence overclaims.
- Document that Preserves remains canonical and tMIR is an architecture reference only.

## Impact

- **Surfaces**: reference IR boundary docs, schema vocabulary, fixture corpus, adapter conformance receipts, Valence export profile.
- **Non-claims**: No tMIR dependency, no tMIR parser, no proof-obligation semantics, no source-language semantic equivalence.
- **Validation**: Kamacite fixture validation, Preserves canonical-byte checks, positive/negative fixture tests, lifecycle validation.