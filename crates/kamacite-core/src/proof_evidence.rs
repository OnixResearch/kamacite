use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::evidence_chain::{
    EVIDENCE_CHAIN_CANONICAL_CODEC, EvidenceChainEnvelope, EvidenceChainEnvelopeInput,
    EvidenceChainJsonProjection, EvidenceChainParentLink, EvidenceChainProfileRoot, EvidenceKind,
    ParentLinkDomain, build_evidence_chain_envelope, evidence_chain_json_projection,
    validate_evidence_chain_json_projection,
};
use crate::identity::{
    EvidenceChainIdentity, Identity, IdentityDomain, IdentityDomainMarker, ReceiptIdentity,
    SourceIdentity, WireIdentity,
};
use crate::preserves_codec::{
    PRESERVES_CANONICAL_CODEC, PreservesLimits, PreservesSchema, encode_typed_preserves,
};
use crate::receipt::ReceiptError;
use serde::{Deserialize, Serialize};

pub const TRELLIS_PROOF_PROFILE_SCHEMA_VERSION: &str = "kamacite.trellis-proof-evidence-profile.v2";
pub const TRELLIS_PROOF_PROFILE_CANONICAL_CODEC: &str = PRESERVES_CANONICAL_CODEC;
pub const TRELLIS_PROOF_PROFILE_JSON_PROJECTION: &str =
    "kamacite.trellis-proof-evidence-profile.v2.compat-json";
const TRELLIS_PROOF_PROFILE_ROOT_LABEL: &str = "kamacite-trellis-proof-evidence-profile-v2";
const TRELLIS_PROOF_COLLECTION_ROOT_LABEL: &str = "kamacite-trellis-proof-collection-v2";
const ASSUMPTIONS_ROOT_KIND: &str = "assumptions";
const DEPENDENCIES_ROOT_KIND: &str = "dependencies";
const REQUIREMENT_SCHEMA: &str =
    "r[kamacite.stack_integration.trellis_proof_evidence_envelope.schema]";
const REQUIREMENT_LINKS: &str =
    "r[kamacite.stack_integration.trellis_proof_evidence_envelope.links]";
const REQUIREMENT_ROLES: &str =
    "r[kamacite.stack_integration.trellis_proof_evidence_envelope.roles]";
const REQUIREMENT_PROJECTION: &str =
    "r[kamacite.stack_integration.trellis_proof_evidence_envelope.projection]";
const REQUIREMENT_NEGATIVE: &str =
    "r[kamacite.stack_integration.trellis_proof_evidence_envelope.negative]";
const NON_CLAIM_RELEASE_ELIGIBILITY: &str = "not release eligibility";
const NON_CLAIM_VERIFIER_SOUNDNESS: &str = "not verifier soundness";
const NON_CLAIM_DOWNSTREAM_CORRECTNESS: &str = "not downstream correctness";
const REQUIRED_NON_CLAIMS: &[&str] = &[
    NON_CLAIM_RELEASE_ELIGIBILITY,
    NON_CLAIM_VERIFIER_SOUNDNESS,
    NON_CLAIM_DOWNSTREAM_CORRECTNESS,
];
const FORBIDDEN_CLAIM_FRAGMENTS: &[&str] = &[
    "proves verifier soundness",
    "proves release eligibility",
    "proves downstream correctness",
    "proves octet acceptance",
    "proves mantle release",
    "proves cairn readiness",
];
const VALID_PROPERTY_ID_CHARS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._:-/[]";

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrellisProofEvidenceRole {
    RecordedOnly,
    FormalProofCandidate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TrellisProofEvidenceInput {
    pub producer: String,
    pub role: TrellisProofEvidenceRole,
    pub source_identity: WireIdentity,
    pub proof_ir_identity: WireIdentity,
    pub verifier_receipt_identity: WireIdentity,
    pub verifier_passed: bool,
    pub assumptions: Vec<String>,
    pub declared_assumptions_root: Option<WireIdentity>,
    pub dependency_identities: Vec<WireIdentity>,
    pub declared_dependency_root: Option<WireIdentity>,
    pub property_ids: Vec<String>,
    pub requirement_ids: Vec<String>,
    pub non_claims: Vec<String>,
    pub semantic_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-trellis-proof-evidence-profile-v2")]
pub struct TrellisProofEvidenceProfile {
    pub schema_version: String,
    pub canonical_codec: String,
    pub producer: String,
    pub role: TrellisProofEvidenceRole,
    pub source_identity: SourceIdentity,
    pub proof_ir_identity: EvidenceChainIdentity,
    pub verifier_receipt_identity: ReceiptIdentity,
    pub verifier_passed: bool,
    pub assumptions_root: EvidenceChainIdentity,
    pub dependency_root: EvidenceChainIdentity,
    pub property_ids: Vec<String>,
    pub requirement_ids: Vec<String>,
    pub non_claim_boundary_hash: EvidenceChainIdentity,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TrellisProofEvidenceEnvelope {
    pub profile: TrellisProofEvidenceProfile,
    pub profile_identity: EvidenceChainIdentity,
    pub evidence_chain: EvidenceChainEnvelope,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TrellisProofEvidenceJsonProjection {
    pub schema_version: String,
    pub compatibility_projection: String,
    pub canonical_identity_owner: String,
    pub profile_identity: EvidenceChainIdentity,
    pub evidence_chain_projection: EvidenceChainJsonProjection,
    pub profile: TrellisProofEvidenceProfile,
}

#[must_use]
pub fn validate_trellis_proof_evidence_input(input: &TrellisProofEvidenceInput) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_required_text(&input.producer, "producer", &mut diagnostics);
    validate_identity_domain(
        &input.source_identity,
        IdentityDomain::Source,
        "source_identity",
        &mut diagnostics,
    );
    validate_identity_domain(
        &input.proof_ir_identity,
        IdentityDomain::EvidenceChain,
        "proof_ir_identity",
        &mut diagnostics,
    );
    validate_identity_domain(
        &input.verifier_receipt_identity,
        IdentityDomain::Receipt,
        "verifier_receipt_identity",
        &mut diagnostics,
    );
    validate_role(input, &mut diagnostics);
    validate_assumptions(input, &mut diagnostics);
    validate_dependencies(input, &mut diagnostics);
    validate_property_ids(&input.property_ids, "property_ids", &mut diagnostics);
    validate_property_ids(&input.requirement_ids, "requirement_ids", &mut diagnostics);
    validate_non_claims(input, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

pub fn build_trellis_proof_evidence_envelope(
    input: &TrellisProofEvidenceInput,
    policy_hash: WireIdentity,
    profile_hash: WireIdentity,
) -> Result<TrellisProofEvidenceEnvelope, Vec<Diagnostic>> {
    let mut diagnostics = validate_trellis_proof_evidence_input(input);
    validate_identity_domain(
        &policy_hash,
        IdentityDomain::Receipt,
        "policy_hash",
        &mut diagnostics,
    );
    validate_identity_domain(
        &profile_hash,
        IdentityDomain::Receipt,
        "profile_hash",
        &mut diagnostics,
    );
    if !diagnostics.is_empty() {
        diagnostics.sort();
        diagnostics.dedup();
        return Err(diagnostics);
    }
    let profile = trellis_proof_evidence_profile(input)?;
    let profile_identity =
        trellis_proof_profile_identity(&profile).map_err(canonicalization_diagnostics)?;
    let evidence_chain_input = EvidenceChainEnvelopeInput {
        evidence_kind: EvidenceKind::Proof,
        policy_hash,
        profile_hash,
        parent_links: vec![
            EvidenceChainParentLink {
                domain: ParentLinkDomain::SourceRoot,
                identity: input.source_identity.clone(),
            },
            EvidenceChainParentLink {
                domain: ParentLinkDomain::ReceiptRoot,
                identity: input.verifier_receipt_identity.clone(),
            },
        ],
        profile_payload_roots: vec![EvidenceChainProfileRoot {
            evidence_kind: EvidenceKind::Proof,
            schema_version: TRELLIS_PROOF_PROFILE_SCHEMA_VERSION.to_string(),
            root_identity: profile_identity.to_wire(),
        }],
        projections: Vec::new(),
        non_claims: input.non_claims.clone(),
    };
    let evidence_chain = build_evidence_chain_envelope(&evidence_chain_input)?;
    Ok(TrellisProofEvidenceEnvelope {
        profile,
        profile_identity,
        evidence_chain,
    })
}

#[must_use]
pub fn trellis_proof_evidence_json_projection(
    envelope: &TrellisProofEvidenceEnvelope,
) -> TrellisProofEvidenceJsonProjection {
    TrellisProofEvidenceJsonProjection {
        schema_version: TRELLIS_PROOF_PROFILE_SCHEMA_VERSION.to_string(),
        compatibility_projection: TRELLIS_PROOF_PROFILE_JSON_PROJECTION.to_string(),
        canonical_identity_owner: EVIDENCE_CHAIN_CANONICAL_CODEC.to_string(),
        profile_identity: envelope.profile_identity.clone(),
        evidence_chain_projection: evidence_chain_json_projection(&envelope.evidence_chain),
        profile: envelope.profile.clone(),
    }
}

#[must_use]
pub fn validate_trellis_proof_evidence_json_projection(
    projection: &TrellisProofEvidenceJsonProjection,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if projection.compatibility_projection != TRELLIS_PROOF_PROFILE_JSON_PROJECTION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PROJECTION,
                "compatibility_projection",
                "Trellis proof projection declares an unsupported projection kind",
            )
            .with_detail("actual", projection.compatibility_projection.clone())
            .with_detail("expected", TRELLIS_PROOF_PROFILE_JSON_PROJECTION),
        );
    }
    if projection.canonical_identity_owner != EVIDENCE_CHAIN_CANONICAL_CODEC {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PROJECTION,
                "canonical_identity_owner",
                "Trellis proof projection must bind to canonical evidence-chain envelope identity",
            )
            .with_detail("actual", projection.canonical_identity_owner.clone())
            .with_detail("expected", EVIDENCE_CHAIN_CANONICAL_CODEC),
        );
    }
    match trellis_proof_profile_identity(&projection.profile) {
        Ok(identity) if identity == projection.profile_identity => {}
        Ok(identity) => diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_PROJECTION,
                "profile_identity",
                "Trellis proof profile identity does not match canonical profile bytes",
            )
            .with_detail("declared", projection.profile_identity.hex().as_str())
            .with_detail("computed", identity.hex().as_str()),
        ),
        Err(error) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_PROJECTION,
            "profile",
            format!("Trellis proof profile canonicalization failed: {error}"),
        )),
    }
    if !projection
        .evidence_chain_projection
        .envelope
        .profile_payload_roots
        .iter()
        .any(|root| root.root_identity == projection.profile_identity.to_wire())
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_PROJECTION,
            "evidence_chain_projection.envelope.profile_payload_roots",
            "Trellis proof projection profile identity is not bound by the evidence-chain envelope",
        ));
    }
    diagnostics.extend(validate_evidence_chain_json_projection(
        &projection.evidence_chain_projection,
    ));
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

pub fn trellis_proof_profile_canonical_bytes(
    profile: &TrellisProofEvidenceProfile,
) -> Result<Vec<u8>, ReceiptError> {
    typed_proof_bytes(
        profile,
        TRELLIS_PROOF_PROFILE_SCHEMA_VERSION,
        TRELLIS_PROOF_PROFILE_ROOT_LABEL,
    )
}

pub fn trellis_proof_assumptions_root(
    assumptions: &[String],
) -> Result<EvidenceChainIdentity, ReceiptError> {
    let root = CanonicalProofCollection {
        kind: ASSUMPTIONS_ROOT_KIND,
        values: sorted_strings(assumptions),
    };
    Ok(EvidenceChainIdentity::compute(&typed_proof_bytes(
        &root,
        TRELLIS_PROOF_PROFILE_SCHEMA_VERSION,
        TRELLIS_PROOF_COLLECTION_ROOT_LABEL,
    )?))
}

pub fn trellis_proof_dependency_root(
    dependencies: &[WireIdentity],
) -> Result<EvidenceChainIdentity, ReceiptError> {
    let mut dependencies = dependencies.to_vec();
    dependencies.sort();
    let root = CanonicalProofCollection {
        kind: DEPENDENCIES_ROOT_KIND,
        values: dependencies,
    };
    Ok(EvidenceChainIdentity::compute(&typed_proof_bytes(
        &root,
        TRELLIS_PROOF_PROFILE_SCHEMA_VERSION,
        TRELLIS_PROOF_COLLECTION_ROOT_LABEL,
    )?))
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-trellis-proof-collection-v2")]
struct CanonicalProofCollection<T> {
    kind: &'static str,
    values: Vec<T>,
}

fn trellis_proof_evidence_profile(
    input: &TrellisProofEvidenceInput,
) -> Result<TrellisProofEvidenceProfile, Vec<Diagnostic>> {
    Ok(TrellisProofEvidenceProfile {
        schema_version: TRELLIS_PROOF_PROFILE_SCHEMA_VERSION.to_string(),
        canonical_codec: TRELLIS_PROOF_PROFILE_CANONICAL_CODEC.to_string(),
        producer: input.producer.clone(),
        role: input.role.clone(),
        source_identity: admit_typed(&input.source_identity, "source_identity")?,
        proof_ir_identity: admit_typed(&input.proof_ir_identity, "proof_ir_identity")?,
        verifier_receipt_identity: admit_typed(
            &input.verifier_receipt_identity,
            "verifier_receipt_identity",
        )?,
        verifier_passed: input.verifier_passed,
        assumptions_root: trellis_proof_assumptions_root(&input.assumptions)
            .map_err(canonicalization_diagnostics)?,
        dependency_root: trellis_proof_dependency_root(&input.dependency_identities)
            .map_err(canonicalization_diagnostics)?,
        property_ids: sorted_strings(&input.property_ids),
        requirement_ids: sorted_strings(&input.requirement_ids),
        non_claim_boundary_hash: trellis_proof_assumptions_root(&input.non_claims)
            .map_err(canonicalization_diagnostics)?,
        non_claims: sorted_strings(&input.non_claims),
    })
}

fn admit_typed<D: IdentityDomainMarker>(
    wire: &WireIdentity,
    field: &str,
) -> Result<Identity<D>, Vec<Diagnostic>> {
    Identity::from_wire(wire.clone()).map_err(|error| {
        vec![Diagnostic::new(
            DiagnosticCode::IdentityDomainMismatch,
            REQUIREMENT_SCHEMA,
            field,
            format!("Trellis proof evidence identity admission failed: {error}"),
        )]
    })
}

fn canonicalization_diagnostics(error: ReceiptError) -> Vec<Diagnostic> {
    vec![Diagnostic::new(
        DiagnosticCode::MalformedProjectionOutput,
        REQUIREMENT_SCHEMA,
        "profile",
        format!("Trellis proof evidence profile canonicalization failed: {error}"),
    )]
}

fn trellis_proof_profile_identity(
    profile: &TrellisProofEvidenceProfile,
) -> Result<EvidenceChainIdentity, ReceiptError> {
    Ok(EvidenceChainIdentity::compute(
        &trellis_proof_profile_canonical_bytes(profile)?,
    ))
}

fn validate_required_text(value: &str, field: &str, diagnostics: &mut Vec<Diagnostic>) {
    if value.trim().is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_SCHEMA,
            field,
            "Trellis proof evidence profile requires a non-empty text field",
        ));
    }
}

fn validate_role(input: &TrellisProofEvidenceInput, diagnostics: &mut Vec<Diagnostic>) {
    if input.role == TrellisProofEvidenceRole::FormalProofCandidate && !input.verifier_passed {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedClaim,
            REQUIREMENT_ROLES,
            "role",
            "formal-proof-candidate role requires a passed verifier receipt but does not imply downstream promotion",
        ));
    }
}

fn validate_assumptions(input: &TrellisProofEvidenceInput, diagnostics: &mut Vec<Diagnostic>) {
    if input.assumptions.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_LINKS,
            "assumptions",
            "Trellis proof evidence must explicitly root assumptions, even when no assumptions are declared",
        ));
    }
    validate_declared_root_result(
        input.declared_assumptions_root.as_ref(),
        trellis_proof_assumptions_root(&input.assumptions),
        "declared_assumptions_root",
        diagnostics,
    );
}

fn validate_dependencies(input: &TrellisProofEvidenceInput, diagnostics: &mut Vec<Diagnostic>) {
    for dependency in &input.dependency_identities {
        validate_identity_domain(
            dependency,
            IdentityDomain::EvidenceChain,
            "dependency_identities",
            diagnostics,
        );
    }
    validate_declared_root_result(
        input.declared_dependency_root.as_ref(),
        trellis_proof_dependency_root(&input.dependency_identities),
        "declared_dependency_root",
        diagnostics,
    );
}

fn validate_declared_root_result(
    declared: Option<&WireIdentity>,
    computed: Result<EvidenceChainIdentity, ReceiptError>,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match computed {
        Ok(computed) => validate_declared_root(declared, computed, field, diagnostics),
        Err(error) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_SCHEMA,
            field,
            format!("Trellis proof evidence root canonicalization failed: {error}"),
        )),
    }
}

fn validate_declared_root(
    declared: Option<&WireIdentity>,
    computed: EvidenceChainIdentity,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match declared {
        Some(identity) if identity == &computed.to_wire() => {}
        Some(identity) => diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_NEGATIVE,
                field,
                "declared Trellis proof evidence root does not match canonical profile content",
            )
            .with_detail("declared", identity.hex.as_str())
            .with_detail("computed", computed.hex().as_str()),
        ),
        None => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_LINKS,
            field,
            "Trellis proof evidence must declare the root it binds for this profile section",
        )),
    }
}

fn validate_property_ids(ids: &[String], field: &str, diagnostics: &mut Vec<Diagnostic>) {
    if ids.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_LINKS,
            field,
            "Trellis proof evidence requires at least one property or requirement identifier",
        ));
    }
    for id in ids {
        if id.trim().is_empty()
            || id.chars().any(char::is_whitespace)
            || !id
                .chars()
                .all(|character| VALID_PROPERTY_ID_CHARS.contains(character))
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MalformedProjectionOutput,
                    REQUIREMENT_NEGATIVE,
                    field,
                    "Trellis proof evidence property identifiers must be stable non-whitespace tokens",
                )
                .with_detail("value", id.clone()),
            );
        }
    }
}

fn validate_non_claims(input: &TrellisProofEvidenceInput, diagnostics: &mut Vec<Diagnostic>) {
    for required in REQUIRED_NON_CLAIMS {
        if !input
            .non_claims
            .iter()
            .any(|non_claim| non_claim == required)
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingNonClaim,
                    REQUIREMENT_ROLES,
                    "non_claims",
                    "Trellis proof evidence must preserve bounded non-claim metadata",
                )
                .with_detail("missing", *required),
            );
        }
    }
    let claims = input.semantic_claims.join(" ").to_ascii_lowercase();
    for forbidden in FORBIDDEN_CLAIM_FRAGMENTS {
        if claims.contains(forbidden) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_ROLES,
                    "semantic_claims",
                    "Trellis proof evidence contains a downstream or verifier-soundness overclaim",
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
                REQUIREMENT_LINKS,
                field,
                "Trellis proof evidence identity uses the wrong domain or algorithm",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

fn sorted_strings(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

fn typed_proof_bytes<T: Serialize>(
    value: &T,
    schema_version: &str,
    root_label: &str,
) -> Result<Vec<u8>, ReceiptError> {
    encode_typed_preserves(
        value,
        PreservesSchema {
            schema_version,
            root_label,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ReceiptError::Serialize(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::compute_identity;

    const PRODUCER: &str = "trellis";
    const NO_EXTRA_ASSUMPTIONS: &str = "no extra assumptions declared";
    const PROPERTY_ID: &str = "trellis.proof::totality";
    const REQUIREMENT_ID: &str = "r[trellis.proof_evidence.totality]";

    fn valid_input(role: TrellisProofEvidenceRole) -> TrellisProofEvidenceInput {
        let assumptions = vec![NO_EXTRA_ASSUMPTIONS.to_string()];
        let dependencies = vec![compute_identity(
            IdentityDomain::EvidenceChain,
            b"dependency",
        )];
        TrellisProofEvidenceInput {
            producer: PRODUCER.to_string(),
            role,
            source_identity: compute_identity(IdentityDomain::Source, b"source"),
            proof_ir_identity: compute_identity(IdentityDomain::EvidenceChain, b"proof-ir"),
            verifier_receipt_identity: compute_identity(IdentityDomain::Receipt, b"verifier"),
            verifier_passed: true,
            declared_assumptions_root: trellis_proof_assumptions_root(&assumptions)
                .ok()
                .map(|typed| typed.to_wire()),
            declared_dependency_root: trellis_proof_dependency_root(&dependencies)
                .ok()
                .map(|typed| typed.to_wire()),
            assumptions,
            dependency_identities: dependencies,
            property_ids: vec![PROPERTY_ID.to_string()],
            requirement_ids: vec![REQUIREMENT_ID.to_string()],
            non_claims: vec![
                NON_CLAIM_RELEASE_ELIGIBILITY.to_string(),
                NON_CLAIM_VERIFIER_SOUNDNESS.to_string(),
                NON_CLAIM_DOWNSTREAM_CORRECTNESS.to_string(),
            ],
            semantic_claims: Vec::new(),
        }
    }

    #[test]
    fn builds_verified_and_recorded_trellis_profiles() {
        let verified_result = build_trellis_proof_evidence_envelope(
            &valid_input(TrellisProofEvidenceRole::FormalProofCandidate),
            compute_identity(IdentityDomain::Receipt, b"policy"),
            compute_identity(IdentityDomain::Receipt, b"profile"),
        );
        assert!(
            verified_result.is_ok(),
            "verified Trellis profile must build"
        );
        let Ok(verified) = verified_result else {
            return;
        };
        let recorded_result = build_trellis_proof_evidence_envelope(
            &valid_input(TrellisProofEvidenceRole::RecordedOnly),
            compute_identity(IdentityDomain::Receipt, b"policy"),
            compute_identity(IdentityDomain::Receipt, b"profile"),
        );
        assert!(
            recorded_result.is_ok(),
            "recorded Trellis profile must build"
        );
        let Ok(recorded) = recorded_result else {
            return;
        };

        assert_eq!(
            verified.evidence_chain.body.evidence_kind,
            EvidenceKind::Proof
        );
        assert_eq!(
            recorded.evidence_chain.body.evidence_kind,
            EvidenceKind::Proof
        );
        assert_ne!(verified.profile.role, recorded.profile.role);
    }

    #[test]
    fn projection_binds_profile_and_evidence_chain_identity() {
        let envelope_result = build_trellis_proof_evidence_envelope(
            &valid_input(TrellisProofEvidenceRole::FormalProofCandidate),
            compute_identity(IdentityDomain::Receipt, b"policy"),
            compute_identity(IdentityDomain::Receipt, b"profile"),
        );
        assert!(
            envelope_result.is_ok(),
            "projection base Trellis profile must build"
        );
        let Ok(envelope) = envelope_result else {
            return;
        };
        let projection = trellis_proof_evidence_json_projection(&envelope);

        assert_eq!(projection.profile_identity, envelope.profile_identity);
        assert_eq!(
            projection.evidence_chain_projection.envelope_identity,
            envelope.evidence_chain.envelope_identity
        );
        assert!(validate_trellis_proof_evidence_json_projection(&projection).is_empty());
    }

    #[test]
    fn rejects_missing_links_drift_malformed_ids_and_overclaims() {
        let mut input = valid_input(TrellisProofEvidenceRole::FormalProofCandidate);
        input.source_identity = compute_identity(IdentityDomain::Receipt, b"wrong source");
        input.verifier_receipt_identity =
            compute_identity(IdentityDomain::Source, b"wrong verifier");
        input.verifier_passed = false;
        input.assumptions.clear();
        input.declared_assumptions_root =
            Some(compute_identity(IdentityDomain::EvidenceChain, b"stale"));
        input.dependency_identities = vec![compute_identity(
            IdentityDomain::Receipt,
            b"wrong dependency",
        )];
        input.property_ids = vec!["bad property".to_string()];
        input.non_claims = Vec::new();
        input.semantic_claims = vec!["proves verifier soundness".to_string()];

        let diagnostics = validate_trellis_proof_evidence_input(&input);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "source_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "verifier_receipt_identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "role")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "assumptions")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "declared_assumptions_root")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "dependency_identities")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "property_ids")
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
    fn rejects_projection_drift() {
        let envelope_result = build_trellis_proof_evidence_envelope(
            &valid_input(TrellisProofEvidenceRole::RecordedOnly),
            compute_identity(IdentityDomain::Receipt, b"policy"),
            compute_identity(IdentityDomain::Receipt, b"profile"),
        );
        assert!(
            envelope_result.is_ok(),
            "projection drift base profile must build"
        );
        let Ok(envelope) = envelope_result else {
            return;
        };
        let mut projection = trellis_proof_evidence_json_projection(&envelope);
        projection.canonical_identity_owner = "json".to_string();
        projection.profile.property_ids.push("drifted".to_string());
        projection
            .evidence_chain_projection
            .envelope
            .profile_payload_roots
            .clear();

        let diagnostics = validate_trellis_proof_evidence_json_projection(&projection);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "canonical_identity_owner")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "profile_identity")
        );
        assert!(diagnostics.iter().any(|diagnostic| diagnostic.field
            == "evidence_chain_projection.envelope.profile_payload_roots"));
    }
}
