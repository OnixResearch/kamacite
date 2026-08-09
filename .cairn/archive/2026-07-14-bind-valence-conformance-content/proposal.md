## Why

Valence export checks only that `conformance_identity` uses the adapter domain; it never recomputes that identity from the conformance entry. A reproduced edit added an invented checked law while retaining the old identity, and export still exited successfully while carrying the unchanged conformance identity. The identity therefore does not identify the reviewed conformance content it purports to bind.

## What Changes

- Define canonical conformance-entry bytes and compute the adapter identity from all normative profile fields.
- Require JSON conformance projections to bind those canonical bytes and reject stale, missing, or substituted identities.
- Make the CLI consume the content-addressed conformance artifact rather than a self-asserted identity field alone.
- Migrate checked fixtures and add field-mutation, reordered-set, wrong-domain, malformed-entry, and stale-identity tests.

## Impact

- **Surfaces**: Valence conformance schema, export core, CLI inputs, golden projections, and update receipts.
- **Behavior**: changing checked laws, schemas, domains, coverage, dependencies, or non-claims necessarily changes conformance identity.
- **Validation**: focused conformance identity tests, export fixtures, package validation, and Cairn gates.
