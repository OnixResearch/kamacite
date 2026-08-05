use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::function_address::FunctionAddressReceiptEnvelope;
use crate::identity::{EvidenceChainIdentity, IdentityDomain, WireIdentity};
use crate::preserves_codec::{
    PRESERVES_CANONICAL_CODEC, PreservesLimits, PreservesSchema, encode_typed_preserves,
};
use crate::receipt::ReceiptError;
use serde::{Deserialize, Serialize};

pub const EVIDENCE_CHAIN_ENVELOPE_SCHEMA_VERSION: &str = "kamacite.evidence-chain-envelope.v2";
pub const EVIDENCE_CHAIN_CANONICAL_CODEC: &str = PRESERVES_CANONICAL_CODEC;
pub const EVIDENCE_CHAIN_JSON_PROJECTION: &str = "kamacite.evidence-chain-envelope.v2.compat-json";
const EVIDENCE_CHAIN_ROOT_LABEL: &str = "kamacite-evidence-chain-envelope-v2";
const EVIDENCE_CHAIN_COLLECTION_ROOT_LABEL: &str = "kamacite-evidence-chain-collection-v2";
const REQUIREMENT_SCHEMA: &str =
    "r[kamacite.stack_integration.canonical_evidence_chain_envelope.schema]";
const REQUIREMENT_HASH: &str =
    "r[kamacite.stack_integration.canonical_evidence_chain_envelope.hash]";
const REQUIREMENT_LINKS: &str =
    "r[kamacite.stack_integration.canonical_evidence_chain_envelope.links]";
const REQUIREMENT_ROOTS: &str =
    "r[kamacite.stack_integration.canonical_evidence_chain_envelope.roots]";
const REQUIREMENT_PROJECTION: &str =
    "r[kamacite.stack_integration.canonical_evidence_chain_envelope.validation]";
const NON_CLAIM_RELEASE_ELIGIBILITY: &str = "not release eligibility";
const NON_CLAIM_VERIFIER_SOUNDNESS: &str = "not verifier soundness";
const REQUIRED_NON_CLAIMS: &[&str] = &[NON_CLAIM_RELEASE_ELIGIBILITY, NON_CLAIM_VERIFIER_SOUNDNESS];
const FORBIDDEN_CLAIM_FRAGMENTS: &[&str] = &[
    "proves release eligibility",
    "proves verifier soundness",
    "proves semantic correctness",
    "proves downstream correctness",
];

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceKind {
    FunctionAddress,
    Proof,
    Lint,
    Dependency,
    Build,
    Attestation,
    Lifecycle,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentLinkDomain {
    SourceRoot,
    ProducerHandoffRoot,
    ReceiptRoot,
    ValidationRoot,
    ReleaseRoot,
    ReadinessRoot,
}

impl ParentLinkDomain {
    fn expected_identity_domain(&self) -> IdentityDomain {
        match self {
            Self::SourceRoot => IdentityDomain::Source,
            Self::ProducerHandoffRoot
            | Self::ReceiptRoot
            | Self::ValidationRoot
            | Self::ReleaseRoot
            | Self::ReadinessRoot => IdentityDomain::Receipt,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EvidenceChainParentLink {
    pub domain: ParentLinkDomain,
    pub identity: WireIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EvidenceChainProfileRoot {
    pub evidence_kind: EvidenceKind,
    pub schema_version: String,
    pub root_identity: WireIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EvidenceChainProjectionRecord {
    pub projection_kind: String,
    pub projection_identity: WireIdentity,
    pub canonical_envelope_identity: WireIdentity,
    pub authority: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceChainEnvelopeInput {
    pub evidence_kind: EvidenceKind,
    pub policy_hash: WireIdentity,
    pub profile_hash: WireIdentity,
    pub parent_links: Vec<EvidenceChainParentLink>,
    pub profile_payload_roots: Vec<EvidenceChainProfileRoot>,
    pub projections: Vec<EvidenceChainProjectionRecord>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceChainEnvelopeBody {
    pub schema_version: String,
    pub canonical_codec: String,
    pub evidence_kind: EvidenceKind,
    pub policy_hash: WireIdentity,
    pub profile_hash: WireIdentity,
    pub parent_links: Vec<EvidenceChainParentLink>,
    pub profile_payload_roots: Vec<EvidenceChainProfileRoot>,
    pub non_claim_boundary_hash: EvidenceChainIdentity,
    pub projections: Vec<EvidenceChainProjectionRecord>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceChainEnvelope {
    pub body: EvidenceChainEnvelopeBody,
    pub envelope_identity: EvidenceChainIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvidenceChainJsonProjection {
    pub schema_version: String,
    pub compatibility_projection: String,
    pub canonical_identity_owner: String,
    pub envelope_identity: EvidenceChainIdentity,
    pub envelope: EvidenceChainEnvelopeBody,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-evidence-chain-envelope-v2")]
struct CanonicalEvidenceChainEnvelope<'a> {
    schema_version: &'a str,
    canonical_codec: &'a str,
    evidence_kind: &'a EvidenceKind,
    policy_hash: &'a WireIdentity,
    profile_hash: &'a WireIdentity,
    parent_links: Vec<EvidenceChainParentLink>,
    profile_payload_roots: Vec<EvidenceChainProfileRoot>,
    non_claim_boundary_hash: EvidenceChainIdentity,
    projections: Vec<EvidenceChainProjectionRecord>,
}

#[must_use]
pub fn validate_evidence_chain_envelope_input(
    input: &EvidenceChainEnvelopeInput,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_identity_domain(
        &input.policy_hash,
        IdentityDomain::Receipt,
        "policy_hash",
        &mut diagnostics,
    );
    validate_identity_domain(
        &input.profile_hash,
        IdentityDomain::Receipt,
        "profile_hash",
        &mut diagnostics,
    );
    validate_parent_links(&input.parent_links, &mut diagnostics);
    validate_profile_roots(
        &input.profile_payload_roots,
        &input.evidence_kind,
        &mut diagnostics,
    );
    validate_non_claims(&input.non_claims, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

pub fn build_evidence_chain_envelope(
    input: &EvidenceChainEnvelopeInput,
) -> Result<EvidenceChainEnvelope, Vec<Diagnostic>> {
    let diagnostics = validate_evidence_chain_envelope_input(input);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    build_evidence_chain_envelope_unchecked(input).map_err(|error| {
        vec![Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_HASH,
            "envelope",
            format!("evidence-chain envelope canonicalization failed: {error}"),
        )]
    })
}

pub fn evidence_chain_canonical_bytes(
    input: &EvidenceChainEnvelopeInput,
) -> Result<Vec<u8>, ReceiptError> {
    let canonical = canonical_evidence_chain(input)?;
    encode_typed_preserves(
        &canonical,
        PreservesSchema {
            schema_version: EVIDENCE_CHAIN_ENVELOPE_SCHEMA_VERSION,
            root_label: EVIDENCE_CHAIN_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ReceiptError::Serialize(error.to_string()))
}

#[must_use]
pub fn evidence_chain_json_projection(
    envelope: &EvidenceChainEnvelope,
) -> EvidenceChainJsonProjection {
    EvidenceChainJsonProjection {
        schema_version: EVIDENCE_CHAIN_ENVELOPE_SCHEMA_VERSION.to_string(),
        compatibility_projection: EVIDENCE_CHAIN_JSON_PROJECTION.to_string(),
        canonical_identity_owner: EVIDENCE_CHAIN_CANONICAL_CODEC.to_string(),
        envelope_identity: envelope.envelope_identity.clone(),
        envelope: envelope.body.clone(),
    }
}

#[must_use]
pub fn validate_evidence_chain_json_projection(
    projection: &EvidenceChainJsonProjection,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if projection.compatibility_projection != EVIDENCE_CHAIN_JSON_PROJECTION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PROJECTION,
                "compatibility_projection",
                "evidence-chain JSON projection declares an unsupported projection kind",
            )
            .with_detail("actual", projection.compatibility_projection.clone())
            .with_detail("expected", EVIDENCE_CHAIN_JSON_PROJECTION),
        );
    }
    if projection.canonical_identity_owner != EVIDENCE_CHAIN_CANONICAL_CODEC {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PROJECTION,
                "canonical_identity_owner",
                "evidence-chain JSON projection must bind to canonical Preserves envelope identity",
            )
            .with_detail("actual", projection.canonical_identity_owner.clone())
            .with_detail("expected", EVIDENCE_CHAIN_CANONICAL_CODEC),
        );
    }
    let input = EvidenceChainEnvelopeInput {
        evidence_kind: projection.envelope.evidence_kind.clone(),
        policy_hash: projection.envelope.policy_hash.clone(),
        profile_hash: projection.envelope.profile_hash.clone(),
        parent_links: projection.envelope.parent_links.clone(),
        profile_payload_roots: projection.envelope.profile_payload_roots.clone(),
        projections: projection.envelope.projections.clone(),
        non_claims: projection.envelope.non_claims.clone(),
    };
    match build_evidence_chain_envelope(&input) {
        Ok(envelope) if envelope.envelope_identity == projection.envelope_identity => {}
        Ok(envelope) => diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_PROJECTION,
                "envelope_identity",
                "evidence-chain JSON projection identity does not match canonical Preserves bytes",
            )
            .with_detail("declared", projection.envelope_identity.hex().as_str())
            .with_detail("computed", envelope.envelope_identity.hex().as_str()),
        ),
        Err(mut inner) => diagnostics.append(&mut inner),
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

pub fn evidence_chain_collection_root(
    evidence_kind: EvidenceKind,
    leaves: &[WireIdentity],
) -> Result<EvidenceChainIdentity, ReceiptError> {
    let mut leaves = leaves.to_vec();
    leaves.sort();
    let collection = CanonicalEvidenceCollection {
        evidence_kind,
        leaves,
    };
    let bytes = encode_typed_preserves(
        &collection,
        PreservesSchema {
            schema_version: EVIDENCE_CHAIN_ENVELOPE_SCHEMA_VERSION,
            root_label: EVIDENCE_CHAIN_COLLECTION_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ReceiptError::Serialize(error.to_string()))?;
    Ok(EvidenceChainIdentity::compute(&bytes))
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-evidence-chain-collection-v2")]
struct CanonicalEvidenceCollection {
    evidence_kind: EvidenceKind,
    leaves: Vec<WireIdentity>,
}

pub fn function_address_evidence_chain_input(
    receipt: &FunctionAddressReceiptEnvelope,
    policy_hash: WireIdentity,
    profile_hash: WireIdentity,
) -> Result<EvidenceChainEnvelopeInput, ReceiptError> {
    let leaves = receipt
        .body
        .functions
        .iter()
        .map(|function| function.function_address.clone())
        .collect::<Vec<_>>();
    let profile_root = evidence_chain_collection_root(EvidenceKind::FunctionAddress, &leaves)?;
    let source_links = receipt
        .body
        .source_envelopes
        .iter()
        .map(|source| EvidenceChainParentLink {
            domain: ParentLinkDomain::SourceRoot,
            identity: source.source_identity.clone(),
        })
        .collect::<Vec<_>>();
    Ok(EvidenceChainEnvelopeInput {
        evidence_kind: EvidenceKind::FunctionAddress,
        policy_hash,
        profile_hash,
        parent_links: source_links,
        profile_payload_roots: vec![EvidenceChainProfileRoot {
            evidence_kind: EvidenceKind::FunctionAddress,
            schema_version: receipt.body.schema_version.clone(),
            root_identity: profile_root.to_wire(),
        }],
        projections: Vec::new(),
        non_claims: receipt.body.non_claims.clone(),
    })
}

fn build_evidence_chain_envelope_unchecked(
    input: &EvidenceChainEnvelopeInput,
) -> Result<EvidenceChainEnvelope, ReceiptError> {
    let body = EvidenceChainEnvelopeBody {
        schema_version: EVIDENCE_CHAIN_ENVELOPE_SCHEMA_VERSION.to_string(),
        canonical_codec: EVIDENCE_CHAIN_CANONICAL_CODEC.to_string(),
        evidence_kind: input.evidence_kind.clone(),
        policy_hash: input.policy_hash.clone(),
        profile_hash: input.profile_hash.clone(),
        parent_links: sorted_parent_links(&input.parent_links),
        profile_payload_roots: sorted_profile_roots(&input.profile_payload_roots),
        non_claim_boundary_hash: non_claim_boundary_hash(&input.non_claims)?,
        projections: sorted_projections(&input.projections),
        non_claims: sorted_non_claims(&input.non_claims),
    };
    let canonical_input = EvidenceChainEnvelopeInput {
        evidence_kind: body.evidence_kind.clone(),
        policy_hash: body.policy_hash.clone(),
        profile_hash: body.profile_hash.clone(),
        parent_links: body.parent_links.clone(),
        profile_payload_roots: body.profile_payload_roots.clone(),
        projections: body.projections.clone(),
        non_claims: body.non_claims.clone(),
    };
    let bytes = evidence_chain_canonical_bytes(&canonical_input)?;
    Ok(EvidenceChainEnvelope {
        body,
        envelope_identity: EvidenceChainIdentity::compute(&bytes),
    })
}

fn canonical_evidence_chain(
    input: &EvidenceChainEnvelopeInput,
) -> Result<CanonicalEvidenceChainEnvelope<'_>, ReceiptError> {
    Ok(CanonicalEvidenceChainEnvelope {
        schema_version: EVIDENCE_CHAIN_ENVELOPE_SCHEMA_VERSION,
        canonical_codec: EVIDENCE_CHAIN_CANONICAL_CODEC,
        evidence_kind: &input.evidence_kind,
        policy_hash: &input.policy_hash,
        profile_hash: &input.profile_hash,
        parent_links: sorted_parent_links(&input.parent_links),
        profile_payload_roots: sorted_profile_roots(&input.profile_payload_roots),
        non_claim_boundary_hash: non_claim_boundary_hash(&input.non_claims)?,
        projections: sorted_projections(&input.projections),
    })
}

fn sorted_parent_links(parent_links: &[EvidenceChainParentLink]) -> Vec<EvidenceChainParentLink> {
    let mut links = parent_links.to_vec();
    links.sort();
    links.dedup();
    links
}

fn sorted_profile_roots(roots: &[EvidenceChainProfileRoot]) -> Vec<EvidenceChainProfileRoot> {
    let mut roots = roots.to_vec();
    roots.sort();
    roots.dedup();
    roots
}

fn sorted_projections(
    projections: &[EvidenceChainProjectionRecord],
) -> Vec<EvidenceChainProjectionRecord> {
    let mut projections = projections.to_vec();
    projections.sort();
    projections.dedup();
    projections
}

fn sorted_non_claims(non_claims: &[String]) -> Vec<String> {
    let mut non_claims = non_claims.to_vec();
    non_claims.sort();
    non_claims.dedup();
    non_claims
}

fn non_claim_boundary_hash(non_claims: &[String]) -> Result<EvidenceChainIdentity, ReceiptError> {
    Ok(EvidenceChainIdentity::compute(
        &crate::receipt::canonical_json_bytes(&sorted_non_claims(non_claims))?,
    ))
}

fn validate_parent_links(
    parent_links: &[EvidenceChainParentLink],
    diagnostics: &mut Vec<Diagnostic>,
) {
    if parent_links.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_LINKS,
            "parent_links",
            "evidence-chain envelopes require at least one typed parent link",
        ));
    }
    for link in parent_links {
        validate_identity_domain(
            &link.identity,
            link.domain.expected_identity_domain(),
            "parent_links.identity",
            diagnostics,
        );
    }
}

fn validate_profile_roots(
    roots: &[EvidenceChainProfileRoot],
    evidence_kind: &EvidenceKind,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if roots.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_ROOTS,
            "profile_payload_roots",
            "evidence-chain envelopes require at least one profile payload root",
        ));
    }
    for root in roots {
        if &root.evidence_kind != evidence_kind {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::IdentityDomainMismatch,
                    REQUIREMENT_ROOTS,
                    "profile_payload_roots.evidence_kind",
                    "profile payload root kind must match the envelope evidence kind",
                )
                .with_detail("envelope_kind", format!("{:?}", evidence_kind))
                .with_detail("root_kind", format!("{:?}", root.evidence_kind)),
            );
        }
        if root.schema_version.trim().is_empty() {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_ROOTS,
                "profile_payload_roots.schema_version",
                "profile payload roots must declare their profile schema version",
            ));
        }
        validate_identity_domain(
            &root.root_identity,
            IdentityDomain::EvidenceChain,
            "profile_payload_roots.root_identity",
            diagnostics,
        );
    }
}

fn validate_non_claims(non_claims: &[String], diagnostics: &mut Vec<Diagnostic>) {
    for required in REQUIRED_NON_CLAIMS {
        if !non_claims.iter().any(|non_claim| non_claim == required) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingNonClaim,
                    REQUIREMENT_SCHEMA,
                    "non_claims",
                    "evidence-chain envelopes must preserve required non-claim boundaries",
                )
                .with_detail("missing", *required),
            );
        }
    }
    let joined = non_claims.join(" ").to_ascii_lowercase();
    for forbidden in FORBIDDEN_CLAIM_FRAGMENTS {
        if joined.contains(forbidden) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_SCHEMA,
                    "non_claims",
                    "evidence-chain envelope contains forbidden overclaim text",
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
                "evidence-chain identity uses the wrong domain or algorithm",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function_address::{
        FUNCTION_ADDRESS_CANONICAL_CODEC, FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION,
        FunctionAddressProducer, FunctionAddressReceiptBody, FunctionAddressReceiptEnvelope,
        FunctionAddressRecord, FunctionAddressSourceEnvelope, FunctionAddressSpan,
        function_address_producer_profile,
    };
    use crate::identity::{ReceiptIdentity, compute_identity};
    use std::collections::BTreeMap;

    const NON_CLAIM_FUNCTION_SEMANTICS: &str = "not host-language semantic correctness";
    const SOURCE_KEY: &str = "src/lib.rs";
    const SOURCE_BYTES: &str = "pub fn f() -> u32 { 1 }\n";
    const FUNCTION_LOCATOR: &str = "src/lib.rs::f";
    const SPAN_START: u64 = 0;

    fn valid_input() -> Result<EvidenceChainEnvelopeInput, ReceiptError> {
        let source = compute_identity(IdentityDomain::Source, b"source");
        let leaf = compute_identity(IdentityDomain::FunctionAddress, b"function");
        Ok(EvidenceChainEnvelopeInput {
            evidence_kind: EvidenceKind::FunctionAddress,
            policy_hash: compute_identity(IdentityDomain::Receipt, b"policy"),
            profile_hash: compute_identity(IdentityDomain::Receipt, b"profile"),
            parent_links: vec![EvidenceChainParentLink {
                domain: ParentLinkDomain::SourceRoot,
                identity: source,
            }],
            profile_payload_roots: vec![EvidenceChainProfileRoot {
                evidence_kind: EvidenceKind::FunctionAddress,
                schema_version: "kamacite.function-address-receipt.v1".to_string(),
                root_identity: evidence_chain_collection_root(
                    EvidenceKind::FunctionAddress,
                    &[leaf],
                )?
                .to_wire(),
            }],
            projections: Vec::new(),
            non_claims: vec![
                NON_CLAIM_FUNCTION_SEMANTICS.to_string(),
                NON_CLAIM_RELEASE_ELIGIBILITY.to_string(),
                NON_CLAIM_VERIFIER_SOUNDNESS.to_string(),
            ],
        })
    }

    #[test]
    fn valid_evidence_chain_envelope_has_stable_canonical_identity_and_projection() {
        let input_result = valid_input();
        assert!(
            input_result.is_ok(),
            "valid evidence-chain fixture must canonicalize"
        );
        let Ok(input) = input_result else {
            return;
        };
        let envelope_result = build_evidence_chain_envelope(&input);
        assert!(
            envelope_result.is_ok(),
            "valid evidence-chain fixture must build"
        );
        let Ok(envelope) = envelope_result else {
            return;
        };
        let bytes_result = evidence_chain_canonical_bytes(&input);
        assert!(
            bytes_result.is_ok(),
            "valid evidence-chain fixture bytes must canonicalize"
        );
        let Ok(canonical_bytes) = bytes_result else {
            return;
        };
        let projection = evidence_chain_json_projection(&envelope);

        assert_eq!(
            EvidenceChainIdentity::compute(&canonical_bytes),
            envelope.envelope_identity
        );
        assert_eq!(projection.envelope_identity, envelope.envelope_identity);
        assert!(validate_evidence_chain_json_projection(&projection).is_empty());
    }

    #[test]
    fn collection_root_is_order_stable_for_profile_leaves() {
        let left = compute_identity(IdentityDomain::FunctionAddress, b"left");
        let right = compute_identity(IdentityDomain::FunctionAddress, b"right");

        let first = evidence_chain_collection_root(
            EvidenceKind::FunctionAddress,
            &[left.clone(), right.clone()],
        );
        let second = evidence_chain_collection_root(EvidenceKind::FunctionAddress, &[right, left]);

        assert!(first.is_ok());
        assert_eq!(first, second);
    }

    #[test]
    fn function_address_receipt_is_first_generic_profile() {
        let receipt = function_address_receipt();
        let input_result = function_address_evidence_chain_input(
            &receipt,
            compute_identity(IdentityDomain::Receipt, b"policy"),
            compute_identity(IdentityDomain::Receipt, b"profile"),
        );
        assert!(
            input_result.is_ok(),
            "function-address profile input must canonicalize"
        );
        let Ok(input) = input_result else {
            return;
        };

        let envelope_result = build_evidence_chain_envelope(&input);
        assert!(
            envelope_result.is_ok(),
            "function-address envelope must build"
        );
        let Ok(envelope) = envelope_result else {
            return;
        };

        assert_eq!(input.evidence_kind, EvidenceKind::FunctionAddress);
        assert_eq!(input.parent_links[0].domain, ParentLinkDomain::SourceRoot);
        assert_eq!(
            input.profile_payload_roots[0].evidence_kind,
            EvidenceKind::FunctionAddress
        );
        assert_eq!(envelope.body.evidence_kind, EvidenceKind::FunctionAddress);
    }

    #[test]
    fn invalid_evidence_chain_envelope_rejects_wrong_domains_and_weakened_non_claims() {
        let input_result = valid_input();
        assert!(
            input_result.is_ok(),
            "invalid-case base fixture must canonicalize"
        );
        let Ok(mut input) = input_result else {
            return;
        };
        input.policy_hash = compute_identity(IdentityDomain::Source, b"wrong policy");
        input.parent_links[0].identity = compute_identity(IdentityDomain::Receipt, b"wrong parent");
        input.profile_payload_roots[0].evidence_kind = EvidenceKind::Proof;
        input.profile_payload_roots[0].root_identity =
            compute_identity(IdentityDomain::Receipt, b"wrong root");
        input.non_claims = vec!["proves release eligibility".to_string()];

        let diagnostics = validate_evidence_chain_envelope_input(&input);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "policy_hash")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "parent_links.identity")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "profile_payload_roots.evidence_kind")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "profile_payload_roots.root_identity")
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
        let input_result = valid_input();
        assert!(input_result.is_ok(), "projection fixture must canonicalize");
        let Ok(input) = input_result else {
            return;
        };
        let envelope_result = build_evidence_chain_envelope(&input);
        assert!(
            envelope_result.is_ok(),
            "projection fixture envelope must build"
        );
        let Ok(envelope) = envelope_result else {
            return;
        };
        let mut projection = evidence_chain_json_projection(&envelope);
        projection.canonical_identity_owner = "json".to_string();
        projection.envelope.non_claims.clear();

        let diagnostics = validate_evidence_chain_json_projection(&projection);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "canonical_identity_owner")
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
    }

    fn function_address_receipt() -> FunctionAddressReceiptEnvelope {
        let source_identity = compute_identity(IdentityDomain::Source, SOURCE_BYTES.as_bytes());
        let function_address = compute_identity(IdentityDomain::FunctionAddress, b"function f");
        let producer = FunctionAddressProducer {
            name: "octet".to_string(),
            canonicalizer: "octet-source-token-function-address-v1".to_string(),
            tool_version: "octet-test".to_string(),
            build_identity: None,
            normative_options: BTreeMap::new(),
            non_claims: vec![
                "not producer correctness".to_string(),
                "not semantic dependency closure".to_string(),
            ],
        };
        let producer_profile = function_address_producer_profile(&producer).ok();
        FunctionAddressReceiptEnvelope {
            body: FunctionAddressReceiptBody {
                schema_version: FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION.to_string(),
                canonical_codec: FUNCTION_ADDRESS_CANONICAL_CODEC.to_string(),
                producer,
                producer_profile,
                source_envelopes: vec![FunctionAddressSourceEnvelope {
                    key: SOURCE_KEY.to_string(),
                    display_path: SOURCE_KEY.to_string(),
                    language: "rust".to_string(),
                    source_bytes: SOURCE_BYTES.to_string(),
                    source_identity: source_identity.clone(),
                }],
                functions: vec![FunctionAddressRecord {
                    locator: FUNCTION_LOCATOR.to_string(),
                    display_name: "f".to_string(),
                    source_key: SOURCE_KEY.to_string(),
                    span: Some(FunctionAddressSpan {
                        source_key: SOURCE_KEY.to_string(),
                        start_byte: SPAN_START,
                        end_byte: u64::try_from(SOURCE_BYTES.len()).unwrap_or(u64::MAX),
                    }),
                    function_address,
                    dependency_closure: None,
                    dependency_identity_kind: None,
                    caveats: Vec::new(),
                }],
                dependency_edges: Vec::new(),
                caveats: Vec::new(),
                non_claims: vec![
                    NON_CLAIM_FUNCTION_SEMANTICS.to_string(),
                    NON_CLAIM_RELEASE_ELIGIBILITY.to_string(),
                    NON_CLAIM_VERIFIER_SOUNDNESS.to_string(),
                ],
            },
            receipt_identity: ReceiptIdentity::compute(b"function-address receipt"),
        }
    }
}
