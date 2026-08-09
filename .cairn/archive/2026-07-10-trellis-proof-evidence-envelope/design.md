## Design

Kamacite adds a Trellis proof profile inside the canonical evidence-chain envelope. The profile payload is a typed Preserves value with references to source envelopes, proof IR, verifier receipt, assumptions, dependencies, and property IDs. The envelope hash remains BLAKE3 over canonical Preserves binary bytes.

### Decisions

- **Profile is canonical Preserves.** Trellis proof evidence is encoded as Preserves, not JSON authority.
- **Source and IR are both linked.** The profile binds original source bytes and canonical proof IR so downstream consumers can audit and query without losing rerun inputs.
- **Verifier receipt is distinct.** The Verus/toolchain result identity is a separate typed parent link from proof IR identity.
- **Role is bounded.** Kamacite records requested/producer role but does not decide whether downstream tools count it as formal proof.
- **Assumptions are hash-rooted.** Trusted boundaries, proof gaps, external bodies, and dependency sets contribute to explicit roots.
- **Projection drift fails closed.** JSON projections bind to canonical envelope hash and profile roots.

### Validation shape

Positive fixtures cover a verified Trellis proof profile, a recorded-only reference profile, stable canonical hash, and JSON projection binding. Negative fixtures cover missing source envelope, missing verifier receipt, malformed property IDs, assumption drift, wrong role domain, projection drift, and overclaims.
