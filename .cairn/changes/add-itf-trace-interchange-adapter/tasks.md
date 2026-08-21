## Tasks

- [ ] [serial] Define the versioned ITF-to-Preserves adapter descriptor, source-envelope records, and result classes. r[kamacite.itf_interchange.adapter]
- [ ] [serial] Define independent BLAKE3 identities for ITF version, parser/tool, source bytes, converted Preserves value, adapter descriptor, schemas, and receipt. r[kamacite.itf_interchange.identity]
- [ ] [serial] Implement pure ITF value mapping, schema validation, version admission, and fail-closed denials for malformed, stale, and tampered traces. r[kamacite.itf_interchange.validation]
- [ ] [parallel] Implement adapter receipt construction and validation binding all layer identities and non-claims. r[kamacite.itf_interchange.receipt]
- [ ] [parallel] Add positive fixtures for valid conversions and negative fixtures for malformed, wrong-version, schema-mismatch, tamper, and equivalence-overclaim cases. r[kamacite.itf_interchange.fixtures]
- [ ] [serial] Run focused core tests, manifest freshness, positive and negative adapter validation, lifecycle validation, Cairn gates, and relevant flake checks. r[kamacite.itf_interchange.validation_rail]
