# Deferred adapter suites

The walking-skeleton harness now includes one explicit Nickel adapter suite. That suite may run the Nickel reference CLI only through `kamacite-cli validate-nickel-adapter`, records evaluator/source/export identities, and keeps Nickel execution outside `kamacite-core`. The default harness still validates stored evidence without executing external evaluators.

All other execution suites remain deferred: Steel, WASM, LLVM, GHC, Rust MIR, Valence, Octet, and algebraic-effect runtimes are not executed by the harness. Current non-Nickel fixtures validate preserved source bytes, deterministic DTOs, target/provenance metadata, declared capabilities, and receipt/non-claim boundaries only.

A future adapter suite must add all of the following before its behavior is trusted:

- a conformance entry naming the adapter descriptor, input and output schemas, declared effects/capabilities, checked laws, dependencies, and non-claims;
- positive fixtures for supported DTO, source-envelope, target, projection, or receipt behavior;
- negative fixtures for stale hashes, wrong identity domains, unsupported schema versions, lossy projections, undeclared effects, unsupported claims, and malformed inputs;
- golden receipts proving deterministic diagnostics and receipt identity;
- explicit execution mode and capability gates if the suite runs evaluators, compilers, WASM modules, Steel code, or other host-effecting probes.

Reference role boundaries remain unchanged:

- Nickel reference tools own Nickel evaluation/export semantics; Kamacite owns source-envelope preservation and projection receipts.
- Steel is a bounded adapter/probe surface with declared host functions and no ambient authority.
- WASM is an optional sandboxed adapter or target artifact, not canonical IR.
- LLVM, GHC Core/STG, bytecode, Rust MIR, object files, and compiler reports are target/provenance evidence only.
- Valence wraps evidence identities and non-claims without broadening Kamacite claims.
- Octet checks Rust source shape and provenance only.
