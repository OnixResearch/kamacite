## Phase 1: Canonical conformance artifact

- [x] [serial] Define a canonical conformance body and deterministic set/sequence normalization rules excluding only the recursive identity field. r[kamacite.findings.valence_conformance.content_identity]
- [x] [serial] Compute and validate adapter-domain conformance identity from canonical body bytes. r[kamacite.findings.valence_conformance.content_identity]
- [x] [serial] Define JSON as a bound compatibility projection carrying canonical codec/schema and identity. r[kamacite.findings.valence_conformance.bound_projection]

## Phase 2: Export integration

- [x] [serial] Change the CLI/core boundary to consume a validated canonical conformance artifact rather than a self-asserted JSON identity. r[kamacite.findings.valence_conformance.cli_input]
- [x] [serial] Recompute conformance identity before carrying laws, domains, coverage, dependencies, or non-claims into an export. r[kamacite.findings.valence_conformance.bound_projection]
- [x] [serial] Migrate conformance/export goldens and emit old/new identity evidence. r[kamacite.findings.valence_conformance.migration]

## Phase 3: Regression evidence

- [x] [serial] Add field-mutation, stale-identity, malformed-entry, duplicate-set, reordered-set, wrong-domain, and self-asserted-JSON tests. r[kamacite.findings.valence_conformance.migration] r[kamacite.findings.valence_conformance.cli_input]
- [x] [serial] Run focused export tests, package export validation, workspace tests, Clippy, Cairn gates, and Nix checks. r[kamacite.findings.valence_conformance.content_identity] r[kamacite.findings.valence_conformance.bound_projection]
