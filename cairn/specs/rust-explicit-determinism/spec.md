# Rust Explicit Determinism Specification

## Purpose

Defines the Rust implementation ratchets Kamacite must satisfy before Rust validators, canonicalization helpers, adapter DTO checks, or receipt constructors are trusted: pure core boundaries, typed identity domains, closed artifact classes, Preserves-backed canonical identity, deterministic diagnostics, explicit capabilities, fixture admission, crate-level rails, and a deliberately small first implementation slice.

## Requirements

### Requirement: Rust deterministic admission role

r[kamacite.rust_determinism.core_role] Kamacite Rust MUST act as a deterministic admission controller over explicit in-memory inputs for trusted validation behavior. Core Rust code MAY validate, canonicalize, construct hash material, classify artifacts, check adapter DTOs, and construct bounded receipts. Core Rust code MUST NOT execute Nickel, Steel, WASM, compilers, network clients, filesystem traversals, environment reads, process execution, clocks, random sources, credential lookups, presentation, or mutation.

#### Scenario: Core validates explicit inputs only

r[kamacite.rust_determinism.core_role.explicit_inputs]

- GIVEN core Rust validation receives source-envelope bytes, canonical bytes, schemas, adapter descriptors, declared capabilities, dependency identities, and receipt fields as in-memory inputs
- WHEN the core classifies the artifact and validates the receipt
- THEN the result MUST depend only on those explicit inputs and MUST NOT depend on filesystem, environment, process, evaluator, compiler, network, clock, random, credential, presentation, or mutable host state.

#### Scenario: Evaluator execution stays outside the core

r[kamacite.rust_determinism.core_role.evaluator_outside_core]

- GIVEN a future adapter shell invokes Nickel, Steel, WASM, a compiler, or another evaluator
- WHEN the shell asks the Rust core to validate the resulting evidence
- THEN the shell MUST provide explicit evaluator identity, declared inputs, declared capabilities, output artifacts, and receipt fields, and the core MUST NOT call the evaluator itself.

### Requirement: Functional core and imperative shell separation

r[kamacite.rust_determinism.core_shell_boundary] Kamacite Rust MUST separate pure deterministic core logic from any imperative shell that performs file discovery, file reads, environment inspection, process execution, evaluator calls, compiler calls, network access, clock or random access, credential access, mutation, stdout or stderr rendering, exit-code mapping, or human presentation.

#### Scenario: Shell normalizes facts before validation

r[kamacite.rust_determinism.core_shell_boundary.normalized_facts]

- GIVEN a CLI or adapter runner discovers fixtures on disk and reads artifact bytes
- WHEN it invokes the core validator
- THEN it MUST pass normalized in-memory DTOs and bytes to the core instead of giving the core paths, file handles, environment variables, process handles, clocks, random generators, or presentation sinks.

#### Scenario: Core result can be unit tested without host services

r[kamacite.rust_determinism.core_shell_boundary.unit_testable]

- GIVEN the same explicit input DTOs and bytes are supplied to a core validator in a unit test and in a CLI run
- WHEN the core validator completes
- THEN it MUST produce the same validation result, diagnostics, and receipt fields without requiring mocks for filesystem, process, network, clock, random, credentials, or evaluator services.

### Requirement: Typed identity domains in Rust

r[kamacite.rust_determinism.typed_identity_domains] Kamacite Rust MUST represent source identity, canonical value identity, projection identity, target-artifact identity, adapter identity, evaluator identity, compiler identity, and receipt identity as distinct typed domains. Raw strings, byte arrays, or generic hash types MUST NOT be accepted interchangeably across these domains in trusted validation APIs.

#### Scenario: Projection identity is rejected as canonical identity

r[kamacite.rust_determinism.typed_identity_domains.projection_not_canonical]

- GIVEN a JSON projection identity is supplied where a Preserves canonical value identity is required
- WHEN the Rust core validates the artifact or receipt
- THEN validation MUST reject the substitution with a deterministic identity-domain diagnostic and MUST NOT coerce the projection identity into a canonical value identity.

#### Scenario: Source identity is rejected as receipt identity

r[kamacite.rust_determinism.typed_identity_domains.source_not_receipt]

- GIVEN a source-envelope identity is supplied where a receipt identity is required
- WHEN the Rust core validates the receipt
- THEN validation MUST reject the substitution unless a distinct receipt identity explicitly binds the source identity as an input field.

#### Scenario: Constructors validate domain tags

r[kamacite.rust_determinism.typed_identity_domains.constructor_validation]

- GIVEN a serialized identity includes hash bytes and a declared identity domain
- WHEN Rust constructs the typed identity value
- THEN construction MUST verify that the declared domain matches the requested Rust type or return a deterministic identity-domain diagnostic.

### Requirement: Closed artifact alternatives

r[kamacite.rust_determinism.closed_artifacts] Kamacite Rust MUST model trusted artifact classes as closed alternatives with explicit variants for source envelope, canonical value, adapter descriptor, target artifact, receipt, and unsupported claim. Unknown schema versions, codecs, artifact classes, target kinds, adapter capabilities, effect classes, or claim kinds MUST fail closed rather than defaulting to a supported variant.

#### Scenario: Unknown artifact class fails closed

r[kamacite.rust_determinism.closed_artifacts.unknown_class]

- GIVEN a well-formed encoded artifact names an artifact class not supported by the current Rust model
- WHEN the core classifies the artifact
- THEN validation MUST return an unsupported-artifact-class diagnostic and MUST NOT classify it as the nearest supported artifact kind.

#### Scenario: Unsupported version fails closed

r[kamacite.rust_determinism.closed_artifacts.unsupported_version]

- GIVEN an artifact uses a syntactically valid schema version that the current validator does not support
- WHEN the core validates the artifact
- THEN validation MUST return an unsupported-version diagnostic and MUST NOT accept the artifact through compatibility defaults.

#### Scenario: Unsupported claim remains unsupported

r[kamacite.rust_determinism.closed_artifacts.unsupported_claim]

- GIVEN an artifact claims arbitrary source translation, semantic equivalence of host languages, compiler correctness, evaluator soundness, runtime authorization, or another unsupported Kamacite claim
- WHEN the Rust core classifies the claim
- THEN it MUST preserve or report the claim as unsupported and MUST NOT promote it to a trusted source-envelope, canonical-value, adapter, target, or receipt claim.

### Requirement: Preserves-backed canonical identity

r[kamacite.rust_determinism.canonical_identity] Where Kamacite controls identity, Rust MUST compute content identity from Kamacite-owned canonical Preserves bytes with BLAKE3. JSON, Serde DTOs, textual debug output, filesystem paths, human names, labels, aliases, or presentation strings MUST NOT replace canonical Preserves bytes as identity material.

#### Scenario: JSON projection has separate identity

r[kamacite.rust_determinism.canonical_identity.json_projection_separate]

- GIVEN a canonical value can be exported as JSON for compatibility
- WHEN Rust computes identities for the canonical value and JSON projection
- THEN the canonical value identity MUST be computed from canonical Preserves bytes, the JSON projection identity MUST remain a separate projection identity, and validation MUST reject attempts to use the projection identity as canonical identity.

#### Scenario: Names do not affect identity

r[kamacite.rust_determinism.canonical_identity.names_metadata]

- GIVEN two artifacts have the same canonical Preserves bytes and different human names, paths, labels, or aliases
- WHEN Rust computes canonical identity
- THEN both artifacts MUST have the same canonical identity and the names, paths, labels, or aliases MUST remain metadata only.

#### Scenario: BLAKE3 is used for stack-owned identity

r[kamacite.rust_determinism.canonical_identity.blake3]

- GIVEN Kamacite owns the canonical bytes for a source envelope, canonical value, adapter descriptor, target artifact, or receipt
- WHEN Rust computes the content identity
- THEN it MUST use BLAKE3 over the Kamacite-owned canonical bytes unless an explicit interop requirement names another algorithm for a non-canonical external surface.

### Requirement: Deterministic ordering for canonical paths

r[kamacite.rust_determinism.deterministic_ordering] Kamacite Rust MUST make validation output, diagnostic lists, receipt fields, canonical collection traversal, fixture reports, and serialized evidence deterministic. Any unordered internal collection MUST be sorted or converted into a deterministic representation before hashing, serializing, comparing, or presenting evidence.

#### Scenario: Receipt dependencies are stable

r[kamacite.rust_determinism.deterministic_ordering.dependencies]

- GIVEN a receipt binds multiple dependency identities discovered or supplied in arbitrary order
- WHEN Rust emits canonical receipt material
- THEN the dependency identities MUST appear in a deterministic order that is independent of discovery order, hash-map seed, filesystem ordering, locale, or host platform.

#### Scenario: Diagnostic order is stable

r[kamacite.rust_determinism.deterministic_ordering.diagnostics]

- GIVEN validation finds multiple malformed fields or unsupported claims in one artifact
- WHEN Rust returns diagnostics
- THEN the diagnostics MUST be ordered by a deterministic rule such as artifact path, field path, span, diagnostic code, or another declared stable key.

#### Scenario: Host error wording does not enter core diagnostics

r[kamacite.rust_determinism.deterministic_ordering.host_errors]

- GIVEN an imperative shell observes a host-specific filesystem, process, network, evaluator, or compiler error
- WHEN it asks the Rust core to validate normalized evidence
- THEN host-specific wording MUST NOT become a core diagnostic unless it is first normalized into a deterministic diagnostic code and explicit data fields.

### Requirement: Total deterministic diagnostics

r[kamacite.rust_determinism.total_diagnostics] Kamacite Rust validation paths MUST return explicit deterministic diagnostics for malformed, unsupported, stale, lossy, undeclared, ambiguous, or cross-domain inputs. Trusted validation paths MUST NOT panic, unwrap, silently default, truncate, coerce across identity domains, or accept partial success for rejected artifacts.

#### Scenario: Malformed input returns diagnostic

r[kamacite.rust_determinism.total_diagnostics.malformed]

- GIVEN malformed artifact bytes, invalid DTO fields, or invalid identity encodings are supplied to the core
- WHEN Rust attempts to decode or validate the input
- THEN validation MUST return deterministic diagnostics naming the failing field, span, schema, artifact class, or identity domain when available and MUST NOT panic or accept a partial artifact.

#### Scenario: Stale hash returns diagnostic

r[kamacite.rust_determinism.total_diagnostics.stale_hash]

- GIVEN artifact bytes and a declared identity do not match under the expected canonical hash rule
- WHEN the Rust core validates the artifact
- THEN validation MUST reject the artifact with a stale-identity diagnostic that identifies the expected identity domain and artifact class.

#### Scenario: Ambiguous claim returns diagnostic

r[kamacite.rust_determinism.total_diagnostics.ambiguous_claim]

- GIVEN a receipt mixes source preservation, canonical value round trip, target provenance, semantic equivalence, or runtime authorization claims without declaring the bounded claim class
- WHEN Rust validates the receipt
- THEN validation MUST reject it with an ambiguous-claim diagnostic and MUST NOT infer a trusted claim class.

### Requirement: Explicit effect and capability validation

r[kamacite.rust_determinism.explicit_capabilities] Kamacite Rust MUST treat host effects and capabilities as explicit declared data before accepting adapter or evaluator evidence. Filesystem, process, network, clock, random, credential, mutation, compiler, evaluator, device, package-manager, dynamic-library, and other host authority MUST be denied by default unless declared by the relevant adapter descriptor and receipt.

#### Scenario: Undeclared filesystem access is rejected

r[kamacite.rust_determinism.explicit_capabilities.filesystem]

- GIVEN an adapter receipt indicates filesystem access occurred but the adapter descriptor did not declare filesystem capability
- WHEN Rust validates the receipt
- THEN validation MUST reject the receipt with an undeclared-capability diagnostic.

#### Scenario: Algebraic operation does not grant host authority

r[kamacite.rust_determinism.explicit_capabilities.operation_not_authority]

- GIVEN an algebraic-effect row declares an operation such as reading configuration, obtaining time, or requesting randomness
- WHEN Rust validates adapter or handler evidence for that operation
- THEN validation MUST NOT treat the operation declaration as host filesystem, clock, random, network, credential, process, or mutation authority unless a separate capability declaration admits that authority.

#### Scenario: Evaluator identity is not a capability grant

r[kamacite.rust_determinism.explicit_capabilities.evaluator_not_capability]

- GIVEN a receipt names a trusted evaluator, compiler, runtime, or adapter identity
- WHEN Rust validates host effects recorded by the receipt
- THEN validation MUST still require explicit capability declarations and MUST NOT infer authority from the evaluator, compiler, runtime, or adapter name.

### Requirement: Capability-oriented shell admission

r[kamacite.rust_determinism.capability_shells] Kamacite shell or adapter-runner code that performs filesystem or network IO SHOULD prefer capability-oriented APIs such as `cap-std`, WASI, or an equivalent declared-capability boundary over ambient `std` APIs. Capability handles MUST remain shell authority only: the trusted Rust core MUST receive normalized DTOs, declared capability facts, artifact bytes, diagnostics, and receipts rather than filesystem handles, network handles, ambient paths, or process-wide authority.

#### Scenario: Shell receives a declared filesystem root

r[kamacite.rust_determinism.capability_shells.filesystem_root]

- GIVEN a Kamacite CLI, fixture runner, or adapter shell is granted filesystem authority for a reviewed artifact root
- WHEN it opens user-controlled relative paths under that root
- THEN it SHOULD use `cap_std::fs::Dir` or an equivalent root-capability API and MUST NOT treat arbitrary absolute paths, current-directory-relative paths, or environment-derived paths as trusted artifact authority.

#### Scenario: Core receives normalized evidence instead of handles

r[kamacite.rust_determinism.capability_shells.no_core_handles]

- GIVEN shell IO was mediated by `cap-std`, WASI, or another capability boundary
- WHEN the shell invokes trusted Rust core validation
- THEN it MUST pass only explicit artifact bytes, typed identities, declared capabilities, normalized host-error codes, and receipt fields, and MUST NOT pass capability handles or ask the core to open paths or sockets.

#### Scenario: Path escape attempts are negative fixtures

r[kamacite.rust_determinism.capability_shells.escape_fixtures]

- GIVEN a shell admits filesystem-backed fixtures, source envelopes, or adapter inputs through a declared root capability
- WHEN the fixture corpus for that shell is evaluated
- THEN it MUST include negative cases for parent traversal, absolute paths, symlink escape, and undeclared filesystem capability, and each rejected case MUST produce a deterministic diagnostic or normalized denial code.

#### Scenario: Capability APIs are not an untrusted-code sandbox

r[kamacite.rust_determinism.capability_shells.not_sandbox]

- GIVEN Kamacite may execute untrusted or review-pending Rust, Steel, WASM, compiler, or adapter code
- WHEN the execution boundary is selected
- THEN `cap-std` alone MUST NOT be treated as a sandbox or authorization proof; untrusted execution MUST remain outside trusted claim admission or use a separately declared isolation boundary with explicit imports, capabilities, receipts, and non-claims.

### Requirement: Rust fixture admission gate

r[kamacite.rust_determinism.fixture_gate] New Rust validation behavior MUST NOT be considered trusted until it has positive fixtures, negative fixtures, stale-identity fixtures, wrong-identity-domain fixtures, unsupported-version fixtures, malformed-input fixtures, and golden-receipt fixtures that exercise the behavior and its deterministic diagnostics.

#### Scenario: Happy path alone is insufficient

r[kamacite.rust_determinism.fixture_gate.positive_only_insufficient]

- GIVEN a new Rust validator accepts a well-formed source envelope, canonical value, adapter descriptor, target artifact, or receipt
- WHEN the fixture corpus contains only acceptance cases for that validator
- THEN the behavior MUST remain untrusted for Kamacite claim admission until matching negative and golden-receipt fixtures exist.

#### Scenario: Wrong identity domain fixture is required

r[kamacite.rust_determinism.fixture_gate.wrong_domain]

- GIVEN a validator accepts a receipt that binds typed identities
- WHEN the fixture corpus is evaluated
- THEN it MUST include a fixture that substitutes at least one incompatible identity domain and verifies deterministic rejection.

#### Scenario: Golden receipt proves deterministic emission

r[kamacite.rust_determinism.fixture_gate.golden_receipt]

- GIVEN a Rust validator emits a receipt or validation report
- WHEN the same fixture is run repeatedly with the same explicit inputs
- THEN the emitted receipt or report MUST match a golden artifact exactly except for fields explicitly declared outside the core and excluded from canonical identity.

### Requirement: Rust crate-level rails

r[kamacite.rust_determinism.crate_rails] Trusted Kamacite Rust core paths MUST forbid unsafe code, deny debug-only or panic-prone helpers in validation logic, avoid generalized shell error types in pure domain APIs, and expose domain-specific result and diagnostic types. Generalized error plumbing MAY exist in an imperative shell only after explicit core DTOs and diagnostics have been produced.

#### Scenario: Unsafe code is forbidden in trusted core

r[kamacite.rust_determinism.crate_rails.unsafe_forbidden]

- GIVEN a Rust crate or module is part of the trusted Kamacite core validation path
- WHEN the crate or module is checked
- THEN unsafe code MUST be forbidden for that trusted core path unless a later Cairn change explicitly narrows and justifies an exception.

#### Scenario: Panic-prone helpers are denied in validation logic

r[kamacite.rust_determinism.crate_rails.no_panic_helpers]

- GIVEN Rust validation logic handles external artifacts, fixtures, receipts, identities, or adapter evidence
- WHEN code review or automated checks inspect that validation path
- THEN panic-prone and debug-only helpers such as unchecked unwrap-style extraction, todo placeholders, unreachable assumptions over external data, debug printing, or direct process exits MUST be rejected from the trusted core path.

#### Scenario: Core APIs return domain diagnostics

r[kamacite.rust_determinism.crate_rails.domain_errors]

- GIVEN a core Rust API validates Kamacite artifacts or constructs receipts
- WHEN it reports failure
- THEN it MUST return domain-specific diagnostics or result types rather than shell-oriented opaque error plumbing.

### Requirement: Kamacite Rust Core Profile naming boundary

r[kamacite.rust_determinism.profile_prior_art] Kamacite MUST describe its restricted trusted Rust surface as the Kamacite Rust Core Profile or an equivalently bounded implementation profile. Kamacite MUST NOT describe this profile as a new Rust dialect, Rust fork, compiler qualification, safety certification, OxCaml-style language extension, or proof of arbitrary Rust program semantics.

#### Scenario: Ferrocene and Verus are prior art, not authority

r[kamacite.rust_determinism.profile_prior_art.ferrocene_verus]

- GIVEN Kamacite cites Ferrocene, Sealed Rust, or Verus as prior art for restricted or verified Rust surfaces
- WHEN Kamacite documents its own Rust implementation boundary
- THEN it MUST state that Kamacite borrows the profile discipline only and MUST NOT claim Ferrocene-style compiler qualification, Verus-style proof coverage, or a new Rust language level.

#### Scenario: Profile claim stays implementation-scoped

r[kamacite.rust_determinism.profile_prior_art.implementation_scoped]

- GIVEN a Kamacite Rust crate satisfies the Kamacite Rust Core Profile checks
- WHEN Kamacite records the result in a receipt
- THEN the receipt MAY claim conformance to the Kamacite implementation profile but MUST NOT claim general Rust correctness, compiler soundness, evaluator soundness, host authorization, or semantic equivalence across Nickel, Rust, Steel, WASM, or compiler artifacts.

### Requirement: Bounded prior-art role adoption

r[kamacite.rust_determinism.prior_art_roles] Kamacite MUST assign a bounded role and explicit non-claims before adopting a prior-art pattern into the Rust profile. Prior-art projects MUST remain references for specific design concerns rather than authorities for Kamacite canonical identity, source preservation, receipt validity, runtime behavior, proof semantics, supply-chain policy, or diagnostic presentation.

#### Scenario: HIF informs explicit host-function boundaries

r[kamacite.rust_determinism.prior_art_roles.hif]

- GIVEN Kamacite adapts ideas from HIF's bounded machine model, explicit host functions, runtime limits, and explicit errors
- WHEN Kamacite defines adapter or probe execution evidence
- THEN Kamacite MUST keep those ideas as host-function and limit-modeling guidance and MUST NOT adopt HIF bytecode, stack semantics, postcard serialization, or Hubris/Humility debugging behavior as Kamacite canonical IR.

#### Scenario: Capability systems inform shells only

r[kamacite.rust_determinism.prior_art_roles.capabilities]

- GIVEN Kamacite adapts ideas from `cap-std`, WASI, WASI Virt, or Wasmtime deterministic-execution guidance
- WHEN Kamacite designs a CLI, adapter runner, or WASM shell
- THEN capabilities MAY be represented as explicit shell inputs or adapter descriptors, but the pure Rust core MUST still receive only normalized DTOs, declarations, artifact bytes, and receipts and MUST NOT receive filesystem, network, clock, random, environment, process, or credential authority.

#### Scenario: Oxide schema and identity projects inform boundaries

r[kamacite.rust_determinism.prior_art_roles.oxide_schema_identity]

- GIVEN Kamacite adapts ideas from `iddqd`, `typify`, `openapi-lint`, `drift`, `daft`, or similar schema/identity projects
- WHEN Kamacite validates identities, schemas, projections, or compatibility evidence
- THEN Kamacite MAY borrow value-owned identity, fail-closed duplicate handling, honest schema-translation limits, valid-but-bad-interchange rejection, explicit unhandled cases, and typed structural comparison patterns while preserving Kamacite-owned Preserves identity, BLAKE3 hashes, and receipt semantics.

### Requirement: Source-shape and supply-chain rails are evidence only

r[kamacite.rust_determinism.source_provenance_rails] Kamacite MAY use Octet, Clippy, Dylint, cargo-geiger, Safety Dance practices, cargo-vet, cargo-deny, cargo-auditable, or similar Rust source-shape and supply-chain tools as implementation evidence. These rails MUST NOT replace Kamacite core validation, fixture admission, canonical identity checks, adapter conformance, or receipt validation.

#### Scenario: Lint pass does not validate Kamacite receipt semantics

r[kamacite.rust_determinism.source_provenance_rails.lints]

- GIVEN Clippy, Dylint, Octet, or another lint rail reports that a trusted core path avoids unsafe code, panic-prone helpers, or ambient IO
- WHEN Kamacite evaluates an artifact or receipt
- THEN the lint result MAY support Rust implementation evidence but MUST NOT by itself make the artifact or receipt semantically valid.

#### Scenario: Dependency evidence does not prove deterministic behavior

r[kamacite.rust_determinism.source_provenance_rails.supply_chain]

- GIVEN cargo-vet, cargo-deny, cargo-auditable, or another supply-chain rail records dependency audits, policy conformance, or embedded dependency metadata
- WHEN Kamacite records Rust implementation evidence
- THEN the result MAY support dependency provenance but MUST NOT prove Kamacite canonicalization, deterministic diagnostics, capability denial, or adapter-law conformance.

### Requirement: Optional proof rails cannot replace fixtures

r[kamacite.rust_determinism.optional_proof_rails] Kamacite MAY use Kani, Prusti, Creusot, Verus, Trellis, or similar proof/model-checking tools as supporting evidence for bounded Rust-core properties. Proof or model-checking evidence MUST NOT replace positive fixtures, negative fixtures, stale-identity fixtures, wrong-domain fixtures, unsupported-version fixtures, golden receipts, or Kamacite receipt validation unless a later Cairn explicitly narrows a replacement claim.

#### Scenario: Proof evidence supports but does not replace fixture gate

r[kamacite.rust_determinism.optional_proof_rails.fixture_gate]

- GIVEN a Rust core function has proof or model-checking evidence for panic absence, overflow absence, assertion validity, or a bounded functional property
- WHEN Kamacite admits new validation behavior
- THEN the proof evidence MAY be recorded as supporting evidence, but the fixture gate MUST still include positive, negative, identity-domain, stale-hash, unsupported-version, malformed-input, and golden-receipt coverage for the admitted behavior.

#### Scenario: Verification subset is not Kamacite source language

r[kamacite.rust_determinism.optional_proof_rails.not_language]

- GIVEN a Rust verifier supports only a subset or annotated form of Rust
- WHEN Kamacite records verifier output
- THEN Kamacite MUST classify it as bounded implementation evidence and MUST NOT treat the verifier's subset, annotations, intermediate language, or proof artifacts as Kamacite canonical IR.

### Requirement: First Rust implementation slice

r[kamacite.rust_determinism.first_slice] The first trusted Kamacite Rust implementation slice SHOULD be limited to source envelope DTOs, BLAKE3 source identity, typed identity wrappers, closed artifact classification, bounded non-claim receipts, deterministic diagnostics, fixture runners over explicit inputs, and golden validation receipts. Nickel, Steel, WASM, LLVM, algebraic-effect runtime integration, Valence export, and Octet check integration SHOULD remain outside the first slice unless they are represented only as preserved source, target evidence, or explicit non-claim fixtures.

#### Scenario: Source envelope slice lands first

r[kamacite.rust_determinism.first_slice.source_envelope]

- GIVEN the repository adds its first trusted Rust crate
- WHEN implementation scope is selected
- THEN source envelope DTOs, source BLAKE3 identity, typed identity wrappers, closed artifact classification, bounded non-claim receipts, deterministic diagnostics, and fixtures SHOULD land before adapter execution, compiler target production, or evidence-export integrations.

#### Scenario: Adapter integration waits for core fixtures

r[kamacite.rust_determinism.first_slice.adapter_waits]

- GIVEN a future Nickel, Steel, WASM, LLVM, Valence, Octet, or algebraic-effect integration depends on Rust validation behavior
- WHEN the core fixture gate for that behavior has not passed
- THEN the integration MUST remain untrusted for Kamacite claim admission or be represented only as external evidence with explicit non-claims.
