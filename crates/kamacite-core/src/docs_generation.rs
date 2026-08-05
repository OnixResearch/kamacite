use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::fixture::{
    DocumentationCellDescriptor, DocumentationIrInput, DocumentationPageInput,
    DocumentationSectionInput,
};
use crate::identity::{
    AdapterIdentity, DocumentationFactsIdentity, DocumentationOutputIdentity, IdentityDomain,
    WireIdentity, compute_identity,
};
use crate::preserves_codec::{PreservesLimits, PreservesSchema, encode_typed_preserves};
use crate::receipt::{ReceiptError, canonical_json_bytes};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const LANGUAGE_DOCUMENTATION_FACTS_SCHEMA_VERSION: u32 = 1;
pub const DOCUMENTATION_IR_SCHEMA_VERSION: u32 = 1;
pub const DOCS_CONFIG_SCHEMA_VERSION: u32 = 1;
const REQUIREMENT_LANGUAGE_FACTS: &str = "r[kamacite.docs.language_facts.schema]";
const REQUIREMENT_LANGUAGE_VALIDATION: &str = "r[kamacite.docs.language_facts.validation]";
const REQUIREMENT_SOURCE_BINDING: &str = "r[kamacite.docs.language_facts.source_binding]";
const REQUIREMENT_NON_CLAIMS: &str = "r[kamacite.docs.language_facts.non_claims]";
const REQUIREMENT_IR_ASSEMBLY: &str = "r[kamacite.docs.ir_assembly]";
const REQUIREMENT_ANCHORS: &str = "r[kamacite.docs.ir_assembly.anchors]";
const REQUIREMENT_LINKS: &str = "r[kamacite.docs.ir_assembly.links]";
const REQUIREMENT_WASM_SCHEMA: &str = "r[kamacite.docs.wasm_runner.schema]";
const REQUIREMENT_WASM_VALIDATION: &str = "r[kamacite.docs.wasm_runner.validation]";
const REQUIREMENT_WASM_EFFECTS: &str = "r[kamacite.docs.wasm_runner.effects]";
const REQUIREMENT_WASM_LIMITS: &str = "r[kamacite.docs.wasm_runner.resource_limits]";
const REQUIREMENT_DOCS_CONFIG: &str = "r[kamacite.findings.docs_commands.config_validation]";
const LANGUAGE_RUST: &str = "rust";
const LANGUAGE_NICKEL: &str = "nickel";
const LANGUAGE_STEEL: &str = "steel";
const NON_CLAIM_COMPILER_CORRECTNESS: &str = "not compiler correctness";
const NON_CLAIM_EVALUATOR_SOUNDNESS: &str = "not evaluator soundness";
const NON_CLAIM_SEMANTIC_EQUIVALENCE: &str = "not source-language semantic equivalence";
const NON_CLAIM_RENDERED_AUTHORITY: &str = "not rendered-output authority";
const NON_CLAIM_WASM_CANONICAL_IR: &str = "wasm is not canonical Kamacite IR";
const REQUIRED_FACT_NON_CLAIMS: &[&str] = &[
    NON_CLAIM_COMPILER_CORRECTNESS,
    NON_CLAIM_EVALUATOR_SOUNDNESS,
    NON_CLAIM_SEMANTIC_EQUIVALENCE,
    NON_CLAIM_RENDERED_AUTHORITY,
];
const REQUIRED_WASM_NON_CLAIMS: &[&str] =
    &[NON_CLAIM_WASM_CANONICAL_IR, NON_CLAIM_SEMANTIC_EQUIVALENCE];
const FORBIDDEN_DOCS_CLAIMS: &[&str] = &[
    "proves compiler correctness",
    "proves evaluator soundness",
    "proves source-language semantic equivalence",
    "rendered output is canonical",
    "wasm is canonical kamacite ir",
];
const DEFAULT_PAGE_ID: &str = "generated";
const DEFAULT_PAGE_TITLE: &str = "Generated documentation";
const DEFAULT_TARGET_FORMAT: &str = "markdown";
const CHECKED_LAW_SOURCE_BOUND: &str =
    "language documentation facts remain source-bound projections";
const CHECKED_LAW_STABLE_ANCHORS: &str = "documentation anchors are deterministic";
const DOCUMENTATION_IR_PRESERVES_SCHEMA_VERSION: &str = "kamacite.documentation-ir.v2";
const DOCUMENTATION_IR_ROOT_LABEL: &str = "kamacite-documentation-ir-v2";
const DOCS_LINK_POLICY: &str = "identity-only";
const DOCS_UPDATE_POLICY: &str = "explicit-docs-update-only";
const DOCS_CHECK_PROFILE: &str = "check-only";
const DOCS_RENDER_FORMATS: &[&str] = &["asciidoc", "html", "markdown"];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceByteSpan {
    pub start_byte: u64,
    pub end_byte: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RunnableCellReference {
    pub cell_id: String,
    pub language: String,
    pub source_identity: WireIdentity,
    pub required_capabilities: Vec<String>,
    pub expected_result: String,
    pub evaluator_identity: Option<WireIdentity>,
    pub wasm_target: Option<WasmTargetBinding>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WasmTargetBinding {
    pub module_identity: WireIdentity,
    pub runner_profile_identity: WireIdentity,
    pub expected_output_identity: Option<WireIdentity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanguageDocumentationItem {
    pub item_id: String,
    pub symbol_path: String,
    pub title: String,
    pub signature: Option<String>,
    pub contract_summary: Option<String>,
    pub doc_blocks: Vec<String>,
    pub code_blocks: Vec<String>,
    pub source_span: SourceByteSpan,
    pub links: Vec<String>,
    pub runnable_cells: Vec<RunnableCellReference>,
    pub dependency_identities: Vec<WireIdentity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanguageDocumentationFactsInput {
    pub schema_version: u32,
    pub language: String,
    pub source_identity: WireIdentity,
    pub source_len: u64,
    pub collector_identity: WireIdentity,
    pub collector_profile_identity: WireIdentity,
    pub items: Vec<LanguageDocumentationItem>,
    pub diagnostics: Vec<String>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
    pub semantic_claims: Vec<String>,
    pub declared_facts_identity: Option<WireIdentity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LanguageDocumentationFactsArtifact {
    pub schema_version: u32,
    pub language: String,
    pub source_identity: WireIdentity,
    pub collector_identity: WireIdentity,
    pub collector_profile_identity: WireIdentity,
    pub items: Vec<LanguageDocumentationItem>,
    pub diagnostics: Vec<String>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
    pub facts_identity: WireIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocsAssemblyInput {
    pub facts: Vec<LanguageDocumentationFactsArtifact>,
    pub target_formats: Vec<String>,
    pub page_grouping: BTreeMap<String, String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WasmRunnerProfileInput {
    pub profile_id: String,
    pub engine_identity: WireIdentity,
    pub declared_imports: Vec<String>,
    pub declared_capabilities: Vec<String>,
    pub fuel_limit: u64,
    pub memory_limit_bytes: u64,
    pub output_limit_bytes: u64,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WasmExecutionRecordInput {
    pub cell_id: String,
    pub source_identity: WireIdentity,
    pub module_identity: WireIdentity,
    pub runner_profile_identity: WireIdentity,
    pub engine_identity: WireIdentity,
    pub declared_imports: Vec<String>,
    pub observed_imports: Vec<String>,
    pub declared_capabilities: Vec<String>,
    pub observed_effects: Vec<String>,
    pub fuel_used: u64,
    pub memory_used_bytes: u64,
    pub output_bytes: Vec<u8>,
    pub expected_output_identity: Option<WireIdentity>,
    pub diagnostics: Vec<String>,
    pub non_claims: Vec<String>,
    pub semantic_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocsSourceRootConfig {
    pub path: String,
    pub language: String,
    pub collector_profile: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocsOutputConfig {
    pub format: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocsPageGroupingConfig {
    pub pattern: String,
    pub page: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocsWasmRunnerConfig {
    pub profile: String,
    pub engine_identity_policy: String,
    pub imports: Vec<String>,
    pub capabilities: Vec<String>,
    pub fuel_limit: u64,
    pub memory_limit_bytes: u64,
    pub output_limit_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocsConfigProjection {
    pub schema_version: u32,
    pub source_roots: Vec<DocsSourceRootConfig>,
    pub output_formats: Vec<DocsOutputConfig>,
    pub page_grouping: Vec<DocsPageGroupingConfig>,
    pub link_policy: String,
    pub run_profiles: Vec<String>,
    pub wasm_runner_profiles: Vec<DocsWasmRunnerConfig>,
    pub update_policy: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocsActionKind {
    Collect,
    Check,
    Render,
    RunCells,
    Update,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocsCompletionState {
    Planned,
    Completed,
    Unsupported,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocsActionEvidenceInput {
    pub action: DocsActionKind,
    pub format: Option<String>,
    pub profile: Option<String>,
    pub target: Option<String>,
    pub dry_run: bool,
    pub completed_work_items: usize,
    pub documentation_ir_identity: Option<WireIdentity>,
    pub output_identity: Option<DocumentationOutputIdentity>,
    pub executed: bool,
    pub mutated: bool,
    pub diagnostics: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocsActionOutcome {
    pub passed: bool,
    pub state: DocsCompletionState,
    pub action: DocsActionKind,
    pub completed_work_items: usize,
    pub documentation_ir_identity: Option<WireIdentity>,
    pub output_identity: Option<DocumentationOutputIdentity>,
    pub executed: bool,
    pub mutated: bool,
    pub diagnostics: Vec<String>,
}

#[must_use]
pub fn validate_language_documentation_facts(
    input: &LanguageDocumentationFactsInput,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if input.schema_version != LANGUAGE_DOCUMENTATION_FACTS_SCHEMA_VERSION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_LANGUAGE_FACTS,
                "schema_version",
                "language documentation facts schema version is unsupported",
            )
            .with_detail("actual", input.schema_version.to_string())
            .with_detail(
                "expected",
                LANGUAGE_DOCUMENTATION_FACTS_SCHEMA_VERSION.to_string(),
            ),
        );
    }
    validate_language(&input.language, "language", &mut diagnostics);
    validate_identity_domain(
        &input.source_identity,
        IdentityDomain::Source,
        "source_identity",
        &mut diagnostics,
    );
    validate_identity_domain(
        &input.collector_identity,
        IdentityDomain::Adapter,
        "collector_identity",
        &mut diagnostics,
    );
    validate_identity_domain(
        &input.collector_profile_identity,
        IdentityDomain::Adapter,
        "collector_profile_identity",
        &mut diagnostics,
    );
    validate_facts_items(input, &mut diagnostics);
    validate_required_non_claims(
        &input.non_claims,
        REQUIRED_FACT_NON_CLAIMS,
        "non_claims",
        &mut diagnostics,
    );
    validate_semantic_claims(&input.semantic_claims, "semantic_claims", &mut diagnostics);
    if let Some(declared) = &input.declared_facts_identity {
        validate_identity_domain(
            declared,
            IdentityDomain::DocumentationFacts,
            "declared_facts_identity",
            &mut diagnostics,
        );
        match language_documentation_facts_identity(input) {
            Ok(computed)
                if declared.domain == IdentityDomain::DocumentationFacts
                    && declared.hex != *computed.hex() =>
            {
                diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::StaleIdentity,
                        REQUIREMENT_LANGUAGE_FACTS,
                        "declared_facts_identity.hex",
                        "declared language documentation facts identity is stale",
                    )
                    .with_detail("declared", declared.hex.as_str())
                    .with_detail("computed", computed.hex().as_str()),
                );
            }
            Ok(_) => {}
            Err(error) => diagnostics.push(canonicalization_diagnostic(
                "declared_facts_identity",
                "language documentation facts identity",
                &error,
            )),
        }
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

pub fn language_documentation_facts_artifact(
    input: &LanguageDocumentationFactsInput,
) -> Result<LanguageDocumentationFactsArtifact, Vec<Diagnostic>> {
    let diagnostics = validate_language_documentation_facts(input);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    let facts_identity = language_documentation_facts_identity(input).map_err(|error| {
        vec![canonicalization_diagnostic(
            "facts_identity",
            "language documentation facts identity",
            &error,
        )]
    })?;
    Ok(LanguageDocumentationFactsArtifact {
        schema_version: input.schema_version,
        language: input.language.clone(),
        source_identity: input.source_identity.clone(),
        collector_identity: input.collector_identity.clone(),
        collector_profile_identity: input.collector_profile_identity.clone(),
        items: sorted_items(&input.items),
        diagnostics: sorted_strings(&input.diagnostics),
        checked_laws: sorted_strings(&input.checked_laws),
        non_claims: sorted_strings(&input.non_claims),
        facts_identity: facts_identity.to_wire(),
    })
}

pub fn assemble_documentation_ir(
    input: &DocsAssemblyInput,
) -> Result<DocumentationIrInput, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    if input.facts.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_IR_ASSEMBLY,
            "facts",
            "documentation IR assembly requires at least one language facts artifact",
        ));
    }
    let mut item_ids = BTreeSet::new();
    let mut duplicate_ids = BTreeSet::new();
    let mut known_links = BTreeSet::new();
    for facts in &input.facts {
        for item in &facts.items {
            if !item_ids.insert(item.item_id.clone()) {
                duplicate_ids.insert(item.item_id.clone());
            }
            known_links.insert(item.item_id.clone());
        }
    }
    for duplicate in duplicate_ids {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::DuplicateArtifactIdentity,
                REQUIREMENT_ANCHORS,
                "facts.items.item_id",
                "language documentation item identity would produce duplicate documentation anchors",
            )
            .with_detail("item_id", duplicate),
        );
    }
    for facts in &input.facts {
        for item in &facts.items {
            for link in &item.links {
                if !known_links.contains(link) {
                    diagnostics.push(
                        Diagnostic::new(
                            DiagnosticCode::MissingInput,
                            REQUIREMENT_LINKS,
                            "facts.items.links",
                            "language documentation item link target is unknown",
                        )
                        .with_detail("link", link.clone()),
                    );
                }
            }
        }
    }
    validate_required_non_claims(
        &input.non_claims,
        REQUIRED_FACT_NON_CLAIMS,
        "non_claims",
        &mut diagnostics,
    );
    if !diagnostics.is_empty() {
        diagnostics.sort();
        diagnostics.dedup();
        return Err(diagnostics);
    }
    let pages = documentation_pages(input);
    let canonical_bytes =
        documentation_ir_canonical_bytes(&pages, &input.target_formats, &input.non_claims)
            .map_err(|error| {
                vec![canonicalization_diagnostic(
                    "canonical_bytes",
                    "documentation IR",
                    &error,
                )]
            })?;
    let declared_identity = compute_identity(IdentityDomain::DocumentationIr, &canonical_bytes);
    let collector_identity = docs_collector_identity(&input.facts).map_err(|error| {
        vec![canonicalization_diagnostic(
            "collector_identity",
            "documentation collector identity",
            &error,
        )]
    })?;
    let dependency_identities = input
        .facts
        .iter()
        .map(|facts| facts.facts_identity.clone())
        .chain(
            input
                .facts
                .iter()
                .flat_map(|facts| facts.items.iter())
                .flat_map(|item| item.dependency_identities.iter().cloned()),
        )
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    Ok(DocumentationIrInput {
        schema_version: DOCUMENTATION_IR_SCHEMA_VERSION,
        canonical_bytes,
        declared_identity,
        collector_identity: collector_identity.to_wire(),
        pages,
        target_formats: sorted_or_default(&input.target_formats),
        dependency_identities,
        checked_laws: vec![
            CHECKED_LAW_SOURCE_BOUND.to_string(),
            CHECKED_LAW_STABLE_ANCHORS.to_string(),
        ],
        non_claims: sorted_strings(&input.non_claims),
    })
}

#[must_use]
pub fn validate_wasm_runner_profile(input: &WasmRunnerProfileInput) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_required_text(&input.profile_id, "profile_id", &mut diagnostics);
    validate_identity_domain(
        &input.engine_identity,
        IdentityDomain::Evaluator,
        "engine_identity",
        &mut diagnostics,
    );
    validate_non_empty_strings(
        &input.declared_imports,
        "declared_imports",
        &mut diagnostics,
    );
    validate_non_empty_strings(
        &input.declared_capabilities,
        "declared_capabilities",
        &mut diagnostics,
    );
    validate_limit(input.fuel_limit, "fuel_limit", &mut diagnostics);
    validate_limit(
        input.memory_limit_bytes,
        "memory_limit_bytes",
        &mut diagnostics,
    );
    validate_limit(
        input.output_limit_bytes,
        "output_limit_bytes",
        &mut diagnostics,
    );
    validate_required_non_claims(
        &input.non_claims,
        REQUIRED_WASM_NON_CLAIMS,
        "non_claims",
        &mut diagnostics,
    );
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

#[must_use]
pub fn validate_wasm_execution_record(
    profile: &WasmRunnerProfileInput,
    record: &WasmExecutionRecordInput,
) -> Vec<Diagnostic> {
    let mut diagnostics = validate_wasm_runner_profile(profile);
    validate_required_text(&record.cell_id, "cell_id", &mut diagnostics);
    validate_identity_domain(
        &record.source_identity,
        IdentityDomain::Source,
        "source_identity",
        &mut diagnostics,
    );
    validate_identity_domain(
        &record.module_identity,
        IdentityDomain::TargetArtifact,
        "module_identity",
        &mut diagnostics,
    );
    validate_identity_domain(
        &record.runner_profile_identity,
        IdentityDomain::Adapter,
        "runner_profile_identity",
        &mut diagnostics,
    );
    if record.engine_identity != profile.engine_identity {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_WASM_VALIDATION,
            "engine_identity",
            "WASM execution engine identity does not match the runner profile",
        ));
    }
    validate_observed_subset(
        &profile.declared_imports,
        &record.observed_imports,
        "observed_imports",
        &mut diagnostics,
    );
    validate_observed_subset(
        &profile.declared_capabilities,
        &record.observed_effects,
        "observed_effects",
        &mut diagnostics,
    );
    if record.fuel_used > profile.fuel_limit {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedClaim,
            REQUIREMENT_WASM_LIMITS,
            "fuel_used",
            "WASM execution exceeded declared fuel limit",
        ));
    }
    if record.memory_used_bytes > profile.memory_limit_bytes {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedClaim,
            REQUIREMENT_WASM_LIMITS,
            "memory_used_bytes",
            "WASM execution exceeded declared memory limit",
        ));
    }
    if u64::try_from(record.output_bytes.len()).unwrap_or(u64::MAX) > profile.output_limit_bytes {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedClaim,
            REQUIREMENT_WASM_LIMITS,
            "output_bytes",
            "WASM execution exceeded declared output limit",
        ));
    }
    if let Some(expected) = &record.expected_output_identity {
        let computed = compute_identity(IdentityDomain::Projection, &record.output_bytes);
        if expected.domain != IdentityDomain::Projection || expected.hex != computed.hex {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::GoldenReceiptMismatch,
                REQUIREMENT_WASM_VALIDATION,
                "expected_output_identity",
                "WASM execution output identity does not match observed bytes",
            ));
        }
    }
    validate_required_non_claims(
        &record.non_claims,
        REQUIRED_WASM_NON_CLAIMS,
        "non_claims",
        &mut diagnostics,
    );
    validate_semantic_claims(&record.semantic_claims, "semantic_claims", &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

#[must_use]
pub fn validate_docs_config(config: &DocsConfigProjection) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if config.schema_version != DOCS_CONFIG_SCHEMA_VERSION {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedSchemaVersion,
            REQUIREMENT_DOCS_CONFIG,
            "docs_config.schema_version",
            "documentation configuration schema version is unsupported",
        ));
    }
    if config.source_roots.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_CONFIG,
            "docs_config.source_roots",
            "documentation configuration requires at least one source root",
        ));
    }
    let mut source_paths = BTreeSet::new();
    for source in &config.source_roots {
        validate_required_text(
            &source.path,
            "docs_config.source_roots.path",
            &mut diagnostics,
        );
        validate_language(
            &source.language,
            "docs_config.source_roots.language",
            &mut diagnostics,
        );
        validate_required_text(
            &source.collector_profile,
            "docs_config.source_roots.collector_profile",
            &mut diagnostics,
        );
        if !source_paths.insert(source.path.clone()) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::DuplicateArtifactIdentity,
                REQUIREMENT_DOCS_CONFIG,
                "docs_config.source_roots.path",
                "documentation source-root paths must be unique",
            ));
        }
    }
    if config.output_formats.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_CONFIG,
            "docs_config.output_formats",
            "documentation configuration requires at least one output format",
        ));
    }
    let mut output_formats = BTreeSet::new();
    for output in &config.output_formats {
        validate_required_text(
            &output.path,
            "docs_config.output_formats.path",
            &mut diagnostics,
        );
        if !DOCS_RENDER_FORMATS.contains(&output.format.as_str()) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::UnsupportedArtifactClass,
                REQUIREMENT_DOCS_CONFIG,
                "docs_config.output_formats.format",
                "documentation output format is unsupported",
            ));
        }
        if !output_formats.insert(output.format.clone()) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::DuplicateArtifactIdentity,
                REQUIREMENT_DOCS_CONFIG,
                "docs_config.output_formats.format",
                "documentation output formats must be unique",
            ));
        }
    }
    if config.link_policy != DOCS_LINK_POLICY {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_DOCS_CONFIG,
            "docs_config.link_policy",
            "documentation link policy is unsupported",
        ));
    }
    if config.run_profiles.is_empty()
        || config
            .run_profiles
            .iter()
            .any(|profile| profile != DOCS_CHECK_PROFILE)
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_DOCS_CONFIG,
            "docs_config.run_profiles",
            "documentation run profile is unsupported",
        ));
    }
    if config.update_policy != DOCS_UPDATE_POLICY {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_DOCS_CONFIG,
            "docs_config.update_policy",
            "documentation update policy is unsupported",
        ));
    }
    for profile in &config.wasm_runner_profiles {
        validate_required_text(
            &profile.profile,
            "docs_config.wasm_runner_profiles.profile",
            &mut diagnostics,
        );
        validate_required_text(
            &profile.engine_identity_policy,
            "docs_config.wasm_runner_profiles.engine_identity_policy",
            &mut diagnostics,
        );
        validate_limit(
            profile.fuel_limit,
            "docs_config.wasm_runner_profiles.fuel_limit",
            &mut diagnostics,
        );
        validate_limit(
            profile.memory_limit_bytes,
            "docs_config.wasm_runner_profiles.memory_limit_bytes",
            &mut diagnostics,
        );
        validate_limit(
            profile.output_limit_bytes,
            "docs_config.wasm_runner_profiles.output_limit_bytes",
            &mut diagnostics,
        );
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

#[must_use]
pub fn evaluate_docs_action(
    config: &DocsConfigProjection,
    mut input: DocsActionEvidenceInput,
) -> DocsActionOutcome {
    let mut diagnostics = std::mem::take(&mut input.diagnostics);
    diagnostics.extend(
        validate_docs_config(config)
            .into_iter()
            .map(|diagnostic| diagnostic.message),
    );
    validate_docs_request(config, &input, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();

    let (passed, state) = docs_completion_state(&input, diagnostics.is_empty());
    DocsActionOutcome {
        passed,
        state,
        action: input.action,
        completed_work_items: input.completed_work_items,
        documentation_ir_identity: input.documentation_ir_identity,
        output_identity: input.output_identity,
        executed: input.executed,
        mutated: input.mutated,
        diagnostics,
    }
}

fn validate_docs_request(
    config: &DocsConfigProjection,
    input: &DocsActionEvidenceInput,
    diagnostics: &mut Vec<String>,
) {
    if input
        .profile
        .as_ref()
        .is_some_and(|profile| !config.run_profiles.contains(profile))
    {
        diagnostics.push("requested documentation profile is unsupported".to_string());
    }
    match input.action {
        DocsActionKind::Collect | DocsActionKind::Check => {
            if input.format.is_some() || input.target.is_some() {
                diagnostics.push(
                    "collect and check do not admit render formats or execution targets"
                        .to_string(),
                );
            }
            require_documentation_ir(input, diagnostics);
        }
        DocsActionKind::Render => {
            let format = input.format.as_deref().unwrap_or(DEFAULT_TARGET_FORMAT);
            if !config
                .output_formats
                .iter()
                .any(|output| output.format == format)
            {
                diagnostics
                    .push("requested documentation render format is unsupported".to_string());
            }
            if input.target.is_some() {
                diagnostics
                    .push("documentation render does not admit an execution target".to_string());
            }
            require_documentation_ir(input, diagnostics);
            if input.output_identity.is_none() {
                diagnostics.push("documentation render produced no output identity".to_string());
            }
        }
        DocsActionKind::RunCells => {
            if input.format.is_some() {
                diagnostics.push(
                    "documentation cell execution does not admit a render format".to_string(),
                );
            }
            if input.target.as_deref() != Some("wasm") {
                diagnostics.push("documentation cell execution target is unsupported".to_string());
            }
            if !input.executed {
                diagnostics.push(
                    "documentation cell execution was requested but no bounded runner executed"
                        .to_string(),
                );
            }
        }
        DocsActionKind::Update => {
            require_documentation_ir(input, diagnostics);
            if input.target.is_some() {
                diagnostics
                    .push("documentation update does not admit an execution target".to_string());
            }
            if input.format.as_ref().is_some_and(|format| {
                !config
                    .output_formats
                    .iter()
                    .any(|output| &output.format == format)
            }) {
                diagnostics
                    .push("requested documentation update format is unsupported".to_string());
            }
            if !input.dry_run && !input.mutated {
                diagnostics.push(
                    "documentation update was requested but no receipt-producing mutation occurred"
                        .to_string(),
                );
            }
        }
    }
}

fn require_documentation_ir(input: &DocsActionEvidenceInput, diagnostics: &mut Vec<String>) {
    if input.completed_work_items == 0 {
        diagnostics.push("documentation action completed no required work".to_string());
    }
    if input.documentation_ir_identity.is_none() {
        diagnostics.push("documentation action produced no checked IR identity".to_string());
    }
}

fn docs_completion_state(
    input: &DocsActionEvidenceInput,
    diagnostics_empty: bool,
) -> (bool, DocsCompletionState) {
    if !diagnostics_empty {
        if matches!(input.action, DocsActionKind::RunCells) && !input.executed {
            return (false, DocsCompletionState::Unsupported);
        }
        if matches!(input.action, DocsActionKind::Update) && !input.mutated {
            return (false, DocsCompletionState::Unsupported);
        }
        return (false, DocsCompletionState::Failed);
    }
    if matches!(input.action, DocsActionKind::Update) && input.dry_run {
        return (true, DocsCompletionState::Planned);
    }
    (true, DocsCompletionState::Completed)
}

pub fn validate_documentation_render_output(
    generated_bytes: &[u8],
    declared_bytes: &[u8],
) -> Result<DocumentationOutputIdentity, Diagnostic> {
    if generated_bytes != declared_bytes {
        return Err(Diagnostic::new(
            DiagnosticCode::GoldenReceiptMismatch,
            REQUIREMENT_DOCS_CONFIG,
            "documentation_output.bytes",
            "generated documentation output differs from the checked output projection",
        ));
    }
    Ok(DocumentationOutputIdentity::compute(generated_bytes))
}

#[must_use]
pub fn docs_config_identity(config_bytes: &[u8]) -> DocumentationFactsIdentity {
    DocumentationFactsIdentity::compute(config_bytes)
}

fn language_documentation_facts_identity(
    input: &LanguageDocumentationFactsInput,
) -> Result<DocumentationFactsIdentity, ReceiptError> {
    let payload = LanguageDocumentationFactsArtifact {
        schema_version: input.schema_version,
        language: input.language.clone(),
        source_identity: input.source_identity.clone(),
        collector_identity: input.collector_identity.clone(),
        collector_profile_identity: input.collector_profile_identity.clone(),
        items: sorted_items(&input.items),
        diagnostics: sorted_strings(&input.diagnostics),
        checked_laws: sorted_strings(&input.checked_laws),
        non_claims: sorted_strings(&input.non_claims),
        facts_identity: compute_identity(
            IdentityDomain::DocumentationFacts,
            b"language facts identity placeholder",
        ),
    };
    Ok(DocumentationFactsIdentity::compute(&canonical_json_bytes(
        &payload,
    )?))
}

fn canonicalization_diagnostic(field: &str, artifact: &str, error: &ReceiptError) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::MalformedProjectionOutput,
        REQUIREMENT_LANGUAGE_VALIDATION,
        field,
        format!("{artifact} canonicalization failed: {error}"),
    )
}

fn validate_facts_items(
    input: &LanguageDocumentationFactsInput,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if input.items.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_LANGUAGE_VALIDATION,
            "items",
            "language documentation facts require at least one item",
        ));
    }
    let mut ids = BTreeSet::new();
    for item in &input.items {
        validate_required_text(&item.item_id, "items.item_id", diagnostics);
        validate_required_text(&item.symbol_path, "items.symbol_path", diagnostics);
        validate_required_text(&item.title, "items.title", diagnostics);
        if item
            .signature
            .as_ref()
            .is_none_or(|value| value.trim().is_empty())
            && item
                .contract_summary
                .as_ref()
                .is_none_or(|value| value.trim().is_empty())
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_LANGUAGE_VALIDATION,
                "items.signature",
                "language documentation item requires a signature or contract summary",
            ));
        }
        if !ids.insert(item.item_id.clone()) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::DuplicateArtifactIdentity,
                REQUIREMENT_ANCHORS,
                "items.item_id",
                "language documentation item IDs must be unique",
            ));
        }
        validate_span(item, input.source_len, diagnostics);
        for dependency in &item.dependency_identities {
            if dependency.algorithm != "blake3" {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::IdentityDomainMismatch,
                    REQUIREMENT_LANGUAGE_VALIDATION,
                    "items.dependency_identities",
                    "documentation dependency identities must use BLAKE3",
                ));
            }
        }
        for cell in &item.runnable_cells {
            validate_runnable_cell(cell, &input.language, diagnostics);
        }
    }
}

fn validate_runnable_cell(
    cell: &RunnableCellReference,
    source_language: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    validate_required_text(&cell.cell_id, "items.runnable_cells.cell_id", diagnostics);
    validate_language(&cell.language, "items.runnable_cells.language", diagnostics);
    if cell.language != source_language {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_LANGUAGE_VALIDATION,
            "items.runnable_cells.language",
            "runnable cell language must match the source facts language for this collector output",
        ));
    }
    validate_identity_domain(
        &cell.source_identity,
        IdentityDomain::Source,
        "items.runnable_cells.source_identity",
        diagnostics,
    );
    if !cell.required_capabilities.is_empty() && cell.evaluator_identity.is_none() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedEvaluatorIdentity,
            REQUIREMENT_LANGUAGE_VALIDATION,
            "items.runnable_cells.evaluator_identity",
            "capability-backed runnable cells require evaluator identity",
        ));
    }
    if let Some(identity) = &cell.evaluator_identity {
        validate_identity_domain(
            identity,
            IdentityDomain::Evaluator,
            "items.runnable_cells.evaluator_identity",
            diagnostics,
        );
    }
    if let Some(target) = &cell.wasm_target {
        validate_identity_domain(
            &target.module_identity,
            IdentityDomain::TargetArtifact,
            "items.runnable_cells.wasm_target.module_identity",
            diagnostics,
        );
        validate_identity_domain(
            &target.runner_profile_identity,
            IdentityDomain::Adapter,
            "items.runnable_cells.wasm_target.runner_profile_identity",
            diagnostics,
        );
    }
    validate_required_non_claims(
        &cell.non_claims,
        REQUIRED_FACT_NON_CLAIMS,
        "items.runnable_cells.non_claims",
        diagnostics,
    );
}

fn validate_span(
    item: &LanguageDocumentationItem,
    source_len: u64,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if item.source_span.end_byte <= item.source_span.start_byte
        || item.source_span.end_byte > source_len
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::SourceSpanFabrication,
            REQUIREMENT_SOURCE_BINDING,
            "items.source_span",
            "documentation source span must be a non-empty byte range inside the source envelope",
        ));
    }
}

fn validate_language(language: &str, field: &str, diagnostics: &mut Vec<Diagnostic>) {
    if !matches!(language, LANGUAGE_RUST | LANGUAGE_NICKEL | LANGUAGE_STEEL) {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedArtifactClass,
                REQUIREMENT_LANGUAGE_FACTS,
                field,
                "documentation language is not admitted for autogenerated facts",
            )
            .with_detail("language", language.to_string()),
        );
    }
}

fn validate_required_text(value: &str, field: &str, diagnostics: &mut Vec<Diagnostic>) {
    if value.trim().is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_LANGUAGE_VALIDATION,
            field,
            "documentation generation input requires a non-empty text field",
        ));
    }
}

fn validate_required_non_claims(
    non_claims: &[String],
    required: &[&str],
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for required_claim in required {
        if !non_claims
            .iter()
            .any(|non_claim| non_claim == required_claim)
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingNonClaim,
                    REQUIREMENT_NON_CLAIMS,
                    field,
                    "documentation generation evidence is missing a required non-claim boundary",
                )
                .with_detail("missing", *required_claim),
            );
        }
    }
}

fn validate_semantic_claims(
    semantic_claims: &[String],
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let joined = semantic_claims.join(" ").to_ascii_lowercase();
    for forbidden in FORBIDDEN_DOCS_CLAIMS {
        if joined.contains(forbidden) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_NON_CLAIMS,
                    field,
                    "documentation generation evidence contains a forbidden overclaim",
                )
                .with_detail("claim", *forbidden),
            );
        }
    }
}

fn validate_identity_domain(
    identity: &WireIdentity,
    expected: IdentityDomain,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if identity.validate_domain(expected.clone()).is_err() {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_LANGUAGE_VALIDATION,
                field,
                "documentation generation identity uses the wrong domain or algorithm",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

fn validate_non_empty_strings(values: &[String], field: &str, diagnostics: &mut Vec<Diagnostic>) {
    if values.iter().any(|value| value.trim().is_empty()) {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_WASM_SCHEMA,
            field,
            "WASM runner profile values must not contain empty entries",
        ));
    }
}

fn validate_limit(value: u64, field: &str, diagnostics: &mut Vec<Diagnostic>) {
    if value == 0 {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_WASM_SCHEMA,
            field,
            "WASM runner profiles require explicit non-zero resource limits",
        ));
    }
}

fn validate_observed_subset(
    declared: &[String],
    observed: &[String],
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let declared = declared.iter().cloned().collect::<BTreeSet<_>>();
    for value in observed {
        if !declared.contains(value) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredCapability,
                    REQUIREMENT_WASM_EFFECTS,
                    field,
                    "WASM execution observed an undeclared import or capability",
                )
                .with_detail("observed", value.clone()),
            );
        }
    }
}

fn documentation_pages(input: &DocsAssemblyInput) -> Vec<DocumentationPageInput> {
    let mut pages = BTreeMap::<String, DocumentationPageInput>::new();
    for facts in &input.facts {
        for item in &facts.items {
            let page_id = input
                .page_grouping
                .get(&item.item_id)
                .cloned()
                .unwrap_or_else(|| format!("{DEFAULT_PAGE_ID}-{}", facts.language));
            let page = pages
                .entry(page_id.clone())
                .or_insert_with(|| DocumentationPageInput {
                    page_id: page_id.clone(),
                    title: DEFAULT_PAGE_TITLE.to_string(),
                    source_envelope_identity: Some(facts.source_identity.clone()),
                    sections: Vec::new(),
                    cells: Vec::new(),
                });
            page.sections.push(DocumentationSectionInput {
                section_id: item.item_id.clone(),
                anchor: stable_anchor(&facts.language, &item.item_id),
                title: item.title.clone(),
                prose_blocks: item.doc_blocks.clone(),
                code_blocks: item.code_blocks.clone(),
                runnable_cell_ids: item
                    .runnable_cells
                    .iter()
                    .map(|cell| cell.cell_id.clone())
                    .collect(),
                links: item.links.clone(),
            });
            page.cells.extend(
                item.runnable_cells
                    .iter()
                    .map(|cell| DocumentationCellDescriptor {
                        cell_id: cell.cell_id.clone(),
                        language: cell.language.clone(),
                        source_identity: Some(cell.source_identity.clone()),
                        required_capabilities: cell.required_capabilities.clone(),
                        expected_result: cell.expected_result.clone(),
                        evaluator_identity: cell.evaluator_identity.clone(),
                        non_claims: cell.non_claims.clone(),
                    }),
            );
        }
    }
    pages
        .into_values()
        .map(|mut page| {
            page.sections
                .sort_by(|left, right| left.section_id.cmp(&right.section_id));
            page.cells
                .sort_by(|left, right| left.cell_id.cmp(&right.cell_id));
            page
        })
        .collect()
}

fn stable_anchor(language: &str, item_id: &str) -> String {
    format!("{language}-{item_id}")
        .chars()
        .map(|character| match character {
            'a'..='z' | '0'..='9' | '-' => character,
            'A'..='Z' => character.to_ascii_lowercase(),
            _ => '-',
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-documentation-ir-v2")]
struct CanonicalDocumentationIr {
    pages: Vec<DocumentationPageInput>,
    target_formats: Vec<String>,
    non_claims: Vec<String>,
}

fn documentation_ir_canonical_bytes(
    pages: &[DocumentationPageInput],
    target_formats: &[String],
    non_claims: &[String],
) -> Result<Vec<u8>, ReceiptError> {
    let body = CanonicalDocumentationIr {
        pages: pages.to_vec(),
        target_formats: sorted_or_default(target_formats),
        non_claims: sorted_strings(non_claims),
    };
    encode_typed_preserves(
        &body,
        PreservesSchema {
            schema_version: DOCUMENTATION_IR_PRESERVES_SCHEMA_VERSION,
            root_label: DOCUMENTATION_IR_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ReceiptError::Serialize(error.to_string()))
}

fn docs_collector_identity(
    facts: &[LanguageDocumentationFactsArtifact],
) -> Result<AdapterIdentity, ReceiptError> {
    let collector_identities = facts
        .iter()
        .map(|fact| fact.collector_identity.clone())
        .collect::<BTreeSet<_>>();
    Ok(AdapterIdentity::compute(&canonical_json_bytes(
        &collector_identities,
    )?))
}

fn sorted_items(items: &[LanguageDocumentationItem]) -> Vec<LanguageDocumentationItem> {
    let mut items = items.to_vec();
    items.sort_by(|left, right| left.item_id.cmp(&right.item_id));
    items
}

fn sorted_strings(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

fn sorted_or_default(values: &[String]) -> Vec<String> {
    if values.is_empty() {
        vec![DEFAULT_TARGET_FORMAT.to_string()]
    } else {
        sorted_strings(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE_LENGTH: u64 = 64;
    const SPAN_END: u64 = 16;
    const CELL_ID: &str = "cell.rust.add";
    const WASM_FUEL_LIMIT: u64 = 100;
    const WASM_MEMORY_LIMIT_BYTES: u64 = 65_536;
    const WASM_OUTPUT_LIMIT_BYTES: u64 = 1_024;
    const OVERSIZED_OUTPUT_BYTES: usize = 2_048;

    fn source_identity() -> WireIdentity {
        compute_identity(IdentityDomain::Source, b"source")
    }

    fn collector_identity() -> WireIdentity {
        compute_identity(IdentityDomain::Adapter, b"collector")
    }

    fn fact_non_claims() -> Vec<String> {
        REQUIRED_FACT_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect()
    }

    fn wasm_non_claims() -> Vec<String> {
        REQUIRED_WASM_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect()
    }

    fn valid_facts_input() -> LanguageDocumentationFactsInput {
        LanguageDocumentationFactsInput {
            schema_version: LANGUAGE_DOCUMENTATION_FACTS_SCHEMA_VERSION,
            language: LANGUAGE_RUST.to_string(),
            source_identity: source_identity(),
            source_len: SOURCE_LENGTH,
            collector_identity: collector_identity(),
            collector_profile_identity: compute_identity(
                IdentityDomain::Adapter,
                b"collector-profile",
            ),
            items: vec![LanguageDocumentationItem {
                item_id: "add".to_string(),
                symbol_path: "crate::add".to_string(),
                title: "add".to_string(),
                signature: Some("pub fn add(left: u32, right: u32) -> u32".to_string()),
                contract_summary: None,
                doc_blocks: vec!["Adds two values.".to_string()],
                code_blocks: vec!["assert_eq!(add(1, 1), 2);".to_string()],
                source_span: SourceByteSpan {
                    start_byte: 0,
                    end_byte: SPAN_END,
                },
                links: Vec::new(),
                runnable_cells: vec![RunnableCellReference {
                    cell_id: CELL_ID.to_string(),
                    language: LANGUAGE_RUST.to_string(),
                    source_identity: source_identity(),
                    required_capabilities: Vec::new(),
                    expected_result: "success".to_string(),
                    evaluator_identity: None,
                    wasm_target: None,
                    non_claims: fact_non_claims(),
                }],
                dependency_identities: Vec::new(),
            }],
            diagnostics: Vec::new(),
            checked_laws: vec![CHECKED_LAW_SOURCE_BOUND.to_string()],
            non_claims: fact_non_claims(),
            semantic_claims: Vec::new(),
            declared_facts_identity: None,
        }
    }

    fn valid_wasm_profile() -> WasmRunnerProfileInput {
        WasmRunnerProfileInput {
            profile_id: "wasm-docs".to_string(),
            engine_identity: compute_identity(IdentityDomain::Evaluator, b"wasmtime"),
            declared_imports: vec!["wasi:cli/stdout".to_string()],
            declared_capabilities: vec!["stdout".to_string()],
            fuel_limit: WASM_FUEL_LIMIT,
            memory_limit_bytes: WASM_MEMORY_LIMIT_BYTES,
            output_limit_bytes: WASM_OUTPUT_LIMIT_BYTES,
            non_claims: wasm_non_claims(),
        }
    }

    #[test]
    fn language_facts_validate_and_assemble_into_documentation_ir() {
        let facts_result = language_documentation_facts_artifact(&valid_facts_input());
        assert!(
            facts_result.is_ok(),
            "valid documentation facts must materialize"
        );
        let Ok(facts) = facts_result else {
            return;
        };
        let ir_result = assemble_documentation_ir(&DocsAssemblyInput {
            facts: vec![facts.clone()],
            target_formats: vec![DEFAULT_TARGET_FORMAT.to_string()],
            page_grouping: BTreeMap::new(),
            non_claims: fact_non_claims(),
        });
        assert!(ir_result.is_ok(), "valid documentation IR must assemble");
        let Ok(ir) = ir_result else {
            return;
        };

        assert_eq!(
            facts.facts_identity.domain,
            IdentityDomain::DocumentationFacts
        );
        assert_eq!(ir.pages[0].sections[0].anchor, "rust-add");
        assert_eq!(ir.pages[0].cells[0].cell_id, CELL_ID);
        assert_eq!(ir.declared_identity.domain, IdentityDomain::DocumentationIr);
    }

    #[test]
    fn language_facts_reject_unsupported_inputs_and_overclaims() {
        let mut input = valid_facts_input();
        input.language = "python".to_string();
        input.source_identity = compute_identity(IdentityDomain::Receipt, b"wrong source");
        input.items[0].source_span.end_byte = SOURCE_LENGTH + 1;
        input.items[0].signature = None;
        input.items[0].runnable_cells[0].required_capabilities = vec!["filesystem".to_string()];
        input.non_claims.clear();
        input.semantic_claims = vec!["rendered output is canonical".to_string()];

        let diagnostics = validate_language_documentation_facts(&input);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "language")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "source_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "items.source_span")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "items.signature")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "items.runnable_cells.evaluator_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::SemanticOverclaim)
        );
    }

    #[test]
    fn documentation_ir_assembly_rejects_duplicates_and_unknown_links() {
        let facts_result = language_documentation_facts_artifact(&valid_facts_input());
        assert!(
            facts_result.is_ok(),
            "invalid assembly base facts must materialize"
        );
        let Ok(mut facts) = facts_result else {
            return;
        };
        facts.items[0].links = vec!["missing".to_string()];
        facts.items.push(facts.items[0].clone());

        let assembly_result = assemble_documentation_ir(&DocsAssemblyInput {
            facts: vec![facts],
            target_formats: Vec::new(),
            page_grouping: BTreeMap::new(),
            non_claims: fact_non_claims(),
        });
        assert!(
            assembly_result.is_err(),
            "duplicate and unknown links must fail assembly"
        );
        let Err(diagnostics) = assembly_result else {
            return;
        };

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::DuplicateArtifactIdentity)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "facts.items.links")
        );
    }

    #[test]
    fn wasm_runner_profile_and_execution_validate_bounded_execution() {
        let profile = valid_wasm_profile();
        let output = b"ok".to_vec();
        let record = WasmExecutionRecordInput {
            cell_id: CELL_ID.to_string(),
            source_identity: source_identity(),
            module_identity: compute_identity(IdentityDomain::TargetArtifact, b"module"),
            runner_profile_identity: compute_identity(IdentityDomain::Adapter, b"runner"),
            engine_identity: profile.engine_identity.clone(),
            declared_imports: profile.declared_imports.clone(),
            observed_imports: profile.declared_imports.clone(),
            declared_capabilities: profile.declared_capabilities.clone(),
            observed_effects: profile.declared_capabilities.clone(),
            fuel_used: profile.fuel_limit,
            memory_used_bytes: profile.memory_limit_bytes,
            output_bytes: output.clone(),
            expected_output_identity: Some(compute_identity(IdentityDomain::Projection, &output)),
            diagnostics: Vec::new(),
            non_claims: wasm_non_claims(),
            semantic_claims: Vec::new(),
        };

        assert!(validate_wasm_execution_record(&profile, &record).is_empty());
    }

    fn valid_docs_config() -> DocsConfigProjection {
        DocsConfigProjection {
            schema_version: DOCS_CONFIG_SCHEMA_VERSION,
            source_roots: vec![DocsSourceRootConfig {
                path: "crates/kamacite-core/src".to_string(),
                language: LANGUAGE_RUST.to_string(),
                collector_profile: "rust-bounded-docs-v1".to_string(),
            }],
            output_formats: vec![DocsOutputConfig {
                format: DEFAULT_TARGET_FORMAT.to_string(),
                path: "target/generated-docs".to_string(),
            }],
            page_grouping: Vec::new(),
            link_policy: DOCS_LINK_POLICY.to_string(),
            run_profiles: vec![DOCS_CHECK_PROFILE.to_string()],
            wasm_runner_profiles: Vec::new(),
            update_policy: DOCS_UPDATE_POLICY.to_string(),
        }
    }

    fn valid_docs_action(action: DocsActionKind) -> DocsActionEvidenceInput {
        DocsActionEvidenceInput {
            action,
            format: None,
            profile: Some(DOCS_CHECK_PROFILE.to_string()),
            target: None,
            dry_run: false,
            completed_work_items: 1,
            documentation_ir_identity: Some(compute_identity(
                IdentityDomain::DocumentationIr,
                b"checked docs IR",
            )),
            output_identity: None,
            executed: false,
            mutated: false,
            diagnostics: Vec::new(),
        }
    }

    #[test]
    fn docs_config_and_completed_actions_are_accepted() {
        let config = valid_docs_config();
        assert!(validate_docs_config(&config).is_empty());

        let collect = evaluate_docs_action(&config, valid_docs_action(DocsActionKind::Collect));
        assert!(collect.passed);
        assert_eq!(collect.state, DocsCompletionState::Completed);

        let mut render_input = valid_docs_action(DocsActionKind::Render);
        render_input.format = Some(DEFAULT_TARGET_FORMAT.to_string());
        render_input.output_identity = Some(DocumentationOutputIdentity::compute(b"rendered docs"));
        let render = evaluate_docs_action(&config, render_input);
        assert!(render.passed);
        assert_eq!(render.state, DocsCompletionState::Completed);
    }

    #[test]
    fn docs_render_comparison_rejects_stale_output() {
        let generated = b"checked rendered output";
        let accepted = validate_documentation_render_output(generated, generated);
        assert!(accepted.is_ok());
        assert_eq!(
            accepted.map(|identity| identity.domain()),
            Ok(IdentityDomain::DocumentationOutput)
        );

        let stale = validate_documentation_render_output(generated, b"stale rendered output");
        assert!(stale.is_err());
        assert_eq!(
            stale.map_err(|diagnostic| diagnostic.code),
            Err(DiagnosticCode::GoldenReceiptMismatch)
        );
    }

    #[test]
    fn docs_actions_reject_invalid_config_no_ops_and_unsupported_values() {
        let mut config = valid_docs_config();
        config.schema_version = DOCS_CONFIG_SCHEMA_VERSION + 1;
        config.source_roots[0].language = "python".to_string();
        assert!(!validate_docs_config(&config).is_empty());

        let config = valid_docs_config();
        let mut render_input = valid_docs_action(DocsActionKind::Render);
        render_input.format = Some("pdf".to_string());
        let render = evaluate_docs_action(&config, render_input);
        assert!(!render.passed);
        assert_eq!(render.state, DocsCompletionState::Failed);

        let run_cells = evaluate_docs_action(&config, valid_docs_action(DocsActionKind::RunCells));
        assert!(!run_cells.passed);
        assert_eq!(run_cells.state, DocsCompletionState::Unsupported);

        let mut failed_cell_input = valid_docs_action(DocsActionKind::RunCells);
        failed_cell_input.executed = true;
        failed_cell_input.target = Some("wasm".to_string());
        failed_cell_input.diagnostics = vec!["documentation cell failed".to_string()];
        let failed_cell = evaluate_docs_action(&config, failed_cell_input);
        assert!(!failed_cell.passed);
        assert_eq!(failed_cell.state, DocsCompletionState::Failed);

        let update = evaluate_docs_action(&config, valid_docs_action(DocsActionKind::Update));
        assert!(!update.passed);
        assert_eq!(update.state, DocsCompletionState::Unsupported);

        let mut dry_run_input = valid_docs_action(DocsActionKind::Update);
        dry_run_input.dry_run = true;
        let dry_run = evaluate_docs_action(&config, dry_run_input);
        assert!(dry_run.passed);
        assert_eq!(dry_run.state, DocsCompletionState::Planned);
    }

    #[test]
    fn wasm_execution_rejects_undeclared_effects_limits_and_overclaims() {
        let profile = valid_wasm_profile();
        let mut record = WasmExecutionRecordInput {
            cell_id: CELL_ID.to_string(),
            source_identity: compute_identity(IdentityDomain::Receipt, b"wrong source"),
            module_identity: compute_identity(IdentityDomain::Receipt, b"wrong module"),
            runner_profile_identity: compute_identity(IdentityDomain::Source, b"wrong runner"),
            engine_identity: compute_identity(IdentityDomain::Evaluator, b"other engine"),
            declared_imports: profile.declared_imports.clone(),
            observed_imports: vec!["wasi:filesystem".to_string()],
            declared_capabilities: profile.declared_capabilities.clone(),
            observed_effects: vec!["filesystem".to_string()],
            fuel_used: profile.fuel_limit + 1,
            memory_used_bytes: profile.memory_limit_bytes + 1,
            output_bytes: vec![0; OVERSIZED_OUTPUT_BYTES],
            expected_output_identity: Some(compute_identity(IdentityDomain::Projection, b"other")),
            diagnostics: Vec::new(),
            non_claims: Vec::new(),
            semantic_claims: vec!["wasm is canonical Kamacite IR".to_string()],
        };
        record.declared_imports.clear();

        let diagnostics = validate_wasm_execution_record(&profile, &record);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "source_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "module_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "runner_profile_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "engine_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "observed_imports")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "fuel_used")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "output_bytes")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "expected_output_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::SemanticOverclaim)
        );
    }
}
