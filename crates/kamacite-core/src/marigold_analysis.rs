use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{Identity, IdentityDomain, compute_identity};
use crate::receipt::{ReceiptError, canonical_json_bytes};
use serde::{Deserialize, Serialize};

pub const MARIGOLD_ANALYSIS_SCHEMA_VERSION: &str = "kamacite.marigold-analysis-receipt.v1";
pub const MARIGOLD_ANALYSIS_PROFILE: &str = "kamacite-marigold-operator-shape-v1";
pub const MARIGOLD_ANALYSIS_EVIDENCE_ROLE: &str = "advisory";
pub const MARIGOLD_ANALYZER_TOOL: &str = "marigold-grammar";
pub const MARIGOLD_ANALYZER_VERSION: &str = "0.1.17";
pub const MARIGOLD_ANALYSIS_CANONICAL_CODEC: &str =
    "kamacite-deterministic-json-marigold-analysis-receipt-v1";
pub const MARIGOLD_ANALYSIS_JSON_PROJECTION: &str = "kamacite.marigold-analysis.v1.compat-json";
pub const MARIGOLD_MAX_SOURCE_BYTES: usize = 1024 * 1024;
pub const MARIGOLD_REQUIRED_CHECKED_LAWS: &[&str] = &[
    "advisory-role-only",
    "analysis-projection-hash-bound",
    "analyzer-version-bound",
    "source-bytes-bound",
];
pub const MARIGOLD_REQUIRED_NON_CLAIMS: &[&str] = &[
    "does not inspect arbitrary function bodies",
    "not canonical Kamacite IR",
    "not deterministic execution or replay proof",
    "not host authority",
    "not runtime cost or resource-safety proof",
];

const REQUIREMENT_PROFILE: &str = "r[kamacite.marigold_analysis.profile]";
const REQUIREMENT_IDENTITY: &str = "r[kamacite.marigold_analysis.identity]";
const REQUIREMENT_CORE: &str = "r[kamacite.marigold_analysis.functional_core]";
const REQUIREMENT_RECEIPT: &str = "r[kamacite.marigold_analysis.receipt]";
const REQUIREMENT_NON_CLAIMS: &str = "r[kamacite.marigold_analysis.nonclaims]";
const REQUIREMENT_FIXTURES: &str = "r[kamacite.marigold_analysis.fixtures]";
const MAX_STREAM_COUNT: usize = 128;
const MAX_DESCRIPTION_BYTES: usize = 512;
const MAX_ANALYSIS_TEXT_BYTES: usize = 256;
const MIN_STREAM_COUNT: usize = 1;
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarigoldAnalyzerDescriptor {
    pub profile: String,
    pub tool: String,
    pub tool_version: String,
    pub evidence_role: String,
}

impl Default for MarigoldAnalyzerDescriptor {
    fn default() -> Self {
        Self {
            profile: MARIGOLD_ANALYSIS_PROFILE.to_string(),
            tool: MARIGOLD_ANALYZER_TOOL.to_string(),
            tool_version: MARIGOLD_ANALYZER_VERSION.to_string(),
            evidence_role: MARIGOLD_ANALYSIS_EVIDENCE_ROLE.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarigoldStreamAnalysis {
    pub description: String,
    pub cardinality: String,
    pub time_class: String,
    pub exact_time: String,
    pub space_class: String,
    pub exact_space: String,
    pub collects_input: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarigoldProgramAnalysis {
    pub streams: Vec<MarigoldStreamAnalysis>,
    pub program_time: String,
    pub program_exact_time: String,
    pub program_space: String,
    pub program_exact_space: String,
    pub program_cardinality: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarigoldAnalysisInput {
    pub analyzer: MarigoldAnalyzerDescriptor,
    pub source_bytes: Vec<u8>,
    pub source_identity: Identity,
    pub evaluator_identity: Identity,
    pub analysis: MarigoldProgramAnalysis,
    pub declared_analysis_projection_identity: Option<Identity>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
    pub semantic_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarigoldAnalysisReceiptBody {
    pub schema_version: String,
    pub canonical_codec: String,
    pub compatibility_projection: String,
    pub analyzer: MarigoldAnalyzerDescriptor,
    pub source_identity: Identity,
    pub evaluator_identity: Identity,
    pub analysis_projection_identity: Identity,
    pub analysis: MarigoldProgramAnalysis,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarigoldAnalysisReceiptEnvelope {
    pub body: MarigoldAnalysisReceiptBody,
    pub receipt_identity: Identity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MarigoldAnalysisReceiptResult {
    pub passed: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub receipt: Option<MarigoldAnalysisReceiptEnvelope>,
}

pub fn marigold_evaluator_identity(
    analyzer: &MarigoldAnalyzerDescriptor,
) -> Result<Identity, ReceiptError> {
    Ok(compute_identity(
        IdentityDomain::Evaluator,
        &canonical_json_bytes(analyzer)?,
    ))
}

pub fn marigold_analysis_projection_identity(
    analysis: &MarigoldProgramAnalysis,
) -> Result<Identity, ReceiptError> {
    Ok(compute_identity(
        IdentityDomain::Projection,
        &canonical_json_bytes(analysis)?,
    ))
}

#[must_use]
pub fn validate_marigold_analysis(input: &MarigoldAnalysisInput) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_analyzer(input, &mut diagnostics);
    validate_source_identity(input, &mut diagnostics);
    validate_evaluator_identity(input, &mut diagnostics);
    validate_analysis_report(&input.analysis, &mut diagnostics);
    validate_analysis_projection_identity(input, &mut diagnostics);
    validate_required_values(
        &input.checked_laws,
        RequiredValueCheck {
            required: MARIGOLD_REQUIRED_CHECKED_LAWS,
            code: DiagnosticCode::MissingCheckedLaw,
            requirement: REQUIREMENT_RECEIPT,
            field: "checked_laws",
            message: "Marigold analysis receipt is missing a required checked law",
        },
        &mut diagnostics,
    );
    validate_required_values(
        &input.non_claims,
        RequiredValueCheck {
            required: MARIGOLD_REQUIRED_NON_CLAIMS,
            code: DiagnosticCode::MissingNonClaim,
            requirement: REQUIREMENT_NON_CLAIMS,
            field: "non_claims",
            message: "Marigold analysis receipt is missing a required non-claim",
        },
        &mut diagnostics,
    );
    validate_semantic_claims(&input.semantic_claims, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

pub fn build_marigold_analysis_receipt(
    input: &MarigoldAnalysisInput,
) -> Result<MarigoldAnalysisReceiptResult, ReceiptError> {
    let diagnostics = validate_marigold_analysis(input);
    if !diagnostics.is_empty() {
        return Ok(MarigoldAnalysisReceiptResult {
            passed: false,
            diagnostics,
            receipt: None,
        });
    }
    let analysis_projection_identity = marigold_analysis_projection_identity(&input.analysis)?;
    let body = MarigoldAnalysisReceiptBody {
        schema_version: MARIGOLD_ANALYSIS_SCHEMA_VERSION.to_string(),
        canonical_codec: MARIGOLD_ANALYSIS_CANONICAL_CODEC.to_string(),
        compatibility_projection: MARIGOLD_ANALYSIS_JSON_PROJECTION.to_string(),
        analyzer: input.analyzer.clone(),
        source_identity: input.source_identity.clone(),
        evaluator_identity: input.evaluator_identity.clone(),
        analysis_projection_identity,
        analysis: input.analysis.clone(),
        checked_laws: sorted_unique(&input.checked_laws),
        non_claims: sorted_unique(&input.non_claims),
    };
    let receipt_identity = compute_identity(IdentityDomain::Receipt, &canonical_json_bytes(&body)?);
    Ok(MarigoldAnalysisReceiptResult {
        passed: true,
        diagnostics,
        receipt: Some(MarigoldAnalysisReceiptEnvelope {
            body,
            receipt_identity,
        }),
    })
}

fn validate_analyzer(input: &MarigoldAnalysisInput, diagnostics: &mut Vec<Diagnostic>) {
    if input.analyzer.profile != MARIGOLD_ANALYSIS_PROFILE {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedEvaluatorIdentity,
                REQUIREMENT_PROFILE,
                "analyzer.profile",
                "Marigold analysis profile is unsupported",
            )
            .with_detail("actual", input.analyzer.profile.clone())
            .with_detail("expected", MARIGOLD_ANALYSIS_PROFILE),
        );
    }
    if input.analyzer.tool != MARIGOLD_ANALYZER_TOOL {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedEvaluatorIdentity,
                REQUIREMENT_PROFILE,
                "analyzer.tool",
                "Marigold analyzer tool is unsupported",
            )
            .with_detail("actual", input.analyzer.tool.clone())
            .with_detail("expected", MARIGOLD_ANALYZER_TOOL),
        );
    }
    if input.analyzer.tool_version != MARIGOLD_ANALYZER_VERSION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedEvaluatorIdentity,
                REQUIREMENT_PROFILE,
                "analyzer.tool_version",
                "Marigold analyzer version is unsupported",
            )
            .with_detail("actual", input.analyzer.tool_version.clone())
            .with_detail("expected", MARIGOLD_ANALYZER_VERSION),
        );
    }
    if input.analyzer.evidence_role != MARIGOLD_ANALYSIS_EVIDENCE_ROLE {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::SemanticOverclaim,
                REQUIREMENT_NON_CLAIMS,
                "analyzer.evidence_role",
                "Marigold analysis must remain advisory",
            )
            .with_detail("actual", input.analyzer.evidence_role.clone())
            .with_detail("expected", MARIGOLD_ANALYSIS_EVIDENCE_ROLE),
        );
    }
}

fn validate_source_identity(input: &MarigoldAnalysisInput, diagnostics: &mut Vec<Diagnostic>) {
    validate_identity_domain(
        &input.source_identity,
        IdentityDomain::Source,
        "source_identity",
        diagnostics,
    );
    if input.source_bytes.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_IDENTITY,
            "source_bytes",
            "Marigold source bytes must not be empty",
        ));
    }
    if input.source_bytes.len() > MARIGOLD_MAX_SOURCE_BYTES {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_PROFILE,
                "source_bytes",
                "Marigold source exceeds the profile byte bound",
            )
            .with_detail("actual", input.source_bytes.len().to_string())
            .with_detail("maximum", MARIGOLD_MAX_SOURCE_BYTES.to_string()),
        );
    }
    if std::str::from_utf8(&input.source_bytes).is_err() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_PROFILE,
            "source_bytes",
            "Marigold source must be UTF-8",
        ));
    }
    let expected = compute_identity(IdentityDomain::Source, &input.source_bytes);
    if input.source_identity.domain == IdentityDomain::Source && input.source_identity != expected {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_IDENTITY,
                "source_identity",
                "Marigold source identity is stale",
            )
            .with_detail("declared", input.source_identity.hex.as_str())
            .with_detail("computed", expected.hex.as_str()),
        );
    }
}

fn validate_evaluator_identity(input: &MarigoldAnalysisInput, diagnostics: &mut Vec<Diagnostic>) {
    validate_identity_domain(
        &input.evaluator_identity,
        IdentityDomain::Evaluator,
        "evaluator_identity",
        diagnostics,
    );
    let Ok(expected) = marigold_evaluator_identity(&input.analyzer) else {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_CORE,
            "analyzer",
            "Marigold analyzer descriptor could not be serialized",
        ));
        return;
    };
    if input.evaluator_identity.domain == IdentityDomain::Evaluator
        && input.evaluator_identity != expected
    {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_IDENTITY,
                "evaluator_identity",
                "Marigold evaluator identity is stale",
            )
            .with_detail("declared", input.evaluator_identity.hex.as_str())
            .with_detail("computed", expected.hex.as_str()),
        );
    }
}

fn validate_analysis_projection_identity(
    input: &MarigoldAnalysisInput,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(declared) = &input.declared_analysis_projection_identity else {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_IDENTITY,
            "declared_analysis_projection_identity",
            "Marigold analysis requires a declared projection identity",
        ));
        return;
    };
    validate_identity_domain(
        declared,
        IdentityDomain::Projection,
        "declared_analysis_projection_identity",
        diagnostics,
    );
    let Ok(computed) = marigold_analysis_projection_identity(&input.analysis) else {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_CORE,
            "analysis",
            "Marigold analysis projection could not be serialized",
        ));
        return;
    };
    if declared.domain == IdentityDomain::Projection && declared != &computed {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_IDENTITY,
                "declared_analysis_projection_identity",
                "Marigold analysis projection identity is stale",
            )
            .with_detail("declared", declared.hex.as_str())
            .with_detail("computed", computed.hex.as_str()),
        );
    }
}

fn validate_analysis_report(report: &MarigoldProgramAnalysis, diagnostics: &mut Vec<Diagnostic>) {
    if report.streams.len() < MIN_STREAM_COUNT {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FIXTURES,
            "analysis.streams",
            "Marigold analysis requires at least one output stream",
        ));
    }
    if report.streams.len() > MAX_STREAM_COUNT {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_PROFILE,
                "analysis.streams",
                "Marigold analysis exceeds the stream-count bound",
            )
            .with_detail("actual", report.streams.len().to_string())
            .with_detail("maximum", MAX_STREAM_COUNT.to_string()),
        );
    }
    for stream in &report.streams {
        validate_bounded_text(
            &stream.description,
            MAX_DESCRIPTION_BYTES,
            "analysis.streams.description",
            diagnostics,
        );
        validate_cardinality(
            &stream.cardinality,
            "analysis.streams.cardinality",
            diagnostics,
        );
        validate_complexity_class(
            &stream.time_class,
            "analysis.streams.time_class",
            diagnostics,
        );
        validate_complexity_expression(
            &stream.exact_time,
            "analysis.streams.exact_time",
            diagnostics,
        );
        validate_complexity_class(
            &stream.space_class,
            "analysis.streams.space_class",
            diagnostics,
        );
        validate_complexity_expression(
            &stream.exact_space,
            "analysis.streams.exact_space",
            diagnostics,
        );
    }
    validate_complexity_class(&report.program_time, "analysis.program_time", diagnostics);
    validate_complexity_expression(
        &report.program_exact_time,
        "analysis.program_exact_time",
        diagnostics,
    );
    validate_complexity_class(&report.program_space, "analysis.program_space", diagnostics);
    validate_complexity_expression(
        &report.program_exact_space,
        "analysis.program_exact_space",
        diagnostics,
    );
    validate_cardinality(
        &report.program_cardinality,
        "analysis.program_cardinality",
        diagnostics,
    );
}

fn validate_bounded_text(
    value: &str,
    maximum_bytes: usize,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if value.trim().is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_PROFILE,
            field,
            "Marigold analysis field must not be empty",
        ));
    }
    if value.len() > maximum_bytes || value.chars().any(char::is_control) {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PROFILE,
                field,
                "Marigold analysis field exceeds its bound or contains control characters",
            )
            .with_detail("actual_bytes", value.len().to_string())
            .with_detail("maximum_bytes", maximum_bytes.to_string()),
        );
    }
}

fn validate_cardinality(value: &str, field: &str, diagnostics: &mut Vec<Diagnostic>) {
    validate_bounded_text(value, MAX_ANALYSIS_TEXT_BYTES, field, diagnostics);
    if !value.chars().all(is_cardinality_character) {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_PROFILE,
            field,
            "Marigold cardinality contains unsupported characters",
        ));
    }
}

fn is_cardinality_character(character: char) -> bool {
    character.is_ascii_digit()
        || matches!(
            character,
            '?' | '≤' | 'P' | 'C' | 'm' | 'i' | 'n' | '(' | ')' | ',' | ' ' | '+' | '^'
        )
}

fn validate_complexity_class(value: &str, field: &str, diagnostics: &mut Vec<Diagnostic>) {
    validate_bounded_text(value, MAX_ANALYSIS_TEXT_BYTES, field, diagnostics);
    if !is_supported_complexity_class(value) {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_PROFILE,
            field,
            "Marigold complexity class is unsupported",
        ));
    }
}

fn is_supported_complexity_class(value: &str) -> bool {
    matches!(
        value,
        "O(1)" | "O(log(n))" | "O(n)" | "O(n*log(n))" | "O(n!)" | "O(?)"
    ) || numeric_complexity_parameter(value, "O(n*log(", "))")
        || numeric_complexity_parameter(value, "O(n^", ")")
        || numeric_complexity_parameter(value, "O(C(n,", "))")
        || numeric_complexity_parameter(value, "O(n!/(n-", ")!)")
}

fn numeric_complexity_parameter(value: &str, prefix: &str, suffix: &str) -> bool {
    let Some(parameter) = value
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(suffix))
    else {
        return false;
    };
    !parameter.is_empty() && parameter.bytes().all(|byte| byte.is_ascii_digit())
}

fn validate_complexity_expression(value: &str, field: &str, diagnostics: &mut Vec<Diagnostic>) {
    validate_bounded_text(value, MAX_ANALYSIS_TEXT_BYTES, field, diagnostics);
    let has_outer_shape = value.starts_with("O(") && value.ends_with(')');
    let has_supported_characters = value.chars().all(is_complexity_expression_character);
    if !has_outer_shape || !has_supported_characters {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_PROFILE,
            field,
            "Marigold exact complexity expression is malformed",
        ));
    }
}

fn is_complexity_expression_character(character: char) -> bool {
    character.is_ascii_digit()
        || matches!(
            character,
            'O' | 'C'
                | 'n'
                | 'l'
                | 'o'
                | 'g'
                | '?'
                | '('
                | ')'
                | ','
                | ' '
                | '+'
                | '^'
                | '*'
                | '/'
                | '!'
                | '-'
        )
}

struct RequiredValueCheck<'a> {
    required: &'a [&'a str],
    code: DiagnosticCode,
    requirement: &'a str,
    field: &'a str,
    message: &'a str,
}

fn validate_required_values(
    actual: &[String],
    check: RequiredValueCheck<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for required_value in check.required {
        if !actual.iter().any(|value| value == required_value) {
            diagnostics.push(
                Diagnostic::new(
                    check.code.clone(),
                    check.requirement,
                    check.field,
                    check.message,
                )
                .with_detail("missing", *required_value),
            );
        }
    }
}

fn validate_semantic_claims(claims: &[String], diagnostics: &mut Vec<Diagnostic>) {
    for claim in claims {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::SemanticOverclaim,
                REQUIREMENT_NON_CLAIMS,
                "semantic_claims",
                "Marigold advisory analysis cannot carry semantic claims",
            )
            .with_detail("claim", claim),
        );
    }
}

fn validate_identity_domain(
    identity: &Identity,
    expected: IdentityDomain,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if identity.validate_domain(expected.clone()).is_err() {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_IDENTITY,
                field,
                "Marigold analysis identity uses the wrong domain or algorithm",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

fn sorted_unique(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DESCRIPTION: &str = "input.map(...).filter(...).return";
    const TEST_CARDINALITY: &str = "≤100";
    const TEST_CONSTANT_COMPLEXITY: &str = "O(1)";
    const TEST_LINEAR_COMPLEXITY: &str = "O(n)";
    const TEST_EXACT_COMPLEXITY: &str = "O(2n)";
    const UNSUPPORTED_VERSION: &str = "0.1.18";
    const BAD_COMPLEXITY: &str = "linear";
    const INVALID_UTF8_BYTE: u8 = 0xff;
    const TEST_SOURCE: &[u8] = b"range(0, 100).map(double).filter(is_odd).return";

    fn required(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn valid_analysis() -> MarigoldProgramAnalysis {
        MarigoldProgramAnalysis {
            streams: vec![MarigoldStreamAnalysis {
                description: TEST_DESCRIPTION.to_string(),
                cardinality: TEST_CARDINALITY.to_string(),
                time_class: TEST_CONSTANT_COMPLEXITY.to_string(),
                exact_time: TEST_EXACT_COMPLEXITY.to_string(),
                space_class: TEST_CONSTANT_COMPLEXITY.to_string(),
                exact_space: TEST_CONSTANT_COMPLEXITY.to_string(),
                collects_input: false,
            }],
            program_time: TEST_CONSTANT_COMPLEXITY.to_string(),
            program_exact_time: TEST_EXACT_COMPLEXITY.to_string(),
            program_space: TEST_CONSTANT_COMPLEXITY.to_string(),
            program_exact_space: TEST_CONSTANT_COMPLEXITY.to_string(),
            program_cardinality: TEST_CARDINALITY.to_string(),
        }
    }

    fn valid_input() -> MarigoldAnalysisInput {
        let analyzer = MarigoldAnalyzerDescriptor::default();
        let analysis = valid_analysis();
        let evaluator_identity = marigold_evaluator_identity(&analyzer);
        assert!(evaluator_identity.is_ok());
        let Ok(evaluator_identity) = evaluator_identity else {
            return MarigoldAnalysisInput {
                analyzer,
                source_bytes: TEST_SOURCE.to_vec(),
                source_identity: compute_identity(IdentityDomain::Source, TEST_SOURCE),
                evaluator_identity: compute_identity(IdentityDomain::Evaluator, b"unavailable"),
                analysis,
                declared_analysis_projection_identity: None,
                checked_laws: Vec::new(),
                non_claims: Vec::new(),
                semantic_claims: Vec::new(),
            };
        };
        let analysis_identity = marigold_analysis_projection_identity(&analysis);
        assert!(analysis_identity.is_ok());
        MarigoldAnalysisInput {
            analyzer,
            source_bytes: TEST_SOURCE.to_vec(),
            source_identity: compute_identity(IdentityDomain::Source, TEST_SOURCE),
            evaluator_identity,
            declared_analysis_projection_identity: analysis_identity.ok(),
            analysis,
            checked_laws: required(MARIGOLD_REQUIRED_CHECKED_LAWS),
            non_claims: required(MARIGOLD_REQUIRED_NON_CLAIMS),
            semantic_claims: Vec::new(),
        }
    }

    #[test]
    fn valid_advisory_analysis_binds_source_evaluator_projection_and_receipt() {
        let result = build_marigold_analysis_receipt(&valid_input());
        assert!(result.is_ok());
        let Ok(result) = result else {
            return;
        };
        assert!(result.passed);
        assert!(result.diagnostics.is_empty());
        assert!(
            result.receipt.is_some(),
            "passing analysis must emit a receipt"
        );
        let Some(receipt) = result.receipt else {
            return;
        };
        assert_eq!(
            receipt.body.analyzer.evidence_role,
            MARIGOLD_ANALYSIS_EVIDENCE_ROLE
        );
        assert_eq!(receipt.body.source_identity.domain, IdentityDomain::Source);
        assert_eq!(
            receipt.body.evaluator_identity.domain,
            IdentityDomain::Evaluator
        );
        assert_eq!(
            receipt.body.analysis_projection_identity.domain,
            IdentityDomain::Projection
        );
        assert_eq!(receipt.receipt_identity.domain, IdentityDomain::Receipt);
    }

    #[test]
    fn malformed_stale_and_overclaimed_analysis_fails_closed() {
        let mut input = valid_input();
        input.analyzer.tool_version = UNSUPPORTED_VERSION.to_string();
        input.source_identity = compute_identity(IdentityDomain::Receipt, TEST_SOURCE);
        input.evaluator_identity = compute_identity(IdentityDomain::Evaluator, b"stale");
        input.declared_analysis_projection_identity = Some(compute_identity(
            IdentityDomain::Projection,
            b"stale projection",
        ));
        assert!(!input.analysis.streams.is_empty());
        let Some(stream) = input.analysis.streams.first_mut() else {
            return;
        };
        stream.time_class = BAD_COMPLEXITY.to_string();
        input.analysis.program_space = TEST_LINEAR_COMPLEXITY.to_string();
        input.checked_laws.clear();
        input.non_claims.clear();
        input.semantic_claims = vec!["proves runtime cost and semantic equivalence".to_string()];

        let diagnostics = validate_marigold_analysis(&input);

        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.code == DiagnosticCode::UnsupportedEvaluatorIdentity
            })
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::IdentityDomainMismatch })
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::MalformedProjectionOutput })
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingCheckedLaw)
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
        let result = build_marigold_analysis_receipt(&input);
        assert!(matches!(
            result,
            Ok(MarigoldAnalysisReceiptResult {
                passed: false,
                receipt: None,
                ..
            })
        ));
    }

    #[test]
    fn stale_and_non_utf8_source_evidence_fails_closed() {
        let mut input = valid_input();
        input.source_bytes = vec![INVALID_UTF8_BYTE];

        let diagnostics = validate_marigold_analysis(&input);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MalformedProjectionOutput)
        );
    }

    #[test]
    fn finite_report_bounds_reject_oversized_analyzer_output() {
        let mut input = valid_input();
        assert!(!input.analysis.streams.is_empty());
        let Some(stream) = input.analysis.streams.first().cloned() else {
            return;
        };
        input.analysis.streams = vec![stream; MAX_STREAM_COUNT + 1];

        let diagnostics = validate_marigold_analysis(&input);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::IncompleteMetadata)
        );
    }

    #[test]
    fn law_and_non_claim_order_or_duplicates_do_not_change_receipt_identity() {
        let first = valid_input();
        let mut second = valid_input();
        second.checked_laws.reverse();
        assert!(!second.checked_laws.is_empty());
        let Some(duplicate_law) = second.checked_laws.first().cloned() else {
            return;
        };
        second.checked_laws.push(duplicate_law);
        second.non_claims.reverse();
        assert!(!second.non_claims.is_empty());
        let Some(duplicate_non_claim) = second.non_claims.first().cloned() else {
            return;
        };
        second.non_claims.push(duplicate_non_claim);

        let first_result = build_marigold_analysis_receipt(&first);
        let second_result = build_marigold_analysis_receipt(&second);
        assert!(first_result.is_ok());
        assert!(second_result.is_ok());
        let (Ok(first_result), Ok(second_result)) = (first_result, second_result) else {
            return;
        };
        assert!(first_result.receipt.is_some());
        assert!(second_result.receipt.is_some());
        let (Some(first_receipt), Some(second_receipt)) =
            (first_result.receipt, second_result.receipt)
        else {
            return;
        };

        assert_eq!(
            first_receipt.receipt_identity,
            second_receipt.receipt_identity
        );
    }

    #[test]
    fn collection_observation_changes_projection_and_receipt_identity() {
        let first = valid_input();
        let mut second = valid_input();
        assert!(!second.analysis.streams.is_empty());
        let Some(stream) = second.analysis.streams.first_mut() else {
            return;
        };
        stream.collects_input = true;
        let second_projection = marigold_analysis_projection_identity(&second.analysis);
        assert!(second_projection.is_ok());
        second.declared_analysis_projection_identity = second_projection.ok();

        let first_result = build_marigold_analysis_receipt(&first);
        let second_result = build_marigold_analysis_receipt(&second);
        assert!(first_result.is_ok());
        assert!(second_result.is_ok());
        let (Ok(first_result), Ok(second_result)) = (first_result, second_result) else {
            return;
        };
        assert!(
            first_result.receipt.is_some(),
            "valid analysis must emit a receipt"
        );
        assert!(
            second_result.receipt.is_some(),
            "valid analysis must emit a receipt"
        );
        let (Some(first_receipt), Some(second_receipt)) =
            (first_result.receipt, second_result.receipt)
        else {
            return;
        };
        assert_ne!(
            first_receipt.body.analysis_projection_identity,
            second_receipt.body.analysis_projection_identity
        );
        assert_ne!(
            first_receipt.receipt_identity,
            second_receipt.receipt_identity
        );
    }
}
