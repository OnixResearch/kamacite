## Phase 1: Baseline and export profile

- [ ] [serial] Run baseline Rust tests, default harness validation, Nickel adapter validation, manifest projection freshness check, and Cairn validation before implementation. r[kamacite.valence_receipt_export.profile]
- [ ] [serial] Define the Valence receipt export DTO/profile with source receipt identity, export identity, artifact identities, typed domains, roles, BLAKE3 hashes, checked laws, diagnostics, source span handling, conformance identity, and non-claims. r[kamacite.valence_receipt_export.profile] r[kamacite.valence_receipt_export.profile.export_identity]
- [ ] [serial] Add the Valence export conformance entry shape for profile version, source schema, output schema, identity domains, checked laws, dependency profile, fixtures, and non-claims. r[kamacite.valence_receipt_export.conformance]

## Phase 2: Pure core and CLI shell

- [ ] [serial] Implement a pure `kamacite-core` mapper from validated receipt DTOs to Valence export DTOs with deterministic diagnostics and no IO, environment, evaluator, process, clock, random, network, or mutation access. r[kamacite.valence_receipt_export.profile.bounded_projection]
- [ ] [serial] Add an explicit `kamacite-cli` command or mode for Valence receipt export that reads declared inputs, calls the pure mapper, renders deterministic reports, and remains non-mutating by default. r[kamacite.valence_receipt_export.command] r[kamacite.valence_receipt_export.command.explicit_inputs] r[kamacite.valence_receipt_export.command.non_mutating]
- [ ] [serial] Add explicit update/report behavior for refreshed export golden projections, including old/new export identities and preserved non-claims. r[kamacite.valence_receipt_export.command.explicit_update]

## Phase 3: Positive fixture coverage

- [ ] [parallel] Add a positive Valence export fixture that preserves source receipt identity, artifact identities, roles, BLAKE3 hashes, checked laws, diagnostics, and non-claims. r[kamacite.valence_receipt_export.profile.bounded_projection] r[kamacite.valence_receipt_export.profile.non_claims]
- [ ] [parallel] Add source-span coverage for both present spans and explicit absence without fabricating spans. r[kamacite.valence_receipt_export.profile.source_spans]
- [ ] [parallel] Add deterministic export identity and golden receipt/projection coverage. r[kamacite.valence_receipt_export.profile.export_identity]
- [ ] [parallel] Add an artifacts-first compatibility fixture that does not require live Valence crates or runtime APIs. r[kamacite.valence_receipt_export.coupling_boundary.artifacts_first]

## Phase 4: Negative fixture coverage

- [ ] [parallel] Add identity-domain substitution negative fixture coverage. r[kamacite.valence_receipt_export.conformance.identity_domains]
- [ ] [parallel] Add missing or weakened non-claims negative fixture coverage. r[kamacite.valence_receipt_export.conformance.missing_non_claims]
- [ ] [parallel] Add semantic-equivalence overclaim negative fixture coverage. r[kamacite.valence_receipt_export.conformance.semantic_overclaim]
- [ ] [parallel] Add malformed input, unsupported export-profile version, duplicate artifact identity, missing checked laws, and stale export identity negative fixture coverage. r[kamacite.valence_receipt_export.conformance.malformed_input] r[kamacite.valence_receipt_export.command.fail_closed]
- [ ] [parallel] Add untrusted-export-without-conformance negative fixture coverage. r[kamacite.valence_receipt_export.conformance.entry_required]

## Phase 5: Manifest, docs, and lifecycle

- [ ] [serial] Add Valence export fixtures to `fixtures/manifest.ncl`, refresh `fixtures/manifest.json`, and update the source-declared projection identity. r[kamacite.valence_receipt_export.command.explicit_inputs]
- [ ] [parallel] Document the Valence export profile, command, non-claims, source-span behavior, artifacts-first coupling boundary, and deferred direct-dependency path. r[kamacite.valence_receipt_export.coupling_boundary] r[kamacite.valence_receipt_export.coupling_boundary.deferred_dependency]
- [ ] [serial] Run final Rust tests, fixture/golden validation, Valence export command checks, manifest projection freshness check, Nickel adapter validation, Cairn validation, and proposal/design/tasks gates. r[kamacite.valence_receipt_export.command]
- [ ] [serial] Sync accepted specs and archive the completed change after implementation evidence is green. r[kamacite.valence_receipt_export.profile]
