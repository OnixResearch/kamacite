# Marigold analysis adapter

Kamacite's `marigold-analysis-v1` pilot preserves one `.marigold` source as a source envelope, invokes exactly `marigold-grammar` `0.1.17` for syntax/operator-shape analysis, normalizes the report into Kamacite-owned DTOs, and emits an advisory receipt.

```bash
nix run .#kamacite -- analyze-marigold \
  --root . \
  --source fixtures/marigold-analysis/valid.marigold \
  --out target/marigold-analysis/receipt.json
```

The command accepts only a root-relative `.marigold` source path. It rejects absolute paths, parent traversal, canonicalized source symlink escapes, receipt-output symlinks, non-UTF-8 input, oversized source input, malformed syntax, unsupported report values, stale identities, wrong identity domains, missing checked laws/non-claims, and semantic claim promotion.

## Artifact roles

| Artifact | Identity domain | Role |
|---|---|---|
| Exact `.marigold` bytes | `source` | Preserved input; not an analyzed or canonical value |
| Analyzer profile/tool/version descriptor | `evaluator` | Exact admitted analyzer cohort |
| Normalized operator-shape report | `projection` | JSON compatibility projection; advisory only |
| Receipt body | `receipt` | Deterministic linkage of the bounded facts above |

Preserves remains Kamacite's canonical value and source-envelope authority. The normalized JSON report is not canonical Kamacite IR and carries no semantic-equivalence authority.

## Execution boundary

The pilot calls only the parser/analyzer entry point. It does not:

- compile generated Rust or invoke Cargo;
- create or use `~/.marigold` state;
- enable Tokio, async-std, or Marigold I/O features;
- run declared stream operations;
- access pipeline-declared files, networks, processes, clocks, randomness, credentials, devices, packages, or mutation targets;
- adopt Marigold's multi-consumer runtime or its backpressure semantics.

The shell performs only the explicitly selected root-confined source read and optional receipt write. Pure report admission, identity construction, deterministic diagnostics, and receipt construction live in `kamacite-core`, which has no Marigold dependency.

## Analyzer limitations and non-claims

Marigold's analyzer classifies recognized stream operators, cardinality expressions, collection stages, and asymptotic shapes. It does not inspect arbitrary called-function bodies. A constant or linear class can therefore be valid for the recognized pipeline shape while saying nothing about the actual cost, allocation, authority, blocking, or nondeterminism of user functions.

A passing receipt proves only:

- exact source bytes were BLAKE3-bound;
- the exact supported analyzer descriptor was BLAKE3-bound;
- the admitted normalized report was BLAKE3-bound;
- checked laws and required non-claims matched the profile; and
- those identities were deterministically linked in one advisory receipt.

It does not prove runtime cost, resource safety, host authority, deterministic execution, replay, compiler correctness, canonical Kamacite IR, or semantic equivalence.

## Promotion and removal criteria

Keep the pilot optional and independently removable. Promotion requires a concrete downstream trace/ETL consumer and evidence that the projection improves admission or review enough to justify its dependency and schema surface. Before promotion, compare a plain Rust implementation and this adapter on one non-sensitive fixture, require byte-identical repeated output ordering, measure queue/memory behavior under a slow secondary consumer, and bind toolchain/result identities into a bounded receipt.

Remove the adapter if no concrete consumer adopts it, Preserves conversion dominates the workflow, analyzer drift makes the profile unstable, or Marigold's coupled consumer/backpressure assumptions conflict with the consumer. Removal must not require migration of canonical Kamacite artifacts.
