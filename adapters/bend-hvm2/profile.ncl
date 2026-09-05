let Contracts = import "contracts.ncl" in
let kibibyte = 1024 in
let mebibyte = kibibyte * kibibyte in
let gibibyte = mebibyte * kibibyte in
let u24_modulus = 16777216 in
let u24_maximum = u24_modulus - 1 in
let pilot_leaf_limit = 4 in
let pilot_timeout_milliseconds = 10000 in
let pilot_cpu_seconds = 10 in
let pilot_output_limit_bytes = mebibyte in
let pilot_diagnostic_limit_bytes = 64 * kibibyte in
let pilot_artifact_limit_bytes = mebibyte in
let pilot_file_limit_bytes = 2 * mebibyte in
let pilot_process_limit = 256 in
let pilot_warmup_runs = 1 in
let pilot_measured_runs = 5 in
let pilot_maximum_spread_basis_points = 20000 in
let pilot_maximum_generated_c_oracle_ratio = 50 in
let pilot_maximum_run_rs_oracle_ratio = 200 in
let pilot_maximum_stage_wall_milliseconds = 5000 in
let trellis_revision = "f26e3cd465101698ade12f8d142e0dc95578326b" in
let Profile = {
  schema_version = "kamacite.bend-hvm2-evaluation-profile.v1",
  profile_id = "kamacite-bend-hvm2-cpu-pilot-v1",
  disposition = "evaluation-only",
  canonical_ir = "kamacite-packed-preserves",
  cohort = {
    bend = {
      repository = "https://github.com/HigherOrderCO/Bend",
      revision = "814453670d0e0d6777c1313c972764dba0491b7f",
      version = "0.2.38",
      nix_nar_sha256 = "sha256-T8BF/C2a1g9FlzOQ1CoiEce0ozUVNXyhJ3QIksg7iYo=",
      license = "Apache-2.0",
      license_path = "adapters/bend-hvm2/upstream/bend/LICENSE-APACHE",
      license_blake3 = "7e16ff676b2bb64fea3d601f332aaefb4ae8c8123271a01e8ebf52c7bf5592c2",
      attribution = "Source repository HigherOrderCO/Bend with retained Apache-2.0 text",
    },
    hvm2 = {
      repository = "https://github.com/HigherOrderCO/HVM",
      revision = "654276018084b8f44a22b562dd68ab18583bfb5b",
      version = "2.0.22",
      nix_nar_sha256 = "sha256-6cm+zpzJ0d77cjOfiM/fU7TGVue6GK3Fncit67RW8bA=",
      license = "Apache-2.0",
      license_path = "adapters/bend-hvm2/upstream/hvm2/LICENSE",
      license_blake3 = "4e2804b387ec9c0ed5b18f26bdbb741e14c2dc166d08a6ecb90cce8d0ebddbaa",
      attribution = "Source repository HigherOrderCO/HVM with retained Apache-2.0 text",
    },
    bend_hvm_dependency = "=2.0.22",
    compiler = {
      family = "gcc",
      version = "15.2.0",
      target = "x86_64-unknown-linux-gnu",
      flags = ["-O2", "-lm", "-lpthread"],
    },
    command_profiles = [
      {
        id = "bend-run-rs-v1",
        executable = "bend",
        arguments = ["--hvm-bin", "<hvm>", "run-rs", "/work/workload.bend", "-s"],
      },
      {
        id = "bend-gen-c-v1",
        executable = "bend",
        arguments = ["--hvm-bin", "<hvm>", "gen-c", "/work/workload.bend"],
      },
      {
        id = "gcc-generated-c-v1",
        executable = "cc",
        arguments = ["/work/generated.c", "-o", "/work/generated", "-O2", "-lm", "-lpthread"],
      },
      {
        id = "generated-c-run-v1",
        executable = "/work/generated",
        arguments = [],
      },
      {
        id = "trellis-oracle-v1",
        executable = "kamacite-bend-hvm2-oracle",
        arguments = ["16777215", "1", "0", "1"],
      },
    ],
    excluded_toolchains = ["cuda", "hvm3", "hvm4", "kind", "unpinned-tools"],
  },
  workload = {
    id = "balanced-u24-four-leaf-v1",
    algorithm_profile = "trellis.balanced-u24-wrapping-sum.v1",
    leaves = [u24_maximum, 1, 0, 1],
    leaf_limit = pilot_leaf_limit,
    numeric_type = "u24",
    modulus = u24_modulus,
    result = 1,
    source_path = "fixtures/bend-hvm2/workload.bend",
    canonical_input_path = "fixtures/bend-hvm2/input.json",
    required_annotations = [
      "def sum(tree: U24Tree) -> u24:",
      "def main() -> u24:",
      "value: u24",
    ],
    forbidden_source_tokens = ["Any", "IO/", "open(", "load(", "store(", "unsafe", "ffi", "cuda"],
  },
  resources = {
    timeout_milliseconds = pilot_timeout_milliseconds,
    cpu_seconds = pilot_cpu_seconds,
    memory_limit_bytes = gibibyte,
    output_limit_bytes = pilot_output_limit_bytes,
    diagnostic_limit_bytes = pilot_diagnostic_limit_bytes,
    artifact_limit_bytes = pilot_artifact_limit_bytes,
    file_limit_bytes = pilot_file_limit_bytes,
    process_limit = pilot_process_limit,
  },
  benchmark = {
    warmup_runs = pilot_warmup_runs,
    measured_runs = pilot_measured_runs,
    statistic = "median-wall-nanoseconds",
    maximum_spread_basis_points = pilot_maximum_spread_basis_points,
    maximum_generated_c_oracle_ratio = pilot_maximum_generated_c_oracle_ratio,
    maximum_run_rs_oracle_ratio = pilot_maximum_run_rs_oracle_ratio,
    maximum_stage_wall_milliseconds = pilot_maximum_stage_wall_milliseconds,
  },
  trellis = {
    repository = "https://github.com/OnixResearch/trellis",
    revision = trellis_revision,
    nix_nar_sha256 = "sha256-GR3BbWGDuTqj2Gy5IKZ4KSBzDyJdnvVrKEK2Y1rCx+c=",
    proof_bundle_path = "adapters/bend-hvm2/upstream/trellis/proof-evidence-bundle.json",
    proof_bundle_blake3 = "e58a90561093f7ae28adfe8eb56c16a618c4e99e056af8822633609d5201a70e",
    scope = "module:bounded_modular_tree_fold",
    algorithm_profile = "trellis.balanced-u24-wrapping-sum.v1",
    executable_oracle_symbol = "verified_logic::bounded_modular_tree_fold::sequential_u24_sum",
    proof_command = "./scripts/verus.sh src/lib.rs",
    verifier_toolchain = "Verus via scripts/verus.sh; Tracey via docs/requirements.md",
    verifier_provider_revision = "374bd16b26cee2af34211a29bfa531c016811f51",
    proof_check = "trellis-verus-proofs-exact-source-v1",
    required_requirement_ids = [
      "bounded-modular-tree-fold.admission",
      "bounded-modular-tree-fold.algebra",
      "bounded-modular-tree-fold.boundary",
      "bounded-modular-tree-fold.docs",
      "bounded-modular-tree-fold.equivalence",
      "bounded-modular-tree-fold.evidence",
      "bounded-modular-tree-fold.fixtures",
      "bounded-modular-tree-fold.non-claims",
      "bounded-modular-tree-fold.refinement",
      "bounded-modular-tree-fold.u24-add",
      "bounded-modular-tree-fold.u24-domain",
      "bounded-modular-tree-fold.validation",
      "bounded-modular-tree-fold.work",
    ],
    required_proof_functions = [
      "bounded_modular_tree_fold::core::power_of_two_combine_count_exact",
      "bounded_modular_tree_fold::core::power_of_two_round_progress",
      "bounded_modular_tree_fold::core::u24_pair_round_preserves_sum",
      "u24::u24_add_associative",
      "u24::u24_add_closed",
      "u24::u24_add_commutative",
      "u24::u24_add_zero_identity",
      "u24::u24_normalize_closed",
      "u24::u24_normalize_idempotent",
      "u24::u24_normalize_valid_identity",
    ],
    required_non_claims = [
      "not release approval",
      "not whole-program correctness",
      "not third-party conformance",
    ],
  },
  denied_authority = [
    "network",
    "runtime-io",
    "filesystem-outside-scratch",
    "host-process-namespace",
    "environment-inheritance",
    "clock-input",
    "random",
    "credential",
    "device",
    "package-manager",
    "mutation-outside-scratch-and-receipt-output",
  ],
  required_parent_roles = [
    "bend-source-envelope",
    "canonical-leaf-input",
    "adapter-profile",
    "trellis-proof-bundle",
    "trellis-proof-validation",
    "bend-run-rs-observation",
    "generated-c-source",
    "compiler-invocation",
    "generated-executable",
    "generated-c-observation",
    "pilot-decision",
  ],
  checked_laws = [
    "exact-cohort-precedes-execution",
    "isolated-proc-mount-exposes-only-sandbox-processes",
    "canonical-leaves-render-the-reviewed-source",
    "trellis-oracle-does-not-call-bend-or-hvm2",
    "three-route-equality-precedes-passing-receipt",
    "artifact-layers-retain-distinct-identities",
    "benchmark-policy-precedes-measurement",
    "runtime-consumer-authority-remains-external",
  ],
  non_claims = [
    "not Bend correctness",
    "not HVM2 correctness",
    "not generated-C semantic equivalence",
    "not host-compiler correctness",
    "not verifier soundness",
    "not sandbox completeness",
    "not portable performance",
    "not runtime adoption",
    "not production readiness",
    "not release eligibility",
  ],
} in
Profile | Contracts.Profile
