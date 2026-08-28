## Why

Kamacite already declares optional source span indexes. It has no shared structural path contract for canonical values.

Adapters can return string diagnostics without stable value paths. They can also lose the source location that explains a rejected value.

Jane Street `sexplib` provides useful design references for typed paths, annotated values, and delayed location refinement. Kamacite needs equivalent mechanics over Preserves values and source envelopes.

## What Changes

- Add a pure located-value component with bounded, schema-bound structural paths.
- Add deterministic lookup and identity-guarded immutable replacement.
- Add exact, derived, and explicitly absent source-origin classes.
- Add structured diagnostics that bind value paths, source origins, locator identities, optional parser identities, and limit profiles.
- Add delayed location refinement over one captured byte snapshot.
- Add versioned Preserves schemas, adapter descriptors, receipts, and positive and negative fixtures.

## Impact

- **Files**: `crates/kamacite-located-value/`, workspace manifests, `schemas/preserves/`, `fixtures/`, adapter docs, `README.md`, and this lifecycle package.
- **Consumers**: Nickel and Steel adapters can project located diagnostics. Valence can consume exported spans without owning parsing.
- **Dependencies**: no runtime `sexplib` dependency. The design references MIT-licensed `sexplib` revision `e5a4df62c104c8f7c2e8a11ce36e88808649fce4`.
- **Testing**: baseline and final workspace tests, positive and negative fixtures, property tests, adapter checks, Cairn gates, Clippy, Octet, and `nix flake check -L`.

## Non-goals

- Do not add S-expressions as a Kamacite format.
- Do not make parser spans proof of Nickel or Steel evaluation semantics.
- Do not infer exact source origins for computed, merged, generated, or deserialized values.
- Do not give the pure component file, evaluator, process, network, clock, random, credential, or mutation authority.
- Do not move evidence-span ownership from Valence into Kamacite.
