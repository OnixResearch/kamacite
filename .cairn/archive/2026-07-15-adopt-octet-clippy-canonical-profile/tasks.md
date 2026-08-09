## Phase 1: Policy and fixture

- [x] [serial] Add one named `canonical-determinism` Clippy restriction flag set and apply it to the existing workspace/all-target Nix Clippy gate. r[kamacite.packaging.clippy_restriction_profile] r[kamacite.packaging.clippy_restriction_profile.positive]
- [x] [serial] Extend the negative fixture and check so both hidden panic and selected restriction diagnostics are required. r[kamacite.packaging.clippy_restriction_profile.negative]

## Phase 2: Documentation and validation

- [x] [parallel] Document the adopted profile, exact command, narrow exception posture, Octet relationship, and non-claims. r[kamacite.packaging.clippy_restriction_profile.boundaries]
- [x] [serial] Run formatting, positive and negative Nix Clippy checks, the Octet profile evidence rail, lifecycle validation, and Cairn gates; record exact evidence. r[kamacite.packaging.clippy_restriction_profile.positive] r[kamacite.packaging.clippy_restriction_profile.negative]
