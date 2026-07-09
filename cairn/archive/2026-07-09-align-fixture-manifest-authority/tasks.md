## Phase 1: Lifecycle and source authority

- [x] [serial] Create the native Cairn change package for fixture manifest authority. r[kamacite.validation_harness.fixture_manifest.full_corpus_authority]
- [x] [serial] Materialize the full fixture corpus in `fixtures/manifest.ncl` with source-owned projection identity evidence. r[kamacite.validation_harness.fixture_manifest.full_corpus_authority] r[kamacite.validation_harness.fixture_manifest.projection_identity]

## Phase 2: Projection freshness checks

- [x] [serial] Add CLI shell projection freshness validation before fixture results are trusted. r[kamacite.validation_harness.fixture_manifest.projection_freshness]
- [x] [parallel] Add positive and negative tests for matching projection evidence, stale projection hashes, and fixture-ID drift. r[kamacite.validation_harness.fixture_manifest.projection_freshness] r[kamacite.validation_harness.fixture_manifest.fixture_id_alignment]
- [x] [parallel] Update validation docs to name the source/projection freshness gate and its non-claims. r[kamacite.validation_harness.fixture_manifest.projection_identity]

## Phase 3: Verification and archive

- [x] [serial] Run Rust tests, harness validation, Cairn validation, and change gates. r[kamacite.validation_harness.fixture_manifest.projection_freshness]
- [x] [serial] Sync accepted specs and archive the completed change. r[kamacite.validation_harness.fixture_manifest.full_corpus_authority]
