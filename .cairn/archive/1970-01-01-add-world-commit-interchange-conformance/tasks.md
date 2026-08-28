## Phase 1: Pin and model the cohort

- [x] [serial] Record the immutable Molten source revision, schema, codec, domain tag, fixture manifest, license, and non-claims. r[kamacite.world_commit_conformance.cohort]
- [x] [serial] Record baseline Preserves codec, Nickel adapter, Steel adapter, fixture corpus, manifest, and Valence export tests. r[kamacite.world_commit_conformance.verification]
- [x] [depends:world-commit-cohort] Define the world-commit dialect, adapter descriptor, typed root-domain inventory, bounds, checked laws, parity result, diagnostic, and receipt DTOs. r[kamacite.world_commit_conformance.entry] r[kamacite.world_commit_conformance.domains]
- [x] [depends:world-commit-dialect] Implement pure canonical-value admission, domain checks, bound checks, exact-byte comparison, adapter-law comparison, and deterministic diagnostics. r[kamacite.world_commit_conformance.parity]
- [x] [parallel] Add source envelopes and identities for authoritative Rust schema, codec fixture source, Nickel contracts, and Steel adapter source. r[kamacite.world_commit_conformance.sources]

## Phase 2: Adapter fixtures

- [x] [depends:world-commit-parity-core] Import the Rust-produced canonical packed Preserves corpus through an immutable fixture manifest. r[kamacite.world_commit_conformance.rust]
- [x] [depends:world-commit-parity-core] Add Nickel contract-backed review projections with exact evaluator identity and separate source and export identities. r[kamacite.world_commit_conformance.nickel]
- [x] [depends:world-commit-parity-core] Add a fresh no-kernel Steel adapter with declared DTOs, pure host functions, modules, contracts, effects, limits, and evaluator identity. r[kamacite.world_commit_conformance.steel]
- [x] [depends:world-commit-adapters] Add deterministic three-adapter parity receipts and optional Valence exports that preserve non-claims. r[kamacite.world_commit_conformance.receipt]
- [x] [parallel] Add dry-run schema-cohort update and explicit convergence workflows. r[kamacite.world_commit_conformance.update]

## Phase 3: Verification and documentation

- [x] [parallel] Add positive vectors for all typed roots, ordered parents, logical and opaque profiles, empty and bounded collections, and detached evidence exclusion. r[kamacite.world_commit_conformance.verification]
- [x] [parallel] Add negative malformed, noncanonical, unknown-version, wrong-domain, duplicate, order drift, bound overflow, stale schema, codec drift, embedded signature, secret value, undeclared Steel authority, JSON-as-authority, and semantic-equivalence-overclaim vectors. r[kamacite.world_commit_conformance.verification]
- [x] [serial] Document product ownership, canonical bytes, adapter roles, source envelopes, update flow, consumer pinning, and non-claims. r[kamacite.world_commit_conformance.receipt]
- [x] [depends:world-commit-conformance-verification] Run Kamacite validation, Nickel and Steel adapter rails, manifest freshness, Valence export fixtures, focused Cargo tests, Octet, Clippy with warnings denied, Cairn validation and gates, and `nix flake check`. r[kamacite.world_commit_conformance.verification]
  - Passed 240 Rust tests, 8 doctests, formatting, and workspace Clippy with warnings denied and the canonical-determinism lint flags.
  - Passed two fresh sandboxed Nickel and Steel conformance passes, two identical regeneration passes, normal freshness validation, and the unsafe-update denial.
  - Passed the pinned full-catalog Octet deny-all gate across the workspace, all targets, and all features with zero findings.
  - Passed Cairn validation plus proposal, design, and task gates with the current shared Cairn policy.
  - Passed focused Nix checks and the complete `nix flake check -L` gate on `x86_64-linux`.
  - Recorded commands, identities, outcomes, and the bounded policy-export caveat in `evidence/verification.md`.
