## Why

The smoke path needed custom Rust glue to turn Octet `function-addresses.json` into a Kamacite portable function-address receipt and Valence projection. Kamacite owns the interchange layer, so this conversion should be a first-class, testable CLI surface instead of one-off adapter code.

## What Changes

- Add a Kamacite CLI command that validates Octet function-address reports and emits a portable Kamacite receipt.
- Add an option to emit the Kamacite-to-Valence projection alongside the receipt.
- Preserve Preserves/canonical identity as the authority and JSON as compatibility projection.
- Add positive and negative fixtures for valid Octet reports, missing source envelopes, malformed BLAKE3, stale source identity, missing non-claims, and overclaims.

## Impact

Downstream Valence, Mantle, and Cairn smoke fixtures can consume a repo-owned Kamacite receipt command rather than embedding Kamacite conversion logic elsewhere.
