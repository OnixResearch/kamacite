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

- `cargo test --workspace`: all suites pass, including 16 new
  `application_host_binding` tests (positive, negative, and property).
- `kamacite validate --root . --manifest fixtures/manifest.json`: exit 0
  with 11 new fixtures (3 positive, 8 negative) and generated golden
  receipts.
- The metadata-rename golden and the capability-free golden record the
  same `application-host-binding` identity
  (`4d93c4b6ca20031aa2fe665119f423d773351072e94a9975a8284e968e24b886`),
  proving metadata exclusion at the golden level.
- `generate-manifest --update` regenerated `fixtures/manifest.json` from
  the closed Nickel contract; the contract admits the new
  `application-host-binding` payload kind and identity domain.
- Clippy with the repository restriction profile: clean.
- Cairn lifecycle validation: pass.

## Claim boundary

A passing binding proves canonical identity, required-field validity, one
selected host profile, closed declared effects, and exact linked cohort
identities. It does not prove application correctness, authorization,
sandboxing, resource enforcement, runtime behavior, or release
eligibility.
