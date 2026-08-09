## Design

Kamacite defines a Preserves schema for `EvidenceChainEnvelope`. The envelope contains generic chain metadata and one or more profile payload roots. Profile payloads remain typed and schema-bound, but the envelope provides the common canonical identity and link structure for Valence, Mantle, and Cairn.

### Decisions

- **Preserves canonical bytes own identity.** The envelope hash is BLAKE3 over canonical Preserves binary bytes.
- **Evidence kind is typed.** Function-address, proof, lint, dependency, build, attestation, and lifecycle evidence use explicit kind values rather than free-form labels.
- **Parent links are domain-separated.** Source roots, producer handoff roots, receipt roots, validation roots, release roots, and readiness roots are distinct typed links.
- **Collections use deterministic roots.** Profile payloads expose deterministic Merkle-style roots over sorted typed leaves when they contain sets.
- **JSON is projection only.** JSON output must include the canonical Preserves envelope hash and projection status.
- **Non-claims are hash-bound.** The envelope carries or references a non-claim boundary hash so weakening claims changes identity or fails validation.

### Validation shape

Positive fixtures cover a function-address evidence-chain envelope, canonical hash stability, projection binding, typed parent links, and deterministic collection roots. Negative fixtures cover unknown evidence kind, wrong parent-link domain, projection drift, malformed BLAKE3, missing policy hash, non-canonical bytes, and weakened non-claims.
