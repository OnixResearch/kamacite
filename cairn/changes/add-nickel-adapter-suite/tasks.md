## Phase 1: Baseline and Nickel execution boundary

- [ ] [serial] Run the current Rust tests, manifest projection check, harness validation, and Cairn validation before implementation. r[kamacite.nickel_adapter_suite.execution_boundary]
- [ ] [serial] Add an explicit Nickel-capable CLI command or mode that keeps Nickel invocation in the shell and passes normalized DTOs to the core. r[kamacite.nickel_adapter_suite.execution_boundary.shell_only] r[kamacite.nickel_adapter_suite.execution_boundary.explicit_command]
- [ ] [serial] Define the minimal Nickel adapter DTOs for source identity, evaluator identity, export projection identity, checked laws, capabilities, diagnostics, and non-claims. r[kamacite.nickel_adapter_suite.source_export_separation] r[kamacite.nickel_adapter_suite.evaluator_identity]

## Phase 2: Positive fixture coverage

- [ ] [parallel] Add a valid Nickel source/export fixture that binds source bytes, evaluator identity, projection bytes, projection identity, checked laws, and non-claims. r[kamacite.nickel_adapter_suite.positive_fixtures.valid_export] r[kamacite.nickel_adapter_suite.source_export_separation.derived_projection]
- [ ] [parallel] Add evaluator-identity-bound receipt coverage that records Nickel reference evaluator version/profile data without granting authority or semantic-equivalence claims. r[kamacite.nickel_adapter_suite.evaluator_identity.bound]
- [ ] [parallel] Add or preserve parser/span evidence as syntax-only evidence when parser fixtures are included. r[kamacite.nickel_adapter_suite.positive_fixtures.parser_non_claim]
- [ ] [serial] Add deterministic golden receipts for positive Nickel adapter fixtures. r[kamacite.nickel_adapter_suite.golden_receipts.match]

## Phase 3: Negative fixture coverage

- [ ] [parallel] Add missing-source-envelope negative fixture coverage. r[kamacite.nickel_adapter_suite.negative_fixtures.missing_source]
- [ ] [parallel] Add stale export projection identity negative fixture coverage. r[kamacite.nickel_adapter_suite.negative_fixtures.stale_projection]
- [ ] [parallel] Add JSON-export-as-canonical-IR overclaim negative fixture coverage. r[kamacite.nickel_adapter_suite.source_export_separation.json_not_canonical]
- [ ] [parallel] Add missing and unsupported evaluator identity negative fixture coverage. r[kamacite.nickel_adapter_suite.evaluator_identity.missing] r[kamacite.nickel_adapter_suite.evaluator_identity.unsupported]
- [ ] [parallel] Add malformed export output negative fixture coverage. r[kamacite.nickel_adapter_suite.negative_fixtures.malformed_export]
- [ ] [parallel] Add undeclared host-effect negative fixture coverage for Nickel adapter receipts. r[kamacite.nickel_adapter_suite.negative_fixtures.undeclared_effect] r[kamacite.nickel_adapter_suite.capability_boundary]

## Phase 4: Manifest, docs, and update workflow

- [ ] [serial] Add Nickel adapter fixtures to `fixtures/manifest.ncl`, refresh the runtime projection, and update the source-declared projection identity. r[kamacite.nickel_adapter_suite.golden_receipts] r[kamacite.nickel_adapter_suite.capability_boundary.root_relative]
- [ ] [parallel] Document the Nickel-capable validation command, declared capabilities, non-claims, and deferred-suite relationship. r[kamacite.nickel_adapter_suite.execution_boundary.explicit_command]
- [ ] [parallel] Ensure golden receipt update evidence records changed source/export/evaluator/receipt identities. r[kamacite.nickel_adapter_suite.golden_receipts.explicit_update]

## Phase 5: Final verification and lifecycle

- [ ] [serial] Run Rust tests, Nickel adapter positive/negative fixtures, manifest projection freshness check, harness validation, Clippy, Cairn validation, and proposal/design/tasks gates. r[kamacite.nickel_adapter_suite.golden_receipts.match]
- [ ] [serial] Sync accepted specs and archive the completed change. r[kamacite.nickel_adapter_suite.execution_boundary]
