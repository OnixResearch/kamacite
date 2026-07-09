# Agent Notes

## Repository purpose

`braid` defines the lossless interchange boundary for Nickel-authored configuration/contracts, Rust-owned adapters and validators, and bounded Steel Scheme predicates or probes.

## Lifecycle

Use native Cairn artifacts under `cairn/` for product decisions:

- accepted specs: `cairn/specs/`
- active changes: `cairn/changes/`
- archived changes: `cairn/archive/`

Run Cairn through the sibling repo unless this repo later grows its own package:

```bash
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- validate --root .
```

## Format boundary

- Preserves is the canonical encoded IR candidate for Braid value/envelope identity.
- JSON is a compatibility projection only; do not treat JSON export as the authority for losslessness.
- Nickel remains the preferred human-authored contract/config language.
- Rust owns pure validation cores, canonicalization, hashing, adapter DTOs, and receipts.
- Steel Scheme is bounded adapter/probe logic only; do not grant ambient filesystem, process, network, credential, or mutation authority.
- WebAssembly is an optional adapter/validator target, not Braid's IR. WASM inputs and outputs must remain Braid IR/source-envelope bytes plus deterministic receipts.

## Losslessness claims

Never claim arbitrary source-to-source translation across Nickel, Rust, and Steel. Preserve original source bytes and BLAKE3 hashes, then make narrower claims about parsed projections or constrained Braid IR values.

Every new conversion or validation behavior needs positive and negative fixtures before it is considered implemented.
