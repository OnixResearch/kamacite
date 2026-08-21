## Why

Kamacite is the lossless interchange boundary for the stack. Traces emitted by external model checkers arrive in the ITF (Informal Trace Format), a JSON representation with no stack identity. Without an adapter, these traces cannot enter the evidence chain: they have no Preserves binding, no BLAKE3 identity, and no fail-closed admission, so consumers either trust raw foreign JSON or reject the evidence entirely.

The model-checked artifact evidence layer (Valence) and the parity rail (Octet) need a bounded ITF-to-Preserves adapter with adapter receipts, layer-separated identity, and explicit non-claims.

## What Changes

- Add a versioned ITF-to-Preserves exchange adapter descriptor for model-checked trace artifacts.
- Preserve ITF source bytes as a source envelope and bind ITF version, parser/tool identity, trace, and converted Preserves value as separate BLAKE3 identities.
- Validate traces in pure core against the adapter schemas and fail closed on malformed, stale, or tampered input.
- Emit deterministic adapter receipts binding source, conversion, schemas, and non-claims.
- Add positive and negative fixtures for valid conversions and for malformed, stale, tampered, and overclaim cases.

## Impact

- **Files**: Kamacite adapter descriptor, Preserves schemas, pure validation, receipts, fixtures, and docs.
- **Testing**: Positive ITF-to-Preserves round trips plus negative malformed, wrong-version, schema-mismatch, tamper, and equivalence-overclaim cases.
- **Boundary**: A passing adapter receipt proves the declared byte-level conversion and identity binding only. It does not prove semantic equivalence between ITF and Preserves meaning, checker correctness, or trace validity beyond the schema.
