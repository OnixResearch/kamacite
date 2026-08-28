## Why

Kamacite adapter descriptors name input schemas, output schemas, laws, and non-claims. They do not carry a machine-readable witness for the accepted value shape.

A separate hand-written grammar can drift from its codec. That drift can make generated documentation, completion, fixtures, or compatibility reviews unsound.

Jane Street `ppx_sexp_conv` generates a runtime grammar beside each generated converter. Kamacite can adapt that mechanism without adopting PPX, OCaml types, or S-expression syntax.

## What Changes

- Add a pure codec-witness component with a bounded grammar vocabulary.
- Produce each codec and witness from one checked schema definition or generated descriptor.
- Bind codec, witness, schema, and limit-profile identities in adapter descriptors and receipts.
- Classify witnesses as exact or conservative.
- Make unknown fields, defaults, recursion, alternatives, and custom predicates explicit.
- Add bounded corpus validation with positive and negative fixtures.

## Impact

- **Files**: `crates/kamacite-codec-witness/`, workspace manifests, `schemas/preserves/`, fixture manifests, adapter descriptors, documentation, and this lifecycle package.
- **Dependencies**: no runtime `ppx_sexp_conv` dependency. The design references MIT-licensed revision `fd48b5f67a8149d0b3546a4a3b3c852e8da06398`.
- **Testing**: baseline and final workspace tests, witness and codec corpus tests, negative drift tests, Cairn gates, Clippy, Octet, and `nix flake check -L`.

## Non-goals

- Do not replace Nickel contracts or product policy.
- Do not claim that a grammar proves semantic correctness.
- Do not make generated examples trusted evidence without codec admission.
- Do not permit unknown fields or implicit defaults through witness omission.
- Do not add an OCaml code generator or S-expression schema.
