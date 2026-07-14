# Function-address receipts

Kamacite owns the portable `function-address-receipt` interchange layer for Octet-produced function inventories.

The current canonical identity input is Kamacite-owned canonical Preserves binary bytes (`function-address-receipt-v2.preserves`). Version 2 binds a content-addressed producer profile covering producer kind, tool version or build identity, canonicalizer profile/version, normative options, and producer non-claims. JSON is a deterministic compatibility/debug projection that binds back to both producer-profile and receipt identities; projection drift or JSON-only authority fails validation.

Version 1 remains documented by `function-address-receipt-v1.preserves`, but its identities did not bind `tool_version`. A v1 receipt is therefore recorded-only legacy evidence and requires the explicit `tool-version-unbound` caveat; Kamacite does not reinterpret its identity as version-bound. `fixtures/function-address/migration-v1-to-v2.json` records the reviewed old/new identities for the reference fixture.

## Boundaries

- Octet owns Rust source-shape extraction and function-address computation.
- Kamacite preserves receipt and producer-profile identity, validates identity domains, duplicate locators, byte spans against source envelopes, dependency edges, dependency identity kind, caveats, projection binding, normative producer fields, and non-claims.
- Dependency identities preserve their evidence kind, such as `source-token-dependency-set` or `source-shape-dependency-set`; legacy `dependency_closure` slots remain compatibility identity slots unless a kind declares stronger evidence.
- Valence may consume the Kamacite projection as evidence linkage input, including dependency-kind metadata so graph consumers do not infer semantic closure from a hash alone.
- Mantle may bundle validated evidence sidecars without parsing Rust.
- Cairn gates lifecycle/release readiness from receipt metadata and policy.

## Non-claims

A function-address receipt does not prove host-language semantic correctness, compiler correctness, runtime behavior, whole-program safety, verifier soundness, release eligibility, or arbitrary source equivalence.
