use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{
    DialectPayloadIdentity, Identity, IdentityDomain, IdentityDomainMarker, ProjectionIdentity,
    ReceiptIdentity, SourceIdentity, WireIdentity,
};
use serde::{Deserialize, Serialize};

pub const DIALECT_ROUNDTRIP_PROFILE_SCHEMA_VERSION: u32 = 1;
pub const DIALECT_ROUNDTRIP_JSON_PROJECTION: &str =
    "kamacite.dialect-roundtrip-profile.v1.compat-json";
pub const DIALECT_ROUNDTRIP_CANONICAL_OWNER: &str =
    "kamacite-preserves-canonical-binary-dialect-roundtrip-profile-v1";
const REQUIREMENT_PROFILE: &str =
    "r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.profile]";
const REQUIREMENT_CANONICAL: &str =
    "r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.canonical_preserves]";
const REQUIREMENT_NEGATIVE: &str =
    "r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.negative]";
const REQUIREMENT_VALENCE: &str =
    "r[kamacite.reference_ir_boundary.tmir_dialect_roundtrip.valence_export]";
const SUPPORTED_PAYLOAD_VERSION: u32 = 1;
const RECORDED_ONLY_ROLE: &str = "recorded-only";
const NON_CLAIM_NO_TMIR_DEPENDENCY: &str = "no tMIR dependency";
const NON_CLAIM_NO_PROOF_OBLIGATION_SEMANTICS: &str = "not proof-obligation semantics";
const NON_CLAIM_NO_SEMANTIC_EQUIVALENCE: &str = "not source-language semantic equivalence";
const REQUIRED_NON_CLAIMS: &[&str] = &[
    NON_CLAIM_NO_TMIR_DEPENDENCY,
    NON_CLAIM_NO_PROOF_OBLIGATION_SEMANTICS,
    NON_CLAIM_NO_SEMANTIC_EQUIVALENCE,
];
const FORBIDDEN_CLAIMS: &[&str] = &[
    "tmir dependency required",
    "proves proof obligations",
    "proves source-language semantic equivalence",
    "proves runtime behavior",
    "proves compiler correctness",
    "proves verifier soundness",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialectSourceSpan {
    pub source_identity: WireIdentity,
    pub start_byte: u64,
    pub end_byte: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialectRoundtripInput {
    pub schema_version: u32,
    pub namespace: String,
    pub operation_name: String,
    pub payload_version: u32,
    pub canonical_payload_bytes: Vec<u8>,
    pub declared_payload_identity: Option<WireIdentity>,
    pub source_identity: WireIdentity,
    pub source_len: u64,
    pub source_spans: Vec<DialectSourceSpan>,
    pub lowering_receipt_identities: Vec<WireIdentity>,
    pub compatibility_projection_identity: Option<WireIdentity>,
    pub non_claims: Vec<String>,
    pub semantic_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialectRoundtripProfile {
    pub schema_version: u32,
    pub namespace: String,
    pub operation_name: String,
    pub payload_version: u32,
    pub payload_identity: DialectPayloadIdentity,
    pub source_identity: SourceIdentity,
    pub source_spans: Vec<DialectSourceSpan>,
    pub lowering_receipt_identities: Vec<ReceiptIdentity>,
    pub compatibility_projection_identity: Option<ProjectionIdentity>,
    pub opaque_unknown_namespace: bool,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialectRoundtripJsonProjection {
    pub compatibility_projection: String,
    pub canonical_identity_owner: String,
    pub payload_identity: DialectPayloadIdentity,
    pub profile: DialectRoundtripProfile,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialectRoundtripValenceProjection {
    pub role: String,
    pub payload_identity: DialectPayloadIdentity,
    pub source_identity: SourceIdentity,
    pub lowering_receipt_identities: Vec<ReceiptIdentity>,
    pub non_claims: Vec<String>,
}

#[must_use]
pub fn validate_dialect_roundtrip_input(input: &DialectRoundtripInput) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if input.schema_version != DIALECT_ROUNDTRIP_PROFILE_SCHEMA_VERSION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_PROFILE,
                "schema_version",
                "dialect roundtrip profile schema version is unsupported",
            )
            .with_detail("actual", input.schema_version.to_string())
            .with_detail(
                "expected",
                DIALECT_ROUNDTRIP_PROFILE_SCHEMA_VERSION.to_string(),
            ),
        );
    }
    validate_required_text(&input.namespace, "namespace", &mut diagnostics);
    validate_required_text(&input.operation_name, "operation_name", &mut diagnostics);
    if input.payload_version != SUPPORTED_PAYLOAD_VERSION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_NEGATIVE,
                "payload_version",
                "dialect roundtrip payload version is unsupported",
            )
            .with_detail("actual", input.payload_version.to_string())
            .with_detail("expected", SUPPORTED_PAYLOAD_VERSION.to_string()),
        );
    }
    if input.canonical_payload_bytes.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_CANONICAL,
            "canonical_payload_bytes",
            "dialect roundtrip validation requires canonical Preserves payload bytes",
        ));
    }
    let computed_payload = dialect_payload_identity(&input.canonical_payload_bytes);
    if let Some(declared) = &input.declared_payload_identity {
        validate_identity_domain(
            declared,
            IdentityDomain::DialectPayload,
            "declared_payload_identity",
            &mut diagnostics,
        );
        if declared.domain == IdentityDomain::DialectPayload
            && declared.hex != *computed_payload.hex()
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::StaleIdentity,
                    REQUIREMENT_CANONICAL,
                    "declared_payload_identity.hex",
                    "dialect roundtrip declared payload identity is stale",
                )
                .with_detail("declared", declared.hex.as_str())
                .with_detail("computed", computed_payload.hex().as_str()),
            );
        }
    }
    validate_identity_domain(
        &input.source_identity,
        IdentityDomain::Source,
        "source_identity",
        &mut diagnostics,
    );
    validate_source_spans(input, &mut diagnostics);
    for receipt_identity in &input.lowering_receipt_identities {
        validate_identity_domain(
            receipt_identity,
            IdentityDomain::Receipt,
            "lowering_receipt_identities",
            &mut diagnostics,
        );
    }
    if let Some(projection_identity) = &input.compatibility_projection_identity {
        validate_identity_domain(
            projection_identity,
            IdentityDomain::Projection,
            "compatibility_projection_identity",
            &mut diagnostics,
        );
    }
    validate_non_claims(input, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

pub fn dialect_roundtrip_profile(
    input: &DialectRoundtripInput,
) -> Result<DialectRoundtripProfile, Vec<Diagnostic>> {
    let diagnostics = validate_dialect_roundtrip_input(input);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    Ok(DialectRoundtripProfile {
        schema_version: input.schema_version,
        namespace: input.namespace.clone(),
        operation_name: input.operation_name.clone(),
        payload_version: input.payload_version,
        payload_identity: dialect_payload_identity(&input.canonical_payload_bytes),
        source_identity: admit_identity(&input.source_identity, "source_identity")?,
        source_spans: sorted_source_spans(&input.source_spans),
        lowering_receipt_identities: sorted_typed_identities(
            &input.lowering_receipt_identities,
            "lowering_receipt_identities",
        )?,
        compatibility_projection_identity: input
            .compatibility_projection_identity
            .as_ref()
            .map(|identity| admit_identity(identity, "compatibility_projection_identity"))
            .transpose()?,
        opaque_unknown_namespace: is_unknown_namespace(&input.namespace),
        non_claims: sorted_strings(&input.non_claims),
    })
}

fn sorted_typed_identities<D: IdentityDomainMarker>(
    wires: &[WireIdentity],
    field: &str,
) -> Result<Vec<Identity<D>>, Vec<Diagnostic>> {
    let mut admitted = wires
        .iter()
        .map(|identity| admit_identity(identity, field))
        .collect::<Result<Vec<_>, _>>()?;
    admitted.sort();
    admitted.dedup();
    Ok(admitted)
}

fn admit_identity<D: IdentityDomainMarker>(
    wire: &WireIdentity,
    field: &str,
) -> Result<Identity<D>, Vec<Diagnostic>> {
    Identity::from_wire(wire.clone()).map_err(|error| {
        vec![Diagnostic::new(
            DiagnosticCode::IdentityDomainMismatch,
            REQUIREMENT_PROFILE,
            field,
            format!("dialect roundtrip identity admission failed: {error}"),
        )]
    })
}

#[must_use]
pub fn dialect_roundtrip_json_projection(
    profile: &DialectRoundtripProfile,
) -> DialectRoundtripJsonProjection {
    DialectRoundtripJsonProjection {
        compatibility_projection: DIALECT_ROUNDTRIP_JSON_PROJECTION.to_string(),
        canonical_identity_owner: DIALECT_ROUNDTRIP_CANONICAL_OWNER.to_string(),
        payload_identity: profile.payload_identity.clone(),
        profile: profile.clone(),
    }
}

#[must_use]
pub fn validate_dialect_roundtrip_json_projection(
    projection: &DialectRoundtripJsonProjection,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if projection.compatibility_projection != DIALECT_ROUNDTRIP_JSON_PROJECTION {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_NEGATIVE,
            "compatibility_projection",
            "dialect roundtrip JSON projection kind is unsupported",
        ));
    }
    if projection.canonical_identity_owner != DIALECT_ROUNDTRIP_CANONICAL_OWNER {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_CANONICAL,
            "canonical_identity_owner",
            "dialect roundtrip JSON projection must identify canonical Preserves authority",
        ));
    }
    if projection.payload_identity != projection.profile.payload_identity {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_NEGATIVE,
            "payload_identity",
            "dialect roundtrip JSON projection payload identity drifted from profile",
        ));
    }
    validate_profile_non_claims(
        &projection.profile.non_claims,
        "profile.non_claims",
        &mut diagnostics,
    );
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

#[must_use]
pub fn dialect_roundtrip_valence_projection(
    profile: &DialectRoundtripProfile,
) -> DialectRoundtripValenceProjection {
    DialectRoundtripValenceProjection {
        role: RECORDED_ONLY_ROLE.to_string(),
        payload_identity: profile.payload_identity.clone(),
        source_identity: profile.source_identity.clone(),
        lowering_receipt_identities: profile.lowering_receipt_identities.clone(),
        non_claims: profile.non_claims.clone(),
    }
}

#[must_use]
pub fn dialect_payload_identity(canonical_payload_bytes: &[u8]) -> DialectPayloadIdentity {
    DialectPayloadIdentity::compute(canonical_payload_bytes)
}

fn validate_source_spans(input: &DialectRoundtripInput, diagnostics: &mut Vec<Diagnostic>) {
    if input.source_spans.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_PROFILE,
            "source_spans",
            "dialect roundtrip profile requires at least one source span",
        ));
    }
    for span in &input.source_spans {
        validate_identity_domain(
            &span.source_identity,
            IdentityDomain::Source,
            "source_spans.source_identity",
            diagnostics,
        );
        if span.source_identity != input.source_identity {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::SourceHashMismatch,
                REQUIREMENT_PROFILE,
                "source_spans.source_identity",
                "dialect roundtrip span source identity must match the profile source envelope",
            ));
        }
        if span.end_byte <= span.start_byte || span.end_byte > input.source_len {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::SourceSpanFabrication,
                REQUIREMENT_NEGATIVE,
                "source_spans",
                "dialect roundtrip source span must be a non-empty byte range inside the source envelope",
            ));
        }
    }
}

fn validate_non_claims(input: &DialectRoundtripInput, diagnostics: &mut Vec<Diagnostic>) {
    validate_profile_non_claims(&input.non_claims, "non_claims", diagnostics);
    let claims = input.semantic_claims.join(" ").to_ascii_lowercase();
    for forbidden in FORBIDDEN_CLAIMS {
        if claims.contains(forbidden) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_NEGATIVE,
                    "semantic_claims",
                    "dialect roundtrip profile contains a forbidden semantic overclaim",
                )
                .with_detail("claim", *forbidden),
            );
        }
    }
}

fn validate_profile_non_claims(
    non_claims: &[String],
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for required in REQUIRED_NON_CLAIMS {
        if !non_claims.iter().any(|non_claim| non_claim == required) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingNonClaim,
                    REQUIREMENT_VALENCE,
                    field,
                    "dialect roundtrip profile is missing a required non-claim boundary",
                )
                .with_detail("missing", *required),
            );
        }
    }
}

fn validate_required_text(value: &str, field: &str, diagnostics: &mut Vec<Diagnostic>) {
    if value.trim().is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_PROFILE,
            field,
            "dialect roundtrip profile requires a non-empty text field",
        ));
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
                REQUIREMENT_PROFILE,
                field,
                "dialect roundtrip identity uses the wrong domain or algorithm",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

fn is_unknown_namespace(namespace: &str) -> bool {
    !namespace.starts_with("kamacite.example.")
}

fn sorted_source_spans(spans: &[DialectSourceSpan]) -> Vec<DialectSourceSpan> {
    let mut spans = spans.to_vec();
    spans.sort_by(|left, right| {
        left.source_identity
            .cmp(&right.source_identity)
            .then(left.start_byte.cmp(&right.start_byte))
            .then(left.end_byte.cmp(&right.end_byte))
    });
    spans.dedup();
    spans
}

fn sorted_strings(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::compute_identity;

    const KNOWN_NAMESPACE: &str = "kamacite.example.adapter";
    const UNKNOWN_NAMESPACE: &str = "foreign.adapter";
    const OPERATION_NAME: &str = "emit";
    const SOURCE_LEN: u64 = 64;
    const SPAN_START: u64 = 2;
    const SPAN_END: u64 = 12;

    fn source_identity() -> WireIdentity {
        compute_identity(IdentityDomain::Source, b"source")
    }

    fn non_claims() -> Vec<String> {
        REQUIRED_NON_CLAIMS
            .iter()
            .map(|non_claim| (*non_claim).to_string())
            .collect()
    }

    fn valid_input(namespace: &str) -> DialectRoundtripInput {
        let payload = b"<dialect-payload>".to_vec();
        DialectRoundtripInput {
            schema_version: DIALECT_ROUNDTRIP_PROFILE_SCHEMA_VERSION,
            namespace: namespace.to_string(),
            operation_name: OPERATION_NAME.to_string(),
            payload_version: SUPPORTED_PAYLOAD_VERSION,
            declared_payload_identity: Some(dialect_payload_identity(&payload).to_wire()),
            canonical_payload_bytes: payload,
            source_identity: source_identity(),
            source_len: SOURCE_LEN,
            source_spans: vec![DialectSourceSpan {
                source_identity: source_identity(),
                start_byte: SPAN_START,
                end_byte: SPAN_END,
            }],
            lowering_receipt_identities: vec![compute_identity(
                IdentityDomain::Receipt,
                b"lowering",
            )],
            compatibility_projection_identity: Some(compute_identity(
                IdentityDomain::Projection,
                b"projection",
            )),
            non_claims: non_claims(),
            semantic_claims: Vec::new(),
        }
    }

    #[test]
    fn known_and_unknown_dialect_payloads_roundtrip_with_preserves_identity() {
        let known_result = dialect_roundtrip_profile(&valid_input(KNOWN_NAMESPACE));
        assert!(known_result.is_ok(), "known dialect profile must validate");
        let Ok(known) = known_result else {
            return;
        };
        let unknown_result = dialect_roundtrip_profile(&valid_input(UNKNOWN_NAMESPACE));
        assert!(
            unknown_result.is_ok(),
            "unknown dialect profile must validate as opaque"
        );
        let Ok(unknown) = unknown_result else {
            return;
        };

        assert!(!known.opaque_unknown_namespace);
        assert!(unknown.opaque_unknown_namespace);
        assert_eq!(
            known.payload_identity.domain(),
            IdentityDomain::DialectPayload
        );
        assert_eq!(
            dialect_roundtrip_valence_projection(&known).role,
            RECORDED_ONLY_ROLE
        );
    }

    #[test]
    fn json_projection_binds_preserves_authority() {
        let profile_result = dialect_roundtrip_profile(&valid_input(KNOWN_NAMESPACE));
        assert!(
            profile_result.is_ok(),
            "projection base profile must validate"
        );
        let Ok(profile) = profile_result else {
            return;
        };
        let projection = dialect_roundtrip_json_projection(&profile);

        assert!(validate_dialect_roundtrip_json_projection(&projection).is_empty());
    }

    #[test]
    fn invalid_profile_rejects_stale_hash_domains_spans_versions_and_overclaims() {
        let mut input = valid_input(KNOWN_NAMESPACE);
        input.payload_version = SUPPORTED_PAYLOAD_VERSION + 1;
        input.declared_payload_identity =
            Some(compute_identity(IdentityDomain::DialectPayload, b"stale"));
        input.source_identity = compute_identity(IdentityDomain::Receipt, b"wrong source");
        input.source_spans[0].end_byte = SOURCE_LEN + 1;
        input.lowering_receipt_identities =
            vec![compute_identity(IdentityDomain::Source, b"wrong receipt")];
        input.compatibility_projection_identity = Some(compute_identity(
            IdentityDomain::Receipt,
            b"wrong projection",
        ));
        input.non_claims.clear();
        input.semantic_claims = vec!["proves source-language semantic equivalence".to_string()];

        let diagnostics = validate_dialect_roundtrip_input(&input);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "payload_version")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "declared_payload_identity.hex")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "source_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "source_spans")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "lowering_receipt_identities")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "compatibility_projection_identity")
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
    fn projection_drift_is_rejected() {
        let profile_result = dialect_roundtrip_profile(&valid_input(KNOWN_NAMESPACE));
        assert!(
            profile_result.is_ok(),
            "projection drift base profile must validate"
        );
        let Ok(profile) = profile_result else {
            return;
        };
        let mut projection = dialect_roundtrip_json_projection(&profile);
        projection.canonical_identity_owner = "json-only".to_string();
        projection.payload_identity = DialectPayloadIdentity::compute(b"other");
        projection.profile.non_claims.clear();

        let diagnostics = validate_dialect_roundtrip_json_projection(&projection);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "canonical_identity_owner")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "payload_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
    }
}
