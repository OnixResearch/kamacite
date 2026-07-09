## Phase 1: Baseline and Steel execution boundary

- [ ] [serial] Run baseline Rust tests, default harness validation, Nickel adapter validation, Valence export validation, manifest projection freshness check, and Cairn validation before implementation. r[kamacite.steel_adapter_suite.execution_boundary]
- [ ] [serial] Add an explicit Steel-capable CLI command or mode that keeps Steel invocation or embedded-engine evaluation in the shell and passes normalized DTOs to the core. r[kamacite.steel_adapter_suite.execution_boundary.shell_only] r[kamacite.steel_adapter_suite.execution_boundary.explicit_command] r[kamacite.steel_adapter_suite.command]
- [ ] [serial] Preserve default validation as stored-evidence-only for Steel fixtures. r[kamacite.steel_adapter_suite.execution_boundary.stored_evidence] r[kamacite.steel_adapter_suite.command.default_receipt_only]

## Phase 2: Receipt profile and pure core DTOs

- [ ] [serial] Define minimal Steel adapter DTOs for source identity, evaluator identity, module or descriptor identity, input/output DTO schemas, host functions, imports, capabilities, effect class, output projection identity, diagnostics, checked laws, and non-claims. r[kamacite.steel_adapter_suite.receipt_profile]
- [ ] [serial] Add pure core validation for Steel evaluator identity, source/module identity separation, output projection identity, stale identity rejection, and wrong-domain rejection. r[kamacite.steel_adapter_suite.receipt_profile.evaluator_identity] r[kamacite.steel_adapter_suite.receipt_profile.source_module_identity] r[kamacite.steel_adapter_suite.receipt_profile.output_projection]
- [ ] [serial] Add pure core validation for declared host functions, built-in modules, imports, effect classes, and undeclared host-function rejection. r[kamacite.steel_adapter_suite.host_function_boundary] r[kamacite.steel_adapter_suite.host_function_boundary.declared_host_functions] r[kamacite.steel_adapter_suite.host_function_boundary.undeclared_host_function]
- [ ] [serial] Add pure core validation for DTO-only outputs, opaque closure rejection, and bytecode or JSON canonical-IR overclaim rejection. r[kamacite.steel_adapter_suite.output_boundary] r[kamacite.steel_adapter_suite.output_boundary.dto_only] r[kamacite.steel_adapter_suite.output_boundary.opaque_closure] r[kamacite.steel_adapter_suite.output_boundary.bytecode_not_ir]

## Phase 3: Shell execution and command behavior

- [ ] [serial] Implement Steel command argument parsing, explicit input loading, root-relative path checks, evaluator identity capture, declared host-function registration, declared module/import loading, output capture, and deterministic report rendering in the CLI shell. r[kamacite.steel_adapter_suite.command.explicit_inputs] r[kamacite.steel_adapter_suite.capability_boundary.root_relative] r[kamacite.steel_adapter_suite.capability_boundary.evaluator_invocation]
- [ ] [serial] Make the Steel command non-mutating by default and fail closed for malformed, stale, unsupported, overclaiming, or undeclared-capability evidence. r[kamacite.steel_adapter_suite.command.non_mutating] r[kamacite.steel_adapter_suite.command.fail_closed]
- [ ] [serial] Add explicit update/report behavior for refreshed Steel adapter golden receipts or output projections, including old/new identities and non-claims. r[kamacite.steel_adapter_suite.command.explicit_update] r[kamacite.steel_adapter_suite.golden_receipts.explicit_update]

## Phase 4: Positive fixture coverage

- [ ] [parallel] Add a positive DTO-only Steel fixture that binds declared DTO schemas, evaluator identity, declared host functions, declared modules/imports, declared capabilities, output projection identity, checked laws, and non-claims. r[kamacite.steel_adapter_suite.positive_fixtures] r[kamacite.steel_adapter_suite.positive_fixtures.dto_only]
- [ ] [parallel] Add positive coverage for declared module/import descriptors without ambient authority. r[kamacite.steel_adapter_suite.positive_fixtures.declared_module]
- [ ] [parallel] Add deterministic golden receipt coverage for accepted Steel fixtures. r[kamacite.steel_adapter_suite.golden_receipts] r[kamacite.steel_adapter_suite.golden_receipts.match]

## Phase 5: Negative fixture coverage

- [ ] [parallel] Add missing and unsupported evaluator identity negative fixture coverage. r[kamacite.steel_adapter_suite.negative_fixtures] r[kamacite.steel_adapter_suite.negative_fixtures.missing_evaluator] r[kamacite.steel_adapter_suite.negative_fixtures.unsupported_evaluator]
- [ ] [parallel] Add undeclared host-function and ambient import/authority negative fixture coverage. r[kamacite.steel_adapter_suite.host_function_boundary.ambient_import] r[kamacite.steel_adapter_suite.negative_fixtures.ambient_authority] r[kamacite.steel_adapter_suite.capability_boundary.mismatch]
- [ ] [parallel] Add stale output identity and malformed output negative fixture coverage. r[kamacite.steel_adapter_suite.negative_fixtures.stale_output] r[kamacite.steel_adapter_suite.negative_fixtures.malformed_output]
- [ ] [parallel] Add opaque closure export and Steel bytecode or JSON canonical-IR overclaim negative fixture coverage. r[kamacite.steel_adapter_suite.output_boundary.opaque_closure] r[kamacite.steel_adapter_suite.output_boundary.bytecode_not_ir]

## Phase 6: Manifest, docs, and lifecycle

- [ ] [serial] Add Steel adapter fixtures to `fixtures/manifest.ncl`, refresh `fixtures/manifest.json`, and update the source-declared projection identity. r[kamacite.steel_adapter_suite.command.explicit_inputs]
- [ ] [parallel] Document the Steel-capable validation command, declared capabilities, evaluator identity, host-function/import boundary, callable descriptor boundary, non-claims, and deferred-suite relationship. r[kamacite.steel_adapter_suite.command] r[kamacite.steel_adapter_suite.capability_boundary] r[kamacite.steel_adapter_suite.output_boundary]
- [ ] [serial] Run final Rust tests, default fixture/golden validation, Steel adapter command checks, Nickel adapter validation, Valence export validation, manifest projection freshness check, Clippy if available, Cairn validation, and proposal/design/tasks gates. r[kamacite.steel_adapter_suite.golden_receipts.match] r[kamacite.steel_adapter_suite.command.fail_closed]
- [ ] [serial] Sync accepted specs and archive the completed change after implementation evidence is green. r[kamacite.steel_adapter_suite.execution_boundary]
