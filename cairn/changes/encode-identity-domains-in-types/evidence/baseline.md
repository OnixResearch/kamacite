# Baseline evidence: identity-domain migration

Task V1 baseline, captured before the nominal-type migration on branch
`drain/encode-identity-domains-in-types`.

## Baseline tree

- Base commit: `d76fe4a` (`origin/main` at drain start).
- Workspace tests at base: 131 `kamacite-core` tests, 34 `kamacite-cli`
  binary tests, all passing (`nix develop . -c cargo test --workspace`).
- Fixture validation at base: `nix run .#kamacite -- validate --root .
  --manifest fixtures/manifest.json` exits 0.

## Canonical-byte stability method

The canonical Preserves codec embeds the serde container name as the record
label. The migration therefore keeps the wire record's serde name as
`Identity` via `#[serde(rename = "Identity")]` on `WireIdentity`. Typed
`Identity<D>` and `AnyIdentity` serialize through the same wire record, so
record labels, field order, and canonical bytes stay unchanged.

Stability was verified by dumping
`kamacite_core::function_address_canonical_bytes` for the
`fixtures/function-address/octet-valid-report.json` cohort before and after
the rename: the bytes are identical (1451 bytes in this checkout, byte count
is input-dependent) once the serde container name is pinned. The CLI test
`migration_evidence_preserves_legacy_pseudo_and_current_identities` compares
the computed receipt identity against the recorded migration fixture and
passes.

## Golden receipts

Existing golden receipts under `fixtures/golden/` and the Preserves migration
evidence under `fixtures/preserves/` are unchanged by this phase. Any
canonical-byte drift fails the CLI migration-evidence test and the
`migrate-preserves` dry-run used in later phases.
