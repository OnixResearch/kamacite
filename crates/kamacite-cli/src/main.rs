#![forbid(unsafe_code)]

mod nickel_preserves_adapter;
mod nickel_sandbox;
mod root_io;
mod steel_runner;

use kamacite_core::{
    AdapterReceiptInput, AlgebraicEffectInput, ApplicationHostBindingInput, ArtifactClass,
    AstGrepAuditInput, AstGrepCodemodArtifact, AstGrepFinding, AstGrepLanguageProfile,
    AstGrepRuleCoverage, AstGrepSourceEnvelope, Blake3Hex, CanonicalValueInput,
    CapabilityShellInput, DocsActionEvidenceInput, DocsActionKind, DocsActionOutcome,
    DocsCompletionState, DocsConfigProjection, DocumentationCellInput, DocumentationFactsIdentity,
    DocumentationIrInput, DocumentationOutputIdentity, DocumentationOutputInput,
    DocumentationPageInput, DocumentationReceiptInput, EffectOperation, FixtureCase, FixtureClass,
    FixturePayload, FixtureSuite, FsmDenialRowInput, FsmEventInput, FsmGuardInput, FsmReceiptInput,
    FsmSideEffectInput, FsmSpecInput, FsmStateInput, FsmTraceStepInput, FsmTransitionRowInput,
    FunctionAddressCaveat, FunctionAddressDependencyEdge, FunctionAddressDependencyIdentityKind,
    FunctionAddressProducer, FunctionAddressReceiptInput, FunctionAddressRecord,
    FunctionAddressSourceEnvelope, FunctionAddressSpan, FunctionAddressValenceProjection,
    HandlerDescriptorInput, HandlerReceiptInput, IdentityDomain, IdentityUseInput,
    MANIFEST_EVALUATOR_DESCRIPTOR, MANIFEST_EVALUATOR_PROFILE, ManifestGenerationInput,
    ManifestGenerationReceipt, ManifestGenerationValidation, NICKEL_SANDBOX_SOURCE_PATH,
    NickelAdapterReceiptInput, NickelSandboxEvidence, NickelSandboxOutcome, NickelSandboxProfile,
    PRESERVES_CANONICAL_CODEC, PRESERVES_LEGACY_PREFIXED_JSON_CODEC, PRESERVES_PACKAGE_VERSION,
    PRESERVES_SPEC_VERSION, ProjectionIdentity, ReceiptBody, ReceiptEnvelope, ReceiptIdentity,
    ReplayProfileInput, SourceEnvelopeInput, SourceSpanSet, StackEvidenceInput,
    SteelAdapterReceiptInput, SteelExecutedProbeEvidence, SteelExecutionEvidence,
    SteelHostFunctionInput, SteelOutputClass, TargetArtifactInput, UnsupportedClaimInput,
    ValenceExportConformance, ValenceReceiptExportEnvelope, ValenceReceiptExportInput,
    ValidationVerdict, WireIdentity, WireReceiptEnvelope, admit_valence_conformance,
    build_manifest_generation_receipt, compute_identity, docs_config_identity,
    evaluate_docs_action, export_function_address_valence_projection, export_valence_receipt,
    fixture_expected_canonical_artifact, function_address_projection_json, nickel_sandbox_profile,
    receipt_json, render_documentation_projection, resolve_steel_dto_contract,
    validate_docs_config, validate_documentation_render_output, validate_fixture_suite,
    validate_function_address_receipt, validate_manifest_generation_receipt,
};
use nickel_preserves_adapter::{AdaptMode, NickelPreservesCommand, run_nickel_preserves_command};
use nickel_sandbox::{
    NickelSandboxFile, NickelSandboxRequest, inheritable_sandbox_data_file, resolve_command,
    run_nickel_sandbox, runtime_closure_paths, validate_enforcement_command_paths,
};
use root_io::{RootCapability, RootIoError};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use std::io::{Seek, SeekFrom, Write};
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, Output};
use steel_runner::{
    STEEL_RUNNER_DESCRIPTOR, STEEL_RUNNER_PROFILE, SteelProbeRequest, run_steel_probe,
};

const DEFAULT_ROOT: &str = ".";
const DEFAULT_MANIFEST: &str = "fixtures/manifest.json";
const DEFAULT_MANIFEST_SOURCE: &str = "fixtures/manifest.ncl";
const DEFAULT_MANIFEST_CONTRACT: &str = "fixtures/manifest-contract.ncl";
const DEFAULT_MANIFEST_GENERATION_RECEIPT: &str = "fixtures/manifest-generation-receipt.json";
const DEFAULT_NICKEL_PRESERVES_PROFILE: &str = "adapters/nickel-preserves/generated/profile.json";
const MANIFEST_SCHEMA_VERSION: u32 = 2;
const DEFAULT_UPDATE_REPORT: &str = "target/kamacite-golden-update-receipt.json";
const DEFAULT_VALENCE_EXPORT_UPDATE_REPORT: &str =
    "target/kamacite-valence-export-update-receipt.json";
const DEFAULT_PRESERVES_MIGRATION_REPORT: &str = "target/kamacite-preserves-migration-report.json";
const PRESERVES_MIGRATION_SCHEMA_VERSION: &str = "kamacite.preserves-migration.v1";
const PRESERVES_MIGRATION_MAX_PASSES: usize = 4;
const DEFAULT_DOCS_CONFIG: &str = "docs/site.ncl";
const NICKEL_EXPORT_ARGUMENT: &str = "export";
const NICKEL_VERSION_ARGUMENT: &str = "--version";
const NICKEL_SOURCE_MEMFD_NAME: &str = "kamacite-nickel-source";
const NICKEL_IMPORT_MEMFD_NAME: &str = "kamacite-nickel-import";
const DEFAULT_DOCS_CONTRACT: &str = "docs/docs-config-contract.ncl";
const DOCS_CONFIG_IMPORT_PREAMBLE: &str =
    "let DocsConfig = import \"docs-config-contract.ncl\" in\n";
const DOCS_MEMFD_NAME: &str = "kamacite-docs-config";
const DOCS_ACTION_ARTIFACT_CLASS: &str = "documentation-action";
const DOCS_ACTION_CHECKED_LAW: &str =
    "successful documentation actions complete their named checked work";
const DOCS_ACTION_NON_CLAIMS: &[&str] = &[
    "not rendered-output authority",
    "not source-language semantic equivalence",
];
const REQUIREMENT_DOCS_CONFIG: &str = "r[kamacite.findings.docs_commands.config_validation]";
const REQUIREMENT_DOCS_COMPLETION: &str = "r[kamacite.findings.docs_commands.action_completion]";
const REQUIREMENT_DOCS_NO_OP: &str = "r[kamacite.findings.docs_commands.no_op_rejection]";
const REQUIREMENT_DOCS_RECEIPTS: &str = "r[kamacite.findings.docs_commands.receipts]";
const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const DOMAIN_SOURCE: &str = "source";
const DOMAIN_CANONICAL_VALUE: &str = "canonical-value";
const DOMAIN_PROJECTION: &str = "projection";
const DOMAIN_TARGET_ARTIFACT: &str = "target-artifact";
const DOMAIN_ADAPTER: &str = "adapter";
const DOMAIN_EVALUATOR: &str = "evaluator";
const DOMAIN_COMPILER: &str = "compiler";
const DOMAIN_RECEIPT: &str = "receipt";
const DOMAIN_FUNCTION_ADDRESS: &str = "function-address";
const DOMAIN_DEPENDENCY_CLOSURE: &str = "dependency-closure";
const DOMAIN_ALGEBRAIC_EFFECT: &str = "algebraic-effect";
const DOMAIN_EFFECT_OPERATION: &str = "effect-operation";
const DOMAIN_EFFECT_LOG: &str = "effect-log";
const DOMAIN_FSM_SPEC: &str = "fsm-spec";
const DOMAIN_FSM_UNIVERSE: &str = "fsm-universe";
const DOMAIN_FSM_COVERAGE: &str = "fsm-coverage";
const DOMAIN_DOCUMENTATION_IR: &str = "documentation-ir";
const DOMAIN_DOCUMENTATION_OUTPUT: &str = "documentation-output";
const DOMAIN_DOCUMENTATION_FACTS: &str = "documentation-facts";
const DOMAIN_DIALECT_PAYLOAD: &str = "dialect-payload";
const DOMAIN_EVIDENCE_CHAIN: &str = "evidence-chain";
const DEFAULT_NICKEL_EVALUATOR_COMMAND: &str = "nickel";
const DEFAULT_NICKEL_SANDBOX_COMMAND: &str = "bwrap";
const DEFAULT_NICKEL_LIMIT_COMMAND: &str = "prlimit";
const NICKEL_COMMAND_EXPORT: &str = "export";
const NICKEL_COMMAND_VERSION: &str = "--version";
const NICKEL_OPTION_FORMAT: &str = "--format";
const NICKEL_FORMAT_JSON: &str = "json";
const MANIFEST_MEMFD_NAME: &str = "kamacite-manifest-generation";
const PROC_SELF_FD_PREFIX: &str = "/proc/self/fd/";
const NICKEL_EVALUATOR_DESCRIPTOR_PREFIX: &str = "nickel-reference-cli:";
const NICKEL_EXECUTION_MODE_REFERENCE_EXPORT: &str = "reference-export";
const NICKEL_EXECUTION_MODE_STORED_EVIDENCE: &str = "stored-evidence";
const NICKEL_EXECUTION_MODE_SANDBOXED_REFERENCE_EXPORT: &str = "sandboxed-reference-export";
const NICKEL_EXECUTION_MODE_STORED_SANDBOX_EVIDENCE: &str = "stored-sandbox-evidence";
const DEFAULT_STEEL_EVALUATOR_DESCRIPTOR: &str = STEEL_RUNNER_DESCRIPTOR;
const STEEL_EXECUTION_MODE_DTO_PROBE: &str = "dto-probe";
const STEEL_EXECUTION_MODE_STORED_EVIDENCE: &str = "stored-evidence";
const STEEL_DTO_INPUT_PATH: &str = "fixtures/adapters/steel/dto-input.json";
const GOLDEN_STATUS_MATCHED: &str = "matched";
const GOLDEN_STATUS_UPDATED: &str = "updated";
const GOLDEN_STATUS_NOT_DECLARED: &str = "not-declared";
const GOLDEN_STATUS_MISSING_RESULT: &str = "missing-result";
const GOLDEN_STATUS_MISSING_RECEIPT: &str = "missing-receipt";
const GOLDEN_STATUS_MISSING_GOLDEN: &str = "missing-golden";
const GOLDEN_STATUS_MISMATCHED: &str = "mismatched";
const FUNCTION_ADDRESS_PRODUCER_ROLE_OPTION: &str = "producer-role";
const FUNCTION_ADDRESS_PRODUCER_NON_CLAIM_CORRECTNESS: &str = "not producer correctness";
const FUNCTION_ADDRESS_PRODUCER_NON_CLAIM_SEMANTIC_CLOSURE: &str =
    "not semantic dependency closure";
const STALE_FSM_CANONICAL_FIXTURE_ID: &str = "fsm-spec.negative.stale-identity";
const STALE_ALGEBRAIC_CANONICAL_FIXTURE_ID: &str = "algebraic.negative.stale-canonical-identity";
const AUTHORITATIVE_DOCUMENTATION_IR_FIXTURE_ID: &str = "docs-ir.positive.valid-page";

#[derive(Debug)]
struct CliError(String);

impl Display for CliError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

impl From<RootIoError> for CliError {
    fn from(error: RootIoError) -> Self {
        Self(error.to_string())
    }
}

type CliResult<T> = Result<T, CliError>;

#[derive(Clone, Debug)]
enum Command {
    Validate(ValidateArgs),
    ValidateNickelAdapter(NickelAdapterArgs),
    ValidateSteelAdapter(SteelAdapterArgs),
    AdaptNickelExport(NickelPreservesCommand),
    HashFile(HashArgs),
    CheckManifestProjection(ManifestProjectionArgs),
    GenerateManifest(GenerateManifestArgs),
    ExportValenceReceipt(ValenceExportArgs),
    EmitFunctionAddressReceipt(FunctionAddressReceiptArgs),
    MigratePreserves(MigratePreservesArgs),
    Docs(DocsArgs),
    Help,
}

#[derive(Clone, Debug)]
struct ValidateArgs {
    root: PathBuf,
    manifest: PathBuf,
    manifest_source: PathBuf,
    manifest_contract: PathBuf,
    manifest_generation_receipt: PathBuf,
    update_golden: bool,
    update_report: PathBuf,
}

#[derive(Clone, Debug)]
struct NickelAdapterArgs {
    validate: ValidateArgs,
    evaluator_command: PathBuf,
    sandbox_command: PathBuf,
    limit_command: PathBuf,
}

#[derive(Clone, Debug)]
struct SteelAdapterArgs {
    validate: ValidateArgs,
    evaluator_descriptor: String,
}

#[derive(Clone, Debug)]
struct NickelExecutionRequest {
    evaluator_command: PathBuf,
    sandbox_command: PathBuf,
    limit_command: PathBuf,
}

#[derive(Clone, Debug)]
struct NickelExecutionShell {
    evaluator_command: PathBuf,
    sandbox_command: PathBuf,
    limit_command: PathBuf,
    runtime_closure_paths: Vec<PathBuf>,
}

#[derive(Clone, Debug)]
struct SteelExecutionShell {
    evaluator_descriptor: String,
}

#[derive(Clone, Debug)]
struct HashArgs {
    domain: IdentityDomain,
    path: PathBuf,
}

#[derive(Clone, Debug)]
struct ManifestProjectionArgs {
    source: PathBuf,
    contract: PathBuf,
    projection: PathBuf,
    evaluator_command: PathBuf,
}

#[derive(Clone, Debug)]
struct GenerateManifestArgs {
    root: PathBuf,
    source: PathBuf,
    contract: PathBuf,
    projection: PathBuf,
    receipt: PathBuf,
    evaluator_command: PathBuf,
    update: bool,
}

#[derive(Clone, Debug)]
struct ValenceExportArgs {
    root: PathBuf,
    receipt: PathBuf,
    conformance: PathBuf,
    source_spans: Option<PathBuf>,
    golden: Option<PathBuf>,
    update_golden: bool,
    update_report: PathBuf,
}

#[derive(Clone, Debug)]
struct FunctionAddressReceiptArgs {
    root: PathBuf,
    report: PathBuf,
    sources: BTreeMap<String, PathBuf>,
    out: Option<PathBuf>,
    valence_out: Option<PathBuf>,
}

#[derive(Clone, Debug)]
struct MigratePreservesArgs {
    root: PathBuf,
    manifest: PathBuf,
    manifest_source: PathBuf,
    update: bool,
    report: PathBuf,
}

#[derive(Clone, Debug)]
struct DocsArgs {
    action: DocsActionKind,
    root: PathBuf,
    config: PathBuf,
    contract: PathBuf,
    manifest: PathBuf,
    evaluator_command: PathBuf,
    format: Option<String>,
    profile: Option<String>,
    target: Option<String>,
    dry_run: bool,
}

#[derive(Debug, Serialize)]
struct DocsCommandReport {
    passed: bool,
    state: DocsCompletionState,
    action: DocsActionKind,
    config_path: String,
    config_schema_version: u32,
    config_source_identity: DocumentationFactsIdentity,
    config_contract_identity: WireIdentity,
    evaluator_identity: WireIdentity,
    projection_identity: WireIdentity,
    format: Option<String>,
    profile: Option<String>,
    target: Option<String>,
    dry_run: bool,
    completed_work_items: usize,
    documentation_ir_identity: Option<WireIdentity>,
    output_identity: Option<DocumentationOutputIdentity>,
    mutated: bool,
    executed: bool,
    diagnostics: Vec<String>,
    receipt: Option<ReceiptEnvelope>,
}

#[derive(Clone, Debug, Deserialize)]
struct OctetFunctionAddressReport {
    #[serde(default)]
    canonicalizer: String,
    #[serde(default)]
    tool: String,
    producer: OctetFunctionAddressProducer,
    source_envelopes: Vec<OctetFunctionAddressSourceEnvelope>,
    #[serde(default, alias = "functions")]
    records: Vec<OctetFunctionAddressRecord>,
    #[serde(default)]
    dependency_edges: Vec<FunctionAddressDependencyEdge>,
    #[serde(default)]
    caveats: Vec<FunctionAddressCaveat>,
    #[serde(default)]
    semantic_claims: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct OctetFunctionAddressProducer {
    name: String,
    #[serde(default)]
    role: String,
    #[serde(default)]
    canonicalizer: String,
    #[serde(default)]
    tool_version: String,
    #[serde(default)]
    build_identity: Option<OctetIdentity>,
    #[serde(default)]
    normative_options: BTreeMap<String, String>,
    #[serde(default)]
    non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct OctetFunctionAddressSourceEnvelope {
    #[serde(default)]
    key: String,
    #[serde(default, alias = "display_path")]
    path: String,
    language: String,
    source_identity: OctetIdentity,
}

#[derive(Clone, Debug, Deserialize)]
struct OctetFunctionAddressRecord {
    #[serde(default, alias = "locator")]
    path: String,
    #[serde(default, alias = "display_name")]
    name: String,
    #[serde(default)]
    source_key: String,
    #[serde(default)]
    start_byte: u64,
    #[serde(default)]
    end_byte: u64,
    #[serde(default)]
    span: Option<FunctionAddressSpan>,
    function_address: OctetIdentity,
    #[serde(default, alias = "dependency_closure")]
    dependency_set_hash: Option<OctetIdentity>,
    #[serde(default)]
    dependency_identity_kind: Option<String>,
    #[serde(default)]
    caveats: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum OctetIdentity {
    Text(String),
    Structured(WireIdentity),
}

#[derive(Debug, Serialize)]
struct FunctionAddressReceiptCommandReport {
    passed: bool,
    diagnostics: Vec<kamacite_core::Diagnostic>,
    receipt_identity: Option<ReceiptIdentity>,
    receipt_path: Option<String>,
    valence_projection_path: Option<String>,
    valence_projection: Option<FunctionAddressValenceProjection>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestProjection {
    schema_version: u32,
    authority_non_claims: Vec<String>,
    fixtures: Vec<FixtureProjection>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureProjection {
    id: String,
    fixture_class: FixtureClass,
    artifact_class: ArtifactClass,
    requirement_ids: Vec<String>,
    non_claims: Vec<String>,
    expected_verdict: ValidationVerdict,
    checked_laws: Vec<String>,
    golden_receipt: Option<String>,
    payload: PayloadProjection,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AstGrepSourceProjection {
    path: String,
    identity: WireIdentity,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AstGrepCodemodProjection {
    recipe_path: String,
    recipe_identity: WireIdentity,
    before_source_path: String,
    before_source_identity: WireIdentity,
    after_source_path: String,
    after_source_identity: WireIdentity,
    dry_run: bool,
    post_check_commands: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
enum PayloadProjection {
    SourceEnvelope {
        schema_version: u32,
        language: String,
        media_type: String,
        parser_identity: String,
        source_path: String,
        declared_identity: WireIdentity,
        aliases: Vec<String>,
    },
    CanonicalValue {
        schema_version: u32,
        canonical_path: String,
        declared_identity: WireIdentity,
        projection_identity: Option<WireIdentity>,
        projection_claims_authority: bool,
        aliases: Vec<String>,
    },
    AdapterReceipt(AdapterReceiptInput),
    AstGrepAudit {
        schema_version: String,
        profile_path: String,
        profile_identity: WireIdentity,
        rule_bundle_path: String,
        rule_bundle_identity: WireIdentity,
        source_scope_identity: WireIdentity,
        findings_identity: WireIdentity,
        valence_projection_path: String,
        valence_projection_identity: WireIdentity,
        language_profiles: Vec<AstGrepLanguageProfile>,
        rules: Vec<AstGrepRuleCoverage>,
        sources: Vec<AstGrepSourceProjection>,
        findings: Vec<AstGrepFinding>,
        verification_role: String,
        codemod: Box<Option<AstGrepCodemodProjection>>,
        non_claims: Vec<String>,
    },
    NickelAdapterReceipt {
        schema_version: u32,
        adapter_kind: String,
        source_path: Option<String>,
        declared_source_identity: Option<WireIdentity>,
        evaluator_profile: String,
        evaluator_descriptor: String,
        evaluator_identity: Option<WireIdentity>,
        export_path: Option<String>,
        projection_identity: Option<WireIdentity>,
        declared_capabilities: Vec<String>,
        observed_effects: Vec<String>,
        #[serde(default)]
        import_paths: Vec<String>,
        #[serde(default)]
        sandbox_profile: Option<NickelSandboxProfile>,
        #[serde(default)]
        sandbox_evidence: Box<Option<NickelSandboxEvidence>>,
        claims_canonical_ir: bool,
        checked_laws: Vec<String>,
        execution_mode: String,
    },
    SteelAdapterReceipt {
        schema_version: u32,
        adapter_kind: String,
        source_path: Option<String>,
        declared_source_identity: Option<WireIdentity>,
        evaluator_profile: String,
        evaluator_descriptor: String,
        evaluator_identity: Option<WireIdentity>,
        module_descriptor_path: String,
        module_identity: Box<Option<WireIdentity>>,
        input_schema: String,
        output_schema: String,
        host_functions: Vec<SteelHostFunctionInput>,
        observed_host_functions: Vec<String>,
        declared_builtin_modules: Vec<String>,
        declared_imports: Vec<String>,
        requested_imports: Vec<String>,
        declared_capabilities: Vec<String>,
        observed_effects: Vec<String>,
        effect_class: String,
        output_path: Option<String>,
        output_projection_identity: Box<Option<WireIdentity>>,
        output_class: SteelOutputClass,
        claims_canonical_ir: bool,
        checked_laws: Vec<String>,
        execution_mode: String,
    },
    TargetArtifact {
        target_kind: String,
        output_path: String,
        declared_identity: WireIdentity,
        claims_canonical_ir: bool,
        metadata: BTreeMap<String, String>,
    },
    UnsupportedClaim(UnsupportedClaimInput),
    IdentityUse(IdentityUseInput),
    ApplicationHostBinding(Box<ApplicationHostBindingInput>),
    CapabilityShell(CapabilityShellInput),
    StackEvidence(StackEvidenceInput),
    FsmSpec {
        schema_version: u32,
        fsm_id: String,
        canonical_path: String,
        source_envelope_identity: Box<Option<WireIdentity>>,
        declared_identity: WireIdentity,
        states: Vec<FsmStateInput>,
        events: Vec<FsmEventInput>,
        initial_state: String,
        terminal_states: Vec<String>,
        intentionally_unreachable_states: Vec<String>,
        determinism_policy: String,
        totality_policy: String,
        guards: Vec<FsmGuardInput>,
        side_effects: Vec<FsmSideEffectInput>,
        transitions: Vec<FsmTransitionRowInput>,
        denials: Vec<FsmDenialRowInput>,
        checked_laws: Vec<String>,
        non_claims: Vec<String>,
    },
    FsmReceipt {
        schema_version: u32,
        receipt_kind: String,
        canonical_path: String,
        descriptor_fixture_id: String,
        spec_identity: WireIdentity,
        state_universe_identity: WireIdentity,
        event_universe_identity: WireIdentity,
        transition_coverage_identity: WireIdentity,
        denial_coverage_identity: WireIdentity,
        adapter_identity: WireIdentity,
        dependency_identities: Vec<WireIdentity>,
        octet_receipt_identities: Vec<WireIdentity>,
        valence_export_identity: Box<Option<WireIdentity>>,
        checked_laws: Vec<String>,
        diagnostics: Vec<String>,
        non_claims: Vec<String>,
        semantic_claims: Vec<String>,
        declared_receipt_identity: Box<Option<WireIdentity>>,
        trace_start_state: Box<Option<String>>,
        trace_events: Vec<String>,
        trace_steps: Vec<FsmTraceStepInput>,
        trace_final_state: Box<Option<String>>,
    },
    DocumentationIr {
        schema_version: u32,
        canonical_path: String,
        declared_identity: WireIdentity,
        collector_identity: WireIdentity,
        pages: Vec<DocumentationPageInput>,
        target_formats: Vec<String>,
        dependency_identities: Vec<WireIdentity>,
        checked_laws: Vec<String>,
        non_claims: Vec<String>,
    },
    DocumentationOutput {
        schema_version: u32,
        documentation_ir_identity: WireIdentity,
        renderer_identity: WireIdentity,
        target_format: String,
        output_path: String,
        declared_output_identity: WireIdentity,
        claims_canonical_ir: bool,
        checked_laws: Vec<String>,
        non_claims: Vec<String>,
    },
    DocumentationCell {
        schema_version: u32,
        cell_id: String,
        language: String,
        source_path: String,
        source_identity: Box<Option<WireIdentity>>,
        run_profile: String,
        execution_mode: String,
        evaluator_identity: Box<Option<WireIdentity>>,
        declared_capabilities: Vec<String>,
        observed_effects: Vec<String>,
        expected_result: String,
        expected_output_identity: Box<Option<WireIdentity>>,
        observed_output_path: Option<String>,
        observed_output_identity: Box<Option<WireIdentity>>,
        expected_diagnostic: Box<Option<String>>,
        observed_diagnostic: Box<Option<String>>,
        claims_semantic_equivalence: bool,
        checked_laws: Vec<String>,
        non_claims: Vec<String>,
    },
    DocumentationReceipt {
        schema_version: u32,
        receipt_kind: String,
        canonical_path: String,
        documentation_ir_identity: WireIdentity,
        renderer_identity: Box<Option<WireIdentity>>,
        output_identities: Vec<WireIdentity>,
        cell_receipt_identities: Vec<WireIdentity>,
        valence_export_identity: Box<Option<WireIdentity>>,
        declared_receipt_identity: Box<Option<WireIdentity>>,
        diagnostics: Vec<String>,
        semantic_claims: Vec<String>,
        checked_laws: Vec<String>,
        non_claims: Vec<String>,
    },
    ValenceReceiptExport {
        source_receipt_path: String,
        conformance_path: String,
        source_spans_path: Option<String>,
        semantic_claims: Vec<String>,
        declared_export_identity: Option<WireIdentity>,
    },
    AlgebraicEffect {
        schema_version: u32,
        canonical_path: String,
        declared_identity: WireIdentity,
        signatures: Vec<EffectOperation>,
        input_row: Vec<String>,
        row_open: bool,
        claimed_pure: bool,
        handler: Box<Option<HandlerDescriptorInput>>,
        receipt: Box<Option<HandlerReceiptInput>>,
        replay: Box<Option<ReplayProjection>>,
        claimed_host_authority: Vec<String>,
        admitted_host_capabilities: Vec<String>,
    },
}

#[derive(Clone, Debug, Deserialize)]
struct ReplayProjection {
    replay_class: String,
    effect_log_path: Option<String>,
    effect_log_identity: Option<WireIdentity>,
}

impl PayloadProjection {
    fn canonical_path(&self) -> Option<&str> {
        match self {
            Self::CanonicalValue { canonical_path, .. }
            | Self::FsmSpec { canonical_path, .. }
            | Self::FsmReceipt { canonical_path, .. }
            | Self::DocumentationIr { canonical_path, .. }
            | Self::DocumentationReceipt { canonical_path, .. }
            | Self::AlgebraicEffect { canonical_path, .. } => Some(canonical_path),
            _ => None,
        }
    }

    fn declared_canonical_identity(&self) -> Option<WireIdentity> {
        match self {
            Self::CanonicalValue {
                declared_identity, ..
            }
            | Self::FsmSpec {
                declared_identity, ..
            }
            | Self::DocumentationIr {
                declared_identity, ..
            }
            | Self::AlgebraicEffect {
                declared_identity, ..
            } => Some(declared_identity.clone()),
            Self::FsmReceipt {
                declared_receipt_identity,
                ..
            }
            | Self::DocumentationReceipt {
                declared_receipt_identity,
                ..
            } => declared_receipt_identity.as_ref().clone(),
            _ => None,
        }
    }

    fn canonical_identity_domain(&self) -> Option<IdentityDomain> {
        match self {
            Self::CanonicalValue { .. } => Some(IdentityDomain::CanonicalValue),
            Self::FsmSpec { .. } => Some(IdentityDomain::FsmSpec),
            Self::FsmReceipt { .. } | Self::DocumentationReceipt { .. } => {
                Some(IdentityDomain::Receipt)
            }
            Self::DocumentationIr { .. } => Some(IdentityDomain::DocumentationIr),
            Self::AlgebraicEffect { .. } => Some(IdentityDomain::AlgebraicEffect),
            _ => None,
        }
    }
}

struct NickelAdapterProjectionParts<'a> {
    schema_version: u32,
    adapter_kind: &'a str,
    source_path: &'a Option<String>,
    declared_source_identity: &'a Option<WireIdentity>,
    evaluator_profile: &'a str,
    evaluator_descriptor: &'a str,
    evaluator_identity: &'a Option<WireIdentity>,
    export_path: &'a Option<String>,
    projection_identity: &'a Option<WireIdentity>,
    declared_capabilities: &'a [String],
    observed_effects: &'a [String],
    import_paths: &'a [String],
    sandbox_profile: &'a Option<NickelSandboxProfile>,
    sandbox_evidence: &'a Option<NickelSandboxEvidence>,
    claims_canonical_ir: bool,
    checked_laws: &'a [String],
    execution_mode: &'a str,
}

struct SteelAdapterProjectionParts<'a> {
    schema_version: u32,
    adapter_kind: &'a str,
    source_path: &'a Option<String>,
    declared_source_identity: &'a Option<WireIdentity>,
    evaluator_profile: &'a str,
    evaluator_descriptor: &'a str,
    evaluator_identity: &'a Option<WireIdentity>,
    module_descriptor_path: &'a str,
    module_identity: &'a Option<WireIdentity>,
    input_schema: &'a str,
    output_schema: &'a str,
    host_functions: &'a [SteelHostFunctionInput],
    observed_host_functions: &'a [String],
    declared_builtin_modules: &'a [String],
    declared_imports: &'a [String],
    requested_imports: &'a [String],
    declared_capabilities: &'a [String],
    observed_effects: &'a [String],
    effect_class: &'a str,
    output_path: &'a Option<String>,
    output_projection_identity: &'a Option<WireIdentity>,
    output_class: &'a SteelOutputClass,
    claims_canonical_ir: bool,
    checked_laws: &'a [String],
    execution_mode: &'a str,
}

#[derive(Debug, Serialize)]
struct CliReport {
    passed: bool,
    manifest_generation: ManifestGenerationValidation,
    fixture_report: Option<kamacite_core::FixtureSuiteReport>,
    golden_results: Vec<GoldenResult>,
}

#[derive(Debug, Serialize)]
struct GoldenResult {
    fixture_id: String,
    path: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct ManifestProjectionReport {
    passed: bool,
    generated_projection_identity: WireIdentity,
    checked_projection_identity: WireIdentity,
    evaluator_identity: WireIdentity,
    diagnostics: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ManifestGenerationCommandReport {
    passed: bool,
    updated: bool,
    projection_path: String,
    receipt_path: String,
    receipt: ManifestGenerationReceipt,
    diagnostics: Vec<String>,
}

#[derive(Debug, Serialize)]
struct GoldenUpdateReport {
    changed_artifacts: Vec<GoldenUpdate>,
}

#[derive(Debug, Serialize)]
struct GoldenUpdate {
    fixture_id: String,
    path: String,
    old_identity: Option<WireIdentity>,
    new_identity: WireIdentity,
    input_identities: BTreeMap<String, WireIdentity>,
    output_identities: BTreeMap<String, WireIdentity>,
    requirement_ids: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ValenceExportCommandReport {
    passed: bool,
    validation_passed: bool,
    diagnostics: Vec<kamacite_core::Diagnostic>,
    export_identity: Option<ProjectionIdentity>,
    golden_result: Option<ValenceExportGoldenResult>,
    export: Option<ValenceReceiptExportEnvelope>,
}

#[derive(Debug, Serialize)]
struct ValenceExportGoldenResult {
    path: String,
    status: String,
    old_export_identity: Option<ProjectionIdentity>,
    new_export_identity: Option<ProjectionIdentity>,
}

#[derive(Debug, Serialize)]
struct ValenceExportUpdateReport {
    changed_exports: Vec<ValenceExportUpdate>,
}

#[derive(Debug, Serialize)]
struct ValenceExportUpdate {
    path: String,
    source_receipt_identity: ReceiptIdentity,
    old_export_identity: Option<ProjectionIdentity>,
    new_export_identity: ProjectionIdentity,
    checked_laws: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Debug, Serialize)]
struct PreservesMigrationReport {
    schema_version: String,
    legacy_codec: String,
    canonical_codec: String,
    preserves_package_version: String,
    preserves_spec_version: String,
    dry_run: bool,
    mutated: bool,
    converged: bool,
    migration_passes: usize,
    artifacts: Vec<PreservesMigrationArtifact>,
    non_claims: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct PreservesMigrationArtifact {
    fixture_id: String,
    path: String,
    old_identity: Option<WireIdentity>,
    new_identity: Option<WireIdentity>,
    content_changed: bool,
    canonical_packed_size: Option<usize>,
    error: Option<String>,
}

struct PreservesMigrationWrite {
    path: PathBuf,
    bytes: Vec<u8>,
}

struct PreservesMigrationPlan {
    manifest: ManifestProjection,
    artifacts: Vec<PreservesMigrationArtifact>,
    writes: Vec<PreservesMigrationWrite>,
}

struct PreservesMigrationExecution {
    artifacts: Vec<PreservesMigrationArtifact>,
    mutated: bool,
    converged: bool,
    migration_passes: usize,
}

fn main() {
    match run() {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(EXIT_FAILURE);
        }
    }
}

fn run() -> CliResult<i32> {
    let command = parse_command(std::env::args().skip(1).collect())?;
    match command {
        Command::Validate(args) => run_validate(args),
        Command::ValidateNickelAdapter(args) => run_validate_nickel_adapter(args),
        Command::ValidateSteelAdapter(args) => run_validate_steel_adapter(args),
        Command::AdaptNickelExport(args) => run_adapt_nickel_export(&args),
        Command::HashFile(args) => run_hash_file(args),
        Command::CheckManifestProjection(args) => run_check_manifest_projection(args),
        Command::GenerateManifest(args) => run_generate_manifest(args),
        Command::ExportValenceReceipt(args) => run_export_valence_receipt(args),
        Command::EmitFunctionAddressReceipt(args) => run_emit_function_address_receipt(args),
        Command::MigratePreserves(args) => run_migrate_preserves(args),
        Command::Docs(args) => run_docs(args),
        Command::Help => {
            print_help();
            Ok(EXIT_SUCCESS)
        }
    }
}

fn parse_command(args: Vec<String>) -> CliResult<Command> {
    let mut args_iter = args.into_iter();
    let Some(command) = args_iter.next() else {
        return Ok(Command::Validate(default_validate_args()));
    };
    match command.as_str() {
        "validate" => parse_validate_args(args_iter.collect()),
        "validate-nickel-adapter" => parse_nickel_adapter_args(args_iter.collect()),
        "validate-steel-adapter" => parse_steel_adapter_args(args_iter.collect()),
        "adapt" => parse_adapt_command(args_iter.collect()),
        "hash-file" => parse_hash_args(args_iter.collect()),
        "check-manifest-projection" => parse_manifest_projection_args(args_iter.collect()),
        "generate-manifest" => parse_generate_manifest_args(args_iter.collect()),
        "export-valence-receipt" => parse_valence_export_args(args_iter.collect()),
        "emit-function-address-receipt" => parse_function_address_receipt_args(args_iter.collect()),
        "migrate-preserves" => parse_migrate_preserves_args(args_iter.collect()),
        "docs" => parse_docs_args(args_iter.collect()),
        "help" | "--help" | "-h" => Ok(Command::Help),
        unknown => Err(CliError(format!("unknown command: {unknown}"))),
    }
}

fn default_validate_args() -> ValidateArgs {
    ValidateArgs {
        root: PathBuf::from(DEFAULT_ROOT),
        manifest: PathBuf::from(DEFAULT_MANIFEST),
        manifest_source: PathBuf::from(DEFAULT_MANIFEST_SOURCE),
        manifest_contract: PathBuf::from(DEFAULT_MANIFEST_CONTRACT),
        manifest_generation_receipt: PathBuf::from(DEFAULT_MANIFEST_GENERATION_RECEIPT),
        update_golden: false,
        update_report: PathBuf::from(DEFAULT_UPDATE_REPORT),
    }
}

fn parse_validate_args(args: Vec<String>) -> CliResult<Command> {
    let parsed = parse_common_validate_args(args)?;
    Ok(Command::Validate(parsed))
}

fn parse_nickel_adapter_args(args: Vec<String>) -> CliResult<Command> {
    let mut validate_args = Vec::new();
    let mut evaluator_command = PathBuf::from(DEFAULT_NICKEL_EVALUATOR_COMMAND);
    let mut sandbox_command = PathBuf::from(DEFAULT_NICKEL_SANDBOX_COMMAND);
    let mut limit_command = PathBuf::from(DEFAULT_NICKEL_LIMIT_COMMAND);
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--evaluator-command" => {
                evaluator_command = next_path(&mut index, "--evaluator-command")?
            }
            "--sandbox-command" => sandbox_command = next_path(&mut index, "--sandbox-command")?,
            "--limit-command" => limit_command = next_path(&mut index, "--limit-command")?,
            _ => {
                validate_args.push(flag);
                if validate_args
                    .last()
                    .is_some_and(|value| value == "--update-golden")
                {
                    continue;
                }
                if let Some(value) = index.next() {
                    validate_args.push(value);
                }
            }
        }
    }
    let validate = parse_common_validate_args(validate_args)?;
    Ok(Command::ValidateNickelAdapter(NickelAdapterArgs {
        validate,
        evaluator_command,
        sandbox_command,
        limit_command,
    }))
}

fn parse_steel_adapter_args(args: Vec<String>) -> CliResult<Command> {
    let mut validate_args = Vec::new();
    let mut evaluator_descriptor = DEFAULT_STEEL_EVALUATOR_DESCRIPTOR.to_string();
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--evaluator-descriptor" => {
                evaluator_descriptor = next_value(&mut index, "--evaluator-descriptor")?
            }
            _ => {
                validate_args.push(flag);
                if validate_args
                    .last()
                    .is_some_and(|value| value == "--update-golden")
                {
                    continue;
                }
                if let Some(value) = index.next() {
                    validate_args.push(value);
                }
            }
        }
    }
    let validate = parse_common_validate_args(validate_args)?;
    Ok(Command::ValidateSteelAdapter(SteelAdapterArgs {
        validate,
        evaluator_descriptor,
    }))
}

fn parse_adapt_command(args: Vec<String>) -> CliResult<Command> {
    let mut index = args.into_iter();
    let adapter = index
        .next()
        .ok_or_else(|| CliError("adapt requires an adapter name".to_string()))?;
    if adapter != "nickel-export" {
        return Err(CliError(format!("unsupported adapt command: {adapter}")));
    }
    parse_adapt_nickel_export_args(index.collect())
}

fn parse_adapt_nickel_export_args(args: Vec<String>) -> CliResult<Command> {
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut manifest = None;
    let mut selected_export = None;
    let mut artifact = None;
    let mut packed_output = None;
    let mut text_output = None;
    let mut receipt = None;
    let mut profile = PathBuf::from(DEFAULT_NICKEL_PRESERVES_PROFILE);
    let mut canonical_codec = PRESERVES_CANONICAL_CODEC.to_string();
    let mut check = false;
    let mut write = false;
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--root" => root = PathBuf::from(next_value(&mut index, "--root")?),
            "--manifest" => manifest = Some(PathBuf::from(next_value(&mut index, "--manifest")?)),
            "--export" => selected_export = Some(next_value(&mut index, "--export")?),
            "--artifact" => artifact = Some(PathBuf::from(next_value(&mut index, "--artifact")?)),
            "--packed-output" => {
                packed_output = Some(PathBuf::from(next_value(&mut index, "--packed-output")?))
            }
            "--text-output" => {
                text_output = Some(PathBuf::from(next_value(&mut index, "--text-output")?))
            }
            "--receipt" => receipt = Some(PathBuf::from(next_value(&mut index, "--receipt")?)),
            "--profile" => profile = PathBuf::from(next_value(&mut index, "--profile")?),
            "--codec" => canonical_codec = next_value(&mut index, "--codec")?,
            "--check" => check = true,
            "--write" => write = true,
            _ => {
                return Err(CliError(format!(
                    "unknown adapt nickel-export option: {flag}"
                )));
            }
        }
    }
    if check == write {
        return Err(CliError(
            "adapt nickel-export requires exactly one of --check or --write".to_string(),
        ));
    }
    Ok(Command::AdaptNickelExport(NickelPreservesCommand {
        root,
        manifest: manifest
            .ok_or_else(|| CliError("adapt nickel-export requires --manifest".to_string()))?,
        selected_export: selected_export
            .ok_or_else(|| CliError("adapt nickel-export requires --export".to_string()))?,
        artifact: artifact
            .ok_or_else(|| CliError("adapt nickel-export requires --artifact".to_string()))?,
        packed_output: packed_output
            .ok_or_else(|| CliError("adapt nickel-export requires --packed-output".to_string()))?,
        text_output,
        receipt: receipt
            .ok_or_else(|| CliError("adapt nickel-export requires --receipt".to_string()))?,
        profile,
        canonical_codec,
        mode: if check {
            AdaptMode::Check
        } else {
            AdaptMode::Write
        },
    }))
}

fn parse_common_validate_args(args: Vec<String>) -> CliResult<ValidateArgs> {
    let mut parsed = default_validate_args();
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--root" => parsed.root = next_path(&mut index, "--root")?,
            "--manifest" => parsed.manifest = next_path(&mut index, "--manifest")?,
            "--manifest-source" => {
                parsed.manifest_source = next_path(&mut index, "--manifest-source")?
            }
            "--manifest-contract" => {
                parsed.manifest_contract = next_path(&mut index, "--manifest-contract")?
            }
            "--manifest-generation-receipt" => {
                parsed.manifest_generation_receipt =
                    next_path(&mut index, "--manifest-generation-receipt")?
            }
            "--update-golden" => parsed.update_golden = true,
            "--update-report" => parsed.update_report = next_path(&mut index, "--update-report")?,
            unknown => return Err(CliError(format!("unknown validate argument: {unknown}"))),
        }
    }
    Ok(parsed)
}

fn parse_hash_args(args: Vec<String>) -> CliResult<Command> {
    let mut domain: Option<IdentityDomain> = None;
    let mut path: Option<PathBuf> = None;
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--domain" => {
                let value = next_value(&mut index, "--domain")?;
                domain = Some(parse_domain(&value)?);
            }
            "--path" => path = Some(next_path(&mut index, "--path")?),
            unknown => return Err(CliError(format!("unknown hash-file argument: {unknown}"))),
        }
    }
    let Some(domain) = domain else {
        return Err(CliError("hash-file requires --domain".to_string()));
    };
    let Some(path) = path else {
        return Err(CliError("hash-file requires --path".to_string()));
    };
    Ok(Command::HashFile(HashArgs { domain, path }))
}

fn parse_manifest_projection_args(args: Vec<String>) -> CliResult<Command> {
    let mut parsed = ManifestProjectionArgs {
        source: PathBuf::from(DEFAULT_MANIFEST_SOURCE),
        contract: PathBuf::from(DEFAULT_MANIFEST_CONTRACT),
        projection: PathBuf::from(DEFAULT_MANIFEST),
        evaluator_command: PathBuf::from(DEFAULT_NICKEL_EVALUATOR_COMMAND),
    };
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--source" => parsed.source = next_path(&mut index, "--source")?,
            "--contract" => parsed.contract = next_path(&mut index, "--contract")?,
            "--projection" => parsed.projection = next_path(&mut index, "--projection")?,
            "--evaluator-command" => {
                parsed.evaluator_command = next_path(&mut index, "--evaluator-command")?
            }
            unknown => {
                return Err(CliError(format!(
                    "unknown check-manifest-projection argument: {unknown}"
                )));
            }
        }
    }
    Ok(Command::CheckManifestProjection(parsed))
}

fn parse_generate_manifest_args(args: Vec<String>) -> CliResult<Command> {
    let mut parsed = GenerateManifestArgs {
        root: PathBuf::from(DEFAULT_ROOT),
        source: PathBuf::from(DEFAULT_MANIFEST_SOURCE),
        contract: PathBuf::from(DEFAULT_MANIFEST_CONTRACT),
        projection: PathBuf::from(DEFAULT_MANIFEST),
        receipt: PathBuf::from(DEFAULT_MANIFEST_GENERATION_RECEIPT),
        evaluator_command: PathBuf::from(DEFAULT_NICKEL_EVALUATOR_COMMAND),
        update: false,
    };
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--root" => parsed.root = next_path(&mut index, "--root")?,
            "--source" => parsed.source = next_path(&mut index, "--source")?,
            "--contract" => parsed.contract = next_path(&mut index, "--contract")?,
            "--projection" => parsed.projection = next_path(&mut index, "--projection")?,
            "--receipt" => parsed.receipt = next_path(&mut index, "--receipt")?,
            "--evaluator-command" => {
                parsed.evaluator_command = next_path(&mut index, "--evaluator-command")?
            }
            "--update" => parsed.update = true,
            unknown => {
                return Err(CliError(format!(
                    "unknown generate-manifest argument: {unknown}"
                )));
            }
        }
    }
    Ok(Command::GenerateManifest(parsed))
}

fn parse_valence_export_args(args: Vec<String>) -> CliResult<Command> {
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut receipt = None;
    let mut conformance = None;
    let mut source_spans = None;
    let mut golden = None;
    let mut update_golden = false;
    let mut update_report = PathBuf::from(DEFAULT_VALENCE_EXPORT_UPDATE_REPORT);
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--root" => root = next_path(&mut index, "--root")?,
            "--receipt" => receipt = Some(next_path(&mut index, "--receipt")?),
            "--conformance" => conformance = Some(next_path(&mut index, "--conformance")?),
            "--source-spans" => source_spans = Some(next_path(&mut index, "--source-spans")?),
            "--golden" => golden = Some(next_path(&mut index, "--golden")?),
            "--update-golden" => update_golden = true,
            "--update-report" => update_report = next_path(&mut index, "--update-report")?,
            unknown => {
                return Err(CliError(format!(
                    "unknown Valence export argument: {unknown}"
                )));
            }
        }
    }
    let Some(receipt) = receipt else {
        return Err(CliError(
            "export-valence-receipt requires --receipt".to_string(),
        ));
    };
    let Some(conformance) = conformance else {
        return Err(CliError(
            "export-valence-receipt requires --conformance".to_string(),
        ));
    };
    if update_golden && golden.is_none() {
        return Err(CliError(
            "export-valence-receipt --update-golden requires --golden".to_string(),
        ));
    }
    Ok(Command::ExportValenceReceipt(ValenceExportArgs {
        root,
        receipt,
        conformance,
        source_spans,
        golden,
        update_golden,
        update_report,
    }))
}

fn parse_function_address_receipt_args(args: Vec<String>) -> CliResult<Command> {
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut report = None;
    let mut sources = BTreeMap::new();
    let mut out = None;
    let mut valence_out = None;
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--root" => root = next_path(&mut index, "--root")?,
            "--report" => report = Some(next_path(&mut index, "--report")?),
            "--source" => {
                let assignment = next_value(&mut index, "--source")?;
                let (key, path) = parse_source_assignment(&assignment)?;
                sources.insert(key, path);
            }
            "--out" => out = Some(next_path(&mut index, "--out")?),
            "--valence-out" => valence_out = Some(next_path(&mut index, "--valence-out")?),
            unknown => {
                return Err(CliError(format!(
                    "unknown emit-function-address-receipt argument: {unknown}"
                )));
            }
        }
    }
    let Some(report) = report else {
        return Err(CliError(
            "emit-function-address-receipt requires --report".to_string(),
        ));
    };
    if sources.is_empty() {
        return Err(CliError(
            "emit-function-address-receipt requires at least one --source KEY=PATH".to_string(),
        ));
    }
    Ok(Command::EmitFunctionAddressReceipt(
        FunctionAddressReceiptArgs {
            root,
            report,
            sources,
            out,
            valence_out,
        },
    ))
}

fn parse_migrate_preserves_args(args: Vec<String>) -> CliResult<Command> {
    let mut parsed = MigratePreservesArgs {
        root: PathBuf::from(DEFAULT_ROOT),
        manifest: PathBuf::from(DEFAULT_MANIFEST),
        manifest_source: PathBuf::from(DEFAULT_MANIFEST_SOURCE),
        update: false,
        report: PathBuf::from(DEFAULT_PRESERVES_MIGRATION_REPORT),
    };
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--root" => parsed.root = next_path(&mut index, "--root")?,
            "--manifest" => parsed.manifest = next_path(&mut index, "--manifest")?,
            "--manifest-source" => {
                parsed.manifest_source = next_path(&mut index, "--manifest-source")?
            }
            "--update" => parsed.update = true,
            "--report" => parsed.report = next_path(&mut index, "--report")?,
            unknown => {
                return Err(CliError(format!(
                    "unknown migrate-preserves argument: {unknown}"
                )));
            }
        }
    }
    Ok(Command::MigratePreserves(parsed))
}

fn parse_docs_args(args: Vec<String>) -> CliResult<Command> {
    let mut index = args.into_iter();
    let Some(action) = index.next() else {
        return Err(CliError(
            "docs requires an action: collect, check, render, run-cells, or update".to_string(),
        ));
    };
    let action = match action.as_str() {
        "collect" => DocsActionKind::Collect,
        "check" => DocsActionKind::Check,
        "render" => DocsActionKind::Render,
        "run-cells" => DocsActionKind::RunCells,
        "update" => DocsActionKind::Update,
        unknown => return Err(CliError(format!("unknown docs action: {unknown}"))),
    };
    let mut parsed = DocsArgs {
        action,
        root: PathBuf::from(DEFAULT_ROOT),
        config: PathBuf::from(DEFAULT_DOCS_CONFIG),
        contract: PathBuf::from(DEFAULT_DOCS_CONTRACT),
        manifest: PathBuf::from(DEFAULT_MANIFEST),
        evaluator_command: PathBuf::from(DEFAULT_NICKEL_EVALUATOR_COMMAND),
        format: None,
        profile: None,
        target: None,
        dry_run: false,
    };
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--root" => parsed.root = next_path(&mut index, "--root")?,
            "--config" => parsed.config = next_path(&mut index, "--config")?,
            "--contract" => parsed.contract = next_path(&mut index, "--contract")?,
            "--manifest" => parsed.manifest = next_path(&mut index, "--manifest")?,
            "--evaluator-command" => {
                parsed.evaluator_command = next_path(&mut index, "--evaluator-command")?;
            }
            "--format" => parsed.format = Some(next_value(&mut index, "--format")?),
            "--profile" => parsed.profile = Some(next_value(&mut index, "--profile")?),
            "--target" => parsed.target = Some(next_value(&mut index, "--target")?),
            "--dry-run" => parsed.dry_run = true,
            unknown => return Err(CliError(format!("unknown docs argument: {unknown}"))),
        }
    }
    Ok(Command::Docs(parsed))
}

fn parse_source_assignment(value: &str) -> CliResult<(String, PathBuf)> {
    let Some((key, path)) = value.split_once('=') else {
        return Err(CliError(
            "--source must use KEY=PATH syntax for function-address receipts".to_string(),
        ));
    };
    if key.is_empty() {
        return Err(CliError("--source key cannot be empty".to_string()));
    }
    if path.is_empty() {
        return Err(CliError("--source path cannot be empty".to_string()));
    }
    Ok((key.to_string(), PathBuf::from(path)))
}

fn next_value(index: &mut impl Iterator<Item = String>, flag: &str) -> CliResult<String> {
    index
        .next()
        .ok_or_else(|| CliError(format!("{flag} requires a value")))
}

fn next_path(index: &mut impl Iterator<Item = String>, flag: &str) -> CliResult<PathBuf> {
    Ok(PathBuf::from(next_value(index, flag)?))
}

fn parse_domain(value: &str) -> CliResult<IdentityDomain> {
    match value {
        DOMAIN_SOURCE => Ok(IdentityDomain::Source),
        DOMAIN_CANONICAL_VALUE => Ok(IdentityDomain::CanonicalValue),
        DOMAIN_PROJECTION => Ok(IdentityDomain::Projection),
        DOMAIN_TARGET_ARTIFACT => Ok(IdentityDomain::TargetArtifact),
        DOMAIN_ADAPTER => Ok(IdentityDomain::Adapter),
        DOMAIN_EVALUATOR => Ok(IdentityDomain::Evaluator),
        DOMAIN_COMPILER => Ok(IdentityDomain::Compiler),
        DOMAIN_RECEIPT => Ok(IdentityDomain::Receipt),
        DOMAIN_FUNCTION_ADDRESS => Ok(IdentityDomain::FunctionAddress),
        DOMAIN_DEPENDENCY_CLOSURE => Ok(IdentityDomain::DependencyClosure),
        DOMAIN_ALGEBRAIC_EFFECT => Ok(IdentityDomain::AlgebraicEffect),
        DOMAIN_EFFECT_OPERATION => Ok(IdentityDomain::EffectOperation),
        DOMAIN_EFFECT_LOG => Ok(IdentityDomain::EffectLog),
        DOMAIN_FSM_SPEC => Ok(IdentityDomain::FsmSpec),
        DOMAIN_FSM_UNIVERSE => Ok(IdentityDomain::FsmUniverse),
        DOMAIN_FSM_COVERAGE => Ok(IdentityDomain::FsmCoverage),
        DOMAIN_DOCUMENTATION_IR => Ok(IdentityDomain::DocumentationIr),
        DOMAIN_DOCUMENTATION_OUTPUT => Ok(IdentityDomain::DocumentationOutput),
        DOMAIN_DOCUMENTATION_FACTS => Ok(IdentityDomain::DocumentationFacts),
        DOMAIN_DIALECT_PAYLOAD => Ok(IdentityDomain::DialectPayload),
        DOMAIN_EVIDENCE_CHAIN => Ok(IdentityDomain::EvidenceChain),
        unknown => Err(CliError(format!("unknown identity domain: {unknown}"))),
    }
}

// r[impl kamacite.findings.docs_commands.config_validation]
// r[impl kamacite.findings.docs_commands.action_completion]
// r[impl kamacite.findings.docs_commands.no_op_rejection]
// r[impl kamacite.findings.docs_commands.receipts]
fn run_docs(args: DocsArgs) -> CliResult<i32> {
    if args
        .config
        .extension()
        .and_then(|extension| extension.to_str())
        != Some("ncl")
    {
        return Err(CliError(
            "docs config must be a Nickel .ncl file".to_string(),
        ));
    }
    let root = RootCapability::open(&args.root)?;
    let config_bytes = root.read(&args.config)?;
    let contract_bytes = root.read(&args.contract)?;
    let evaluator_descriptor =
        nickel_evaluator_descriptor(&run_nickel_version(&args.evaluator_command)?)?;
    let evaluator_identity =
        compute_identity(IdentityDomain::Evaluator, evaluator_descriptor.as_bytes());
    let projection_bytes =
        run_nickel_docs_export(&args.evaluator_command, &config_bytes, &contract_bytes)?;
    let config: DocsConfigProjection =
        serde_json::from_slice(&projection_bytes).map_err(|error| {
            CliError(format!(
                "docs config projection is malformed or contract-invalid: {error}"
            ))
        })?;
    let config_diagnostics = validate_docs_config(&config);
    if !config_diagnostics.is_empty() {
        let messages = config_diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        return Err(CliError(format!(
            "docs config validation failed: {messages}"
        )));
    }

    let config_source_identity = docs_config_identity(&config_bytes);
    let config_contract_identity = compute_identity(IdentityDomain::Source, &contract_bytes);
    let projection_identity = compute_identity(IdentityDomain::Projection, &projection_bytes);
    let work = docs_action_work(&root, &args)?;
    let outcome = evaluate_docs_action(
        &config,
        DocsActionEvidenceInput {
            action: args.action.clone(),
            format: args.format.clone(),
            profile: args.profile.clone(),
            target: args.target.clone(),
            dry_run: args.dry_run,
            completed_work_items: work.completed_work_items,
            documentation_ir_identity: work.documentation_ir_identity.clone(),
            output_identity: work.output_identity.clone(),
            executed: false,
            mutated: false,
            diagnostics: work.diagnostics,
        },
    );
    let receipt = docs_action_receipt(
        &outcome,
        DocsReceiptContext {
            config_source_identity: &config_source_identity,
            config_contract_identity: &config_contract_identity,
            evaluator_identity: &evaluator_identity,
            projection_identity: &projection_identity,
            format: args.format.as_deref(),
            profile: args.profile.as_deref(),
            target: args.target.as_deref(),
        },
    )?;
    let report = DocsCommandReport {
        passed: outcome.passed,
        state: outcome.state,
        action: outcome.action,
        config_path: path_display(&args.config),
        config_schema_version: config.schema_version,
        config_source_identity,
        config_contract_identity,
        evaluator_identity,
        projection_identity,
        format: args.format,
        profile: args.profile,
        target: args.target,
        dry_run: args.dry_run,
        completed_work_items: outcome.completed_work_items,
        documentation_ir_identity: outcome.documentation_ir_identity,
        output_identity: outcome.output_identity,
        mutated: outcome.mutated,
        executed: outcome.executed,
        diagnostics: outcome.diagnostics,
        receipt,
    };
    print_json_report(&report, "docs command report")?;
    if report.passed {
        Ok(EXIT_SUCCESS)
    } else {
        Ok(EXIT_FAILURE)
    }
}

#[derive(Debug)]
struct DocsActionWork {
    completed_work_items: usize,
    documentation_ir_identity: Option<WireIdentity>,
    output_identity: Option<DocumentationOutputIdentity>,
    diagnostics: Vec<String>,
}

fn docs_action_work(root: &RootCapability, args: &DocsArgs) -> CliResult<DocsActionWork> {
    if matches!(args.action, DocsActionKind::RunCells) {
        return Ok(DocsActionWork {
            completed_work_items: 0,
            documentation_ir_identity: None,
            output_identity: None,
            diagnostics: Vec::new(),
        });
    }
    let suite = load_docs_fixture_suite(root, args)?;
    let authoritative = suite
        .fixtures
        .iter()
        .find(|fixture| fixture.id == AUTHORITATIVE_DOCUMENTATION_IR_FIXTURE_ID)
        .ok_or_else(|| CliError("authoritative documentation IR fixture is missing".to_string()))?;
    let FixturePayload::DocumentationIr(documentation_ir) = &authoritative.payload else {
        return Err(CliError(
            "authoritative documentation fixture does not contain documentation IR".to_string(),
        ));
    };
    let documentation_ir_identity = Some(documentation_ir.declared_identity.clone());
    let validation_suite = if matches!(args.action, DocsActionKind::Check) {
        FixtureSuite {
            fixtures: suite
                .fixtures
                .iter()
                .filter(|fixture| is_documentation_fixture(fixture))
                .cloned()
                .collect(),
        }
    } else {
        FixtureSuite {
            fixtures: vec![authoritative.clone()],
        }
    };
    let validation = validate_fixture_suite(&validation_suite);
    let mut diagnostics = Vec::new();
    if !validation.passed {
        diagnostics.extend(
            validation
                .results
                .iter()
                .filter(|result| !result.expectation_met)
                .flat_map(|result| result.diagnostics.iter())
                .map(|diagnostic| diagnostic.message.clone()),
        );
    }
    let mut completed_work_items = validation.results.len();
    let mut output_identity = None;
    if matches!(args.action, DocsActionKind::Render) {
        let format = args.format.as_deref().unwrap_or("markdown");
        match render_documentation_projection(documentation_ir, format) {
            Ok(rendered_bytes) => {
                let declared_output = suite.fixtures.iter().find_map(|fixture| {
                    let FixturePayload::DocumentationOutput(output) = &fixture.payload else {
                        return None;
                    };
                    (output.documentation_ir_identity == documentation_ir.declared_identity
                        && output.target_format == format)
                        .then_some(output.as_ref())
                });
                match declared_output {
                    Some(output) => match validate_documentation_render_output(
                        &rendered_bytes,
                        &output.output_bytes,
                    ) {
                        Ok(identity) => {
                            output_identity = Some(identity);
                            completed_work_items += 1;
                        }
                        Err(diagnostic) => diagnostics.push(diagnostic.message),
                    },
                    None => diagnostics
                        .push("documentation render has no declared output projection".to_string()),
                }
            }
            Err(diagnostic) => diagnostics.push(diagnostic.message),
        }
    }
    Ok(DocsActionWork {
        completed_work_items,
        documentation_ir_identity,
        output_identity,
        diagnostics,
    })
}

fn load_docs_fixture_suite(root: &RootCapability, args: &DocsArgs) -> CliResult<FixtureSuite> {
    let manifest_bytes = root.read(&args.manifest)?;
    let validate_args = ValidateArgs {
        root: args.root.clone(),
        manifest: args.manifest.clone(),
        manifest_source: PathBuf::from(DEFAULT_MANIFEST_SOURCE),
        manifest_contract: PathBuf::from(DEFAULT_MANIFEST_CONTRACT),
        manifest_generation_receipt: PathBuf::from(DEFAULT_MANIFEST_GENERATION_RECEIPT),
        update_golden: false,
        update_report: PathBuf::from(DEFAULT_UPDATE_REPORT),
    };
    validate_checked_manifest_generation(root, &validate_args, &manifest_bytes)?;
    let manifest: ManifestProjection = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| CliError(format!("manifest parse failed: {error}")))?;
    materialize_suite(root, &manifest, None, None)
}

fn is_documentation_fixture(fixture: &FixtureCase) -> bool {
    fixture.id.starts_with("docs-")
        || matches!(
            fixture.artifact_class,
            ArtifactClass::DocumentationIr
                | ArtifactClass::DocumentationOutput
                | ArtifactClass::DocumentationCell
                | ArtifactClass::DocumentationReceipt
        )
}

struct DocsReceiptContext<'a> {
    config_source_identity: &'a DocumentationFactsIdentity,
    config_contract_identity: &'a WireIdentity,
    evaluator_identity: &'a WireIdentity,
    projection_identity: &'a WireIdentity,
    format: Option<&'a str>,
    profile: Option<&'a str>,
    target: Option<&'a str>,
}

fn docs_action_receipt(
    outcome: &DocsActionOutcome,
    context: DocsReceiptContext<'_>,
) -> CliResult<Option<ReceiptEnvelope>> {
    if outcome.state != DocsCompletionState::Completed {
        return Ok(None);
    }
    let action_name = docs_action_name(&outcome.action);
    let action_descriptor = serde_json::to_vec(&(
        action_name,
        context.format,
        context.profile,
        context.target,
        outcome.completed_work_items,
    ))
    .map_err(|error| {
        CliError(format!(
            "docs action descriptor serialization failed: {error}"
        ))
    })?;
    let mut body = ReceiptBody::new(
        format!("docs-action.{action_name}"),
        DOCS_ACTION_ARTIFACT_CLASS,
        "accepted",
        vec![
            REQUIREMENT_DOCS_CONFIG.to_string(),
            REQUIREMENT_DOCS_COMPLETION.to_string(),
            REQUIREMENT_DOCS_NO_OP.to_string(),
            REQUIREMENT_DOCS_RECEIPTS.to_string(),
        ],
        DOCS_ACTION_NON_CLAIMS
            .iter()
            .map(|non_claim| (*non_claim).to_string())
            .collect(),
    );
    body.input_identities.insert(
        "config-source".to_string(),
        context.config_source_identity.to_wire(),
    );
    body.input_identities.insert(
        "config-contract".to_string(),
        context.config_contract_identity.clone(),
    );
    body.input_identities
        .insert("evaluator".to_string(), context.evaluator_identity.clone());
    body.input_identities.insert(
        "config-projection".to_string(),
        context.projection_identity.clone(),
    );
    body.input_identities.insert(
        "action-profile".to_string(),
        compute_identity(IdentityDomain::Projection, &action_descriptor),
    );
    if let Some(identity) = &outcome.documentation_ir_identity {
        body.input_identities
            .insert("documentation-ir".to_string(), identity.clone());
    }
    if let Some(identity) = &outcome.output_identity {
        body.output_identities
            .insert("documentation-output".to_string(), identity.to_wire());
    }
    body.checked_laws = vec![DOCS_ACTION_CHECKED_LAW.to_string()];
    ReceiptEnvelope::finalize(body)
        .map(Some)
        .map_err(|error| CliError(format!("docs action receipt construction failed: {error}")))
}

fn docs_action_name(action: &DocsActionKind) -> &'static str {
    match action {
        DocsActionKind::Collect => "collect",
        DocsActionKind::Check => "check",
        DocsActionKind::Render => "render",
        DocsActionKind::RunCells => "run-cells",
        DocsActionKind::Update => "update",
    }
}

#[cfg(target_os = "linux")]
fn run_nickel_docs_export(
    evaluator_command: &Path,
    source_bytes: &[u8],
    contract_bytes: &[u8],
) -> CliResult<Vec<u8>> {
    use rustix::fs::{MemfdFlags, memfd_create};

    let source = std::str::from_utf8(source_bytes)
        .map_err(|error| CliError(format!("docs config source is not UTF-8: {error}")))?;
    let contract = std::str::from_utf8(contract_bytes)
        .map_err(|error| CliError(format!("docs config contract is not UTF-8: {error}")))?;
    let body = source
        .strip_prefix(DOCS_CONFIG_IMPORT_PREAMBLE)
        .ok_or_else(|| {
            CliError(
                "docs config must import the checked sibling docs-config-contract.ncl contract"
                    .to_string(),
            )
        })?;
    let combined = format!("let DocsConfig = ({contract}) in\n{body}");
    let mut descriptor = std::fs::File::from(
        memfd_create(DOCS_MEMFD_NAME, MemfdFlags::empty())
            .map_err(|error| CliError(format!("docs config memfd creation failed: {error}")))?,
    );
    descriptor
        .write_all(combined.as_bytes())
        .map_err(|error| CliError(format!("docs config memfd write failed: {error}")))?;
    descriptor
        .seek(SeekFrom::Start(0))
        .map_err(|error| CliError(format!("docs config memfd seek failed: {error}")))?;
    let descriptor_path = format!("{PROC_SELF_FD_PREFIX}{}", descriptor.as_raw_fd());
    let output = ProcessCommand::new(evaluator_command)
        .arg(NICKEL_COMMAND_EXPORT)
        .arg(descriptor_path)
        .arg(NICKEL_OPTION_FORMAT)
        .arg(NICKEL_FORMAT_JSON)
        .output()
        .map_err(|error| CliError(format!("docs Nickel export launch failed: {error}")))?;
    command_stdout(output, "docs Nickel export")
}

#[cfg(not(target_os = "linux"))]
fn run_nickel_docs_export(
    _evaluator_command: &Path,
    _source_bytes: &[u8],
    _contract_bytes: &[u8],
) -> CliResult<Vec<u8>> {
    Err(CliError(
        "docs Nickel evaluation requires Linux descriptor-backed evaluator input".to_string(),
    ))
}

fn run_hash_file(args: HashArgs) -> CliResult<i32> {
    let bytes = read_ambient_path(&args.path)?;
    let identity = compute_identity(args.domain, &bytes);
    let json = serde_json::to_string_pretty(&identity)
        .map_err(|error| CliError(format!("hash serialization failed: {error}")))?;
    println!("{json}");
    Ok(EXIT_SUCCESS)
}

fn run_check_manifest_projection(args: ManifestProjectionArgs) -> CliResult<i32> {
    let source_bytes = read_ambient_path(&args.source)?;
    let contract_bytes = read_ambient_path(&args.contract)?;
    let checked_projection_bytes = read_ambient_path(&args.projection)?;
    let (evaluator_descriptor, evaluator_identity) =
        checked_manifest_evaluator(&args.evaluator_command)?;
    let generated_projection_bytes =
        run_nickel_manifest_export(&args.evaluator_command, &source_bytes, &contract_bytes)?;
    validate_manifest_projection_shape(&generated_projection_bytes)?;
    let generated_projection_identity =
        compute_identity(IdentityDomain::Projection, &generated_projection_bytes);
    let checked_projection_identity =
        compute_identity(IdentityDomain::Projection, &checked_projection_bytes);
    let mut diagnostics = Vec::new();
    if generated_projection_bytes != checked_projection_bytes {
        diagnostics.push(
            "checked manifest projection differs byte-for-byte from the typed Nickel export"
                .to_string(),
        );
    }
    let report = ManifestProjectionReport {
        passed: diagnostics.is_empty(),
        generated_projection_identity,
        checked_projection_identity,
        evaluator_identity,
        diagnostics,
    };
    debug_assert_eq!(evaluator_descriptor, MANIFEST_EVALUATOR_DESCRIPTOR);
    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| CliError(format!("projection report serialization failed: {error}")))?;
    println!("{json}");
    if report.passed {
        Ok(EXIT_SUCCESS)
    } else {
        Ok(EXIT_FAILURE)
    }
}

// r[impl kamacite.findings.manifest_semantic_freshness.generation_receipt]
fn run_generate_manifest(args: GenerateManifestArgs) -> CliResult<i32> {
    let root = RootCapability::open(&args.root)?;
    let source_bytes = root.read(&args.source)?;
    let contract_bytes = root.read(&args.contract)?;
    let (evaluator_descriptor, _) = checked_manifest_evaluator(&args.evaluator_command)?;
    let generated_projection_bytes =
        run_nickel_manifest_export(&args.evaluator_command, &source_bytes, &contract_bytes)?;
    validate_manifest_projection_shape(&generated_projection_bytes)?;
    let receipt = build_manifest_generation_receipt(ManifestGenerationInput {
        source_bytes: &source_bytes,
        contract_bytes: &contract_bytes,
        evaluator_profile: MANIFEST_EVALUATOR_PROFILE,
        evaluator_descriptor: &evaluator_descriptor,
        projection_bytes: &generated_projection_bytes,
    })
    .map_err(|error| CliError(format!("manifest receipt construction failed: {error}")))?;
    let mut diagnostics = Vec::new();
    if args.update {
        root.write(&args.projection, &generated_projection_bytes)?;
        let mut receipt_bytes = serde_json::to_vec_pretty(&receipt).map_err(|error| {
            CliError(format!(
                "manifest generation receipt serialization failed: {error}"
            ))
        })?;
        receipt_bytes.push(b'\n');
        root.write(&args.receipt, &receipt_bytes)?;
    } else {
        let checked_projection_bytes = root.read(&args.projection)?;
        if checked_projection_bytes != generated_projection_bytes {
            diagnostics.push(
                "checked manifest projection differs byte-for-byte from the typed Nickel export"
                    .to_string(),
            );
        }
        let checked_receipt_bytes = root.read(&args.receipt)?;
        let checked_receipt: ManifestGenerationReceipt =
            serde_json::from_slice(&checked_receipt_bytes).map_err(|error| {
                CliError(format!("manifest generation receipt parse failed: {error}"))
            })?;
        let validation = validate_manifest_generation_receipt(
            ManifestGenerationInput {
                source_bytes: &source_bytes,
                contract_bytes: &contract_bytes,
                evaluator_profile: MANIFEST_EVALUATOR_PROFILE,
                evaluator_descriptor: &evaluator_descriptor,
                projection_bytes: &checked_projection_bytes,
            },
            &checked_receipt,
        );
        diagnostics.extend(
            validation
                .diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.message),
        );
    }
    let report = ManifestGenerationCommandReport {
        passed: diagnostics.is_empty(),
        updated: args.update,
        projection_path: args.projection.display().to_string(),
        receipt_path: args.receipt.display().to_string(),
        receipt,
        diagnostics,
    };
    print_json_report(&report, "manifest generation report")?;
    Ok(if report.passed {
        EXIT_SUCCESS
    } else {
        EXIT_FAILURE
    })
}

fn checked_manifest_evaluator(evaluator_command: &Path) -> CliResult<(String, WireIdentity)> {
    let descriptor = nickel_evaluator_descriptor(&run_nickel_version(evaluator_command)?)?;
    if descriptor != MANIFEST_EVALUATOR_DESCRIPTOR {
        return Err(CliError(format!(
            "manifest evaluator descriptor drift: expected `{MANIFEST_EVALUATOR_DESCRIPTOR}`, got `{descriptor}`"
        )));
    }
    let identity = compute_identity(IdentityDomain::Evaluator, descriptor.as_bytes());
    Ok((descriptor, identity))
}

// r[impl kamacite.findings.manifest_semantic_freshness.complete_projection]
fn validate_manifest_projection_shape(projection_bytes: &[u8]) -> CliResult<()> {
    let projection: ManifestProjection = serde_json::from_slice(projection_bytes)
        .map_err(|error| CliError(format!("manifest projection parse failed: {error}")))?;
    if projection.schema_version != MANIFEST_SCHEMA_VERSION {
        return Err(CliError(format!(
            "manifest schema version must be {MANIFEST_SCHEMA_VERSION}, got {}",
            projection.schema_version
        )));
    }
    if projection.authority_non_claims.is_empty() {
        return Err(CliError(
            "manifest authority_non_claims must not be empty".to_string(),
        ));
    }
    if projection.fixtures.is_empty() {
        return Err(CliError(
            "manifest projection must contain at least one fixture".to_string(),
        ));
    }
    let mut ids = BTreeSet::new();
    for fixture in &projection.fixtures {
        if !ids.insert(fixture.id.as_str()) {
            return Err(CliError(format!(
                "manifest fixture id `{}` is duplicated",
                fixture.id
            )));
        }
    }
    Ok(())
}

// r[impl kamacite.findings.manifest_semantic_freshness.typed_contract]
#[cfg(target_os = "linux")]
fn run_nickel_manifest_export(
    evaluator_command: &Path,
    source_bytes: &[u8],
    contract_bytes: &[u8],
) -> CliResult<Vec<u8>> {
    use rustix::fs::{MemfdFlags, memfd_create};

    let mut evaluator_input = Vec::with_capacity(source_bytes.len() + contract_bytes.len());
    evaluator_input.extend_from_slice(b"(\n");
    evaluator_input.extend_from_slice(source_bytes);
    evaluator_input.extend_from_slice(b"\n) | (\n");
    evaluator_input.extend_from_slice(contract_bytes);
    evaluator_input.extend_from_slice(b"\n)\n");

    let descriptor = memfd_create(MANIFEST_MEMFD_NAME, MemfdFlags::empty())
        .map_err(|error| CliError(format!("manifest evaluator memfd creation failed: {error}")))?;
    let mut file = std::fs::File::from(descriptor);
    file.write_all(&evaluator_input)
        .map_err(|error| CliError(format!("manifest evaluator input write failed: {error}")))?;
    file.seek(SeekFrom::Start(0))
        .map_err(|error| CliError(format!("manifest evaluator input seek failed: {error}")))?;
    let descriptor_path = format!("{PROC_SELF_FD_PREFIX}{}", file.as_raw_fd());
    let output = ProcessCommand::new(evaluator_command)
        .arg(NICKEL_COMMAND_EXPORT)
        .arg(descriptor_path)
        .arg(NICKEL_OPTION_FORMAT)
        .arg(NICKEL_FORMAT_JSON)
        .output()
        .map_err(|error| CliError(format!("manifest Nickel export launch failed: {error}")))?;
    if !output.status.success() {
        return Err(CliError(format!(
            "manifest Nickel export failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    if output.stdout.is_empty() {
        return Err(CliError(
            "manifest Nickel export produced no projection bytes".to_string(),
        ));
    }
    Ok(output.stdout)
}

#[cfg(not(target_os = "linux"))]
fn run_nickel_manifest_export(
    _evaluator_command: &Path,
    _source_bytes: &[u8],
    _contract_bytes: &[u8],
) -> CliResult<Vec<u8>> {
    Err(CliError(
        "manifest Nickel generation requires Linux descriptor-backed evaluator input".to_string(),
    ))
}

fn run_export_valence_receipt(args: ValenceExportArgs) -> CliResult<i32> {
    let root = RootCapability::open(&args.root)?;
    let source_receipt: WireReceiptEnvelope =
        read_root_json(&root, &args.receipt, "Valence source receipt")?;
    let conformance_projection: ValenceExportConformance =
        read_root_json(&root, &args.conformance, "Valence export conformance")?;
    let validated_conformance =
        admit_valence_conformance(conformance_projection).map_err(|diagnostics| {
            CliError(format!(
                "Valence export conformance admission failed: {diagnostics:?}"
            ))
        })?;
    let conformance = validated_conformance.projection().clone();
    let source_spans = match &args.source_spans {
        Some(path) => read_root_json(&root, path, "Valence export source spans")?,
        None => SourceSpanSet::default(),
    };
    let input = ValenceReceiptExportInput {
        source_receipt,
        conformance,
        source_spans,
        semantic_claims: Vec::new(),
        declared_export_identity: None,
    };
    let result = export_valence_receipt(&input);
    let export_identity = result
        .export
        .as_ref()
        .map(|export| export.export_identity.clone());
    let (golden_result, golden_passed) =
        check_or_update_valence_export_golden(&root, &args, result.export.as_ref())?;
    let passed = result.passed && golden_passed;
    let report = ValenceExportCommandReport {
        passed,
        validation_passed: result.passed,
        diagnostics: result.diagnostics,
        export_identity,
        golden_result,
        export: result.export,
    };
    let json = serde_json::to_string_pretty(&report).map_err(|error| {
        CliError(format!(
            "Valence export report serialization failed: {error}"
        ))
    })?;
    println!("{json}");
    if passed {
        Ok(EXIT_SUCCESS)
    } else {
        Ok(EXIT_FAILURE)
    }
}

fn run_emit_function_address_receipt(args: FunctionAddressReceiptArgs) -> CliResult<i32> {
    let root = RootCapability::open(&args.root)?;
    let report: OctetFunctionAddressReport =
        read_root_json(&root, &args.report, "Octet function-address report")?;
    let source_bytes = read_function_address_sources(&root, &args.sources)?;
    let input = build_function_address_receipt_input(report, &source_bytes)?;
    let result = validate_function_address_receipt(&input);
    let receipt_identity = result
        .receipt
        .as_ref()
        .map(|receipt| receipt.receipt_identity.clone());
    let Some(receipt) = result.receipt else {
        let report = FunctionAddressReceiptCommandReport {
            passed: false,
            diagnostics: result.diagnostics,
            receipt_identity,
            receipt_path: args.out.as_ref().map(|path| path_display(path)),
            valence_projection_path: args.valence_out.as_ref().map(|path| path_display(path)),
            valence_projection: None,
        };
        print_json_report(&report, "function-address receipt report")?;
        return Ok(EXIT_FAILURE);
    };

    let receipt_json = function_address_projection_json(&receipt).map_err(|error| {
        CliError(format!(
            "function-address receipt projection serialization failed: {error}"
        ))
    })?;
    let receipt_path = match &args.out {
        Some(path) => {
            root.write(path, receipt_json.as_bytes())?;
            Some(path_display(path))
        }
        None => None,
    };

    let (valence_projection, valence_projection_path) = match &args.valence_out {
        Some(path) => {
            let projection =
                export_function_address_valence_projection(&receipt).map_err(|diagnostics| {
                    CliError(format!(
                        "function-address Valence projection validation failed: {diagnostics:?}"
                    ))
                })?;
            let rendered = serde_json::to_string_pretty(&projection).map_err(|error| {
                CliError(format!(
                    "function-address Valence projection serialization failed: {error}"
                ))
            })?;
            root.write(path, rendered.as_bytes())?;
            (Some(projection), Some(path_display(path)))
        }
        None => (None, None),
    };

    if args.out.is_none() {
        println!("{receipt_json}");
        return Ok(EXIT_SUCCESS);
    }

    let report = FunctionAddressReceiptCommandReport {
        passed: true,
        diagnostics: result.diagnostics,
        receipt_identity,
        receipt_path,
        valence_projection_path,
        valence_projection,
    };
    print_json_report(&report, "function-address receipt report")?;
    Ok(EXIT_SUCCESS)
}

fn read_function_address_sources(
    root: &RootCapability,
    sources: &BTreeMap<String, PathBuf>,
) -> CliResult<BTreeMap<String, String>> {
    let mut source_bytes = BTreeMap::new();
    for (key, path) in sources {
        let bytes = root.read(path)?;
        let source = String::from_utf8(bytes).map_err(|error| {
            CliError(format!(
                "function-address source is not UTF-8 for {}: {error}",
                path.display()
            ))
        })?;
        source_bytes.insert(key.clone(), source);
    }
    Ok(source_bytes)
}

fn build_function_address_receipt_input(
    report: OctetFunctionAddressReport,
    source_bytes: &BTreeMap<String, String>,
) -> CliResult<FunctionAddressReceiptInput> {
    let mut normalized_sources = Vec::new();
    for source in report.source_envelopes {
        let key = octet_source_key(&source);
        let Some(bytes) = source_bytes.get(&key) else {
            return Err(CliError(format!(
                "Octet function-address report source {key} has no matching --source KEY=PATH input"
            )));
        };
        normalized_sources.push(FunctionAddressSourceEnvelope {
            key,
            display_path: source.path,
            language: source.language,
            source_bytes: bytes.clone(),
            source_identity: octet_identity(IdentityDomain::Source, &source.source_identity)?,
        });
    }
    for key in source_bytes.keys() {
        if !normalized_sources.iter().any(|source| source.key == *key) {
            return Err(CliError(format!(
                "--source key {key} is not declared by the Octet function-address report"
            )));
        }
    }
    let functions = report
        .records
        .into_iter()
        .map(octet_function_record)
        .collect::<CliResult<Vec<_>>>()?;
    let producer = report.producer;
    let canonicalizer = if report.canonicalizer.is_empty() {
        producer.canonicalizer.clone()
    } else {
        report.canonicalizer
    };
    let tool_version = if !report.tool.is_empty() {
        report.tool
    } else if !producer.tool_version.is_empty() {
        producer.tool_version.clone()
    } else {
        producer.role.clone()
    };
    let build_identity = producer
        .build_identity
        .as_ref()
        .map(|identity| octet_identity(IdentityDomain::Adapter, identity))
        .transpose()?;
    let mut normative_options = producer.normative_options;
    if !producer.role.is_empty() {
        normative_options.insert(
            FUNCTION_ADDRESS_PRODUCER_ROLE_OPTION.to_string(),
            producer.role,
        );
    }
    let mut producer_non_claims = producer.non_claims;
    producer_non_claims.push(FUNCTION_ADDRESS_PRODUCER_NON_CLAIM_CORRECTNESS.to_string());
    producer_non_claims.push(FUNCTION_ADDRESS_PRODUCER_NON_CLAIM_SEMANTIC_CLOSURE.to_string());
    producer_non_claims.sort();
    producer_non_claims.dedup();
    Ok(FunctionAddressReceiptInput {
        producer: FunctionAddressProducer {
            name: producer.name,
            canonicalizer,
            tool_version,
            build_identity,
            normative_options,
            non_claims: producer_non_claims,
        },
        source_envelopes: normalized_sources,
        functions,
        dependency_edges: report.dependency_edges,
        caveats: report.caveats,
        semantic_claims: report.semantic_claims,
        non_claims: report.non_claims,
    })
}

fn octet_source_key(source: &OctetFunctionAddressSourceEnvelope) -> String {
    if source.key.is_empty() {
        source.path.clone()
    } else {
        source.key.clone()
    }
}

fn octet_function_record(record: OctetFunctionAddressRecord) -> CliResult<FunctionAddressRecord> {
    let source_key = if record.source_key.is_empty() {
        record.path.clone()
    } else {
        record.source_key
    };
    let locator = if record.path.contains("::") {
        record.path.clone()
    } else {
        format!("{}::{}", record.path, record.name)
    };
    let span = record.span.or_else(|| {
        Some(FunctionAddressSpan {
            source_key: source_key.clone(),
            start_byte: record.start_byte,
            end_byte: record.end_byte,
        })
    });
    Ok(FunctionAddressRecord {
        locator,
        display_name: record.name,
        source_key,
        span,
        function_address: octet_identity(
            IdentityDomain::FunctionAddress,
            &record.function_address,
        )?,
        dependency_closure: record
            .dependency_set_hash
            .as_ref()
            .map(|hash| octet_identity(IdentityDomain::DependencyClosure, hash))
            .transpose()?,
        dependency_identity_kind: record
            .dependency_identity_kind
            .as_deref()
            .map(octet_dependency_kind)
            .transpose()?,
        caveats: record.caveats,
    })
}

fn octet_identity(domain: IdentityDomain, value: &OctetIdentity) -> CliResult<WireIdentity> {
    match value {
        OctetIdentity::Text(value) => {
            let hex = value.strip_prefix("b3:").unwrap_or(value);
            let parsed = Blake3Hex::parse(hex.to_owned()).map_err(|error| {
                CliError(format!("invalid Octet BLAKE3 identity `{value}`: {error}"))
            })?;
            Ok(WireIdentity::new(domain, parsed))
        }
        OctetIdentity::Structured(identity) => {
            identity
                .validate_domain(domain)
                .map_err(|error| CliError(format!("invalid structured Octet identity: {error}")))?;
            Ok(identity.clone())
        }
    }
}

fn octet_dependency_kind(value: &str) -> CliResult<FunctionAddressDependencyIdentityKind> {
    match value {
        "source-token-set" | "source-token-dependency-set" => {
            Ok(FunctionAddressDependencyIdentityKind::SourceTokenDependencySet)
        }
        "source-shape-set" | "source-shape-dependency-set" => {
            Ok(FunctionAddressDependencyIdentityKind::SourceShapeDependencySet)
        }
        other => Err(CliError(format!(
            "unsupported Octet dependency_identity_kind `{other}`"
        ))),
    }
}

fn print_json_report<T: Serialize>(report: &T, label: &str) -> CliResult<()> {
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| CliError(format!("{label} serialization failed: {error}")))?;
    println!("{json}");
    Ok(())
}

fn path_display(path: &Path) -> String {
    path.display().to_string()
}

fn check_or_update_valence_export_golden(
    root: &RootCapability,
    args: &ValenceExportArgs,
    export: Option<&ValenceReceiptExportEnvelope>,
) -> CliResult<(Option<ValenceExportGoldenResult>, bool)> {
    let Some(golden_path) = &args.golden else {
        return Ok((None, true));
    };
    let Some(export) = export else {
        return Ok((
            Some(ValenceExportGoldenResult {
                path: golden_path.display().to_string(),
                status: GOLDEN_STATUS_MISSING_RECEIPT.to_string(),
                old_export_identity: None,
                new_export_identity: None,
            }),
            false,
        ));
    };
    let rendered = serde_json::to_string_pretty(export)
        .map_err(|error| CliError(format!("Valence export serialization failed: {error}")))?;
    if args.update_golden {
        let old_export_identity = read_existing_valence_export_identity(root, golden_path)?;
        root.write(golden_path, rendered.as_bytes())?;
        write_valence_export_update_report(root, args, export, old_export_identity.clone())?;
        return Ok((
            Some(ValenceExportGoldenResult {
                path: golden_path.display().to_string(),
                status: GOLDEN_STATUS_UPDATED.to_string(),
                old_export_identity,
                new_export_identity: Some(export.export_identity.clone()),
            }),
            true,
        ));
    }
    let Some(existing) = root.read_optional(golden_path)? else {
        return Ok((
            Some(ValenceExportGoldenResult {
                path: golden_path.display().to_string(),
                status: GOLDEN_STATUS_MISSING_GOLDEN.to_string(),
                old_export_identity: None,
                new_export_identity: Some(export.export_identity.clone()),
            }),
            false,
        ));
    };
    let matched = existing == rendered.as_bytes();
    let old_export_identity = parse_valence_export_identity(&existing);
    Ok((
        Some(ValenceExportGoldenResult {
            path: golden_path.display().to_string(),
            status: if matched {
                GOLDEN_STATUS_MATCHED.to_string()
            } else {
                GOLDEN_STATUS_MISMATCHED.to_string()
            },
            old_export_identity,
            new_export_identity: Some(export.export_identity.clone()),
        }),
        matched,
    ))
}

fn write_valence_export_update_report(
    root: &RootCapability,
    args: &ValenceExportArgs,
    export: &ValenceReceiptExportEnvelope,
    old_export_identity: Option<ProjectionIdentity>,
) -> CliResult<()> {
    let Some(golden_path) = &args.golden else {
        return Ok(());
    };
    let update = ValenceExportUpdate {
        path: golden_path.display().to_string(),
        source_receipt_identity: export.body.source_receipt_identity.clone(),
        old_export_identity,
        new_export_identity: export.export_identity.clone(),
        checked_laws: export.body.checked_laws.clone(),
        non_claims: export.body.non_claims.clone(),
    };
    let report = ValenceExportUpdateReport {
        changed_exports: vec![update],
    };
    let rendered = serde_json::to_string_pretty(&report).map_err(|error| {
        CliError(format!(
            "Valence export update report serialization failed: {error}"
        ))
    })?;
    root.write(&args.update_report, rendered.as_bytes())?;
    Ok(())
}

fn read_existing_valence_export_identity(
    root: &RootCapability,
    path: &Path,
) -> CliResult<Option<ProjectionIdentity>> {
    let Some(bytes) = root.read_optional(path)? else {
        return Ok(None);
    };
    Ok(parse_valence_export_identity(&bytes))
}

fn parse_valence_export_identity(bytes: &[u8]) -> Option<ProjectionIdentity> {
    let parsed: Result<ValenceReceiptExportEnvelope, serde_json::Error> =
        serde_json::from_slice(bytes);
    parsed.ok().map(|export| export.export_identity)
}

fn read_root_json<T: for<'de> Deserialize<'de>>(
    root: &RootCapability,
    path: &Path,
    label: &str,
) -> CliResult<T> {
    let bytes = root.read(path)?;
    serde_json::from_slice(&bytes).map_err(|error| {
        CliError(format!(
            "{label} parse failed for {}: {error}",
            path.display()
        ))
    })
}

fn run_migrate_preserves(args: MigratePreservesArgs) -> CliResult<i32> {
    let root = RootCapability::open(&args.root)?;
    let execution = if args.update {
        execute_preserves_migration(&root, &args)?
    } else {
        let plan = plan_preserves_migration(&root, &args)?;
        PreservesMigrationExecution {
            artifacts: plan.artifacts,
            mutated: false,
            converged: false,
            migration_passes: 1,
        }
    };
    let report = PreservesMigrationReport {
        schema_version: PRESERVES_MIGRATION_SCHEMA_VERSION.to_string(),
        legacy_codec: PRESERVES_LEGACY_PREFIXED_JSON_CODEC.to_string(),
        canonical_codec: PRESERVES_CANONICAL_CODEC.to_string(),
        preserves_package_version: PRESERVES_PACKAGE_VERSION.to_string(),
        preserves_spec_version: PRESERVES_SPEC_VERSION.to_string(),
        dry_run: !args.update,
        mutated: execution.mutated,
        converged: execution.converged,
        migration_passes: execution.migration_passes,
        artifacts: execution.artifacts,
        non_claims: vec![
            "legacy raw-text and labeled-JSON identities remain compatibility evidence only"
                .to_string(),
            "codec migration does not prove evaluator, compiler, or runtime correctness"
                .to_string(),
        ],
    };
    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| CliError(format!("Preserves migration report failed: {error}")))?;
    if args.update {
        root.write(&args.report, json.as_bytes())?;
    }
    println!("{json}");
    Ok(EXIT_SUCCESS)
}

fn execute_preserves_migration(
    root: &RootCapability,
    args: &MigratePreservesArgs,
) -> CliResult<PreservesMigrationExecution> {
    let mut accumulated_artifacts: Option<Vec<PreservesMigrationArtifact>> = None;
    let mut mutated = false;

    for pass_index in 0..PRESERVES_MIGRATION_MAX_PASSES {
        let plan = plan_preserves_migration(root, args)?;
        let PreservesMigrationPlan {
            manifest,
            artifacts,
            writes,
        } = plan;
        let content_changed = !writes.is_empty();
        for write in &writes {
            root.write(&write.path, &write.bytes)?;
        }
        let manifest_changed =
            update_preserves_manifest_identities(root, args, &manifest, &artifacts)?;
        accumulated_artifacts = Some(match accumulated_artifacts {
            Some(previous) => merge_migration_artifacts(previous, &artifacts)?,
            None => artifacts,
        });
        mutated |= content_changed || manifest_changed;
        if !content_changed && !manifest_changed {
            return Ok(PreservesMigrationExecution {
                artifacts: accumulated_artifacts.unwrap_or_default(),
                mutated,
                converged: true,
                migration_passes: pass_index + 1,
            });
        }
    }

    Err(CliError(format!(
        "Preserves migration did not converge within {PRESERVES_MIGRATION_MAX_PASSES} passes"
    )))
}

fn plan_preserves_migration(
    root: &RootCapability,
    args: &MigratePreservesArgs,
) -> CliResult<PreservesMigrationPlan> {
    let manifest_bytes = root.read(&args.manifest)?;
    let manifest: ManifestProjection = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| CliError(format!("manifest parse failed: {error}")))?;
    let suite = materialize_suite(root, &manifest, None, None)?;
    if suite.fixtures.len() != manifest.fixtures.len() {
        return Err(CliError(
            "manifest and materialized fixture counts differ".to_string(),
        ));
    }

    let mut artifacts = Vec::new();
    let mut writes = Vec::new();
    let mut planned_identities = BTreeMap::new();
    for (projection, fixture) in manifest.fixtures.iter().zip(&suite.fixtures) {
        let Some(canonical_path) = projection.payload.canonical_path() else {
            continue;
        };
        let Some(domain) = projection.payload.canonical_identity_domain() else {
            return Err(CliError(format!(
                "canonical fixture {} has no identity domain",
                projection.id
            )));
        };
        let expected = match fixture_expected_canonical_artifact(&fixture.payload) {
            Ok(Some(expected)) => expected,
            Ok(None) => {
                return Err(CliError(format!(
                    "canonical fixture {} has no canonical artifact encoder",
                    projection.id
                )));
            }
            Err(error) if projection.expected_verdict == ValidationVerdict::Rejected => {
                artifacts.push(PreservesMigrationArtifact {
                    fixture_id: projection.id.clone(),
                    path: canonical_path.to_string(),
                    old_identity: projection.payload.declared_canonical_identity(),
                    new_identity: None,
                    content_changed: false,
                    canonical_packed_size: None,
                    error: Some(error.to_string()),
                });
                continue;
            }
            Err(error) => {
                return Err(CliError(format!(
                    "Preserves migration failed for {}: {error}",
                    projection.id
                )));
            }
        };
        let relative_path = PathBuf::from(canonical_path);
        let current = root.read(&relative_path)?;
        let mut rendered = expected.text_projection.as_bytes().to_vec();
        rendered.push(b'\n');
        let content_changed = current != rendered;
        let new_identity = compute_identity(domain, &expected.packed_bytes);
        if let Some(existing) =
            planned_identities.insert(canonical_path.to_string(), new_identity.clone())
            && existing != new_identity
        {
            return Err(CliError(format!(
                "canonical path {canonical_path} is shared by fixtures with different normative content"
            )));
        }
        artifacts.push(PreservesMigrationArtifact {
            fixture_id: projection.id.clone(),
            path: canonical_path.to_string(),
            old_identity: projection.payload.declared_canonical_identity(),
            new_identity: Some(new_identity),
            content_changed,
            canonical_packed_size: Some(expected.packed_bytes.len()),
            error: None,
        });
        if content_changed {
            writes.push(PreservesMigrationWrite {
                path: relative_path,
                bytes: rendered,
            });
        }
    }

    Ok(PreservesMigrationPlan {
        manifest,
        artifacts,
        writes,
    })
}

fn merge_migration_artifacts(
    previous: Vec<PreservesMigrationArtifact>,
    current: &[PreservesMigrationArtifact],
) -> CliResult<Vec<PreservesMigrationArtifact>> {
    if previous.len() != current.len() {
        return Err(CliError(
            "Preserves migration artifact count changed between passes".to_string(),
        ));
    }
    previous
        .into_iter()
        .zip(current)
        .map(|(previous, current)| {
            if previous.fixture_id != current.fixture_id {
                return Err(CliError(format!(
                    "Preserves migration artifact ordering changed from {} to {}",
                    previous.fixture_id, current.fixture_id
                )));
            }
            Ok(PreservesMigrationArtifact {
                fixture_id: current.fixture_id.clone(),
                path: current.path.clone(),
                old_identity: previous.old_identity,
                new_identity: current.new_identity.clone(),
                content_changed: previous.content_changed || current.content_changed,
                canonical_packed_size: current.canonical_packed_size,
                error: current.error.clone(),
            })
        })
        .collect()
}

fn update_preserves_manifest_identities(
    root: &RootCapability,
    args: &MigratePreservesArgs,
    manifest: &ManifestProjection,
    artifacts: &[PreservesMigrationArtifact],
) -> CliResult<bool> {
    let mut projection = String::from_utf8(root.read(&args.manifest)?).map_err(|error| {
        CliError(format!(
            "manifest projection is not UTF-8 for {}: {error}",
            args.manifest.display()
        ))
    })?;
    let mut source = String::from_utf8(root.read(&args.manifest_source)?).map_err(|error| {
        CliError(format!(
            "manifest source is not UTF-8 for {}: {error}",
            args.manifest_source.display()
        ))
    })?;
    let original_projection = projection.clone();
    let original_source = source.clone();

    let mut artifact_by_id = BTreeMap::new();
    let mut replacements: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for artifact in artifacts {
        artifact_by_id.insert(artifact.fixture_id.as_str(), artifact);
        if preserves_fixture_retains_stale_identity(&artifact.fixture_id) {
            continue;
        }
        if let (Some(old), Some(new)) = (&artifact.old_identity, &artifact.new_identity) {
            replacements
                .entry(old.hex.as_str().to_string())
                .or_default()
                .insert(new.hex.as_str().to_string());
        }
    }

    for (old, new_values) in &replacements {
        if new_values.len() != 1 {
            continue;
        }
        let Some(new) = new_values.first() else {
            continue;
        };
        projection = projection.replace(old, new);
        source = source.replace(old, new);
    }

    if let Some(authoritative) = artifact_by_id.get(AUTHORITATIVE_DOCUMENTATION_IR_FIXTURE_ID)
        && let (Some(old), Some(new)) = (&authoritative.old_identity, &authoritative.new_identity)
    {
        projection = projection.replace(old.hex.as_str(), new.hex.as_str());
        source = source.replace(old.hex.as_str(), new.hex.as_str());
    }

    for fixture in &manifest.fixtures {
        let Some(artifact) = artifact_by_id.get(fixture.id.as_str()) else {
            continue;
        };
        if preserves_fixture_retains_stale_identity(&fixture.id) {
            continue;
        }
        if artifact.old_identity.is_none() {
            continue;
        }
        let Some(new_identity) = &artifact.new_identity else {
            continue;
        };
        let Some(field) = fixture.payload.declared_identity_field() else {
            continue;
        };
        replace_fixture_identity_hex(
            &mut projection,
            &fixture.id,
            field,
            new_identity.hex.as_str(),
        )?;
        replace_fixture_identity_hex_if_present(
            &mut source,
            &fixture.id,
            field,
            new_identity.hex.as_str(),
        )?;
    }

    let projection_changed = projection != original_projection;
    let source_changed = source != original_source;
    if projection_changed {
        root.write(&args.manifest, projection.as_bytes())?;
    }
    if source_changed {
        root.write(&args.manifest_source, source.as_bytes())?;
    }
    Ok(projection_changed || source_changed)
}

fn preserves_fixture_retains_stale_identity(fixture_id: &str) -> bool {
    fixture_id == STALE_FSM_CANONICAL_FIXTURE_ID
        || fixture_id == STALE_ALGEBRAIC_CANONICAL_FIXTURE_ID
}

impl PayloadProjection {
    fn declared_identity_field(&self) -> Option<&'static str> {
        match self {
            Self::CanonicalValue { .. }
            | Self::FsmSpec { .. }
            | Self::DocumentationIr { .. }
            | Self::AlgebraicEffect { .. } => Some("declared_identity"),
            Self::FsmReceipt { .. } | Self::DocumentationReceipt { .. } => {
                Some("declared_receipt_identity")
            }
            _ => None,
        }
    }
}

fn replace_fixture_identity_hex(
    text: &mut String,
    fixture_id: &str,
    field: &str,
    new_hex: &str,
) -> CliResult<()> {
    if replace_fixture_identity_hex_inner(text, fixture_id, field, new_hex)? {
        Ok(())
    } else {
        Err(CliError(format!(
            "fixture {fixture_id} is missing {field} identity bytes"
        )))
    }
}

fn replace_fixture_identity_hex_if_present(
    text: &mut String,
    fixture_id: &str,
    field: &str,
    new_hex: &str,
) -> CliResult<()> {
    let _ = replace_fixture_identity_hex_inner(text, fixture_id, field, new_hex)?;
    Ok(())
}

fn replace_fixture_identity_hex_inner(
    text: &mut String,
    fixture_id: &str,
    field: &str,
    new_hex: &str,
) -> CliResult<bool> {
    let json_id = format!("\"id\": \"{fixture_id}\"");
    let compact_json_id = format!("\"id\":\"{fixture_id}\"");
    let nickel_id = format!("id = \"{fixture_id}\"");
    let quoted_nickel_id = format!("\"id\" = \"{fixture_id}\"");
    let start = text
        .find(&json_id)
        .or_else(|| text.find(&compact_json_id))
        .or_else(|| text.find(&nickel_id))
        .or_else(|| text.find(&quoted_nickel_id))
        .ok_or_else(|| CliError(format!("fixture {fixture_id} not found during migration")))?;
    let tail = &text[start..];
    let fixture_end = next_fixture_offset(tail).unwrap_or(tail.len());
    let fixture_text = &tail[..fixture_end];
    let Some(field_offset) = fixture_text.find(field) else {
        return Ok(false);
    };
    let field_tail = &fixture_text[field_offset + field.len()..];
    let Some(open_brace_offset) = field_tail.find('{') else {
        return Ok(false);
    };
    let identity_start = field_offset + field.len() + open_brace_offset;
    let identity_tail = &fixture_text[identity_start..];
    let Some(close_brace_offset) = identity_tail.find('}') else {
        return Err(CliError(format!(
            "fixture {fixture_id} has malformed {field} identity"
        )));
    };
    let identity_text = &identity_tail[..close_brace_offset];
    let Some(hex_offset) = identity_text.find("hex") else {
        return Ok(false);
    };
    let after_hex_start = hex_offset + "hex".len();
    let after_hex = &identity_text[after_hex_start..];
    let assignment_offset = after_hex
        .find(':')
        .into_iter()
        .chain(after_hex.find('='))
        .min()
        .ok_or_else(|| CliError(format!("fixture {fixture_id} has malformed {field} hex")))?;
    let after_assignment_start = after_hex_start + assignment_offset + 1;
    let after_assignment = &identity_text[after_assignment_start..];
    let quote_offset = after_assignment
        .find('"')
        .ok_or_else(|| CliError(format!("fixture {fixture_id} has malformed {field} hex")))?;
    let value_offset = after_assignment_start + quote_offset + 1;
    let value_tail = &identity_text[value_offset..];
    let end_quote_offset = value_tail
        .find('"')
        .ok_or_else(|| CliError(format!("fixture {fixture_id} has unterminated {field} hex")))?;
    let value_start = start + identity_start + value_offset;
    let value_end = value_start + end_quote_offset;
    text.replace_range(value_start..value_end, new_hex);
    Ok(true)
}

fn next_fixture_offset(text: &str) -> Option<usize> {
    let markers = [
        "\n      \"id\":",
        "\n    {\"id\":",
        "\n      id = ",
        "\n      \"id\" = ",
    ];
    let tail = text.get(1..)?;
    markers
        .iter()
        .filter_map(|marker| tail.find(marker).map(|offset| offset + 1))
        .min()
}

fn run_validate(args: ValidateArgs) -> CliResult<i32> {
    run_validate_with_shells(args, None, None)
}

fn run_validate_nickel_adapter(args: NickelAdapterArgs) -> CliResult<i32> {
    let request = NickelExecutionRequest {
        evaluator_command: args.evaluator_command,
        sandbox_command: args.sandbox_command,
        limit_command: args.limit_command,
    };
    run_validate_with_shells(args.validate, Some(&request), None)
}

fn resolve_nickel_execution_shell(
    request: &NickelExecutionRequest,
) -> CliResult<NickelExecutionShell> {
    let evaluator_command = resolve_command(&request.evaluator_command).map_err(CliError)?;
    let sandbox_command = resolve_command(&request.sandbox_command).map_err(CliError)?;
    let limit_command = resolve_command(&request.limit_command).map_err(CliError)?;
    validate_enforcement_command_paths(&sandbox_command, &limit_command).map_err(CliError)?;
    let runtime_closure_paths = runtime_closure_paths(&evaluator_command).map_err(CliError)?;
    Ok(NickelExecutionShell {
        evaluator_command,
        sandbox_command,
        limit_command,
        runtime_closure_paths,
    })
}

fn run_adapt_nickel_export(command: &NickelPreservesCommand) -> CliResult<i32> {
    let report = run_nickel_preserves_command(command).map_err(CliError)?;
    print_json_report(&report, "Nickel Preserves adapter report")?;
    Ok(EXIT_SUCCESS)
}

fn run_validate_steel_adapter(args: SteelAdapterArgs) -> CliResult<i32> {
    let shell = SteelExecutionShell {
        evaluator_descriptor: args.evaluator_descriptor,
    };
    run_validate_with_shells(args.validate, None, Some(&shell))
}

// r[impl kamacite.findings.manifest_semantic_freshness.generation_receipt]
fn run_validate_with_shells(
    args: ValidateArgs,
    nickel_request: Option<&NickelExecutionRequest>,
    steel_shell: Option<&SteelExecutionShell>,
) -> CliResult<i32> {
    let root = RootCapability::open(&args.root)?;
    let manifest_bytes = root.read(&args.manifest)?;
    let manifest_generation = validate_checked_manifest_generation(&root, &args, &manifest_bytes)?;
    if !manifest_generation.passed {
        let report = CliReport {
            passed: false,
            manifest_generation,
            fixture_report: None,
            golden_results: Vec::new(),
        };
        print_json_report(&report, "validation report")?;
        return Ok(EXIT_FAILURE);
    }

    validate_manifest_projection_shape(&manifest_bytes)?;
    let manifest: ManifestProjection = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| CliError(format!("manifest parse failed: {error}")))?;
    let nickel_shell = nickel_request
        .map(resolve_nickel_execution_shell)
        .transpose()?;
    let suite = materialize_suite(&root, &manifest, nickel_shell.as_ref(), steel_shell)?;
    let fixture_report = validate_fixture_suite(&suite);
    let golden_report = check_golden_receipts(
        &root,
        &manifest.fixtures,
        &fixture_report.results,
        &args,
        steel_shell.is_some(),
    )?;
    let goldens_passed = golden_report.iter().all(|result| {
        result.status == GOLDEN_STATUS_MATCHED || result.status == GOLDEN_STATUS_UPDATED
    });
    let passed = fixture_report.passed && goldens_passed;
    let report = CliReport {
        passed,
        manifest_generation,
        fixture_report: Some(fixture_report),
        golden_results: golden_report,
    };
    print_json_report(&report, "validation report")?;
    Ok(if passed { EXIT_SUCCESS } else { EXIT_FAILURE })
}

fn validate_checked_manifest_generation(
    root: &RootCapability,
    args: &ValidateArgs,
    manifest_bytes: &[u8],
) -> CliResult<ManifestGenerationValidation> {
    let source_bytes = root.read(&args.manifest_source)?;
    let contract_bytes = root.read(&args.manifest_contract)?;
    let receipt_bytes = root.read(&args.manifest_generation_receipt)?;
    let receipt: ManifestGenerationReceipt = serde_json::from_slice(&receipt_bytes)
        .map_err(|error| CliError(format!("manifest generation receipt parse failed: {error}")))?;
    Ok(validate_manifest_generation_receipt(
        ManifestGenerationInput {
            source_bytes: &source_bytes,
            contract_bytes: &contract_bytes,
            evaluator_profile: MANIFEST_EVALUATOR_PROFILE,
            evaluator_descriptor: MANIFEST_EVALUATOR_DESCRIPTOR,
            projection_bytes: manifest_bytes,
        },
        &receipt,
    ))
}

fn materialize_suite(
    root: &RootCapability,
    manifest: &ManifestProjection,
    nickel_shell: Option<&NickelExecutionShell>,
    steel_shell: Option<&SteelExecutionShell>,
) -> CliResult<FixtureSuite> {
    let fsm_descriptors = materialize_fsm_descriptors(root, manifest)?;
    let mut fixtures = Vec::new();
    for projection in &manifest.fixtures {
        fixtures.push(materialize_fixture(
            root,
            projection,
            nickel_shell,
            steel_shell,
            &fsm_descriptors,
        )?);
    }
    Ok(FixtureSuite { fixtures })
}

fn materialize_fsm_descriptors(
    root: &RootCapability,
    manifest: &ManifestProjection,
) -> CliResult<BTreeMap<String, FsmSpecInput>> {
    let mut descriptors = BTreeMap::new();
    for fixture in &manifest.fixtures {
        let PayloadProjection::FsmSpec {
            schema_version,
            fsm_id,
            canonical_path,
            source_envelope_identity,
            declared_identity,
            states,
            events,
            initial_state,
            terminal_states,
            intentionally_unreachable_states,
            determinism_policy,
            totality_policy,
            guards,
            side_effects,
            transitions,
            denials,
            checked_laws,
            non_claims,
        } = &fixture.payload
        else {
            continue;
        };
        descriptors.insert(
            fixture.id.clone(),
            FsmSpecInput {
                schema_version: *schema_version,
                fsm_id: fsm_id.clone(),
                canonical_bytes: read_manifest_bytes(root, canonical_path)?,
                source_envelope_identity: source_envelope_identity.as_ref().clone(),
                declared_identity: declared_identity.clone(),
                states: states.clone(),
                events: events.clone(),
                initial_state: initial_state.clone(),
                terminal_states: terminal_states.clone(),
                intentionally_unreachable_states: intentionally_unreachable_states.clone(),
                determinism_policy: determinism_policy.clone(),
                totality_policy: totality_policy.clone(),
                guards: guards.clone(),
                side_effects: side_effects.clone(),
                transitions: transitions.clone(),
                denials: denials.clone(),
                checked_laws: checked_laws.clone(),
                non_claims: non_claims.clone(),
            },
        );
    }
    Ok(descriptors)
}

fn materialize_fixture(
    root: &RootCapability,
    projection: &FixtureProjection,
    nickel_shell: Option<&NickelExecutionShell>,
    steel_shell: Option<&SteelExecutionShell>,
    fsm_descriptors: &BTreeMap<String, FsmSpecInput>,
) -> CliResult<FixtureCase> {
    let payload = match &projection.payload {
        PayloadProjection::SourceEnvelope {
            schema_version,
            language,
            media_type,
            parser_identity,
            source_path,
            declared_identity,
            aliases,
        } => FixturePayload::SourceEnvelope(SourceEnvelopeInput {
            schema_version: *schema_version,
            language: language.clone(),
            media_type: media_type.clone(),
            parser_identity: parser_identity.clone(),
            source_bytes: read_manifest_bytes(root, source_path)?,
            declared_identity: declared_identity.clone(),
            aliases: aliases.clone(),
        }),
        PayloadProjection::CanonicalValue {
            schema_version,
            canonical_path,
            declared_identity,
            projection_identity,
            projection_claims_authority,
            aliases,
        } => FixturePayload::CanonicalValue(CanonicalValueInput {
            schema_version: *schema_version,
            canonical_bytes: read_manifest_bytes(root, canonical_path)?,
            declared_identity: declared_identity.clone(),
            projection_identity: projection_identity.clone(),
            projection_claims_authority: *projection_claims_authority,
            aliases: aliases.clone(),
        }),
        PayloadProjection::AdapterReceipt(input) => FixturePayload::AdapterReceipt(input.clone()),
        PayloadProjection::AstGrepAudit {
            schema_version,
            profile_path,
            profile_identity,
            rule_bundle_path,
            rule_bundle_identity,
            source_scope_identity,
            findings_identity,
            valence_projection_path,
            valence_projection_identity,
            language_profiles,
            rules,
            sources,
            findings,
            verification_role,
            codemod,
            non_claims,
        } => FixturePayload::AstGrepAudit(Box::new(AstGrepAuditInput {
            schema_version: schema_version.clone(),
            profile_bytes: read_manifest_bytes(root, profile_path)?,
            profile_identity: profile_identity.clone(),
            rule_bundle_bytes: read_manifest_bytes(root, rule_bundle_path)?,
            rule_bundle_identity: rule_bundle_identity.clone(),
            source_scope_identity: source_scope_identity.clone(),
            findings_identity: findings_identity.clone(),
            valence_projection_bytes: read_manifest_bytes(root, valence_projection_path)?,
            valence_projection_identity: valence_projection_identity.clone(),
            language_profiles: language_profiles.clone(),
            rules: rules.clone(),
            sources: sources
                .iter()
                .map(|source| {
                    Ok(AstGrepSourceEnvelope {
                        path: source.path.clone(),
                        bytes: read_manifest_bytes(root, &source.path)?,
                        identity: source.identity.clone(),
                    })
                })
                .collect::<CliResult<Vec<_>>>()?,
            findings: findings.clone(),
            verification_role: verification_role.clone(),
            codemod: codemod
                .as_ref()
                .as_ref()
                .map(|codemod| {
                    Ok::<AstGrepCodemodArtifact, CliError>(AstGrepCodemodArtifact {
                        recipe_bytes: read_manifest_bytes(root, &codemod.recipe_path)?,
                        recipe_identity: codemod.recipe_identity.clone(),
                        before_source_bytes: read_manifest_bytes(
                            root,
                            &codemod.before_source_path,
                        )?,
                        before_source_identity: codemod.before_source_identity.clone(),
                        after_source_bytes: read_manifest_bytes(root, &codemod.after_source_path)?,
                        after_source_identity: codemod.after_source_identity.clone(),
                        dry_run: codemod.dry_run,
                        post_check_commands: codemod.post_check_commands.clone(),
                        non_claims: codemod.non_claims.clone(),
                    })
                })
                .transpose()?,
            non_claims: non_claims.clone(),
        })),
        PayloadProjection::NickelAdapterReceipt {
            schema_version,
            adapter_kind,
            source_path,
            declared_source_identity,
            evaluator_profile,
            evaluator_descriptor,
            evaluator_identity,
            export_path,
            projection_identity,
            declared_capabilities,
            observed_effects,
            import_paths,
            sandbox_profile,
            sandbox_evidence,
            claims_canonical_ir,
            checked_laws,
            execution_mode,
        } => FixturePayload::NickelAdapterReceipt(Box::new(materialize_nickel_adapter_receipt(
            root,
            nickel_shell,
            NickelAdapterProjectionParts {
                schema_version: *schema_version,
                adapter_kind,
                source_path,
                declared_source_identity,
                evaluator_profile,
                evaluator_descriptor,
                evaluator_identity,
                export_path,
                projection_identity,
                declared_capabilities,
                observed_effects,
                import_paths,
                sandbox_profile,
                sandbox_evidence,
                claims_canonical_ir: *claims_canonical_ir,
                checked_laws,
                execution_mode,
            },
        )?)),
        PayloadProjection::SteelAdapterReceipt {
            schema_version,
            adapter_kind,
            source_path,
            declared_source_identity,
            evaluator_profile,
            evaluator_descriptor,
            evaluator_identity,
            module_descriptor_path,
            module_identity,
            input_schema,
            output_schema,
            host_functions,
            observed_host_functions,
            declared_builtin_modules,
            declared_imports,
            requested_imports,
            declared_capabilities,
            observed_effects,
            effect_class,
            output_path,
            output_projection_identity,
            output_class,
            claims_canonical_ir,
            checked_laws,
            execution_mode,
        } => FixturePayload::SteelAdapterReceipt(Box::new(materialize_steel_adapter_receipt(
            root,
            steel_shell,
            SteelAdapterProjectionParts {
                schema_version: *schema_version,
                adapter_kind,
                source_path,
                declared_source_identity,
                evaluator_profile,
                evaluator_descriptor,
                evaluator_identity,
                module_descriptor_path,
                module_identity: module_identity.as_ref(),
                input_schema,
                output_schema,
                host_functions,
                observed_host_functions,
                declared_builtin_modules,
                declared_imports,
                requested_imports,
                declared_capabilities,
                observed_effects,
                effect_class,
                output_path,
                output_projection_identity: output_projection_identity.as_ref(),
                output_class,
                claims_canonical_ir: *claims_canonical_ir,
                checked_laws,
                execution_mode,
            },
        )?)),
        PayloadProjection::TargetArtifact {
            target_kind,
            output_path,
            declared_identity,
            claims_canonical_ir,
            metadata,
        } => FixturePayload::TargetArtifact(TargetArtifactInput {
            target_kind: target_kind.clone(),
            output_bytes: read_manifest_bytes(root, output_path)?,
            declared_identity: declared_identity.clone(),
            claims_canonical_ir: *claims_canonical_ir,
            metadata: metadata.clone(),
        }),
        PayloadProjection::UnsupportedClaim(input) => {
            FixturePayload::UnsupportedClaim(input.clone())
        }
        PayloadProjection::IdentityUse(input) => FixturePayload::IdentityUse(input.clone()),
        PayloadProjection::ApplicationHostBinding(input) => {
            FixturePayload::ApplicationHostBinding(input.clone())
        }
        PayloadProjection::CapabilityShell(input) => FixturePayload::CapabilityShell(input.clone()),
        PayloadProjection::StackEvidence(input) => FixturePayload::StackEvidence(input.clone()),
        PayloadProjection::FsmSpec { .. } => {
            let descriptor = fsm_descriptors.get(&projection.id).ok_or_else(|| {
                CliError(format!(
                    "FSM descriptor fixture `{}` was not materialized",
                    projection.id
                ))
            })?;
            FixturePayload::FsmSpec(Box::new(descriptor.clone()))
        }
        PayloadProjection::FsmReceipt {
            schema_version,
            receipt_kind,
            canonical_path,
            descriptor_fixture_id,
            spec_identity,
            state_universe_identity,
            event_universe_identity,
            transition_coverage_identity,
            denial_coverage_identity,
            adapter_identity,
            dependency_identities,
            octet_receipt_identities,
            valence_export_identity,
            checked_laws,
            diagnostics,
            non_claims,
            semantic_claims,
            declared_receipt_identity,
            trace_start_state,
            trace_events,
            trace_steps,
            trace_final_state,
        } => {
            let descriptor = fsm_descriptors.get(descriptor_fixture_id).ok_or_else(|| {
                CliError(format!(
                    "FSM receipt descriptor fixture `{descriptor_fixture_id}` was not found"
                ))
            })?;
            FixturePayload::FsmReceipt(Box::new(FsmReceiptInput {
                schema_version: *schema_version,
                receipt_kind: receipt_kind.clone(),
                canonical_bytes: read_manifest_bytes(root, canonical_path)?,
                spec_identity: spec_identity.clone(),
                state_universe_identity: state_universe_identity.clone(),
                event_universe_identity: event_universe_identity.clone(),
                transition_coverage_identity: transition_coverage_identity.clone(),
                denial_coverage_identity: denial_coverage_identity.clone(),
                adapter_identity: adapter_identity.clone(),
                dependency_identities: dependency_identities.clone(),
                octet_receipt_identities: octet_receipt_identities.clone(),
                valence_export_identity: valence_export_identity.as_ref().clone(),
                checked_laws: checked_laws.clone(),
                diagnostics: diagnostics.clone(),
                non_claims: non_claims.clone(),
                semantic_claims: semantic_claims.clone(),
                declared_receipt_identity: declared_receipt_identity.as_ref().clone(),
                trace_start_state: trace_start_state.as_ref().clone(),
                trace_events: trace_events.clone(),
                trace_steps: trace_steps.clone(),
                trace_final_state: trace_final_state.as_ref().clone(),
                descriptor: Some(Box::new(descriptor.clone())),
            }))
        }
        PayloadProjection::DocumentationIr {
            schema_version,
            canonical_path,
            declared_identity,
            collector_identity,
            pages,
            target_formats,
            dependency_identities,
            checked_laws,
            non_claims,
        } => FixturePayload::DocumentationIr(Box::new(DocumentationIrInput {
            schema_version: *schema_version,
            canonical_bytes: read_manifest_bytes(root, canonical_path)?,
            declared_identity: declared_identity.clone(),
            collector_identity: collector_identity.clone(),
            pages: pages.clone(),
            target_formats: target_formats.clone(),
            dependency_identities: dependency_identities.clone(),
            checked_laws: checked_laws.clone(),
            non_claims: non_claims.clone(),
        })),
        PayloadProjection::DocumentationOutput {
            schema_version,
            documentation_ir_identity,
            renderer_identity,
            target_format,
            output_path,
            declared_output_identity,
            claims_canonical_ir,
            checked_laws,
            non_claims,
        } => FixturePayload::DocumentationOutput(Box::new(DocumentationOutputInput {
            schema_version: *schema_version,
            documentation_ir_identity: documentation_ir_identity.clone(),
            renderer_identity: renderer_identity.clone(),
            target_format: target_format.clone(),
            output_bytes: read_manifest_bytes(root, output_path)?,
            declared_output_identity: declared_output_identity.clone(),
            claims_canonical_ir: *claims_canonical_ir,
            checked_laws: checked_laws.clone(),
            non_claims: non_claims.clone(),
        })),
        PayloadProjection::DocumentationCell {
            schema_version,
            cell_id,
            language,
            source_path,
            source_identity,
            run_profile,
            execution_mode,
            evaluator_identity,
            declared_capabilities,
            observed_effects,
            expected_result,
            expected_output_identity,
            observed_output_path,
            observed_output_identity,
            expected_diagnostic,
            observed_diagnostic,
            claims_semantic_equivalence,
            checked_laws,
            non_claims,
        } => FixturePayload::DocumentationCell(Box::new(DocumentationCellInput {
            schema_version: *schema_version,
            cell_id: cell_id.clone(),
            language: language.clone(),
            source_bytes: read_manifest_bytes(root, source_path)?,
            source_identity: source_identity.as_ref().clone(),
            run_profile: run_profile.clone(),
            execution_mode: execution_mode.clone(),
            evaluator_identity: evaluator_identity.as_ref().clone(),
            declared_capabilities: declared_capabilities.clone(),
            observed_effects: observed_effects.clone(),
            expected_result: expected_result.clone(),
            expected_output_identity: expected_output_identity.as_ref().clone(),
            observed_output_bytes: read_optional_manifest_bytes(root, observed_output_path)?,
            observed_output_identity: observed_output_identity.as_ref().clone(),
            expected_diagnostic: expected_diagnostic.as_ref().clone(),
            observed_diagnostic: observed_diagnostic.as_ref().clone(),
            claims_semantic_equivalence: *claims_semantic_equivalence,
            checked_laws: checked_laws.clone(),
            non_claims: non_claims.clone(),
        })),
        PayloadProjection::DocumentationReceipt {
            schema_version,
            receipt_kind,
            canonical_path,
            documentation_ir_identity,
            renderer_identity,
            output_identities,
            cell_receipt_identities,
            valence_export_identity,
            declared_receipt_identity,
            diagnostics,
            semantic_claims,
            checked_laws,
            non_claims,
        } => FixturePayload::DocumentationReceipt(Box::new(DocumentationReceiptInput {
            schema_version: *schema_version,
            receipt_kind: receipt_kind.clone(),
            canonical_bytes: read_manifest_bytes(root, canonical_path)?,
            documentation_ir_identity: documentation_ir_identity.clone(),
            renderer_identity: renderer_identity.as_ref().clone(),
            output_identities: output_identities.clone(),
            cell_receipt_identities: cell_receipt_identities.clone(),
            valence_export_identity: valence_export_identity.as_ref().clone(),
            declared_receipt_identity: declared_receipt_identity.as_ref().clone(),
            diagnostics: diagnostics.clone(),
            semantic_claims: semantic_claims.clone(),
            checked_laws: checked_laws.clone(),
            non_claims: non_claims.clone(),
        })),
        PayloadProjection::ValenceReceiptExport {
            source_receipt_path,
            conformance_path,
            source_spans_path,
            semantic_claims,
            declared_export_identity,
        } => FixturePayload::ValenceReceiptExport(Box::new(ValenceReceiptExportInput {
            source_receipt: read_manifest_json(
                root,
                source_receipt_path,
                "Valence source receipt",
            )?,
            conformance: read_manifest_json(root, conformance_path, "Valence export conformance")?,
            source_spans: match source_spans_path {
                Some(path) => read_manifest_json(root, path, "Valence export source spans")?,
                None => SourceSpanSet::default(),
            },
            semantic_claims: semantic_claims.clone(),
            declared_export_identity: declared_export_identity.clone(),
        })),
        PayloadProjection::AlgebraicEffect {
            schema_version,
            canonical_path,
            declared_identity,
            signatures,
            input_row,
            row_open,
            claimed_pure,
            handler,
            receipt,
            replay,
            claimed_host_authority,
            admitted_host_capabilities,
        } => FixturePayload::AlgebraicEffect(Box::new(AlgebraicEffectInput {
            schema_version: *schema_version,
            canonical_bytes: read_manifest_bytes(root, canonical_path)?,
            declared_identity: declared_identity.clone(),
            signatures: signatures.clone(),
            input_row: input_row.clone(),
            row_open: *row_open,
            claimed_pure: *claimed_pure,
            handler: handler.as_ref().clone(),
            receipt: receipt.as_ref().clone(),
            replay: materialize_replay(root, replay.as_ref())?,
            claimed_host_authority: claimed_host_authority.clone(),
            admitted_host_capabilities: admitted_host_capabilities.clone(),
        })),
    };

    Ok(FixtureCase {
        id: projection.id.clone(),
        fixture_class: projection.fixture_class.clone(),
        artifact_class: projection.artifact_class.clone(),
        requirement_ids: projection.requirement_ids.clone(),
        non_claims: projection.non_claims.clone(),
        expected_verdict: projection.expected_verdict.clone(),
        checked_laws: projection.checked_laws.clone(),
        payload,
    })
}

fn materialize_nickel_adapter_receipt(
    root: &RootCapability,
    nickel_shell: Option<&NickelExecutionShell>,
    parts: NickelAdapterProjectionParts<'_>,
) -> CliResult<NickelAdapterReceiptInput> {
    let source_bytes = match parts.source_path {
        Some(path) => read_manifest_bytes(root, path)?,
        None => Vec::new(),
    };
    let stored_export_bytes = match parts.export_path {
        Some(path) => read_manifest_bytes(root, path)?,
        None => Vec::new(),
    };
    let mut execution_mode = NICKEL_EXECUTION_MODE_STORED_SANDBOX_EVIDENCE.to_string();
    let mut sandbox_profile = parts.sandbox_profile.clone();
    let mut sandbox_evidence = parts.sandbox_evidence.clone();
    let export_bytes = if parts.execution_mode == NICKEL_EXECUTION_MODE_REFERENCE_EXPORT {
        match nickel_shell {
            Some(shell) => {
                if parts.source_path.is_none() {
                    return Err(CliError(
                        "reference-export Nickel fixture requires source_path".to_string(),
                    ));
                }
                let profile = nickel_sandbox_profile().map_err(|error| {
                    CliError(format!(
                        "Nickel sandbox profile construction failed: {error}"
                    ))
                })?;
                let source_identity = compute_identity(IdentityDomain::Source, &source_bytes);
                let admitted_imports = admit_nickel_import_closure(root, parts.import_paths)?;
                let declared_evaluator_identity =
                    parts.evaluator_identity.clone().ok_or_else(|| {
                        CliError(
                            "reference-export Nickel fixture requires evaluator identity"
                                .to_string(),
                        )
                    })?;
                let version = run_nickel_sandbox(NickelSandboxRequest {
                    evaluator_command: shell.evaluator_command.clone(),
                    evaluator_arguments: vec![NICKEL_VERSION_ARGUMENT.to_string()],
                    evaluator_identity: declared_evaluator_identity,
                    sandbox_command: shell.sandbox_command.clone(),
                    limit_command: shell.limit_command.clone(),
                    runtime_closure_paths: shell.runtime_closure_paths.clone(),
                    source_file: inheritable_sandbox_data_file(
                        NICKEL_SOURCE_MEMFD_NAME,
                        &source_bytes,
                    )
                    .map_err(CliError)?,
                    source_identity: source_identity.clone(),
                    imports: materialize_nickel_import_closure(&admitted_imports)?,
                    requested_capabilities: parts.declared_capabilities.to_vec(),
                    expected_effects: parts.observed_effects.to_vec(),
                    profile: profile.clone(),
                })
                .map_err(CliError)?;
                if version.evidence.outcome != NickelSandboxOutcome::Success {
                    return Err(CliError(format!(
                        "sandboxed Nickel version probe did not complete successfully: {:?} ({:?}): {}",
                        version.evidence.outcome,
                        version.evidence.exit_code,
                        String::from_utf8_lossy(&version.stderr).trim()
                    )));
                }
                let observed_descriptor = nickel_evaluator_descriptor(&version.stdout)?;
                let observed_evaluator_identity =
                    compute_identity(IdentityDomain::Evaluator, observed_descriptor.as_bytes());
                let run = run_nickel_sandbox(NickelSandboxRequest {
                    evaluator_command: shell.evaluator_command.clone(),
                    evaluator_arguments: vec![
                        NICKEL_EXPORT_ARGUMENT.to_string(),
                        NICKEL_SANDBOX_SOURCE_PATH.to_string(),
                    ],
                    evaluator_identity: observed_evaluator_identity,
                    sandbox_command: shell.sandbox_command.clone(),
                    limit_command: shell.limit_command.clone(),
                    runtime_closure_paths: shell.runtime_closure_paths.clone(),
                    source_file: inheritable_sandbox_data_file(
                        NICKEL_SOURCE_MEMFD_NAME,
                        &source_bytes,
                    )
                    .map_err(CliError)?,
                    source_identity,
                    imports: materialize_nickel_import_closure(&admitted_imports)?,
                    requested_capabilities: parts.declared_capabilities.to_vec(),
                    expected_effects: parts.observed_effects.to_vec(),
                    profile: profile.clone(),
                })
                .map_err(CliError)?;
                execution_mode = NICKEL_EXECUTION_MODE_SANDBOXED_REFERENCE_EXPORT.to_string();
                sandbox_profile = Some(profile);
                sandbox_evidence = Some(run.evidence);
                run.stdout
            }
            None => stored_export_bytes,
        }
    } else if parts.execution_mode == NICKEL_EXECUTION_MODE_STORED_EVIDENCE {
        stored_export_bytes
    } else {
        return Err(CliError(format!(
            "unknown Nickel execution mode: {}",
            parts.execution_mode
        )));
    };

    Ok(NickelAdapterReceiptInput {
        schema_version: parts.schema_version,
        adapter_kind: parts.adapter_kind.to_string(),
        execution_mode,
        source_bytes,
        source_identity: parts.declared_source_identity.clone(),
        evaluator_profile: parts.evaluator_profile.to_string(),
        evaluator_descriptor: parts.evaluator_descriptor.to_string(),
        evaluator_identity: parts.evaluator_identity.clone(),
        export_bytes,
        projection_identity: parts.projection_identity.clone(),
        declared_capabilities: parts.declared_capabilities.to_vec(),
        expected_effects: parts.observed_effects.to_vec(),
        sandbox_profile,
        sandbox_evidence,
        claims_canonical_ir: parts.claims_canonical_ir,
        checked_laws: parts.checked_laws.to_vec(),
    })
}

struct AdmittedNickelImport {
    sandbox_path: String,
    bytes: Vec<u8>,
    identity: WireIdentity,
}

fn admit_nickel_import_closure(
    root: &RootCapability,
    import_paths: &[String],
) -> CliResult<Vec<AdmittedNickelImport>> {
    let mut imports = Vec::with_capacity(import_paths.len());
    for path in import_paths {
        let bytes = root.read(Path::new(path))?;
        let identity = compute_identity(IdentityDomain::Source, &bytes);
        imports.push(AdmittedNickelImport {
            sandbox_path: path.clone(),
            bytes,
            identity,
        });
    }
    Ok(imports)
}

fn materialize_nickel_import_closure(
    imports: &[AdmittedNickelImport],
) -> CliResult<Vec<NickelSandboxFile>> {
    imports
        .iter()
        .map(|import| {
            Ok(NickelSandboxFile {
                sandbox_path: import.sandbox_path.clone(),
                file: inheritable_sandbox_data_file(NICKEL_IMPORT_MEMFD_NAME, &import.bytes)
                    .map_err(CliError)?,
                identity: import.identity.clone(),
            })
        })
        .collect()
}

fn materialize_steel_adapter_receipt(
    root: &RootCapability,
    steel_shell: Option<&SteelExecutionShell>,
    parts: SteelAdapterProjectionParts<'_>,
) -> CliResult<SteelAdapterReceiptInput> {
    let source_bytes = match parts.source_path {
        Some(path) => read_manifest_bytes(root, path)?,
        None => Vec::new(),
    };
    let execute_probe =
        parts.execution_mode == STEEL_EXECUTION_MODE_DTO_PROBE && steel_shell.is_some();
    let mut output_bytes = if execute_probe {
        Vec::new()
    } else {
        match parts.output_path {
            Some(path) => read_manifest_bytes(root, path)?,
            None => Vec::new(),
        }
    };
    let module_descriptor_bytes = read_manifest_bytes(root, parts.module_descriptor_path)?;
    let input_bytes = if execute_probe {
        read_manifest_bytes(root, STEEL_DTO_INPUT_PATH)?
    } else {
        Vec::new()
    };
    let mut evaluator_profile = parts.evaluator_profile.to_string();
    let mut evaluator_descriptor = parts.evaluator_descriptor.to_string();
    let mut evaluator_identity = parts.evaluator_identity.clone();
    let mut input_identity = None;
    let mut output_projection_identity = parts.output_projection_identity.clone();
    let mut observed_host_functions = parts.observed_host_functions.to_vec();
    let mut observed_effects = parts.observed_effects.to_vec();
    let execution_evidence = if parts.execution_mode == STEEL_EXECUTION_MODE_DTO_PROBE {
        match steel_shell {
            Some(shell) => {
                if shell.evaluator_descriptor != STEEL_RUNNER_DESCRIPTOR {
                    return Err(CliError(
                        "caller Steel evaluator descriptor does not match the admitted embedded runner"
                            .to_string(),
                    ));
                }
                let source_identity = require_steel_identity(
                    parts.declared_source_identity,
                    IdentityDomain::Source,
                    &source_bytes,
                    "source",
                )?;
                let module_identity = require_steel_identity(
                    parts.module_identity,
                    IdentityDomain::Adapter,
                    &module_descriptor_bytes,
                    "module descriptor",
                )?;
                let verified_input_identity =
                    compute_identity(IdentityDomain::Projection, &input_bytes);
                let run = run_steel_probe(SteelProbeRequest {
                    source_bytes: &source_bytes,
                    module_descriptor_bytes: &module_descriptor_bytes,
                    input_bytes: &input_bytes,
                    input_schema: parts.input_schema,
                    output_schema: parts.output_schema,
                    output_class: parts.output_class,
                    host_functions: parts.host_functions,
                    declared_builtin_modules: parts.declared_builtin_modules,
                    declared_imports: parts.declared_imports,
                    requested_imports: parts.requested_imports,
                    declared_capabilities: parts.declared_capabilities,
                })
                .map_err(|error| CliError(format!("Steel probe execution failed: {error}")))?;
                output_bytes = run.output_bytes;
                observed_host_functions = run.observed_host_functions;
                observed_effects = run.observed_effects;
                evaluator_profile = STEEL_RUNNER_PROFILE.to_string();
                evaluator_descriptor = STEEL_RUNNER_DESCRIPTOR.to_string();
                let runner_identity =
                    compute_identity(IdentityDomain::Evaluator, evaluator_descriptor.as_bytes());
                evaluator_identity = Some(runner_identity.clone());
                input_identity = Some(verified_input_identity.clone());
                let output_identity = compute_identity(IdentityDomain::Projection, &output_bytes);
                output_projection_identity = Some(output_identity.clone());
                let input_contract = resolve_steel_dto_contract(parts.input_schema)
                    .map_err(|error| CliError(error.to_string()))?;
                let output_contract = resolve_steel_dto_contract(parts.output_schema)
                    .map_err(|error| CliError(error.to_string()))?;
                let output_byte_length = u64::try_from(output_bytes.len()).map_err(|error| {
                    CliError(format!(
                        "Steel output byte length conversion failed: {error}"
                    ))
                })?;
                SteelExecutionEvidence::ExecutedProbe(Box::new(SteelExecutedProbeEvidence {
                    runner_profile: evaluator_profile.clone(),
                    runner_descriptor: evaluator_descriptor.clone(),
                    runner_identity,
                    source_identity,
                    module_identity,
                    input_identity: verified_input_identity,
                    input_contract_identity: input_contract.identity.to_wire(),
                    output_contract_identity: output_contract.identity.to_wire(),
                    output_identity,
                    declared_builtin_modules: parts.declared_builtin_modules.to_vec(),
                    registered_host_functions: parts
                        .host_functions
                        .iter()
                        .map(|function| function.function_id.clone())
                        .collect(),
                    observed_host_functions: observed_host_functions.clone(),
                    enforced_capabilities: parts.declared_capabilities.to_vec(),
                    observed_effects: observed_effects.clone(),
                    exit_class: kamacite_core::STEEL_SUCCESS_EXIT_CLASS.to_string(),
                    output_byte_length,
                    authority_enforced: true,
                    completed: true,
                }))
            }
            None => SteelExecutionEvidence::StoredEvidence {
                fresh: false,
                output_identity: output_projection_identity.clone(),
            },
        }
    } else if parts.execution_mode == STEEL_EXECUTION_MODE_STORED_EVIDENCE {
        SteelExecutionEvidence::StoredEvidence {
            fresh: false,
            output_identity: output_projection_identity.clone(),
        }
    } else {
        return Err(CliError(format!(
            "unknown Steel execution mode: {}",
            parts.execution_mode
        )));
    };
    Ok(SteelAdapterReceiptInput {
        schema_version: parts.schema_version,
        adapter_kind: parts.adapter_kind.to_string(),
        source_bytes,
        source_identity: parts.declared_source_identity.clone(),
        evaluator_profile,
        evaluator_descriptor,
        evaluator_identity,
        module_descriptor_bytes,
        module_identity: parts.module_identity.clone(),
        input_bytes,
        input_identity,
        input_schema: parts.input_schema.to_string(),
        output_schema: parts.output_schema.to_string(),
        host_functions: parts.host_functions.to_vec(),
        observed_host_functions,
        declared_builtin_modules: parts.declared_builtin_modules.to_vec(),
        declared_imports: parts.declared_imports.to_vec(),
        requested_imports: parts.requested_imports.to_vec(),
        declared_capabilities: parts.declared_capabilities.to_vec(),
        observed_effects,
        effect_class: parts.effect_class.to_string(),
        output_bytes,
        output_projection_identity,
        output_class: parts.output_class.clone(),
        claims_canonical_ir: parts.claims_canonical_ir,
        execution_evidence,
        checked_laws: parts.checked_laws.to_vec(),
    })
}

fn require_steel_identity(
    declared_identity: &Option<WireIdentity>,
    domain: IdentityDomain,
    bytes: &[u8],
    label: &str,
) -> CliResult<WireIdentity> {
    let declared_identity = declared_identity
        .as_ref()
        .ok_or_else(|| CliError(format!("executed Steel {label} is missing identity")))?;
    let computed_identity = compute_identity(domain, bytes);
    if declared_identity != &computed_identity {
        return Err(CliError(format!(
            "executed Steel {label} identity does not match exact admitted bytes"
        )));
    }
    Ok(computed_identity)
}

fn run_nickel_version(command: &Path) -> CliResult<Vec<u8>> {
    let output = ProcessCommand::new(command)
        .arg(NICKEL_COMMAND_VERSION)
        .output()
        .map_err(|error| {
            CliError(format!(
                "Nickel version command failed for {}: {error}",
                command.display()
            ))
        })?;
    command_stdout(output, "Nickel version")
}

fn command_stdout(output: Output, label: &str) -> CliResult<Vec<u8>> {
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(CliError(format!(
            "{label} exited unsuccessfully: {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn nickel_evaluator_descriptor(version_stdout: &[u8]) -> CliResult<String> {
    let version = String::from_utf8(version_stdout.to_vec())
        .map_err(|error| CliError(format!("Nickel version output is not UTF-8: {error}")))?;
    Ok(format!(
        "{NICKEL_EVALUATOR_DESCRIPTOR_PREFIX}{}",
        version.trim()
    ))
}

fn materialize_replay(
    root: &RootCapability,
    replay: &Option<ReplayProjection>,
) -> CliResult<Option<ReplayProfileInput>> {
    let Some(replay) = replay else {
        return Ok(None);
    };
    let effect_log_bytes = match &replay.effect_log_path {
        Some(path) => read_manifest_bytes(root, path)?,
        None => Vec::new(),
    };
    Ok(Some(ReplayProfileInput {
        replay_class: replay.replay_class.clone(),
        effect_log_bytes,
        effect_log_identity: replay.effect_log_identity.clone(),
    }))
}

fn executed_steel_golden_path(
    projection: &FixtureProjection,
    default_golden_path: &str,
    executed_steel: bool,
) -> Option<String> {
    if !executed_steel {
        return None;
    }
    let PayloadProjection::SteelAdapterReceipt { execution_mode, .. } = &projection.payload else {
        return None;
    };
    if execution_mode != STEEL_EXECUTION_MODE_DTO_PROBE {
        return None;
    }
    const RECEIPT_SUFFIX: &str = ".receipt.json";
    const EXECUTED_RECEIPT_SUFFIX: &str = ".executed.receipt.json";
    Some(match default_golden_path.strip_suffix(RECEIPT_SUFFIX) {
        Some(prefix) => format!("{prefix}{EXECUTED_RECEIPT_SUFFIX}"),
        None => format!("{default_golden_path}{EXECUTED_RECEIPT_SUFFIX}"),
    })
}

fn check_golden_receipts(
    root: &RootCapability,
    projections: &[FixtureProjection],
    results: &[kamacite_core::FixtureResult],
    args: &ValidateArgs,
    executed_steel: bool,
) -> CliResult<Vec<GoldenResult>> {
    let result_by_id: BTreeMap<&str, &kamacite_core::FixtureResult> = results
        .iter()
        .map(|result| (result.id.as_str(), result))
        .collect();
    let mut golden_results = Vec::new();
    let mut updates = Vec::new();

    for projection in projections {
        let Some(default_golden_path) = projection.golden_receipt.as_ref() else {
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: String::new(),
                status: GOLDEN_STATUS_NOT_DECLARED.to_string(),
            });
            continue;
        };
        let executed_golden_path =
            executed_steel_golden_path(projection, default_golden_path, executed_steel);
        let golden_path = executed_golden_path.as_ref().unwrap_or(default_golden_path);
        if golden_path.is_empty() {
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: String::new(),
                status: GOLDEN_STATUS_NOT_DECLARED.to_string(),
            });
            continue;
        };
        let Some(result) = result_by_id.get(projection.id.as_str()) else {
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                status: GOLDEN_STATUS_MISSING_RESULT.to_string(),
            });
            continue;
        };
        let Some(receipt) = &result.receipt else {
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                status: GOLDEN_STATUS_MISSING_RECEIPT.to_string(),
            });
            continue;
        };
        let rendered = receipt_json(receipt).map_err(|error| {
            CliError(format!(
                "receipt render failed for {}: {error}",
                projection.id
            ))
        })?;
        let golden_path_ref = Path::new(golden_path);
        let existing_bytes = root.read_optional(golden_path_ref)?;
        if args.update_golden {
            let old_bytes = existing_bytes;
            root.write(golden_path_ref, rendered.as_bytes())?;
            let old_identity = old_bytes
                .as_deref()
                .map(|bytes| compute_identity(IdentityDomain::Receipt, bytes));
            let new_identity = compute_identity(IdentityDomain::Receipt, rendered.as_bytes());
            updates.push(GoldenUpdate {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                old_identity,
                new_identity,
                input_identities: receipt.body.input_identities.clone(),
                output_identities: receipt.body.output_identities.clone(),
                requirement_ids: receipt.body.requirement_ids.clone(),
                non_claims: projection.non_claims.clone(),
            });
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                status: GOLDEN_STATUS_UPDATED.to_string(),
            });
        } else {
            match existing_bytes {
                None => golden_results.push(GoldenResult {
                    fixture_id: projection.id.clone(),
                    path: golden_path.clone(),
                    status: GOLDEN_STATUS_MISSING_GOLDEN.to_string(),
                }),
                Some(existing) if existing == rendered.as_bytes() => {
                    golden_results.push(GoldenResult {
                        fixture_id: projection.id.clone(),
                        path: golden_path.clone(),
                        status: GOLDEN_STATUS_MATCHED.to_string(),
                    });
                }
                Some(_) => golden_results.push(GoldenResult {
                    fixture_id: projection.id.clone(),
                    path: golden_path.clone(),
                    status: GOLDEN_STATUS_MISMATCHED.to_string(),
                }),
            }
        }
    }

    if args.update_golden {
        let update_report = GoldenUpdateReport {
            changed_artifacts: updates,
        };
        let rendered = serde_json::to_string_pretty(&update_report)
            .map_err(|error| CliError(format!("update report serialization failed: {error}")))?;
        root.write(&args.update_report, rendered.as_bytes())?;
    }

    Ok(golden_results)
}

fn read_manifest_bytes(root: &RootCapability, path: &str) -> CliResult<Vec<u8>> {
    Ok(root.read(Path::new(path))?)
}

fn read_optional_manifest_bytes(
    root: &RootCapability,
    path: &Option<String>,
) -> CliResult<Vec<u8>> {
    match path {
        Some(path) => read_manifest_bytes(root, path),
        None => Ok(Vec::new()),
    }
}

fn read_manifest_json<T: for<'de> Deserialize<'de>>(
    root: &RootCapability,
    path: &str,
    label: &str,
) -> CliResult<T> {
    read_root_json(root, Path::new(path), label)
}

fn read_ambient_path(path: &Path) -> CliResult<Vec<u8>> {
    std::fs::read(path)
        .map_err(|error| CliError(format!("read failed for {}: {error}", path.display())))
}

fn print_help() {
    println!(
        "kamacite-validate validate [--root PATH] [--manifest PATH] [--manifest-source PATH] [--manifest-contract PATH] [--manifest-generation-receipt PATH] [--update-golden] [--update-report PATH]\n\
         kamacite-validate validate-nickel-adapter [--root PATH] [--manifest PATH] [--manifest-source PATH] [--manifest-contract PATH] [--manifest-generation-receipt PATH] [--evaluator-command PATH] [--update-golden] [--update-report PATH]\n\
         kamacite-validate validate-steel-adapter [--root PATH] [--manifest PATH] [--manifest-source PATH] [--manifest-contract PATH] [--manifest-generation-receipt PATH] [--evaluator-descriptor TEXT] [--update-golden] [--update-report PATH]\n\
         kamacite-validate hash-file --domain DOMAIN --path PATH\n\
         kamacite-validate check-manifest-projection [--source PATH] [--contract PATH] [--projection PATH] [--evaluator-command PATH]\n\
         kamacite-validate generate-manifest [--root PATH] [--source PATH] [--contract PATH] [--projection PATH] [--receipt PATH] [--evaluator-command PATH] [--update]\n\
         kamacite-validate export-valence-receipt --receipt PATH --conformance PATH [--root PATH] [--source-spans PATH] [--golden PATH] [--update-golden] [--update-report PATH]\n\
         kamacite-validate emit-function-address-receipt --report PATH --source KEY=PATH [--root PATH] [--out PATH] [--valence-out PATH]\n\
         kamacite-validate migrate-preserves [--root PATH] [--manifest PATH] [--manifest-source PATH] [--update] [--report PATH]\n\
         kamacite-validate docs collect|check|render|run-cells|update [--root PATH] [--config PATH] [--contract PATH] [--manifest PATH] [--evaluator-command PATH] [--format FORMAT] [--profile PROFILE] [--target TARGET] [--dry-run]"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FIXTURE_ID: &str = "unit.fixture";
    const TEST_OTHER_FIXTURE_ID: &str = "unit.other";
    const TEST_ZERO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const TEST_SOURCE_KEY: &str = "src/lib.rs";
    const TEST_SOURCE_PATH: &str = "src/lib.rs";
    const TEST_SOURCE_BYTES: &str = "pub fn f() -> u32 { 1 }\n";
    const TEST_FUNCTION_LOCATOR: &str = "src/lib.rs::f";
    const TEST_FUNCTION_NAME: &str = "f";
    const TEST_CANONICALIZER: &str = "octet-source-token-function-address-v1";
    const TEST_OCTET_VERSION: &str = "octet-test";
    const TEST_SYMLINK_IMPORT_PATH: &str = "fixtures/adapters/nickel/sandbox/support-link.ncl";

    fn projection_json(fixture_id: &str) -> String {
        format!(
            "{{\"schema_version\":{MANIFEST_SCHEMA_VERSION},\"authority_non_claims\":[\"test manifest only\"],\"fixtures\":[{{\"id\":\"{fixture_id}\",\"fixture_class\":\"negative\",\"artifact_class\":\"receipt\",\"requirement_ids\":[\"r[kamacite.validation_harness.fixture_manifest.projection_freshness]\"],\"non_claims\":[\"not projection authority\"],\"expected_verdict\":\"rejected\",\"checked_laws\":[],\"golden_receipt\":null,\"payload\":{{\"kind\":\"identity-use\",\"required_domain\":\"receipt\",\"supplied_identity\":{{\"domain\":\"source\",\"algorithm\":\"blake3\",\"hex\":\"{TEST_ZERO_HEX}\"}}}}}}]}}"
        )
    }

    fn valid_octet_function_address_report() -> OctetFunctionAddressReport {
        let source_identity =
            compute_identity(IdentityDomain::Source, TEST_SOURCE_BYTES.as_bytes());
        let function_address = compute_identity(IdentityDomain::FunctionAddress, b"function f");
        let dependency_identity = compute_identity(IdentityDomain::DependencyClosure, b"tokens f");
        OctetFunctionAddressReport {
            canonicalizer: TEST_CANONICALIZER.to_string(),
            tool: TEST_OCTET_VERSION.to_string(),
            producer: OctetFunctionAddressProducer {
                name: "octet".to_string(),
                role: "rust-source-shape-producer".to_string(),
                canonicalizer: String::new(),
                tool_version: String::new(),
                build_identity: None,
                normative_options: BTreeMap::new(),
                non_claims: Vec::new(),
            },
            source_envelopes: vec![OctetFunctionAddressSourceEnvelope {
                key: TEST_SOURCE_KEY.to_string(),
                path: TEST_SOURCE_PATH.to_string(),
                language: "rust".to_string(),
                source_identity: OctetIdentity::Text(format!(
                    "b3:{}",
                    source_identity.hex.as_str()
                )),
            }],
            records: vec![OctetFunctionAddressRecord {
                path: TEST_SOURCE_PATH.to_string(),
                name: TEST_FUNCTION_NAME.to_string(),
                source_key: TEST_SOURCE_KEY.to_string(),
                start_byte: 0,
                end_byte: u64::try_from(TEST_SOURCE_BYTES.len()).unwrap_or(u64::MAX),
                span: None,
                function_address: OctetIdentity::Text(format!(
                    "b3:{}",
                    function_address.hex.as_str()
                )),
                dependency_set_hash: Some(OctetIdentity::Text(format!(
                    "b3:{}",
                    dependency_identity.hex.as_str()
                ))),
                dependency_identity_kind: Some("source-token-set".to_string()),
                caveats: Vec::new(),
            }],
            dependency_edges: vec![FunctionAddressDependencyEdge {
                from_locator: TEST_FUNCTION_LOCATOR.to_string(),
                to_locator: TEST_FUNCTION_LOCATOR.to_string(),
                dependency_identity: Some(dependency_identity),
                dependency_identity_kind: Some(
                    kamacite_core::FunctionAddressDependencyIdentityKind::SourceTokenDependencySet,
                ),
            }],
            caveats: Vec::new(),
            semantic_claims: Vec::new(),
            non_claims: vec![
                "not host-language semantic correctness".to_string(),
                "not compiler correctness".to_string(),
                "not runtime behavior".to_string(),
                "not whole-program safety".to_string(),
                "not verifier soundness".to_string(),
                "not release eligibility".to_string(),
            ],
        }
    }

    fn valid_function_address_sources() -> BTreeMap<String, String> {
        let mut sources = BTreeMap::new();
        sources.insert(TEST_SOURCE_KEY.to_string(), TEST_SOURCE_BYTES.to_string());
        sources
    }

    fn migration_artifact(
        fixture_id: &str,
        old_identity: WireIdentity,
        new_identity: WireIdentity,
        content_changed: bool,
    ) -> PreservesMigrationArtifact {
        PreservesMigrationArtifact {
            fixture_id: fixture_id.to_string(),
            path: format!("fixtures/{fixture_id}.preserves"),
            old_identity: Some(old_identity),
            new_identity: Some(new_identity),
            content_changed,
            canonical_packed_size: Some(1),
            error: None,
        }
    }

    #[test]
    fn merges_migration_passes_without_reinterpreting_the_baseline_identity() {
        let old_identity = compute_identity(IdentityDomain::CanonicalValue, b"legacy");
        let intermediate_identity = compute_identity(IdentityDomain::CanonicalValue, b"first");
        let final_identity = compute_identity(IdentityDomain::CanonicalValue, b"final");
        let previous = migration_artifact(
            TEST_FIXTURE_ID,
            old_identity.clone(),
            intermediate_identity.clone(),
            true,
        );
        let current = migration_artifact(
            TEST_FIXTURE_ID,
            intermediate_identity,
            final_identity.clone(),
            false,
        );

        let result = merge_migration_artifacts(vec![previous], &[current]);

        assert!(result.is_ok());
        let Ok(artifacts) = result else {
            return;
        };
        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].old_identity.as_ref(), Some(&old_identity));
        assert_eq!(artifacts[0].new_identity.as_ref(), Some(&final_identity));
        assert!(artifacts[0].content_changed);
    }

    #[test]
    fn rejects_migration_passes_with_different_artifact_ordering() {
        let identity = compute_identity(IdentityDomain::CanonicalValue, b"value");
        let previous =
            migration_artifact(TEST_FIXTURE_ID, identity.clone(), identity.clone(), false);
        let current = migration_artifact(TEST_OTHER_FIXTURE_ID, identity.clone(), identity, false);

        let result = merge_migration_artifacts(vec![previous], &[current]);

        assert!(result.is_err());
    }

    // r[verify kamacite.findings.nickel_sandbox.import_closure]
    #[cfg(target_os = "linux")]
    #[test]
    fn rejects_symlink_import_fixture_before_sandbox_execution() {
        let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let root = RootCapability::open(&repository_root);
        assert!(root.is_ok());
        let Ok(root) = root else {
            return;
        };
        let result = admit_nickel_import_closure(&root, &[TEST_SYMLINK_IMPORT_PATH.to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn accepts_complete_manifest_projection_shape() {
        let json = projection_json(TEST_FIXTURE_ID);

        let result = validate_manifest_projection_shape(json.as_bytes());

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_duplicate_manifest_fixture_identifiers() {
        let json = projection_json(TEST_FIXTURE_ID);
        let value_result = serde_json::from_str::<serde_json::Value>(&json);
        assert!(value_result.is_ok());
        let Ok(mut value) = value_result else {
            return;
        };
        let duplicate = value["fixtures"][0].clone();
        let Some(fixtures) = value["fixtures"].as_array_mut() else {
            panic!("test manifest fixtures must be an array");
        };
        fixtures.push(duplicate);
        let duplicate_json = serde_json::to_vec(&value);
        assert!(duplicate_json.is_ok());
        let Ok(duplicate_json) = duplicate_json else {
            return;
        };

        let result = validate_manifest_projection_shape(&duplicate_json);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_manifest_payload_missing_required_metadata() {
        let json = projection_json(TEST_FIXTURE_ID);
        let value_result = serde_json::from_str::<serde_json::Value>(&json);
        assert!(value_result.is_ok());
        let Ok(mut value) = value_result else {
            return;
        };
        let Some(payload) = value["fixtures"][0]["payload"].as_object_mut() else {
            panic!("test fixture payload must be an object");
        };
        payload.remove("supplied_identity");
        let malformed_json = serde_json::to_vec(&value);
        assert!(malformed_json.is_ok());
        let Ok(malformed_json) = malformed_json else {
            return;
        };

        let result = validate_manifest_projection_shape(&malformed_json);

        assert!(result.is_err());
    }

    #[test]
    fn replaces_json_and_nickel_fixture_identity_values_without_changing_syntax() {
        const NEW_HEX: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let mut json = format!(
            "{{\n  \"id\": \"{TEST_FIXTURE_ID}\",\n  \"payload\": {{\"declared_identity\": {{\"domain\": \"canonical-value\", \"algorithm\": \"blake3\", \"hex\": \"{TEST_ZERO_HEX}\"}}}}\n}}"
        );
        let mut nickel = format!(
            "{{\n  id = \"{TEST_FIXTURE_ID}\",\n  payload.declared_identity = {{ domain = \"canonical-value\", algorithm = \"blake3\", hex = \"{TEST_ZERO_HEX}\" }},\n}}"
        );

        let json_result =
            replace_fixture_identity_hex(&mut json, TEST_FIXTURE_ID, "declared_identity", NEW_HEX);
        let nickel_result = replace_fixture_identity_hex(
            &mut nickel,
            TEST_FIXTURE_ID,
            "declared_identity",
            NEW_HEX,
        );

        assert!(json_result.is_ok());
        assert!(nickel_result.is_ok());
        assert!(serde_json::from_str::<serde_json::Value>(&json).is_ok());
        assert!(json.contains(&format!("\"hex\": \"{NEW_HEX}\"")));
        assert!(nickel.contains(&format!("hex = \"{NEW_HEX}\"")));
        assert!(!json.contains(TEST_ZERO_HEX));
        assert!(!nickel.contains(TEST_ZERO_HEX));
    }

    #[test]
    fn rejects_malformed_fixture_identity_assignment() {
        let mut malformed = format!(
            "{{\n  \"id\": \"{TEST_FIXTURE_ID}\",\n  \"declared_identity\": {{\"hex\" \"{TEST_ZERO_HEX}\"}}\n}}"
        );

        let result = replace_fixture_identity_hex(
            &mut malformed,
            TEST_FIXTURE_ID,
            "declared_identity",
            TEST_ZERO_HEX,
        );

        assert!(result.is_err());
    }

    #[test]
    fn leaves_explicitly_missing_fixture_identity_unchanged() {
        let mut missing = format!(
            "{{\n  \"id\": \"{TEST_FIXTURE_ID}\",\n  \"declared_receipt_identity\": null\n}}"
        );
        let original = missing.clone();

        let result = replace_fixture_identity_hex_if_present(
            &mut missing,
            TEST_FIXTURE_ID,
            "declared_receipt_identity",
            TEST_ZERO_HEX,
        );

        assert!(result.is_ok());
        assert_eq!(missing, original);
    }

    #[test]
    fn materializes_octet_function_address_report_with_source_bytes() {
        let report = valid_octet_function_address_report();
        let sources = valid_function_address_sources();

        let input_result = build_function_address_receipt_input(report, &sources);
        assert!(
            input_result.is_ok(),
            "valid Octet function-address input must materialize"
        );
        let Ok(input) = input_result else {
            return;
        };
        let result = validate_function_address_receipt(&input);

        assert!(
            result.passed,
            "unexpected diagnostics: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn accepts_compatibility_shaped_function_address_fixture() {
        const FIXTURE_SOURCE_KEY: &str = "fixtures/function-address/source.rs";
        let report_result = serde_json::from_str::<OctetFunctionAddressReport>(include_str!(
            "../../../fixtures/function-address/octet-valid-report.json"
        ));
        assert!(report_result.is_ok());
        let Ok(report) = report_result else {
            return;
        };
        let mut sources = BTreeMap::new();
        sources.insert(
            FIXTURE_SOURCE_KEY.to_string(),
            include_str!("../../../fixtures/function-address/source.rs").to_string(),
        );

        let input_result = build_function_address_receipt_input(report, &sources);
        assert!(input_result.is_ok());
        let Ok(input) = input_result else {
            return;
        };
        let result = validate_function_address_receipt(&input);

        assert!(
            result.passed,
            "unexpected diagnostics: {:?}",
            result.diagnostics
        );
        assert_eq!(input.producer.tool_version, "octet-fixture");
        assert_eq!(input.producer.canonicalizer, TEST_CANONICALIZER);
    }

    #[test]
    fn migration_evidence_preserves_legacy_pseudo_and_current_identities() {
        const FIXTURE_SOURCE_KEY: &str = "fixtures/function-address/source.rs";
        let report_result = serde_json::from_str::<OctetFunctionAddressReport>(include_str!(
            "../../../fixtures/function-address/octet-valid-report.json"
        ));
        assert!(report_result.is_ok());
        let Ok(report) = report_result else {
            return;
        };
        let migration_result = serde_json::from_str::<serde_json::Value>(include_str!(
            "../../../fixtures/function-address/migration-v1-to-v2.json"
        ));
        assert!(migration_result.is_ok());
        let Ok(migration) = migration_result else {
            return;
        };
        let mut sources = BTreeMap::new();
        sources.insert(
            FIXTURE_SOURCE_KEY.to_string(),
            include_str!("../../../fixtures/function-address/source.rs").to_string(),
        );
        let input_result = build_function_address_receipt_input(report, &sources);
        assert!(input_result.is_ok());
        let Ok(input) = input_result else {
            return;
        };
        let receipt_result = validate_function_address_receipt(&input);
        assert!(receipt_result.receipt.is_some());
        let Some(receipt) = receipt_result.receipt else {
            return;
        };
        let Some(profile) = receipt.body.producer_profile else {
            assert!(!receipt_result.passed);
            return;
        };

        assert_eq!(
            migration
                .pointer("/current/receipt_identity")
                .and_then(serde_json::Value::as_str),
            Some(receipt.receipt_identity.hex().as_str())
        );
        assert_eq!(
            migration
                .pointer("/current/producer_profile_identity")
                .and_then(serde_json::Value::as_str),
            Some(profile.profile_identity.hex().as_str())
        );
        assert_ne!(
            migration
                .pointer("/legacy/receipt_identity")
                .and_then(serde_json::Value::as_str),
            Some(receipt.receipt_identity.hex().as_str())
        );
        assert_ne!(
            migration
                .pointer("/previous/receipt_identity")
                .and_then(serde_json::Value::as_str),
            Some(receipt.receipt_identity.hex().as_str())
        );
    }

    #[test]
    fn rejects_compatibility_fixture_with_wrong_identity_domain() {
        const FIXTURE_SOURCE_KEY: &str = "fixtures/function-address/source.rs";
        let report_result = serde_json::from_str::<OctetFunctionAddressReport>(include_str!(
            "../../../fixtures/function-address/octet-wrong-domain-report.json"
        ));
        assert!(report_result.is_ok());
        let Ok(report) = report_result else {
            return;
        };
        let mut sources = BTreeMap::new();
        sources.insert(
            FIXTURE_SOURCE_KEY.to_string(),
            include_str!("../../../fixtures/function-address/source.rs").to_string(),
        );

        let result = build_function_address_receipt_input(report, &sources);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_function_address_report_without_matching_source_bytes() {
        let report = valid_octet_function_address_report();
        let sources = BTreeMap::new();

        let result = build_function_address_receipt_input(report, &sources);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_function_address_source_assignment() {
        let report = valid_octet_function_address_report();
        let mut sources = valid_function_address_sources();
        sources.insert("src/extra.rs".to_string(), TEST_SOURCE_BYTES.to_string());

        let result = build_function_address_receipt_input(report, &sources);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_malformed_function_address_source_flag() {
        let result = parse_source_assignment("missing-separator");

        assert!(result.is_err());
    }
}
