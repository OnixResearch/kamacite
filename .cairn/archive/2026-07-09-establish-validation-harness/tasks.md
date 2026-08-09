## Phase 1: Cairn decision package

- [x] [serial] Define the walking-skeleton validation harness as the pre-implementation admission gate for source identity, stale hash rejection, wrong identity-domain rejection, and golden receipt comparison. r[kamacite.validation_harness.walking_skeleton]
- [x] [serial] Specify the deterministic fixture layout, pure core versus adapter/probe separation, and deferred adapter-suite boundary. r[kamacite.validation_harness.fixture_layout] r[kamacite.validation_harness.adapter_suite_deferral]
- [x] [serial] Specify the Nickel fixture manifest contract and generated-projection authority boundary. r[kamacite.validation_harness.fixture_manifest]
- [x] [serial] Specify the pure validation core and effectful harness runner boundary. r[kamacite.validation_harness.core_runner_boundary]
- [x] [serial] Specify the seed fixture set and non-mutating golden receipt update boundary. r[kamacite.validation_harness.seed_fixtures] r[kamacite.validation_harness.receipt_update_boundary]
- [x] [serial] Validate the Cairn package with repo validation plus proposal, design, and tasks gates. r[kamacite.validation_harness.walking_skeleton]

## Phase 2: Walking-skeleton implementation

- [x] [serial] Add the minimal Rust workspace and trusted core/runner module boundary without granting filesystem, process, evaluator, network, clock, random, credential, or mutation authority to the core. r[kamacite.validation_harness.core_runner_boundary] r[kamacite.validation_harness.fixture_layout]
- [x] [serial] Add the fixture directory layout that separates pure core fixtures, adapter/probe fixtures, and golden receipts with deterministic discovery rules. r[kamacite.validation_harness.fixture_layout] r[kamacite.validation_harness.adapter_suite_deferral]
- [x] [serial] Add the typed Nickel fixture manifest contract plus the first human-authored manifest entries for seed fixtures. r[kamacite.validation_harness.fixture_manifest]
- [x] [parallel] Add the positive valid source-envelope seed fixture and expected deterministic receipt data. r[kamacite.validation_harness.seed_fixtures] r[kamacite.validation_harness.seed_fixtures.valid_source_envelope]
- [x] [parallel] Add the negative stale-source-hash seed fixture with expected deterministic stale-identity diagnostic. r[kamacite.validation_harness.seed_fixtures] r[kamacite.validation_harness.seed_fixtures.stale_source_hash]
- [x] [parallel] Add the negative wrong-identity-domain seed fixture with expected deterministic identity-domain diagnostic. r[kamacite.validation_harness.seed_fixtures] r[kamacite.validation_harness.seed_fixtures.wrong_identity_domain]
- [x] [parallel] Add the checked-in golden receipt fixture and default no-rewrite mismatch check. r[kamacite.validation_harness.seed_fixtures.golden_receipt] r[kamacite.validation_harness.receipt_update_boundary]
- [x] [serial] Add the first harness command or cargo test target that loads the manifest projection, invokes the pure core over normalized DTOs, compares golden receipts, and emits deterministic diagnostics. r[kamacite.validation_harness.walking_skeleton] r[kamacite.validation_harness.core_runner_boundary] r[kamacite.validation_harness.receipt_update_boundary]
- [x] [serial] Document that Nickel, Steel, WASM, LLVM, Valence, Octet, and algebraic-effect execution suites remain deferred until their implementation slices add dedicated positive, negative, capability, diagnostic, and golden fixtures. r[kamacite.validation_harness.adapter_suite_deferral]
