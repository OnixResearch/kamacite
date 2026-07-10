#![forbid(unsafe_code)]

use kamacite_core::{
    AdapterReceiptInput, AlgebraicEffectInput, ArtifactClass, Blake3Hex, CanonicalValueInput,
    CapabilityShellInput, DocumentationCellInput, DocumentationIrInput, DocumentationOutputInput,
    DocumentationPageInput, DocumentationReceiptInput, EffectOperation, FixtureCase, FixtureClass,
    FixturePayload, FixtureSuite, FsmDenialRowInput, FsmEventInput, FsmGuardInput, FsmReceiptInput,
    FsmSideEffectInput, FsmSpecInput, FsmStateInput, FsmTraceStepInput, FsmTransitionRowInput,
    FunctionAddressCaveat, FunctionAddressDependencyEdge, FunctionAddressDependencyIdentityKind,
    FunctionAddressProducer, FunctionAddressReceiptInput, FunctionAddressRecord,
    FunctionAddressSourceEnvelope, FunctionAddressSpan, FunctionAddressValenceProjection,
    HandlerDescriptorInput, HandlerReceiptInput, Identity, IdentityDomain, IdentityUseInput,
    NickelAdapterReceiptInput, ReceiptEnvelope, ReplayProfileInput, SourceEnvelopeInput,
    SourceSpanSet, StackEvidenceInput, SteelAdapterReceiptInput, SteelHostFunctionInput,
    SteelOutputClass, TargetArtifactInput, UnsupportedClaimInput, ValenceExportConformance,
    ValenceReceiptExportEnvelope, ValenceReceiptExportInput, ValidationVerdict, compute_identity,
    export_function_address_valence_projection, export_valence_receipt,
    function_address_projection_json, receipt_json, validate_fixture_suite,
    validate_function_address_receipt,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::{Component, Path, PathBuf};
use std::process::{Command as ProcessCommand, Output};

const DEFAULT_ROOT: &str = ".";
const DEFAULT_MANIFEST: &str = "fixtures/manifest.json";
const DEFAULT_MANIFEST_SOURCE: &str = "fixtures/manifest.ncl";
const DEFAULT_UPDATE_REPORT: &str = "target/kamacite-golden-update-receipt.json";
const DEFAULT_VALENCE_EXPORT_UPDATE_REPORT: &str =
    "target/kamacite-valence-export-update-receipt.json";
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
const DOMAIN_EVIDENCE_CHAIN: &str = "evidence-chain";
const PROJECTION_IDENTITY_FIELD: &str = "projection_identity_hex";
const FIXTURE_ID_FIELD_PREFIX: &str = "id = \"";
const DIAGNOSTIC_MISSING_PROJECTION_IDENTITY: &str = "manifest-source-missing-projection-identity";
const DIAGNOSTIC_INVALID_PROJECTION_IDENTITY: &str = "manifest-source-invalid-projection-identity";
const DIAGNOSTIC_STALE_PROJECTION: &str = "manifest-projection-stale";
const DIAGNOSTIC_MISSING_SOURCE_FIXTURES: &str = "manifest-source-missing-fixture-ids";
const DIAGNOSTIC_FIXTURE_ID_DRIFT: &str = "manifest-fixture-id-drift";
const DEFAULT_NICKEL_EVALUATOR_COMMAND: &str = "nickel";
const NICKEL_COMMAND_EXPORT: &str = "export";
const NICKEL_COMMAND_VERSION: &str = "--version";
const NICKEL_EVALUATOR_DESCRIPTOR_PREFIX: &str = "nickel-reference-cli:";
const NICKEL_EXECUTION_MODE_REFERENCE_EXPORT: &str = "reference-export";
const NICKEL_EXECUTION_MODE_STORED_EVIDENCE: &str = "stored-evidence";
const DEFAULT_STEEL_EVALUATOR_DESCRIPTOR: &str =
    "steel-dto-probe-shell:kamacite-cli-dto-profile-v1";
const STEEL_EXECUTION_MODE_DTO_PROBE: &str = "dto-probe";
const STEEL_EXECUTION_MODE_STORED_EVIDENCE: &str = "stored-evidence";
const GOLDEN_STATUS_MATCHED: &str = "matched";
const GOLDEN_STATUS_UPDATED: &str = "updated";
const GOLDEN_STATUS_NOT_DECLARED: &str = "not-declared";
const GOLDEN_STATUS_MISSING_RESULT: &str = "missing-result";
const GOLDEN_STATUS_MISSING_RECEIPT: &str = "missing-receipt";
const GOLDEN_STATUS_MISSING_GOLDEN: &str = "missing-golden";
const GOLDEN_STATUS_MISMATCHED: &str = "mismatched";

#[derive(Debug)]
struct CliError(String);

impl Display for CliError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

type CliResult<T> = Result<T, CliError>;

#[derive(Clone, Debug)]
enum Command {
    Validate(ValidateArgs),
    ValidateNickelAdapter(NickelAdapterArgs),
    ValidateSteelAdapter(SteelAdapterArgs),
    HashFile(HashArgs),
    CheckManifestProjection(ManifestProjectionArgs),
    ExportValenceReceipt(ValenceExportArgs),
    EmitFunctionAddressReceipt(FunctionAddressReceiptArgs),
    Help,
}

#[derive(Clone, Debug)]
struct ValidateArgs {
    root: PathBuf,
    manifest: PathBuf,
    manifest_source: Option<PathBuf>,
    update_golden: bool,
    update_report: PathBuf,
}

#[derive(Clone, Debug)]
struct NickelAdapterArgs {
    validate: ValidateArgs,
    evaluator_command: PathBuf,
}

#[derive(Clone, Debug)]
struct SteelAdapterArgs {
    validate: ValidateArgs,
    evaluator_descriptor: String,
}

#[derive(Clone, Debug)]
struct NickelExecutionShell {
    evaluator_command: PathBuf,
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
    projection: PathBuf,
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

#[derive(Clone, Debug, Deserialize)]
struct OctetFunctionAddressReport {
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
}

#[derive(Clone, Debug, Deserialize)]
struct OctetFunctionAddressSourceEnvelope {
    #[serde(default)]
    key: String,
    #[serde(default, alias = "display_path")]
    path: String,
    language: String,
    source_identity: String,
}

#[derive(Clone, Debug, Deserialize)]
struct OctetFunctionAddressRecord {
    #[serde(default, alias = "locator")]
    path: String,
    #[serde(default, alias = "display_name")]
    name: String,
    #[serde(default)]
    source_key: String,
    start_byte: u64,
    end_byte: u64,
    function_address: String,
    #[serde(default, alias = "dependency_closure")]
    dependency_set_hash: Option<String>,
    #[serde(default)]
    dependency_identity_kind: Option<String>,
    #[serde(default)]
    caveats: Vec<String>,
}

#[derive(Debug, Serialize)]
struct FunctionAddressReceiptCommandReport {
    passed: bool,
    diagnostics: Vec<kamacite_core::Diagnostic>,
    receipt_identity: Option<Identity>,
    receipt_path: Option<String>,
    valence_projection_path: Option<String>,
    valence_projection: Option<FunctionAddressValenceProjection>,
}

#[derive(Clone, Debug, Deserialize)]
struct ManifestProjection {
    fixtures: Vec<FixtureProjection>,
}

#[derive(Clone, Debug, Deserialize)]
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
#[serde(tag = "kind", rename_all = "kebab-case")]
enum PayloadProjection {
    SourceEnvelope {
        schema_version: u32,
        language: String,
        media_type: String,
        parser_identity: String,
        source_path: String,
        declared_identity: Identity,
        aliases: Vec<String>,
    },
    CanonicalValue {
        schema_version: u32,
        canonical_path: String,
        declared_identity: Identity,
        projection_identity: Option<Identity>,
        projection_claims_authority: bool,
        aliases: Vec<String>,
    },
    AdapterReceipt(AdapterReceiptInput),
    NickelAdapterReceipt {
        schema_version: u32,
        adapter_kind: String,
        source_path: Option<String>,
        declared_source_identity: Option<Identity>,
        evaluator_profile: String,
        evaluator_descriptor: String,
        evaluator_identity: Option<Identity>,
        export_path: Option<String>,
        projection_identity: Option<Identity>,
        declared_capabilities: Vec<String>,
        observed_effects: Vec<String>,
        claims_canonical_ir: bool,
        checked_laws: Vec<String>,
        execution_mode: String,
    },
    SteelAdapterReceipt {
        schema_version: u32,
        adapter_kind: String,
        source_path: Option<String>,
        declared_source_identity: Option<Identity>,
        evaluator_profile: String,
        evaluator_descriptor: String,
        evaluator_identity: Option<Identity>,
        module_descriptor_path: String,
        module_identity: Box<Option<Identity>>,
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
        output_projection_identity: Box<Option<Identity>>,
        output_class: SteelOutputClass,
        claims_canonical_ir: bool,
        checked_laws: Vec<String>,
        execution_mode: String,
    },
    TargetArtifact {
        target_kind: String,
        output_path: String,
        declared_identity: Identity,
        claims_canonical_ir: bool,
        metadata: BTreeMap<String, String>,
    },
    UnsupportedClaim(UnsupportedClaimInput),
    IdentityUse(IdentityUseInput),
    CapabilityShell(CapabilityShellInput),
    StackEvidence(StackEvidenceInput),
    FsmSpec {
        schema_version: u32,
        fsm_id: String,
        canonical_path: String,
        source_envelope_identity: Box<Option<Identity>>,
        declared_identity: Identity,
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
        spec_identity: Identity,
        state_universe_identity: Identity,
        event_universe_identity: Identity,
        transition_coverage_identity: Identity,
        denial_coverage_identity: Identity,
        adapter_identity: Identity,
        dependency_identities: Vec<Identity>,
        octet_receipt_identities: Vec<Identity>,
        valence_export_identity: Box<Option<Identity>>,
        checked_laws: Vec<String>,
        diagnostics: Vec<String>,
        non_claims: Vec<String>,
        semantic_claims: Vec<String>,
        declared_receipt_identity: Box<Option<Identity>>,
        trace_start_state: Box<Option<String>>,
        trace_events: Vec<String>,
        trace_steps: Vec<FsmTraceStepInput>,
        trace_final_state: Box<Option<String>>,
    },
    DocumentationIr {
        schema_version: u32,
        canonical_path: String,
        declared_identity: Identity,
        collector_identity: Identity,
        pages: Vec<DocumentationPageInput>,
        target_formats: Vec<String>,
        dependency_identities: Vec<Identity>,
        checked_laws: Vec<String>,
        non_claims: Vec<String>,
    },
    DocumentationOutput {
        schema_version: u32,
        documentation_ir_identity: Identity,
        renderer_identity: Identity,
        target_format: String,
        output_path: String,
        declared_output_identity: Identity,
        claims_canonical_ir: bool,
        checked_laws: Vec<String>,
        non_claims: Vec<String>,
    },
    DocumentationCell {
        schema_version: u32,
        cell_id: String,
        language: String,
        source_path: String,
        source_identity: Box<Option<Identity>>,
        run_profile: String,
        execution_mode: String,
        evaluator_identity: Box<Option<Identity>>,
        declared_capabilities: Vec<String>,
        observed_effects: Vec<String>,
        expected_result: String,
        expected_output_identity: Box<Option<Identity>>,
        observed_output_path: Option<String>,
        observed_output_identity: Box<Option<Identity>>,
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
        documentation_ir_identity: Identity,
        renderer_identity: Box<Option<Identity>>,
        output_identities: Vec<Identity>,
        cell_receipt_identities: Vec<Identity>,
        valence_export_identity: Box<Option<Identity>>,
        declared_receipt_identity: Box<Option<Identity>>,
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
        declared_export_identity: Option<Identity>,
    },
    AlgebraicEffect {
        schema_version: u32,
        canonical_path: String,
        declared_identity: Identity,
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
    effect_log_identity: Option<Identity>,
}

struct NickelAdapterProjectionParts<'a> {
    schema_version: u32,
    adapter_kind: &'a str,
    source_path: &'a Option<String>,
    declared_source_identity: &'a Option<Identity>,
    evaluator_profile: &'a str,
    evaluator_descriptor: &'a str,
    evaluator_identity: &'a Option<Identity>,
    export_path: &'a Option<String>,
    projection_identity: &'a Option<Identity>,
    declared_capabilities: &'a [String],
    observed_effects: &'a [String],
    claims_canonical_ir: bool,
    checked_laws: &'a [String],
    execution_mode: &'a str,
}

struct SteelAdapterProjectionParts<'a> {
    schema_version: u32,
    adapter_kind: &'a str,
    source_path: &'a Option<String>,
    declared_source_identity: &'a Option<Identity>,
    evaluator_profile: &'a str,
    evaluator_descriptor: &'a str,
    evaluator_identity: &'a Option<Identity>,
    module_descriptor_path: &'a str,
    module_identity: &'a Option<Identity>,
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
    output_projection_identity: &'a Option<Identity>,
    output_class: &'a SteelOutputClass,
    claims_canonical_ir: bool,
    checked_laws: &'a [String],
    execution_mode: &'a str,
}

#[derive(Debug, Serialize)]
struct CliReport {
    passed: bool,
    manifest_projection: Option<ManifestProjectionReport>,
    fixture_report: kamacite_core::FixtureSuiteReport,
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
    declared_projection_identity: Option<Identity>,
    computed_projection_identity: Identity,
    source_fixture_count: usize,
    projection_fixture_count: usize,
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
    old_identity: Option<Identity>,
    new_identity: Identity,
    input_identities: BTreeMap<String, Identity>,
    output_identities: BTreeMap<String, Identity>,
    requirement_ids: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ValenceExportCommandReport {
    passed: bool,
    validation_passed: bool,
    diagnostics: Vec<kamacite_core::Diagnostic>,
    export_identity: Option<Identity>,
    golden_result: Option<ValenceExportGoldenResult>,
    export: Option<ValenceReceiptExportEnvelope>,
}

#[derive(Debug, Serialize)]
struct ValenceExportGoldenResult {
    path: String,
    status: String,
    old_export_identity: Option<Identity>,
    new_export_identity: Option<Identity>,
}

#[derive(Debug, Serialize)]
struct ValenceExportUpdateReport {
    changed_exports: Vec<ValenceExportUpdate>,
}

#[derive(Debug, Serialize)]
struct ValenceExportUpdate {
    path: String,
    source_receipt_identity: Identity,
    old_export_identity: Option<Identity>,
    new_export_identity: Identity,
    checked_laws: Vec<String>,
    non_claims: Vec<String>,
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
        Command::HashFile(args) => run_hash_file(args),
        Command::CheckManifestProjection(args) => run_check_manifest_projection(args),
        Command::ExportValenceReceipt(args) => run_export_valence_receipt(args),
        Command::EmitFunctionAddressReceipt(args) => run_emit_function_address_receipt(args),
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
        "hash-file" => parse_hash_args(args_iter.collect()),
        "check-manifest-projection" => parse_manifest_projection_args(args_iter.collect()),
        "export-valence-receipt" => parse_valence_export_args(args_iter.collect()),
        "emit-function-address-receipt" => parse_function_address_receipt_args(args_iter.collect()),
        "help" | "--help" | "-h" => Ok(Command::Help),
        unknown => Err(CliError(format!("unknown command: {unknown}"))),
    }
}

fn default_validate_args() -> ValidateArgs {
    ValidateArgs {
        root: PathBuf::from(DEFAULT_ROOT),
        manifest: PathBuf::from(DEFAULT_MANIFEST),
        manifest_source: Some(PathBuf::from(DEFAULT_MANIFEST_SOURCE)),
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
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--evaluator-command" => {
                evaluator_command = next_path(&mut index, "--evaluator-command")?
            }
            _ => {
                validate_args.push(flag);
                if validate_args
                    .last()
                    .is_some_and(|value| value == "--update-golden")
                    || validate_args
                        .last()
                        .is_some_and(|value| value == "--skip-manifest-source-check")
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
                    || validate_args
                        .last()
                        .is_some_and(|value| value == "--skip-manifest-source-check")
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

fn parse_common_validate_args(args: Vec<String>) -> CliResult<ValidateArgs> {
    let mut parsed = default_validate_args();
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--root" => parsed.root = next_path(&mut index, "--root")?,
            "--manifest" => parsed.manifest = next_path(&mut index, "--manifest")?,
            "--manifest-source" => {
                parsed.manifest_source = Some(next_path(&mut index, "--manifest-source")?)
            }
            "--skip-manifest-source-check" => parsed.manifest_source = None,
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
        projection: PathBuf::from(DEFAULT_MANIFEST),
    };
    let mut index = args.into_iter();
    while let Some(flag) = index.next() {
        match flag.as_str() {
            "--source" => parsed.source = next_path(&mut index, "--source")?,
            "--projection" => parsed.projection = next_path(&mut index, "--projection")?,
            unknown => {
                return Err(CliError(format!(
                    "unknown check-manifest-projection argument: {unknown}"
                )));
            }
        }
    }
    Ok(Command::CheckManifestProjection(parsed))
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
        DOMAIN_EVIDENCE_CHAIN => Ok(IdentityDomain::EvidenceChain),
        unknown => Err(CliError(format!("unknown identity domain: {unknown}"))),
    }
}

fn run_hash_file(args: HashArgs) -> CliResult<i32> {
    let bytes = read_path(&args.path)?;
    let identity = compute_identity(args.domain, &bytes);
    let json = serde_json::to_string_pretty(&identity)
        .map_err(|error| CliError(format!("hash serialization failed: {error}")))?;
    println!("{json}");
    Ok(EXIT_SUCCESS)
}

fn run_check_manifest_projection(args: ManifestProjectionArgs) -> CliResult<i32> {
    let source_bytes = read_path(&args.source)?;
    let projection_bytes = read_path(&args.projection)?;
    let source = String::from_utf8(source_bytes).map_err(|error| {
        CliError(format!(
            "manifest source is not UTF-8 for {}: {error}",
            args.source.display()
        ))
    })?;
    let projection: ManifestProjection = serde_json::from_slice(&projection_bytes)
        .map_err(|error| CliError(format!("manifest projection parse failed: {error}")))?;
    let report = check_manifest_projection(&source, &projection, &projection_bytes);
    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| CliError(format!("projection report serialization failed: {error}")))?;
    println!("{json}");
    if report.passed {
        Ok(EXIT_SUCCESS)
    } else {
        Ok(EXIT_FAILURE)
    }
}

fn run_export_valence_receipt(args: ValenceExportArgs) -> CliResult<i32> {
    let root = args.root.clone();
    let receipt_path = resolve_under_root(&root, &args.receipt)?;
    let conformance_path = resolve_under_root(&root, &args.conformance)?;
    let source_receipt: ReceiptEnvelope = read_json(&receipt_path, "Valence source receipt")?;
    let conformance: ValenceExportConformance =
        read_json(&conformance_path, "Valence export conformance")?;
    let source_spans = match &args.source_spans {
        Some(path) => read_json(
            &resolve_under_root(&root, path)?,
            "Valence export source spans",
        )?,
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
    let root = args.root.clone();
    let report_path = resolve_under_root(&root, &args.report)?;
    let report: OctetFunctionAddressReport =
        read_json(&report_path, "Octet function-address report")?;
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
            receipt_path: args.out.as_ref().map(path_display),
            valence_projection_path: args.valence_out.as_ref().map(path_display),
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
            let resolved = resolve_under_root(&root, path)?;
            write_path(&resolved, receipt_json.as_bytes())?;
            Some(path_display(path))
        }
        None => None,
    };

    let (valence_projection, valence_projection_path) = match &args.valence_out {
        Some(path) => {
            let projection = export_function_address_valence_projection(&receipt);
            let rendered = serde_json::to_string_pretty(&projection).map_err(|error| {
                CliError(format!(
                    "function-address Valence projection serialization failed: {error}"
                ))
            })?;
            let resolved = resolve_under_root(&root, path)?;
            write_path(&resolved, rendered.as_bytes())?;
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
    root: &Path,
    sources: &BTreeMap<String, PathBuf>,
) -> CliResult<BTreeMap<String, String>> {
    let mut source_bytes = BTreeMap::new();
    for (key, path) in sources {
        let resolved = resolve_under_root(root, path)?;
        let bytes = read_path(&resolved)?;
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
    let tool_version = if report.tool.is_empty() {
        report.producer.role
    } else {
        report.tool
    };
    Ok(FunctionAddressReceiptInput {
        producer: FunctionAddressProducer {
            name: report.producer.name,
            canonicalizer: report.canonicalizer,
            tool_version,
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
    Ok(FunctionAddressRecord {
        locator: format!("{}::{}", record.path, record.name),
        display_name: record.name,
        source_key: source_key.clone(),
        span: Some(FunctionAddressSpan {
            source_key,
            start_byte: record.start_byte,
            end_byte: record.end_byte,
        }),
        function_address: octet_identity(
            IdentityDomain::FunctionAddress,
            &record.function_address,
        )?,
        dependency_closure: record
            .dependency_set_hash
            .as_deref()
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

fn octet_identity(domain: IdentityDomain, value: &str) -> CliResult<Identity> {
    let hex = value.strip_prefix("b3:").unwrap_or(value);
    let parsed = Blake3Hex::parse(hex.to_owned())
        .map_err(|error| CliError(format!("invalid Octet BLAKE3 identity `{value}`: {error}")))?;
    Ok(Identity::new(domain, parsed))
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

fn path_display(path: &PathBuf) -> String {
    path.display().to_string()
}

fn check_or_update_valence_export_golden(
    root: &Path,
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
    let resolved = resolve_under_root(root, golden_path)?;
    if args.update_golden {
        let old_export_identity = read_existing_valence_export_identity(&resolved)?;
        write_path(&resolved, rendered.as_bytes())?;
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
    if !resolved.exists() {
        return Ok((
            Some(ValenceExportGoldenResult {
                path: golden_path.display().to_string(),
                status: GOLDEN_STATUS_MISSING_GOLDEN.to_string(),
                old_export_identity: None,
                new_export_identity: Some(export.export_identity.clone()),
            }),
            false,
        ));
    }
    let existing = read_path(&resolved)?;
    let matched = existing == rendered.as_bytes();
    Ok((
        Some(ValenceExportGoldenResult {
            path: golden_path.display().to_string(),
            status: if matched {
                GOLDEN_STATUS_MATCHED.to_string()
            } else {
                GOLDEN_STATUS_MISMATCHED.to_string()
            },
            old_export_identity: read_existing_valence_export_identity(&resolved)?,
            new_export_identity: Some(export.export_identity.clone()),
        }),
        matched,
    ))
}

fn write_valence_export_update_report(
    root: &Path,
    args: &ValenceExportArgs,
    export: &ValenceReceiptExportEnvelope,
    old_export_identity: Option<Identity>,
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
    let path = resolve_under_root(root, &args.update_report)?;
    write_path(&path, rendered.as_bytes())
}

fn read_existing_valence_export_identity(path: &Path) -> CliResult<Option<Identity>> {
    if !path.exists() {
        return Ok(None);
    }
    let bytes = read_path(path)?;
    let parsed: Result<ValenceReceiptExportEnvelope, serde_json::Error> =
        serde_json::from_slice(&bytes);
    match parsed {
        Ok(export) => Ok(Some(export.export_identity)),
        Err(_) => Ok(None),
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path, label: &str) -> CliResult<T> {
    let bytes = read_path(path)?;
    serde_json::from_slice(&bytes).map_err(|error| {
        CliError(format!(
            "{label} parse failed for {}: {error}",
            path.display()
        ))
    })
}

fn run_validate(args: ValidateArgs) -> CliResult<i32> {
    run_validate_with_shells(args, None, None)
}

fn run_validate_nickel_adapter(args: NickelAdapterArgs) -> CliResult<i32> {
    let shell = NickelExecutionShell {
        evaluator_command: args.evaluator_command,
    };
    run_validate_with_shells(args.validate, Some(&shell), None)
}

fn run_validate_steel_adapter(args: SteelAdapterArgs) -> CliResult<i32> {
    let shell = SteelExecutionShell {
        evaluator_descriptor: args.evaluator_descriptor,
    };
    run_validate_with_shells(args.validate, None, Some(&shell))
}

fn run_validate_with_shells(
    args: ValidateArgs,
    nickel_shell: Option<&NickelExecutionShell>,
    steel_shell: Option<&SteelExecutionShell>,
) -> CliResult<i32> {
    let root = args.root.clone();
    let manifest_path = resolve_under_root(&root, &args.manifest)?;
    let manifest_bytes = read_path(&manifest_path)?;
    let manifest: ManifestProjection = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| CliError(format!("manifest parse failed: {error}")))?;
    let manifest_projection =
        check_manifest_projection_if_configured(&root, &args, &manifest, &manifest_bytes)?;
    let suite = materialize_suite(&root, &manifest, nickel_shell, steel_shell)?;
    let fixture_report = validate_fixture_suite(&suite);
    let golden_report =
        check_golden_receipts(&root, &manifest.fixtures, &fixture_report.results, &args)?;
    let goldens_passed = golden_report.iter().all(|result| {
        result.status == GOLDEN_STATUS_MATCHED || result.status == GOLDEN_STATUS_UPDATED
    });
    let manifest_projection_passed = match &manifest_projection {
        Some(report) => report.passed,
        None => true,
    };
    let passed = manifest_projection_passed && fixture_report.passed && goldens_passed;
    let report = CliReport {
        passed,
        manifest_projection,
        fixture_report,
        golden_results: golden_report,
    };
    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| CliError(format!("report serialization failed: {error}")))?;
    println!("{json}");
    if passed {
        Ok(EXIT_SUCCESS)
    } else {
        Ok(EXIT_FAILURE)
    }
}

fn check_manifest_projection_if_configured(
    root: &Path,
    args: &ValidateArgs,
    manifest: &ManifestProjection,
    manifest_bytes: &[u8],
) -> CliResult<Option<ManifestProjectionReport>> {
    let Some(source_path) = &args.manifest_source else {
        return Ok(None);
    };
    let resolved = resolve_under_root(root, source_path)?;
    let source_bytes = read_path(&resolved)?;
    let source = String::from_utf8(source_bytes).map_err(|error| {
        CliError(format!(
            "manifest source is not UTF-8 for {}: {error}",
            resolved.display()
        ))
    })?;
    Ok(Some(check_manifest_projection(
        &source,
        manifest,
        manifest_bytes,
    )))
}

fn check_manifest_projection(
    source: &str,
    projection: &ManifestProjection,
    projection_bytes: &[u8],
) -> ManifestProjectionReport {
    let computed_projection_identity =
        compute_identity(IdentityDomain::Projection, projection_bytes);
    let mut diagnostics = Vec::new();
    let declared_projection_identity =
        match extract_string_assignment(source, PROJECTION_IDENTITY_FIELD) {
            Some(hex) => match Blake3Hex::parse(hex) {
                Ok(parsed) => Some(Identity::new(IdentityDomain::Projection, parsed)),
                Err(_) => {
                    diagnostics.push(DIAGNOSTIC_INVALID_PROJECTION_IDENTITY.to_string());
                    None
                }
            },
            None => {
                diagnostics.push(DIAGNOSTIC_MISSING_PROJECTION_IDENTITY.to_string());
                None
            }
        };
    if let Some(declared) = &declared_projection_identity
        && declared.hex != computed_projection_identity.hex
    {
        diagnostics.push(DIAGNOSTIC_STALE_PROJECTION.to_string());
    }

    let source_fixture_ids = extract_source_fixture_ids(source);
    let projection_fixture_ids: Vec<&str> = projection
        .fixtures
        .iter()
        .map(|fixture| fixture.id.as_str())
        .collect();
    if source_fixture_ids.is_empty() {
        diagnostics.push(DIAGNOSTIC_MISSING_SOURCE_FIXTURES.to_string());
    }
    if source_fixture_ids != projection_fixture_ids {
        diagnostics.push(DIAGNOSTIC_FIXTURE_ID_DRIFT.to_string());
    }
    diagnostics.sort();
    diagnostics.dedup();

    ManifestProjectionReport {
        passed: diagnostics.is_empty(),
        declared_projection_identity,
        computed_projection_identity,
        source_fixture_count: source_fixture_ids.len(),
        projection_fixture_count: projection_fixture_ids.len(),
        diagnostics,
    }
}

fn extract_string_assignment(source: &str, field_name: &str) -> Option<String> {
    for line in source.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with(field_name) {
            continue;
        }
        let Some((_, assignment)) = trimmed.split_once('=') else {
            continue;
        };
        return extract_quoted_string(assignment).map(ToOwned::to_owned);
    }
    None
}

fn extract_source_fixture_ids(source: &str) -> Vec<&str> {
    let mut fixture_ids = Vec::new();
    for line in source.lines() {
        for (prefix_start, _) in line.match_indices(FIXTURE_ID_FIELD_PREFIX) {
            if !is_fixture_id_field_boundary(line, prefix_start) {
                continue;
            }
            if let Some(id) = extract_quoted_string(&line[prefix_start..]) {
                fixture_ids.push(id);
            }
        }
    }
    fixture_ids
}

fn is_fixture_id_field_boundary(line: &str, prefix_start: usize) -> bool {
    prefix_start == 0
        || line[..prefix_start]
            .chars()
            .next_back()
            .is_some_and(|character| {
                !(character.is_ascii_alphanumeric() || character == '_' || character == '-')
            })
}

fn extract_quoted_string(text: &str) -> Option<&str> {
    let start = text.find('"')?;
    let tail = &text[start + 1..];
    let end = tail.find('"')?;
    Some(&tail[..end])
}

fn materialize_suite(
    root: &Path,
    manifest: &ManifestProjection,
    nickel_shell: Option<&NickelExecutionShell>,
    steel_shell: Option<&SteelExecutionShell>,
) -> CliResult<FixtureSuite> {
    let mut fixtures = Vec::new();
    for projection in &manifest.fixtures {
        fixtures.push(materialize_fixture(
            root,
            projection,
            nickel_shell,
            steel_shell,
        )?);
    }
    Ok(FixtureSuite { fixtures })
}

fn materialize_fixture(
    root: &Path,
    projection: &FixtureProjection,
    nickel_shell: Option<&NickelExecutionShell>,
    steel_shell: Option<&SteelExecutionShell>,
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
            claims_canonical_ir,
            checked_laws,
            execution_mode,
        } => FixturePayload::NickelAdapterReceipt(materialize_nickel_adapter_receipt(
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
                claims_canonical_ir: *claims_canonical_ir,
                checked_laws,
                execution_mode,
            },
        )?),
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
        PayloadProjection::CapabilityShell(input) => FixturePayload::CapabilityShell(input.clone()),
        PayloadProjection::StackEvidence(input) => FixturePayload::StackEvidence(input.clone()),
        PayloadProjection::FsmSpec {
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
        } => FixturePayload::FsmSpec(Box::new(FsmSpecInput {
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
        })),
        PayloadProjection::FsmReceipt {
            schema_version,
            receipt_kind,
            canonical_path,
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
        } => FixturePayload::FsmReceipt(Box::new(FsmReceiptInput {
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
        })),
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
    root: &Path,
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
    let mut evaluator_descriptor = parts.evaluator_descriptor.to_string();
    let mut evaluator_identity = parts.evaluator_identity.clone();
    let export_bytes = if parts.execution_mode == NICKEL_EXECUTION_MODE_REFERENCE_EXPORT {
        match nickel_shell {
            Some(shell) => {
                let Some(source_path) = parts.source_path else {
                    return Err(CliError(
                        "reference-export Nickel fixture requires source_path".to_string(),
                    ));
                };
                evaluator_descriptor =
                    nickel_evaluator_descriptor(&run_nickel_version(&shell.evaluator_command)?)?;
                evaluator_identity = Some(compute_identity(
                    IdentityDomain::Evaluator,
                    evaluator_descriptor.as_bytes(),
                ));
                let resolved_source = resolve_under_root(root, Path::new(source_path))?;
                run_nickel_export(&shell.evaluator_command, &resolved_source)?
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
        source_bytes,
        source_identity: parts.declared_source_identity.clone(),
        evaluator_profile: parts.evaluator_profile.to_string(),
        evaluator_descriptor,
        evaluator_identity,
        export_bytes,
        projection_identity: parts.projection_identity.clone(),
        declared_capabilities: parts.declared_capabilities.to_vec(),
        observed_effects: parts.observed_effects.to_vec(),
        claims_canonical_ir: parts.claims_canonical_ir,
        checked_laws: parts.checked_laws.to_vec(),
    })
}

fn materialize_steel_adapter_receipt(
    root: &Path,
    steel_shell: Option<&SteelExecutionShell>,
    parts: SteelAdapterProjectionParts<'_>,
) -> CliResult<SteelAdapterReceiptInput> {
    let source_bytes = match parts.source_path {
        Some(path) => read_manifest_bytes(root, path)?,
        None => Vec::new(),
    };
    let output_bytes = match parts.output_path {
        Some(path) => read_manifest_bytes(root, path)?,
        None => Vec::new(),
    };
    let module_descriptor_bytes = read_manifest_bytes(root, parts.module_descriptor_path)?;
    let mut evaluator_descriptor = parts.evaluator_descriptor.to_string();
    let mut evaluator_identity = parts.evaluator_identity.clone();
    if parts.execution_mode == STEEL_EXECUTION_MODE_DTO_PROBE {
        if let Some(shell) = steel_shell {
            evaluator_descriptor = shell.evaluator_descriptor.clone();
            evaluator_identity = Some(compute_identity(
                IdentityDomain::Evaluator,
                evaluator_descriptor.as_bytes(),
            ));
        }
    } else if parts.execution_mode != STEEL_EXECUTION_MODE_STORED_EVIDENCE {
        return Err(CliError(format!(
            "unknown Steel execution mode: {}",
            parts.execution_mode
        )));
    }
    Ok(SteelAdapterReceiptInput {
        schema_version: parts.schema_version,
        adapter_kind: parts.adapter_kind.to_string(),
        source_bytes,
        source_identity: parts.declared_source_identity.clone(),
        evaluator_profile: parts.evaluator_profile.to_string(),
        evaluator_descriptor,
        evaluator_identity,
        module_descriptor_bytes,
        module_identity: parts.module_identity.clone(),
        input_schema: parts.input_schema.to_string(),
        output_schema: parts.output_schema.to_string(),
        host_functions: parts.host_functions.to_vec(),
        observed_host_functions: parts.observed_host_functions.to_vec(),
        declared_builtin_modules: parts.declared_builtin_modules.to_vec(),
        declared_imports: parts.declared_imports.to_vec(),
        requested_imports: parts.requested_imports.to_vec(),
        declared_capabilities: parts.declared_capabilities.to_vec(),
        observed_effects: parts.observed_effects.to_vec(),
        effect_class: parts.effect_class.to_string(),
        output_bytes,
        output_projection_identity: parts.output_projection_identity.clone(),
        output_class: parts.output_class.clone(),
        claims_canonical_ir: parts.claims_canonical_ir,
        checked_laws: parts.checked_laws.to_vec(),
    })
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

fn run_nickel_export(command: &Path, source: &Path) -> CliResult<Vec<u8>> {
    let output = ProcessCommand::new(command)
        .arg(NICKEL_COMMAND_EXPORT)
        .arg(source)
        .output()
        .map_err(|error| {
            CliError(format!(
                "Nickel export command failed for {}: {error}",
                source.display()
            ))
        })?;
    command_stdout(output, "Nickel export")
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
    root: &Path,
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

fn check_golden_receipts(
    root: &Path,
    projections: &[FixtureProjection],
    results: &[kamacite_core::FixtureResult],
    args: &ValidateArgs,
) -> CliResult<Vec<GoldenResult>> {
    let result_by_id: BTreeMap<&str, &kamacite_core::FixtureResult> = results
        .iter()
        .map(|result| (result.id.as_str(), result))
        .collect();
    let mut golden_results = Vec::new();
    let mut updates = Vec::new();

    for projection in projections {
        let Some(golden_path) = &projection.golden_receipt else {
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
        let path = root.join(golden_path);
        if args.update_golden {
            let old_bytes = if path.exists() {
                Some(read_path(&path)?)
            } else {
                None
            };
            write_path(&path, rendered.as_bytes())?;
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
        } else if !path.exists() {
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                status: GOLDEN_STATUS_MISSING_GOLDEN.to_string(),
            });
        } else {
            let existing = read_path(&path)?;
            if existing == rendered.as_bytes() {
                golden_results.push(GoldenResult {
                    fixture_id: projection.id.clone(),
                    path: golden_path.clone(),
                    status: GOLDEN_STATUS_MATCHED.to_string(),
                });
            } else {
                golden_results.push(GoldenResult {
                    fixture_id: projection.id.clone(),
                    path: golden_path.clone(),
                    status: GOLDEN_STATUS_MISMATCHED.to_string(),
                });
            }
        }
    }

    if args.update_golden {
        let update_report = GoldenUpdateReport {
            changed_artifacts: updates,
        };
        let rendered = serde_json::to_string_pretty(&update_report)
            .map_err(|error| CliError(format!("update report serialization failed: {error}")))?;
        let path = root.join(&args.update_report);
        write_path(&path, rendered.as_bytes())?;
    }

    Ok(golden_results)
}

fn read_manifest_bytes(root: &Path, path: &str) -> CliResult<Vec<u8>> {
    let resolved = resolve_under_root(root, Path::new(path))?;
    read_path(&resolved)
}

fn read_optional_manifest_bytes(root: &Path, path: &Option<String>) -> CliResult<Vec<u8>> {
    match path {
        Some(path) => read_manifest_bytes(root, path),
        None => Ok(Vec::new()),
    }
}

fn read_manifest_json<T: for<'de> Deserialize<'de>>(
    root: &Path,
    path: &str,
    label: &str,
) -> CliResult<T> {
    let resolved = resolve_under_root(root, Path::new(path))?;
    read_json(&resolved, label)
}

fn resolve_under_root(root: &Path, path: &Path) -> CliResult<PathBuf> {
    if path.is_absolute() {
        return Err(CliError(format!(
            "manifest path must be relative: {}",
            path.display()
        )));
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(CliError(format!(
            "manifest path cannot escape root: {}",
            path.display()
        )));
    }
    Ok(root.join(path))
}

fn read_path(path: &Path) -> CliResult<Vec<u8>> {
    std::fs::read(path)
        .map_err(|error| CliError(format!("read failed for {}: {error}", path.display())))
}

fn write_path(path: &Path, bytes: &[u8]) -> CliResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            CliError(format!(
                "create dir failed for {}: {error}",
                parent.display()
            ))
        })?;
    }
    std::fs::write(path, bytes)
        .map_err(|error| CliError(format!("write failed for {}: {error}", path.display())))
}

fn print_help() {
    println!(
        "kamacite-validate validate [--root PATH] [--manifest PATH] [--manifest-source PATH] [--skip-manifest-source-check] [--update-golden] [--update-report PATH]\n\
         kamacite-validate validate-nickel-adapter [--root PATH] [--manifest PATH] [--manifest-source PATH] [--evaluator-command PATH] [--update-golden] [--update-report PATH]\n\
         kamacite-validate validate-steel-adapter [--root PATH] [--manifest PATH] [--manifest-source PATH] [--evaluator-descriptor TEXT] [--update-golden] [--update-report PATH]\n\
         kamacite-validate hash-file --domain DOMAIN --path PATH\n\
         kamacite-validate check-manifest-projection [--source PATH] [--projection PATH]\n\
         kamacite-validate export-valence-receipt --receipt PATH --conformance PATH [--root PATH] [--source-spans PATH] [--golden PATH] [--update-golden] [--update-report PATH]\n\
         kamacite-validate emit-function-address-receipt --report PATH --source KEY=PATH [--root PATH] [--out PATH] [--valence-out PATH]"
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

    fn projection_json(fixture_id: &str) -> String {
        format!(
            "{{\"fixtures\":[{{\"id\":\"{fixture_id}\",\"fixture_class\":\"negative\",\"artifact_class\":\"receipt\",\"requirement_ids\":[\"r[kamacite.validation_harness.fixture_manifest.projection_freshness]\"],\"non_claims\":[\"not projection authority\"],\"expected_verdict\":\"rejected\",\"checked_laws\":[],\"payload\":{{\"kind\":\"identity-use\",\"required_domain\":\"receipt\",\"supplied_identity\":{{\"domain\":\"source\",\"algorithm\":\"blake3\",\"hex\":\"{TEST_ZERO_HEX}\"}}}}}}]}}"
        )
    }

    fn source_manifest(projection_hex: &str, fixture_id: &str) -> String {
        format!(
            "projection_identity_hex = \"{projection_hex}\"\nfixtures = [\n  {{\n    id = \"{fixture_id}\",\n  }},\n]\n"
        )
    }

    fn parse_projection(json: &str) -> ManifestProjection {
        match serde_json::from_str(json) {
            Ok(parsed) => parsed,
            Err(error) => panic!("test projection must parse: {error}"),
        }
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
            },
            source_envelopes: vec![OctetFunctionAddressSourceEnvelope {
                key: TEST_SOURCE_KEY.to_string(),
                path: TEST_SOURCE_PATH.to_string(),
                language: "rust".to_string(),
                source_identity: format!("b3:{}", source_identity.hex.as_str()),
            }],
            records: vec![OctetFunctionAddressRecord {
                path: TEST_SOURCE_PATH.to_string(),
                name: TEST_FUNCTION_NAME.to_string(),
                source_key: TEST_SOURCE_KEY.to_string(),
                start_byte: 0,
                end_byte: u64::try_from(TEST_SOURCE_BYTES.len())
                    .expect("test source length fits u64"),
                function_address: format!("b3:{}", function_address.hex.as_str()),
                dependency_set_hash: Some(format!("b3:{}", dependency_identity.hex.as_str())),
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

    #[test]
    fn accepts_matching_manifest_projection_identity() {
        let json = projection_json(TEST_FIXTURE_ID);
        let computed = compute_identity(IdentityDomain::Projection, json.as_bytes());
        let source = source_manifest(computed.hex.as_str(), TEST_FIXTURE_ID);
        let projection = parse_projection(&json);

        let report = check_manifest_projection(&source, &projection, json.as_bytes());

        assert!(report.passed);
        assert!(report.diagnostics.is_empty());
        assert_eq!(report.source_fixture_count, report.projection_fixture_count);
    }

    #[test]
    fn rejects_stale_manifest_projection_identity() {
        let json = projection_json(TEST_FIXTURE_ID);
        let source = source_manifest(TEST_ZERO_HEX, TEST_FIXTURE_ID);
        let projection = parse_projection(&json);

        let report = check_manifest_projection(&source, &projection, json.as_bytes());

        assert!(!report.passed);
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic == DIAGNOSTIC_STALE_PROJECTION)
        );
    }

    #[test]
    fn rejects_manifest_projection_fixture_id_drift() {
        let json = projection_json(TEST_FIXTURE_ID);
        let computed = compute_identity(IdentityDomain::Projection, json.as_bytes());
        let source = source_manifest(computed.hex.as_str(), TEST_OTHER_FIXTURE_ID);
        let projection = parse_projection(&json);

        let report = check_manifest_projection(&source, &projection, json.as_bytes());

        assert!(!report.passed);
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic == DIAGNOSTIC_FIXTURE_ID_DRIFT)
        );
    }

    #[test]
    fn materializes_octet_function_address_report_with_source_bytes() {
        let report = valid_octet_function_address_report();
        let sources = valid_function_address_sources();

        let input = match build_function_address_receipt_input(report, &sources) {
            Ok(input) => input,
            Err(error) => panic!("test input should materialize: {error}"),
        };
        let result = validate_function_address_receipt(&input);

        assert!(
            result.passed,
            "unexpected diagnostics: {:?}",
            result.diagnostics
        );
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
