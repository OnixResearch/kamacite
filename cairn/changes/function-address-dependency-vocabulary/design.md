## Design

The receipt model keeps BLAKE3 identity domains explicit and adds a dependency identity kind or bounded vocabulary entry that says what the producer supplied. For Octet v1 inputs this kind is source-token/source-shape dependency set. A future producer may supply stronger closure evidence only with a separate kind, schema, and non-claim boundary.

### Decisions

1. **Kind before strength.** Receipts record the dependency identity kind before any consumer interprets the hash.
2. **Compatibility is controlled.** Existing `dependency_closure` fields remain readable but are documented as recorded identity slots unless a kind declares stronger evidence.
3. **Overclaim text is rejected.** Receipts fail closed if claims promote token dependency sets to Rust semantic dependency closure.
4. **Valence projection preserves kind.** Downstream projections carry the dependency identity kind so graph consumers do not infer closure from a hash alone.

### Validation shape

Positive fixtures cover Octet source-token dependency records. Negative fixtures cover missing kind, wrong identity domain, kind/hash mismatch, and claims that the dependency identity proves semantic call graph closure or whole-program safety.
