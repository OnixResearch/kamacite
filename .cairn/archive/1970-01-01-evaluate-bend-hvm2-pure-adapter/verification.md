# Verification

## Decision

The pilot decision is `defer` for the declared 1 GiB address-space profile.

The exact Trellis oracle returns `1` for all six runs. Each oracle run reports three abstract combines.

Bend source generation and GCC compilation pass. The generated C and executable remain separate target artifacts.

`bend run-rs` does not return an HVM result under the limit. The generated executable stops with signal-class exit code `139`.

The generated C reserves at least `10737418240` bytes in fixed global buffers. This value excludes smaller runtime structures.

The core records this fact as `hvm2-address-space-profile`. It does not report a benchmark summary or invent missing route output.

## Exact identities

- Profile: `adapter:7640b25994e778a4d5340f90bd706196b220da22e09987028923267d37240dff`
- Bend source: `source:dfd13c5a80fe38463dbe531a159f597d99b4238d58f1a84867185febf234ae74`
- Canonical input: `canonical-value:b9d7e3fde5cdab7b9de2867ca2bff1038821169cf7fc28fd60a9310501757bc6`
- Trellis proof bundle: `source:e58a90561093f7ae28adfe8eb56c16a618c4e99e056af8822633609d5201a70e`
- Proof validation: `evidence-chain:05f63e3eede85bffd84aeb226dd62d4c57f32b58cae1b79850e9d3b3a829f79b`
- Generated C: `target-artifact:677fb01c8db8b0488fa213e0194f4f47bf055f4dd7bf07bc614255b6474105e6`
- Compiler invocation: `compiler:d7011ae9507d400a615aa039026e6b153090b86e7a93862efa22b99955917362`
- Executable: `target-artifact:6ce66d9200036f81b3f0b4e1e902238ebe321d3fcdf636f964047cd773768e9c`
- Pilot receipt: `receipt:53a0a561b0d523f5a62b1561387e420830b7a3d14c090f9b6eccafd4c28d99b6`
- Valence projection: `projection:365df710e066ed1b61694cb372fd250ab9d74f86cae5980b31efcc63c087453c`

## Baseline

The proposal, design, tasks, and strict lifecycle validation gates passed before implementation.

The baseline workspace tests passed. The first test attempt waited on a shared target lock, so the rerun used an isolated target directory.

The baseline `nix flake check -L` failed on a stale world-commit implementation manifest. The failure existed before pilot code changes.

The final world-commit update converged. The check now reports receipt `c69b5dd1ba624dd6ca6795f5e45ad10888aae1330e82586d50d3d7e5cffe5bdc`.

## Proof evidence

The Nix proof rail uses Trellis `f26e3cd465101698ade12f8d142e0dc95578326b` and Octet `374bd16b26cee2af34211a29bfa531c016811f51`.

Production Verus reports `11098 verified, 0 errors`. The Trellis proof bundle checker validates seven records.

This result gives the Rust oracle a verified Trellis-local role. It does not transfer that role to Bend, HVM2, C, or GCC.

## Positive and negative evidence

The focused core suite passes 12 tests. The stored evidence test recomputes stream, artifact, receipt, and Valence identities.

The typed negative catalog contains 20 cases. Each case reaches its declared pre-execution, execution, post-output, benchmark, or receipt boundary.

The first live attempt found a missing isolated `/proc`. The final sandbox mounts `/proc` inside a separate PID namespace.

The second live attempt found the fixed HVM2 address-space layout. Both attempts remain in `evidence/pilot-attempts/`.

## Rust and policy checks

The workspace test suite passes, including 186 `kamacite-core` tests and all CLI integration tests.

Strict Clippy passes with warnings denied and the repository canonical-determinism restrictions enabled.

The final Octet gate reports `Status: clean`, with zero findings, warnings, and errors.

The profile and negative fixture projections are fresh Nickel exports. The exact source revisions, licenses, tools, schemas, and proof result pass the focused Nix profile gate.

The final `nix flake check -L` passes all repository checks after archival.

## Evidence locations

- Retained pilot bundle: `fixtures/bend-hvm2/golden/`
- Final lifecycle evidence: `.cairn/changes/evaluate-bend-hvm2-pure-adapter/evidence/final-2026-09-04/`
- Failed attempt evidence: `.cairn/changes/evaluate-bend-hvm2-pure-adapter/evidence/pilot-attempts/`
- Pilot Nix result: `/nix/store/8890grxah7cp9cxswwr85rrj4lpgyjjz-kamacite-bend-hvm2-pilot-run`
- Trellis proof result: `/nix/store/z3zp3j8p6dbfsjd9jpf0gbqs1qxm35wr-trellis-verus-proofs-exact-source-v1`

## Remaining promotion boundary

A later evaluation can review a larger address-space profile or a configurable HVM2 layout.

That evaluation requires a new profile identity and fresh measurements. Runtime consumers still require separate Cairn changes.
