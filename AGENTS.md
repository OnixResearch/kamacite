# Agent Notes

## Repository purpose

`taenite` defines the lossless interchange boundary for Nickel-authored configuration/contracts, Rust-owned adapters and validators, and bounded Steel Scheme predicates or probes.

## Lifecycle

Use native Cairn artifacts under `cairn/` for product decisions:

- accepted specs: `cairn/specs/`
- active changes: `cairn/changes/`
- archived changes: `cairn/archive/`

Run Cairn through the sibling repo unless this repo later grows its own package:

```bash
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- validate --root .
```

## Format boundary

- Preserves is the canonical encoded IR candidate for Taenite value/envelope identity.
- JSON is a compatibility projection only; do not treat JSON export as the authority for losslessness.
- MLIR is an architecture reference for dialect-style vocabularies, verifiers, attributes, locations, and receipt-producing passes; do not treat MLIR text or bytecode as Taenite canonical IR.
- Use Taenite-owned layering instead of another language's IR: source envelopes, canonical Preserves values, adapter descriptors, target artifacts, and receipts have separate identities and non-claims.
- Nickel remains the preferred human-authored contract/config language.
- Nickel integration must preserve source envelopes separately from `nickel export` projections; use Nickel's reference CLI/crates for evaluation semantics instead of reimplementing them.
- Rust owns pure validation cores, canonicalization, hashing, adapter DTOs, and receipts.
- Steel Scheme is bounded adapter/probe logic only; do not grant ambient filesystem, process, network, credential, clock, random, dylib, package-manager, or mutation authority by default.
- Steel integration should use explicit Engine/evaluator identity, typed host-function registration, declared built-in modules, module contracts, and deterministic receipts. Do not treat Steel bytecode, opaque closures, or JSON strings as Taenite canonical IR.
- Taenite owns algebraic-effect descriptors and receipts as canonical data: effect signatures, rows, summaries, handler descriptors, replay profiles, and effect-log projections. Do not treat this as a Taenite evaluator, runtime, or host-authority grant; undeclared effects and capabilities fail closed.
- WebAssembly is an optional adapter/validator target, not Taenite's IR. WASM inputs and outputs must remain Taenite IR/source-envelope bytes plus deterministic receipts.
- LLVM is an optional compiled-target/provenance artifact, not Taenite's IR. LLVM receipts must bind compiler version, target triple, optimization level, pass pipeline, debug-info availability, and BLAKE3 artifact hashes without claiming source losslessness or semantic equivalence.
- Rust MIR, rustc reports, GHC Core, STG, bytecode, interface files, object files, and compiler reports are target/provenance evidence only unless a later Cairn narrows their role.
- Use Haskell-inspired design discipline only as implementation guidance: pure core/effect shell, algebraic artifact alternatives, explicit effect data, law-based adapters, property-style fixtures, typed identity domains, and total diagnostics. Do not treat Haskell source, GHC Core, STG, bytecode, interface files, laziness, typeclass resolution, or monad terminology as Taenite IR or receipt authority.
- Follow Unison-inspired identity discipline: canonical content owns identity; names, paths, aliases, and labels are metadata. Use BLAKE3 over Taenite canonical bytes rather than adopting another language's hash format.
- Valence is the evidence/linkage integration target for Taenite receipts; Taenite still owns source preservation, value interchange, adapter classes, and round-trip claims.
- Octet is the checked Rust/provenance rail for future Taenite Rust code; do not treat Octet findings as proof of Nickel/Rust/Steel semantic equivalence.

## Losslessness claims

Never claim arbitrary source-to-source translation across Nickel, Rust, and Steel. Preserve original source bytes and BLAKE3 hashes, then make narrower claims about parsed projections or constrained Taenite IR values.

Do not call Taenite a programming language unless a later Cairn change defines a human-authored syntax, grammar, evaluator, type rules, and semantics. Prefer "IR", "source-envelope format", "receipt contract", or "constrained policy vocabulary" for the current scope.

Every new conversion or validation behavior needs positive and negative fixtures before it is considered implemented.
