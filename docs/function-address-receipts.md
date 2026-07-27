# Function-address receipts

<!-- r[related kamacite.algebraic_effects.semantic_operation_migration] -->

Function-address identities can support a behavior review. They do not replace a semantic operation descriptor or prove behavior correctness.

Kamacite owns the portable `function-address-receipt` interchange layer for Octet-produced function inventories.

The current canonical identity input is canonical packed Preserves produced by the official `preserves` codec (`function-address-receipt-v3.preserves`). Version 3 binds a content-addressed producer profile covering producer kind, tool version or build identity, canonicalizer profile/version, normative options, and producer non-claims. JSON is a deterministic compatibility/debug projection that binds back to both producer-profile and receipt identities; projection drift or JSON-only authority fails validation.

Version 1 remains documented by `function-address-receipt-v1.preserves`, but its identities did not bind `tool_version`. Version 2 bound the producer profile while still using the former labeled deterministic-JSON pseudo-codec. Both identity generations remain recorded compatibility evidence and are never reinterpreted as v3 canonical Preserves. `fixtures/function-address/migration-v1-to-v2.json` retains the compatibility path name while recording the v1, v2, and current v3 identities for the reference fixture.

## Boundaries

- Octet owns Rust source-shape extraction and function-address computation.
- Kamacite preserves receipt and producer-profile identity, validates identity domains, duplicate locators, byte spans against source envelopes, dependency edges, dependency identity kind, caveats, projection binding, normative producer fields, and non-claims.
- Dependency identities preserve their evidence kind, such as `source-token-dependency-set` or `source-shape-dependency-set`; legacy `dependency_closure` slots remain compatibility identity slots unless a kind declares stronger evidence.
- Valence may consume the Kamacite projection as evidence linkage input, including dependency-kind metadata so graph consumers do not infer semantic closure from a hash alone.
- Mantle may bundle validated evidence sidecars without parsing Rust.
- Cairn gates lifecycle/release readiness from receipt metadata and policy.

## Non-claims

A function-address receipt does not prove host-language semantic correctness, compiler correctness, runtime behavior, whole-program safety, verifier soundness, release eligibility, or arbitrary source equivalence.
