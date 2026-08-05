// r[impl kamacite.ast_grep_adapter_audits.profile]
// r[impl kamacite.ast_grep_adapter_audits.language_boundary]
// r[impl kamacite.ast_grep_adapter_audits.identity]
// r[impl kamacite.ast_grep_adapter_audits.fixtures]
// r[impl kamacite.ast_grep_adapter_audits.codemods]
// r[impl kamacite.ast_grep_adapter_audits.valence_export]
use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{IdentityDomain, WireIdentity, compute_identity};
use crate::receipt::canonical_json_bytes;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const AST_GREP_AUDIT_SCHEMA_VERSION: &str = "kamacite.ast-grep-audit.v1";
pub const AST_GREP_RECORDED_ONLY_ROLE: &str = "recorded-only";
pub const AST_GREP_BOUNDARY_ROLE: &str = "boundary";
pub const AST_GREP_VALENCE_SCHEMA_VERSION: &str = "kamacite.ast-grep-valence.v1";

const REQUIREMENT_PROFILE: &str = "r[kamacite.ast_grep_adapter_audits.profile]";
const REQUIREMENT_LANGUAGE: &str = "r[kamacite.ast_grep_adapter_audits.language_boundary]";
const REQUIREMENT_IDENTITY: &str = "r[kamacite.ast_grep_adapter_audits.identity]";
const REQUIREMENT_FIXTURES: &str = "r[kamacite.ast_grep_adapter_audits.fixtures]";
const REQUIREMENT_CODEMODS: &str = "r[kamacite.ast_grep_adapter_audits.codemods]";
const REQUIREMENT_VALENCE: &str = "r[kamacite.ast_grep_adapter_audits.valence_export]";
const BUILT_IN_PARSER_KIND: &str = "built-in";
const CUSTOM_PARSER_KIND: &str = "custom-tree-sitter";
const AST_GREP_PROFILE_SCHEMA_VERSION: &str = "kamacite.ast-grep-profile.v1";

#[derive(Deserialize, Serialize)]
struct AstGrepProfileProjection {
    schema_version: String,
    profile_id: String,
    bundle_identity: WireIdentity,
    language_profiles: Vec<AstGrepLanguageProfile>,
    rules: Vec<AstGrepProfileRule>,
    verification_role: String,
    non_claims: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct AstGrepProfileRule {
    rule_id: String,
    language: String,
    rule_identity: WireIdentity,
    positive_fixture_identity: WireIdentity,
    negative_fixture_identity: WireIdentity,
    positive_cases: u32,
    negative_cases: u32,
    gate_eligible: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AstGrepLanguageProfile {
    pub language: String,
    pub parser_kind: String,
    pub parser_identity: WireIdentity,
    pub extensions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AstGrepRuleCoverage {
    pub rule_id: String,
    pub language: String,
    pub rule_identity: WireIdentity,
    pub positive_fixture_identity: Option<WireIdentity>,
    pub negative_fixture_identity: Option<WireIdentity>,
    pub positive_cases: u32,
    pub negative_cases: u32,
    pub gate_eligible: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AstGrepSourceEnvelope {
    pub path: String,
    pub bytes: Vec<u8>,
    pub identity: WireIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AstGrepFinding {
    pub rule_id: String,
    pub source_identity: WireIdentity,
    pub start_byte: u64,
    pub end_byte: u64,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AstGrepValenceProjection {
    pub schema_version: String,
    pub profile_identity: WireIdentity,
    pub rule_bundle_identity: WireIdentity,
    pub findings_identity: WireIdentity,
    pub verification_role: String,
    pub findings: Vec<AstGrepFinding>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AstGrepCodemodArtifact {
    pub recipe_bytes: Vec<u8>,
    pub recipe_identity: WireIdentity,
    pub before_source_bytes: Vec<u8>,
    pub before_source_identity: WireIdentity,
    pub after_source_bytes: Vec<u8>,
    pub after_source_identity: WireIdentity,
    pub dry_run: bool,
    pub post_check_commands: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AstGrepAuditInput {
    pub schema_version: String,
    pub profile_bytes: Vec<u8>,
    pub profile_identity: WireIdentity,
    pub rule_bundle_bytes: Vec<u8>,
    pub rule_bundle_identity: WireIdentity,
    pub source_scope_identity: WireIdentity,
    pub findings_identity: WireIdentity,
    pub valence_projection_bytes: Vec<u8>,
    pub valence_projection_identity: WireIdentity,
    pub language_profiles: Vec<AstGrepLanguageProfile>,
    pub rules: Vec<AstGrepRuleCoverage>,
    pub sources: Vec<AstGrepSourceEnvelope>,
    pub findings: Vec<AstGrepFinding>,
    pub verification_role: String,
    pub codemod: Option<AstGrepCodemodArtifact>,
    pub non_claims: Vec<String>,
}

#[derive(Serialize)]
struct ScopeIdentityEntry<'a> {
    path: &'a str,
    identity: &'a WireIdentity,
}

#[must_use]
pub fn validate_ast_grep_audit(input: &AstGrepAuditInput) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_profile(input, &mut diagnostics);
    validate_languages(input, &mut diagnostics);
    validate_rules(input, &mut diagnostics);
    validate_sources_and_findings(input, &mut diagnostics);
    validate_codemod(input.codemod.as_ref(), &mut diagnostics);
    validate_valence_projection(input, &mut diagnostics);
    if !matches!(
        input.verification_role.as_str(),
        AST_GREP_RECORDED_ONLY_ROLE | AST_GREP_BOUNDARY_ROLE
    ) {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedClaim,
            REQUIREMENT_VALENCE,
            "ast_grep.verification_role",
            "ast-grep evidence may use only recorded-only or boundary verification roles",
        ));
    }
    if input.non_claims.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_PROFILE,
            "ast_grep.non_claims",
            "ast-grep audit evidence must retain syntax-only non-claims",
        ));
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

fn validate_profile(input: &AstGrepAuditInput, diagnostics: &mut Vec<Diagnostic>) {
    if input.schema_version != AST_GREP_AUDIT_SCHEMA_VERSION {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedSchemaVersion,
            REQUIREMENT_PROFILE,
            "ast_grep.schema_version",
            "ast-grep audit schema version is unsupported",
        ));
    }
    validate_computed_identity(
        &input.profile_identity,
        IdentityDomain::Adapter,
        &input.profile_bytes,
        "ast_grep.profile_identity",
        diagnostics,
    );
    validate_computed_identity(
        &input.rule_bundle_identity,
        IdentityDomain::Adapter,
        &input.rule_bundle_bytes,
        "ast_grep.rule_bundle_identity",
        diagnostics,
    );
    if input.language_profiles.is_empty() || input.rules.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_PROFILE,
            "ast_grep.profile",
            "ast-grep audit profile must declare languages and rules",
        ));
    }
    let profile = serde_json::from_slice::<AstGrepProfileProjection>(&input.profile_bytes);
    match profile {
        Ok(profile)
            if profile.schema_version == AST_GREP_PROFILE_SCHEMA_VERSION
                && !profile.profile_id.is_empty()
                && profile.bundle_identity == input.rule_bundle_identity
                && profile.language_profiles == input.language_profiles
                && profile.verification_role == input.verification_role
                && profile.non_claims == input.non_claims
                && profile_rules_match(&profile.rules, &input.rules) => {}
        Ok(_) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_PROFILE,
            "ast_grep.profile",
            "ast-grep runtime profile does not match the admitted audit DTO",
        )),
        Err(error) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_PROFILE,
            "ast_grep.profile",
            format!("ast-grep runtime profile is malformed: {error}"),
        )),
    }
}

fn profile_rules_match(
    profile_rules: &[AstGrepProfileRule],
    input_rules: &[AstGrepRuleCoverage],
) -> bool {
    profile_rules.len() == input_rules.len()
        && profile_rules
            .iter()
            .zip(input_rules)
            .all(|(profile, input)| {
                profile.rule_id == input.rule_id
                    && profile.language == input.language
                    && profile.rule_identity == input.rule_identity
                    && input.positive_fixture_identity.as_ref()
                        == Some(&profile.positive_fixture_identity)
                    && input.negative_fixture_identity.as_ref()
                        == Some(&profile.negative_fixture_identity)
                    && profile.positive_cases == input.positive_cases
                    && profile.negative_cases == input.negative_cases
                    && profile.gate_eligible == input.gate_eligible
            })
}

fn validate_languages(input: &AstGrepAuditInput, diagnostics: &mut Vec<Diagnostic>) {
    let mut languages = BTreeSet::new();
    for language in &input.language_profiles {
        if language.language.is_empty()
            || language.parser_kind.is_empty()
            || language.extensions.is_empty()
            || !matches!(
                language.parser_kind.as_str(),
                BUILT_IN_PARSER_KIND | CUSTOM_PARSER_KIND
            )
            || !languages.insert(language.language.as_str())
            || language
                .parser_identity
                .validate_domain(IdentityDomain::Evaluator)
                .is_err()
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_LANGUAGE,
                "ast_grep.language_profiles",
                "each ast-grep language profile must have a unique language, parser identity, and extension scope",
            ));
        }
    }
}

fn validate_rules(input: &AstGrepAuditInput, diagnostics: &mut Vec<Diagnostic>) {
    let languages = input
        .language_profiles
        .iter()
        .map(|profile| profile.language.as_str())
        .collect::<BTreeSet<_>>();
    let source_identities = input
        .sources
        .iter()
        .map(|source| &source.identity)
        .collect::<BTreeSet<_>>();
    let mut rule_ids = BTreeSet::new();
    for rule in &input.rules {
        let fixture_domains_valid = rule
            .positive_fixture_identity
            .iter()
            .chain(rule.negative_fixture_identity.iter())
            .all(|identity| identity.validate_domain(IdentityDomain::Source).is_ok());
        if rule.rule_id.is_empty()
            || !rule_ids.insert(rule.rule_id.as_str())
            || !languages.contains(rule.language.as_str())
            || rule
                .rule_identity
                .validate_domain(IdentityDomain::Source)
                .is_err()
            || !source_identities.contains(&rule.rule_identity)
            || !rule
                .positive_fixture_identity
                .iter()
                .chain(rule.negative_fixture_identity.iter())
                .all(|identity| source_identities.contains(identity))
            || !fixture_domains_valid
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_PROFILE,
                "ast_grep.rules",
                "ast-grep rules must have unique IDs, declared languages, and source-domain identities",
            ));
        }
        if rule.gate_eligible
            && (rule.positive_cases == 0
                || rule.negative_cases == 0
                || rule.positive_fixture_identity.is_none()
                || rule.negative_fixture_identity.is_none())
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_FIXTURES,
                "ast_grep.rules.fixture_coverage",
                "gate-eligible ast-grep rules require positive and negative fixtures",
            ));
        }
    }
}

fn validate_sources_and_findings(input: &AstGrepAuditInput, diagnostics: &mut Vec<Diagnostic>) {
    let mut source_identities = BTreeSet::new();
    let mut scope_entries = Vec::new();
    for source in &input.sources {
        let computed = compute_identity(IdentityDomain::Source, &source.bytes);
        if source.path.is_empty()
            || source.identity != computed
            || !source_identities.insert(source.identity.clone())
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::SourceHashMismatch,
                REQUIREMENT_IDENTITY,
                "ast_grep.sources",
                "ast-grep source-envelope scope must bind unique exact source bytes",
            ));
        }
        scope_entries.push(ScopeIdentityEntry {
            path: &source.path,
            identity: &source.identity,
        });
    }
    match canonical_json_bytes(&scope_entries) {
        Ok(bytes) => validate_computed_identity(
            &input.source_scope_identity,
            IdentityDomain::Adapter,
            &bytes,
            "ast_grep.source_scope_identity",
            diagnostics,
        ),
        Err(error) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_IDENTITY,
            "ast_grep.source_scope_identity",
            format!("ast-grep source scope identity failed: {error}"),
        )),
    }

    let rule_ids = input
        .rules
        .iter()
        .map(|rule| rule.rule_id.as_str())
        .collect::<BTreeSet<_>>();
    for finding in &input.findings {
        if !rule_ids.contains(finding.rule_id.as_str())
            || !source_identities.contains(&finding.source_identity)
            || finding.start_byte >= finding.end_byte
            || finding.message.is_empty()
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::SourceSpanFabrication,
                REQUIREMENT_IDENTITY,
                "ast_grep.findings",
                "ast-grep findings must bind a declared rule, source envelope, and non-empty span",
            ));
        }
    }
    match canonical_json_bytes(&input.findings) {
        Ok(bytes) => validate_computed_identity(
            &input.findings_identity,
            IdentityDomain::Adapter,
            &bytes,
            "ast_grep.findings_identity",
            diagnostics,
        ),
        Err(error) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_IDENTITY,
            "ast_grep.findings_identity",
            format!("ast-grep finding identity failed: {error}"),
        )),
    }
}

fn validate_valence_projection(input: &AstGrepAuditInput, diagnostics: &mut Vec<Diagnostic>) {
    validate_computed_identity(
        &input.valence_projection_identity,
        IdentityDomain::Projection,
        &input.valence_projection_bytes,
        "ast_grep.valence_projection_identity",
        diagnostics,
    );
    let projection =
        serde_json::from_slice::<AstGrepValenceProjection>(&input.valence_projection_bytes);
    match projection {
        Ok(projection)
            if projection.schema_version == AST_GREP_VALENCE_SCHEMA_VERSION
                && projection.profile_identity == input.profile_identity
                && projection.rule_bundle_identity == input.rule_bundle_identity
                && projection.findings_identity == input.findings_identity
                && projection.verification_role == input.verification_role
                && projection.findings == input.findings
                && projection.non_claims == input.non_claims => {}
        Ok(_) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_VALENCE,
            "ast_grep.valence_projection",
            "ast-grep Valence projection does not bind the validated audit evidence",
        )),
        Err(error) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_VALENCE,
            "ast_grep.valence_projection",
            format!("ast-grep Valence projection is malformed: {error}"),
        )),
    }
}

fn validate_codemod(codemod: Option<&AstGrepCodemodArtifact>, diagnostics: &mut Vec<Diagnostic>) {
    let Some(codemod) = codemod else {
        return;
    };
    let identities_valid = codemod.recipe_identity
        == compute_identity(IdentityDomain::Source, &codemod.recipe_bytes)
        && codemod.before_source_identity
            == compute_identity(IdentityDomain::Source, &codemod.before_source_bytes)
        && codemod.after_source_identity
            == compute_identity(IdentityDomain::Source, &codemod.after_source_bytes);
    if !codemod.dry_run
        || codemod.post_check_commands.is_empty()
        || codemod.non_claims.is_empty()
        || !identities_valid
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedClaim,
            REQUIREMENT_CODEMODS,
            "ast_grep.codemod",
            "ast-grep codemods must remain dry-run source artifacts with post-checks and non-claims",
        ));
    }
}

fn validate_computed_identity(
    declared: &WireIdentity,
    domain: IdentityDomain,
    bytes: &[u8],
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let computed = compute_identity(domain.clone(), bytes);
    if declared.domain != domain || declared != &computed {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_IDENTITY,
            field,
            "declared ast-grep identity does not match exact evidence bytes",
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULE_BUNDLE_BYTES: &[u8] = b"rule-bundle";
    const RULE_BYTES: &[u8] = b"rule: ambient-process";
    const SOURCE_BYTES: &[u8] = b"fn main() {}";
    const NEGATIVE_SOURCE_BYTES: &[u8] = b"fn pure() {}";
    const FINDING_END_BYTE: u64 = 2;

    fn valid_input() -> Option<AstGrepAuditInput> {
        let source_identity = compute_identity(IdentityDomain::Source, SOURCE_BYTES);
        let negative_source_identity =
            compute_identity(IdentityDomain::Source, NEGATIVE_SOURCE_BYTES);
        let rule_identity = compute_identity(IdentityDomain::Source, RULE_BYTES);
        let findings = vec![AstGrepFinding {
            rule_id: "ambient-process".to_string(),
            source_identity: source_identity.clone(),
            start_byte: 0,
            end_byte: FINDING_END_BYTE,
            message: "ambient process construction".to_string(),
        }];
        let findings_bytes = canonical_json_bytes(&findings).ok()?;
        let scope_entries = vec![
            ScopeIdentityEntry {
                path: "rules/ambient-process.yml",
                identity: &rule_identity,
            },
            ScopeIdentityEntry {
                path: "fixtures/positive.rs",
                identity: &source_identity,
            },
            ScopeIdentityEntry {
                path: "fixtures/negative.rs",
                identity: &negative_source_identity,
            },
        ];
        let scope_bytes = canonical_json_bytes(&scope_entries).ok()?;
        let rule_bundle_identity = compute_identity(IdentityDomain::Adapter, RULE_BUNDLE_BYTES);
        let findings_identity = compute_identity(IdentityDomain::Adapter, &findings_bytes);
        let non_claims = vec!["syntax evidence only".to_string()];
        let language_profiles = vec![AstGrepLanguageProfile {
            language: "Rust".to_string(),
            parser_kind: BUILT_IN_PARSER_KIND.to_string(),
            parser_identity: compute_identity(IdentityDomain::Evaluator, b"ast-grep:Rust:test"),
            extensions: vec!["rs".to_string()],
        }];
        let rules = vec![AstGrepRuleCoverage {
            rule_id: "ambient-process".to_string(),
            language: "Rust".to_string(),
            rule_identity: rule_identity.clone(),
            positive_fixture_identity: Some(source_identity.clone()),
            negative_fixture_identity: Some(negative_source_identity.clone()),
            positive_cases: 1,
            negative_cases: 1,
            gate_eligible: true,
        }];
        let profile = AstGrepProfileProjection {
            schema_version: AST_GREP_PROFILE_SCHEMA_VERSION.to_string(),
            profile_id: "test-profile".to_string(),
            bundle_identity: rule_bundle_identity.clone(),
            language_profiles: language_profiles.clone(),
            rules: rules
                .iter()
                .map(|rule| {
                    Some(AstGrepProfileRule {
                        rule_id: rule.rule_id.clone(),
                        language: rule.language.clone(),
                        rule_identity: rule.rule_identity.clone(),
                        positive_fixture_identity: rule.positive_fixture_identity.clone()?,
                        negative_fixture_identity: rule.negative_fixture_identity.clone()?,
                        positive_cases: rule.positive_cases,
                        negative_cases: rule.negative_cases,
                        gate_eligible: rule.gate_eligible,
                    })
                })
                .collect::<Option<Vec<_>>>()?,
            verification_role: AST_GREP_BOUNDARY_ROLE.to_string(),
            non_claims: non_claims.clone(),
        };
        let profile_bytes = canonical_json_bytes(&profile).ok()?;
        let profile_identity = compute_identity(IdentityDomain::Adapter, &profile_bytes);
        let projection = AstGrepValenceProjection {
            schema_version: AST_GREP_VALENCE_SCHEMA_VERSION.to_string(),
            profile_identity: profile_identity.clone(),
            rule_bundle_identity: rule_bundle_identity.clone(),
            findings_identity: findings_identity.clone(),
            verification_role: AST_GREP_BOUNDARY_ROLE.to_string(),
            findings: findings.clone(),
            non_claims: non_claims.clone(),
        };
        let valence_projection_bytes = canonical_json_bytes(&projection).ok()?;
        Some(AstGrepAuditInput {
            schema_version: AST_GREP_AUDIT_SCHEMA_VERSION.to_string(),
            profile_bytes,
            profile_identity,
            rule_bundle_bytes: RULE_BUNDLE_BYTES.to_vec(),
            rule_bundle_identity,
            source_scope_identity: compute_identity(IdentityDomain::Adapter, &scope_bytes),
            findings_identity,
            valence_projection_identity: compute_identity(
                IdentityDomain::Projection,
                &valence_projection_bytes,
            ),
            valence_projection_bytes,
            language_profiles,
            rules,
            sources: vec![
                AstGrepSourceEnvelope {
                    path: "rules/ambient-process.yml".to_string(),
                    bytes: RULE_BYTES.to_vec(),
                    identity: rule_identity,
                },
                AstGrepSourceEnvelope {
                    path: "fixtures/positive.rs".to_string(),
                    bytes: SOURCE_BYTES.to_vec(),
                    identity: source_identity.clone(),
                },
                AstGrepSourceEnvelope {
                    path: "fixtures/negative.rs".to_string(),
                    bytes: NEGATIVE_SOURCE_BYTES.to_vec(),
                    identity: negative_source_identity,
                },
            ],
            findings,
            verification_role: AST_GREP_BOUNDARY_ROLE.to_string(),
            codemod: Some(AstGrepCodemodArtifact {
                recipe_bytes: b"recipe".to_vec(),
                recipe_identity: compute_identity(IdentityDomain::Source, b"recipe"),
                before_source_bytes: SOURCE_BYTES.to_vec(),
                before_source_identity: source_identity,
                after_source_bytes: b"fn main() { safe(); }".to_vec(),
                after_source_identity: compute_identity(
                    IdentityDomain::Source,
                    b"fn main() { safe(); }",
                ),
                dry_run: true,
                post_check_commands: vec!["cargo test --workspace".to_string()],
                non_claims: vec!["not semantic equivalence".to_string()],
            }),
            non_claims,
        })
    }

    #[test]
    fn accepts_identity_bound_audit_with_fixture_coverage() {
        let input = valid_input();
        assert!(input.is_some());
        let Some(input) = input else {
            return;
        };
        assert!(validate_ast_grep_audit(&input).is_empty());
    }

    #[test]
    fn rejects_gate_without_negative_fixture_and_semantic_role() {
        let input = valid_input();
        assert!(input.is_some());
        let Some(mut input) = input else {
            return;
        };
        input.rules[0].negative_fixture_identity = None;
        input.rules[0].negative_cases = 0;
        input.verification_role = "verified-semantic".to_string();

        let diagnostics = validate_ast_grep_audit(&input);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.requirement_id == REQUIREMENT_FIXTURES)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.requirement_id == REQUIREMENT_VALENCE)
        );
    }
}
