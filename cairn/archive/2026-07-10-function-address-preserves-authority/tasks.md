## Tasks

- [x] [serial] Define the versioned Preserves schema for canonical function-address receipts. r[kamacite.function_address_receipts.preserves_authority.schema]
- [x] [serial] Compute function-address receipt identity from BLAKE3 over canonical Preserves binary bytes. r[kamacite.function_address_receipts.preserves_authority.hash]
- [x] [serial] Generate JSON compatibility projections bound to the canonical Preserves receipt hash. r[kamacite.function_address_receipts.preserves_authority.json_projection]
- [x] [serial] Add positive fixtures for Octet handoff, canonical hash stability, text/binary round trip, and projection binding. r[kamacite.function_address_receipts.preserves_authority.positive]
- [x] [serial] Add negative fixtures for malformed Preserves, schema mismatch, projection drift, non-canonical hash input, missing non-claims, and overclaims. r[kamacite.function_address_receipts.preserves_authority.negative]
- [x] [serial] Run Kamacite tests, Preserves schema checks, and the Preserves-backed stack smoke. r[kamacite.function_address_receipts.preserves_authority.validation]
