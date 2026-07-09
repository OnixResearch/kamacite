#![forbid(unsafe_code)]

use kamacite_core::{
    AdapterReceiptInput, AlgebraicEffectInput, ArtifactClass, Blake3Hex, CanonicalValueInput,
    CapabilityShellInput, EffectOperation, FixtureCase, FixtureClass, FixturePayload, FixtureSuite,
    HandlerDescriptorInput, HandlerReceiptInput, Identity, IdentityDomain, IdentityUseInput,
    NickelAdapterReceiptInput, ReceiptEnvelope, ReplayProfileInput, SourceEnvelopeInput,
    SourceSpanSet, StackEvidenceInput, TargetArtifactInput, UnsupportedClaimInput,
    ValenceExportConformance, ValenceReceiptExportEnvelope, ValenceReceiptExportInput,
    ValidationVerdict, compute_identity, export_valence_receipt, receipt_json,
    validate_fixture_suite,
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
const DOMAIN_ALGEBRAIC_EFFECT: &str = "algebraic-effect";
const DOMAIN_EFFECT_OPERATION: &str = "effect-operation";
const DOMAIN_EFFECT_LOG: &str = "effect-log";
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
    HashFile(HashArgs),
    CheckManifestProjection(ManifestProjectionArgs),
    ExportValenceReceipt(ValenceExportArgs),
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
struct NickelExecutionShell {
    evaluator_command: PathBuf,
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
        Command::HashFile(args) => run_hash_file(args),
        Command::CheckManifestProjection(args) => run_check_manifest_projection(args),
        Command::ExportValenceReceipt(args) => run_export_valence_receipt(args),
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
        "hash-file" => parse_hash_args(args_iter.collect()),
        "check-manifest-projection" => parse_manifest_projection_args(args_iter.collect()),
        "export-valence-receipt" => parse_valence_export_args(args_iter.collect()),
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
        DOMAIN_ALGEBRAIC_EFFECT => Ok(IdentityDomain::AlgebraicEffect),
        DOMAIN_EFFECT_OPERATION => Ok(IdentityDomain::EffectOperation),
        DOMAIN_EFFECT_LOG => Ok(IdentityDomain::EffectLog),
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
    run_validate_with_nickel_shell(args, None)
}

fn run_validate_nickel_adapter(args: NickelAdapterArgs) -> CliResult<i32> {
    let shell = NickelExecutionShell {
        evaluator_command: args.evaluator_command,
    };
    run_validate_with_nickel_shell(args.validate, Some(&shell))
}

fn run_validate_with_nickel_shell(
    args: ValidateArgs,
    nickel_shell: Option<&NickelExecutionShell>,
) -> CliResult<i32> {
    let root = args.root.clone();
    let manifest_path = resolve_under_root(&root, &args.manifest)?;
    let manifest_bytes = read_path(&manifest_path)?;
    let manifest: ManifestProjection = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| CliError(format!("manifest parse failed: {error}")))?;
    let manifest_projection =
        check_manifest_projection_if_configured(&root, &args, &manifest, &manifest_bytes)?;
    let suite = materialize_suite(&root, &manifest, nickel_shell)?;
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
        let trimmed = line.trim_start();
        if !trimmed.starts_with(FIXTURE_ID_FIELD_PREFIX) {
            continue;
        }
        if let Some(id) = extract_quoted_string(trimmed) {
            fixture_ids.push(id);
        }
    }
    fixture_ids
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
) -> CliResult<FixtureSuite> {
    let mut fixtures = Vec::new();
    for projection in &manifest.fixtures {
        fixtures.push(materialize_fixture(root, projection, nickel_shell)?);
    }
    Ok(FixtureSuite { fixtures })
}

fn materialize_fixture(
    root: &Path,
    projection: &FixtureProjection,
    nickel_shell: Option<&NickelExecutionShell>,
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
         kamacite-validate hash-file --domain DOMAIN --path PATH\n\
         kamacite-validate check-manifest-projection [--source PATH] [--projection PATH]\n\
         kamacite-validate export-valence-receipt --receipt PATH --conformance PATH [--root PATH] [--source-spans PATH] [--golden PATH] [--update-golden] [--update-report PATH]"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FIXTURE_ID: &str = "unit.fixture";
    const TEST_OTHER_FIXTURE_ID: &str = "unit.other";
    const TEST_ZERO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000000";

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
}
