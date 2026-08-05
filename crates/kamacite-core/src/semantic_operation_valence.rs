use crate::{
    AnyIdentity, EffectOperationIdentity, IdentityDomain, ProjectionIdentity,
    SemanticOperationError, WireIdentity,
};
use serde::{Deserialize, Serialize};

pub const SEMANTIC_OPERATION_VALENCE_SCHEMA: &str = "kamacite.semantic-effect-operation-valence.v1";
const PARENT_RELATION: &str = "supports";

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticOperationValenceRole {
    SemanticOperationDescriptor,
    DefaultBehavior,
    CompatibilityArtifact,
    HandlerDescriptor,
    RuntimeReceipt,
    SupportingEvidence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationValenceArtifact {
    pub role: SemanticOperationValenceRole,
    pub identity: AnyIdentity,
    pub checked_claim: String,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationValenceEdge {
    pub parent_identity: AnyIdentity,
    pub child_identity: AnyIdentity,
    pub relation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationValenceProjectionBody {
    pub schema_version: String,
    pub subject_identity: EffectOperationIdentity,
    pub artifacts: Vec<SemanticOperationValenceArtifact>,
    pub parent_edges: Vec<SemanticOperationValenceEdge>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationValenceProjection {
    pub body: SemanticOperationValenceProjectionBody,
    pub projection_identity: ProjectionIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticOperationValenceInput {
    pub descriptor_identity: WireIdentity,
    pub default_behavior_identity: Option<WireIdentity>,
    pub compatibility_identities: Vec<WireIdentity>,
    pub handler_identities: Vec<WireIdentity>,
    pub runtime_receipt_identities: Vec<WireIdentity>,
    pub supporting_evidence_identities: Vec<WireIdentity>,
}

pub fn project_semantic_operation_to_valence(
    input: &SemanticOperationValenceInput,
) -> Result<SemanticOperationValenceProjection, SemanticOperationError> {
    require_domain(
        &input.descriptor_identity,
        IdentityDomain::EffectOperation,
        "descriptor_identity",
    )?;
    let descriptor = EffectOperationIdentity::from_wire(input.descriptor_identity.clone())
        .map_err(|_| SemanticOperationError::InvalidIdentity {
            field: "descriptor_identity",
        })?;
    let mut artifacts = vec![artifact(
        SemanticOperationValenceRole::SemanticOperationDescriptor,
        AnyIdentity::EffectOperation(descriptor.clone()),
        "canonical semantic operation identity",
        &["not behavior correctness", "not host authority"],
    )];
    append_typed_artifacts(
        &mut artifacts,
        input.default_behavior_identity.iter(),
        IdentityDomain::CanonicalValue,
        SemanticOperationValenceRole::DefaultBehavior,
        "content identity of declared default behavior",
    )?;
    append_typed_artifacts(
        &mut artifacts,
        input.compatibility_identities.iter(),
        IdentityDomain::EvidenceChain,
        SemanticOperationValenceRole::CompatibilityArtifact,
        "directional bounded compatibility claim",
    )?;
    append_typed_artifacts(
        &mut artifacts,
        input.handler_identities.iter(),
        IdentityDomain::Adapter,
        SemanticOperationValenceRole::HandlerDescriptor,
        "handler declaration for exact operation identity",
    )?;
    append_typed_artifacts(
        &mut artifacts,
        input.runtime_receipt_identities.iter(),
        IdentityDomain::Receipt,
        SemanticOperationValenceRole::RuntimeReceipt,
        "runtime observation bound to exact operation identity",
    )?;
    for identity in &input.supporting_evidence_identities {
        identity
            .validate()
            .map_err(|_| SemanticOperationError::InvalidIdentity {
                field: "supporting_evidence_identities",
            })?;
        artifacts.push(artifact(
            SemanticOperationValenceRole::SupportingEvidence,
            AnyIdentity::admit(identity.clone()).map_err(|_| {
                SemanticOperationError::InvalidIdentity {
                    field: "supporting_evidence_identities",
                }
            })?,
            "supporting source or proof evidence",
            &["does not replace the Kamacite descriptor"],
        ));
    }
    artifacts.sort_by(|left, right| {
        left.role
            .cmp(&right.role)
            .then_with(|| left.identity.cmp(&right.identity))
    });
    artifacts.dedup_by(|left, right| left.role == right.role && left.identity == right.identity);
    let descriptor_any = AnyIdentity::EffectOperation(descriptor.clone());
    let parent_edges = artifacts
        .iter()
        .filter(|artifact| artifact.identity != descriptor_any)
        .map(|artifact| SemanticOperationValenceEdge {
            parent_identity: descriptor_any.clone(),
            child_identity: artifact.identity.clone(),
            relation: PARENT_RELATION.to_string(),
        })
        .collect();
    let body = SemanticOperationValenceProjectionBody {
        schema_version: SEMANTIC_OPERATION_VALENCE_SCHEMA.to_string(),
        subject_identity: descriptor,
        artifacts,
        parent_edges,
        non_claims: vec![
            "not semantic equivalence".to_string(),
            "not handler correctness".to_string(),
            "not runtime authority".to_string(),
            "not host authorization".to_string(),
            "not release eligibility".to_string(),
        ],
    };
    let bytes = serde_json::to_vec(&body)
        .map_err(|error| SemanticOperationError::Canonicalization(error.to_string()))?;
    Ok(SemanticOperationValenceProjection {
        body,
        projection_identity: ProjectionIdentity::compute(&bytes),
    })
}

fn append_typed_artifacts<'a>(
    artifacts: &mut Vec<SemanticOperationValenceArtifact>,
    identities: impl Iterator<Item = &'a WireIdentity>,
    expected: IdentityDomain,
    role: SemanticOperationValenceRole,
    checked_claim: &str,
) -> Result<(), SemanticOperationError> {
    for identity in identities {
        require_domain(identity, expected.clone(), "valence_role_identity")?;
        artifacts.push(artifact(
            role.clone(),
            AnyIdentity::admit(identity.clone()).map_err(|_| {
                SemanticOperationError::InvalidIdentity {
                    field: "valence_role_identity",
                }
            })?,
            checked_claim,
            &["not correctness or authority"],
        ));
    }
    Ok(())
}

fn artifact(
    role: SemanticOperationValenceRole,
    identity: AnyIdentity,
    checked_claim: &str,
    non_claims: &[&str],
) -> SemanticOperationValenceArtifact {
    SemanticOperationValenceArtifact {
        role,
        identity,
        checked_claim: checked_claim.to_string(),
        non_claims: non_claims
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    }
}

fn require_domain(
    identity: &WireIdentity,
    expected: IdentityDomain,
    field: &'static str,
) -> Result<(), SemanticOperationError> {
    identity
        .validate_domain(expected.clone())
        .map_err(|_| SemanticOperationError::WrongIdentityDomain { field, expected })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_identity;

    #[test]
    fn projection_preserves_roles_and_non_claims() {
        let input = SemanticOperationValenceInput {
            descriptor_identity: compute_identity(IdentityDomain::EffectOperation, b"operation"),
            default_behavior_identity: Some(compute_identity(
                IdentityDomain::CanonicalValue,
                b"behavior",
            )),
            compatibility_identities: vec![compute_identity(
                IdentityDomain::EvidenceChain,
                b"compatibility",
            )],
            handler_identities: vec![compute_identity(IdentityDomain::Adapter, b"handler")],
            runtime_receipt_identities: vec![compute_identity(IdentityDomain::Receipt, b"runtime")],
            supporting_evidence_identities: vec![compute_identity(
                IdentityDomain::Source,
                b"source",
            )],
        };
        let projection = project_semantic_operation_to_valence(&input);
        assert!(projection.is_ok());
        if let Ok(projection) = projection {
            assert_eq!(projection.body.artifacts.len(), 6);
            assert_eq!(projection.body.parent_edges.len(), 5);
            assert!(
                projection
                    .body
                    .non_claims
                    .contains(&"not host authorization".to_string())
            );
        }
    }

    #[test]
    fn projection_rejects_role_substitution() {
        let input = SemanticOperationValenceInput {
            descriptor_identity: compute_identity(IdentityDomain::EffectOperation, b"operation"),
            default_behavior_identity: None,
            compatibility_identities: Vec::new(),
            handler_identities: vec![compute_identity(IdentityDomain::Receipt, b"wrong-role")],
            runtime_receipt_identities: Vec::new(),
            supporting_evidence_identities: Vec::new(),
        };
        assert!(matches!(
            project_semantic_operation_to_valence(&input),
            Err(SemanticOperationError::WrongIdentityDomain {
                field: "valence_role_identity",
                ..
            })
        ));
    }
}
