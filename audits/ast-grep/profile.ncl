let contracts = import "contracts.ncl" in
{
  schema_version = "kamacite.ast-grep-profile.v1",
  profile_id = "kamacite-adapter-boundary-audits",
  tool = {
    name = "ast-grep",
    version = "0.44.1",
  },
  sgconfig_path = "audits/ast-grep/sgconfig.yml",
  bundle_path = "audits/ast-grep/generated/rule-bundle.txt",
  bundle_identity = {
    domain = "adapter",
    algorithm = "blake3",
    hex = "2d92694ed79df8c79637a5b350fb79d51888e8718dae9aeef18a46f2d55dcad5",
  },
  bundle_inputs = [
    "audits/ast-grep/sgconfig.yml",
    "audits/ast-grep/rules/core-ambient-process.yml",
    "audits/ast-grep/rule-tests/core-ambient-process-test.yml",
    "audits/ast-grep/rule-tests/__snapshots__/core-ambient-process-snapshot.yml",
  ],
  scan_scope = {
    paths = ["crates/kamacite-core/src"],
    globs = ["*.rs", "!**/target/**"],
  },
  language_profiles = [
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
  ],
  rules = [
    {
      rule_id = "core-ambient-process",
      language = "Rust",
      rule_path = "audits/ast-grep/rules/core-ambient-process.yml",
      rule_identity = { domain = "source", algorithm = "blake3", hex = "0ece21345e4c362c56ab265c2839f2d8dc13b1d6ac8152bb481f1bc65b582717" },
      positive_fixture_path = "audits/ast-grep/fixtures/positive/core-ambient-process.rs",
      positive_fixture_identity = { domain = "source", algorithm = "blake3", hex = "e36a898551e0be83b4b9cb325e42c47c919953c15f722521e10209e0f41f70eb" },
      negative_fixture_path = "audits/ast-grep/fixtures/negative/core-pure.rs",
      negative_fixture_identity = { domain = "source", algorithm = "blake3", hex = "e208cae51ae00173bc8882f3865a5914002a44cd16e1094bd4ee84c93c797dc6" },
      positive_cases = 2,
      negative_cases = 2,
      gate_eligible = true,
    },
  ],
  severity_posture = "error",
  verification_role = "boundary",
  codemod = {
    mode = "dry-run-only",
    recipe_path = "audits/ast-grep/codemods/core-ambient-process.json",
    validation_commands = [
      "ast-grep test --config audits/ast-grep/sgconfig.yml",
      "cargo test --workspace",
      "cargo clippy --workspace --all-targets -- -D warnings",
    ],
  },
  non_claims = [
    "ast-grep findings are syntax-only candidate boundary evidence",
    "a clean scan does not prove adapter correctness or semantic equivalence",
    "codemod output remains an unapplied migration proposal until post-checks pass",
  ],
} | contracts.Profile
