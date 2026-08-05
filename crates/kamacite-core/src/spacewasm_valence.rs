use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    AnyIdentity, ProjectionIdentity, ReceiptIdentity, SourceSpan, SourceSpanSet,
    SpaceWasmParentRole, SpaceWasmStaticRequest, SpaceWasmValidationError,
    build_spacewasm_static_receipt,
};

pub const SPACEWASM_VALENCE_PROJECTION_SCHEMA: &str = "kamacite.spacewasm-valence-projection.v1";

const SPACEWASM_VALENCE_COUPLING_BOUNDARY: &str =
    "Valence records exact SpaceWasm static evidence roles without transferring producer trust";
const SPACEWASM_PARENT_RELATION: &str = "parent-of";
const SPACEWASM_DESCRIPTOR_ROLE: &str = "adapter-descriptor";
const SPACEWASM_RECEIPT_ROLE: &str = "static-denial-receipt";
const SPACEWASM_ABSENT_SPAN_REASON: &str =
    "identity has no byte-local source span in this projection";

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpaceWasmValenceVerificationClass {
    RecordedOnly,
    Boundary,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmValenceArtifact {
    pub role: String,
    pub identity: AnyIdentity,
    pub verification_class: SpaceWasmValenceVerificationClass,
    pub spans: SourceSpanSet,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmValenceEdge {
    pub parent_role: String,
    pub parent_identity: AnyIdentity,
    pub child_role: String,
    pub child_identity: AnyIdentity,
    pub relation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmValenceProjectionBody {
    pub schema_version: String,
    pub subject_receipt_identity: ReceiptIdentity,
    pub artifacts: Vec<SpaceWasmValenceArtifact>,
    pub parent_edges: Vec<SpaceWasmValenceEdge>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
    pub coupling_boundary: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmValenceProjection {
    pub body: SpaceWasmValenceProjectionBody,
    pub projection_identity: ProjectionIdentity,
}

#[derive(Debug)]
pub enum SpaceWasmValenceProjectionError {
    StaticValidation(SpaceWasmValidationError),
    Encoding(String),
    SizeOverflow { artifact: &'static str },
}

impl Display for SpaceWasmValenceProjectionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaticValidation(error) => Display::fmt(error, formatter),
            Self::Encoding(error) => {
                write!(formatter, "SpaceWasm Valence encoding failed: {error}")
            }
            Self::SizeOverflow { artifact } => {
                write!(
                    formatter,
                    "SpaceWasm Valence span size overflow for {artifact}"
                )
            }
        }
    }
}

impl std::error::Error for SpaceWasmValenceProjectionError {}

impl From<SpaceWasmValidationError> for SpaceWasmValenceProjectionError {
    fn from(error: SpaceWasmValidationError) -> Self {
        Self::StaticValidation(error)
    }
}

// r[impl kamacite.spacewasm_adapter.valence_export]
pub fn project_spacewasm_static_to_valence(
    request: &SpaceWasmStaticRequest<'_>,
) -> Result<SpaceWasmValenceProjection, SpaceWasmValenceProjectionError> {
    let receipt = build_spacewasm_static_receipt(request)?;
    let descriptor_bytes = serde_json::to_vec(request.descriptor)
        .map_err(|error| SpaceWasmValenceProjectionError::Encoding(error.to_string()))?;
    let body = SpaceWasmValenceProjectionBody {
        schema_version: SPACEWASM_VALENCE_PROJECTION_SCHEMA.to_string(),
        subject_receipt_identity: receipt.receipt_identity.clone(),
        artifacts: artifacts(request, &receipt, &descriptor_bytes)?,
        parent_edges: parent_edges(request, &receipt)?,
        checked_laws: receipt.body.checked_laws.clone(),
        non_claims: receipt.body.non_claims.clone(),
        coupling_boundary: SPACEWASM_VALENCE_COUPLING_BOUNDARY.to_string(),
    };
    let projection_bytes = serde_json::to_vec(&body)
        .map_err(|error| SpaceWasmValenceProjectionError::Encoding(error.to_string()))?;
    Ok(SpaceWasmValenceProjection {
        body,
        projection_identity: ProjectionIdentity::compute(&projection_bytes),
    })
}

fn artifacts(
    request: &SpaceWasmStaticRequest<'_>,
    receipt: &crate::SpaceWasmStaticReceipt,
    descriptor_bytes: &[u8],
) -> Result<Vec<SpaceWasmValenceArtifact>, SpaceWasmValenceProjectionError> {
    let mut artifacts = vec![
        artifact_with_span(
            "source-envelope",
            AnyIdentity::Source(receipt.body.source_identity.clone()),
            SpaceWasmValenceVerificationClass::RecordedOnly,
            "source-envelope",
            request.source_envelope_bytes.len(),
        )?,
        artifact_with_span(
            "canonical-input",
            AnyIdentity::CanonicalValue(receipt.body.canonical_input_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
            "canonical-input",
            request.canonical_input_bytes.len(),
        )?,
        artifact_without_span(
            "input-schema",
            AnyIdentity::Projection(receipt.body.input_schema_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
        ),
        artifact_without_span(
            "output-schema",
            AnyIdentity::Projection(receipt.body.output_schema_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
        ),
        artifact_with_span(
            "adapter-profile",
            AnyIdentity::Adapter(receipt.body.profile_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
            "adapters/spacewasm-mvp/generated/profile.json",
            request.profile_bytes.len(),
        )?,
        artifact_without_span(
            "abi-profile",
            AnyIdentity::Adapter(receipt.body.abi_profile_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
        ),
        artifact_without_span(
            "capability-plan",
            AnyIdentity::Adapter(receipt.body.capability_plan_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
        ),
        artifact_without_span(
            "effect-plan",
            AnyIdentity::AlgebraicEffect(receipt.body.effect_plan_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
        ),
        artifact_without_span(
            "resource-plan",
            AnyIdentity::Adapter(receipt.body.resource_plan_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
        ),
        artifact_with_span(
            SPACEWASM_DESCRIPTOR_ROLE,
            AnyIdentity::Adapter(receipt.body.descriptor_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
            "descriptor-json",
            descriptor_bytes.len(),
        )?,
        artifact_with_span(
            "core-module",
            AnyIdentity::TargetArtifact(receipt.body.module_identity.clone()),
            SpaceWasmValenceVerificationClass::RecordedOnly,
            "adapters/spacewasm-mvp/upstream/mantle/mvp-positive.wasm",
            request.module_bytes.len(),
        )?,
        artifact_with_span(
            "spacewasm-runtime-profile",
            AnyIdentity::Evaluator(receipt.body.runtime_identity.clone()),
            SpaceWasmValenceVerificationClass::RecordedOnly,
            "adapters/spacewasm-mvp/upstream/mantle/profile.json",
            request.mantle_profile_bytes.len(),
        )?,
        artifact_without_span(
            "mantle-bundle",
            AnyIdentity::DependencyClosure(receipt.body.mantle_bundle_identity.clone()),
            SpaceWasmValenceVerificationClass::RecordedOnly,
        ),
        artifact_with_span(
            "octet-static-report",
            AnyIdentity::Projection(receipt.body.octet_report_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
            "adapters/spacewasm-mvp/upstream/octet/evidence-rail-receipt.json",
            request.octet_report_bytes.len(),
        )?,
        artifact_without_span(
            SPACEWASM_RECEIPT_ROLE,
            AnyIdentity::Receipt(receipt.receipt_identity.clone()),
            SpaceWasmValenceVerificationClass::Boundary,
        ),
    ];
    artifacts.sort_by(|left, right| left.role.cmp(&right.role));
    Ok(artifacts)
}

fn parent_edges(
    request: &SpaceWasmStaticRequest<'_>,
    receipt: &crate::SpaceWasmStaticReceipt,
) -> Result<Vec<SpaceWasmValenceEdge>, SpaceWasmValenceProjectionError> {
    let mut edges = request
        .descriptor
        .parent_edges
        .iter()
        .map(|edge| {
            AnyIdentity::admit(edge.parent_identity.clone()).map(|parent| SpaceWasmValenceEdge {
                parent_role: parent_role_label(&edge.role).to_string(),
                parent_identity: parent,
                child_role: SPACEWASM_DESCRIPTOR_ROLE.to_string(),
                child_identity: AnyIdentity::Adapter(receipt.body.descriptor_identity.clone()),
                relation: SPACEWASM_PARENT_RELATION.to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| SpaceWasmValenceProjectionError::Encoding(error.to_string()))?;
    edges.push(SpaceWasmValenceEdge {
        parent_role: SPACEWASM_DESCRIPTOR_ROLE.to_string(),
        parent_identity: AnyIdentity::Adapter(receipt.body.descriptor_identity.clone()),
        child_role: SPACEWASM_RECEIPT_ROLE.to_string(),
        child_identity: AnyIdentity::Receipt(receipt.receipt_identity.clone()),
        relation: SPACEWASM_PARENT_RELATION.to_string(),
    });
    edges.sort();
    Ok(edges)
}

fn artifact_with_span(
    role: &str,
    identity: AnyIdentity,
    verification_class: SpaceWasmValenceVerificationClass,
    artifact_key: &str,
    byte_len: usize,
) -> Result<SpaceWasmValenceArtifact, SpaceWasmValenceProjectionError> {
    let end_byte =
        u64::try_from(byte_len).map_err(|_| SpaceWasmValenceProjectionError::SizeOverflow {
            artifact: "byte-bearing artifact",
        })?;
    Ok(SpaceWasmValenceArtifact {
        role: role.to_string(),
        identity,
        verification_class,
        spans: SourceSpanSet::Present {
            spans: vec![SourceSpan {
                artifact_key: artifact_key.to_string(),
                start_byte: 0,
                end_byte,
            }],
        },
    })
}

fn artifact_without_span(
    role: &str,
    identity: AnyIdentity,
    verification_class: SpaceWasmValenceVerificationClass,
) -> SpaceWasmValenceArtifact {
    SpaceWasmValenceArtifact {
        role: role.to_string(),
        identity,
        verification_class,
        spans: SourceSpanSet::Absent {
            reason: SPACEWASM_ABSENT_SPAN_REASON.to_string(),
        },
    }
}

fn parent_role_label(role: &SpaceWasmParentRole) -> &'static str {
    match role {
        SpaceWasmParentRole::SourceEnvelope => "source-envelope",
        SpaceWasmParentRole::CanonicalInput => "canonical-input",
        SpaceWasmParentRole::InputSchema => "input-schema",
        SpaceWasmParentRole::OutputSchema => "output-schema",
        SpaceWasmParentRole::AdapterProfile => "adapter-profile",
        SpaceWasmParentRole::AbiProfile => "abi-profile",
        SpaceWasmParentRole::CapabilityPlan => "capability-plan",
        SpaceWasmParentRole::EffectPlan => "effect-plan",
        SpaceWasmParentRole::ResourcePlan => "resource-plan",
        SpaceWasmParentRole::Module => "core-module",
        SpaceWasmParentRole::SpacewasmRuntime => "spacewasm-runtime-profile",
        SpaceWasmParentRole::MantleBundle => "mantle-bundle",
        SpaceWasmParentRole::OctetStaticReport => "octet-static-report",
    }
}
