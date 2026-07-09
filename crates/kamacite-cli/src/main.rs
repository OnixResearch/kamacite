#![forbid(unsafe_code)]

use kamacite_core::{
    AdapterReceiptInput, AlgebraicEffectInput, ArtifactClass, Blake3Hex, CanonicalValueInput,
    CapabilityShellInput, EffectOperation, FixtureCase, FixtureClass, FixturePayload, FixtureSuite,
    HandlerDescriptorInput, HandlerReceiptInput, Identity, IdentityDomain, IdentityUseInput,
    ReplayProfileInput, SourceEnvelopeInput, StackEvidenceInput, TargetArtifactInput,
    UnsupportedClaimInput, ValidationVerdict, compute_identity, receipt_json,
    validate_fixture_suite,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::{Component, Path, PathBuf};

const DEFAULT_ROOT: &str = ".";
const DEFAULT_MANIFEST: &str = "fixtures/manifest.json";
const DEFAULT_MANIFEST_SOURCE: &str = "fixtures/manifest.ncl";
const DEFAULT_UPDATE_REPORT: &str = "target/kamacite-golden-update-receipt.json";
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
    HashFile(HashArgs),
    CheckManifestProjection(ManifestProjectionArgs),
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
struct HashArgs {
    domain: IdentityDomain,
    path: PathBuf,
}

#[derive(Clone, Debug)]
struct ManifestProjectionArgs {
    source: PathBuf,
    projection: PathBuf,
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
        Command::HashFile(args) => run_hash_file(args),
        Command::CheckManifestProjection(args) => run_check_manifest_projection(args),
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
        "hash-file" => parse_hash_args(args_iter.collect()),
        "check-manifest-projection" => parse_manifest_projection_args(args_iter.collect()),
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
    Ok(Command::Validate(parsed))
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

fn run_validate(args: ValidateArgs) -> CliResult<i32> {
    let root = args.root.clone();
    let manifest_path = resolve_under_root(&root, &args.manifest)?;
    let manifest_bytes = read_path(&manifest_path)?;
    let manifest: ManifestProjection = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| CliError(format!("manifest parse failed: {error}")))?;
    let manifest_projection =
        check_manifest_projection_if_configured(&root, &args, &manifest, &manifest_bytes)?;
    let suite = materialize_suite(&root, &manifest)?;
    let fixture_report = validate_fixture_suite(&suite);
    let golden_report =
        check_golden_receipts(&root, &manifest.fixtures, &fixture_report.results, &args)?;
    let goldens_passed = golden_report
        .iter()
        .all(|result| result.status == "matched" || result.status == "updated");
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

fn materialize_suite(root: &Path, manifest: &ManifestProjection) -> CliResult<FixtureSuite> {
    let mut fixtures = Vec::new();
    for projection in &manifest.fixtures {
        fixtures.push(materialize_fixture(root, projection)?);
    }
    Ok(FixtureSuite { fixtures })
}

fn materialize_fixture(root: &Path, projection: &FixtureProjection) -> CliResult<FixtureCase> {
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
                status: "not-declared".to_string(),
            });
            continue;
        };
        let Some(result) = result_by_id.get(projection.id.as_str()) else {
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                status: "missing-result".to_string(),
            });
            continue;
        };
        let Some(receipt) = &result.receipt else {
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                status: "missing-receipt".to_string(),
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
                non_claims: projection.non_claims.clone(),
            });
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                status: "updated".to_string(),
            });
        } else if !path.exists() {
            golden_results.push(GoldenResult {
                fixture_id: projection.id.clone(),
                path: golden_path.clone(),
                status: "missing-golden".to_string(),
            });
        } else {
            let existing = read_path(&path)?;
            if existing == rendered.as_bytes() {
                golden_results.push(GoldenResult {
                    fixture_id: projection.id.clone(),
                    path: golden_path.clone(),
                    status: "matched".to_string(),
                });
            } else {
                golden_results.push(GoldenResult {
                    fixture_id: projection.id.clone(),
                    path: golden_path.clone(),
                    status: "mismatched".to_string(),
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
         kamacite-validate hash-file --domain DOMAIN --path PATH\n\
         kamacite-validate check-manifest-projection [--source PATH] [--projection PATH]"
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
