## Design

The function-address receipt core should build a typed receipt value, encode it as Preserves, canonicalize it with Preserves canonical binary form, and hash those canonical bytes with BLAKE3. JSON output is generated from the typed receipt and includes the canonical Preserves receipt identity so consumers can detect projection drift.

### Decisions

1. **Canonical Preserves bytes are authoritative.** Receipt identity is BLAKE3 over canonical Preserves binary bytes, not JSON bytes.
2. **Schema is versioned.** The Preserves schema names the function-address receipt version, digest domains, dependency identity vocabulary, span fields, source envelopes, producer metadata, and non-claims.
3. **JSON is non-authoritative.** JSON projections exist for CLI compatibility and inspection only; they must bind back to the canonical Preserves receipt hash.
4. **Round-trip fidelity is tested.** Preserves text/binary conversion and JSON projection must preserve typed receipt content without changing canonical identity.
5. **No semantic promotion.** Canonicalization proves receipt identity and schema conformance only.

### Validation shape

Positive fixtures cover valid Octet handoff to Preserves receipt, canonical binary hash stability, Preserves text/binary round trip, and JSON projection binding. Negative fixtures cover malformed Preserves values, schema version mismatch, projection drift, non-canonical hash input, missing non-claims, and source-token dependency overclaims.
