## Why

Function-address receipts should not be the only Preserves-backed stack contract. Kamacite needs a generic canonical evidence-chain envelope that can carry multiple evidence profiles while preserving content identity, typed parent links, policy hashes, projections, and non-claim boundaries.

## What Changes

- Define a canonical Preserves evidence-chain envelope owned by Kamacite.
- Compute envelope identity as BLAKE3 over canonical Preserves binary bytes.
- Support typed parent links, evidence kinds, profile payload roots, policy/profile hashes, Merkle roots, projections, and non-claim boundary hashes.
- Treat function-address as the first concrete profile carried by the generic envelope.

## Impact

Kamacite becomes the portable canonical receipt authority for stack evidence chains, not just function-address receipts. Future evidence kinds can share one envelope while keeping profile-specific schemas separate.
