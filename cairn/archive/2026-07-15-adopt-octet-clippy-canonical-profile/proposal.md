## Why

Kamacite's canonical Clippy gate denies ordinary warnings but does not enable Clippy restriction lints, which are allow-by-default. Kamacite owns canonical bytes, receipts, and deterministic projections, so Octet's reviewed `canonical-determinism` restriction profile is a suitable compiler-level defense in depth. A clean inventory already passes that profile.

## What Changes

- Promote the six lints in Octet's `canonical-determinism` profile into Kamacite's existing workspace/all-target Nix Clippy gate.
- Keep one named Nix flag set shared by the clean workspace check and the negative fixture check.
- Extend the negative fixture to prove both the existing hidden-panic policy and a selected restriction lint fail closed.
- Keep Clippy evidence bounded to source-shape diagnostics and preserve existing semantic and release non-claims.

## Impact

- **Surfaces**: `flake.nix`, Clippy negative fixture, contributor validation documentation, and packaging/validation requirements.
- **Behavior**: `dbg_macro`, `mem_forget`, `undocumented_unsafe_blocks`, `multiple_unsafe_ops_per_block`, `host_endian_bytes`, and `iter_over_hash_type` become denied in the canonical Clippy gate.
- **Validation**: baseline and post-change Nix Clippy checks, positive/negative restriction-profile evidence, formatting, lifecycle validation, and Cairn gates.
