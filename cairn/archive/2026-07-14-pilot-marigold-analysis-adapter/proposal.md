## Why

Marigold provides a compact Rust-hosted DSL and an operator-shape analyzer for async stream pipelines. Its exact/upper-bound/unknown cardinality vocabulary and explicit `collects_input` observation are useful advisory inputs for stack experiments, but Marigold does not preserve Kamacite source identity, bind analyzer/toolchain provenance, enforce host authority, or prove whole-program cost, resource safety, determinism, or replay.

Kamacite is the correct layer for a bounded pilot because it can preserve `.marigold` source bytes separately from analyzer output, classify the analyzer report as a compatibility projection, bind the exact evaluator cohort, and emit a receipt with explicit non-claims. The pilot must not make Marigold a canonical IR, runtime, general adapter language, or dependency of `kamacite-core`.

## What Changes

- Add a pure Marigold analysis profile in `kamacite-core` for typed stream summaries, exact source/evaluator/projection identities, advisory-role validation, required non-claims, and deterministic receipt construction.
- Add an explicit `analyze-marigold` CLI shell command that reads one root-confined `.marigold` source, invokes the exactly pinned `marigold-grammar` analyzer in-process without executing the program, revalidates the normalized report through the pure core, and emits or writes a receipt.
- Keep `marigold-grammar` isolated to the AGPL CLI shell; reusable core behavior remains independent of Marigold, async runtimes, filesystem access, process execution, and network access.
- Add positive and negative source fixtures, core and CLI tests, and a focused Nix check proving valid analysis and malformed-source rejection.
- Document analyzer limitations, promotion criteria, and downstream non-claims, and record Marigold in the README reference list.

## Impact

- **Code**: new pure analysis module, narrow CLI command, shell-only dependency pin, and focused Nix check.
- **Artifacts**: `.marigold` source remains a source envelope; analyzer JSON is a BLAKE3-bound compatibility projection; the receipt is advisory evidence only.
- **Testing**: positive valid-pipeline analysis plus negative malformed source, stale identity, wrong identity domain, unsupported analyzer version, malformed report, missing non-claim, and semantic-overclaim cases.
- **Claims**: passing proves source/analyzer/report identity linkage and profile conformance only. It does not prove function-body cost, runtime behavior, resource safety, host authority, deterministic execution, replay, compiler correctness, or semantic equivalence.
