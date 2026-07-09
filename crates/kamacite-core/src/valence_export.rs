use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{Identity, IdentityDomain, compute_identity};
use crate::receipt::{RECEIPT_SCHEMA_VERSION, ReceiptEnvelope, canonical_json_bytes};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const VALENCE_EXPORT_SCHEMA_VERSION: &str = "kamacite.valence_receipt_export.v1";
pub const VALENCE_EXPORT_PROFILE_VERSION: &str = "kamacite.valence_receipt_export.v1";
pub const VALENCE_EXPORT_CANONICAL_CODEC: &str =
    "kamacite-deterministic-json-valence-export-projection-v1";
pub const VALENCE_EXPORT_DEPENDENCY_PROFILE: &str = "artifacts-first-no-valence-runtime";
const VALENCE_EXPORT_EVIDENCE_ROLE: &str = "derived-evidence-projection";
const VALENCE_EXPORT_CLAIM_BOUNDARY: &str =
    "Valence export preserves Kamacite receipt facts without broadening claims";
const SOURCE_SPANS_ABSENT_REASON: &str = "source span metadata absent in source receipt";
const SOURCE_RECEIPT_ARTIFACT_ROLE: &str = "source-receipt";
const CONFORMANCE_ARTIFACT_ROLE: &str = "conformance-entry";
const INPUT_ARTIFACT_ROLE: &str = "source-receipt-input";
const OUTPUT_ARTIFACT_ROLE: &str = "source-receipt-output";
const EXPORT_ARTIFACT_ROLE: &str = "valence-export-projection";
const REQUIRED_POSITIVE_FIXTURE_COVERAGE: &str = "positive";
const REQUIRED_NEGATIVE_FIXTURE_COVERAGE: &str = "negative";
const FORBIDDEN_CLAIM_SEMANTIC_EQUIVALENCE: &str = "semantic equivalence";
const FORBIDDEN_CLAIM_SEMANTIC_EQUIVALENCE_HYPHENATED: &str = "semantic-equivalence";
const FORBIDDEN_CLAIM_RUNTIME_CORRECTNESS: &str = "runtime correctness";
const FORBIDDEN_CLAIM_COMPILER_CORRECTNESS: &str = "compiler correctness";
const FORBIDDEN_CLAIM_EVALUATOR_SOUNDNESS: &str = "evaluator soundness";
const FORBIDDEN_CLAIM_CANONICAL_IR_AUTHORITY: &str = "canonical ir authority";
const FORBIDDEN_CLAIM_ARBITRARY_PROGRAM: &str = "arbitrary program";
const REQUIREMENT_PROFILE: &str = "r[kamacite.valence_receipt_export.profile]";
const REQUIREMENT_BOUNDED_PROJECTION: &str =
    "r[kamacite.valence_receipt_export.profile.bounded_projection]";
const REQUIREMENT_SOURCE_SPANS: &str = "r[kamacite.valence_receipt_export.profile.source_spans]";
const REQUIREMENT_EXPORT_IDENTITY: &str =
    "r[kamacite.valence_receipt_export.profile.export_identity]";
const REQUIREMENT_CONFORMANCE: &str = "r[kamacite.valence_receipt_export.conformance]";
const REQUIREMENT_ENTRY_REQUIRED: &str =
    "r[kamacite.valence_receipt_export.conformance.entry_required]";
const REQUIREMENT_IDENTITY_DOMAINS: &str =
    "r[kamacite.valence_receipt_export.conformance.identity_domains]";
const REQUIREMENT_MISSING_NON_CLAIMS: &str =
    "r[kamacite.valence_receipt_export.conformance.missing_non_claims]";
const REQUIREMENT_SEMANTIC_OVERCLAIM: &str =
    "r[kamacite.valence_receipt_export.conformance.semantic_overclaim]";
const REQUIREMENT_MALFORMED_INPUT: &str =
    "r[kamacite.valence_receipt_export.conformance.malformed_input]";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceSpan {
    pub artifact_key: String,
    pub start_byte: u64,
    pub end_byte: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum SourceSpanSet {
    Present { spans: Vec<SourceSpan> },
    Absent { reason: String },
}

impl Default for SourceSpanSet {
    fn default() -> Self {
        Self::Absent {
            reason: SOURCE_SPANS_ABSENT_REASON.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValenceExportConformance {
    pub has_entry: bool,
    pub profile_version: String,
    pub source_schema: String,
    pub output_schema: String,
    pub required_input_identity_domains: Vec<IdentityDomain>,
    pub output_identity_domain: IdentityDomain,
    pub checked_laws: Vec<String>,
    pub dependency_profile: String,
    pub fixture_coverage: Vec<String>,
    pub non_claims: Vec<String>,
    pub conformance_identity: Identity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValenceReceiptExportInput {
    pub source_receipt: ReceiptEnvelope,
    pub conformance: ValenceExportConformance,
    pub source_spans: SourceSpanSet,
    pub semantic_claims: Vec<String>,
    pub declared_export_identity: Option<Identity>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ValenceArtifactIdentity {
    pub role: String,
    pub key: String,
    pub identity: Identity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValenceReceiptExportBody {
    pub schema_version: String,
    pub profile_version: String,
    pub canonical_codec: String,
    pub evidence_role: String,
    pub source_receipt_identity: Identity,
    pub source_fixture_id: String,
    pub source_artifact_class: String,
    pub source_verdict: String,
    pub conformance_identity: Identity,
    pub artifact_identities: Vec<ValenceArtifactIdentity>,
    pub checked_laws: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
    pub source_spans: SourceSpanSet,
    pub non_claims: Vec<String>,
    pub coupling_boundary: String,
    pub claim_boundary: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValenceReceiptExportEnvelope {
    pub body: ValenceReceiptExportBody,
    pub export_identity: Identity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ValenceReceiptExportResult {
    pub passed: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub export: Option<ValenceReceiptExportEnvelope>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ArtifactKey(String);

pub fn export_valence_receipt(input: &ValenceReceiptExportInput) -> ValenceReceiptExportResult {
    let mut diagnostics = Vec::new();
    validate_source_receipt(&input.source_receipt, &mut diagnostics);
    validate_conformance(&input.conformance, &mut diagnostics);
    validate_semantic_claims(&input.semantic_claims, &mut diagnostics);
    let artifact_keys = artifact_identity_keys(&input.source_receipt);
    validate_source_spans(&input.source_spans, &artifact_keys, &mut diagnostics);

    let export = build_export(
        &input.source_receipt,
        &input.conformance,
        &input.source_spans,
    );
    if let Some(declared) = &input.declared_export_identity {
        validate_declared_export_identity(declared, export.as_ref(), &mut diagnostics);
    }

    diagnostics.sort();
    diagnostics.dedup();
    let passed = diagnostics.is_empty();
    ValenceReceiptExportResult {
        passed,
        diagnostics,
        export: if passed { export } else { None },
    }
}

fn validate_source_receipt(receipt: &ReceiptEnvelope, diagnostics: &mut Vec<Diagnostic>) {
    if receipt
        .receipt_identity
        .validate_domain(IdentityDomain::Receipt)
        .is_err()
    {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_IDENTITY_DOMAINS,
                "source_receipt.receipt_identity",
                "Valence export source receipt identity must use the receipt domain",
            )
            .with_detail("actual", receipt.receipt_identity.domain.to_string())
            .with_detail("expected", IdentityDomain::Receipt.to_string()),
        );
    }
    if receipt.body.schema_version != RECEIPT_SCHEMA_VERSION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_MALFORMED_INPUT,
                "source_receipt.body.schema_version",
                "Valence export only supports the accepted Kamacite receipt schema",
            )
            .with_detail("actual", receipt.body.schema_version.clone())
            .with_detail("expected", RECEIPT_SCHEMA_VERSION.to_string()),
        );
    }
    validate_source_receipt_hash(receipt, diagnostics);
    if receipt.body.non_claims.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_MISSING_NON_CLAIMS,
            "source_receipt.body.non_claims",
            "Valence export must preserve non-claims from the source Kamacite receipt",
        ));
    }
    if receipt.body.checked_laws.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingCheckedLaw,
            REQUIREMENT_MALFORMED_INPUT,
            "source_receipt.body.checked_laws",
            "Valence export requires checked laws from the source receipt before wrapping evidence",
        ));
    }
    validate_artifact_identity_keys(receipt, diagnostics);
}

fn validate_source_receipt_hash(receipt: &ReceiptEnvelope, diagnostics: &mut Vec<Diagnostic>) {
    let Ok(body_bytes) = canonical_json_bytes(&receipt.body) else {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_MALFORMED_INPUT,
            "source_receipt.body",
            "source receipt body could not be serialized deterministically",
        ));
        return;
    };
    let computed = compute_identity(IdentityDomain::Receipt, &body_bytes);
    if receipt.receipt_identity.domain == IdentityDomain::Receipt
        && receipt.receipt_identity.hex != computed.hex
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_MALFORMED_INPUT,
            "source_receipt.receipt_identity.hex",
            "source receipt identity does not match the canonical source receipt body",
        ));
    }
}

fn validate_artifact_identity_keys(receipt: &ReceiptEnvelope, diagnostics: &mut Vec<Diagnostic>) {
    if receipt.body.input_identities.is_empty() && receipt.body.output_identities.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_BOUNDED_PROJECTION,
            "source_receipt.body.input_identities",
            "Valence export requires at least one artifact identity to preserve",
        ));
    }
    let mut keys = BTreeSet::new();
    for key in receipt.body.input_identities.keys() {
        if !keys.insert(ArtifactKey(key.clone())) {
            diagnostics.push(duplicate_artifact_key_diagnostic(key));
        }
    }
    for key in receipt.body.output_identities.keys() {
        if !keys.insert(ArtifactKey(key.clone())) {
            diagnostics.push(duplicate_artifact_key_diagnostic(key));
        }
    }
}

fn duplicate_artifact_key_diagnostic(key: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::DuplicateArtifactIdentity,
        REQUIREMENT_MALFORMED_INPUT,
        "source_receipt.body.artifact_identities",
        "Valence export rejects duplicate artifact identity keys before emitting trusted evidence",
    )
    .with_detail("key", key.to_string())
}

fn validate_conformance(conformance: &ValenceExportConformance, diagnostics: &mut Vec<Diagnostic>) {
    if !conformance.has_entry {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingConformanceEntry,
            REQUIREMENT_ENTRY_REQUIRED,
            "conformance.has_entry",
            "Valence export requires a matching conformance entry before trust",
        ));
    }
    if conformance.profile_version != VALENCE_EXPORT_PROFILE_VERSION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_MALFORMED_INPUT,
                "conformance.profile_version",
                "unsupported Valence export profile version rejected",
            )
            .with_detail("actual", conformance.profile_version.clone())
            .with_detail("expected", VALENCE_EXPORT_PROFILE_VERSION.to_string()),
        );
    }
    if conformance.source_schema != RECEIPT_SCHEMA_VERSION {
        diagnostics.push(schema_mismatch_diagnostic(
            "conformance.source_schema",
            &conformance.source_schema,
            RECEIPT_SCHEMA_VERSION,
        ));
    }
    if conformance.output_schema != VALENCE_EXPORT_SCHEMA_VERSION {
        diagnostics.push(schema_mismatch_diagnostic(
            "conformance.output_schema",
            &conformance.output_schema,
            VALENCE_EXPORT_SCHEMA_VERSION,
        ));
    }
    if !conformance
        .required_input_identity_domains
        .contains(&IdentityDomain::Receipt)
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::IdentityDomainMismatch,
            REQUIREMENT_IDENTITY_DOMAINS,
            "conformance.required_input_identity_domains",
            "Valence export conformance must require a source receipt identity domain",
        ));
    }
    if conformance.output_identity_domain != IdentityDomain::Projection {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_IDENTITY_DOMAINS,
                "conformance.output_identity_domain",
                "Valence export conformance output identity must use the projection domain",
            )
            .with_detail("actual", conformance.output_identity_domain.to_string())
            .with_detail("expected", IdentityDomain::Projection.to_string()),
        );
    }
    if conformance
        .conformance_identity
        .validate_domain(IdentityDomain::Adapter)
        .is_err()
    {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_IDENTITY_DOMAINS,
                "conformance.conformance_identity",
                "Valence export conformance entry identity must use the adapter descriptor domain",
            )
            .with_detail(
                "actual",
                conformance.conformance_identity.domain.to_string(),
            )
            .with_detail("expected", IdentityDomain::Adapter.to_string()),
        );
    }
    if conformance.checked_laws.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingCheckedLaw,
            REQUIREMENT_CONFORMANCE,
            "conformance.checked_laws",
            "Valence export conformance entry must name checked laws",
        ));
    }
    if conformance.dependency_profile != VALENCE_EXPORT_DEPENDENCY_PROFILE {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_PROFILE,
                "conformance.dependency_profile",
                "Valence export conformance must remain artifacts-first without live Valence runtime coupling",
            )
            .with_detail("actual", conformance.dependency_profile.clone())
            .with_detail("expected", VALENCE_EXPORT_DEPENDENCY_PROFILE.to_string()),
        );
    }
    validate_fixture_coverage(&conformance.fixture_coverage, diagnostics);
    if conformance.non_claims.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_CONFORMANCE,
            "conformance.non_claims",
            "Valence export conformance entry must carry explicit non-claims",
        ));
    }
}

fn schema_mismatch_diagnostic(field: &str, actual: &str, expected: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::UnsupportedSchemaVersion,
        REQUIREMENT_MALFORMED_INPUT,
        field,
        "Valence export conformance schema declaration is unsupported",
    )
    .with_detail("actual", actual.to_string())
    .with_detail("expected", expected.to_string())
}

fn validate_fixture_coverage(coverage: &[String], diagnostics: &mut Vec<Diagnostic>) {
    if !coverage
        .iter()
        .any(|entry| entry == REQUIRED_POSITIVE_FIXTURE_COVERAGE)
        || !coverage
            .iter()
            .any(|entry| entry == REQUIRED_NEGATIVE_FIXTURE_COVERAGE)
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_CONFORMANCE,
            "conformance.fixture_coverage",
            "Valence export conformance must include positive and negative fixture coverage",
        ));
    }
}

fn validate_semantic_claims(claims: &[String], diagnostics: &mut Vec<Diagnostic>) {
    for claim in claims {
        if forbidden_claim(claim) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_SEMANTIC_OVERCLAIM,
                    "semantic_claims",
                    "Valence export rejected semantic-equivalence or runtime correctness overclaim",
                )
                .with_detail("claim", claim.clone()),
            );
        }
    }
}

fn forbidden_claim(claim: &str) -> bool {
    let normalized = claim.to_ascii_lowercase();
    normalized.contains(FORBIDDEN_CLAIM_SEMANTIC_EQUIVALENCE)
        || normalized.contains(FORBIDDEN_CLAIM_SEMANTIC_EQUIVALENCE_HYPHENATED)
        || normalized.contains(FORBIDDEN_CLAIM_RUNTIME_CORRECTNESS)
        || normalized.contains(FORBIDDEN_CLAIM_COMPILER_CORRECTNESS)
        || normalized.contains(FORBIDDEN_CLAIM_EVALUATOR_SOUNDNESS)
        || normalized.contains(FORBIDDEN_CLAIM_CANONICAL_IR_AUTHORITY)
        || normalized.contains(FORBIDDEN_CLAIM_ARBITRARY_PROGRAM)
}

fn artifact_identity_keys(receipt: &ReceiptEnvelope) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    keys.extend(receipt.body.input_identities.keys().cloned());
    keys.extend(receipt.body.output_identities.keys().cloned());
    keys
}

fn validate_source_spans(
    source_spans: &SourceSpanSet,
    artifact_keys: &BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match source_spans {
        SourceSpanSet::Present { spans } => {
            if spans.is_empty() {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_SOURCE_SPANS,
                    "source_spans.spans",
                    "present source-span evidence must include at least one explicit span",
                ));
            }
            for span in spans {
                validate_one_source_span(span, artifact_keys, diagnostics);
            }
        }
        SourceSpanSet::Absent { reason } => {
            if reason.is_empty() {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_SOURCE_SPANS,
                    "source_spans.reason",
                    "absent source-span evidence must record explicit absence",
                ));
            }
        }
    }
}

fn validate_one_source_span(
    span: &SourceSpan,
    artifact_keys: &BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if span.artifact_key.is_empty() || !artifact_keys.contains(&span.artifact_key) {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::SourceSpanFabrication,
                REQUIREMENT_SOURCE_SPANS,
                "source_spans.spans.artifact_key",
                "Valence export must not fabricate source spans from paths, labels, or diagnostics",
            )
            .with_detail("artifact_key", span.artifact_key.clone()),
        );
    }
    if span.start_byte > span.end_byte {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::SourceSpanFabrication,
            REQUIREMENT_SOURCE_SPANS,
            "source_spans.spans",
            "source span byte range must be explicit and ordered",
        ));
    }
}

fn validate_declared_export_identity(
    declared: &Identity,
    export: Option<&ValenceReceiptExportEnvelope>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if declared
        .validate_domain(IdentityDomain::Projection)
        .is_err()
    {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_EXPORT_IDENTITY,
                "declared_export_identity",
                "Valence export identity must use the projection/export identity domain",
            )
            .with_detail("actual", declared.domain.to_string())
            .with_detail("expected", IdentityDomain::Projection.to_string()),
        );
        return;
    }
    let Some(export) = export else {
        return;
    };
    if declared.hex != export.export_identity.hex {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_EXPORT_IDENTITY,
            "declared_export_identity.hex",
            "declared Valence export identity is stale for the computed projection body",
        ));
    }
}

fn build_export(
    source_receipt: &ReceiptEnvelope,
    conformance: &ValenceExportConformance,
    source_spans: &SourceSpanSet,
) -> Option<ValenceReceiptExportEnvelope> {
    let body = ValenceReceiptExportBody {
        schema_version: VALENCE_EXPORT_SCHEMA_VERSION.to_string(),
        profile_version: VALENCE_EXPORT_PROFILE_VERSION.to_string(),
        canonical_codec: VALENCE_EXPORT_CANONICAL_CODEC.to_string(),
        evidence_role: VALENCE_EXPORT_EVIDENCE_ROLE.to_string(),
        source_receipt_identity: source_receipt.receipt_identity.clone(),
        source_fixture_id: source_receipt.body.fixture_id.clone(),
        source_artifact_class: source_receipt.body.artifact_class.clone(),
        source_verdict: source_receipt.body.verdict.clone(),
        conformance_identity: conformance.conformance_identity.clone(),
        artifact_identities: collect_artifact_identities(source_receipt, conformance),
        checked_laws: sorted_unique(
            source_receipt
                .body
                .checked_laws
                .iter()
                .chain(conformance.checked_laws.iter())
                .cloned()
                .collect(),
        ),
        diagnostics: source_receipt.body.diagnostics.clone(),
        source_spans: source_spans.clone(),
        non_claims: sorted_unique(
            source_receipt
                .body
                .non_claims
                .iter()
                .chain(conformance.non_claims.iter())
                .cloned()
                .collect(),
        ),
        coupling_boundary: VALENCE_EXPORT_DEPENDENCY_PROFILE.to_string(),
        claim_boundary: VALENCE_EXPORT_CLAIM_BOUNDARY.to_string(),
    };
    let body_bytes = match canonical_json_bytes(&body) {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };
    Some(ValenceReceiptExportEnvelope {
        export_identity: compute_identity(IdentityDomain::Projection, &body_bytes),
        body,
    })
}

fn collect_artifact_identities(
    source_receipt: &ReceiptEnvelope,
    conformance: &ValenceExportConformance,
) -> Vec<ValenceArtifactIdentity> {
    let mut identities = Vec::new();
    identities.push(ValenceArtifactIdentity {
        role: SOURCE_RECEIPT_ARTIFACT_ROLE.to_string(),
        key: SOURCE_RECEIPT_ARTIFACT_ROLE.to_string(),
        identity: source_receipt.receipt_identity.clone(),
    });
    identities.push(ValenceArtifactIdentity {
        role: CONFORMANCE_ARTIFACT_ROLE.to_string(),
        key: CONFORMANCE_ARTIFACT_ROLE.to_string(),
        identity: conformance.conformance_identity.clone(),
    });
    for (key, identity) in &source_receipt.body.input_identities {
        identities.push(ValenceArtifactIdentity {
            role: INPUT_ARTIFACT_ROLE.to_string(),
            key: key.clone(),
            identity: identity.clone(),
        });
    }
    for (key, identity) in &source_receipt.body.output_identities {
        identities.push(ValenceArtifactIdentity {
            role: OUTPUT_ARTIFACT_ROLE.to_string(),
            key: key.clone(),
            identity: identity.clone(),
        });
    }
    identities.sort();
    identities
}

fn sorted_unique(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

pub fn export_identity_key() -> String {
    EXPORT_ARTIFACT_ROLE.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receipt::{ReceiptBody, ReceiptEnvelope};
    use std::collections::BTreeMap;

    const FIXTURE_ID: &str = "unit.valence.export.source";
    const ARTIFACT_CLASS: &str = "source-envelope";
    const VERDICT: &str = "accepted";
    const SOURCE_NON_CLAIM: &str = "not Nickel semantic equivalence";
    const CONFORMANCE_NON_CLAIM: &str = "not Valence semantic authority";
    const SOURCE_LAW: &str = "source-envelope-preservation";
    const CONFORMANCE_LAW: &str = "non-claim-preservation";
    const REQUIREMENT_ID: &str = "r[kamacite.valence_receipt_export.profile]";
    const SOURCE_BYTES: &[u8] = b"let value = 1 in value\n";
    const CONFORMANCE_DESCRIPTOR: &[u8] = b"kamacite-valence-export-conformance-v1";
    const SPAN_START_BYTE: u64 = 0;
    const SPAN_END_BYTE: u64 = 3;

    fn valid_source_receipt() -> ReceiptEnvelope {
        let mut body = ReceiptBody::new(
            FIXTURE_ID,
            ARTIFACT_CLASS,
            VERDICT,
            vec![REQUIREMENT_ID.to_string()],
            vec![SOURCE_NON_CLAIM.to_string()],
        );
        body.input_identities.insert(
            "declared-source".to_string(),
            compute_identity(IdentityDomain::Source, SOURCE_BYTES),
        );
        body.checked_laws.push(SOURCE_LAW.to_string());
        match ReceiptEnvelope::finalize(body) {
            Ok(receipt) => receipt,
            Err(error) => panic!("test receipt must finalize: {error}"),
        }
    }

    fn valid_conformance() -> ValenceExportConformance {
        ValenceExportConformance {
            has_entry: true,
            profile_version: VALENCE_EXPORT_PROFILE_VERSION.to_string(),
            source_schema: RECEIPT_SCHEMA_VERSION.to_string(),
            output_schema: VALENCE_EXPORT_SCHEMA_VERSION.to_string(),
            required_input_identity_domains: vec![IdentityDomain::Receipt],
            output_identity_domain: IdentityDomain::Projection,
            checked_laws: vec![CONFORMANCE_LAW.to_string()],
            dependency_profile: VALENCE_EXPORT_DEPENDENCY_PROFILE.to_string(),
            fixture_coverage: vec![
                REQUIRED_POSITIVE_FIXTURE_COVERAGE.to_string(),
                REQUIRED_NEGATIVE_FIXTURE_COVERAGE.to_string(),
            ],
            non_claims: vec![CONFORMANCE_NON_CLAIM.to_string()],
            conformance_identity: compute_identity(IdentityDomain::Adapter, CONFORMANCE_DESCRIPTOR),
        }
    }

    fn valid_input() -> ValenceReceiptExportInput {
        ValenceReceiptExportInput {
            source_receipt: valid_source_receipt(),
            conformance: valid_conformance(),
            source_spans: SourceSpanSet::Absent {
                reason: SOURCE_SPANS_ABSENT_REASON.to_string(),
            },
            semantic_claims: Vec::new(),
            declared_export_identity: None,
        }
    }

    #[test]
    fn exports_bounded_receipt_projection() {
        let input = valid_input();

        let result = export_valence_receipt(&input);

        assert!(result.passed);
        assert!(result.diagnostics.is_empty());
        let Some(export) = result.export else {
            panic!("accepted export must return projection");
        };
        assert_eq!(export.export_identity.domain, IdentityDomain::Projection);
        assert_eq!(
            export.body.source_receipt_identity,
            input.source_receipt.receipt_identity
        );
        assert!(
            export
                .body
                .non_claims
                .iter()
                .any(|non_claim| non_claim == SOURCE_NON_CLAIM)
        );
        assert!(export.body.checked_laws.iter().any(|law| law == SOURCE_LAW));
    }

    #[test]
    fn preserves_present_source_spans_without_fabrication() {
        let mut input = valid_input();
        input.source_spans = SourceSpanSet::Present {
            spans: vec![SourceSpan {
                artifact_key: "declared-source".to_string(),
                start_byte: SPAN_START_BYTE,
                end_byte: SPAN_END_BYTE,
            }],
        };

        let result = export_valence_receipt(&input);

        assert!(result.passed);
        let Some(export) = result.export else {
            panic!("accepted export must preserve spans");
        };
        assert_eq!(export.body.source_spans, input.source_spans);
    }

    #[test]
    fn rejects_missing_source_non_claims() {
        let mut input = valid_input();
        input.source_receipt.body.non_claims = Vec::new();
        input.source_receipt = match ReceiptEnvelope::finalize(input.source_receipt.body) {
            Ok(receipt) => receipt,
            Err(error) => panic!("test receipt must finalize: {error}"),
        };

        let result = export_valence_receipt(&input);

        assert!(!result.passed);
        assert!(result.export.is_none());
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
    }

    #[test]
    fn rejects_identity_domain_substitution() {
        let mut input = valid_input();
        let substituted = compute_identity(IdentityDomain::Source, SOURCE_BYTES);
        input.source_receipt.receipt_identity = substituted;

        let result = export_valence_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::IdentityDomainMismatch })
        );
    }

    #[test]
    fn rejects_semantic_equivalence_overclaim() {
        let mut input = valid_input();
        input
            .semantic_claims
            .push("Valence proves semantic equivalence".to_string());

        let result = export_valence_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::SemanticOverclaim)
        );
    }

    #[test]
    fn rejects_stale_declared_export_identity() {
        let mut input = valid_input();
        input.declared_export_identity =
            Some(compute_identity(IdentityDomain::Projection, SOURCE_BYTES));

        let result = export_valence_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
    }

    #[test]
    fn rejects_duplicate_artifact_identity_key() {
        let mut input = valid_input();
        let mut body = input.source_receipt.body;
        body.output_identities = BTreeMap::new();
        body.output_identities.insert(
            "declared-source".to_string(),
            compute_identity(IdentityDomain::Source, SOURCE_BYTES),
        );
        input.source_receipt = match ReceiptEnvelope::finalize(body) {
            Ok(receipt) => receipt,
            Err(error) => panic!("test receipt must finalize: {error}"),
        };

        let result = export_valence_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::DuplicateArtifactIdentity })
        );
    }
}
