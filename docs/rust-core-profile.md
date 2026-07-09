# Kamacite Rust Core Profile

The Kamacite Rust Core Profile is a restricted implementation profile for trusted Kamacite validation code. It is not a Rust dialect, compiler qualification, safety certification, Verus proof claim, or proof of arbitrary Rust program semantics.

## Core role

`kamacite-core` is a pure deterministic admission controller over explicit bytes and DTOs. It may validate source envelopes, typed identity domains, canonical bytes, adapter declarations, target evidence, algebraic-effect descriptors, diagnostics, and receipts. It must not perform filesystem traversal, environment reads, process execution, evaluator calls, compiler calls, network access, clock or random access, credential lookup, mutation, or presentation.

Crate rails:

- `#![forbid(unsafe_code)]` in trusted Rust crates.
- No panic-prone helpers in validation paths: no unchecked unwrap-style extraction, debug-only placeholders, `todo`, `unreachable`, or process exits in the core.
- Core APIs return domain diagnostics and deterministic receipt data rather than shell-oriented opaque errors.
- Identity domains are typed: source, canonical value, projection, target artifact, adapter, evaluator, compiler, receipt, algebraic-effect, effect-operation, and effect-log identities are not interchangeable.

## Shell role

`kamacite-cli` is allowed to read files, materialize fixture bytes, run explicitly declared Nickel reference-export fixtures, compare golden receipts, write explicit update artifacts, and render JSON. Shells normalize observations into DTOs before calling the core. If a future shell admits filesystem-backed user paths, it should use `cap-std`, WASI, or equivalent root-capability APIs and must keep capability handles out of the core.

The current shell includes negative fixture evidence for parent traversal, absolute paths, symlink escape, and undeclared filesystem authority. These are admission fixtures for future capability-safe shells; they are not a sandbox claim.

## Prior-art roles

Kamacite adapts prior art by role only:

- Ferrocene, Sealed Rust, Verus, Kani, Prusti, Creusot, and Trellis inform restricted/profile/proof discipline but do not certify Kamacite Rust.
- HIF informs explicit host-function and bounded-machine modeling without donating bytecode, serialization, or runtime semantics.
- `cap-std`, WASI, WASI Virt, and Wasmtime inform shell/adaptor capability boundaries only.
- Oxide schema and identity projects inform typed identity, duplicate rejection, schema translation honesty, and structural comparison.
- Octet, Clippy, Dylint, cargo-geiger, cargo-vet, cargo-deny, and cargo-auditable are source-shape or provenance evidence only. They do not validate Kamacite receipt semantics.
