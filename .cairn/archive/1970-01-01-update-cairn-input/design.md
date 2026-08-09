## Context

Kamacite uses Cairn only as an external lifecycle validator. The admitted producer revision rejects a repository that has only the legacy `cairn/` layout. A lock-only update therefore cannot pass Kamacite's existing lifecycle gate.

## Decisions

### Decision: Generate the lock update with Nix

**Choice:** Run a targeted Nix input update for `cairn`. Do not edit `flake.lock` by hand.

**Rationale:** Nix owns the transitive lock graph and records the exact producer revision and NAR identity.

### Decision: Use the hash-bound Cairn migration

**Choice:** Preview `migrate cairn`, review its source and destination entries, execute the exact plan hash, verify old/new byte parity, and then remove the legacy source tree.

**Rationale:** This keeps lifecycle content byte-identical during authority migration and prevents a permanent split between two writable lifecycle roots.

### Decision: Change no accepted product requirement

**Choice:** Use the `no-spec-delta` workflow profile.

**Rationale:** This change updates validation tooling and repository layout. It does not change Kamacite IR, adapter, receipt, or runtime behavior.

## Risks / Trade-offs

- The migration touches many paths even though content stays byte-identical. Rename review and migration receipts bound this risk.
- Historical text can name the legacy layout. Only maintained operator and policy paths change; historical evidence remains faithful.
- Passing validation proves only this exact tool and repository snapshot, not Cairn correctness or release readiness.
