# Stack Integration Specification

## Purpose

Make the Valence export conformance identity identify the complete reviewed conformance entry.

## Requirements

### Requirement: Conformance identity covers normative content

r[kamacite.findings.valence_conformance.content_identity] Kamacite MUST compute the adapter-domain conformance identity from canonical bytes covering profile version, source/output schemas, identity-domain policy, checked laws, dependency profile, fixture coverage, and non-claims. The identity field itself MUST be excluded from recursive hash material.

#### Scenario: Checked law changes

- GIVEN a reviewed conformance entry adds, removes, or changes a checked law
- WHEN Kamacite canonicalizes the entry
- THEN the conformance identity MUST change and the old identity MUST be rejected as stale.

### Requirement: JSON conformance is a bound projection

r[kamacite.findings.valence_conformance.bound_projection] A JSON conformance projection MUST identify its canonical conformance codec and identity, and validation MUST recompute or resolve that identity before export. Domain validation alone MUST NOT establish conformance trust.

#### Scenario: JSON retains an old identity

- GIVEN a JSON conformance projection changes a normative field while retaining its previous adapter-domain identity
- WHEN Valence export validation runs
- THEN export MUST fail before carrying any conformance laws into derived evidence.

### Requirement: CLI consumes the reviewed artifact

r[kamacite.findings.valence_conformance.cli_input] The Valence export command MUST consume canonical conformance bytes or a previously validated canonical conformance artifact, with any JSON input treated only as a compatibility projection.

#### Scenario: Self-asserted JSON is the only input

- GIVEN a caller supplies a structurally valid JSON object with an arbitrary adapter-domain hash but no bound canonical conformance artifact
- WHEN export is requested
- THEN the command MUST reject it as untrusted conformance input.

### Requirement: Conformance migration is explicit

r[kamacite.findings.valence_conformance.migration] Conformance identity changes MUST update export goldens and emit old/new identity evidence, and tests MUST cover field mutation, set reordering, duplicates, malformed entries, and wrong identity domains.

#### Scenario: Set-like fields reorder only

- GIVEN two conformance entries differ only in the presentation order of fields declared set-like
- WHEN canonical identities are computed
- THEN the identities MUST remain equal after sorted-unique canonicalization.
