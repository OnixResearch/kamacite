## Design

Kamacite adds a source-preserving FSM spec and receipt family in the existing Preserves-first style. The FSM vocabulary is canonical data, not a runtime, not a Rust macro system, and not an evaluator. It can describe the same reviewed transition relation that Octet checks in Rust and Valence records as evidence.

## Schema shape

The FSM spec binds:

- schema version and FSM identity;
- optional source envelope identity for the human-authored spec;
- closed state universe with stable state identities and labels;
- closed event universe with stable event identities and labels;
- guard vocabulary with input identity, evidence-ref requirements, and non-claim text;
- transition rows for admitted state/event pairs;
- denial rows for invalid state/event pairs;
- side-effect class vocabulary, including a no-side-effect denial class;
- checked-law names such as total-pair-coverage, duplicate-row-denial, declared-guard-use, and deny-preserves-state;
- explicit non-claims.

The FSM receipt binds the canonical FSM spec identity, universe identities, transition coverage identity, denial coverage identity, adapter descriptor identity, diagnostics, checked laws, dependency identities, optional Octet receipt identities, optional Valence export identity, and BLAKE3 receipt identity over canonical receipt bytes.

## Validation rules

Kamacite validates FSM specs and receipts over parsed in-memory DTOs. File reads, Nickel evaluation, Steel probes, Rust projection generation, fixture update workflows, stdout/stderr, and Valence export rendering remain in CLI shells.

Validation fails closed for duplicate states or events, duplicate transition rows, undeclared states/events/guards/side-effect classes, missing denial rows for invalid pairs, wildcard/default rows, stale coverage identities, malformed receipt identities, weakened non-claims, and semantic overclaims.

## Stack integration

Kamacite exports a Valence-friendly projection that preserves Kamacite receipt identity, artifact identities, checked laws, diagnostics, source spans when available, and non-claims. The projection does not require a live Valence crate dependency. Octet may consume Kamacite FSM specs as checked Rust projection inputs, but Octet findings remain source-shape/provenance evidence only.

## Validation shape

Positive fixtures cover a complete small FSM spec, a spec with guards and side-effect classes, a receipt linked to an Octet source-shape receipt, and a Valence export projection. Negative fixtures cover missing coverage, duplicate rows, wildcard rows, undeclared guards, stale BLAKE3 identities, wrong identity domains, missing non-claims, and overclaims.
