## Design

1. Add a `DialectRoundtripProfile` fixture profile for adapter operation vocabularies. Fields include namespace, operation name, payload version, canonical payload hash, source envelope ref, source spans, lowering receipt refs, compatibility projection refs, and non-claims.
2. Encode the authoritative representation as Kamacite Preserves bytes. JSON remains a compatibility projection and must not be the source of truth for roundtrip identity.
3. Treat unknown dialects as opaque payloads. Validation preserves identity and payload hash but does not interpret semantics or lower them.
4. Treat known example dialects as fixture vocabulary only. Lowering receipts can prove receipt shape and identity linkage, not semantic equivalence across languages.
5. Add negative fixtures for stale payload hash, unsupported payload version, missing source envelope, lossy JSON-only projection, cross-dialect payload substitution, and proof/semantic overclaim.
6. Add Valence export mapping fixtures only after the Kamacite profile validates locally.

## Non-Goals

- No adoption of tMIR as Kamacite IR or as a dependency.
- No tMIR text/binary/JSON/MessagePack parser or serializer.
- No proof that a dialect lowering preserves source semantics, proof obligations, or runtime behavior.