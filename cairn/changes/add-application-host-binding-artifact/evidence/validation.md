# Application-host binding validation

<!-- r[verify kamacite.application_host_binding.fixtures] -->

Date: 2026-08-03

## Baseline

The pre-change workspace test suite passed. The `kamacite-core` suite reported 131 passing tests.

The existing fixture manifest validation passed. Cairn validation and the proposal, design, and tasks gates also passed.

The first lifecycle command used the newer sibling Cairn checkout. It rejected the older generated policy shape before product validation.

The repeated baseline used Kamacite's pinned Cairn tool and passed. This tool version matches the repository policy.

## Implemented core

Kamacite now admits one version 1 application-host binding through a pure deterministic core.

The binding uses canonical packed Preserves bytes and the `application-host-binding` BLAKE3 identity domain. It links existing typed identities instead of copying product policy.

Admission requires one host, closed known effects, declared semantic operation identities, exact reviewed cohorts, required non-claims, safe diagnostics, and no authority claims.

Canonical validation receipts and Valence-friendly projections preserve separate identity roles. Rejected receipts cannot produce passing projections.

## Focused validation

The focused core suite passed 11 positive, negative, property, receipt, and projection tests.

The focused CLI suite passed four capability-root, positive, negative, and mutation-boundary tests.

Strict Clippy passed for all Kamacite core and CLI targets. The generated manifest, generation receipt, and positive and negative golden receipts are current.

## Pending downstream evidence

The frozen Animus producer fixture is not yet recorded. The Kamacite schema and core must stabilize before that exact producer projection is imported.

## Non-claims

These results do not prove application correctness, effect completeness, host authorization, sandbox enforcement, resource enforcement, runtime correctness, Valence verifier soundness, downstream integration, or release eligibility.
