## Why

Kamacite now has deterministic receipt validation, a reviewed fixture manifest, and an executable Nickel adapter suite. Its Valence integration is still only a boundary statement: accepted specs say Valence should wrap Kamacite receipts without broadening claims, but there is no concrete export profile, fixture coverage, or command contract that downstream stack evidence can consume.

A bounded Valence receipt export is the next low-risk integration slice because it is pure projection over existing Kamacite receipts. It exercises stack evidence semantics before adding more executable adapter suites, preserves Kamacite's non-claim discipline, and avoids premature direct crate coupling to Valence while the receipt shape is still stabilizing.

## What Changes

- Define a Valence-friendly evidence export profile for Kamacite receipts that preserves receipt identity, artifact identities, roles, BLAKE3 hashes, source spans when present, adapter/conformance identities, checked laws, diagnostics summaries, and non-claims.
- Add a pure core mapper from Kamacite receipt DTOs to Valence export DTOs; the mapper must not read files, inspect environment, invoke Valence, or broaden claims.
- Add an explicit CLI command or mode that reads reviewed receipt inputs and writes or reports deterministic Valence export projections without mutating source receipts by default.
- Add positive fixtures for preserving receipt facts and negative fixtures for lost non-claims, identity-domain substitution, malformed input, stale export identity, and semantic-equivalence overclaims.
- Document the export profile as a compatibility evidence projection, not Valence ownership of Kamacite semantics and not canonical Kamacite IR.

## Impact

- **Files**: `cairn/changes/add-valence-receipt-export/**`, `cairn/specs/stack-integration/spec.md`, `cairn/specs/adapter-conformance/spec.md`, `cairn/specs/validation-command/spec.md`, `schemas/preserves/**` or equivalent export-profile schema artifacts, `fixtures/manifest.ncl`, `fixtures/manifest.json`, `fixtures/golden/**`, `crates/kamacite-core/**`, `crates/kamacite-cli/**`, `docs/**`, and `README.md`.
- **Testing**: Baseline Rust tests, current harness validation, Nickel adapter validation, and Cairn validation before implementation; final Rust positive/negative tests, fixture/golden validation, Valence export command checks, manifest projection freshness check, and Cairn proposal/design/tasks gates.
