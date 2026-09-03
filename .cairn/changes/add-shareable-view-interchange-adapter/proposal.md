## Why

Tile's proposed Shareable View contract separates typed application state, dependencies, actions, presentation profiles, privacy, fallback, and optional live behavior. Tile owns the product meaning, but independent producers and consumers need a canonical cross-language envelope.

Kamacite already owns source envelopes, canonical Preserves values, adapter descriptors, target artifacts, receipts, and narrow round-trip claims. It does not yet define a Shareable View interchange vocabulary.

Without this adapter, Tile, Animus, Steel, and future producers can invent incompatible JSON records or confuse serialization success with UI, action, transport, and authority correctness.

## What Changes

- Add a versioned canonical Preserves vocabulary for Shareable View envelope artifacts and adapter receipts.
- Preserve exact Tile source manifest bytes, schema profile, parser identity, and BLAKE3 source identity separately from canonical target identity.
- Bind application state schema and snapshot references, dependency roles, action catalog references, presentation profiles, privacy facts, modes, fallback classes, and non-claims.
- Define inert live and fork descriptors that contain no bearer token, endpoint secret, callback, host command, or authority grant.
- Add an exact Tile adapter descriptor and a later Animus conversation-projection descriptor against published revisions.
- Validate constrained source-to-envelope and envelope-to-source projections in the pure core.
- Emit deterministic mapping, round-trip, denial, and compatibility receipts.
- Add positive and negative fixtures for identity drift, unknown roles, hidden authority, executable data, privacy downgrade, missing dependencies, and overclaims.

## Capabilities

### New Capabilities

- `kamacite.shareable_view_interchange.boundary`: Kamacite defines a canonical Shareable View interchange layer without owning UI semantics.
- `kamacite.shareable_view_interchange.envelope`: Canonical Preserves records bind the complete declared Shareable View structure.
- `kamacite.shareable_view_interchange.identity`: Source, canonical target, dependency, action, presentation, and external identities remain separate.
- `kamacite.shareable_view_interchange.adapter`: Exact source-profile adapters produce deterministic target artifacts and receipts.
- `kamacite.shareable_view_interchange.non_authority`: Envelope data, live descriptors, and digests grant no runtime or action authority.
- `kamacite.shareable_view_interchange.validation`: Positive, negative, round-trip, and compatibility evidence covers the vocabulary.

## Success Criteria

- One exact Tile Shareable View fixture maps into canonical Preserves bytes and back without losing declared fields.
- Source manifest identity remains distinct from canonical envelope identity.
- Unknown fields, roles, modes, identity domains, and privacy downgrades fail closed.
- Callbacks, closures, commands, credentials, paths, sockets, handles, bearer tokens, and executable payloads fail before target publication.
- A second Animus fixture uses the same envelope vocabulary through a distinct adapter descriptor.
- Receipts preserve layer identities and explicit UI, runtime, transport, authority, and semantic non-claims.

## False Completion

JSON serialization, Preserves decoding, BLAKE3 equality, or a passing schema check is not semantic interoperability.

A canonical envelope does not prove Tile rendering, Animus transcript truth, Molten delivery, application action success, current authority, or fork correctness.

A Wasm, Steel, Rust, or callback artifact inside an envelope must not become executable through this change.

## Impact

- **Core**: Envelope, descriptor, identity, edge, privacy, role, mode, and receipt validation.
- **Schemas**: Canonical Preserves definitions for Shareable View artifacts and receipts.
- **Adapters**: Exact Tile source mapping and independent Animus projection mapping.
- **CLI**: Bounded inspect, validate, adapt, compare, and receipt operations.
- **Fixtures**: Positive and negative source, canonical, round-trip, and compatibility corpora.
- **Evidence**: Deterministic receipts and optional Valence-friendly projections with unchanged claim boundaries.

## Dependencies

Implementation requires an exact published Tile Shareable View contract and fixture corpus. The Animus adapter requires an exact published conversation projection profile.

Kamacite keeps Molten transport, Tile rendering, Animus runtime, Basalt authority, and host effects outside its pure core.

## Non-goals

- Owning Shareable View product semantics, application schemas, UI layout, rendering, accessibility, semantic actions, live transport, caching, retention, or fork behavior.
- Defining a universal widget tree or executing presentation code.
- Serializing callbacks, closures, processes, heaps, PTYs, Wayland objects, GPU resources, credentials, or host handles.
- Claiming arbitrary Nickel, Rust, Steel, Wasm, Tile, Animus, or application semantic equivalence.
- Production interoperability, transport security, runtime correctness, or release readiness.
