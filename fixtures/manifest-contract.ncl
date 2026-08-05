# r[impl kamacite.findings.manifest_semantic_freshness.typed_contract]
# Closed contract for the human-authored Kamacite fixture manifest.
# Every payload alternative is selected by an immediate `kind` check before
# its closed record contract is applied.

let Natural = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value >= 0
) in

let OneOf = fun values =>
  std.contract.from_predicate (fun value =>
    std.is_string value && std.array.elem value values
  )
in

let Hex64 = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in

let Nullable = fun Contract =>
  std.contract.custom (fun label value =>
    if value == null then
      'Ok value
    else
      std.contract.check Contract label value
  )
in

let Tagged = fun Contract =>
  std.contract.custom (fun label =>
    match {
      value @ { kind, .. } if kind == Contract.kind =>
        std.contract.check Contract label value,
      { kind, .. } =>
        'Error { message = "unsupported manifest payload kind `%{kind}`" },
      _ =>
        'Error { message = "manifest payload is missing its kind field" },
    }
  )
in

let IdentityDomain = OneOf [
  "source",
  "canonical-value",
  "projection",
  "target-artifact",
  "adapter",
  "evaluator",
  "compiler",
  "receipt",
  "function-address",
  "dependency-closure",
  "algebraic-effect",
  "effect-operation",
  "effect-log",
  "fsm-spec",
  "fsm-universe",
  "fsm-coverage",
  "documentation-ir",
  "documentation-output",
  "documentation-facts",
  "dialect-payload",
  "evidence-chain",
  "application-host-binding",
] in

let Identity = {
  domain | IdentityDomain,
  algorithm | OneOf ["blake3"],
  hex | Hex64,
} in

let NullableIdentity = Nullable Identity in
let NullableString = Nullable String in
let StringArray = Array String in
let IdentityArray = Array Identity in

let SteelHostFunction = {
  function_id | String,
  input_schema | String,
  output_schema | String,
  effect_class | String,
  required_capabilities | StringArray,
  checked_laws | StringArray,
} in

let EffectOperation = {
  operation_id | String,
  display_name | String,
  parameter_schema | NullableString,
  result_schema | NullableString,
} in

let HandlerDescriptor = {
  name | String,
  handled_operations | StringArray,
  residual_operations | StringArray,
  continuation_policy | String,
  claims_discharge | Bool,
  host_capability_requirements | StringArray,
} in

let HandlerReceipt = {
  discharged_operations | StringArray,
  residual_operations | StringArray,
  continuation_policy | String,
  continuation_evidence | NullableString,
  observed_operations | StringArray,
  checked_laws | StringArray,
} in

let ReplayProfile = {
  replay_class | String,
  effect_log_path | NullableString,
  effect_log_identity | NullableIdentity,
} in

let FsmState = {
  state_id | String,
  label | String,
} in

let FsmEvent = {
  event_id | String,
  label | String,
} in

let FsmGuard = {
  guard_id | String,
  required_evidence_refs | StringArray,
  non_claims | StringArray,
} in

let FsmSideEffect = {
  effect_id | String,
  class | String,
} in

let FsmTransition = {
  from_state | String,
  event | String,
  to_state | String,
  guards | StringArray,
  side_effect | String,
} in

let FsmDenial = {
  from_state | String,
  event | String,
  reason | String,
  guards | StringArray,
  side_effect | String,
  preserves_state | Bool,
} in

let FsmGuardDecision = {
  guard_id | String,
  allowed | Bool,
  evidence_refs | StringArray,
} in

let FsmTraceStep = {
  from_state | String,
  event | String,
  to_state | String,
  transition_identity | NullableIdentity,
  guard_decisions | Array FsmGuardDecision,
} in

let DocumentationSection = {
  section_id | String,
  anchor | String,
  title | String,
  prose_blocks | StringArray,
  code_blocks | StringArray,
  runnable_cell_ids | StringArray,
  links | StringArray,
} in

let DocumentationCellDescriptor = {
  cell_id | String,
  language | String,
  source_identity | NullableIdentity,
  required_capabilities | StringArray,
  expected_result | String,
  evaluator_identity | NullableIdentity,
  non_claims | StringArray,
} in

let DocumentationPage = {
  page_id | String,
  title | String,
  source_envelope_identity | NullableIdentity,
  sections | Array DocumentationSection,
  cells | Array DocumentationCellDescriptor,
} in

let SourceEnvelope = Tagged {
  kind = "source-envelope",
  schema_version | Natural,
  language | String,
  media_type | String,
  parser_identity | String,
  source_path | String,
  declared_identity | Identity,
  aliases | StringArray,
} in

let CanonicalValue = Tagged {
  kind = "canonical-value",
  schema_version | Natural,
  canonical_path | String,
  declared_identity | Identity,
  projection_identity | NullableIdentity,
  projection_claims_authority | Bool,
  aliases | StringArray,
} in

let AdapterReceipt = Tagged {
  kind = "adapter-receipt",
  adapter_kind | String,
  has_conformance_entry | Bool,
  declared_capabilities | StringArray,
  observed_effects | StringArray,
  claims_canonical_ir | Bool,
  checked_laws | StringArray,
} in

let AstGrepLanguageProfile = {
  language | String,
  parser_kind | String,
  parser_identity | Identity,
  extensions | StringArray,
} in
let AstGrepRuleCoverage = {
  rule_id | String,
  language | String,
  rule_identity | Identity,
  positive_fixture_identity | NullableIdentity,
  negative_fixture_identity | NullableIdentity,
  positive_cases | Natural,
  negative_cases | Natural,
  gate_eligible | Bool,
} in
let AstGrepSource = {
  path | String,
  identity | Identity,
} in
let AstGrepFinding = {
  rule_id | String,
  source_identity | Identity,
  start_byte | Natural,
  end_byte | Natural,
  message | String,
} in
let AstGrepCodemod = {
  recipe_path | String,
  recipe_identity | Identity,
  before_source_path | String,
  before_source_identity | Identity,
  after_source_path | String,
  after_source_identity | Identity,
  dry_run | Bool,
  post_check_commands | StringArray,
  non_claims | StringArray,
} in
let AstGrepAudit = Tagged {
  kind = "ast-grep-audit",
  schema_version | String,
  profile_path | String,
  profile_identity | Identity,
  rule_bundle_path | String,
  rule_bundle_identity | Identity,
  source_scope_identity | Identity,
  findings_identity | Identity,
  valence_projection_path | String,
  valence_projection_identity | Identity,
  language_profiles | Array AstGrepLanguageProfile,
  rules | Array AstGrepRuleCoverage,
  sources | Array AstGrepSource,
  findings | Array AstGrepFinding,
  verification_role | String,
  codemod | Nullable AstGrepCodemod,
  non_claims | StringArray,
} in

let NickelSandboxLimits = {
  execution_timeout_milliseconds | Natural,
  cpu_seconds | Natural,
  memory_limit_bytes | Natural,
  output_limit_bytes | Natural,
  diagnostic_limit_bytes | Natural,
} in
let NickelSandboxProfile = {
  schema_version | String,
  sandbox_kind | String,
  evaluator_profile | String,
  environment | Dyn,
  read_only_source_path | String,
  read_only_import_root | String,
  network_allowed | Bool,
  process_context_visible | Bool,
  writable_paths | StringArray,
  admitted_capabilities | StringArray,
  limits | NickelSandboxLimits,
  profile_identity | Identity,
} in
let NickelSandboxEvidence = {
  profile_identity | Identity,
  evaluator_identity | Identity,
  import_identities | Dyn,
  requested_capabilities | StringArray,
  admitted_capabilities | StringArray,
  expected_effects | StringArray,
  observed_execution_facts | StringArray,
  outcome | String,
  exit_code | Nullable Natural,
  stdout_identity | NullableIdentity,
  stderr_identity | Identity,
  sandbox_enforced | Bool,
} in
let NickelAdapterReceipt = Tagged {
  kind = "nickel-adapter-receipt",
  schema_version | Natural,
  adapter_kind | String,
  source_path | NullableString,
  declared_source_identity | NullableIdentity,
  evaluator_profile | String,
  evaluator_descriptor | String,
  evaluator_identity | NullableIdentity,
  export_path | NullableString,
  projection_identity | NullableIdentity,
  declared_capabilities | StringArray,
  observed_effects | StringArray,
  import_paths | StringArray,
  sandbox_profile | Nullable NickelSandboxProfile,
  sandbox_evidence | Nullable NickelSandboxEvidence,
  claims_canonical_ir | Bool,
  checked_laws | StringArray,
  execution_mode | String,
} in

let SteelAdapterReceipt = Tagged {
  kind = "steel-adapter-receipt",
  schema_version | Natural,
  adapter_kind | String,
  source_path | NullableString,
  declared_source_identity | NullableIdentity,
  evaluator_profile | String,
  evaluator_descriptor | String,
  evaluator_identity | NullableIdentity,
  module_descriptor_path | String,
  module_identity | NullableIdentity,
  input_schema | String,
  output_schema | String,
  host_functions | Array SteelHostFunction,
  observed_host_functions | StringArray,
  declared_builtin_modules | StringArray,
  declared_imports | StringArray,
  requested_imports | StringArray,
  declared_capabilities | StringArray,
  observed_effects | StringArray,
  effect_class | String,
  output_path | NullableString,
  output_projection_identity | NullableIdentity,
  output_class | OneOf [
    "dto",
    "projection",
    "descriptor",
    "diagnostic",
    "receipt",
    "opaque-closure",
    "bytecode",
    "json-string",
  ],
  claims_canonical_ir | Bool,
  checked_laws | StringArray,
  execution_mode | String,
} in

let TargetArtifact = Tagged {
  kind = "target-artifact",
  target_kind | String,
  output_path | String,
  declared_identity | Identity,
  claims_canonical_ir | Bool,
  metadata | {_: String},
} in

let UnsupportedClaim = Tagged {
  kind = "unsupported-claim",
  claim_class | String,
  claim_text | String,
  preserve_source_only | Bool,
} in

let IdentityUse = Tagged {
  kind = "identity-use",
  required_domain | IdentityDomain,
  supplied_identity | Identity,
} in

let CapabilityShell = Tagged {
  kind = "capability-shell",
  declared_filesystem | Bool,
  path | String,
  observation | OneOf [
    "normal-relative",
    "parent-traversal",
    "absolute-path",
    "symlink-escape",
    "undeclared-filesystem",
  ],
} in

let StackEvidence = Tagged {
  kind = "stack-evidence",
  role | String,
  bounded_claim | String,
  broadens_claim | Bool,
  referenced_identities | IdentityArray,
} in

let FsmSpec = Tagged {
  kind = "fsm-spec",
  schema_version | Natural,
  fsm_id | String,
  canonical_path | String,
  source_envelope_identity | NullableIdentity,
  declared_identity | Identity,
  states | Array FsmState,
  events | Array FsmEvent,
  initial_state | String,
  terminal_states | StringArray,
  intentionally_unreachable_states | StringArray,
  determinism_policy | String,
  totality_policy | String,
  guards | Array FsmGuard,
  side_effects | Array FsmSideEffect,
  transitions | Array FsmTransition,
  denials | Array FsmDenial,
  checked_laws | StringArray,
  non_claims | StringArray,
} in

let FsmReceipt = Tagged {
  kind = "fsm-receipt",
  schema_version | Natural,
  receipt_kind | String,
  canonical_path | String,
  descriptor_fixture_id | String,
  spec_identity | Identity,
  state_universe_identity | Identity,
  event_universe_identity | Identity,
  transition_coverage_identity | Identity,
  denial_coverage_identity | Identity,
  adapter_identity | Identity,
  dependency_identities | IdentityArray,
  octet_receipt_identities | IdentityArray,
  valence_export_identity | NullableIdentity,
  checked_laws | StringArray,
  diagnostics | StringArray,
  non_claims | StringArray,
  semantic_claims | StringArray,
  declared_receipt_identity | NullableIdentity,
  trace_start_state | NullableString,
  trace_events | StringArray,
  trace_steps | Array FsmTraceStep,
  trace_final_state | NullableString,
} in

let DocumentationIr = Tagged {
  kind = "documentation-ir",
  schema_version | Natural,
  canonical_path | String,
  declared_identity | Identity,
  collector_identity | Identity,
  pages | Array DocumentationPage,
  target_formats | StringArray,
  dependency_identities | IdentityArray,
  checked_laws | StringArray,
  non_claims | StringArray,
} in

let DocumentationOutput = Tagged {
  kind = "documentation-output",
  schema_version | Natural,
  documentation_ir_identity | Identity,
  renderer_identity | Identity,
  target_format | String,
  output_path | String,
  declared_output_identity | Identity,
  claims_canonical_ir | Bool,
  checked_laws | StringArray,
  non_claims | StringArray,
} in

let DocumentationCell = Tagged {
  kind = "documentation-cell",
  schema_version | Natural,
  cell_id | String,
  language | String,
  source_path | String,
  source_identity | NullableIdentity,
  run_profile | String,
  execution_mode | String,
  evaluator_identity | NullableIdentity,
  declared_capabilities | StringArray,
  observed_effects | StringArray,
  expected_result | String,
  expected_output_identity | NullableIdentity,
  observed_output_path | NullableString,
  observed_output_identity | NullableIdentity,
  expected_diagnostic | NullableString,
  observed_diagnostic | NullableString,
  claims_semantic_equivalence | Bool,
  checked_laws | StringArray,
  non_claims | StringArray,
} in

let DocumentationReceipt = Tagged {
  kind = "documentation-receipt",
  schema_version | Natural,
  receipt_kind | String,
  canonical_path | String,
  documentation_ir_identity | Identity,
  renderer_identity | NullableIdentity,
  output_identities | IdentityArray,
  cell_receipt_identities | IdentityArray,
  valence_export_identity | NullableIdentity,
  declared_receipt_identity | NullableIdentity,
  diagnostics | StringArray,
  semantic_claims | StringArray,
  checked_laws | StringArray,
  non_claims | StringArray,
} in

let ValenceReceiptExport = Tagged {
  kind = "valence-receipt-export",
  source_receipt_path | String,
  conformance_path | String,
  source_spans_path | NullableString,
  semantic_claims | StringArray,
  declared_export_identity | NullableIdentity,
} in

let AlgebraicEffect = Tagged {
  kind = "algebraic-effect",
  schema_version | Natural,
  canonical_path | String,
  declared_identity | Identity,
  signatures | Array EffectOperation,
  input_row | StringArray,
  row_open | Bool,
  claimed_pure | Bool,
  handler | Nullable HandlerDescriptor,
  receipt | Nullable HandlerReceipt,
  replay | Nullable ReplayProfile,
  claimed_host_authority | StringArray,
  admitted_host_capabilities | StringArray,
} in

let BindingMetadata = {
  display_name | NullableString,
  aliases | StringArray,
  path | NullableString,
  source_span | NullableString,
  documentation | NullableString,
} in

let ApplicationHostBinding = Tagged {
  kind = "application-host-binding",
  schema_version | String,
  application_identity | Identity,
  entrypoint_identity | Identity,
  input_schema_identity | Identity,
  output_schema_identity | Identity,
  effect_summary_identity | Identity,
  effect_summary_closed | Bool,
  effect_operation_identities | IdentityArray,
  host_profile_identity | NullableIdentity,
  alternative_host_profile_identities | IdentityArray,
  fallback_host_profile_identity | NullableIdentity,
  adapter_cohort_identity | Identity,
  resource_profile_identity | Identity,
  policy_identity | NullableIdentity,
  compatibility_identity | NullableIdentity,
  expected_adapter_cohort_identity | NullableIdentity,
  expected_resource_profile_identity | NullableIdentity,
  expected_host_profile_identity | NullableIdentity,
  metadata | BindingMetadata,
  semantic_claims | StringArray,
  non_claims | StringArray,
} in

let Payload = std.contract.any_of [
  SourceEnvelope,
  CanonicalValue,
  AdapterReceipt,
  AstGrepAudit,
  NickelAdapterReceipt,
  SteelAdapterReceipt,
  TargetArtifact,
  UnsupportedClaim,
  IdentityUse,
  CapabilityShell,
  StackEvidence,
  FsmSpec,
  FsmReceipt,
  DocumentationIr,
  DocumentationOutput,
  DocumentationCell,
  DocumentationReceipt,
  ValenceReceiptExport,
  AlgebraicEffect,
  ApplicationHostBinding,
] in

let Fixture = {
  id | String,
  fixture_class | OneOf ["positive", "negative", "property"],
  artifact_class | OneOf [
    "source-envelope",
    "canonical-value",
    "adapter-descriptor",
    "adapter-receipt",
    "target-artifact",
    "receipt",
    "unsupported-claim",
    "capability-shell",
    "algebraic-effect",
    "stack-evidence",
    "valence-receipt-export",
    "fsm-spec",
    "fsm-receipt",
    "documentation-ir",
    "documentation-output",
    "documentation-cell",
    "documentation-receipt",
    "application-host-binding",
  ],
  requirement_ids | StringArray,
  non_claims | StringArray,
  expected_verdict | OneOf ["accepted", "rejected"],
  checked_laws | StringArray,
  golden_receipt | NullableString,
  payload | Payload,
} in

{
  schema_version | Natural,
  authority_non_claims | StringArray,
  fixtures | Array Fixture,
}
