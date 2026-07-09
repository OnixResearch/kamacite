## Why

Kamacite is about to grow Rust validators, adapter DTOs, canonicalization helpers, and receipt code. If the first Rust surface is too permissive, later adapters can accidentally inherit nondeterministic behavior, ambient host authority, interchangeable hash strings, lossy projection shortcuts, or diagnostics that depend on runtime context.

The existing lossless-IR Cairn says Rust should own pure validation cores and typed identities. This change ratchets that guidance into implementation admission rules for Kamacite Rust code before the first crate lands: Rust is a deterministic admission controller over explicit bytes, schemas, identities, and receipts, not an interpreter, evaluator, or ambient automation host.

## What Changes

- Define the Kamacite Rust Core Profile as a reviewed implementation profile, inspired by Ferrocene/Sealed Rust, Verus, HIF, capability-oriented Rust, Wasmtime determinism, and Oxide schema/identity projects, without claiming a new Rust dialect, compiler qualification, or semantic proof of arbitrary programs.
- Define a Rust role boundary: `kamacite-core` owns pure deterministic validation, canonicalization, hash-material construction, artifact classification, adapter DTO checks, and receipt construction; effectful IO belongs in a thin shell such as `kamacite-cli`.
- Require typed identity domains instead of raw string hashes for source, canonical value, projection, target artifact, adapter, evaluator, compiler, and receipt identities.
- Require closed artifact alternatives and fail-closed handling for unknown schema versions, codecs, artifact classes, target kinds, adapter capabilities, and unsupported claims.
- Require Preserves canonical bytes and BLAKE3 hashes to be authoritative where Kamacite controls identity; JSON remains a compatibility projection with its own identity domain.
- Require deterministic diagnostics as data, deterministic ordering for canonical output, and no panics, silent defaults, truncation, or cross-domain coercions in validation paths.
- Require explicit effect/capability declarations for any adapter-facing Rust surface, with denied-by-default behavior for filesystem, process, network, clock, randomness, credentials, mutation, compiler, or evaluator authority.
- Treat `cap-std` or equivalent capability-root APIs as the preferred pattern for shell-only filesystem/network authority, while preserving the rule that the core receives normalized DTOs and never receives capability handles.
- Require positive, negative, stale-hash, wrong-identity-domain, unsupported-version, and golden-receipt fixtures before trusting new Rust validation behavior.
- Define crate-level rails for the first Rust slice: unsafe code forbidden, panic-prone helpers denied in core paths, domain error enums in the core, and shell-only generalized error plumbing.

## Impact

- **Files**: `cairn/changes/ratchet-rust-determinism/**`.
- **Testing**: Cairn validation plus proposal/design/tasks gates now; later implementation must add Rust crates, lint rails, positive/negative fixtures, capability-shell escape/undeclared-authority fixtures, golden receipts, and deterministic validation receipts before the Rust surface is considered trusted.
