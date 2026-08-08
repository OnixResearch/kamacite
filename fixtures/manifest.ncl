# Human-authored Kamacite fixture manifest authority.
# Validated against fixtures/manifest-contract.ncl before projection trust.

let nickel_sandbox_profile = {
  schema_version = "kamacite.nickel-sandbox.v1",
  sandbox_kind = "bubblewrap-linux-v1",
  evaluator_profile = "nickel-reference-cli",
  environment = { HOME = "/homeless", LANG = "C", LC_ALL = "C", TZ = "UTC" },
  read_only_source_path = "/work/source.ncl",
  read_only_import_root = "/work",
  network_allowed = false,
  process_context_visible = false,
  writable_paths = ["/tmp"],
  admitted_capabilities = ["capture-stdout", "fixture-read", "nickel-reference-export"],
  limits = {
    execution_timeout_milliseconds = 5000,
    cpu_seconds = 5,
    memory_limit_bytes = 1073741824,
    output_limit_bytes = 1048576,
    diagnostic_limit_bytes = 65536,
  },
  profile_identity = {
    algorithm = "blake3",
    domain = "adapter",
    hex = "1b97ef2db104877c185fbf849bde9999a98c0b66f240a5f24d9331d83bf2c706",
  },
} in
let nickel_sandbox_evidence = fun source_hex outcome_value exit_code_value stdout_identity_value stderr_hex => {
  profile_identity = nickel_sandbox_profile.profile_identity,
  evaluator_identity = {
    algorithm = "blake3",
    domain = "evaluator",
    hex = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524",
  },
  import_identities = {
    "source.ncl" = { algorithm = "blake3", domain = "source", hex = source_hex },
  },
  requested_capabilities = ["fixture-read", "nickel-reference-export", "capture-stdout"],
  admitted_capabilities = nickel_sandbox_profile.admitted_capabilities,
  expected_effects = ["fixture-read", "nickel-reference-export", "capture-stdout"],
  observed_execution_facts = [
    "environment-sanitized",
    "network-isolated",
    "bounded-output",
    "read-only-import-closure",
    "sandbox-enforced",
  ],
  outcome = outcome_value,
  exit_code = exit_code_value,
  stdout_identity = stdout_identity_value,
  stderr_identity = { algorithm = "blake3", domain = "projection", hex = stderr_hex },
  sandbox_enforced = true,
} in
let ast_grep_languages = [
  {
    language = "Rust",
    parser_kind = "built-in",
    parser_identity = { domain = "evaluator", algorithm = "blake3", hex = "6fe5e10a782fc7ce7a96991458c9c1db3787e8f11c142ecc4ded94bda4401ae2" },
    extensions = ["rs"],
  },
  {
    language = "Nix",
    parser_kind = "built-in",
    parser_identity = { domain = "evaluator", algorithm = "blake3", hex = "8799a1f54aa491df9cc34c384c65abdba9926e9a3eca2008e7d2c6fd3836c7c7" },
    extensions = ["nix"],
  },
  {
    language = "Yaml",
    parser_kind = "built-in",
    parser_identity = { domain = "evaluator", algorithm = "blake3", hex = "f875d0fe110bddc1c1a53512d81fe1235989c302e84f3ea6c2fdcb321d0923fe" },
    extensions = ["yaml", "yml"],
  },
  {
    language = "Json",
    parser_kind = "built-in",
    parser_identity = { domain = "evaluator", algorithm = "blake3", hex = "4ad43ce58b36a9ea053552e9231bc290ec34e4f3932f7ad893365866b165924b" },
    extensions = ["json"],
  },
] in
let ast_grep_rule = fun negative_count negative_identity_value gate_value => {
  rule_id = "core-ambient-process",
  language = "Rust",
  rule_identity = { domain = "source", algorithm = "blake3", hex = "0ece21345e4c362c56ab265c2839f2d8dc13b1d6ac8152bb481f1bc65b582717" },
  positive_fixture_identity = { domain = "source", algorithm = "blake3", hex = "e36a898551e0be83b4b9cb325e42c47c919953c15f722521e10209e0f41f70eb" },
  negative_fixture_identity = negative_identity_value,
  positive_cases = 2,
  negative_cases = negative_count,
  gate_eligible = gate_value,
} in
let ast_grep_sources = [
  { path = "audits/ast-grep/rules/core-ambient-process.yml", identity = { domain = "source", algorithm = "blake3", hex = "0ece21345e4c362c56ab265c2839f2d8dc13b1d6ac8152bb481f1bc65b582717" } },
  { path = "audits/ast-grep/fixtures/positive/core-ambient-process.rs", identity = { domain = "source", algorithm = "blake3", hex = "e36a898551e0be83b4b9cb325e42c47c919953c15f722521e10209e0f41f70eb" } },
  { path = "audits/ast-grep/fixtures/negative/core-pure.rs", identity = { domain = "source", algorithm = "blake3", hex = "e208cae51ae00173bc8882f3865a5914002a44cd16e1094bd4ee84c93c797dc6" } },
] in
let ast_grep_findings = [
  {
    rule_id = "core-ambient-process",
    source_identity = { domain = "source", algorithm = "blake3", hex = "e36a898551e0be83b4b9cb325e42c47c919953c15f722521e10209e0f41f70eb" },
    start_byte = 41,
    end_byte = 76,
    message = "Pure Kamacite core must not construct ambient processes.",
  },
] in
let ast_grep_codemod = {
  recipe_path = "audits/ast-grep/codemods/core-ambient-process.json",
  recipe_identity = { domain = "source", algorithm = "blake3", hex = "5ac74155860d20ea9c7e5deaeb5e92f1ed89ffeb46fe5b4aaf5a43742de51606" },
  before_source_path = "audits/ast-grep/fixtures/positive/core-ambient-process.rs",
  before_source_identity = { domain = "source", algorithm = "blake3", hex = "e36a898551e0be83b4b9cb325e42c47c919953c15f722521e10209e0f41f70eb" },
  after_source_path = "audits/ast-grep/fixtures/codemod/core-process-after.rs",
  after_source_identity = { domain = "source", algorithm = "blake3", hex = "13fe64e6a9461dc00c330590668730dd1a7876912f2e55c31f034f7bc929c76a" },
  dry_run = true,
  post_check_commands = [
    "ast-grep test --config audits/ast-grep/sgconfig.yml",
    "cargo test --workspace",
  ],
  non_claims = ["the rewrite proposal does not prove semantic equivalence"],
} in
let ahb_required_non_claims = [
  "not host capability or authorization",
  "not sandbox admission",
  "not resource enforcement",
  "not runtime correctness",
  "not application correctness",
  "not release eligibility",
] in
let ahb_empty_metadata = {
  display_name = null,
  aliases = [],
  path = null,
  source_span = null,
  documentation = null,
} in
let ahb_identity = fun domain_value hex_value => {
  algorithm = "blake3",
  domain = domain_value,
  hex = hex_value,
} in
let ahb_base_payload = {
  kind = "application-host-binding",
  schema_version = "kamacite.application-host-binding.v1",
  application_identity = ahb_identity "target-artifact" "1fadd6a5c4841b7e5a68b83ff1faef189d785f775d87ca301e82b2058ae0d228",
  entrypoint_identity = ahb_identity "target-artifact" "1d3ef4c1636eaa8e447a8f44e6837c25f4433bd924929ba4dd9282d03d69dd01",
  input_schema_identity = ahb_identity "adapter" "497027c98ba56782ddeb1f334082f5db2f52ccb993689db57c2d444fed5e1396",
  output_schema_identity = ahb_identity "adapter" "16589b99411e76a180fd747b73089203f4d6f549192bac6f7e6213e7bcb3e785",
  effect_summary_identity = ahb_identity "algebraic-effect" "a82fe292271b6fd99413231c2dc5718f7391cfe6e753b93426cc8fe9f07b3d6e",
  effect_summary_closed = true,
  effect_operation_identities = [
    ahb_identity "effect-operation" "a3007e263daab76335bf515e1b88b237335097a8de7104ccfa4ae38aa0d49063",
    ahb_identity "effect-operation" "7bde87c5223afc4641c618a9b1c64df17c83a43f4e3df7cb06e0673feb7b1e45",
  ],
  host_profile_identity = ahb_identity "evaluator" "ccbe18b932f33d3c86ca55104958ec5855d0408371849b3960e344109e9bf989",
  alternative_host_profile_identities = [],
  fallback_host_profile_identity = null,
  adapter_cohort_identity = ahb_identity "adapter" "06afa4477b70db84ea9ae8a4e90c954ed687b52f5ba0dc3c3751bd1586536210",
  resource_profile_identity = ahb_identity "adapter" "bb662261c01b9a95dbede5db4c93713d61d2de330cf0529a73cbc8ffbef3faf4",
  policy_identity = null,
  compatibility_identity = null,
  expected_adapter_cohort_identity = null,
  expected_resource_profile_identity = null,
  expected_host_profile_identity = null,
  metadata = ahb_empty_metadata,
  semantic_claims = [],
  non_claims = ahb_required_non_claims,
} in
let animus_application_host_binding = {
  kind = "application-host-binding",
  schema_version = "kamacite.application-host-binding.v1",
  application_identity = ahb_identity "target-artifact" "5fd77e6bf7e3a618faf48a8a16c398126797a5245011f0d06dd40f45513b9cb1",
  entrypoint_identity = ahb_identity "target-artifact" "bb661c376f2a83ceedd730d404de471dfcc5abd6f75f9f4a42a95b8afd9a0d42",
  input_schema_identity = ahb_identity "adapter" "33df3bb18d797392d6421016a76ca9970321dc4d62dda749815a06547419db49",
  output_schema_identity = ahb_identity "adapter" "f3f6e96b49daecc77fd98bd5434fe34ab23f39d226b506bdfd551ed773b2e81f",
  effect_summary_identity = ahb_identity "algebraic-effect" "64471a04261cbb5f5d9e9b3df5ba9f2454145535355601760e779d2f6b0a46ac",
  effect_summary_closed = true,
  effect_operation_identities = [
    ahb_identity "effect-operation" "7fa2709c205ccfa8c4b8773ab97396193a5130a2e42b2f94916802b04e07f5f8",
    ahb_identity "effect-operation" "c598d03de53f54b01e8bc368840e6394e5cc3334b4e1be118f563ddfb8cfba2a",
  ],
  host_profile_identity = ahb_identity "evaluator" "53a1955677292245ccd4ccc6c0c3a7f3bcd9431f4ac0d6f1a7f9fb7ca1ac5070",
  alternative_host_profile_identities = [],
  fallback_host_profile_identity = null,
  adapter_cohort_identity = ahb_identity "adapter" "3333333333333333333333333333333333333333333333333333333333333333",
  resource_profile_identity = ahb_identity "adapter" "876e8e3aedf280155e40378e5ba1add80ca30a7a0931f9a9c972b898694619f2",
  policy_identity = ahb_identity "adapter" "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  compatibility_identity = ahb_identity "evidence-chain" "0ec64025422e00a28a2bfdb45f43b8fcd590ec1a4d3325bc07dae71d7ea0dbcb",
  expected_adapter_cohort_identity = ahb_identity "adapter" "3333333333333333333333333333333333333333333333333333333333333333",
  expected_resource_profile_identity = ahb_identity "adapter" "876e8e3aedf280155e40378e5ba1add80ca30a7a0931f9a9c972b898694619f2",
  expected_host_profile_identity = ahb_identity "evaluator" "53a1955677292245ccd4ccc6c0c3a7f3bcd9431f4ac0d6f1a7f9fb7ca1ac5070",
  metadata = ahb_empty_metadata,
  semantic_claims = [],
  non_claims = [
    "not effect completeness",
    "not application correctness",
    "not host capability or authorization",
    "not host behavior",
    "not sandbox admission",
    "not provider correctness",
    "not tool safety",
    "not runtime success",
    "not runtime correctness",
    "not resource enforcement",
    "not release eligibility",
  ],
} in
{
  "authority_non_claims" = [
    "manifest source is review authority",
    "json projection is runtime compatibility input only",
    "generation receipt must bind source, contract, evaluator, and exact projection bytes",
    "manifest generation does not prove Nickel evaluator or source semantic correctness",
  ],
  "fixtures" = [
    {
      "artifact_class" = "source-envelope",
      "checked_laws" = [
        "source-envelope-preservation",
        "names-as-metadata",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json",
      "id" = "source-envelope.valid.nickel-minimal",
      "non_claims" = [
        "not Nickel evaluation semantics",
        "not arbitrary source translation",
      ],
      "payload" = {
        "aliases" = [
          "seed-valid",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "kind" = "source-envelope",
        "language" = "nickel",
        "media_type" = "text/x-nickel",
        "parser_identity" = "nickel-reference-deferred",
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.validation_harness.seed_fixtures.valid_source_envelope]",
        "r[kamacite.lossless_ir.positive_fixtures.source_bytes]",
        "r[kamacite.rust_determinism.first_slice.source_envelope]",
      ],
    },
    {
      "artifact_class" = "source-envelope",
      "checked_laws" = [
        "fail-closed-stale-identity",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/source-envelope.negative.stale-source-hash.receipt.json",
      "id" = "source-envelope.negative.stale-source-hash",
      "non_claims" = [
        "not trusted source identity",
      ],
      "payload" = {
        "aliases" = [
          "seed-stale",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "kind" = "source-envelope",
        "language" = "nickel",
        "media_type" = "text/x-nickel",
        "parser_identity" = "nickel-reference-deferred",
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/stale.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.validation_harness.seed_fixtures.stale_source_hash]",
        "r[kamacite.rust_determinism.total_diagnostics.stale_hash]",
      ],
    },
    {
      "artifact_class" = "source-envelope",
      "checked_laws" = [
        "typed-identity-domain",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/source-envelope.negative.wrong-identity-domain.receipt.json",
      "id" = "source-envelope.negative.wrong-identity-domain",
      "non_claims" = [
        "not cross-domain coercion",
      ],
      "payload" = {
        "aliases" = [
          "seed-wrong-domain",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "afe694dfcd747b297c6638d11318a9a5063015cb50123a9bae763d160e2ad2b5",
        },
        "kind" = "source-envelope",
        "language" = "nickel",
        "media_type" = "text/x-nickel",
        "parser_identity" = "nickel-reference-deferred",
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/wrong-domain.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.validation_harness.seed_fixtures.wrong_identity_domain]",
        "r[kamacite.rust_determinism.typed_identity_domains.constructor_validation]",
      ],
    },
    {
      "artifact_class" = "source-envelope",
      "checked_laws" = [
        "fail-closed-version",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/source-envelope.negative.unsupported-version.receipt.json",
      "id" = "source-envelope.negative.unsupported-version",
      "non_claims" = [
        "not compatibility defaulting",
      ],
      "payload" = {
        "aliases" = [
          "seed-unsupported-version",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "kind" = "source-envelope",
        "language" = "nickel",
        "media_type" = "text/x-nickel",
        "parser_identity" = "nickel-reference-deferred",
        "schema_version" = 2,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.schema_vocabulary.unsupported_version]",
      ],
    },
    {
      "artifact_class" = "source-envelope",
      "checked_laws" = [
        "source-bytes-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/source-envelope.negative.missing-source-bytes.receipt.json",
      "id" = "source-envelope.negative.missing-source-bytes",
      "non_claims" = [
        "not source preservation",
      ],
      "payload" = {
        "aliases" = [
          "seed-missing-source",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "kind" = "source-envelope",
        "language" = "nickel",
        "media_type" = "text/x-nickel",
        "parser_identity" = "nickel-reference-deferred",
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/missing-source.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.source_envelope_vocabulary.missing_bytes]",
      ],
    },
    {
      "artifact_class" = "canonical-value",
      "checked_laws" = [
        "preserves-bytes-authoritative",
        "projection-derived",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/canonical-value.positive.preserves-minimal.receipt.json",
      "id" = "canonical-value.positive.preserves-minimal",
      "non_claims" = [
        "JSON projection is compatibility only",
      ],
      "payload" = {
        "aliases" = [
          "minimal",
        ],
        "canonical_path" = "fixtures/core/canonical-values/minimal.preserves",
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "canonical-value",
          "hex" = "4aae45a48df037e16bd2a160ba815b264b5dfd40725957561f9f75300681f846",
        },
        "kind" = "canonical-value",
        "projection_claims_authority" = false,
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "b02c13f93f2ede1a565750121130cd146c0fa162824868e65f038d3250536f87",
        },
        "schema_version" = 1,
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.format_strategy.preserves_roundtrip]",
        "r[kamacite.rust_determinism.canonical_identity.blake3]",
      ],
    },
    {
      "artifact_class" = "canonical-value",
      "checked_laws" = [
        "projection-not-authority",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/canonical-value.negative.lossy-json-authority.receipt.json",
      "id" = "canonical-value.negative.lossy-json-authority",
      "non_claims" = [
        "JSON is not canonical identity",
      ],
      "payload" = {
        "aliases" = [
          "lossy-json",
        ],
        "canonical_path" = "fixtures/core/canonical-values/lossy-json.preserves",
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "canonical-value",
          "hex" = "2b9579ca093fb1ca47516d0cdac8fe5dccf015400f642230a4a14eed7e0cc3ae",
        },
        "kind" = "canonical-value",
        "projection_claims_authority" = true,
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "b02c13f93f2ede1a565750121130cd146c0fa162824868e65f038d3250536f87",
        },
        "schema_version" = 1,
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.negative_fixtures.lossy_projection]",
      ],
    },
    {
      "artifact_class" = "receipt",
      "checked_laws" = [
        "typed-domain-rejection",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/identity.negative.projection-as-canonical.receipt.json",
      "id" = "identity.negative.projection-as-canonical",
      "non_claims" = [
        "not identity-domain substitution",
      ],
      "payload" = {
        "kind" = "identity-use",
        "required_domain" = "canonical-value",
        "supplied_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "b02c13f93f2ede1a565750121130cd146c0fa162824868e65f038d3250536f87",
        },
      },
      "requirement_ids" = [
        "r[kamacite.rust_determinism.typed_identity_domains.projection_not_canonical]",
      ],
    },
    {
      "artifact_class" = "receipt",
      "checked_laws" = [
        "receipt-identity-separation",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/identity.negative.source-as-receipt.receipt.json",
      "id" = "identity.negative.source-as-receipt",
      "non_claims" = [
        "receipt identity is separate domain",
      ],
      "payload" = {
        "kind" = "identity-use",
        "required_domain" = "receipt",
        "supplied_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
      },
      "requirement_ids" = [
        "r[kamacite.rust_determinism.typed_identity_domains.source_not_receipt]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "source-export-separation",
        "evaluator-identity-bound",
        "root-relative-fixture-read",
        "declared-evaluator-invocation",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/adapter.positive.nickel-source-export-separated.receipt.json",
      "id" = "adapter.positive.nickel-source-export-separated",
      "non_claims" = [
        "nickel export is derived projection",
        "not Nickel semantic-equivalence proof",
        "not canonical Kamacite IR",
      ],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = [
          "source-export-separation",
          "evaluator-identity-bound",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524",
        },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "reference-export",
        "import_paths" = [],
        "sandbox_profile" = nickel_sandbox_profile,
        "sandbox_evidence" = nickel_sandbox_evidence
          "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93"
          "success"
          0
          { algorithm = "blake3", domain = "projection", hex = "50cc1102b1c612e6962547aacdcef9a400d4416ef8dd9388e885991853c400c9" }
          "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
        "export_path" = "fixtures/adapters/nickel/valid-export.json",
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "50cc1102b1c612e6962547aacdcef9a400d4416ef8dd9388e885991853c400c9",
        },
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.nickel_adapter_suite.positive_fixtures.valid_export]",
        "r[kamacite.nickel_adapter_suite.source_export_separation.derived_projection]",
        "r[kamacite.nickel_adapter_suite.evaluator_identity.bound]",
        "r[kamacite.nickel_adapter_suite.capability_boundary.evaluator_invocation]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "source-envelope-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-missing-source-envelope.receipt.json",
      "id" = "adapter.negative.nickel-missing-source-envelope",
      "non_claims" = [
        "Nickel export projection is not source preservation evidence",
      ],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = [
          "source-envelope-required",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "declared_source_identity" = null,
        "import_paths" = [],
        "sandbox_profile" = null,
        "sandbox_evidence" = null,
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524",
        },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "stored-evidence",
        "export_path" = "fixtures/adapters/nickel/valid-export.json",
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "50cc1102b1c612e6962547aacdcef9a400d4416ef8dd9388e885991853c400c9",
        },
        "schema_version" = 1,
        "source_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.nickel_adapter_suite.negative_fixtures.missing_source]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "projection-identity-matches-export-bytes",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-stale-projection-identity.receipt.json",
      "id" = "adapter.negative.nickel-stale-projection-identity",
      "non_claims" = [
        "stale Nickel export bytes are not trusted projection evidence",
      ],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = [
          "projection-identity-matches-export-bytes",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524",
        },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "stored-evidence",
        "export_path" = "fixtures/adapters/nickel/valid-export.json",
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "0000000000000000000000000000000000000000000000000000000000000000",
        },
        "import_paths" = [],
        "sandbox_profile" = null,
        "sandbox_evidence" = null,
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.nickel_adapter_suite.negative_fixtures.stale_projection]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "json-export-not-canonical-ir",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-json-canonical-ir-overclaim.receipt.json",
      "id" = "adapter.negative.nickel-json-canonical-ir-overclaim",
      "non_claims" = [
        "Nickel JSON export is not canonical Kamacite IR",
      ],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = [
          "json-export-not-canonical-ir",
        ],
        "claims_canonical_ir" = true,
        "import_paths" = [],
        "sandbox_profile" = null,
        "sandbox_evidence" = null,
        "declared_capabilities" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524",
        },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "stored-evidence",
        "export_path" = "fixtures/adapters/nickel/valid-export.json",
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "50cc1102b1c612e6962547aacdcef9a400d4416ef8dd9388e885991853c400c9",
        },
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.nickel_adapter_suite.source_export_separation.json_not_canonical]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "evaluator-identity-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-missing-evaluator-identity.receipt.json",
      "id" = "adapter.negative.nickel-missing-evaluator-identity",
      "non_claims" = [
        "Nickel export evidence is untrusted without evaluator identity",
      ],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = [
          "evaluator-identity-required",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = null,
        "import_paths" = [],
        "sandbox_profile" = null,
        "sandbox_evidence" = null,
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "stored-evidence",
        "export_path" = "fixtures/adapters/nickel/valid-export.json",
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "50cc1102b1c612e6962547aacdcef9a400d4416ef8dd9388e885991853c400c9",
        },
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.nickel_adapter_suite.evaluator_identity.missing]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "unsupported-evaluator-fails-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-unsupported-evaluator-identity.receipt.json",
      "id" = "adapter.negative.nickel-unsupported-evaluator-identity",
      "non_claims" = [
        "unsupported evaluator identity is external untrusted evidence",
      ],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = [
          "unsupported-evaluator-fails-closed",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524",
        },
        "evaluator_profile" = "third-party-nickel-wrapper",
        "import_paths" = [],
        "sandbox_profile" = null,
        "sandbox_evidence" = null,
        "execution_mode" = "stored-evidence",
        "export_path" = "fixtures/adapters/nickel/valid-export.json",
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "50cc1102b1c612e6962547aacdcef9a400d4416ef8dd9388e885991853c400c9",
        },
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.nickel_adapter_suite.evaluator_identity.unsupported]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "export-output-decodes-as-json",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-malformed-export-output.receipt.json",
      "id" = "adapter.negative.nickel-malformed-export-output",
      "non_claims" = [
        "malformed export bytes are not trusted projection evidence",
      ],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = [
          "export-output-decodes-as-json",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524",
        },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "stored-evidence",
        "export_path" = "fixtures/adapters/nickel/malformed-export.json",
        "import_paths" = [],
        "sandbox_profile" = null,
        "sandbox_evidence" = null,
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "0dea94cdddc2781473066572c89204968f46c27e6e6d390d9e5c8db75869cff9",
        },
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.nickel_adapter_suite.negative_fixtures.malformed_export]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "undeclared-host-effects-fail-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-undeclared-host-effect.receipt.json",
      "id" = "adapter.negative.nickel-undeclared-host-effect",
      "non_claims" = [
        "declaring Nickel execution does not grant ambient host authority",
      ],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = [
          "undeclared-host-effects-fail-closed",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
        ],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
        },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524",
        },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "stored-evidence",
        "export_path" = "fixtures/adapters/nickel/valid-export.json",
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = [
          "fixture-read",
          "nickel-reference-export",
          "capture-stdout",
          "network",
        ],
        "import_paths" = [],
        "sandbox_profile" = null,
        "sandbox_evidence" = null,
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "50cc1102b1c612e6962547aacdcef9a400d4416ef8dd9388e885991853c400c9",
        },
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.nickel_adapter_suite.negative_fixtures.undeclared_effect]",
        "r[kamacite.nickel_adapter_suite.capability_boundary]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = ["sandbox-import-closure"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-sandbox-import-escape.receipt.json",
      "id" = "adapter.negative.nickel-sandbox-import-escape",
      "non_claims" = ["outside-root imports are not admitted"],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = ["sandbox-import-closure"],
        "claims_canonical_ir" = false,
        "declared_capabilities" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "declared_source_identity" = { "algorithm" = "blake3", "domain" = "source", "hex" = "d4685005dc977e4903a82bfa2bf6783a1fd72719d6e3a59f435f5782eaeb28b3" },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = { "algorithm" = "blake3", "domain" = "evaluator", "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524" },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "reference-export",
        "export_path" = "fixtures/adapters/nickel/sandbox/empty-export.json",
        "import_paths" = [],
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "projection_identity" = { "algorithm" = "blake3", "domain" = "projection", "hex" = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262" },
        "sandbox_profile" = nickel_sandbox_profile,
        "sandbox_evidence" = nickel_sandbox_evidence
          "d4685005dc977e4903a82bfa2bf6783a1fd72719d6e3a59f435f5782eaeb28b3"
          "exit-failure" 1 null
          "65c60d806494ab0518e505ac8a086fb58286f4ca0f3fd55d005114fd03f30139",
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/nickel/sandbox/import-escape.ncl",
      },
      "requirement_ids" = ["r[kamacite.findings.nickel_sandbox.import_closure]"],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = ["declared-import-closure"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-sandbox-undeclared-import.receipt.json",
      "id" = "adapter.negative.nickel-sandbox-undeclared-import",
      "non_claims" = ["sibling files are not implicitly admitted"],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = ["declared-import-closure"],
        "claims_canonical_ir" = false,
        "declared_capabilities" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "declared_source_identity" = { "algorithm" = "blake3", "domain" = "source", "hex" = "57bcaec8e2ced80f71ad975d39c033b45cc10b41182bc6bc47726b8cd75101f3" },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = { "algorithm" = "blake3", "domain" = "evaluator", "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524" },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "reference-export",
        "export_path" = "fixtures/adapters/nickel/sandbox/empty-export.json",
        "import_paths" = [],
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "projection_identity" = { "algorithm" = "blake3", "domain" = "projection", "hex" = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262" },
        "sandbox_profile" = nickel_sandbox_profile,
        "sandbox_evidence" = nickel_sandbox_evidence
          "57bcaec8e2ced80f71ad975d39c033b45cc10b41182bc6bc47726b8cd75101f3"
          "exit-failure" 1 null
          "6c9514cef195452dd9ecd3a689b69e5ddea15282a2c36996cc6349c7a5779bea",
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/nickel/sandbox/undeclared-import.ncl",
      },
      "requirement_ids" = ["r[kamacite.findings.nickel_sandbox.import_closure]"],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = ["declared-package-closure"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-sandbox-undeclared-package.receipt.json",
      "id" = "adapter.negative.nickel-sandbox-undeclared-package",
      "non_claims" = ["package inputs are denied unless reviewed"],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = ["declared-package-closure"],
        "claims_canonical_ir" = false,
        "declared_capabilities" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "declared_source_identity" = { "algorithm" = "blake3", "domain" = "source", "hex" = "fae733102b96301cf4bca3e9147a2e7cf4ca02b286addd79117b4dcb953078ad" },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = { "algorithm" = "blake3", "domain" = "evaluator", "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524" },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "reference-export",
        "export_path" = "fixtures/adapters/nickel/sandbox/empty-export.json",
        "import_paths" = [],
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "projection_identity" = { "algorithm" = "blake3", "domain" = "projection", "hex" = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262" },
        "sandbox_profile" = nickel_sandbox_profile,
        "sandbox_evidence" = nickel_sandbox_evidence
          "fae733102b96301cf4bca3e9147a2e7cf4ca02b286addd79117b4dcb953078ad"
          "exit-failure" 1 null
          "7219292e4262459688434c9ed20cd8eff59f008bc7fce8c6d9c0f77b4009a1c0",
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/nickel/sandbox/undeclared-package.ncl",
      },
      "requirement_ids" = ["r[kamacite.findings.nickel_sandbox.import_closure]"],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = ["environment-sanitized"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-sandbox-environment-dependence.receipt.json",
      "id" = "adapter.negative.nickel-sandbox-environment-dependence",
      "non_claims" = ["ambient environment is not evaluator input"],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = ["environment-sanitized"],
        "claims_canonical_ir" = false,
        "declared_capabilities" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "declared_source_identity" = { "algorithm" = "blake3", "domain" = "source", "hex" = "b91f56420df56ffa1a5640f242426f0b12787540401a3d43157e1c42c6f7c70e" },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = { "algorithm" = "blake3", "domain" = "evaluator", "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524" },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "reference-export",
        "export_path" = "fixtures/adapters/nickel/sandbox/empty-export.json",
        "import_paths" = [],
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "projection_identity" = { "algorithm" = "blake3", "domain" = "projection", "hex" = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262" },
        "sandbox_profile" = nickel_sandbox_profile,
        "sandbox_evidence" = nickel_sandbox_evidence
          "b91f56420df56ffa1a5640f242426f0b12787540401a3d43157e1c42c6f7c70e"
          "exit-failure" 1 null
          "1bbf6592a4db8870c537c758a6f9faf84a0f6812125037dc6c7e1f58606636bc",
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/nickel/sandbox/environment-dependence.ncl",
      },
      "requirement_ids" = ["r[kamacite.findings.nickel_sandbox.enforced_evidence]"],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = ["execution-budget"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-sandbox-resource-exhaustion.receipt.json",
      "id" = "adapter.negative.nickel-sandbox-resource-exhaustion",
      "non_claims" = ["nontermination is not admitted"],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = ["execution-budget"],
        "claims_canonical_ir" = false,
        "declared_capabilities" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "declared_source_identity" = { "algorithm" = "blake3", "domain" = "source", "hex" = "5809f24d41411b808fe4f39c212856ac422efe73b295cf2922aca932ae2aa9ef" },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = { "algorithm" = "blake3", "domain" = "evaluator", "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524" },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "reference-export",
        "export_path" = "fixtures/adapters/nickel/sandbox/empty-export.json",
        "import_paths" = [],
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "projection_identity" = { "algorithm" = "blake3", "domain" = "projection", "hex" = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262" },
        "sandbox_profile" = nickel_sandbox_profile,
        "sandbox_evidence" = nickel_sandbox_evidence
          "5809f24d41411b808fe4f39c212856ac422efe73b295cf2922aca932ae2aa9ef"
          "resource-limit" 134 null
          "d53f2e4f5287511e65de77ee29010ddc6fd331a144c1f9632fce5f812415cfb6",
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/nickel/sandbox/resource-exhaustion.ncl",
      },
      "requirement_ids" = ["r[kamacite.findings.nickel_sandbox.resource_limits]"],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = ["output-budget"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.nickel-sandbox-oversized-output.receipt.json",
      "id" = "adapter.negative.nickel-sandbox-oversized-output",
      "non_claims" = ["unbounded output is not admitted"],
      "payload" = {
        "adapter_kind" = "nickel-export",
        "checked_laws" = ["output-budget"],
        "claims_canonical_ir" = false,
        "declared_capabilities" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "declared_source_identity" = { "algorithm" = "blake3", "domain" = "source", "hex" = "a990b8a9f1ac5a655c9b2f8b7e48108a476c3ceee2e9643f2c3ba971b5fc48c5" },
        "evaluator_descriptor" = "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)",
        "evaluator_identity" = { "algorithm" = "blake3", "domain" = "evaluator", "hex" = "97c4f5c61695ff74293d0eca8371587368de724256328082e095867fbfac3524" },
        "evaluator_profile" = "nickel-reference-cli",
        "execution_mode" = "reference-export",
        "export_path" = "fixtures/adapters/nickel/sandbox/oversized-export-prefix.json",
        "import_paths" = [],
        "kind" = "nickel-adapter-receipt",
        "observed_effects" = ["fixture-read", "nickel-reference-export", "capture-stdout"],
        "projection_identity" = { "algorithm" = "blake3", "domain" = "projection", "hex" = "4948eb60229c01923ca4ef87c42fc069f106446000103828a68e22cfd5300634" },
        "sandbox_profile" = nickel_sandbox_profile,
        "sandbox_evidence" = nickel_sandbox_evidence
          "a990b8a9f1ac5a655c9b2f8b7e48108a476c3ceee2e9643f2c3ba971b5fc48c5"
          "output-limit" 153
          { algorithm = "blake3", domain = "projection", hex = "4948eb60229c01923ca4ef87c42fc069f106446000103828a68e22cfd5300634" }
          "2c29636f125516625621fd12e2b81b4dd8f54cd3a24cda829a384bd0dcb74fb6",
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/nickel/sandbox/oversized-output.ncl",
      },
      "requirement_ids" = ["r[kamacite.findings.nickel_sandbox.resource_limits]"],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "declared-host-functions",
        "dto-only-evaluation",
        "output-projection-bound",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/adapter.positive.steel-dto-only.receipt.json",
      "id" = "adapter.positive.steel-dto-only",
      "non_claims" = [
        "not ambient Steel scripting authority",
        "not Steel evaluator soundness",
        "not canonical Kamacite IR",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "declared-host-functions",
          "dto-only-evaluation",
          "output-projection-bound",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "dto-probe",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.integer-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/dto-output.json",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "8945ab91591ef0cd6b3caa2843789aecc808a039bf202c888fb39108c41fe61e",
        },
        "output_schema" = "kamacite.dto.integer-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.positive_fixtures.dto_only]",
        "r[kamacite.steel_adapter_suite.receipt_profile.evaluator_identity]",
        "r[kamacite.steel_adapter_suite.host_function_boundary.declared_host_functions]",
        "r[kamacite.steel_adapter_suite.output_boundary.dto_only]",
        "r[kamacite.findings.steel_execution.fresh_probe]",
        "r[kamacite.findings.steel_execution.runner_identity]",
        "r[kamacite.findings.steel_execution.schema_conformance]",
        "r[kamacite.findings.steel_execution.capability_enforcement]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "declared-module-imports",
        "root-relative-fixture-read",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/adapter.positive.steel-declared-module.receipt.json",
      "id" = "adapter.positive.steel-declared-module",
      "non_claims" = [
        "declared module descriptor is not portable closure identity",
        "not ambient module loading",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "declared-module-imports",
          "root-relative-fixture-read",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "dto-probe",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.module-output-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/declared-module-output.json",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "16a4492b01b703597130f18c2dd17bdbd9f700043820eff791900f02c5d04a25",
        },
        "output_schema" = "kamacite.dto.module-output-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.positive_fixtures.declared_module]",
        "r[kamacite.steel_adapter_suite.receipt_profile.source_module_identity]",
        "r[kamacite.steel_adapter_suite.capability_boundary.root_relative]",
        "r[kamacite.findings.steel_execution.fresh_probe]",
        "r[kamacite.findings.steel_execution.runner_identity]",
        "r[kamacite.findings.steel_execution.schema_conformance]",
        "r[kamacite.findings.steel_execution.capability_enforcement]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "evaluator-identity-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-missing-evaluator-identity.receipt.json",
      "id" = "adapter.negative.steel-missing-evaluator-identity",
      "non_claims" = [
        "Steel output evidence is untrusted without evaluator identity",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "evaluator-identity-required",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = null,
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.integer-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/dto-output.json",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "8945ab91591ef0cd6b3caa2843789aecc808a039bf202c888fb39108c41fe61e",
        },
        "output_schema" = "kamacite.dto.integer-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.negative_fixtures.missing_evaluator]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "unsupported-evaluator-fails-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-unsupported-evaluator-identity.receipt.json",
      "id" = "adapter.negative.steel-unsupported-evaluator-identity",
      "non_claims" = [
        "unsupported Steel evaluator evidence remains external and untrusted",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "unsupported-evaluator-fails-closed",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "third-party-steel-wrapper",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.integer-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/dto-output.json",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "8945ab91591ef0cd6b3caa2843789aecc808a039bf202c888fb39108c41fe61e",
        },
        "output_schema" = "kamacite.dto.integer-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.negative_fixtures.unsupported_evaluator]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "undeclared-host-functions-fail-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-undeclared-host-function.receipt.json",
      "id" = "adapter.negative.steel-undeclared-host-function",
      "non_claims" = [
        "observed host functions are not trusted without descriptor declaration",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "undeclared-host-functions-fail-closed",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.integer-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
          "kamacite.read-file",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/dto-output.json",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "8945ab91591ef0cd6b3caa2843789aecc808a039bf202c888fb39108c41fe61e",
        },
        "output_schema" = "kamacite.dto.integer-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.host_function_boundary.undeclared_host_function]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "deny-undeclared-authority",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-ambient-filesystem.receipt.json",
      "id" = "adapter.negative.steel-ambient-filesystem",
      "non_claims" = [
        "ambient filesystem denied",
        "declaring Steel evaluation does not grant host authority",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "deny-undeclared-authority",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.integer-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
          "filesystem",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/dto-output.json",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "8945ab91591ef0cd6b3caa2843789aecc808a039bf202c888fb39108c41fe61e",
        },
        "output_schema" = "kamacite.dto.integer-v1",
        "requested_imports" = [
          "steel/base",
          "filesystem",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.host_function_boundary.ambient_import]",
        "r[kamacite.steel_adapter_suite.negative_fixtures.ambient_authority]",
        "r[kamacite.steel_adapter_suite.capability_boundary.mismatch]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "output-projection-identity-matches",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-stale-output-identity.receipt.json",
      "id" = "adapter.negative.steel-stale-output-identity",
      "non_claims" = [
        "stale Steel output bytes are not trusted projection evidence",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "output-projection-identity-matches",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.integer-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/dto-output.json",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "16a4492b01b703597130f18c2dd17bdbd9f700043820eff791900f02c5d04a25",
        },
        "output_schema" = "kamacite.dto.integer-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.negative_fixtures.stale_output]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "typed-output-identity-domain",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-output-wrong-domain.receipt.json",
      "id" = "adapter.negative.steel-output-wrong-domain",
      "non_claims" = [
        "output identity domains are not interchangeable",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "typed-output-identity-domain",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.integer-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/dto-output.json",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "output_schema" = "kamacite.dto.integer-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.negative_fixtures.stale_output]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "dto-output-decodes",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-malformed-output.receipt.json",
      "id" = "adapter.negative.steel-malformed-output",
      "non_claims" = [
        "malformed Steel output is not trusted DTO evidence",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "dto-output-decodes",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.integer-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "dto",
        "output_path" = "fixtures/adapters/steel/malformed-output.txt",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "1b177a67c3e7bb8eea9d46dd2591fed49356206d8a37a3637acac1b00f0278b9",
        },
        "output_schema" = "kamacite.dto.integer-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.negative_fixtures.malformed_output]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "opaque-closure-export-denied",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-opaque-closure-export.receipt.json",
      "id" = "adapter.negative.steel-opaque-closure-export",
      "non_claims" = [
        "opaque closures are not portable Kamacite values",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "opaque-closure-export-denied",
        ],
        "claims_canonical_ir" = false,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.callable-descriptor-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "opaque-closure",
        "output_path" = "fixtures/adapters/steel/opaque-closure-output.txt",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "896c85e34bd6f3c9445415bbe7c7fa43a4a47de1bed66a0c0a3e0ce49a478dad",
        },
        "output_schema" = "kamacite.dto.callable-descriptor-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.output_boundary.opaque_closure]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "bytecode-not-canonical-ir",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.steel-bytecode-canonical-ir-overclaim.receipt.json",
      "id" = "adapter.negative.steel-bytecode-canonical-ir-overclaim",
      "non_claims" = [
        "Steel bytecode and JSON strings are not canonical Kamacite IR",
      ],
      "payload" = {
        "adapter_kind" = "steel-dto-probe",
        "checked_laws" = [
          "bytecode-not-canonical-ir",
        ],
        "claims_canonical_ir" = true,
        "declared_builtin_modules" = [
          "steel/base",
        ],
        "declared_capabilities" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "declared_imports" = [],
        "declared_source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "2e9351bf6cd31a2cfd9634f7aa02ee64ecdc04af552ea8aa53e5f1f3ddf073c4",
        },
        "effect_class" = "pure-dto",
        "evaluator_descriptor" = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1",
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "b06189cdd31f3de2f7f0ee18640199bb287e54338b677bf092c07dd81cf80040",
        },
        "evaluator_profile" = "steel-dto-probe-shell",
        "execution_mode" = "stored-evidence",
        "host_functions" = [
          {
            "checked_laws" = [
              "host-function-dto-contract",
            ],
            "effect_class" = "pure-dto",
            "function_id" = "kamacite.add-one",
            "input_schema" = "kamacite.dto.integer-v1",
            "output_schema" = "kamacite.dto.bytecode-v1",
            "required_capabilities" = [
              "dto-evaluation",
            ],
          },
        ],
        "input_schema" = "kamacite.dto.integer-v1",
        "kind" = "steel-adapter-receipt",
        "module_descriptor_path" = "fixtures/adapters/steel/module-descriptor.json",
        "module_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "4b849c53a6565ad8886186111f445c52ea6f9b96e0311d3f4703c401d2af4195",
        },
        "observed_effects" = [
          "fixture-read",
          "steel-dto-probe",
          "capture-stdout",
          "dto-evaluation",
        ],
        "observed_host_functions" = [
          "kamacite.add-one",
        ],
        "output_class" = "bytecode",
        "output_path" = "fixtures/adapters/steel/bytecode-output.bin.txt",
        "output_projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "5a3b0254cc893f929f3c0bd865521e662a4304b57475cbe587c3b0c7db4c2fb8",
        },
        "output_schema" = "kamacite.dto.bytecode-v1",
        "requested_imports" = [
          "steel/base",
        ],
        "schema_version" = 1,
        "source_path" = "fixtures/adapters/steel/dto-only.scm",
      },
      "requirement_ids" = [
        "r[kamacite.steel_adapter_suite.output_boundary.bytecode_not_ir]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "conformance-entry-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.untrusted-without-conformance.receipt.json",
      "id" = "adapter.negative.untrusted-without-conformance",
      "non_claims" = [
        "no adapter-specific trust without conformance",
      ],
      "payload" = {
        "adapter_kind" = "unknown-adapter",
        "checked_laws" = [
          "conformance-entry-required",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [],
        "has_conformance_entry" = false,
        "kind" = "adapter-receipt",
        "observed_effects" = [],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.adapter_conformance.untrusted_without_entry]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "canonical-ir-overclaim-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.wasm-canonical-ir-overclaim.receipt.json",
      "id" = "adapter.negative.wasm-canonical-ir-overclaim",
      "non_claims" = [
        "compiled WASM is target artifact only",
      ],
      "payload" = {
        "adapter_kind" = "wasm-adapter",
        "checked_laws" = [
          "canonical-ir-overclaim-rejected",
        ],
        "claims_canonical_ir" = true,
        "declared_capabilities" = [],
        "has_conformance_entry" = true,
        "kind" = "adapter-receipt",
        "observed_effects" = [],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.wasm_adapter_conformance.not_ir]",
      ],
    },
    {
      "artifact_class" = "target-artifact",
      "checked_laws" = [
        "target-provenance-bound",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/target.positive.wasm-receipt-bounded.receipt.json",
      "id" = "target.positive.wasm-receipt-bounded",
      "non_claims" = [
        "WASM artifact is not canonical Kamacite IR",
      ],
      "payload" = {
        "claims_canonical_ir" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "target-artifact",
          "hex" = "073709f60d49930fa5e855ecaad567aed0d3c8aa91885934c2f566067ad0601b",
        },
        "kind" = "target-artifact",
        "metadata" = {
          "adapter-role" = "validator",
          "target" = "wasm32-wasi",
        },
        "output_path" = "fixtures/core/targets/validator.wasm.txt",
        "target_kind" = "wasm-component",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.wasm_adapter_conformance.receipt]",
      ],
    },
    {
      "artifact_class" = "target-artifact",
      "checked_laws" = [
        "compiler-target-provenance",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/target.positive.llvm-provenance-bounded.receipt.json",
      "id" = "target.positive.llvm-provenance-bounded",
      "non_claims" = [
        "LLVM IR is target evidence only",
      ],
      "payload" = {
        "claims_canonical_ir" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "target-artifact",
          "hex" = "1f2f5a5f9ff7f04d8a8e9e79890df6c797599ec381c1bba47ae615d9a2922dd5",
        },
        "kind" = "target-artifact",
        "metadata" = {
          "compiler" = "rustc-deferred",
          "debug-info" = "not-recorded",
          "optimization-level" = "0",
          "target-triple" = "x86_64-unknown-linux-gnu",
        },
        "output_path" = "fixtures/core/targets/validator.ll",
        "target_kind" = "llvm-ir",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.compiler_target_conformance.llvm_metadata]",
      ],
    },
    {
      "artifact_class" = "unsupported-claim",
      "checked_laws" = [
        "unsupported-claim-fails-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/claim.negative.arbitrary-source-translation.receipt.json",
      "id" = "claim.negative.arbitrary-source-translation",
      "non_claims" = [
        "not arbitrary host-language equivalence",
      ],
      "payload" = {
        "claim_class" = "unsupported-source-translation",
        "claim_text" = "lossless arbitrary Nickel to Steel translation",
        "kind" = "unsupported-claim",
        "preserve_source_only" = true,
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.negative_fixtures.unsupported_translation]",
      ],
    },
    {
      "artifact_class" = "unsupported-claim",
      "checked_laws" = [
        "ambiguous-claim-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/claim.negative.ambiguous-semantic-equivalence.receipt.json",
      "id" = "claim.negative.ambiguous-semantic-equivalence",
      "non_claims" = [
        "not semantic equivalence proof",
      ],
      "payload" = {
        "claim_class" = "semantic-equivalence",
        "claim_text" = "compiler and evaluator semantics are equivalent",
        "kind" = "unsupported-claim",
        "preserve_source_only" = false,
      },
      "requirement_ids" = [
        "r[kamacite.rust_determinism.total_diagnostics.ambiguous_claim]",
      ],
    },
    {
      "artifact_class" = "capability-shell",
      "checked_laws" = [
        "root-relative-path",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/capability.positive.normal-root-relative.receipt.json",
      "id" = "capability.positive.normal-root-relative",
      "non_claims" = [
        "capability shell evidence is not core filesystem authority",
      ],
      "payload" = {
        "declared_filesystem" = true,
        "kind" = "capability-shell",
        "observation" = "normal-relative",
        "path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.rust_determinism.capability_shells.filesystem_root]",
      ],
    },
    {
      "artifact_class" = "capability-shell",
      "checked_laws" = [
        "path-escape-denied",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/capability.negative.parent-traversal.receipt.json",
      "id" = "capability.negative.parent-traversal",
      "non_claims" = [
        "path traversal denied",
      ],
      "payload" = {
        "declared_filesystem" = true,
        "kind" = "capability-shell",
        "observation" = "parent-traversal",
        "path" = "../outside",
      },
      "requirement_ids" = [
        "r[kamacite.rust_determinism.capability_shells.escape_fixtures]",
      ],
    },
    {
      "artifact_class" = "capability-shell",
      "checked_laws" = [
        "path-escape-denied",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/capability.negative.absolute-path.receipt.json",
      "id" = "capability.negative.absolute-path",
      "non_claims" = [
        "absolute paths are not artifact authority",
      ],
      "payload" = {
        "declared_filesystem" = true,
        "kind" = "capability-shell",
        "observation" = "absolute-path",
        "path" = "/etc/passwd",
      },
      "requirement_ids" = [
        "r[kamacite.rust_determinism.capability_shells.escape_fixtures]",
      ],
    },
    {
      "artifact_class" = "capability-shell",
      "checked_laws" = [
        "path-escape-denied",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/capability.negative.symlink-escape.receipt.json",
      "id" = "capability.negative.symlink-escape",
      "non_claims" = [
        "symlink escape denied",
      ],
      "payload" = {
        "declared_filesystem" = true,
        "kind" = "capability-shell",
        "observation" = "symlink-escape",
        "path" = "fixtures/symlink-to-outside",
      },
      "requirement_ids" = [
        "r[kamacite.rust_determinism.capability_shells.escape_fixtures]",
      ],
    },
    {
      "artifact_class" = "capability-shell",
      "checked_laws" = [
        "undeclared-filesystem-denied",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/capability.negative.undeclared-filesystem.receipt.json",
      "id" = "capability.negative.undeclared-filesystem",
      "non_claims" = [
        "filesystem authority denied by default",
      ],
      "payload" = {
        "declared_filesystem" = false,
        "kind" = "capability-shell",
        "observation" = "undeclared-filesystem",
        "path" = "fixtures/core/source-envelopes/valid.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.rust_determinism.explicit_capabilities.filesystem]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "bounded-valence-export",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/stack.positive.valence-wrap-receipt.receipt.json",
      "id" = "stack.positive.valence-wrap-receipt",
      "non_claims" = [
        "Valence export does not broaden Kamacite claim",
      ],
      "payload" = {
        "bounded_claim" = "wrap Kamacite receipt identity and non-claims",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "source",
            "hex" = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93",
          },
        ],
        "role" = "valence-export",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.stack_evidence_boundary.valence_receipts]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "octet-source-shape-only",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/stack.positive.octet-source-shape-only.receipt.json",
      "id" = "stack.positive.octet-source-shape-only",
      "non_claims" = [
        "Octet check is source-shape evidence only",
      ],
      "payload" = {
        "bounded_claim" = "Rust profile source-shape evidence",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [],
        "role" = "octet-check-plan",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.evidence_integration_conformance.octet]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "trellis-supporting-only",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/stack.positive.trellis-supporting-evidence.receipt.json",
      "id" = "stack.positive.trellis-supporting-evidence",
      "non_claims" = [
        "Trellis proof reference does not replace Kamacite receipt",
      ],
      "payload" = {
        "bounded_claim" = "supporting proof evidence for finite effect facts",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [],
        "role" = "trellis-proof-reference",
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.stack_roles.trellis.supporting_evidence]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "runtime-consumes-kamacite-identity",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/stack.positive.aspen-handler-profile-reference.receipt.json",
      "id" = "stack.positive.aspen-handler-profile-reference",
      "non_claims" = [
        "Aspen runtime evidence does not redefine Kamacite effect schema",
      ],
      "payload" = {
        "bounded_claim" = "runtime evidence references Kamacite handler descriptor identity",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "algebraic-effect",
            "hex" = "283821c5e3b7364cc163169d83890da6ce852be02316aa836f9bdda7ff0750f8",
          },
        ],
        "role" = "aspen-runtime-consumption",
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.stack_roles.aspen.handler_profile]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "stack-overclaim-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/stack.negative.valence-runtime-correctness-overclaim.receipt.json",
      "id" = "stack.negative.valence-runtime-correctness-overclaim",
      "non_claims" = [
        "Valence wrapping is not runtime correctness",
      ],
      "payload" = {
        "bounded_claim" = "runtime correctness",
        "broadens_claim" = true,
        "kind" = "stack-evidence",
        "referenced_identities" = [],
        "role" = "valence-export",
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.stack_roles.valence.wrap_receipt]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "exact-discharge",
        "recorded-replay-log-bound",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/algebraic.positive.declared-discharge-recorded-replay.receipt.json",
      "id" = "algebraic.positive.declared-discharge-recorded-replay",
      "non_claims" = [
        "effect log is replay evidence only",
        "not host authorization",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/declared-discharge.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "561617d253e0e0cf29bd8ab5b8758d848bda594cde178b2572d3939d19a6c21c",
        },
        "handler" = {
          "claims_discharge" = true,
          "continuation_policy" = "no-continuation",
          "handled_operations" = [
            "op.read-config",
          ],
          "host_capability_requirements" = [],
          "name" = "memory-config-handler",
          "residual_operations" = [],
        },
        "input_row" = [
          "op.read-config",
        ],
        "kind" = "algebraic-effect",
        "receipt" = {
          "checked_laws" = [
            "exact-discharge",
          ],
          "continuation_evidence" = null,
          "continuation_policy" = "no-continuation",
          "discharged_operations" = [
            "op.read-config",
          ],
          "observed_operations" = [
            "op.read-config",
          ],
          "residual_operations" = [],
        },
        "replay" = {
          "effect_log_identity" = {
            "algorithm" = "blake3",
            "domain" = "effect-log",
            "hex" = "c5f566889bc03678adeb0bf686685fd3f2472aa106bdeca86ecab3b454ba8f28",
          },
          "effect_log_path" = "fixtures/core/algebraic-effects/recorded-log.preserves",
          "replay_class" = "recorded-replay",
        },
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "ReadConfig",
            "operation_id" = "op.read-config",
            "parameter_schema" = "unit",
            "result_schema" = "string",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.fixture_corpus.positive_discharge]",
        "r[kamacite.algebraic_effects.replay_profile.recorded_log]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "single-use-continuation-evidence",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/algebraic.positive.single-use-continuation-evidence.receipt.json",
      "id" = "algebraic.positive.single-use-continuation-evidence",
      "non_claims" = [
        "single-use evidence is bounded receipt data",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/single-use-continuation-evidence.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "5e31708b44f1a241746fb8841195875679d775da70850558303db3985dc1669b",
        },
        "handler" = {
          "claims_discharge" = true,
          "continuation_policy" = "single-use-continuation",
          "handled_operations" = [
            "op.resume-once",
          ],
          "host_capability_requirements" = [],
          "name" = "single-use-handler",
          "residual_operations" = [],
        },
        "input_row" = [
          "op.resume-once",
        ],
        "kind" = "algebraic-effect",
        "receipt" = {
          "checked_laws" = [
            "single-use-continuation-evidence",
          ],
          "continuation_evidence" = "used-once",
          "continuation_policy" = "single-use-continuation",
          "discharged_operations" = [
            "op.resume-once",
          ],
          "observed_operations" = [
            "op.resume-once",
          ],
          "residual_operations" = [],
        },
        "replay" = null,
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "ResumeOnce",
            "operation_id" = "op.resume-once",
            "parameter_schema" = "unit",
            "result_schema" = "unit",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.continuation_policy.single_use_evidence]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "fail-closed-effect-version",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.unsupported-version.receipt.json",
      "id" = "algebraic.negative.unsupported-version",
      "non_claims" = [
        "not compatibility defaulting",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/unsupported-version.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "5dbe0a11c4da10b4a32360686ca828fa6714cdccb19ae719b4e7dcdfc1738772",
        },
        "handler" = null,
        "input_row" = [
          "op.read-config",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = null,
        "row_open" = false,
        "schema_version" = 2,
        "signatures" = [
          {
            "display_name" = "ReadConfig",
            "operation_id" = "op.read-config",
            "parameter_schema" = "unit",
            "result_schema" = "string",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.ir_boundary.unsupported_version]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "operation-schema-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.missing-operation-schema.receipt.json",
      "id" = "algebraic.negative.missing-operation-schema",
      "non_claims" = [
        "incomplete effect signature denied",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/missing-schema.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "ea18dda5d0ec944b06f843e2f5aad0a5c2d88ad3dd22c5f8605b9226b5c78c37",
        },
        "handler" = null,
        "input_row" = [
          "op.broken",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = null,
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Broken",
            "operation_id" = "op.broken",
            "parameter_schema" = null,
            "result_schema" = "unit",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.signature_vocabulary.missing_schema]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "declared-operation-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.undeclared-operation.receipt.json",
      "id" = "algebraic.negative.undeclared-operation",
      "non_claims" = [
        "undeclared operations fail closed",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/unknown-effect.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "1590cc9decb807a36a274e0b6c8acf06c8dda629aa70fda68fc66dae53e63ed7",
        },
        "handler" = null,
        "input_row" = [
          "op.unknown",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = null,
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Known",
            "operation_id" = "op.known",
            "parameter_schema" = "unit",
            "result_schema" = "unit",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.row_summary_vocabulary.undeclared_operation]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "open-row-preserved",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.open-row-not-pure.receipt.json",
      "id" = "algebraic.negative.open-row-not-pure",
      "non_claims" = [
        "open rows are not inferred pure",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/open-row.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = true,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "a27004d78d76879ffa3f3ad7675ab6fd1706606604b389a87d0849adf52401b1",
        },
        "handler" = null,
        "input_row" = [
          "op.trace",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = null,
        "row_open" = true,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Trace",
            "operation_id" = "op.trace",
            "parameter_schema" = "string",
            "result_schema" = "unit",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.row_summary_vocabulary.open_row_not_pure]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "operation-ids-required-for-discharge",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.handler-name-not-discharge.receipt.json",
      "id" = "algebraic.negative.handler-name-not-discharge",
      "non_claims" = [
        "handler names are metadata",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/handler-name-not-discharge.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "f1b788477a3b51182b8d8f7615fba4fcb24a1a84666dd28a7f965787595ec751",
        },
        "handler" = {
          "claims_discharge" = true,
          "continuation_policy" = "no-continuation",
          "handled_operations" = [],
          "host_capability_requirements" = [],
          "name" = "trusted-looking-handler",
          "residual_operations" = [],
        },
        "input_row" = [
          "op.read-config",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = null,
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "ReadConfig",
            "operation_id" = "op.read-config",
            "parameter_schema" = "unit",
            "result_schema" = "string",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.handler_descriptor.name_not_discharge]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "receipt-effects-declared",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.unknown-effect-in-receipt.receipt.json",
      "id" = "algebraic.negative.unknown-effect-in-receipt",
      "non_claims" = [
        "unknown effects denied",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/unknown-effect-in-receipt.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "dad89096a96a7937c0b3b464e69ac86dd1e3717b29d3c52f6bea1bee600192ea",
        },
        "handler" = {
          "claims_discharge" = true,
          "continuation_policy" = "no-continuation",
          "handled_operations" = [
            "op.known",
          ],
          "host_capability_requirements" = [],
          "name" = "known-handler",
          "residual_operations" = [],
        },
        "input_row" = [
          "op.known",
        ],
        "kind" = "algebraic-effect",
        "receipt" = {
          "checked_laws" = [
            "receipt-effects-declared",
          ],
          "continuation_evidence" = null,
          "continuation_policy" = "no-continuation",
          "discharged_operations" = [
            "op.known",
          ],
          "observed_operations" = [
            "op.unknown",
          ],
          "residual_operations" = [],
        },
        "replay" = null,
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Known",
            "operation_id" = "op.known",
            "parameter_schema" = "unit",
            "result_schema" = "unit",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.handler_receipts.unknown_effect_denied]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "continuation-policy-fail-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.continuation-policy-mismatch.receipt.json",
      "id" = "algebraic.negative.continuation-policy-mismatch",
      "non_claims" = [
        "continuation use must match descriptor",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/continuation-mismatch.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "370d58d8433dca0b34bd7fcb892d6c2edf825d606fb07e7f34be5cebb45ee94c",
        },
        "handler" = {
          "claims_discharge" = true,
          "continuation_policy" = "no-continuation",
          "handled_operations" = [
            "op.resume",
          ],
          "host_capability_requirements" = [],
          "name" = "no-cont-handler",
          "residual_operations" = [],
        },
        "input_row" = [
          "op.resume",
        ],
        "kind" = "algebraic-effect",
        "receipt" = {
          "checked_laws" = [
            "continuation-policy-fail-closed",
          ],
          "continuation_evidence" = "used-once",
          "continuation_policy" = "single-use-continuation",
          "discharged_operations" = [
            "op.resume",
          ],
          "observed_operations" = [
            "op.resume",
          ],
          "residual_operations" = [],
        },
        "replay" = null,
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Resume",
            "operation_id" = "op.resume",
            "parameter_schema" = "unit",
            "result_schema" = "unit",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.continuation_policy.mismatch]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "operation-not-authority",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.host-capability-confusion.receipt.json",
      "id" = "algebraic.negative.host-capability-confusion",
      "non_claims" = [
        "operation declarations do not grant host authority",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/non-replayable.preserves",
        "claimed_host_authority" = [
          "random",
        ],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "122c85def5fdbae3b504a6b47102281db09f97088443d36f9be183279550aef0",
        },
        "handler" = null,
        "input_row" = [
          "op.random",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = null,
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Random",
            "operation_id" = "op.random",
            "parameter_schema" = "unit",
            "result_schema" = "bytes",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.fixture_corpus.negative_host_confusion]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "algebraic-hash-match",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.stale-canonical-identity.receipt.json",
      "id" = "algebraic.negative.stale-canonical-identity",
      "non_claims" = [
        "stale algebraic-effect identity rejected",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/stale-canonical-identity.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "c8de810059cdb58853ae386d5266e77b3d9efab3dc1d03cc78de149c9b0cd25e",
        },
        "handler" = null,
        "input_row" = [
          "op.read-config",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = null,
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "ReadConfig",
            "operation_id" = "op.read-config",
            "parameter_schema" = "unit",
            "result_schema" = "string",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.fixture_corpus.negative_stale_identity]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "non-replayable-not-replay",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/algebraic.negative.non-replayable-evidence-misuse.receipt.json",
      "id" = "algebraic.negative.non-replayable-evidence-misuse",
      "non_claims" = [
        "non-replayable effects are not replay evidence",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/non-replayable-evidence-misuse.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "5424cff66dc82cbe3ffa7d63caaf2ddf4e51ce3fc34fd75aa0945db8799945c6",
        },
        "handler" = null,
        "input_row" = [
          "op.random",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = {
          "effect_log_identity" = {
            "algorithm" = "blake3",
            "domain" = "effect-log",
            "hex" = "c5f566889bc03678adeb0bf686685fd3f2472aa106bdeca86ecab3b454ba8f28",
          },
          "effect_log_path" = "fixtures/core/algebraic-effects/recorded-log.preserves",
          "replay_class" = "non-replayable",
        },
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Random",
            "operation_id" = "op.random",
            "parameter_schema" = "unit",
            "result_schema" = "bytes",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.replay_profile.non_replayable]",
      ],
    },
    {
      "artifact_class" = "source-envelope",
      "checked_laws" = [
        "generated-contract-fixture-only",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/source-envelope.positive.schema-generated-nickel-contract.receipt.json",
      "id" = "source-envelope.positive.schema-generated-nickel-contract",
      "non_claims" = [
        "generated contract corpus is fixture input, not Kamacite schema authority",
      ],
      "payload" = {
        "aliases" = [
          "generated-contract-stress",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "d1d342f960ad13bea41c89601322ae633aa819d6a4f7ab9d815731cb61abb791",
        },
        "kind" = "source-envelope",
        "language" = "nickel",
        "media_type" = "text/x-nickel",
        "parser_identity" = "generated-contract-corpus-fixture",
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/generated-contract.ncl",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.nickel_reference_boundary.generated_contracts]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = [
        "parser-spans-not-evaluation",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/adapter.positive.tree-sitter-nickel-spans.receipt.json",
      "id" = "adapter.positive.tree-sitter-nickel-spans",
      "non_claims" = [
        "parser spans are syntax evidence only",
      ],
      "payload" = {
        "adapter_kind" = "tree-sitter-nickel-span-index",
        "checked_laws" = [
          "parser-spans-not-evaluation",
        ],
        "claims_canonical_ir" = false,
        "declared_capabilities" = [],
        "has_conformance_entry" = true,
        "kind" = "adapter-receipt",
        "observed_effects" = [],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.nickel_reference_boundary.parser_spans]",
        "r[kamacite.nickel_adapter_suite.positive_fixtures.parser_non_claim]",
      ],
    },
    {
      "artifact_class" = "source-envelope",
      "checked_laws" = [
        "haskell-source-envelope-only",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/source-envelope.positive.haskell-source-only.receipt.json",
      "id" = "source-envelope.positive.haskell-source-only",
      "non_claims" = [
        "Haskell source is preserved source only, not Kamacite IR",
      ],
      "payload" = {
        "aliases" = [
          "haskell-source-only",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "9401c4c4268b8632f3d4ab463dca3a330797b3181ba34d8fd2c0b65ec372f0e7",
        },
        "kind" = "source-envelope",
        "language" = "haskell",
        "media_type" = "text/x-haskell",
        "parser_identity" = "haskell-parser-deferred",
        "schema_version" = 1,
        "source_path" = "fixtures/core/source-envelopes/example.hs",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.haskell_boundary.source_envelope]",
      ],
    },
    {
      "artifact_class" = "target-artifact",
      "checked_laws" = [
        "rust-mir-provenance-only",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/target.positive.rust-mir-provenance-bounded.receipt.json",
      "id" = "target.positive.rust-mir-provenance-bounded",
      "non_claims" = [
        "Rust MIR is target provenance evidence only",
      ],
      "payload" = {
        "claims_canonical_ir" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "target-artifact",
          "hex" = "be4fb8bd87ea3ba775e1ecc56c91beda334167f2833e4b70f4fbd8a554ad3a30",
        },
        "kind" = "target-artifact",
        "metadata" = {
          "artifact-role" = "mir-provenance",
          "compiler" = "rustc-deferred",
          "target-triple" = "x86_64-unknown-linux-gnu",
        },
        "output_path" = "fixtures/core/targets/rust-mir.txt",
        "target_kind" = "rust-mir",
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.compiler_target_conformance.ghc_mir_provenance]",
      ],
    },
    {
      "artifact_class" = "unsupported-claim",
      "checked_laws" = [
        "ghc-ir-overclaim-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/claim.negative.ghc-core-as-canonical-ir.receipt.json",
      "id" = "claim.negative.ghc-core-as-canonical-ir",
      "non_claims" = [
        "GHC Core/STG/bytecode/interface files are not canonical Kamacite IR",
      ],
      "payload" = {
        "claim_class" = "semantic-equivalence",
        "claim_text" = "GHC Core is canonical lossless Kamacite IR",
        "kind" = "unsupported-claim",
        "preserve_source_only" = false,
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.haskell_boundary.ghc_ir_not_kamacite_ir]",
      ],
    },
    {
      "artifact_class" = "canonical-value",
      "checked_laws" = [
        "names-as-metadata",
        "dependency-closure-bound",
        "durable-storage-validates-before-decode",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "property",
      "golden_receipt" = "fixtures/golden/canonical-value.property.names-dependencies-durable-storage.receipt.json",
      "id" = "canonical-value.property.names-dependencies-durable-storage",
      "non_claims" = [
        "property fixture is enumerated deterministic evidence, not generated randomness",
      ],
      "payload" = {
        "aliases" = [
          "alias-a",
          "alias-b",
        ],
        "canonical_path" = "fixtures/core/canonical-values/minimal.preserves",
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "canonical-value",
          "hex" = "4aae45a48df037e16bd2a160ba815b264b5dfd40725957561f9f75300681f846",
        },
        "kind" = "canonical-value",
        "projection_claims_authority" = false,
        "projection_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "b02c13f93f2ede1a565750121130cd146c0fa162824868e65f038d3250536f87",
        },
        "schema_version" = 1,
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.property_fixtures.replayable]",
        "r[kamacite.lossless_ir.content_addressed_identity.names_metadata]",
        "r[kamacite.lossless_ir.content_addressed_identity.dependency_closure]",
        "r[kamacite.lossless_ir.content_addressed_identity.durable_storage]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "open-row-status-preserved",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/algebraic.positive.open-row-preserved-not-pure.receipt.json",
      "id" = "algebraic.positive.open-row-preserved-not-pure",
      "non_claims" = [
        "open row is preserved and not classified pure",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/open-row-preserved.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "a16d622f2b7061dbbe8681b494fd86c2ebc4099454f206ed14caaaa70c4cf260",
        },
        "handler" = null,
        "input_row" = [
          "op.trace",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = null,
        "row_open" = true,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Trace",
            "operation_id" = "op.trace",
            "parameter_schema" = "string",
            "result_schema" = "unit",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.row_summary_vocabulary.open_row_not_pure]",
      ],
    },
    {
      "artifact_class" = "algebraic-effect",
      "checked_laws" = [
        "deterministic-effect-classified",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/algebraic.positive.deterministic-effect-summary.receipt.json",
      "id" = "algebraic.positive.deterministic-effect-summary",
      "non_claims" = [
        "deterministic replay class is receipt data only",
      ],
      "payload" = {
        "admitted_host_capabilities" = [],
        "canonical_path" = "fixtures/core/algebraic-effects/deterministic.preserves",
        "claimed_host_authority" = [],
        "claimed_pure" = false,
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "algebraic-effect",
          "hex" = "c46884c2df056e4c7143fc6e2761abe7867807a5fc29fe233e6ad1eec811fd84",
        },
        "handler" = null,
        "input_row" = [
          "op.trace",
        ],
        "kind" = "algebraic-effect",
        "receipt" = null,
        "replay" = {
          "effect_log_identity" = null,
          "effect_log_path" = null,
          "replay_class" = "deterministic",
        },
        "row_open" = false,
        "schema_version" = 1,
        "signatures" = [
          {
            "display_name" = "Trace",
            "operation_id" = "op.trace",
            "parameter_schema" = "string",
            "result_schema" = "unit",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.algebraic_effects.replay_profile]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "source-receipt-identity-bound",
        "artifact-identity-preservation",
        "non-claim-preservation",
        "checked-law-preservation",
        "source-span-absence-explicit",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/valence-export.positive.source-envelope-absent-spans.receipt.json",
      "id" = "valence-export.positive.source-envelope-absent-spans",
      "non_claims" = [
        "Valence export is derived projection only",
        "not canonical Kamacite IR",
        "not runtime correctness",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.profile]",
        "r[kamacite.valence_receipt_export.profile.bounded_projection]",
        "r[kamacite.valence_receipt_export.profile.export_identity]",
        "r[kamacite.valence_receipt_export.profile.source_spans]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "source-span-preservation",
        "non-claim-preservation",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/valence-export.positive.source-envelope-present-spans.receipt.json",
      "id" = "valence-export.positive.source-envelope-present-spans",
      "non_claims" = [
        "source spans are optional evidence only",
        "not fabricated from paths or labels",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json",
        "source_spans_path" = "fixtures/valence-export/source-spans-present.json",
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.profile.source_spans]",
        "r[kamacite.valence_receipt_export.profile.bounded_projection]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "artifacts-first-no-valence-runtime",
        "adapter-identity-preserved",
        "non-claim-preservation",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/valence-export.positive.nickel-adapter-artifacts-first.receipt.json",
      "id" = "valence-export.positive.nickel-adapter-artifacts-first",
      "non_claims" = [
        "artifacts-first export does not require live Valence crates",
        "not Nickel semantic-equivalence proof",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/golden/adapter.positive.nickel-source-export-separated.receipt.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.coupling_boundary.artifacts_first]",
        "r[kamacite.valence_receipt_export.conformance]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "identity-domain-substitution-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/valence-export.negative.identity-domain-substitution.receipt.json",
      "id" = "valence-export.negative.identity-domain-substitution",
      "non_claims" = [
        "receipt identity cannot be substituted with source identity",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/valence-export/source-receipt-identity-domain-substitution.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.conformance.identity_domains]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "missing-non-claims-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/valence-export.negative.missing-non-claims.receipt.json",
      "id" = "valence-export.negative.missing-non-claims",
      "non_claims" = [
        "missing source non-claims fail closed",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/valence-export/source-receipt-missing-non-claims.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.conformance.missing_non_claims]",
        "r[kamacite.valence_receipt_export.profile.non_claims]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "semantic-overclaim-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/valence-export.negative.semantic-equivalence-overclaim.receipt.json",
      "id" = "valence-export.negative.semantic-equivalence-overclaim",
      "non_claims" = [
        "Valence export does not prove semantic equivalence",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [
          "Valence proves Nickel/Rust/Steel semantic equivalence",
        ],
        "source_receipt_path" = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.conformance.semantic_overclaim]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "unsupported-profile-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/valence-export.negative.unsupported-profile-version.receipt.json",
      "id" = "valence-export.negative.unsupported-profile-version",
      "non_claims" = [
        "unsupported export profile versions are not trusted",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance-unsupported-profile.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.conformance.malformed_input]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "duplicate-artifact-identity-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/valence-export.negative.duplicate-artifact-identity.receipt.json",
      "id" = "valence-export.negative.duplicate-artifact-identity",
      "non_claims" = [
        "duplicate artifact identity keys fail closed",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/valence-export/source-receipt-duplicate-artifact-key.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.conformance.malformed_input]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "missing-checked-laws-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/valence-export.negative.missing-checked-laws.receipt.json",
      "id" = "valence-export.negative.missing-checked-laws",
      "non_claims" = [
        "exports without checked laws fail closed",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/valence-export/source-receipt-missing-checked-laws.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.conformance.malformed_input]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "stale-export-identity-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/valence-export.negative.stale-export-identity.receipt.json",
      "id" = "valence-export.negative.stale-export-identity",
      "non_claims" = [
        "stale export identities fail closed",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance.json",
        "declared_export_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "0000000000000000000000000000000000000000000000000000000000000000",
        },
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.profile.export_identity]",
        "r[kamacite.valence_receipt_export.command.fail_closed]",
      ],
    },
    {
      "artifact_class" = "valence-receipt-export",
      "checked_laws" = [
        "conformance-entry-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/valence-export.negative.untrusted-without-conformance.receipt.json",
      "id" = "valence-export.negative.untrusted-without-conformance",
      "non_claims" = [
        "Valence-like exports without conformance are untrusted",
      ],
      "payload" = {
        "conformance_path" = "fixtures/valence-export/conformance-untrusted.json",
        "declared_export_identity" = null,
        "kind" = "valence-receipt-export",
        "semantic_claims" = [],
        "source_receipt_path" = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json",
        "source_spans_path" = null,
      },
      "requirement_ids" = [
        "r[kamacite.valence_receipt_export.conformance.entry_required]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "total-pair-coverage",
        "declared-guard-use",
        "denial-preserves-state",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-spec.positive.complete-denial-coverage.receipt.json",
      "id" = "fsm-spec.positive.complete-denial-coverage",
      "non_claims" = [
        "not runtime correctness",
        "not protocol completeness",
        "not liveness proof",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/complete-spec.preserves",
        "checked_laws" = [
          "total-pair-coverage",
          "declared-guard-use",
          "denial-preserves-state",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle",
        "guards" = [
          {
            "guard_id" = "has-authority",
            "non_claims" = [
              "guard evidence is not host authority",
            ],
            "required_evidence_refs" = [
              "authority-ref",
            ],
          },
        ],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "not runtime correctness",
          "not protocol completeness",
          "not liveness proof",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [
              "has-authority",
            ],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.fsm_spec_vocabulary.complete]",
        "r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage.complete]",
        "r[kamacite.fsms.completeness_matrix.complete_table]",
        "r[kamacite.fsms.completeness_matrix.explicit_denial]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "missing-pair-coverage-fails-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.missing-denial-coverage.receipt.json",
      "id" = "fsm-spec.negative.missing-denial-coverage",
      "non_claims" = [
        "missing FSM cells are not trusted",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/missing-coverage.preserves",
        "checked_laws" = [
          "missing-pair-coverage-fails-closed",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "60be2274325d9cd8b8b72e95a0be0acb645f25ef75c069e363695628a1b5a62e",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.missing",
        "guards" = [
          {
            "guard_id" = "has-authority",
            "non_claims" = [
              "guard evidence is not host authority",
            ],
            "required_evidence_refs" = [
              "authority-ref",
            ],
          },
        ],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "missing FSM cells are not trusted",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [
              "has-authority",
            ],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage.missing_denial]",
        "r[kamacite.fsms.fixture_corpus.negative_missing_cell]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "duplicate-transition-fails-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.duplicate-transition.receipt.json",
      "id" = "fsm-spec.negative.duplicate-transition",
      "non_claims" = [
        "duplicate transition rows are not deterministic evidence",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/duplicate-transition.preserves",
        "checked_laws" = [
          "duplicate-transition-fails-closed",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "55853abcdecb5d45b63e948e3ebb52d825e1170c0d7687c44dc06ae3ee31ac3c",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.duplicate",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "duplicate transition rows are not deterministic evidence",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage.duplicate]",
        "r[kamacite.fsms.fixture_corpus.negative_conflict]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "wildcards-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.wildcard-row.receipt.json",
      "id" = "fsm-spec.negative.wildcard-row",
      "non_claims" = [
        "wildcard FSM rows are not complete coverage",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/wildcard-row.preserves",
        "checked_laws" = [
          "wildcards-rejected",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "388b4dcc34367b1b6b3e2eeeeca72c1b72f88d03e1931ab36c3bb60ee3af9257",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.wildcard",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "wildcard FSM rows are not complete coverage",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "*",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.fsm_spec_vocabulary.no_wildcards.reject]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "declared-guard-use",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.undeclared-guard.receipt.json",
      "id" = "fsm-spec.negative.undeclared-guard",
      "non_claims" = [
        "undeclared guards cannot partition coverage",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/undeclared-guard.preserves",
        "checked_laws" = [
          "declared-guard-use",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "14cec0ef687b7666d0a8878694c59b6bb2dd003ba734d3d8fa21f954e4dfae57",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.undeclared-guard",
        "guards" = [
          {
            "guard_id" = "has-authority",
            "non_claims" = [
              "guard evidence is not host authority",
            ],
            "required_evidence_refs" = [
              "authority-ref",
            ],
          },
        ],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "undeclared guards cannot partition coverage",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [
              "missing-guard",
            ],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.fsm_spec_vocabulary.guards_side_effects.undeclared_guard]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "closed-universe-references",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.unknown-endpoint.receipt.json",
      "id" = "fsm-spec.negative.unknown-endpoint",
      "non_claims" = [
        "unknown state or event references are not trusted",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/unknown-endpoint.preserves",
        "checked_laws" = [
          "closed-universe-references",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "0a0922fcfa7df0903b79f901cb658ceb933ba6726dd48913bf4445db73d60551",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.unknown-endpoint",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "unknown state or event references are not trusted",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "missing",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.fsm_spec_vocabulary.closed_universes.references]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "fsm-identity-matches-canonical-bytes",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.stale-identity.receipt.json",
      "id" = "fsm-spec.negative.stale-identity",
      "non_claims" = [
        "stale FSM identities are not trusted",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/stale-identity.preserves",
        "checked_laws" = [
          "fsm-identity-matches-canonical-bytes",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "0000000000000000000000000000000000000000000000000000000000000000",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.stale",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "stale FSM identities are not trusted",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.fsms.fixture_corpus.negative_stale_identity]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "typed-fsm-identity-domain",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.wrong-identity-domain.receipt.json",
      "id" = "fsm-spec.negative.wrong-identity-domain",
      "non_claims" = [
        "FSM identity domains are not interchangeable",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/wrong-domain.preserves",
        "checked_laws" = [
          "typed-fsm-identity-domain",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "canonical-value",
          "hex" = "41fef4fd656563b1ec9aa70a779350d700e59198adee8c5051d4a7c9e0878e03",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.wrong-domain",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "FSM identity domains are not interchangeable",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.fsm_receipt_vocabulary.identity_domain]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "fsm-non-claims-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.missing-non-claims.receipt.json",
      "id" = "fsm-spec.negative.missing-non-claims",
      "non_claims" = [
        "payload-level non-claims are required",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/missing-non-claims.preserves",
        "checked_laws" = [
          "fsm-non-claims-required",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "a70af2fd927a5e4dc43982a4c1265bf47338790f24ed055c98e375e2bbf52833",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.missing-non-claims",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.lossless_ir.fsm_spec_vocabulary.complete]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "actions-do-not-grant-authority",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.host-authority-confusion.receipt.json",
      "id" = "fsm-spec.negative.host-authority-confusion",
      "non_claims" = [
        "transition effects are symbolic data only",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/host-confusion.preserves",
        "checked_laws" = [
          "actions-do-not-grant-authority",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "d99b6defb9b58122294f2fb209d8b81f066b140cf4ea879377873f73713aede7",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.host-confusion",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "transition effects are symbolic data only",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "filesystem",
            "effect_id" = "filesystem-effect",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "filesystem-effect",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "filesystem-effect",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.fsms.fixture_corpus.negative_host_confusion]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "terminal-outgoing-fails-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.terminal-outgoing.receipt.json",
      "id" = "fsm-spec.negative.terminal-outgoing",
      "non_claims" = [
        "terminal transitions need explicit accepted restart policy",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/terminal-outgoing.preserves",
        "checked_laws" = [
          "terminal-outgoing-fails-closed",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "a4670fc9a36f6233d726526fbb176c1782f6c3c83a006593526730f7c7198149",
        },
        "denials" = [
          {
            "event" = "reset",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "not-done",
            "side_effect" = "none",
          },
          {
            "event" = "finish",
            "from_state" = "done",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-done",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "finish",
            "label" = "Finish",
          },
          {
            "event_id" = "reset",
            "label" = "Reset",
          },
        ],
        "fsm_id" = "fixture.lifecycle.terminal-outgoing",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "terminal transitions need explicit accepted restart policy",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Done",
            "state_id" = "done",
          },
        ],
        "terminal_states" = [
          "done",
        ],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "finish",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "done",
          },
          {
            "event" = "reset",
            "from_state" = "done",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.fsms.reachability_terminal_policy.terminal_outgoing]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "unreachable-state-fails-closed",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-spec.negative.unreachable-unmarked.receipt.json",
      "id" = "fsm-spec.negative.unreachable-unmarked",
      "non_claims" = [
        "unreachable states require reviewed metadata",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/unreachable-unmarked.preserves",
        "checked_laws" = [
          "unreachable-state-fails-closed",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "e3c5990137e705ffc9fd3882e584278817c77940463be94bbf8f3bc9a1ed79c3",
        },
        "denials" = [
          {
            "event" = "stop",
            "from_state" = "idle",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-idle",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "active",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "already-active",
            "side_effect" = "none",
          },
          {
            "event" = "start",
            "from_state" = "orphan",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "unreachable",
            "side_effect" = "none",
          },
          {
            "event" = "stop",
            "from_state" = "orphan",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "unreachable",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "start",
            "label" = "Start",
          },
          {
            "event_id" = "stop",
            "label" = "Stop",
          },
        ],
        "fsm_id" = "fixture.lifecycle.unreachable",
        "guards" = [],
        "initial_state" = "idle",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "unreachable states require reviewed metadata",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "admit",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Idle",
            "state_id" = "idle",
          },
          {
            "label" = "Active",
            "state_id" = "active",
          },
          {
            "label" = "Orphan",
            "state_id" = "orphan",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "active",
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guards" = [],
            "side_effect" = "admit",
            "to_state" = "idle",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.fsms.reachability_terminal_policy.unreachable_unmarked]",
      ],
    },
    {
      "artifact_class" = "fsm-receipt",
      "checked_laws" = [
        "total-pair-coverage",
        "non-claim-preservation",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-receipt.positive.exhaustiveness-octet-valence.receipt.json",
      "id" = "fsm-receipt.positive.exhaustiveness-octet-valence",
      "non_claims" = [
        "Octet evidence is source-shape only",
        "Valence export is derived evidence",
        "not runtime correctness",
      ],
      "payload" = {
        "adapter_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "8ebb62f28593ba13c5d38d0c63810944b242da57d3ee0f2c81c1fde0747aff0e",
        },
        "canonical_path" = "fixtures/core/fsm/receipt-exhaustiveness.preserves",
        "descriptor_fixture_id" = "fsm-spec.positive.complete-denial-coverage",
        "checked_laws" = [
          "total-pair-coverage",
          "non-claim-preservation",
        ],
        "declared_receipt_identity" = {
          "algorithm" = "blake3",
          "domain" = "receipt",
          "hex" = "6426edec5ef7282b545a65fad4abf4c1fbe90d1e734ac878467b2ec420491673",
        },
        "denial_coverage_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-coverage",
          "hex" = "3c469a84fb15ed4177391b809ef242b29a4f14790de0230848213af16b40e79f",
        },
        "dependency_identities" = [],
        "diagnostics" = [],
        "event_universe_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-universe",
          "hex" = "da69e68d038be61ac54cdd5df8a1145281abf3ac9d73b18498e496fcbc9ffe28",
        },
        "kind" = "fsm-receipt",
        "non_claims" = [
          "Octet evidence is source-shape only",
          "Valence export is derived evidence",
          "not runtime correctness",
        ],
        "octet_receipt_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "receipt",
            "hex" = "c07707d770698c323a37eb79283fbb9ca2033aee7b0ada926cb368a0f34edba3",
          },
        ],
        "receipt_kind" = "exhaustiveness",
        "schema_version" = 1,
        "semantic_claims" = [],
        "spec_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
        },
        "state_universe_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-universe",
          "hex" = "1eea718435e25dbd3dea52ec1bcd77dd159a752e54225f8bf09e93a5a799b15b",
        },
        "trace_events" = [],
        "trace_final_state" = null,
        "trace_start_state" = null,
        "trace_steps" = [],
        "transition_coverage_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-coverage",
          "hex" = "ed6b7e12821d15e4b9f4544e5ed98e20abfbcca92ee53c2eea7a772a02799026",
        },
        "valence_export_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "cd7e6ade061bf11ba4fb7be2dbce5a7106e5f9f5318a82b9198b93c8672af8a7",
        },
      },
      "requirement_ids" = [
        "r[kamacite.findings.fsm_receipts.coverage_recomputation]",
        "r[kamacite.findings.fsm_receipts.descriptor_binding]",
        "r[kamacite.lossless_ir.fsm_receipt_vocabulary.checked_claim]",
        "r[kamacite.stack_integration.fsm_valence_export.projection]",
        "r[kamacite.stack_integration.fsm_octet_interop.source_shape]",
      ],
    },
    {
      "artifact_class" = "fsm-receipt",
      "checked_laws" = [
        "semantic-overclaim-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-receipt.negative.semantic-overclaim.receipt.json",
      "id" = "fsm-receipt.negative.semantic-overclaim",
      "non_claims" = [
        "FSM receipts do not prove protocol completeness or liveness",
      ],
      "payload" = {
        "adapter_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "8ebb62f28593ba13c5d38d0c63810944b242da57d3ee0f2c81c1fde0747aff0e",
        },
        "canonical_path" = "fixtures/core/fsm/receipt-overclaim.preserves",
        "descriptor_fixture_id" = "fsm-spec.positive.complete-denial-coverage",
        "checked_laws" = [
          "semantic-overclaim-rejected",
        ],
        "declared_receipt_identity" = {
          "algorithm" = "blake3",
          "domain" = "receipt",
          "hex" = "dccef8617fe3b70773f6a77904537b7e526ed86304ef19c290628969a925757d",
        },
        "denial_coverage_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-coverage",
          "hex" = "3c469a84fb15ed4177391b809ef242b29a4f14790de0230848213af16b40e79f",
        },
        "dependency_identities" = [],
        "diagnostics" = [],
        "event_universe_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-universe",
          "hex" = "da69e68d038be61ac54cdd5df8a1145281abf3ac9d73b18498e496fcbc9ffe28",
        },
        "kind" = "fsm-receipt",
        "non_claims" = [
          "FSM receipts do not prove protocol completeness or liveness",
        ],
        "octet_receipt_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "receipt",
            "hex" = "c07707d770698c323a37eb79283fbb9ca2033aee7b0ada926cb368a0f34edba3",
          },
        ],
        "receipt_kind" = "exhaustiveness",
        "schema_version" = 1,
        "semantic_claims" = [
          "proves protocol completeness and liveness",
        ],
        "spec_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
        },
        "state_universe_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-universe",
          "hex" = "1eea718435e25dbd3dea52ec1bcd77dd159a752e54225f8bf09e93a5a799b15b",
        },
        "trace_events" = [],
        "trace_final_state" = null,
        "trace_start_state" = null,
        "trace_steps" = [],
        "transition_coverage_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-coverage",
          "hex" = "ed6b7e12821d15e4b9f4544e5ed98e20abfbcca92ee53c2eea7a772a02799026",
        },
        "valence_export_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "cd7e6ade061bf11ba4fb7be2dbce5a7106e5f9f5318a82b9198b93c8672af8a7",
        },
      },
      "requirement_ids" = [
        "r[kamacite.findings.fsm_receipts.coverage_recomputation]",
        "r[kamacite.findings.fsm_receipts.descriptor_binding]",
        "r[kamacite.findings.fsm_receipts.negative_coverage]",
        "r[kamacite.stack_integration.fsm_valence_export.non_claims]",
        "r[kamacite.stack_integration.fsm_octet_interop.overclaim]",
      ],
    },
    {
      "artifact_class" = "fsm-receipt",
      "checked_laws" = [
        "trace-path-consistency",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-receipt.positive.trace-path.receipt.json",
      "id" = "fsm-receipt.positive.trace-path",
      "non_claims" = [
        "trace consistency is not runtime correctness",
      ],
      "payload" = {
        "adapter_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "8ebb62f28593ba13c5d38d0c63810944b242da57d3ee0f2c81c1fde0747aff0e",
        },
        "canonical_path" = "fixtures/core/fsm/trace-valid.preserves",
        "descriptor_fixture_id" = "fsm-spec.positive.complete-denial-coverage",
        "checked_laws" = [
          "trace-path-consistency",
        ],
        "declared_receipt_identity" = {
          "algorithm" = "blake3",
          "domain" = "receipt",
          "hex" = "06bfbdedac1c0a8a73ab9faef13196f81fc1e647b3b2f8ea244f6966ff550ead",
        },
        "denial_coverage_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-coverage",
          "hex" = "3c469a84fb15ed4177391b809ef242b29a4f14790de0230848213af16b40e79f",
        },
        "dependency_identities" = [],
        "diagnostics" = [],
        "event_universe_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-universe",
          "hex" = "da69e68d038be61ac54cdd5df8a1145281abf3ac9d73b18498e496fcbc9ffe28",
        },
        "kind" = "fsm-receipt",
        "non_claims" = [
          "trace consistency is not runtime correctness",
        ],
        "octet_receipt_identities" = [],
        "receipt_kind" = "trace",
        "schema_version" = 1,
        "semantic_claims" = [],
        "spec_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
        },
        "state_universe_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-universe",
          "hex" = "1eea718435e25dbd3dea52ec1bcd77dd159a752e54225f8bf09e93a5a799b15b",
        },
        "trace_events" = [
          "start",
          "stop",
        ],
        "trace_final_state" = "idle",
        "trace_start_state" = "idle",
        "trace_steps" = [
          {
            "event" = "start",
            "from_state" = "idle",
            "guard_decisions" = [
              {
                "allowed" = true,
                "evidence_refs" = ["authority-ref"],
                "guard_id" = "has-authority",
              },
            ],
            "to_state" = "active",
            "transition_identity" = {
              "algorithm" = "blake3",
              "domain" = "fsm-coverage",
              "hex" = "d747e3e376fb36e70b5459b71eaf49b8bb56c026e2ad3e9fdd6c0fa111822a1a",
            },
          },
          {
            "event" = "stop",
            "from_state" = "active",
            "guard_decisions" = [],
            "to_state" = "idle",
            "transition_identity" = {
              "algorithm" = "blake3",
              "domain" = "fsm-coverage",
              "hex" = "b800c02bdacc0928118852dadf8b1667e6ac2277a9e144671b0c612fdbfdfe62",
            },
          },
        ],
        "transition_coverage_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-coverage",
          "hex" = "ed6b7e12821d15e4b9f4544e5ed98e20abfbcca92ee53c2eea7a772a02799026",
        },
        "valence_export_identity" = null,
      },
      "requirement_ids" = [
        "r[kamacite.findings.fsm_receipts.descriptor_binding]",
        "r[kamacite.findings.fsm_receipts.trace_consistency]",
        "r[kamacite.fsms.trace_receipts.valid_path]",
      ],
    },
    {
      "artifact_class" = "fsm-receipt",
      "checked_laws" = [
        "impossible-trace-rejected",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/fsm-receipt.negative.impossible-trace.receipt.json",
      "id" = "fsm-receipt.negative.impossible-trace",
      "non_claims" = [
        "impossible trace paths are not trusted evidence",
      ],
      "payload" = {
        "adapter_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "8ebb62f28593ba13c5d38d0c63810944b242da57d3ee0f2c81c1fde0747aff0e",
        },
        "canonical_path" = "fixtures/core/fsm/trace-impossible.preserves",
        "descriptor_fixture_id" = "fsm-spec.positive.complete-denial-coverage",
        "checked_laws" = [
          "impossible-trace-rejected",
        ],
        "declared_receipt_identity" = {
          "algorithm" = "blake3",
          "domain" = "receipt",
          "hex" = "e8d50aa68423f9cf2f291b9b3e1e2b8ad2b0f9478485a297fa12df5b9eb4929b",
        },
        "denial_coverage_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-coverage",
          "hex" = "3c469a84fb15ed4177391b809ef242b29a4f14790de0230848213af16b40e79f",
        },
        "dependency_identities" = [],
        "diagnostics" = [],
        "event_universe_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-universe",
          "hex" = "da69e68d038be61ac54cdd5df8a1145281abf3ac9d73b18498e496fcbc9ffe28",
        },
        "kind" = "fsm-receipt",
        "non_claims" = [
          "impossible trace paths are not trusted evidence",
        ],
        "octet_receipt_identities" = [],
        "receipt_kind" = "trace",
        "schema_version" = 1,
        "semantic_claims" = [],
        "spec_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
        },
        "state_universe_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-universe",
          "hex" = "1eea718435e25dbd3dea52ec1bcd77dd159a752e54225f8bf09e93a5a799b15b",
        },
        "trace_events" = [
          "stop",
        ],
        "trace_final_state" = "idle",
        "trace_start_state" = "idle",
        "trace_steps" = [
          {
            "event" = "stop",
            "from_state" = "active",
            "guard_decisions" = [],
            "to_state" = "idle",
            "transition_identity" = {
              "algorithm" = "blake3",
              "domain" = "fsm-coverage",
              "hex" = "b800c02bdacc0928118852dadf8b1667e6ac2277a9e144671b0c612fdbfdfe62",
            },
          },
        ],
        "transition_coverage_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-coverage",
          "hex" = "ed6b7e12821d15e4b9f4544e5ed98e20abfbcca92ee53c2eea7a772a02799026",
        },
        "valence_export_identity" = null,
      },
      "requirement_ids" = [
        "r[kamacite.findings.fsm_receipts.descriptor_binding]",
        "r[kamacite.findings.fsm_receipts.negative_coverage]",
        "r[kamacite.findings.fsm_receipts.trace_consistency]",
        "r[kamacite.fsms.trace_receipts.impossible_transition]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "validation-workflow-states-explicit",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-spec.positive.validation-workflow.receipt.json",
      "id" = "fsm-spec.positive.validation-workflow",
      "non_claims" = [
        "validation workflow fixture is not CLI branch coverage proof",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/validation-workflow.preserves",
        "checked_laws" = [
          "validation-workflow-states-explicit",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "addd54ceaaf389a2d916bf0f9771689c8950feec3c6cb5fc46d76f9cd89a8485",
        },
        "denials" = [
          {
            "event" = "fixture-pass",
            "from_state" = "discover",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "fixtures-not-loaded",
            "side_effect" = "none",
          },
          {
            "event" = "manifest-ok",
            "from_state" = "validate",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "manifest-already-checked",
            "side_effect" = "none",
          },
          {
            "event" = "manifest-ok",
            "from_state" = "reported",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "terminal-report",
            "side_effect" = "none",
          },
          {
            "event" = "fixture-pass",
            "from_state" = "reported",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "terminal-report",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "manifest-ok",
            "label" = "Manifest fresh",
          },
          {
            "event_id" = "fixture-pass",
            "label" = "Fixture pass",
          },
        ],
        "fsm_id" = "fixture.validation-workflow",
        "guards" = [],
        "initial_state" = "discover",
        "intentionally_unreachable_states" = [],
        "kind" = "fsm-spec",
        "non_claims" = [
          "validation workflow fixture is not CLI branch coverage proof",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "shell-report",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Discover inputs",
            "state_id" = "discover",
          },
          {
            "label" = "Validate pure fixtures",
            "state_id" = "validate",
          },
          {
            "label" = "Report verdict",
            "state_id" = "reported",
          },
        ],
        "terminal_states" = [
          "reported",
        ],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "manifest-ok",
            "from_state" = "discover",
            "guards" = [],
            "side_effect" = "none",
            "to_state" = "validate",
          },
          {
            "event" = "fixture-pass",
            "from_state" = "validate",
            "guards" = [],
            "side_effect" = "shell-report",
            "to_state" = "reported",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.fsms.command_adapter_usage.validation_workflow]",
      ],
    },
    {
      "artifact_class" = "fsm-spec",
      "checked_laws" = [
        "adapter-mode-gates-explicit",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-spec.positive.adapter-modes.receipt.json",
      "id" = "fsm-spec.positive.adapter-modes",
      "non_claims" = [
        "adapter-mode fixture does not grant evaluator or host authority",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/fsm/adapter-modes.preserves",
        "checked_laws" = [
          "adapter-mode-gates-explicit",
        ],
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "fsm-spec",
          "hex" = "f0c1a11e162e041db2161ad3b1048d9ff24b61e65c8ac524314c3efbb2f55d15",
        },
        "denials" = [
          {
            "event" = "undeclared-effect",
            "from_state" = "stored",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "undeclared-effect",
            "side_effect" = "none",
          },
          {
            "event" = "undeclared-effect",
            "from_state" = "execution",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "undeclared-effect",
            "side_effect" = "none",
          },
          {
            "event" = "use-stored",
            "from_state" = "denied",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "denied-state",
            "side_effect" = "none",
          },
          {
            "event" = "request-execution",
            "from_state" = "denied",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "denied-state",
            "side_effect" = "none",
          },
          {
            "event" = "undeclared-effect",
            "from_state" = "denied",
            "guards" = [],
            "preserves_state" = true,
            "reason" = "denied-state",
            "side_effect" = "none",
          },
        ],
        "determinism_policy" = "deterministic",
        "events" = [
          {
            "event_id" = "use-stored",
            "label" = "Use stored evidence",
          },
          {
            "event_id" = "request-execution",
            "label" = "Request explicit execution",
          },
          {
            "event_id" = "undeclared-effect",
            "label" = "Undeclared effect",
          },
        ],
        "fsm_id" = "fixture.adapter-modes",
        "guards" = [
          {
            "guard_id" = "capability-admitted",
            "non_claims" = [
              "capability evidence is not ambient authority",
            ],
            "required_evidence_refs" = [
              "adapter-capability",
            ],
          },
        ],
        "initial_state" = "stored",
        "intentionally_unreachable_states" = [
          "denied",
        ],
        "kind" = "fsm-spec",
        "non_claims" = [
          "adapter-mode fixture does not grant evaluator or host authority",
        ],
        "schema_version" = 1,
        "side_effects" = [
          {
            "class" = "no-side-effect",
            "effect_id" = "none",
          },
          {
            "class" = "symbolic",
            "effect_id" = "explicit-shell",
          },
        ],
        "source_envelope_identity" = null,
        "states" = [
          {
            "label" = "Stored evidence",
            "state_id" = "stored",
          },
          {
            "label" = "Execution admitted",
            "state_id" = "execution",
          },
          {
            "label" = "Denied",
            "state_id" = "denied",
          },
        ],
        "terminal_states" = [],
        "totality_policy" = "all-state-event-pairs-explicit",
        "transitions" = [
          {
            "event" = "use-stored",
            "from_state" = "stored",
            "guards" = [],
            "side_effect" = "none",
            "to_state" = "stored",
          },
          {
            "event" = "request-execution",
            "from_state" = "stored",
            "guards" = [
              "capability-admitted",
            ],
            "side_effect" = "explicit-shell",
            "to_state" = "execution",
          },
          {
            "event" = "use-stored",
            "from_state" = "execution",
            "guards" = [],
            "side_effect" = "none",
            "to_state" = "stored",
          },
          {
            "event" = "request-execution",
            "from_state" = "execution",
            "guards" = [
              "capability-admitted",
            ],
            "side_effect" = "explicit-shell",
            "to_state" = "execution",
          },
        ],
      },
      "requirement_ids" = [
        "r[kamacite.fsms.command_adapter_usage.adapter_modes]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "stack-role-boundary",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-stack.positive.trellis.receipt.json",
      "id" = "fsm-stack.positive.trellis",
      "non_claims" = [
        "Trellis proof reference is supporting evidence only",
      ],
      "payload" = {
        "bounded_claim" = "supporting FSM reachability proof reference only",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "fsm-spec",
            "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
          },
        ],
        "role" = "trellis-proof-reference",
      },
      "requirement_ids" = [
        "r[kamacite.fsms.stack_roles.trellis]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "stack-role-boundary",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-stack.positive.valence.receipt.json",
      "id" = "fsm-stack.positive.valence",
      "non_claims" = [
        "Valence wraps FSM evidence without broadening claims",
      ],
      "payload" = {
        "bounded_claim" = "derived evidence projection preserving Kamacite FSM receipt identity",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "fsm-spec",
            "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
          },
        ],
        "role" = "valence-evidence-export",
      },
      "requirement_ids" = [
        "r[kamacite.fsms.stack_roles.valence]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "stack-role-boundary",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-stack.positive.aspen-molten.receipt.json",
      "id" = "fsm-stack.positive.aspen-molten",
      "non_claims" = [
        "runtime traces do not own the FSM schema",
      ],
      "payload" = {
        "bounded_claim" = "runtime observation references a Kamacite FSM descriptor",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "fsm-spec",
            "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
          },
        ],
        "role" = "aspen-molten-runtime-trace",
      },
      "requirement_ids" = [
        "r[kamacite.fsms.stack_roles.aspen_molten]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "stack-role-boundary",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-stack.positive.octet.receipt.json",
      "id" = "fsm-stack.positive.octet",
      "non_claims" = [
        "Octet checks remain source-shape evidence",
      ],
      "payload" = {
        "bounded_claim" = "Rust source-shape and provenance evidence only",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "fsm-spec",
            "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
          },
        ],
        "role" = "octet-source-shape",
      },
      "requirement_ids" = [
        "r[kamacite.fsms.stack_roles.octet]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "stack-role-boundary",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-stack.positive.cairn.receipt.json",
      "id" = "fsm-stack.positive.cairn",
      "non_claims" = [
        "Cairn lifecycle gates do not override Kamacite FSM identity",
      ],
      "payload" = {
        "bounded_claim" = "lifecycle machine evidence consumes Kamacite FSM descriptors",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "fsm-spec",
            "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
          },
        ],
        "role" = "cairn-lifecycle-consumer",
      },
      "requirement_ids" = [
        "r[kamacite.fsms.stack_roles.cairn]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "stack-role-boundary",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/fsm-stack.positive.mantle.receipt.json",
      "id" = "fsm-stack.positive.mantle",
      "non_claims" = [
        "Mantle release evidence does not imply source losslessness or evaluator soundness",
      ],
      "payload" = {
        "bounded_claim" = "release evidence references Kamacite FSM identities",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "fsm-spec",
            "hex" = "f7e6b8cafb915755fedc8cd1f1d267142ee35d309ba76128e1ee15c6ee0a0bf0",
          },
        ],
        "role" = "mantle-release-evidence",
      },
      "requirement_ids" = [
        "r[kamacite.fsms.stack_roles.mantle]",
      ],
    },
    {
      "artifact_class" = "documentation-ir",
      "checked_laws" = [
        "source-envelopes-separate",
        "stable-anchors",
        "stable-links",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-ir.positive.valid-page.receipt.json",
      "id" = "docs-ir.positive.valid-page",
      "non_claims" = [
        "documentation IR is not rendered output authority",
        "runnable cells are not executed by default",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/docs/valid-ir.preserves",
        "checked_laws" = [
          "source-envelopes-separate",
          "stable-anchors",
          "stable-links",
        ],
        "collector_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "46cced372f18cf919a85f55eae0527b915a2c09e88769559f020e89d54536334",
        },
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "dependency_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "source",
            "hex" = "26de06259db3315ab4da33a959e28f2eacbddcc2373e234b55f8564a2110d422",
          },
        ],
        "kind" = "documentation-ir",
        "non_claims" = [
          "documentation IR is not rendered output authority",
          "runnable cells are not executed by default",
        ],
        "pages" = [
          {
            "cells" = [
              {
                "cell_id" = "cell.nickel.success",
                "evaluator_identity" = {
                  "algorithm" = "blake3",
                  "domain" = "evaluator",
                  "hex" = "880a6d48041b5291afb5aec2d885c4f94f93301d642734a53cd792aaf0486b95",
                },
                "expected_result" = "success",
                "language" = "nickel",
                "non_claims" = [
                  "not Nickel semantic equivalence",
                ],
                "required_capabilities" = [],
                "source_identity" = {
                  "algorithm" = "blake3",
                  "domain" = "source",
                  "hex" = "73bec48c589d0adf4b4de96d3b62ce7781de12a46760d2fd7a09b5ea56b9a17c",
                },
              },
              {
                "cell_id" = "cell.nickel.failure",
                "evaluator_identity" = {
                  "algorithm" = "blake3",
                  "domain" = "evaluator",
                  "hex" = "880a6d48041b5291afb5aec2d885c4f94f93301d642734a53cd792aaf0486b95",
                },
                "expected_result" = "failure",
                "language" = "nickel",
                "non_claims" = [
                  "not command crash",
                ],
                "required_capabilities" = [],
                "source_identity" = {
                  "algorithm" = "blake3",
                  "domain" = "source",
                  "hex" = "f34e5d1005a522d9e966e57551c22da39d15a498c771e4e86c86283a80741229",
                },
              },
            ],
            "page_id" = "getting-started",
            "sections" = [
              {
                "anchor" = "getting-started-intro",
                "code_blocks" = [],
                "links" = [
                  "getting-started-links",
                ],
                "prose_blocks" = [
                  "Kamacite docs are projections.",
                ],
                "runnable_cell_ids" = [
                  "cell.nickel.success",
                ],
                "section_id" = "intro",
                "title" = "Intro",
              },
              {
                "anchor" = "getting-started-links",
                "code_blocks" = [],
                "links" = [],
                "prose_blocks" = [
                  "Links resolve by explicit anchor.",
                ],
                "runnable_cell_ids" = [],
                "section_id" = "links",
                "title" = "Links",
              },
            ],
            "source_envelope_identity" = {
              "algorithm" = "blake3",
              "domain" = "source",
              "hex" = "26de06259db3315ab4da33a959e28f2eacbddcc2373e234b55f8564a2110d422",
            },
            "title" = "Getting Started",
          },
        ],
        "schema_version" = 1,
        "target_formats" = [
          "html",
          "markdown",
          "asciidoc",
        ],
      },
      "requirement_ids" = [
        "r[kamacite.docs.ir_projection]",
        "r[kamacite.docs.renderers.stable_links]",
        "r[kamacite.docs.renderers.stable_anchors]",
        "r[kamacite.docs.fixture_admission.positive_ir]",
      ],
    },
    {
      "artifact_class" = "documentation-output",
      "checked_laws" = [
        "deterministic-render",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-output.positive.html.receipt.json",
      "id" = "docs-output.positive.html",
      "non_claims" = [
        "HTML output is a projection only",
      ],
      "payload" = {
        "checked_laws" = [
          "deterministic-render",
        ],
        "claims_canonical_ir" = false,
        "declared_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "f780600a9f46fd254d00f192129716290ea0490860a3458c2aa14b8787593407",
        },
        "documentation_ir_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "kind" = "documentation-output",
        "non_claims" = [
          "HTML output is a projection only",
        ],
        "output_path" = "fixtures/core/docs/rendered.html",
        "renderer_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "1c68cc4b50751defc83e8ff7aa1ae6b6cff3d067fd8f27de63d37cbe6354d87a",
        },
        "schema_version" = 1,
        "target_format" = "html",
      },
      "requirement_ids" = [
        "r[kamacite.docs.renderers.html]",
        "r[kamacite.docs.fixture_admission.positive_renderers]",
      ],
    },
    {
      "artifact_class" = "documentation-output",
      "checked_laws" = [
        "deterministic-render",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-output.positive.markdown.receipt.json",
      "id" = "docs-output.positive.markdown",
      "non_claims" = [
        "Markdown output is a projection only",
      ],
      "payload" = {
        "checked_laws" = [
          "deterministic-render",
        ],
        "claims_canonical_ir" = false,
        "declared_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "47631b18255ce8007bf4983884627d42847fd37e495db6b167ad6c81a9207a68",
        },
        "documentation_ir_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "kind" = "documentation-output",
        "non_claims" = [
          "Markdown output is a projection only",
        ],
        "output_path" = "fixtures/core/docs/rendered.md",
        "renderer_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "1c68cc4b50751defc83e8ff7aa1ae6b6cff3d067fd8f27de63d37cbe6354d87a",
        },
        "schema_version" = 1,
        "target_format" = "markdown",
      },
      "requirement_ids" = [
        "r[kamacite.docs.renderers.markdown]",
        "r[kamacite.docs.fixture_admission.positive_renderers]",
      ],
    },
    {
      "artifact_class" = "documentation-output",
      "checked_laws" = [
        "deterministic-render",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-output.positive.asciidoc.receipt.json",
      "id" = "docs-output.positive.asciidoc",
      "non_claims" = [
        "AsciiDoc output is a projection only",
      ],
      "payload" = {
        "checked_laws" = [
          "deterministic-render",
        ],
        "claims_canonical_ir" = false,
        "declared_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "76da1c4846bf4ca25cf6dc3d11e4ee749206e57cfe89aa33ebcae1ddc90e23e6",
        },
        "documentation_ir_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "kind" = "documentation-output",
        "non_claims" = [
          "AsciiDoc output is a projection only",
        ],
        "output_path" = "fixtures/core/docs/rendered.adoc",
        "renderer_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "1c68cc4b50751defc83e8ff7aa1ae6b6cff3d067fd8f27de63d37cbe6354d87a",
        },
        "schema_version" = 1,
        "target_format" = "asciidoc",
      },
      "requirement_ids" = [
        "r[kamacite.docs.renderers.asciidoc]",
        "r[kamacite.docs.fixture_admission.positive_renderers]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "expected-output-bound",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-cell.positive.nickel-success.receipt.json",
      "id" = "docs-cell.positive.nickel-success",
      "non_claims" = [
        "not Nickel semantic equivalence",
      ],
      "payload" = {
        "cell_id" = "cell.nickel.success",
        "checked_laws" = [
          "expected-output-bound",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "880a6d48041b5291afb5aec2d885c4f94f93301d642734a53cd792aaf0486b95",
        },
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = null,
        "expected_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "a16072b1b0f8588ee724b9d9fef7771759b1a884dedd56b869049e07fee3559b",
        },
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "nickel",
        "non_claims" = [
          "not Nickel semantic equivalence",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "a16072b1b0f8588ee724b9d9fef7771759b1a884dedd56b869049e07fee3559b",
        },
        "observed_output_path" = "fixtures/core/docs/cell-output.txt",
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "73bec48c589d0adf4b4de96d3b62ce7781de12a46760d2fd7a09b5ea56b9a17c",
        },
        "source_path" = "fixtures/core/docs/cell-success.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.nickel]",
        "r[kamacite.docs.runnable_cells.expected_success]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "expected-diagnostic-bound",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-cell.positive.nickel-failure.receipt.json",
      "id" = "docs-cell.positive.nickel-failure",
      "non_claims" = [
        "expected failure is not command crash",
      ],
      "payload" = {
        "cell_id" = "cell.nickel.failure",
        "checked_laws" = [
          "expected-diagnostic-bound",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "880a6d48041b5291afb5aec2d885c4f94f93301d642734a53cd792aaf0486b95",
        },
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = "unbound-identifier",
        "expected_output_identity" = null,
        "expected_result" = "failure",
        "kind" = "documentation-cell",
        "language" = "nickel",
        "non_claims" = [
          "expected failure is not command crash",
        ],
        "observed_diagnostic" = "unbound-identifier",
        "observed_effects" = [],
        "observed_output_identity" = null,
        "observed_output_path" = null,
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "f34e5d1005a522d9e966e57551c22da39d15a498c771e4e86c86283a80741229",
        },
        "source_path" = "fixtures/core/docs/cell-failure.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.expected_failure]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "declared-runner",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-cell.positive.steel.receipt.json",
      "id" = "docs-cell.positive.steel",
      "non_claims" = [
        "not Steel bytecode or closure authority",
      ],
      "payload" = {
        "cell_id" = "cell.steel.success",
        "checked_laws" = [
          "declared-runner",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "ad60a1c09ee7623253acc9d20345515223300f90fca7446c2a270d0d56bbcecc",
        },
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = null,
        "expected_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "b576bf327e110a373629f7f0c0ae4aa92f13025760d0a5568271bf9f53174687",
        },
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "steel",
        "non_claims" = [
          "not Steel bytecode or closure authority",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "b576bf327e110a373629f7f0c0ae4aa92f13025760d0a5568271bf9f53174687",
        },
        "observed_output_path" = "fixtures/core/docs/cell-output-ok.txt",
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "cf1682d05dc769bd358a0c38ba1eac6ed1864ec4cc3d242724d22ed104ed2449",
        },
        "source_path" = "fixtures/core/docs/cell-steel.scm",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.steel]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "declared-runner",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-cell.positive.rust-validator.receipt.json",
      "id" = "docs-cell.positive.rust-validator",
      "non_claims" = [
        "not compiler correctness",
      ],
      "payload" = {
        "cell_id" = "cell.rust-validator.success",
        "checked_laws" = [
          "declared-runner",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "ad60a1c09ee7623253acc9d20345515223300f90fca7446c2a270d0d56bbcecc",
        },
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = null,
        "expected_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "3500d4212ad0374aec8f76d40868d53adbb8646d964a35bf43f5cbeb198a9a90",
        },
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "rust-validator",
        "non_claims" = [
          "not compiler correctness",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "3500d4212ad0374aec8f76d40868d53adbb8646d964a35bf43f5cbeb198a9a90",
        },
        "observed_output_path" = "fixtures/core/docs/cell-output-accepted.txt",
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "7f11c7e784ea7c74d5ab45b6436c68889b0bcf864d57a876234f610ad90e722f",
        },
        "source_path" = "fixtures/core/docs/cell-rust-validator.txt",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.rust_validator]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "canonical-bytes-bound",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-cell.positive.preserves.receipt.json",
      "id" = "docs-cell.positive.preserves",
      "non_claims" = [
        "not source-language semantics",
      ],
      "payload" = {
        "cell_id" = "cell.preserves.success",
        "checked_laws" = [
          "canonical-bytes-bound",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "ad60a1c09ee7623253acc9d20345515223300f90fca7446c2a270d0d56bbcecc",
        },
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = null,
        "expected_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "35c4845a6c2c0d33099c450ce1791e0fbce2cdf4d882ab929855e10b76980432",
        },
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "preserves",
        "non_claims" = [
          "not source-language semantics",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "35c4845a6c2c0d33099c450ce1791e0fbce2cdf4d882ab929855e10b76980432",
        },
        "observed_output_path" = "fixtures/core/docs/cell-output-preserves.txt",
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "35c4845a6c2c0d33099c450ce1791e0fbce2cdf4d882ab929855e10b76980432",
        },
        "source_path" = "fixtures/core/docs/cell-preserves.preserves",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.preserves]",
      ],
    },
    {
      "artifact_class" = "documentation-receipt",
      "checked_laws" = [
        "output-hashes-bound",
        "non-claims-preserved",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-receipt.positive.output-hashes.receipt.json",
      "id" = "docs-receipt.positive.output-hashes",
      "non_claims" = [
        "not canonical rendered output",
        "not semantic equivalence",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/docs/render-receipt.preserves",
        "cell_receipt_identities" = [],
        "checked_laws" = [
          "output-hashes-bound",
          "non-claims-preserved",
        ],
        "declared_receipt_identity" = {
          "algorithm" = "blake3",
          "domain" = "receipt",
          "hex" = "6cb5b41a2be6ce942b509f2408a569135090d1a6a75cb6a4003df4034c956e33",
        },
        "diagnostics" = [],
        "documentation_ir_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "kind" = "documentation-receipt",
        "non_claims" = [
          "not canonical rendered output",
          "not semantic equivalence",
        ],
        "output_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "documentation-output",
            "hex" = "f780600a9f46fd254d00f192129716290ea0490860a3458c2aa14b8787593407",
          },
          {
            "algorithm" = "blake3",
            "domain" = "documentation-output",
            "hex" = "47631b18255ce8007bf4983884627d42847fd37e495db6b167ad6c81a9207a68",
          },
          {
            "algorithm" = "blake3",
            "domain" = "documentation-output",
            "hex" = "76da1c4846bf4ca25cf6dc3d11e4ee749206e57cfe89aa33ebcae1ddc90e23e6",
          },
        ],
        "receipt_kind" = "render",
        "renderer_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "1c68cc4b50751defc83e8ff7aa1ae6b6cff3d067fd8f27de63d37cbe6354d87a",
        },
        "schema_version" = 1,
        "semantic_claims" = [
          "bounded documentation projection only",
        ],
        "valence_export_identity" = {
          "algorithm" = "blake3",
          "domain" = "projection",
          "hex" = "9bc92f71ece8543631e927dc3785916545cd76ccbb4bc079d81eafad6bcc613e",
        },
      },
      "requirement_ids" = [
        "r[kamacite.docs.receipts.output_hashes]",
        "r[kamacite.docs.stack_roles.valence]",
      ],
    },
    {
      "artifact_class" = "stack-evidence",
      "checked_laws" = [
        "stack-role-boundary",
      ],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/docs-stack.positive.valence.receipt.json",
      "id" = "docs-stack.positive.valence",
      "non_claims" = [
        "Valence evidence does not broaden documentation claims",
      ],
      "payload" = {
        "bounded_claim" = "wraps documentation and runnable-cell receipt identities only",
        "broadens_claim" = false,
        "kind" = "stack-evidence",
        "referenced_identities" = [
          {
            "algorithm" = "blake3",
            "domain" = "receipt",
            "hex" = "9bc92f71ece8543631e927dc3785916545cd76ccbb4bc079d81eafad6bcc613e",
          },
        ],
        "role" = "valence-documentation-export",
      },
      "requirement_ids" = [
        "r[kamacite.docs.stack_roles.valence]",
        "r[kamacite.docs.receipts.non_claims]",
      ],
    },
    {
      "artifact_class" = "documentation-ir",
      "checked_laws" = [
        "fail-closed-version",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-ir.negative.unsupported-version.receipt.json",
      "id" = "docs-ir.negative.unsupported-version",
      "non_claims" = [
        "not compatibility defaulting",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/docs/unsupported-version.preserves",
        "checked_laws" = [
          "fail-closed-version",
        ],
        "collector_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "46cced372f18cf919a85f55eae0527b915a2c09e88769559f020e89d54536334",
        },
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "32bd5b6dea69ce4eb16e4269f9f62165e4dae6f169dfc269714d1f43f5c10bb2",
        },
        "dependency_identities" = [],
        "kind" = "documentation-ir",
        "non_claims" = [
          "not compatibility defaulting",
        ],
        "pages" = [],
        "schema_version" = 2,
        "target_formats" = [
          "html",
        ],
      },
      "requirement_ids" = [
        "r[kamacite.docs.ir_projection.unsupported_version]",
      ],
    },
    {
      "artifact_class" = "documentation-ir",
      "checked_laws" = [
        "fail-closed-duplicate",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-ir.negative.duplicate-id.receipt.json",
      "id" = "docs-ir.negative.duplicate-id",
      "non_claims" = [
        "duplicate anchors not trusted",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/docs/duplicate-anchor.preserves",
        "checked_laws" = [
          "fail-closed-duplicate",
        ],
        "collector_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "46cced372f18cf919a85f55eae0527b915a2c09e88769559f020e89d54536334",
        },
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "716c45f94881f60de1541118978e357baeb8cd846ea75390b5d8af9072a55157",
        },
        "dependency_identities" = [],
        "kind" = "documentation-ir",
        "non_claims" = [
          "duplicate anchors not trusted",
        ],
        "pages" = [
          {
            "cells" = [],
            "page_id" = "duplicate",
            "sections" = [
              {
                "anchor" = "dup",
                "code_blocks" = [],
                "links" = [],
                "prose_blocks" = [],
                "runnable_cell_ids" = [],
                "section_id" = "first",
                "title" = "First",
              },
              {
                "anchor" = "dup",
                "code_blocks" = [],
                "links" = [],
                "prose_blocks" = [],
                "runnable_cell_ids" = [],
                "section_id" = "second",
                "title" = "Second",
              },
            ],
            "source_envelope_identity" = {
              "algorithm" = "blake3",
              "domain" = "source",
              "hex" = "26de06259db3315ab4da33a959e28f2eacbddcc2373e234b55f8564a2110d422",
            },
            "title" = "Duplicate",
          },
        ],
        "schema_version" = 1,
        "target_formats" = [
          "html",
        ],
      },
      "requirement_ids" = [
        "r[kamacite.docs.ir_projection.duplicate_id]",
      ],
    },
    {
      "artifact_class" = "documentation-ir",
      "checked_laws" = [
        "missing-source-fails",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-ir.negative.missing-source.receipt.json",
      "id" = "docs-ir.negative.missing-source",
      "non_claims" = [
        "not source preservation",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/docs/missing-source.preserves",
        "checked_laws" = [
          "missing-source-fails",
        ],
        "collector_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "46cced372f18cf919a85f55eae0527b915a2c09e88769559f020e89d54536334",
        },
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "21084143e88aca242bd0cfe23c1e0a84ab3fd5a94141c00bd25f6b6e24f277a2",
        },
        "dependency_identities" = [],
        "kind" = "documentation-ir",
        "non_claims" = [
          "not source preservation",
        ],
        "pages" = [
          {
            "cells" = [],
            "page_id" = "missing-source",
            "sections" = [],
            "source_envelope_identity" = null,
            "title" = "Missing",
          },
        ],
        "schema_version" = 1,
        "target_formats" = [
          "html",
        ],
      },
      "requirement_ids" = [
        "r[kamacite.docs.ir_projection.missing_source]",
      ],
    },
    {
      "artifact_class" = "documentation-ir",
      "checked_laws" = [
        "fail-closed-link",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-ir.negative.unknown-link.receipt.json",
      "id" = "docs-ir.negative.unknown-link",
      "non_claims" = [
        "unknown links not trusted",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/docs/unknown-link.preserves",
        "checked_laws" = [
          "fail-closed-link",
        ],
        "collector_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "46cced372f18cf919a85f55eae0527b915a2c09e88769559f020e89d54536334",
        },
        "declared_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "36252d442abc263c185f06fcb81604be8af19b029003a6df8868edd0b25ad94e",
        },
        "dependency_identities" = [],
        "kind" = "documentation-ir",
        "non_claims" = [
          "unknown links not trusted",
        ],
        "pages" = [
          {
            "cells" = [],
            "page_id" = "unknown-link",
            "sections" = [
              {
                "anchor" = "known",
                "code_blocks" = [],
                "links" = [
                  "missing-target",
                ],
                "prose_blocks" = [],
                "runnable_cell_ids" = [],
                "section_id" = "known",
                "title" = "Known",
              },
            ],
            "source_envelope_identity" = {
              "algorithm" = "blake3",
              "domain" = "source",
              "hex" = "26de06259db3315ab4da33a959e28f2eacbddcc2373e234b55f8564a2110d422",
            },
            "title" = "Unknown",
          },
        ],
        "schema_version" = 1,
        "target_formats" = [
          "html",
        ],
      },
      "requirement_ids" = [
        "r[kamacite.docs.renderers.unknown_link]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "stale-source-fails",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-cell.negative.stale-source-hash.receipt.json",
      "id" = "docs-cell.negative.stale-source-hash",
      "non_claims" = [
        "stale source not trusted",
      ],
      "payload" = {
        "cell_id" = "cell.nickel.stale",
        "checked_laws" = [
          "stale-source-fails",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = null,
        "execution_mode" = "stored-evidence",
        "expected_diagnostic" = null,
        "expected_output_identity" = null,
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "nickel",
        "non_claims" = [
          "stale source not trusted",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = null,
        "observed_output_path" = null,
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "f34e5d1005a522d9e966e57551c22da39d15a498c771e4e86c86283a80741229",
        },
        "source_path" = "fixtures/core/docs/cell-success.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.docs.ir_projection.stale_source_hash]",
      ],
    },
    {
      "artifact_class" = "documentation-output",
      "checked_laws" = [
        "fail-closed-format",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-output.negative.unsupported-format.receipt.json",
      "id" = "docs-output.negative.unsupported-format",
      "non_claims" = [
        "unsupported format not trusted",
      ],
      "payload" = {
        "checked_laws" = [
          "fail-closed-format",
        ],
        "claims_canonical_ir" = false,
        "declared_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "f780600a9f46fd254d00f192129716290ea0490860a3458c2aa14b8787593407",
        },
        "documentation_ir_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "kind" = "documentation-output",
        "non_claims" = [
          "unsupported format not trusted",
        ],
        "output_path" = "fixtures/core/docs/rendered.html",
        "renderer_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "1c68cc4b50751defc83e8ff7aa1ae6b6cff3d067fd8f27de63d37cbe6354d87a",
        },
        "schema_version" = 1,
        "target_format" = "pdf",
      },
      "requirement_ids" = [
        "r[kamacite.docs.renderers.unsupported_format]",
      ],
    },
    {
      "artifact_class" = "documentation-output",
      "checked_laws" = [
        "drift-fails",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-output.negative.renderer-drift.receipt.json",
      "id" = "docs-output.negative.renderer-drift",
      "non_claims" = [
        "stale output not trusted",
      ],
      "payload" = {
        "checked_laws" = [
          "drift-fails",
        ],
        "claims_canonical_ir" = false,
        "declared_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "47631b18255ce8007bf4983884627d42847fd37e495db6b167ad6c81a9207a68",
        },
        "documentation_ir_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "kind" = "documentation-output",
        "non_claims" = [
          "stale output not trusted",
        ],
        "output_path" = "fixtures/core/docs/rendered.html",
        "renderer_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "1c68cc4b50751defc83e8ff7aa1ae6b6cff3d067fd8f27de63d37cbe6354d87a",
        },
        "schema_version" = 1,
        "target_format" = "html",
      },
      "requirement_ids" = [
        "r[kamacite.docs.renderers.drift]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "fail-closed-language",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-cell.negative.unsupported-language.receipt.json",
      "id" = "docs-cell.negative.unsupported-language",
      "non_claims" = [
        "unsupported language not trusted",
      ],
      "payload" = {
        "cell_id" = "cell.python",
        "checked_laws" = [
          "fail-closed-language",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = null,
        "execution_mode" = "stored-evidence",
        "expected_diagnostic" = null,
        "expected_output_identity" = null,
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "python",
        "non_claims" = [
          "unsupported language not trusted",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = null,
        "observed_output_path" = null,
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "73bec48c589d0adf4b4de96d3b62ce7781de12a46760d2fd7a09b5ea56b9a17c",
        },
        "source_path" = "fixtures/core/docs/cell-success.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.unsupported_language]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "declared-capabilities-only",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-cell.negative.undeclared-capability.receipt.json",
      "id" = "docs-cell.negative.undeclared-capability",
      "non_claims" = [
        "ambient network not admitted",
      ],
      "payload" = {
        "cell_id" = "cell.nickel.network",
        "checked_laws" = [
          "declared-capabilities-only",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "880a6d48041b5291afb5aec2d885c4f94f93301d642734a53cd792aaf0486b95",
        },
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = null,
        "expected_output_identity" = null,
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "nickel",
        "non_claims" = [
          "ambient network not admitted",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [
          "network",
        ],
        "observed_output_identity" = null,
        "observed_output_path" = null,
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "73bec48c589d0adf4b4de96d3b62ce7781de12a46760d2fd7a09b5ea56b9a17c",
        },
        "source_path" = "fixtures/core/docs/cell-success.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.undeclared_capability]",
        "r[kamacite.docs.runnable_cells.ambient_authority]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "evaluator-required",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-cell.negative.missing-evaluator.receipt.json",
      "id" = "docs-cell.negative.missing-evaluator",
      "non_claims" = [
        "missing evaluator not trusted",
      ],
      "payload" = {
        "cell_id" = "cell.nickel.missing-evaluator",
        "checked_laws" = [
          "evaluator-required",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = null,
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = null,
        "expected_output_identity" = null,
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "nickel",
        "non_claims" = [
          "missing evaluator not trusted",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = null,
        "observed_output_path" = null,
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "73bec48c589d0adf4b4de96d3b62ce7781de12a46760d2fd7a09b5ea56b9a17c",
        },
        "source_path" = "fixtures/core/docs/cell-success.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.missing_evaluator]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "expected-output-bound",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-cell.negative.mismatched-output.receipt.json",
      "id" = "docs-cell.negative.mismatched-output",
      "non_claims" = [
        "mismatched output not trusted",
      ],
      "payload" = {
        "cell_id" = "cell.nickel.mismatch",
        "checked_laws" = [
          "expected-output-bound",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "880a6d48041b5291afb5aec2d885c4f94f93301d642734a53cd792aaf0486b95",
        },
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = null,
        "expected_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "a16072b1b0f8588ee724b9d9fef7771759b1a884dedd56b869049e07fee3559b",
        },
        "expected_result" = "success",
        "kind" = "documentation-cell",
        "language" = "nickel",
        "non_claims" = [
          "mismatched output not trusted",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "b576bf327e110a373629f7f0c0ae4aa92f13025760d0a5568271bf9f53174687",
        },
        "observed_output_path" = "fixtures/core/docs/cell-output-ok.txt",
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "73bec48c589d0adf4b4de96d3b62ce7781de12a46760d2fd7a09b5ea56b9a17c",
        },
        "source_path" = "fixtures/core/docs/cell-success.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.mismatched_output]",
      ],
    },
    {
      "artifact_class" = "documentation-cell",
      "checked_laws" = [
        "expected-failure-bound",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-cell.negative.unexpected-success.receipt.json",
      "id" = "docs-cell.negative.unexpected-success",
      "non_claims" = [
        "unexpected success not trusted",
      ],
      "payload" = {
        "cell_id" = "cell.nickel.unexpected-success",
        "checked_laws" = [
          "expected-failure-bound",
        ],
        "claims_semantic_equivalence" = false,
        "declared_capabilities" = [],
        "evaluator_identity" = {
          "algorithm" = "blake3",
          "domain" = "evaluator",
          "hex" = "880a6d48041b5291afb5aec2d885c4f94f93301d642734a53cd792aaf0486b95",
        },
        "execution_mode" = "explicit-run",
        "expected_diagnostic" = "unbound-identifier",
        "expected_output_identity" = null,
        "expected_result" = "failure",
        "kind" = "documentation-cell",
        "language" = "nickel",
        "non_claims" = [
          "unexpected success not trusted",
        ],
        "observed_diagnostic" = null,
        "observed_effects" = [],
        "observed_output_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-output",
          "hex" = "a16072b1b0f8588ee724b9d9fef7771759b1a884dedd56b869049e07fee3559b",
        },
        "observed_output_path" = "fixtures/core/docs/cell-output.txt",
        "run_profile" = "documentation-fixture",
        "schema_version" = 1,
        "source_identity" = {
          "algorithm" = "blake3",
          "domain" = "source",
          "hex" = "f34e5d1005a522d9e966e57551c22da39d15a498c771e4e86c86283a80741229",
        },
        "source_path" = "fixtures/core/docs/cell-failure.nickel",
      },
      "requirement_ids" = [
        "r[kamacite.docs.runnable_cells.unexpected_success]",
      ],
    },
    {
      "artifact_class" = "documentation-receipt",
      "checked_laws" = [
        "malformed-fails",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-receipt.negative.malformed.receipt.json",
      "id" = "docs-receipt.negative.malformed",
      "non_claims" = [
        "malformed receipt not trusted",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/docs/bad-receipt.preserves",
        "cell_receipt_identities" = [],
        "checked_laws" = [
          "malformed-fails",
        ],
        "declared_receipt_identity" = null,
        "diagnostics" = [],
        "documentation_ir_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "kind" = "documentation-receipt",
        "non_claims" = [],
        "output_identities" = [],
        "receipt_kind" = "malformed",
        "renderer_identity" = null,
        "schema_version" = 1,
        "semantic_claims" = [],
        "valence_export_identity" = null,
      },
      "requirement_ids" = [
        "r[kamacite.docs.receipts.malformed]",
      ],
    },
    {
      "artifact_class" = "documentation-receipt",
      "checked_laws" = [
        "overclaim-fails",
      ],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/docs-receipt.negative.non-claim-violation.receipt.json",
      "id" = "docs-receipt.negative.non-claim-violation",
      "non_claims" = [
        "semantic equivalence not trusted",
      ],
      "payload" = {
        "canonical_path" = "fixtures/core/docs/non-claim-violation.preserves",
        "cell_receipt_identities" = [],
        "checked_laws" = [
          "overclaim-fails",
        ],
        "declared_receipt_identity" = {
          "algorithm" = "blake3",
          "domain" = "receipt",
          "hex" = "52acb1e849e290e6656a562d2f8fb250a5e2b37ec6eacedbb80ea22cbf43ae7f",
        },
        "diagnostics" = [],
        "documentation_ir_identity" = {
          "algorithm" = "blake3",
          "domain" = "documentation-ir",
          "hex" = "62cf3fc9b5839cd60c42e73bb15ada84d5de628d19b7986fe782abd7bea680eb",
        },
        "kind" = "documentation-receipt",
        "non_claims" = [
          "semantic equivalence not trusted",
        ],
        "output_identities" = [],
        "receipt_kind" = "render",
        "renderer_identity" = {
          "algorithm" = "blake3",
          "domain" = "adapter",
          "hex" = "1c68cc4b50751defc83e8ff7aa1ae6b6cff3d067fd8f27de63d37cbe6354d87a",
        },
        "schema_version" = 1,
        "semantic_claims" = [
          "semantic equivalence for all examples",
        ],
        "valence_export_identity" = null,
      },
      "requirement_ids" = [
        "r[kamacite.docs.receipts.non_claim_violation]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = ["ast-grep-fixture-coverage", "syntax-only-role"],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/adapter.positive.ast-grep-boundary-audit.receipt.json",
      "id" = "adapter.positive.ast-grep-boundary-audit",
      "non_claims" = [
        "ast-grep findings are syntax-only candidate boundary evidence",
        "a clean scan does not prove adapter correctness or semantic equivalence",
        "codemod output remains an unapplied migration proposal until post-checks pass",
      ],
      "payload" = {
        "kind" = "ast-grep-audit",
        "schema_version" = "kamacite.ast-grep-audit.v1",
        "profile_path" = "audits/ast-grep/generated/profile.json",
        "profile_identity" = { domain = "adapter", algorithm = "blake3", hex = "fb1f8c348ab915ca635be3c58943c05e3009eefa2f290e092e94b5ed9461acd7" },
        "rule_bundle_path" = "audits/ast-grep/generated/rule-bundle.txt",
        "rule_bundle_identity" = { domain = "adapter", algorithm = "blake3", hex = "2d92694ed79df8c79637a5b350fb79d51888e8718dae9aeef18a46f2d55dcad5" },
        "source_scope_identity" = { domain = "adapter", algorithm = "blake3", hex = "3f08ff69255b708c62f384ac88c235337259d15ca2c9421891167db05c4d2547" },
        "findings_identity" = { domain = "adapter", algorithm = "blake3", hex = "e8d5ef04953be9efe92e2cc81f2b7b6ebaa1060590f08d9b6df293fcde05fd2e" },
        "valence_projection_path" = "audits/ast-grep/generated/valence-projection.json",
        "valence_projection_identity" = { domain = "projection", algorithm = "blake3", hex = "31a4ce24f399e9ad96c084fc2a3d732592be1e03b284ada3fef807b5cbe00b6f" },
        "language_profiles" = ast_grep_languages,
        "rules" = [ast_grep_rule 2 { domain = "source", algorithm = "blake3", hex = "e208cae51ae00173bc8882f3865a5914002a44cd16e1094bd4ee84c93c797dc6" } true],
        "sources" = ast_grep_sources,
        "findings" = ast_grep_findings,
        "verification_role" = "boundary",
        "codemod" = ast_grep_codemod,
        "non_claims" = [
          "ast-grep findings are syntax-only candidate boundary evidence",
          "a clean scan does not prove adapter correctness or semantic equivalence",
          "codemod output remains an unapplied migration proposal until post-checks pass",
        ],
      },
      "requirement_ids" = [
        "r[kamacite.ast_grep_adapter_audits.profile]",
        "r[kamacite.ast_grep_adapter_audits.language_boundary]",
        "r[kamacite.ast_grep_adapter_audits.identity]",
        "r[kamacite.ast_grep_adapter_audits.fixtures]",
        "r[kamacite.ast_grep_adapter_audits.codemods]",
        "r[kamacite.ast_grep_adapter_audits.valence_export]",
      ],
    },
    {
      "artifact_class" = "adapter-receipt",
      "checked_laws" = ["rules-without-negative-fixtures-remain-advisory"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/adapter.negative.ast-grep-missing-fixture.receipt.json",
      "id" = "adapter.negative.ast-grep-missing-fixture",
      "non_claims" = ["ast-grep cannot establish semantic verification"],
      "payload" = {
        "kind" = "ast-grep-audit",
        "schema_version" = "kamacite.ast-grep-audit.v1",
        "profile_path" = "audits/ast-grep/generated/profile.json",
        "profile_identity" = { domain = "adapter", algorithm = "blake3", hex = "fb1f8c348ab915ca635be3c58943c05e3009eefa2f290e092e94b5ed9461acd7" },
        "rule_bundle_path" = "audits/ast-grep/generated/rule-bundle.txt",
        "rule_bundle_identity" = { domain = "adapter", algorithm = "blake3", hex = "2d92694ed79df8c79637a5b350fb79d51888e8718dae9aeef18a46f2d55dcad5" },
        "source_scope_identity" = { domain = "adapter", algorithm = "blake3", hex = "3f08ff69255b708c62f384ac88c235337259d15ca2c9421891167db05c4d2547" },
        "findings_identity" = { domain = "adapter", algorithm = "blake3", hex = "e8d5ef04953be9efe92e2cc81f2b7b6ebaa1060590f08d9b6df293fcde05fd2e" },
        "valence_projection_path" = "audits/ast-grep/generated/valence-projection.json",
        "valence_projection_identity" = { domain = "projection", algorithm = "blake3", hex = "31a4ce24f399e9ad96c084fc2a3d732592be1e03b284ada3fef807b5cbe00b6f" },
        "language_profiles" = ast_grep_languages,
        "rules" = [ast_grep_rule 0 null true],
        "sources" = ast_grep_sources,
        "findings" = ast_grep_findings,
        "verification_role" = "verified-semantic",
        "codemod" = ast_grep_codemod,
        "non_claims" = ["ast-grep cannot establish semantic verification"],
      },
      "requirement_ids" = [
        "r[kamacite.ast_grep_adapter_audits.fixtures]",
        "r[kamacite.ast_grep_adapter_audits.valence_export]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["canonical-identity-stable", "exactly-one-host", "closed-effect-summary", "exact-cohort"],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/application-host-binding.positive.animus-producer.receipt.json",
      "id" = "application-host-binding.positive.animus-producer",
      "non_claims" = animus_application_host_binding.non_claims,
      "payload" = animus_application_host_binding,
      "requirement_ids" = [
        "r[kamacite.application_host_binding.artifact]",
        "r[kamacite.application_host_binding.effects]",
        "r[kamacite.application_host_binding.cohorts]",
        "r[kamacite.application_host_binding.fixtures]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["canonical-identity-stable", "exactly-one-host"],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/application-host-binding.positive.capability-free.receipt.json",
      "id" = "application-host-binding.positive.capability-free",
      "non_claims" = ahb_required_non_claims,
      "payload" = ahb_base_payload,
      "requirement_ids" = [
        "r[kamacite.application_host_binding.artifact]",
        "r[kamacite.application_host_binding.fixtures]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["closed-effect-summary", "exact-cohort"],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/application-host-binding.positive.closed-effects.receipt.json",
      "id" = "application-host-binding.positive.closed-effects",
      "non_claims" = ahb_required_non_claims,
      "payload" = ahb_base_payload & {
        policy_identity | force = ahb_identity "adapter" "d84d4c47052a957e8e4cab94a9352349d2c11b68b105acfd3d6ee4be64150555",
        compatibility_identity | force = ahb_identity "evidence-chain" "b98834dc6a41e505e6b8e014f8af8f7f4a8d8d48fe5981105c4da00a178782eb",
        expected_adapter_cohort_identity | force = ahb_base_payload.adapter_cohort_identity,
        expected_resource_profile_identity | force = ahb_base_payload.resource_profile_identity,
        expected_host_profile_identity | force = ahb_base_payload.host_profile_identity,
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.effects]",
        "r[kamacite.application_host_binding.cohorts]",
        "r[kamacite.application_host_binding.fixtures]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["metadata-excluded-from-identity"],
      "expected_verdict" = "accepted",
      "fixture_class" = "positive",
      "golden_receipt" = "fixtures/golden/application-host-binding.positive.metadata-rename.receipt.json",
      "id" = "application-host-binding.positive.metadata-rename",
      "non_claims" = ahb_required_non_claims,
      "payload" = ahb_base_payload & {
        metadata | force = {
          display_name = "Renamed application binding",
          aliases = ["renamed-binding"],
          path = "src/renamed.rs",
          source_span = "0:128",
          documentation = "review metadata never enters canonical identity material",
        },
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.artifact]",
        "r[kamacite.application_host_binding.projection]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["missing-host-rejected"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/application-host-binding.negative.missing-host.receipt.json",
      "id" = "application-host-binding.negative.missing-host",
      "non_claims" = ["missing host selection fails closed"],
      "payload" = ahb_base_payload & {
        host_profile_identity | force = null,
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.host_selection]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["ambiguous-host-rejected"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/application-host-binding.negative.alternative-hosts.receipt.json",
      "id" = "application-host-binding.negative.alternative-hosts",
      "non_claims" = ["alternative host selections fail closed"],
      "payload" = ahb_base_payload & {
        alternative_host_profile_identities | force = [
          ahb_identity "evaluator" "affeb681f4525f30e2acd8fab612a914c249e99f03080fa4d7af29fa831169aa",
        ],
        fallback_host_profile_identity | force = ahb_identity "evaluator" "66199034795f03d88ed1f634405759cf5e36a331b6bb538953f400fe8fd1201c",
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.host_selection]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["open-effect-summary-rejected"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/application-host-binding.negative.open-effect-summary.receipt.json",
      "id" = "application-host-binding.negative.open-effect-summary",
      "non_claims" = ["open effect summaries cannot admit executable bindings"],
      "payload" = ahb_base_payload & {
        effect_summary_closed | force = false,
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.effects]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["undeclared-operation-rejected"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/application-host-binding.negative.undeclared-operation.receipt.json",
      "id" = "application-host-binding.negative.undeclared-operation",
      "non_claims" = ["wrong-domain operations fail closed"],
      "payload" = ahb_base_payload & {
        effect_operation_identities | force = ahb_base_payload.effect_operation_identities @ [
          ahb_identity "source" "219bfba503cce8390619632f8116d3d805c60bd9122f02cbf6eea22e8971af1f",
        ],
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.effects]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["stale-cohort-rejected"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/application-host-binding.negative.stale-cohort.receipt.json",
      "id" = "application-host-binding.negative.stale-cohort",
      "non_claims" = ["stale cohorts fail closed"],
      "payload" = ahb_base_payload & {
        expected_adapter_cohort_identity | force = ahb_identity "adapter" "fca1f44896bd6468aec24033d3cf70826d178a33539efa3569427d1b7337d5e8",
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.cohorts]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["unsupported-version-rejected"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/application-host-binding.negative.unsupported-version.receipt.json",
      "id" = "application-host-binding.negative.unsupported-version",
      "non_claims" = ["unsupported schema versions fail closed"],
      "payload" = ahb_base_payload & {
        schema_version | force = "kamacite.application-host-binding.v0",
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.artifact]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["authority-overclaim-rejected"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/application-host-binding.negative.authority-overclaim.receipt.json",
      "id" = "application-host-binding.negative.authority-overclaim",
      "non_claims" = ["authority overclaims fail closed"],
      "payload" = ahb_base_payload & {
        semantic_claims | force = ["binding grants host authorization"],
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.boundary]",
      ],
    },
    {
      "artifact_class" = "application-host-binding",
      "checked_laws" = ["missing-non-claims-rejected"],
      "expected_verdict" = "rejected",
      "fixture_class" = "negative",
      "golden_receipt" = "fixtures/golden/application-host-binding.negative.missing-non-claims.receipt.json",
      "id" = "application-host-binding.negative.missing-non-claims",
      "non_claims" = ["incomplete non-claims fail closed"],
      "payload" = ahb_base_payload & {
        non_claims | force = ["not host capability or authorization"],
      },
      "requirement_ids" = [
        "r[kamacite.application_host_binding.boundary]",
      ],
    },
  ],
  "schema_version" = 2,
}
