## Tasks

- [x] [serial] Define the Trellis proof evidence profile schema for canonical evidence-chain envelopes. r[kamacite.stack_integration.trellis_proof_evidence_envelope.schema]
- [x] [serial] Bind source envelope, proof IR, verifier receipt, assumptions, dependencies, property IDs, and non-claim roots. r[kamacite.stack_integration.trellis_proof_evidence_envelope.links]
- [x] [serial] Preserve recorded-only and formal-proof candidate roles without deciding downstream promotion. r[kamacite.stack_integration.trellis_proof_evidence_envelope.roles]
- [x] [serial] Generate JSON projections bound to canonical Preserves envelope identity. r[kamacite.stack_integration.trellis_proof_evidence_envelope.projection]
- [x] [serial] Add positive fixtures for verified profile, reference profile, hash stability, and projection binding. r[kamacite.stack_integration.trellis_proof_evidence_envelope.positive]
- [x] [serial] Add negative fixtures for missing source, missing verifier receipt, malformed property IDs, assumption drift, wrong role domain, projection drift, and overclaims. r[kamacite.stack_integration.trellis_proof_evidence_envelope.negative]
- [x] [serial] Run Kamacite tests, Preserves schema checks, and Trellis proof evidence stack smoke. r[kamacite.stack_integration.trellis_proof_evidence_envelope.validation]
