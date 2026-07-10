# Function-address receipts

Kamacite owns the portable `function-address-receipt` interchange layer for Octet-produced function inventories.

The canonical identity input is Kamacite-owned Preserves bytes (`function-address-receipt-v1.preserves`). JSON is a deterministic compatibility projection that binds back to the canonical receipt identity; it is not the authority for losslessness.

## Boundaries

- Octet owns Rust source-shape extraction and function-address computation.
- Kamacite preserves receipt identity, validates identity domains, duplicate locators, dependency edges, caveats, and non-claims.
- Valence may consume the Kamacite projection as evidence linkage input.
- Mantle may bundle validated evidence sidecars without parsing Rust.
- Cairn gates lifecycle/release readiness from receipt metadata and policy.

## Non-claims

A function-address receipt does not prove host-language semantic correctness, compiler correctness, runtime behavior, whole-program safety, verifier soundness, release eligibility, or arbitrary source equivalence.
