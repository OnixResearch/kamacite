# Verification evidence

## Checked scope

This evidence covers `add-world-commit-interchange-conformance` at the final working tree before lifecycle sync and archive.

Kamacite owns byte-level source preservation, canonical Preserves values, adapter parity, diagnostics, and bounded receipts. It does not prove Molten runtime behavior, restore compatibility, authority, semantic equivalence, or release eligibility.

## Octet consumer gate

Kamacite pins `OnixResearch/octet` at `cf04e894e53eb0947230118a086ef6066ddba38c` through Nix and the official pre-commit hook.

The workspace metadata selects `--workspace`, `--all-targets`, and `--all-features`. `dylint.toml` enables the complete pinned catalog. It has no disabled lints, finding budget, or finding baseline.

Legacy source findings use reasoned source-local attributes. Searchable source paths replaced underscore filenames without changing Rust module names. The Nix `octet-deny-all` check derives its deny flags from the pinned Octet inventory.

The final strict check passed with zero findings:

```text
Status: clean
Findings: 0
Warnings: 0
Errors: 0
Toolchain: nightly-2026-03-21-x86_64-unknown-linux-gnu
Config hash: b3:db060937d71af219b35d4d4fe20825f41e4b6550870441c195384636f104fb7f
Profile hash: b3:510ab9c3e4d680c2b97da4b5b468acad4e99837cd2efa20aef41df6676877052
```

Command:

```bash
nix --option builders '' --option secret-key-files '' build "path:$PWD#checks.x86_64-linux.octet-deny-all" -L
```

## Rust verification

The repository development shell passed formatting, all workspace tests, and Clippy with warnings denied.

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings -D clippy::dbg_macro -D clippy::mem_forget -D clippy::undocumented_unsafe_blocks -D clippy::multiple_unsafe_ops_per_block -D clippy::host_endian_bytes -D clippy::iter_over_hash_type
```

Results:

- 240 unit and integration tests passed.
- 2 positive doctests passed.
- 6 compile-fail doctests passed.
- No Rust test failed or remained ignored.
- Clippy completed without a warning.

The positive cases cover admitted Rust, Nickel, and Steel values. Negative cases cover malformed values, drift, unknown versions, wrong domains, hidden data, overclaims, undeclared authority, unsafe paths, and stale evidence.

## Conformance regeneration

The authorized update used the already reviewed Molten revision `2f5d7bf9017b3b00df36e55cf0bbef6d68187611`.

Two update passes produced identical BLAKE3 hashes for the cohort, manifests, Preserves receipt, JSON receipt, and Valence projection.

Final identities:

- Cohort: `compatibility:4d5fde38ca2641cee324e2e8b47f19ee9b23dc3201ebca3346d2ca5e5510846c`
- Canonical value: `canonical-value:63f1d2d914dd791247eadc516498c3cde39259eb66912fa50a5ac94be5134d94`
- Receipt: `receipt:3ca391f9c6ad6c7ee540fc0f7b097ba2129fb2afee838c0db046470da468ad37`
- Positive vectors: 6
- Negative vectors: 19
- Adapters: 3
- Verdict: `pass`

## Focused repository checks

These checks passed:

```bash
nix run "path:$PWD#kamacite" -- validate --root . --manifest fixtures/manifest.json
nix run "path:$PWD#kamacite" -- generate-manifest --root .
nix run "path:$PWD#kamacite" -- validate-nickel-adapter --root . --manifest fixtures/manifest.json
nix run "path:$PWD#kamacite" -- validate-steel-adapter --root . --manifest fixtures/manifest.json
nix run "path:$PWD#world-commit-conformance" -- --root .
nix run "path:$PWD#kamacite" -- export-valence-receipt --root . --receipt fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json --conformance fixtures/valence-export/conformance.json --golden fixtures/valence-export/golden/source-envelope.valid.nickel-minimal.valence-export.json
nix run "path:$PWD#lifecycle-validate" -- --root .
```

The complete repository gate also passed:

```bash
nix --option builders '' --option secret-key-files '' flake check "path:$PWD" -L
```

Nix omitted the incompatible `aarch64-linux` checks on this `x86_64-linux` host.

## Cairn verification

Cairn revision `15f00875562025e7ea7e0d1f4af24d1a2e2ac06f` passed repository validation and the proposal, design, and tasks gates. These runs used the current shared Cairn policy.

The repository-local generated policy was refreshed directly from `cairn-policy/default.ncl` with Nickel and then parsed by the repository-pinned Cairn tool. The pinned Cairn dual-run exporter still reports a legacy-versus-standalone status drift, so that exporter result is not claimed as passing evidence.
