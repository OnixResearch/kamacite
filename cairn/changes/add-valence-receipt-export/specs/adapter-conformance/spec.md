# Adapter Conformance Specification

## Purpose

Defines the conformance matrix for Kamacite adapters and target/provenance producers across Nickel, Steel, WebAssembly, LLVM, Haskell/GHC, Rust MIR, Valence exports, and Octet checks.

## Requirements

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
