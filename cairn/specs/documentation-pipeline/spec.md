# Documentation Pipeline Specification

## Purpose

Defines Kamacite's bounded documentation artifact model: documentation IR, rendered HTML/Markdown/AsciiDoc projections, runnable documentation cells, documentation commands, deterministic receipts, fixture admission, and stack roles.

## Requirements

### Requirement: Documentation IR projection boundary

r[kamacite.docs.ir_projection] Kamacite MUST model trusted documentation facts as Kamacite-owned documentation IR artifacts with Preserves-backed canonical identity. Documentation IR MUST represent pages, sections, anchors, links, prose blocks, code blocks, runnable cells, diagnostics, source spans, dependency identities, metadata, and non-claims. Original human-authored documentation or example source bytes MUST remain source envelopes with BLAKE3 identities when source fidelity matters.

#### Scenario: Rendered output does not replace documentation IR

r[kamacite.docs.ir_projection.rendered_not_canonical]

- GIVEN a documentation IR artifact is rendered to HTML, Markdown, or AsciiDoc
- WHEN Kamacite computes or reports documentation identity
- THEN the rendered output MUST be classified as a projection and MUST NOT replace the documentation IR identity or the original source-envelope identity.

#### Scenario: Source envelopes stay separate from projections

r[kamacite.docs.ir_projection.source_envelopes]

- GIVEN a documentation page or runnable cell originates from human-authored Nickel, Steel, Rust, Preserves, Markdown, or other source bytes
- WHEN Kamacite collects documentation facts
- THEN the source bytes MUST remain represented by a separate source envelope, source hash, parser or collector identity, and non-claims rather than being overwritten by rendered output.

#### Scenario: Unsupported documentation schema version fails closed

r[kamacite.docs.ir_projection.unsupported_version]

- GIVEN a documentation IR artifact declares an unsupported schema version
- WHEN Kamacite validates the artifact
- THEN validation MUST fail closed with a deterministic unsupported-version diagnostic before trusting the documentation facts.

#### Scenario: Duplicate documentation identity fails closed

r[kamacite.docs.ir_projection.duplicate_id]

- GIVEN two documentation IR entries declare the same page, section, anchor, cell, or output identity within one validation scope
- WHEN Kamacite validates the documentation corpus
- THEN validation MUST fail closed with a deterministic duplicate-identity diagnostic.

#### Scenario: Missing source envelope fails closed

r[kamacite.docs.ir_projection.missing_source]

- GIVEN a documentation IR artifact references a required source envelope that is absent
- WHEN Kamacite validates the artifact
- THEN validation MUST fail closed with a deterministic missing-source diagnostic.

#### Scenario: Stale source hash fails closed

r[kamacite.docs.ir_projection.stale_source_hash]

- GIVEN a documentation IR artifact references source bytes by BLAKE3 identity
- WHEN the available source envelope bytes do not match the referenced identity
- THEN validation MUST fail closed with a deterministic stale-source diagnostic.

### Requirement: Documentation renderer projections

r[kamacite.docs.renderers] Kamacite MUST render documentation through deterministic renderer backends that consume documentation IR and emit HTML, Markdown, or AsciiDoc projections. Renderers MUST preserve declared anchors, links, source spans, diagnostics, runnable-cell result summaries, dependency identities, and non-claims. Renderer outputs MUST use explicit output identity domains and MUST NOT become canonical Kamacite IR.

#### Scenario: HTML renderer emits projection

r[kamacite.docs.renderers.html]

- GIVEN a valid documentation IR page and an accepted HTML renderer identity
- WHEN Kamacite renders documentation to HTML
- THEN the output MUST be recorded as an HTML projection that binds the documentation IR identity, renderer identity, diagnostics, output artifact hash, and non-claims.

#### Scenario: Markdown renderer emits projection

r[kamacite.docs.renderers.markdown]

- GIVEN a valid documentation IR page and an accepted Markdown renderer identity
- WHEN Kamacite renders documentation to Markdown
- THEN the output MUST be recorded as a Markdown projection that binds the documentation IR identity, renderer identity, diagnostics, output artifact hash, and non-claims.

#### Scenario: AsciiDoc renderer emits projection

r[kamacite.docs.renderers.asciidoc]

- GIVEN a valid documentation IR page and an accepted AsciiDoc renderer identity
- WHEN Kamacite renders documentation to AsciiDoc
- THEN the output MUST be recorded as an AsciiDoc projection that binds the documentation IR identity, renderer identity, diagnostics, output artifact hash, and non-claims.

#### Scenario: Projection is not canonical authority

r[kamacite.docs.renderers.projection_not_canonical]

- GIVEN a rendered documentation file is available
- WHEN a downstream consumer references it
- THEN the consumer MUST treat it as a derived projection and MUST NOT substitute its identity for the documentation IR, source-envelope, runnable-cell, or receipt identity.

#### Scenario: Stable anchors are deterministic

r[kamacite.docs.renderers.stable_anchors]

- GIVEN the same documentation IR and renderer identity are provided repeatedly
- WHEN anchors are assigned or rendered
- THEN anchor identifiers MUST remain stable regardless of filesystem iteration order, locale, unrelated environment variables, or output directory layout.

#### Scenario: Stable links resolve explicitly

r[kamacite.docs.renderers.stable_links]

- GIVEN a documentation IR artifact contains internal or cross-artifact links
- WHEN Kamacite plans rendered output
- THEN every supported link MUST resolve to an explicit target identity, and rendered links MUST be derived from that resolved target rather than from ambient paths.

#### Scenario: Unknown link fails closed

r[kamacite.docs.renderers.unknown_link]

- GIVEN a documentation IR artifact links to an unknown or ambiguous target
- WHEN Kamacite validates or renders the corpus
- THEN validation MUST fail closed with a deterministic unknown-link diagnostic before emitting a trusted projection.

#### Scenario: Unsupported output format fails closed

r[kamacite.docs.renderers.unsupported_format]

- GIVEN an operator requests a documentation target format outside the accepted renderer set
- WHEN Kamacite plans documentation output
- THEN planning MUST fail closed with a deterministic unsupported-format diagnostic.

#### Scenario: Renderer drift is detected

r[kamacite.docs.renderers.drift]

- GIVEN a golden documentation projection was produced by an older renderer identity or different documentation IR identity
- WHEN Kamacite compares the current render plan to the golden output
- THEN validation MUST report deterministic renderer-drift or output-mismatch diagnostics and MUST NOT silently trust the stale output.

### Requirement: Runnable documentation cells

r[kamacite.docs.runnable_cells] Kamacite MUST model runnable documentation examples as explicit checked cell artifacts. A runnable cell MUST declare its language or adapter class, source-envelope identity when source-backed, dependency identities, expected output or expected diagnostic, declared effects, run profile, evaluator or adapter identity, diagnostics, checked laws, result identity, and non-claims. Runnable-cell receipts MUST prove only the named cell result under the declared profile.

#### Scenario: Default execution is denied

r[kamacite.docs.runnable_cells.denied_by_default]

- GIVEN documentation generation encounters runnable cells
- WHEN the command mode does not explicitly permit cell execution
- THEN Kamacite MUST validate stored cell descriptors or receipts only and MUST NOT execute Nickel, Steel, Rust, Preserves, external tools, host functions, or adapter processes.

#### Scenario: Expected-success cell records result

r[kamacite.docs.runnable_cells.expected_success]

- GIVEN a runnable cell declares an expected successful output and all required capabilities are declared and admitted
- WHEN Kamacite runs the cell in an explicit execution mode
- THEN the cell receipt MUST bind the source identity, evaluator or adapter identity, declared effects, observed output identity, diagnostics, checked laws, and pass verdict.

#### Scenario: Expected-failure cell records diagnostic

r[kamacite.docs.runnable_cells.expected_failure]

- GIVEN a runnable cell declares an expected diagnostic or rejection
- WHEN Kamacite runs the cell in an explicit execution mode
- THEN the cell receipt MUST bind the source identity, evaluator or adapter identity, declared effects, observed diagnostic identity, checked laws, and expected-failure verdict without treating the failure as a command crash.

#### Scenario: Nickel runnable cell stays bounded

r[kamacite.docs.runnable_cells.nickel]

- GIVEN a runnable documentation cell uses Nickel evaluation or export
- WHEN Kamacite runs the cell
- THEN the run MUST use a declared Nickel evaluator identity and declared capabilities, and the receipt MUST classify the output as a checked Nickel-derived projection rather than source identity or arbitrary semantic-equivalence proof.

#### Scenario: Steel runnable cell stays bounded

r[kamacite.docs.runnable_cells.steel]

- GIVEN a runnable documentation cell uses Steel probe or predicate logic
- WHEN Kamacite runs the cell
- THEN the run MUST use a declared Steel engine identity, declared built-ins, declared host functions, and denied-by-default capabilities, and the receipt MUST NOT treat Steel bytecode, opaque closures, or JSON strings as canonical Kamacite IR.

#### Scenario: Rust-validator runnable cell stays bounded

r[kamacite.docs.runnable_cells.rust_validator]

- GIVEN a runnable documentation cell invokes Rust validation logic or examples
- WHEN Kamacite runs the cell
- THEN the run MUST be represented as Rust implementation evidence or validator output and MUST NOT claim compiler correctness, arbitrary program behavior, or semantic equivalence across languages.

#### Scenario: Preserves runnable cell stays bounded

r[kamacite.docs.runnable_cells.preserves]

- GIVEN a runnable documentation cell demonstrates Preserves canonicalization or decoding
- WHEN Kamacite runs the cell
- THEN the receipt MUST bind the canonical Preserves bytes and BLAKE3 value identity while preserving non-claims about source-language semantics.

#### Scenario: Unsupported cell language fails closed

r[kamacite.docs.runnable_cells.unsupported_language]

- GIVEN a runnable cell declares an unsupported language or adapter class
- WHEN Kamacite validates or runs the cell
- THEN validation MUST fail closed with a deterministic unsupported-language diagnostic.

#### Scenario: Undeclared capability fails closed

r[kamacite.docs.runnable_cells.undeclared_capability]

- GIVEN a runnable cell observes or requests filesystem, process, network, clock, random, credential, compiler, evaluator, host-function, or mutation access that its descriptor does not declare
- WHEN Kamacite validates or runs the cell
- THEN validation MUST fail closed with a deterministic undeclared-capability diagnostic.

#### Scenario: Missing evaluator identity fails closed

r[kamacite.docs.runnable_cells.missing_evaluator]

- GIVEN a runnable cell requires an evaluator, adapter, compiler, or validator identity
- WHEN the identity is absent
- THEN validation MUST fail closed with a deterministic missing-evaluator diagnostic.

#### Scenario: Mismatched expected output fails closed

r[kamacite.docs.runnable_cells.mismatched_output]

- GIVEN a runnable cell declares an expected output identity
- WHEN an explicit run produces a different output identity
- THEN validation MUST fail closed with a deterministic output-mismatch diagnostic and MUST NOT update golden evidence unless explicit update mode is selected.

#### Scenario: Unexpected success fails closed

r[kamacite.docs.runnable_cells.unexpected_success]

- GIVEN a runnable cell declares an expected diagnostic or rejection
- WHEN an explicit run succeeds instead
- THEN validation MUST fail closed with a deterministic unexpected-success diagnostic.

#### Scenario: Ambient authority request fails closed

r[kamacite.docs.runnable_cells.ambient_authority]

- GIVEN a runnable cell attempts to use ambient filesystem, process, network, clock, random, credential, compiler, evaluator, host-function, or mutation authority
- WHEN the capability is not explicitly declared by the cell and admitted by the command mode
- THEN validation MUST fail closed before trusting the cell receipt.

### Requirement: Documentation command boundary

r[kamacite.docs.command] Kamacite MUST provide explicit documentation command behavior for checking documentation IR, rendering projections, optionally running runnable cells, comparing golden outputs, and updating generated artifacts. Commands MUST consume explicit roots, manifests, or arguments; use pure core logic for planning and validation; remain non-mutating by default; and mutate only in explicit update modes that emit receipts.

#### Scenario: Command uses pure core planning

r[kamacite.docs.command.pure_core]

- GIVEN the CLI has read explicit documentation inputs into memory
- WHEN it validates documentation IR, plans render output, resolves links, checks runnable-cell descriptors, or constructs receipts
- THEN those decisions MUST be made by pure deterministic core functions without filesystem, process, network, clock, random, credential, evaluator, compiler, stdout, stderr, or mutation access.

#### Scenario: Default rendering does not execute cells

r[kamacite.docs.command.default_no_execution]

- GIVEN an operator runs documentation rendering without an explicit run mode
- WHEN the documentation corpus contains runnable cells
- THEN the command MUST render stored descriptor or receipt summaries only and MUST NOT execute those cells.

#### Scenario: Explicit run mode admits declared cells

r[kamacite.docs.command.explicit_run]

- GIVEN an operator selects an explicit runnable-cell execution mode
- WHEN a cell declares the required capabilities and evaluator or adapter identity
- THEN the shell MAY execute that cell and MUST pass the observed result back to pure receipt validation before trusting it.

#### Scenario: Command is non-mutating by default

r[kamacite.docs.command.non_mutating]

- GIVEN generated documentation, golden projections, or cell receipts are stale
- WHEN the documentation command runs without explicit update mode
- THEN it MUST report deterministic diagnostics and MUST NOT rewrite source docs, fixture manifests, generated projections, output directories, or golden receipts.

#### Scenario: Explicit update records changed output identities

r[kamacite.docs.command.explicit_update]

- GIVEN an operator intentionally refreshes rendered documentation projections or runnable-cell receipts
- WHEN the command writes updated artifacts
- THEN it MUST emit an update receipt listing changed documentation IDs, source identities, old output identities, new output identities, renderer or evaluator identities, checked laws, diagnostics, and non-claims.

### Requirement: Documentation receipts

r[kamacite.docs.receipts] Kamacite MUST emit deterministic documentation receipts for trusted rendering and runnable-cell claims. Receipts MUST bind source-envelope identities, documentation IR identity, dependency closure, renderer identity, target format, output artifact identities, runnable-cell receipts when present, diagnostics, checked laws, receipt schema version, receipt identity, and non-claims.

#### Scenario: Receipt binds output hashes

r[kamacite.docs.receipts.output_hashes]

- GIVEN Kamacite emits rendered documentation files or runnable-cell outputs
- WHEN it constructs a documentation receipt
- THEN the receipt MUST bind BLAKE3 output artifact hashes in explicit identity domains.

#### Scenario: Receipt preserves non-claims

r[kamacite.docs.receipts.non_claims]

- GIVEN a documentation IR artifact, renderer output, or runnable-cell result includes non-claims about semantic equivalence, evaluator soundness, compiler correctness, runtime authorization, source identity, or canonical IR authority
- WHEN Kamacite emits or exports a documentation receipt
- THEN every non-claim MUST remain present and MUST NOT be contradicted by a stronger generated claim.

#### Scenario: Malformed receipt fails closed

r[kamacite.docs.receipts.malformed]

- GIVEN a documentation receipt is malformed, missing required identities, missing non-claims, or uses an unsupported schema version
- WHEN Kamacite validates the receipt
- THEN validation MUST fail closed with deterministic diagnostics before trusting rendered output or runnable-cell evidence.

#### Scenario: Non-claim violation fails closed

r[kamacite.docs.receipts.non_claim_violation]

- GIVEN a documentation receipt claims rendered output is canonical source identity, claims arbitrary source-language semantic equivalence, or drops required non-claims
- WHEN Kamacite validates the receipt
- THEN validation MUST fail closed with a deterministic non-claim violation diagnostic.

### Requirement: Documentation fixture admission

r[kamacite.docs.fixture_admission] Kamacite MUST require positive and negative documentation fixtures before accepting documentation IR, renderer, runnable-cell, receipt, or command behavior as implemented. Fixtures MUST include deterministic golden receipts and MUST be discoverable through explicit manifests or arguments.

#### Scenario: Positive documentation IR fixture is admitted

r[kamacite.docs.fixture_admission.positive_ir]

- GIVEN a fixture contains a valid documentation IR artifact with source envelopes, anchors, links, code blocks, and non-claims
- WHEN Kamacite validates the fixture
- THEN validation MUST accept it and emit a deterministic fixture receipt.

#### Scenario: Positive renderer fixtures are admitted

r[kamacite.docs.fixture_admission.positive_renderers]

- GIVEN fixtures contain valid HTML, Markdown, and AsciiDoc golden projections for the same documentation IR artifact
- WHEN Kamacite validates renderer output
- THEN validation MUST accept each projection only when its receipt identity matches the current documentation IR, renderer identity, target format, output hash, diagnostics, and non-claims.

#### Scenario: Negative fixtures reject invalid documentation behavior

r[kamacite.docs.fixture_admission.negative]

- GIVEN fixtures cover stale hashes, duplicate IDs, missing source envelopes, unknown links, unsupported output formats, undeclared capabilities, unsupported cell languages, mismatched runnable-cell outputs, malformed receipts, and non-claim violations
- WHEN Kamacite validates the fixture corpus
- THEN every invalid fixture MUST fail with the expected deterministic diagnostic and MUST NOT produce trusted documentation evidence.

### Requirement: Documentation implementation admission

r[kamacite.docs.implementation_admission] Kamacite MUST NOT claim documentation generation, runnable documentation, or rendered publication support is implemented until schema definitions, pure validators, renderer backends, command shells, positive fixtures, negative fixtures, golden receipts, and documentation guidance are present and validated.

#### Scenario: Claims wait for fixtures and receipts

r[kamacite.docs.implementation_admission.before_claims]

- GIVEN schema or renderer code exists without complete positive and negative fixtures or receipts
- WHEN project documentation describes the feature status
- THEN it MUST classify the feature as planned or experimental and MUST NOT claim trusted documentation generation support.

### Requirement: Documentation stack roles

r[kamacite.docs.stack_roles] Kamacite MUST preserve stack boundaries for documentation artifacts. Valence MAY wrap documentation receipts as evidence; Octet MAY check Rust source-shape and provenance for renderers or validators; Mantle MAY package rendered docs and receipts; Cairn MAY use documentation evidence in lifecycle gates; Trellis MAY prove narrow pure-core properties if introduced later. None of those consumers owns Kamacite documentation IR or broadens documentation claims.

#### Scenario: Valence wraps documentation evidence without broadening claims

r[kamacite.docs.stack_roles.valence]

- GIVEN a documentation or runnable-cell receipt has stable identity, roles, BLAKE3 hashes, source spans, diagnostics, and non-claims
- WHEN Kamacite exports it for Valence consumption
- THEN the export MUST preserve the source Kamacite receipt identity and MUST NOT broaden rendered-output or runnable-cell claims.

#### Scenario: Octet evidence stays implementation-scoped

r[kamacite.docs.stack_roles.octet]

- GIVEN Octet checks Rust renderer, validator, or command-shell code
- WHEN the result is referenced by Kamacite documentation evidence
- THEN the result MUST be classified as Rust source-shape or provenance evidence and MUST NOT be promoted to documentation IR correctness or arbitrary language semantic-equivalence proof.

#### Scenario: Mantle packages docs as release evidence

r[kamacite.docs.stack_roles.mantle]

- GIVEN Mantle packages rendered documentation and documentation receipts
- WHEN release evidence references those artifacts
- THEN the release evidence MUST bind the Kamacite documentation receipt identities and MUST NOT substitute package paths or release bundle identities for source documentation identity.

#### Scenario: Cairn gates lifecycle evidence only

r[kamacite.docs.stack_roles.cairn]

- GIVEN Cairn evaluates lifecycle readiness using documentation evidence
- WHEN documentation receipts are missing, stale, or invalid
- THEN Cairn MAY block lifecycle readiness, but a passing lifecycle gate MUST NOT be represented as proof of source-language semantics or renderer correctness beyond the accepted receipt claims.
