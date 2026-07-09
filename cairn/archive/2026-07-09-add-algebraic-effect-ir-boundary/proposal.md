## Why

Kamacite already says effects and capabilities must be explicit, but the current lossless-IR Cairn only names host-effect labels at a coarse boundary. Algebraic effects need a narrower product decision before implementation: effect signatures, rows, summaries, handlers, continuation obligations, and replay evidence must be canonical Kamacite data rather than Aspen/Molten runtime conventions, Octet lints, or Trellis proof artifacts alone.

Adding this boundary in Kamacite lets the stack share one portable effect model. Trellis can prove finite effect/handler facts, Valence can identify and link effect receipts, Aspen/Molten can execute handlers and replay effect logs, and Octet can check Rust surfaces without becoming a Rust language-level algebraic-effect system.

## What Changes

- Define Kamacite-owned algebraic-effect artifact classes for effect signatures, effect rows, effect summaries, handler descriptors, handler-application receipts, and effect-log projections.
- Require algebraic-effect artifacts to use canonical Preserves identity and BLAKE3 hashes under Kamacite-owned schemas.
- Separate algebraic operations from host capabilities: declaring an operation does not grant filesystem, process, network, clock, random, credential, mutation, or other ambient authority.
- Require handler descriptors to declare handled operations, residual effects, continuation policy, replay profile, adapter/evaluator identity, checked laws, and non-claims before handler results are trusted.
- Require positive and negative fixtures for declared operations, undeclared operations, handler discharge, residual rows, continuation-use violations, host-capability confusion, stale hashes, and unsupported versions.
- Document stack roles: Trellis is proof/reference evidence, Valence is identity/linkage export, Aspen/Molten is runtime consumption, and Octet is Rust source-shape/provenance checking only.

## Impact

- **Files**: `cairn/changes/add-algebraic-effect-ir-boundary/**`.
- **Testing**: Cairn validation plus proposal/design/tasks gates now; later implementation must add Preserves schema checks, pure Rust validation tests, and positive/negative fixture receipts before any algebraic-effect claim is accepted.
