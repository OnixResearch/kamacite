# Adapter Conformance Specification

## Purpose

Define Kamacite's ast-grep structural audit boundary for adapter conformance and source-envelope review.

## Requirements

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
