# Function-address receipts

Kamacite owns the portable `function-address-receipt` interchange layer for Octet-produced function inventories.

The canonical identity input is Kamacite-owned Preserves bytes (`function-address-receipt-v1.preserves`). JSON is a deterministic compatibility projection that binds back to the canonical receipt identity; it is not the authority for losslessness.

## Boundaries

- Octet owns Rust source-shape extraction and function-address computation.
- Kamacite preserves receipt identity, validates identity domains, duplicate locators, dependency edges, dependency identity kind, caveats, and non-claims.
- Dependency identities preserve their evidence kind, such as `source-token-dependency-set` or `source-shape-dependency-set`; legacy `dependency_closure` slots remain compatibility identity slots unless a kind declares stronger evidence.
- Valence may consume the Kamacite projection as evidence linkage input, including dependency-kind metadata so graph consumers do not infer semantic closure from a hash alone.
- Mantle may bundle validated evidence sidecars without parsing Rust.
- Cairn gates lifecycle/release readiness from receipt metadata and policy.

## Non-claims

A function-address receipt does not prove host-language semantic correctness, compiler correctness, runtime behavior, whole-program safety, verifier soundness, release eligibility, or arbitrary source equivalence.
