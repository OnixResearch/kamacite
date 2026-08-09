## Why

Kamacite can record compiler targets and run bounded adapters, but it has no reviewed boundary for Bend or HVM2. HigherOrderCO presents Bend as a pure, implicitly parallel language over HVM2. That model can fit pure recursive workloads, but the current toolchain also has material limits.

Bend 0.2.38 uses optional typing, 24-bit native numbers, experimental I/O, and immature code generation. Its Rust and generated-C routes share one frontend. HVM2 has a newer project lineage, while Bend still pins HVM2 2.0.22. These facts require a bounded evaluation before any Onix runtime adopts the toolchain.

## What Changes

- Define an evaluation-only Kamacite profile for the exact Bend 0.2.38 and HVM2 2.0.22 cohort.
- Limit the first profile to CPU execution through `bend run-rs` and `bend gen-c`.
- Preserve Bend source, generated artifacts, executable artifacts, observed results, and receipts as separate BLAKE3 identities.
- Require a fully annotated, `Any`-free balanced tree sum over an ordered non-empty power-of-two `u24` leaf vector with no I/O or host authority.
- Consume the exact Trellis bounded modular tree-fold proof artifact and rerun its proof receipt before assigning a verified oracle role.
- Compare both Bend routes with the Trellis verified optimized Rust oracle over the same canonical input.
- Record version-bound interaction counts and bounded performance observations without treating them as portable cost or correctness proof.
- Produce an explicit `adopt`, `defer`, or `reject` decision from a typed Nickel policy and reviewed evidence.
- Require a separate Cairn change in each runtime consumer before any Lattice, Animus, ChaosControl, or other production integration.

## Impact

- **Surfaces**: Bend source envelopes, compiler-target descriptors, evaluator profiles, Trellis proof-handoff descriptors, generated-C target artifacts, run observations, decision receipts, Valence-friendly projections, fixtures, and documentation.
- **Files**: `adapters/`, `schemas/preserves/`, `fixtures/`, `crates/kamacite-core/`, `crates/kamacite-cli/`, `flake.nix`, generated lock state, `README.md`, and this Cairn package.
- **Testing**: focused pure-core and shell tests, Trellis proof-receipt validation, positive and negative fixtures, manifest freshness, adapter validation, lifecycle gates, and relevant Nix checks.
- **Claims**: a passing pilot records one exact cohort and workload. The Trellis proof covers only its local oracle model. Neither the proof nor matching outputs prove Bend, HVM2, generated C, the host compiler, or arbitrary program semantics correct.
