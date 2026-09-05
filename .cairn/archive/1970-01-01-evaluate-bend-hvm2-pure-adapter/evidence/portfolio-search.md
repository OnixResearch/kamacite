# Bend and HVM2 Pilot Search Record

## Goal

Implement one exact CPU-only Bend 0.2.38 and HVM2 2.0.22 evaluation. Produce a reviewed `adopt`, `defer`, or `reject` decision.

## Completion evidence

Completion requires all of these results:

- The Nix toolchain pins the reviewed Bend, HVM2, Trellis, and C compiler sources.
- A typed Nickel policy exists before measured execution.
- The pure core rejects invalid profiles, inputs, proof handoffs, observations, receipts, and claims.
- The shell executes the Trellis oracle, `bend run-rs`, `bend gen-c`, the C compiler, and the generated executable under declared limits.
- All three routes consume the same canonical leaves. An incomplete route produces an exact `defer` blocker instead of invented output.
- The evidence records separate BLAKE3 identities for every artifact layer.
- Positive and negative fixtures reach their declared boundaries.
- The Trellis proof receipt runs through the pinned Nix input.
- The final repository and Cairn gates pass, except for exact inherited baseline errors recorded before this change.

## False completion cases

These outputs do not complete the pilot:

- Static schemas without actual execution.
- Agreement between the two Bend routes without the Trellis oracle.
- A proof citation without a pinned proof validation result.
- Process exit success with missing or malformed output.
- An `adopt` decision without stable measurements and the declared performance rule.
- A receipt that merges source, generated C, executable, output, or proof identities.
- A result that grants runtime, compiler, semantic, sandbox, or release authority.
- A passing process status that contains a failing validation result.

## Audit risks

The main risks are Bend output parsing, 24-bit wrapping, shared frontend errors, HVM source drift, compiler drift, timing noise, hidden network access, stale identities, and broad claims.

## Budget

The search uses three serial architecture lenses because subagent consent is absent. These passes are correlated.

- Source authority: local exact Git revisions and repository-owned artifacts only.
- Retrieval: one focused pass for each chosen mechanism, then only missing facts.
- Tool budget: one exact Nix package route and one bounded execution route.
- Validation: focused tests after each core step, then repository-wide gates once.
- Stop states: `validated`, `blocked`, `exhausted`, or `user-decision-required`.

## Approach registry

| Family | Mechanism | Claim | Artifact | Gap | Blocker | Next check | State |
|---|---|---|---|---|---|---|---|
| `pinned-cli-shell` | Package exact sources in Nix and execute fixed CLI argument vectors from a bounded shell. | Satisfies the specified command and authority boundary. | Nix package, pure plan, shell receipt, and execution bundle. | simpler | HVM2 fixed buffers reserve at least 10 GiB under a 1 GiB profile. | Review a new address-space profile or bounded HVM2 layout in a later evaluation. | validated |
| `embedded-vendor-api` | Link Bend and HVM libraries into Kamacite. | Can avoid output parsing. | New Rust dependencies and vendor API adapter. | stronger | Adds a larger compatibility surface and does not preserve the required CLI command profile. | No further check unless the CLI route fails for a vendor defect. | blocked |
| `static-record-only` | Validate prerecorded outputs without executing the tools. | Can validate artifact shapes only. | Static fixtures and receipt. | weaker | The specification requires repeated execution and proof validation. | None. | falsified |

## Adversarial audit

The audit alters facts at the cohort, proof, source, input, command, artifact, output, resource, benchmark, receipt, and claim boundaries.

The typed fixture catalog contains 20 negative cases. Each case reaches its declared boundary.

The first shell run also exposed a missing isolated `/proc`. The corrected shell mounts `/proc` inside a separate PID namespace.

The second run exposed the fixed HVM2 address-space layout. Source generation and GCC compilation pass, but both runtime routes fail under 1 GiB.

## Result

The validated result is `defer`. The generated C reserves at least `10737418240` bytes before smaller runtime structures.

The pilot does not report a benchmark result. It does not classify missing resource-bound output as semantic disagreement.

The search used one pinned CLI mechanism, three correlated architecture lenses, two failed shell attempts, and one final retained run.
