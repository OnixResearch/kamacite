## Tasks

- [x] [serial] r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.profile] Define the dialect-roundtrip reference profile and Preserves-backed DTOs.
- [x] [depends:dialect-roundtrip-profile] r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.canonical_preserves] Validate canonical Preserves bytes, payload hashes, source envelope refs, source spans, lowering receipt refs, and non-claims.
- [x] [depends:dialect-roundtrip-validation] r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.opaque_unknown] Preserve unknown dialect payload identity without interpreting semantics.
- [x] [depends:dialect-roundtrip-validation] r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.positive] Add positive fixtures for known and unknown dialect payload roundtrips.
- [x] [depends:dialect-roundtrip-validation] r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.negative] Add negative fixtures for stale hash, unsupported version, missing envelope, lossy JSON projection, cross-dialect substitution, and overclaim.
- [x] [depends:dialect-roundtrip-fixtures] r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.valence_export] Add Valence export mapping fixtures after local profile validation.
- [x] [depends:dialect-roundtrip-valence] r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.validation] Run Kamacite validation, fixture tests, lifecycle validation, and focused Valence export checks.