# Proposal: Encode identity domains in Rust types

## Why

Kamacite validates content identities with an `IdentityDomain` runtime field. Source-envelope, canonical-value, projection, target-artifact, adapter, evaluator, compiler, receipt, and effect-operation identities still share one Rust type.

The runtime check protects untrusted input, but it does not stop a wrong-domain value from crossing an internal Rust API. Kamacite needs nominal identity types that match its documented newtype-style discipline.

## What Changes

- Add a private generic `Identity<Domain>` over validated BLAKE3 content.
- Define owned and borrowed aliases for each maintained Kamacite identity domain.
- Separate wire identity records from admitted typed core values.
- Add a closed heterogeneous identity enum for manifests and receipts that carry multiple domains.
- Migrate source-envelope, canonical-value, adapter, target, receipt, effect, and Valence projection APIs in bounded steps.
- Preserve canonical Preserves bytes, JSON projections, legacy identity versions, and explicit algorithm interoperability fields.
- Add compile-pass, compile-fail, parser, migration, canonical-byte, and adapter fixtures.

## Impact

- **Core**: Identity parsing, canonicalization, comparison, adapters, receipts, and domain-specific validators.
- **CLI**: Wire parsing and rendering adapters only.
- **Schemas**: No schema change unless golden evidence requires a separately versioned migration.
- **Active work**: The semantic effect-operation change can use the typed operation-identity alias without transferring effect semantics into the generic identity type.

## Non-goals

- Do not claim semantic equivalence from equal identity bytes.
- Do not replace canonical Preserves authority with Rust layout.
- Do not reinterpret legacy identities under a new domain.
- Do not treat identity possession as capability or authority.
- Do not add live Valence, Nickel, Steel, Wasm, or filesystem effects to the pure core.
