# Adapter Conformance Specification

## Purpose

Defines the conformance matrix for Kamacite adapters and target/provenance producers across Nickel, Steel, WebAssembly, LLVM, Haskell/GHC, Rust MIR, Valence exports, and Octet checks.

## Requirements

### Requirement: Adapter conformance matrix

r[kamacite.lossless_ir.adapter_conformance] Every Kamacite adapter, projection, target producer, evidence export, or checker integration MUST have a conformance entry that names its adapter descriptor, input schemas, output schemas, declared effects, checked laws, positive fixtures, negative fixtures, target/provenance role when applicable, and explicit non-claims.

#### Scenario: Adapter without conformance entry is untrusted

r[kamacite.lossless_ir.adapter_conformance.untrusted_without_entry]

- GIVEN a tool emits Kamacite-like artifacts or receipts but has no conformance entry
- WHEN Kamacite validates the result
- THEN validation MUST treat the result as untrusted external input and MUST NOT accept adapter-specific claims.

#### Scenario: Conformance entry names non-claims

r[kamacite.lossless_ir.adapter_conformance.non_claims]

- GIVEN an adapter conformance entry is accepted
- WHEN its receipts are emitted
- THEN each receipt MUST carry the conformance entry identity and MUST preserve the entry's non-claims.

### Requirement: Nickel adapter conformance

r[kamacite.lossless_ir.nickel_adapter_conformance] Nickel conformance entries MUST preserve Nickel source envelopes, identify the Nickel CLI or crate evaluator used for exports, classify `nickel export` output as derived projection evidence, bind syntax/span parser identity when used, and reject claims that Nickel exports replace source identity.

#### Scenario: Nickel source and export are separate

r[kamacite.lossless_ir.nickel_adapter_conformance.source_export_separation]

- GIVEN a Nickel fixture includes source bytes and an exported value
- WHEN Kamacite validates Nickel conformance
- THEN the source envelope identity MUST be distinct from the exported projection identity and the receipt MUST identify the Nickel evaluator or exporter.

#### Scenario: Nickel parser does not prove evaluation

r[kamacite.lossless_ir.nickel_adapter_conformance.parser_non_claim]

- GIVEN a Nickel span fixture uses tree-sitter Nickel or another parser
- WHEN Kamacite validates the parser evidence
- THEN the receipt MUST classify parser output as syntax/span evidence and MUST NOT claim Nickel evaluation semantics.

### Requirement: Steel adapter conformance

r[kamacite.lossless_ir.steel_adapter_conformance] Steel conformance entries MUST declare evaluator identity, DTO input/output schemas, registered host functions, built-in modules, module imports, callable descriptors, contracts, capabilities, effect class, and receipt schema. Ambient authority MUST be denied by default.

#### Scenario: Steel DTO conformance passes without ambient authority

r[kamacite.lossless_ir.steel_adapter_conformance.dto_only]

- GIVEN a Steel adapter fixture consumes only declared DTO inputs and declared Kamacite host functions
- WHEN Kamacite validates the adapter receipt
- THEN validation MAY accept the receipt if evaluator identity, host functions, effects, outputs, and checked laws match the conformance entry.

#### Scenario: Steel ambient authority fails conformance

r[kamacite.lossless_ir.steel_adapter_conformance.ambient_authority]

- GIVEN a Steel fixture requests filesystem, process, TCP, random, time, package, dylib, credential, mutation, or other undeclared authority
- WHEN Kamacite validates Steel conformance
- THEN validation MUST fail closed with an undeclared-authority diagnostic.

### Requirement: WASM adapter conformance

r[kamacite.lossless_ir.wasm_adapter_conformance] WebAssembly conformance entries MUST classify WASM modules or components as sandboxed adapters, validators, or target artifacts. WASM inputs and outputs MUST be Kamacite IR/source-envelope bytes or declared DTOs plus deterministic receipts, and WASM artifacts MUST NOT be accepted as canonical Kamacite IR.

#### Scenario: WASM adapter returns Kamacite receipt

r[kamacite.lossless_ir.wasm_adapter_conformance.receipt]

- GIVEN a WASM adapter consumes declared Kamacite inputs under declared capabilities
- WHEN Kamacite validates its output
- THEN the output MUST include a deterministic Kamacite receipt over Preserves-backed identities.

#### Scenario: WASM canonical IR claim fails conformance

r[kamacite.lossless_ir.wasm_adapter_conformance.not_ir]

- GIVEN a WASM conformance entry or receipt claims compiled WASM is Kamacite canonical IR
- WHEN Kamacite validates the claim
- THEN validation MUST reject the conformance entry or receipt with a canonical-IR overclaim diagnostic.

### Requirement: Compiler target conformance

r[kamacite.lossless_ir.compiler_target_conformance] LLVM, GHC, Rust MIR, object, bytecode, and other compiler target conformance entries MUST classify outputs as target/provenance evidence only. Receipts MUST bind source or input identities, compiler/toolchain identity, target metadata, output artifact hashes, reproducibility limits, and semantic-equivalence non-claims.

#### Scenario: LLVM target conformance binds compiler metadata

r[kamacite.lossless_ir.compiler_target_conformance.llvm_metadata]

- GIVEN an LLVM target artifact is recorded for a Rust validator or adapter
- WHEN Kamacite validates compiler target conformance
- THEN the receipt MUST bind compiler version, target triple, optimization profile, pass pipeline or profile, debug-info status, input identities, output hashes, and non-claims.

#### Scenario: GHC and Rust MIR are provenance only

r[kamacite.lossless_ir.compiler_target_conformance.ghc_mir_provenance]

- GIVEN GHC Core, STG, bytecode, interface files, Rust MIR, or rustc compiler reports are recorded for a fixture
- WHEN Kamacite validates compiler target conformance
- THEN validation MUST classify them as source envelopes or target/provenance artifacts only and MUST reject canonical Kamacite IR claims.

### Requirement: Evidence integration conformance

r[kamacite.lossless_ir.evidence_integration_conformance] Valence exports and Octet checks MUST have conformance entries before their results are used in Kamacite evidence flows. Valence conformance MUST preserve Kamacite receipt claims and non-claims. Octet conformance MUST classify results as Rust source-shape/provenance evidence only.

#### Scenario: Valence export preserves Kamacite claim

r[kamacite.lossless_ir.evidence_integration_conformance.valence]

- GIVEN a Kamacite receipt is exported to a Valence-friendly evidence bundle
- WHEN Kamacite validates evidence integration conformance
- THEN the export MUST preserve Kamacite stable IDs, source spans, roles, hashes, adapter identities, checked laws, and non-claims.

#### Scenario: Octet check does not prove cross-language semantics

r[kamacite.lossless_ir.evidence_integration_conformance.octet]

- GIVEN Octet checks pass for Kamacite Rust code
- WHEN Kamacite records the Octet result
- THEN the receipt MUST classify it as Rust source-shape/provenance evidence and MUST NOT claim Nickel, Steel, Haskell, WASM, LLVM, or arbitrary program semantic equivalence.

### Requirement: Valence export conformance

r[kamacite.valence_receipt_export.conformance] Valence receipt exports MUST have a conformance entry before their results are trusted as Kamacite stack evidence. The conformance entry MUST name the source Kamacite receipt schema, export profile version, output schema, required input identity domains, output identity domain, checked laws, dependency profile, fixture coverage, and explicit non-claims. Exports without a matching conformance entry MUST be treated as untrusted projections.

#### Scenario: Conformance entry is required

r[kamacite.valence_receipt_export.conformance.entry_required]

- GIVEN a tool emits a Valence-like evidence projection from a Kamacite receipt
- WHEN Kamacite validates or records that projection as stack evidence
- THEN validation MUST require a matching Valence export conformance entry before accepting export-specific claims.

#### Scenario: Identity-domain substitution is rejected

r[kamacite.valence_receipt_export.conformance.identity_domains]

- GIVEN a Valence receipt export substitutes a projection, target-artifact, adapter, or source identity for the source Kamacite receipt identity
- WHEN Kamacite validates export conformance
- THEN validation MUST fail closed with an identity-domain diagnostic.

#### Scenario: Missing non-claims are rejected

r[kamacite.valence_receipt_export.conformance.missing_non_claims]

- GIVEN a Valence receipt export omits or weakens non-claims from the source Kamacite receipt
- WHEN Kamacite validates export conformance
- THEN validation MUST reject the export and MUST NOT treat it as a faithful evidence wrapper.

#### Scenario: Semantic-equivalence overclaim is rejected

r[kamacite.valence_receipt_export.conformance.semantic_overclaim]

- GIVEN a Valence receipt export claims Nickel, Rust, Steel, WASM, LLVM, evaluator, compiler, runtime, or arbitrary program semantic equivalence beyond the source Kamacite receipt
- WHEN Kamacite validates export conformance
- THEN validation MUST reject the export with a non-claim or overclaim diagnostic.

#### Scenario: Malformed export input is rejected

r[kamacite.valence_receipt_export.conformance.malformed_input]

- GIVEN a Valence export command receives malformed receipt input, unsupported export-profile version, duplicate artifact identities, missing checked laws, or stale export identity
- WHEN Kamacite validates export conformance
- THEN validation MUST fail closed before emitting a trusted export receipt.

### Requirement: Adapter-boundary audit profiles

r[kamacite.ast_grep_adapter_audits.profile] Kamacite MUST define ast-grep audit profiles for syntax-level adapter boundary checks with declared rule sets, languages, scan scopes, severity posture, and non-claims.

#### Scenario: Adapter audit profile is listed

- GIVEN Kamacite declares an ast-grep adapter-boundary audit profile
- WHEN the validation harness enumerates conformance checks
- THEN the profile MUST be described as structural source-shape evidence rather than semantic adapter proof.

### Requirement: Language boundary identity

r[kamacite.ast_grep_adapter_audits.language_boundary] Kamacite MUST record the built-in ast-grep language or custom tree-sitter parser identity used by each audit profile.

#### Scenario: Custom language is registered

- GIVEN an audit profile uses a custom tree-sitter language
- WHEN the scan receipt is produced
- THEN the receipt MUST bind the dynamic parser identity and file-extension scope before findings are accepted.

### Requirement: BLAKE3 scan identity

r[kamacite.ast_grep_adapter_audits.identity] Kamacite MUST bind ast-grep rule bundles, source-envelope scopes, scan receipts, and finding sets with BLAKE3 identity.

#### Scenario: Source-envelope scope changes

- GIVEN an ast-grep audit receipt names a source-envelope fixture scope
- WHEN the fixture bytes or scope declaration changes
- THEN a fresh scan identity MUST be produced before the receipt can satisfy the audit profile.

### Requirement: Positive and negative fixtures

r[kamacite.ast_grep_adapter_audits.fixtures] Kamacite MUST require positive and negative fixtures before ast-grep rules can participate in adapter-conformance gates.

#### Scenario: Rule has only matching fixture

- GIVEN an ast-grep adapter-boundary rule has a positive fixture but no negative fixture
- WHEN Kamacite evaluates gate eligibility
- THEN the rule MUST remain advisory and MUST NOT block adapter conformance.

### Requirement: Codemods remain migration artifacts

r[kamacite.ast_grep_adapter_audits.codemods] Kamacite MUST preserve ast-grep rewrite recipes as dry-run migration artifacts with explicit before/after source-envelope identities, post-check commands, and non-claims.

#### Scenario: Rewrite changes adapter DTO code

- GIVEN a rewrite recipe proposes changing adapter DTO construction
- WHEN the migration artifact is reviewed
- THEN Kamacite MUST require post-check evidence and MUST NOT claim semantic equivalence solely from the rewrite.

### Requirement: Valence export role

r[kamacite.ast_grep_adapter_audits.valence_export] Kamacite MUST export ast-grep audit findings to Valence only as recorded-only or boundary evidence unless a stronger adapter validator narrows the role.

#### Scenario: Finding is exported

- GIVEN an ast-grep adapter audit finding is wrapped for Valence
- WHEN the export receipt is produced
- THEN it MUST retain source span identity, rule bundle identity, verification role, and non-claim labels.

### Requirement: Audit validation evidence

r[kamacite.ast_grep_adapter_audits.validation] Kamacite MUST validate ast-grep adapter audits with rule tests, fixture scans, receipt validation, Cairn gates, and focused validation-harness checks.

#### Scenario: Audit package is reviewed

- GIVEN the ast-grep adapter audit profile changes
- WHEN validation evidence is assembled
- THEN it MUST include positive fixtures, negative fixtures, scan receipts, lifecycle gates, and Kamacite validation output.

### Requirement: World-commit conformance pins one exact product cohort

r[kamacite.world_commit_conformance.cohort] Kamacite MUST bind the Molten repository, immutable revision, schema identity, codec identity, canonical domain tag, supported version, fixture manifest, license, and non-claims. A sibling path, mutable branch, or implicit latest revision MUST NOT satisfy conformance.

#### Scenario: Immutable cohort is available

- GIVEN every pinned source and fixture identity matches
- WHEN world-commit conformance admission runs
- THEN Kamacite MAY evaluate the cohort

#### Scenario: Molten source revision drifts

- GIVEN fixture bytes come from a revision other than the conformance entry
- WHEN admission runs
- THEN Kamacite MUST reject the corpus before adapter comparison

### Requirement: The conformance entry preserves Molten ownership

r[kamacite.world_commit_conformance.entry] The entry MUST classify Molten as owner of world-commit schema and semantics. Kamacite MUST own only source envelopes, admitted canonical values, adapter descriptors, checked laws, parity receipts, and conformance diagnostics.

#### Scenario: Entry claims branch authority

- GIVEN an adapter entry claims that byte parity authorizes a branch transition
- WHEN entry validation runs
- THEN Kamacite MUST reject the entry as a runtime-authority overclaim

### Requirement: Typed root domains remain distinct

r[kamacite.world_commit_conformance.domains] Kamacite MUST validate the pinned world-commit root-domain inventory and MUST reject substitution between artifact, schema, durable-state, task, history, effect, scheduler, time, entropy, runtime-profile, policy, authority-observation, and opaque-snapshot references.

#### Scenario: Artifact digest occupies the task-root field

- GIVEN the digest bytes are valid but their nominal domain is artifact
- WHEN world-commit admission runs
- THEN Kamacite MUST reject the value as domain substitution

### Requirement: Canonical Preserves bytes are the parity boundary

r[kamacite.world_commit_conformance.parity] The authoritative Rust fixture, admitted Nickel projection, and fresh Steel adapter MUST produce or resolve to the same canonical Kamacite IR value and exact packed Preserves bytes for a passing vector. JSON and source text MUST remain noncanonical projections or envelopes.

#### Scenario: Values match but packed bytes are noncanonical

- GIVEN an adapter decodes to the expected value but emits a noncanonical packed representation
- WHEN parity comparison runs
- THEN Kamacite MUST reject exact-byte conformance

### Requirement: Rust fixtures retain product provenance

r[kamacite.world_commit_conformance.rust] Rust vectors MUST bind Molten DTO source, schema, codec, domain tag, source revision, canonical bytes, and fixture identity. Kamacite MUST NOT regenerate missing authoritative bytes from its own implementation during normal validation.

#### Scenario: Authoritative bytes are absent

- GIVEN only a Kamacite-generated expected value exists
- WHEN Rust parity runs
- THEN the vector MUST remain incomplete

### Requirement: Nickel remains a review projection

r[kamacite.world_commit_conformance.nickel] Nickel fixtures MUST preserve source envelopes and evaluator identity separately from exports. Nickel contracts MAY validate product DTO shape, but JSON export MUST NOT replace canonical Preserves identity.

#### Scenario: Nickel JSON is labeled canonical

- GIVEN a fixture identifies exported JSON bytes as the world-commit identity
- WHEN Nickel conformance runs
- THEN Kamacite MUST reject the fixture

### Requirement: Steel runs without ambient authority

r[kamacite.world_commit_conformance.steel] The Steel adapter MUST run fresh in a declared no-kernel engine with bounded inputs, pure registered host functions, declared modules and contracts, and no ambient filesystem, process, network, clock, random, credential, package, dynamic-library, or mutation authority.

#### Scenario: Steel adapter requests file access

- GIVEN the adapter requests an undeclared filesystem capability
- WHEN fresh conformance execution runs
- THEN Kamacite MUST fail closed before accepting output

### Requirement: Source identities remain separate from value identity

r[kamacite.world_commit_conformance.sources] Rust, Nickel, and Steel source bytes MUST remain source envelopes with their own BLAKE3 identities. Source identity MUST NOT substitute for canonical value or adapter receipt identity.

#### Scenario: Steel source hash is supplied as commit value identity

- GIVEN a valid Steel source-envelope hash occupies the canonical-value field
- WHEN receipt validation runs
- THEN Kamacite MUST reject the identity-domain mismatch

### Requirement: Cohort updates are explicit and convergent

r[kamacite.world_commit_conformance.update] Updating schema, codec, or supported version MUST create a new pinned cohort, dry-run difference report, explicit fixture regeneration, convergence check, and review of removed or widened negative vectors. Unknown versions MUST fail closed.

#### Scenario: Golden bytes change without cohort update

- GIVEN expected packed bytes change while the pinned schema and codec identities remain unchanged
- WHEN freshness validation runs
- THEN Kamacite MUST reject the update

### Requirement: Receipts preserve bounded conformance claims

r[kamacite.world_commit_conformance.receipt] Receipts MUST bind source cohort, conformance entry, schemas, codec, adapters, evaluator identities, source envelopes, fixtures, checked laws, exact results, and non-claims. They MUST NOT claim runtime behavior, evaluator equivalence, restore compatibility, authority, or release eligibility.

#### Scenario: Valence export widens parity into runtime proof

- GIVEN a Valence projection states that adapter parity proves deterministic execution
- WHEN export validation runs
- THEN Kamacite MUST reject the widened claim

### Requirement: Verification includes positive and hostile vectors

r[kamacite.world_commit_conformance.verification] Kamacite MUST test all supported root and profile classes plus malformed, noncanonical, unknown-version, domain-substitution, duplicate, ordering, bound, stale-schema, embedded-evidence, secret, ambient-authority, and overclaim cases.

#### Scenario: Corpus has only positive vectors

- GIVEN all supported values pass but no hostile vectors exist
- WHEN conformance coverage runs
- THEN the world-commit profile MUST remain incomplete
