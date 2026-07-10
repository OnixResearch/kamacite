## Why

Explicit Nickel validation launches the evaluator directly with inherited environment, working directory, filesystem visibility, network/process context, and no execution or output bound. The receipt's observed effects come from manifest fields rather than shell enforcement or observation. A root-relative source can therefore import outside the reviewed root or consume ambient authority while the resulting receipt still reports only declared capabilities.

## What Changes

- Run Nickel reference exports under an explicit sandbox profile with a reviewed read-only import closure and denied ambient authority.
- Sanitize environment and working directory, deny network and host mutation by default, and apply named execution, memory, and output limits.
- Bind the enforced sandbox profile, evaluator identity, source/import identities, exit result, and bounded observations into adapter receipts.
- Add negative fixtures for import escape, undeclared imports, environment dependence, nontermination/resource exhaustion, and oversized output.

## Impact

- **Surfaces**: `validate-nickel-adapter`, packaged Nickel execution, adapter receipts, and Nix checks.
- **Non-claim**: sandbox evidence bounds the declared run; it does not prove Nickel evaluator soundness.
- **Validation**: sandbox integration tests, capability fixtures, adapter validation, and Cairn gates.
