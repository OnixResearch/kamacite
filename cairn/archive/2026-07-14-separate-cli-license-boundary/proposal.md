## Why

Kamacite's interchange core is intended for broad embedding across Nickel, Rust, and Steel boundaries, while its CLI is an operator application. A single permissive workspace license does not protect distributed CLI modifications, and a single AGPL license would make the canonical IR core unnecessarily difficult to embed.

## What Changes

- Keep `kamacite-core` under `MIT OR Apache-2.0`.
- License `kamacite-cli` under `AGPL-3.0-or-later`.
- Add complete license texts and a package boundary document.
- Preserve all third-party material under upstream terms.

## Impact

Interchange consumers retain a permissive core; the operator shell gains AGPL reciprocity without changing runtime behavior or canonical identity.
