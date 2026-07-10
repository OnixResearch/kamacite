## Why

The documentation CLI currently checks only that a config path ends in `.ncl` and is readable, then reports success without parsing Nickel, validating the docs contract, collecting facts, checking IR, rendering output, running cells, or updating artifacts. A reproduced file containing invalid non-Nickel text passed `docs check` with no diagnostics. Command names therefore overstate work that did not occur.

## What Changes

- Evaluate and validate Nickel docs configuration before every action and bind the checked projection/evaluator identity.
- Wire `collect`, `check`, and `render` to the existing pure documentation core and deterministic receipt generation.
- Keep `run-cells` explicitly denied unless a supported bounded runner actually executes; keep `update` explicit and receipt-producing when it mutates.
- Fail closed when an action is unimplemented, performs no required work, sees malformed config, or receives unsupported format/profile/target values.

## Impact

- **Surfaces**: `kamacite docs`, docs configuration, generated artifacts, runnable cells, and documentation receipts.
- **Behavior**: exit success becomes evidence of the named completed action, not mere admission of an action token.
- **Validation**: malformed-config tests, no-op rejection tests, renderer/cell positive and negative fixtures, and Cairn gates.
