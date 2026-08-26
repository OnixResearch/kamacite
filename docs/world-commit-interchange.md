# Molten world-commit interchange

Kamacite checks one pinned Molten world-commit format across Rust, Nickel, and Steel.

<!-- r[docs kamacite.world_commit_conformance.cohort] -->
<!-- r[docs kamacite.world_commit_conformance.entry] -->
<!-- r[docs kamacite.world_commit_conformance.domains] -->
<!-- r[docs kamacite.world_commit_conformance.parity] -->
<!-- r[docs kamacite.world_commit_conformance.rust] -->
<!-- r[docs kamacite.world_commit_conformance.nickel] -->
<!-- r[docs kamacite.world_commit_conformance.steel] -->
<!-- r[docs kamacite.world_commit_conformance.sources] -->
<!-- r[docs kamacite.world_commit_conformance.update] -->
<!-- r[docs kamacite.world_commit_conformance.receipt] -->
<!-- r[docs kamacite.world_commit_conformance.verification] -->

This check is an interchange conformance profile. It is not a second world runtime or a generic stack protocol.

## Ownership boundary

Molten owns these items:

- world-commit meaning
- root-domain meaning
- snapshot profile meaning
- world capture and restore
- runtime behavior
- merge policy
- branch authority
- effect authority
- release decisions

Kamacite owns these bounded items:

- canonical Preserves admission
- pinned source and evaluator cohorts
- Rust, Nickel, and Steel adapter comparison
- positive and negative conformance vectors
- deterministic conformance receipts
- recorded-only Valence projections

Kamacite does not reinterpret the Molten model.

## Pinned product cohort

The profile pins Molten revision `2f5d7bf9017b3b00df36e55cf0bbef6d68187611`.

The source inventory is in `fixtures/world-commit/cohort.ncl`. Generated identities are in `fixtures/world-commit/cohort.json`.

`fixtures/world-commit/product-source-manifest.json` binds the immutable Molten source snapshot.

The checker pins that manifest identity in Rust. Update mode cannot rewrite altered product sources under the existing revision.

`fixtures/world-commit/implementation-manifest.json` binds the Kamacite core, shell, dependencies, and adapter implementation source closure.

The source snapshot includes the schema, codec, root model, identity logic, validation logic, and canonical logical fixture.

`fixtures/world-commit/source/NOTICE.md` records the source repository, revision, license, paths, and retained Molten claims.

An unknown revision, schema, codec, domain tag, evaluator, or source identity fails closed.

## Canonical value

The Rust path admits Molten's canonical packed Preserves bytes.

The value has these fields:

- schema and version
- logical, opaque, or mixed profile
- ordered parent references
- ordered nominal roots
- profile-relative completeness

The checker validates Molten's domain-separated BLAKE3 world-commit identity.

JSON is a review projection. JSON never becomes canonical world-commit data.

Signatures, attestations, capabilities, current authority, and runtime handles stay outside the immutable value.

## Adapter comparison

The profile runs three fresh adapter paths.

### Rust

Rust admits the authoritative packed Preserves fixture. It validates the closed record shape, limits, references, roots, and completeness.

### Nickel

Nickel exports `fixtures/world-commit/adapters/logical-v1.ncl` through its declared contract.

The shell compares the exported DTO with the admitted Rust value. It then resolves that DTO to the authoritative packed bytes.

The Nickel export remains a projection. It is not a canonical codec or a proof of evaluator equivalence.

The shell runs Nickel in the repository's Bubblewrap and `prlimit` profile. It mounts only reviewed source and import bytes.

### Steel

Steel starts a fresh `Engine::new_raw_no_kernel` instance.

The engine receives one pure host function: `kamacite.resolve-world-vector`.

That function resolves a declared integer vector identifier. It has no file, process, network, environment, clock, or random authority.

The shell verifies the exact Steel source, module descriptor, input DTO, host call, and evaluator profile.

## Fixture corpus

`fixtures/world-commit/vector-manifest.json` records source and packed identities for each vector.

Positive vectors cover these cases:

- logical profiles
- opaque profiles
- mixed profiles
- every nominal root domain
- the optional authority-observation root
- empty parents
- ordered parents
- the admitted parent limit

Negative vectors cover these cases:

- unknown versions and schemas
- unknown or substituted root domains
- duplicate root domains and references
- missing required roots
- wrong reference domains
- excessive parent and root collections
- hidden secret fields
- altered source identities
- unapproved cohort updates

All negative vectors must fail with deterministic diagnostic classes.

## Run the check

All fixture reads and updates use one Linux `openat2` root capability. Paths reject traversal and symlink substitution.

Run the normal check:

```bash
nix run .#world-commit-conformance -- --root .
```

The check evaluates Nickel and Steel twice. Both fresh passes must produce the same cohort, bytes, identities, and receipt.

## Update the cohort

First, create a dry-run plan for a proposed Molten revision:

```bash
nix run .#world-commit-conformance -- \
  --root . \
  --plan-update 1111111111111111111111111111111111111111
```

Use `--proposed-schema` and `--proposed-codec` to compare reviewed replacement files.

The plan does not write files. It reports changed identities, convergence needs, and the new-cohort requirement.

Updates are explicit. The command requires the exact reviewed Molten revision.

```bash
nix run .#world-commit-conformance -- \
  --root . \
  --update \
  --accept-revision 2f5d7bf9017b3b00df36e55cf0bbef6d68187611
```

Run the update twice. The generated manifest, cohort, receipt, and Valence projection must remain byte-identical.

Review source changes, removed negative vectors, evaluator changes, and changed identities before commit.

Do not update the cohort silently when Molten changes its schema or codec.

## Evidence

The checker writes these stable artifacts:

- `fixtures/world-commit/product-source-manifest.json`
- `fixtures/world-commit/implementation-manifest.json`
- `fixtures/world-commit/vector-manifest.json`
- `fixtures/world-commit/cohort.json`
- `fixtures/world-commit/golden/world-commit-conformance-receipt.preserves`
- `fixtures/world-commit/golden/world-commit-conformance-receipt.json`
- `fixtures/world-commit/golden/world-commit-valence-projection.json`

The Preserves receipt is authoritative. The JSON receipt is a review projection.

The Valence projection has the role `recorded-only`. It preserves all non-claims and adds no semantic claim.

## Non-claims

This profile does not prove these properties:

- evaluator semantic equivalence
- Molten runtime behavior
- restore or merge compatibility
- branch or effect authority
- release eligibility
- a generic stack-wide world protocol
- proof of a referenced object's type or contents

A passing result proves only byte-level convergence for the declared cohort and checked laws.
