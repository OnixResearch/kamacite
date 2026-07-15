# Deferred adapter suites

The walking-skeleton harness now includes explicit Nickel and Steel adapter suite slices. The Nickel suite may run the Nickel reference CLI only through `kamacite-cli validate-nickel-adapter`, records evaluator/source/export identities, and keeps Nickel execution outside `kamacite-core`. The Steel suite is narrower: `kamacite-cli validate-steel-adapter` executes admitted DTO probes in a fresh raw Steel engine with only one empty declared module and one pure host function, validates content-addressed input/output contracts, and binds source, evaluator, module descriptor, host-function, capability, output-projection, checked-law, and non-claim identities. Default `validate` preserves prerecorded Steel output only as explicitly non-fresh stored evidence.

All other execution suites remain deferred: WASM, LLVM, GHC, Rust MIR, live Valence ingestion, richer Steel evaluator/module semantics, and algebraic-effect runtimes are not executed by the harness. WIT component profiles and descriptors now have a pure static planning boundary, but missing Mantle component materialization remains an explicit blocker and no Wasmtime shell is admitted. The distinct SpaceWasm MVP boundary binds a real verified Mantle reference bundle and matching Octet static report; those exact artifacts expose only hostless `run`, not Kamacite's canonical-byte ABI, so validation remains `static-only` with no execution plan. Current non-Nickel/non-Steel fixtures validate preserved source bytes, deterministic DTOs, target/provenance metadata, declared capabilities, Valence export projections, and receipt/non-claim boundaries only.

A future adapter suite must add all of the following before its behavior is trusted:

- a conformance entry naming the adapter descriptor, input and output schemas, declared effects/capabilities, checked laws, dependencies, and non-claims;
- positive fixtures for supported DTO, source-envelope, target, projection, or receipt behavior;
- negative fixtures for stale hashes, wrong identity domains, unsupported schema versions, lossy projections, undeclared effects, unsupported claims, and malformed inputs;
- golden receipts proving deterministic diagnostics and receipt identity;
- explicit execution mode and capability gates if the suite runs evaluators, compilers, WASM modules, richer Steel code, or other host-effecting probes.

Reference role boundaries remain unchanged:

- Nickel reference tools own Nickel evaluation/export semantics; Kamacite owns source-envelope preservation and projection receipts.
- Steel is admitted only as a bounded DTO/probe receipt surface with declared host functions, declared imports/modules, output projection identity, and no ambient authority; bytecode, opaque closures, sandboxing, and arbitrary evaluator semantics remain non-claims.
- WASM is an optional sandboxed adapter or target artifact, not canonical IR; WIT components and SpaceWasm core modules remain disjoint profiles.
- LLVM, GHC Core/STG, bytecode, Rust MIR, object files, and compiler reports are target/provenance evidence only.
- Valence export is currently artifacts-first: Kamacite produces deterministic receipt export projections and conformance fixtures without live Valence crates or runtime APIs.
- Octet's Wasm artifact rail records bounded static exact-byte/interface facts only; it does not transfer runtime correctness or execution authority.
