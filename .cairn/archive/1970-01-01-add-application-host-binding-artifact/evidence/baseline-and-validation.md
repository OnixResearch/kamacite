# Baseline evidence: application-host binding

Task I1 baseline, captured before adding the binding artifact on the drain
branch (base `origin/main` = `34738d6`).

- Workspace tests at base: all suites pass
  (`cargo test --workspace`: 143 `kamacite-core` tests, 34 CLI binary
  tests, plus integration and doc suites).
- Fixture validation at base: `kamacite validate --root . --manifest
  fixtures/manifest.json` exits 0.
- Lifecycle validation and Cairn gates at base: pass with the pinned
  policy.
- The fixture manifest contains no `application-host-binding` artifact
  class before this change; `IdentityDomain` has no
  `application-host-binding` variant.

# Validation evidence

- `cargo test --workspace`: all suites pass. This includes 16 core
  binding tests and 3 Animus producer conformance tests.
- `kamacite validate --root . --manifest fixtures/manifest.json`: exit 0
  with 12 new fixtures. The set has 4 positive and 8 negative fixtures.
- The metadata-rename golden and the capability-free golden record the
  same `application-host-binding` identity
  (`4d93c4b6ca20031aa2fe665119f423d773351072e94a9975a8284e968e24b886`),
  proving metadata exclusion at the golden level.
- `generate-manifest --update` regenerated `fixtures/manifest.json` from
  the closed Nickel contract; the contract admits the new
  `application-host-binding` payload kind and identity domain.
- The exact Animus producer revision is
  `c09991fa299f1f74b04d066bf31382e34108fa82`.
- The frozen producer fixture BLAKE3 is
  `f8fbfda873094923004b99ac1150eb4e402b49deaa866b0647bd84e758e8a0ef`.
- Kamacite accepted the producer with receipt identity
  `1bac624751d47bd89ecba53923eda72f9cf20cdcc31dc01186882abf8da1cbb2`.
- The returned conformance fixture BLAKE3 is
  `d5a082fb3ee92bc0fdbccdd0a08bbc9e072b3eea95615df8a17be70fbf9c540c`.
- Clippy with the repository restriction profile: clean.
- Manifest freshness and Cairn lifecycle validation: pass.
- `nix flake check path:$PWD -L`: all 17 checks pass.

## Claim boundary

A passing binding proves canonical identity, required-field validity, one
selected host profile, closed declared effects, and exact linked cohort
identities. It does not prove application correctness, authorization,
sandboxing, resource enforcement, runtime behavior, or release
eligibility.
