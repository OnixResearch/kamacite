## Why

The generic evidence-chain envelope needs a Trellis proof profile that canonicalizes proof evidence IR without losing original source, verifier receipts, assumptions, or non-claim boundaries. Kamacite should encode that profile as canonical Preserves bytes rather than JSON-only proof metadata.

## What Changes

- Define a Kamacite Preserves profile for Trellis proof evidence-chain envelopes.
- Bind source envelope hash, proof IR hash, verifier receipt hash, assumptions root, dependency root, property/requirement IDs, producer identity, and non-claim boundary hash.
- Support recorded-only and formal-proof candidate roles without deciding downstream promotion.
- Add positive and negative fixtures for canonicalization, projection binding, and assumption drift.

## Impact

Trellis proof evidence becomes a first-class canonical evidence-chain profile while Kamacite remains the portable receipt authority and does not claim verifier soundness or downstream correctness.
