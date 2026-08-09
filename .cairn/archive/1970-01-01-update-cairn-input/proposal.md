## Why

The workspace dependency catalog admits Cairn revision `695124d459574ba7aeba6097310d237f393c243c` for Kamacite's validation plane. Kamacite still pins `637e008fe857a4e0c679577766e53f71383ae79d`, and that older tool accepts the retired `cairn/` lifecycle layout.

## What Changes

- Regenerate only the `cairn` Nix input and its transitive lock graph.
- Use Cairn's hash-bound migration to move lifecycle authority to `.cairn/`.
- Retire the duplicated legacy lifecycle tree after byte-parity review.
- Update maintained repository paths and regenerate policy output.
- Preserve all active and archived lifecycle content without changing its meaning.

## Impact

- **Files**: generated `flake.lock`, `.cairn/**`, removed legacy `cairn/**`, maintained lifecycle path references, and generated policy JSON
- **Testing**: migration receipts, path searches, lifecycle validation, policy freshness, focused Kamacite checks, and full Nix flake validation
