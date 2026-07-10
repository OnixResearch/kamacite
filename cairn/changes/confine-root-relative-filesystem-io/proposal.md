## Why

`resolve_under_root` rejects absolute paths and lexical `..` components but then calls `root.join(path)` without resolving or safely opening symlinks. A reproduced `--out escape/receipt.json` write followed a symlink under the root and created a receipt outside the reviewed tree. The golden update report also bypasses the helper with a direct `root.join`, so declared root-relative authority is not enforced consistently.

## What Changes

- Centralize all root-scoped reads and writes behind a capability-oriented path admission and open layer.
- Prevent symlink traversal, parent traversal, absolute paths, and time-of-check/time-of-use replacement for existing and newly created files.
- Route manifest, fixture, source, golden, update-report, receipt, Valence, and documentation paths through the same policy.
- Add positive normal-relative tests and negative symlink, traversal, absolute, destination-parent, and replacement-race tests.

## Impact

- **Surfaces**: every CLI command that accepts `--root` or writes generated evidence.
- **Security boundary**: a reviewed root becomes an enforced filesystem capability rather than a lexical convention.
- **Validation**: shell-level path tests, mutation-boundary checks, Rust tests, and Cairn gates.
