## Why

Kamacite already treats Preserves as the canonical encoded IR candidate, but the function-address smoke still relied on JSON compatibility output and temporary adapters. Function-address receipts should make Preserves the authoritative portable receipt form, with JSON explicitly demoted to a projection.

## What Changes

- Define the canonical Preserves schema and canonical-binary hashing contract for function-address receipts.
- Compute receipt identity as BLAKE3 over canonical Preserves binary bytes.
- Emit JSON only as a compatibility/debug projection bound to the canonical Preserves receipt hash.
- Add positive and negative fixtures for Preserves round-trip fidelity, schema validation, canonical hash stability, projection drift, and non-claim boundaries.

## Impact

Kamacite becomes the durable interchange authority for function-address receipts, while Octet remains the producer and Valence/Mantle/Cairn remain consumers of typed evidence identities.
