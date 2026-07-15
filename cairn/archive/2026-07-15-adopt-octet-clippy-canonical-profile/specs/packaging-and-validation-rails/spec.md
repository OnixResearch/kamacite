# Packaging and Validation Rails Specification Delta

## ADDED Requirements

### Requirement: Canonical Clippy restriction profile

r[kamacite.packaging.clippy_restriction_profile] Kamacite's repo-owned Clippy gate MUST deny the explicit restriction lints selected by Octet's reviewed `canonical-determinism` profile in addition to ordinary warnings, and MUST NOT enable the blanket `clippy::restriction` group.

#### Scenario: Clean workspace passes the selected profile

r[kamacite.packaging.clippy_restriction_profile.positive]

- GIVEN all tracked Kamacite Rust library, binary, and test targets
- WHEN the repo-owned workspace/all-target Clippy gate runs
- THEN it MUST deny `dbg_macro`, `mem_forget`, `undocumented_unsafe_blocks`, `multiple_unsafe_ops_per_block`, `host_endian_bytes`, and `iter_over_hash_type`.
- AND it MUST report success only when no ordinary warning or selected restriction finding remains.

#### Scenario: Violating fixture fails closed

r[kamacite.packaging.clippy_restriction_profile.negative]

- GIVEN a negative fixture contains an existing forbidden hidden panic and a `dbg!` macro selected by the adopted profile
- WHEN the repo-owned negative Clippy check runs with the same restriction flags
- THEN the command MUST fail.
- AND the check MUST observe both the `expect_used` and `dbg_macro` lint identities.

#### Scenario: Compiler lint evidence remains bounded

r[kamacite.packaging.clippy_restriction_profile.boundaries]

- GIVEN the canonical Clippy gate passes
- WHEN Kamacite reports the result
- THEN it MUST treat the result as compiler source-shape evidence only.
- AND it MUST NOT claim canonical semantic correctness, runtime determinism, panic freedom, memory safety, compiler correctness, formal proof, whole-program correctness, or release eligibility.
