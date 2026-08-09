## Context

Kamacite already separates pure IR validation from CLI filesystem and presentation effects. Package licensing follows that tested architectural boundary.

## Decisions

### Keep the interchange core permissive

The workspace default and `kamacite-core` remain `MIT OR Apache-2.0` so independent adapters, validators, and language implementations can embed the IR without licensing their larger work as AGPL.

### Apply AGPL to the CLI

`kamacite-cli` explicitly declares `AGPL-3.0-or-later`. The shell may depend on the permissive core; the core remains independent of the shell.

### Preserve canonical identities

License metadata and license files are packaging evidence, not Kamacite canonical value, source-envelope, adapter, target-artifact, or receipt bytes. The change does not rewrite existing artifact identities.

### State non-claims

The boundary does not relicense dependencies, revoke earlier grants, or constitute legal advice for a specific combination.
