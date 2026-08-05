use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    AnyIdentity, ProjectionIdentity, ReceiptIdentity, SourceSpan, SourceSpanSet,
    WasmComponentParentRole, WasmComponentStaticReceipt, WasmComponentStaticRequest,
    WasmComponentValidationError, build_wasm_component_static_receipt,
};

pub const WASM_COMPONENT_VALENCE_PROJECTION_SCHEMA: &str =
    "kamacite.wasm-component-valence-projection.v1";

const COMPONENT_DESCRIPTOR_ROLE: &str = "component-adapter-descriptor";
const COMPONENT_RECEIPT_ROLE: &str = "component-static-denial-receipt";
const COMPONENT_PARENT_RELATION: &str = "parent-of";
const COMPONENT_ABSENT_SPAN_REASON: &str =
    "identity has no byte-local source span in this projection";
const COMPONENT_COUPLING_BOUNDARY: &str = "Valence records WIT component static evidence without importing Kamacite, Mantle, Aspen, Wasmtime, or Valence trust";

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WasmComponentValenceVerificationClass {
    RecordedOnly,
    Boundary,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentValenceArtifact {
    pub role: String,
    pub identity: AnyIdentity,
    pub verification_class: WasmComponentValenceVerificationClass,
    pub spans: SourceSpanSet,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentValenceEdge {
    pub parent_role: String,
    pub parent_identity: AnyIdentity,
    pub child_role: String,
    pub child_identity: AnyIdentity,
    pub relation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentValenceProjectionBody {
    pub schema_version: String,
    pub subject_receipt_identity: ReceiptIdentity,
    pub artifacts: Vec<WasmComponentValenceArtifact>,
    pub parent_edges: Vec<WasmComponentValenceEdge>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
    pub coupling_boundary: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentValenceProjection {
    pub body: WasmComponentValenceProjectionBody,
    pub projection_identity: ProjectionIdentity,
}

#[derive(Debug)]
pub enum WasmComponentValenceProjectionError {
    StaticValidation(WasmComponentValidationError),
    Encoding(String),
    SizeOverflow { artifact: &'static str },
}

impl Display for WasmComponentValenceProjectionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaticValidation(error) => Display::fmt(error, formatter),
            Self::Encoding(error) => {
                write!(formatter, "WIT component Valence encoding failed: {error}")
            }
            Self::SizeOverflow { artifact } => {
                write!(formatter, "WIT component span size overflow for {artifact}")
            }
        }
    }
}

impl std::error::Error for WasmComponentValenceProjectionError {}

impl From<WasmComponentValidationError> for WasmComponentValenceProjectionError {
    fn from(error: WasmComponentValidationError) -> Self {
        Self::StaticValidation(error)
    }
}

// r[impl kamacite.wasm_adapter.valence_export]
pub fn project_wasm_component_static_to_valence(
    request: &WasmComponentStaticRequest<'_>,
) -> Result<WasmComponentValenceProjection, WasmComponentValenceProjectionError> {
    let receipt = build_wasm_component_static_receipt(request)?;
    let descriptor_bytes = serde_json::to_vec(request.descriptor)
        .map_err(|error| WasmComponentValenceProjectionError::Encoding(error.to_string()))?;
    let inspection_bytes = request
        .inspection
        .map(serde_json::to_vec)
        .transpose()
        .map_err(|error| WasmComponentValenceProjectionError::Encoding(error.to_string()))?;
    let body = WasmComponentValenceProjectionBody {
        schema_version: WASM_COMPONENT_VALENCE_PROJECTION_SCHEMA.to_string(),
        subject_receipt_identity: receipt.receipt_identity.clone(),
        artifacts: artifacts(
            request,
            &receipt,
            &descriptor_bytes,
            inspection_bytes.as_deref(),
        )?,
        parent_edges: parent_edges(request, &receipt)?,
        checked_laws: receipt.body.checked_laws.clone(),
        non_claims: receipt.body.non_claims.clone(),
        coupling_boundary: COMPONENT_COUPLING_BOUNDARY.to_string(),
    };
    let projection_bytes = serde_json::to_vec(&body)
        .map_err(|error| WasmComponentValenceProjectionError::Encoding(error.to_string()))?;
    Ok(WasmComponentValenceProjection {
        body,
        projection_identity: ProjectionIdentity::compute(&projection_bytes),
    })
}

fn artifacts(
    request: &WasmComponentStaticRequest<'_>,
    receipt: &WasmComponentStaticReceipt,
    descriptor_bytes: &[u8],
    inspection_bytes: Option<&[u8]>,
) -> Result<Vec<WasmComponentValenceArtifact>, WasmComponentValenceProjectionError> {
    let mut artifacts = vec![
        artifact_with_span(
            "component-adapter-profile",
            AnyIdentity::Adapter(receipt.body.profile_identity.clone()),
            WasmComponentValenceVerificationClass::Boundary,
            "adapters/wasm-component/generated/profile.json",
            request.profile_bytes.len(),
        )?,
        artifact_with_span(
            "molten-runtime-profile",
            AnyIdentity::Evaluator(receipt.body.runtime_profile_identity.clone()),
            WasmComponentValenceVerificationClass::RecordedOnly,
            "adapters/wasm-component/upstream/molten-profile.json",
            request.runtime_profile_bytes.len(),
        )?,
        artifact_with_span(
            "wit-source",
            AnyIdentity::Source(receipt.body.wit_identity.clone()),
            WasmComponentValenceVerificationClass::RecordedOnly,
            "adapters/wasm-component/upstream/runtime.wit",
            request.wit_bytes.len(),
        )?,
        artifact_with_span(
            COMPONENT_DESCRIPTOR_ROLE,
            AnyIdentity::Adapter(receipt.body.descriptor_identity.clone()),
            WasmComponentValenceVerificationClass::Boundary,
            "component-descriptor-json",
            descriptor_bytes.len(),
        )?,
        artifact_with_span(
            "canonical-input",
            AnyIdentity::CanonicalValue(receipt.body.canonical_input_identity.clone()),
            WasmComponentValenceVerificationClass::Boundary,
            "canonical-input",
            request.canonical_input_bytes.len(),
        )?,
        artifact_without_span(
            "input-schema",
            AnyIdentity::admit(request.descriptor.input_schema.identity.clone()).map_err(
                |error| WasmComponentValenceProjectionError::Encoding(error.to_string()),
            )?,
            WasmComponentValenceVerificationClass::Boundary,
        ),
        artifact_without_span(
            "output-schema",
            AnyIdentity::admit(request.descriptor.output_schema.identity.clone()).map_err(
                |error| WasmComponentValenceProjectionError::Encoding(error.to_string()),
            )?,
            WasmComponentValenceVerificationClass::Boundary,
        ),
        artifact_without_span(
            COMPONENT_RECEIPT_ROLE,
            AnyIdentity::Receipt(receipt.receipt_identity.clone()),
            WasmComponentValenceVerificationClass::Boundary,
        ),
    ];
    if let (Some(identity), Some(bytes)) =
        (&receipt.body.component_identity, request.component_bytes)
    {
        artifacts.push(artifact_with_span(
            "portable-component",
            AnyIdentity::TargetArtifact(identity.clone()),
            WasmComponentValenceVerificationClass::RecordedOnly,
            "component-bytes",
            bytes.len(),
        )?);
    }
    if let (Some(identity), Some(bytes)) = (&receipt.body.inspection_identity, inspection_bytes) {
        artifacts.push(artifact_with_span(
            "component-inspection",
            AnyIdentity::Adapter(identity.clone()),
            WasmComponentValenceVerificationClass::Boundary,
            "component-inspection-json",
            bytes.len(),
        )?);
    }
    if let (Some(identity), Some(bytes)) = (
        &receipt.body.materialization_bundle_identity,
        request.materialization_manifest_bytes,
    ) {
        artifacts.push(artifact_with_span(
            "mantle-materialization-manifest",
            AnyIdentity::DependencyClosure(identity.clone()),
            WasmComponentValenceVerificationClass::RecordedOnly,
            "materialization-manifest-json",
            bytes.len(),
        )?);
    }
    for binding in &request.descriptor.imports {
        artifacts.push(artifact_without_span(
            &format!("capability:{}", binding.import),
            AnyIdentity::admit(binding.capability_identity.clone()).map_err(|error| {
                WasmComponentValenceProjectionError::Encoding(error.to_string())
            })?,
            WasmComponentValenceVerificationClass::Boundary,
        ));
        artifacts.push(artifact_without_span(
            &format!("effect-operation:{}", binding.import),
            AnyIdentity::admit(binding.effect_operation_identity.clone()).map_err(|error| {
                WasmComponentValenceProjectionError::Encoding(error.to_string())
            })?,
            WasmComponentValenceVerificationClass::Boundary,
        ));
        artifacts.push(artifact_without_span(
            &format!("authority:{}", binding.import),
            AnyIdentity::admit(binding.authority_identity.clone()).map_err(|error| {
                WasmComponentValenceProjectionError::Encoding(error.to_string())
            })?,
            WasmComponentValenceVerificationClass::Boundary,
        ));
        artifacts.push(artifact_without_span(
            &format!("resource:{}", binding.import),
            AnyIdentity::admit(binding.resource_identity.clone()).map_err(|error| {
                WasmComponentValenceProjectionError::Encoding(error.to_string())
            })?,
            WasmComponentValenceVerificationClass::Boundary,
        ));
    }
    artifacts.sort_by(|left, right| left.role.cmp(&right.role));
    Ok(artifacts)
}

fn parent_edges(
    request: &WasmComponentStaticRequest<'_>,
    receipt: &WasmComponentStaticReceipt,
) -> Result<Vec<WasmComponentValenceEdge>, WasmComponentValenceProjectionError> {
    let mut edges = request
        .descriptor
        .parent_edges
        .iter()
        .map(|edge| {
            AnyIdentity::admit(edge.identity.clone()).map(|parent| WasmComponentValenceEdge {
                parent_role: parent_role_label(&edge.role).to_string(),
                parent_identity: parent,
                child_role: COMPONENT_DESCRIPTOR_ROLE.to_string(),
                child_identity: AnyIdentity::Adapter(receipt.body.descriptor_identity.clone()),
                relation: COMPONENT_PARENT_RELATION.to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| WasmComponentValenceProjectionError::Encoding(error.to_string()))?;
    edges.push(WasmComponentValenceEdge {
        parent_role: "component-adapter-profile".to_string(),
        parent_identity: AnyIdentity::Adapter(receipt.body.profile_identity.clone()),
        child_role: COMPONENT_DESCRIPTOR_ROLE.to_string(),
        child_identity: AnyIdentity::Adapter(receipt.body.descriptor_identity.clone()),
        relation: COMPONENT_PARENT_RELATION.to_string(),
    });
    edges.push(WasmComponentValenceEdge {
        parent_role: "canonical-input".to_string(),
        parent_identity: AnyIdentity::CanonicalValue(receipt.body.canonical_input_identity.clone()),
        child_role: COMPONENT_RECEIPT_ROLE.to_string(),
        child_identity: AnyIdentity::Receipt(receipt.receipt_identity.clone()),
        relation: COMPONENT_PARENT_RELATION.to_string(),
    });
    edges.push(WasmComponentValenceEdge {
        parent_role: COMPONENT_DESCRIPTOR_ROLE.to_string(),
        parent_identity: AnyIdentity::Adapter(receipt.body.descriptor_identity.clone()),
        child_role: COMPONENT_RECEIPT_ROLE.to_string(),
        child_identity: AnyIdentity::Receipt(receipt.receipt_identity.clone()),
        relation: COMPONENT_PARENT_RELATION.to_string(),
    });
    edges.sort();
    Ok(edges)
}

fn artifact_with_span(
    role: &str,
    identity: AnyIdentity,
    verification_class: WasmComponentValenceVerificationClass,
    artifact_key: &str,
    byte_len: usize,
) -> Result<WasmComponentValenceArtifact, WasmComponentValenceProjectionError> {
    let end_byte =
        u64::try_from(byte_len).map_err(|_| WasmComponentValenceProjectionError::SizeOverflow {
            artifact: "byte-bearing artifact",
        })?;
    Ok(WasmComponentValenceArtifact {
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
    verification_class: WasmComponentValenceVerificationClass,
) -> WasmComponentValenceArtifact {
    WasmComponentValenceArtifact {
        role: role.to_string(),
        identity,
        verification_class,
        spans: SourceSpanSet::Absent {
            reason: COMPONENT_ABSENT_SPAN_REASON.to_string(),
        },
    }
}

fn parent_role_label(role: &WasmComponentParentRole) -> &'static str {
    match role {
        WasmComponentParentRole::Component => "portable-component",
        WasmComponentParentRole::InputSchema => "input-schema",
        WasmComponentParentRole::MaterializationBundle => "mantle-materialization-manifest",
        WasmComponentParentRole::OutputSchema => "output-schema",
        WasmComponentParentRole::ResourceProfile => "resource-profile",
        WasmComponentParentRole::RuntimeProfile => "molten-runtime-profile",
        WasmComponentParentRole::ToolchainCohort => "toolchain-cohort",
        WasmComponentParentRole::WitSource => "wit-source",
    }
}
