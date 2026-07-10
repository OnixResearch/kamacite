## Tasks

- [ ] [serial] Define the canonical Preserves schema for `EvidenceChainEnvelope`. r[kamacite.stack_integration.canonical_evidence_chain_envelope.schema]
- [ ] [serial] Compute envelope identity as BLAKE3 over canonical Preserves binary bytes. r[kamacite.stack_integration.canonical_evidence_chain_envelope.hash]
- [ ] [serial] Add typed parent-link domains, evidence kinds, policy hashes, projection records, and non-claim boundary hashes. r[kamacite.stack_integration.canonical_evidence_chain_envelope.links]
- [ ] [serial] Add deterministic Merkle-style roots for profile payload collections. r[kamacite.stack_integration.canonical_evidence_chain_envelope.roots]
- [ ] [serial] Make function-address the first profile carried by the generic envelope. r[kamacite.stack_integration.canonical_evidence_chain_envelope.function_address]
- [ ] [serial] Add positive fixtures for hash stability, projection binding, typed links, and collection roots. r[kamacite.stack_integration.canonical_evidence_chain_envelope.positive]
- [ ] [serial] Add negative fixtures for unknown kind, wrong link domain, projection drift, malformed digest, missing policy hash, non-canonical bytes, and weakened non-claims. r[kamacite.stack_integration.canonical_evidence_chain_envelope.negative]
- [ ] [serial] Run Kamacite tests, Preserves schema checks, and stack smoke. r[kamacite.stack_integration.canonical_evidence_chain_envelope.validation]
