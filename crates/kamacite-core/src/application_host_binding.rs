//! Application-host binding artifacts: canonical composition records that
//! bind one executable application entrypoint to exactly one host profile.
//!
//! The binding is portable interchange data. It proves canonical identity,
//! required-field validity, one selected host profile, closed declared
//! effects, and exact linked cohort identities. It never grants capability,
//! authorization, sandbox, resource-enforcement, or runtime authority.

use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{
    AdapterIdentity, AnyIdentity, ApplicationHostBindingIdentity, EffectFamilyIdentity,
    EffectOperationIdentity, EvaluatorIdentity, EvidenceChainIdentity, Identity, IdentityDomain,
    IdentityDomainMarker, IdentityError, ProjectionIdentity, ReceiptIdentity,
    TargetArtifactIdentity, WireIdentity,
};
use crate::preserves_codec::{PreservesLimits, PreservesSchema, encode_typed_preserves};
use crate::receipt::{ReceiptError, canonical_json_bytes};
use serde::{Deserialize, Serialize};

pub const APPLICATION_HOST_BINDING_SCHEMA_VERSION: &str = "kamacite.application-host-binding.v1";
pub const APPLICATION_HOST_BINDING_RECEIPT_SCHEMA: &str =
    "kamacite.application-host-binding-receipt.v1";
pub const APPLICATION_HOST_BINDING_ROOT_LABEL: &str = "kamacite-application-host-binding-v1";
pub const APPLICATION_HOST_BINDING_RECEIPT_ROOT_LABEL: &str =
    "kamacite-application-host-binding-receipt-v1";
pub const APPLICATION_HOST_BINDING_VALENCE_SCHEMA: &str =
    "kamacite.application-host-binding-valence-projection.v1";

const REQUIREMENT_ARTIFACT: &str = "r[kamacite.application_host_binding.artifact]";
const REQUIREMENT_HOST_SELECTION: &str = "r[kamacite.application_host_binding.host_selection]";
const REQUIREMENT_EFFECTS: &str = "r[kamacite.application_host_binding.effects]";
const REQUIREMENT_COHORTS: &str = "r[kamacite.application_host_binding.cohorts]";
const REQUIREMENT_BOUNDARY: &str = "r[kamacite.application_host_binding.boundary]";
const REQUIREMENT_RECEIPT: &str = "r[kamacite.application_host_binding.receipt]";

const VERDICT_ACCEPTED: &str = "accepted";

const NON_CLAIM_HOST_CAPABILITY: &str = "not host capability or authorization";
const NON_CLAIM_SANDBOX: &str = "not sandbox admission";
const NON_CLAIM_RESOURCE_ENFORCEMENT: &str = "not resource enforcement";
const NON_CLAIM_RUNTIME: &str = "not runtime correctness";
const NON_CLAIM_APPLICATION: &str = "not application correctness";
const NON_CLAIM_RELEASE: &str = "not release eligibility";

const REQUIRED_NON_CLAIMS: [&str; 6] = [
    NON_CLAIM_HOST_CAPABILITY,
    NON_CLAIM_SANDBOX,
    NON_CLAIM_RESOURCE_ENFORCEMENT,
    NON_CLAIM_RUNTIME,
    NON_CLAIM_APPLICATION,
    NON_CLAIM_RELEASE,
];

const FORBIDDEN_CLAIMS: [&str; 6] = [
    "semantic equivalence",
    "authorization",
    "release eligibility",
    "runtime correctness",
    "application correctness",
    "sandbox",
];

/// Review metadata that never enters canonical identity material.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationHostBindingMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
}

/// Untrusted application-host binding input. Wire identities keep
/// wrong-domain values expressible so admission can reject them with
/// deterministic diagnostics.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingInput {
    pub schema_version: String,
    pub application_identity: WireIdentity,
    pub entrypoint_identity: WireIdentity,
    pub input_schema_identity: WireIdentity,
    pub output_schema_identity: WireIdentity,
    pub effect_summary_identity: WireIdentity,
    pub effect_summary_closed: bool,
    pub effect_operation_identities: Vec<WireIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_profile_identity: Option<WireIdentity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternative_host_profile_identities: Vec<WireIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_host_profile_identity: Option<WireIdentity>,
    pub adapter_cohort_identity: WireIdentity,
    pub resource_profile_identity: WireIdentity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_identity: Option<WireIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_identity: Option<WireIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_adapter_cohort_identity: Option<WireIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_resource_profile_identity: Option<WireIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_host_profile_identity: Option<WireIdentity>,
    #[serde(default)]
    pub metadata: ApplicationHostBindingMetadata,
    #[serde(default)]
    pub semantic_claims: Vec<String>,
    pub non_claims: Vec<String>,
}

/// Validated binding with exact typed identities for every linked role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedApplicationHostBinding {
    pub application_identity: TargetArtifactIdentity,
    pub entrypoint_identity: TargetArtifactIdentity,
    pub input_schema_identity: AdapterIdentity,
    pub output_schema_identity: AdapterIdentity,
    pub effect_summary_identity: EffectFamilyIdentity,
    pub effect_operation_identities: Vec<EffectOperationIdentity>,
    pub host_profile_identity: EvaluatorIdentity,
    pub adapter_cohort_identity: AdapterIdentity,
    pub resource_profile_identity: AdapterIdentity,
    pub policy_identity: Option<AdapterIdentity>,
    pub compatibility_identity: Option<EvidenceChainIdentity>,
    pub binding_identity: ApplicationHostBindingIdentity,
    pub canonical_bytes: Vec<u8>,
}

#[derive(Serialize)]
#[serde(rename = "kamacite-application-host-binding-v1")]
struct CanonicalApplicationHostBinding {
    schema_version: String,
    application_identity: WireIdentity,
    entrypoint_identity: WireIdentity,
    input_schema_identity: WireIdentity,
    output_schema_identity: WireIdentity,
    effect_summary_identity: WireIdentity,
    effect_operation_identities: Vec<WireIdentity>,
    host_profile_identity: WireIdentity,
    adapter_cohort_identity: WireIdentity,
    resource_profile_identity: WireIdentity,
    policy_identity: Option<WireIdentity>,
    compatibility_identity: Option<WireIdentity>,
    non_claims: Vec<String>,
}

/// Deterministic validation receipt for one binding admission.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationHostBindingReceiptBody {
    pub schema_version: String,
    pub verdict: String,
    pub binding_identity: ApplicationHostBindingIdentity,
    pub application_identity: TargetArtifactIdentity,
    pub entrypoint_identity: TargetArtifactIdentity,
    pub input_schema_identity: AdapterIdentity,
    pub output_schema_identity: AdapterIdentity,
    pub effect_summary_identity: EffectFamilyIdentity,
    pub effect_operation_identities: Vec<EffectOperationIdentity>,
    pub host_profile_identity: EvaluatorIdentity,
    pub adapter_cohort_identity: AdapterIdentity,
    pub resource_profile_identity: AdapterIdentity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_identity: Option<AdapterIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_identity: Option<EvidenceChainIdentity>,
    pub checked_laws: Vec<String>,
    pub diagnostics: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationHostBindingReceipt {
    pub body: ApplicationHostBindingReceiptBody,
    pub receipt_identity: ReceiptIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationHostBindingValidation {
    pub passed: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub binding: Option<ValidatedApplicationHostBinding>,
    pub receipt: Option<ApplicationHostBindingReceipt>,
}

/// Valence-friendly projection artifact with a distinct role per identity.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ApplicationHostBindingValenceArtifact {
    pub role: String,
    pub identity: AnyIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingValenceProjectionBody {
    pub schema_version: String,
    pub binding_identity: ApplicationHostBindingIdentity,
    pub artifacts: Vec<ApplicationHostBindingValenceArtifact>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingValenceProjection {
    pub body: ApplicationHostBindingValenceProjectionBody,
    pub projection_identity: ProjectionIdentity,
}

fn sorted_unique(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn domain_diagnostic(field: &str, requirement: &str, error: &IdentityError) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::IdentityDomainMismatch,
        requirement,
        field,
        format!("application-host binding identity admission failed: {error}"),
    )
}

fn validate_domain(
    identity: &WireIdentity,
    expected: IdentityDomain,
    field: &str,
    requirement: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if identity.validate_domain(expected.clone()).is_err() {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                requirement,
                field,
                "application-host binding identity uses the wrong domain",
            )
            .with_detail("expected", expected.to_string())
            .with_detail("actual", identity.domain.to_string()),
        );
    }
}

fn admit_field<D: IdentityDomainMarker>(
    wire: &WireIdentity,
    field: &str,
    requirement: &str,
) -> Result<Identity<D>, Diagnostic> {
    Identity::from_wire(wire.clone()).map_err(|error| domain_diagnostic(field, requirement, &error))
}

fn validate_host_selection(input: &ApplicationHostBindingInput, diagnostics: &mut Vec<Diagnostic>) {
    if input.host_profile_identity.is_none() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_HOST_SELECTION,
            "host_profile_identity",
            "executable binding requires exactly one host-profile identity",
        ));
    }
    if !input.alternative_host_profile_identities.is_empty()
        || input.fallback_host_profile_identity.is_some()
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::AmbiguousClaim,
            REQUIREMENT_HOST_SELECTION,
            "host_profile_identity",
            "executable binding must not carry alternative or fallback host selections",
        ));
    }
    if let Some(host) = &input.host_profile_identity {
        validate_domain(
            host,
            IdentityDomain::Evaluator,
            "host_profile_identity",
            REQUIREMENT_HOST_SELECTION,
            diagnostics,
        );
    }
}

fn validate_effect_closure(input: &ApplicationHostBindingInput, diagnostics: &mut Vec<Diagnostic>) {
    validate_domain(
        &input.effect_summary_identity,
        IdentityDomain::AlgebraicEffect,
        "effect_summary_identity",
        REQUIREMENT_EFFECTS,
        diagnostics,
    );
    if !input.effect_summary_closed {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::OpenRowNotPure,
            REQUIREMENT_EFFECTS,
            "effect_summary_closed",
            "executable binding requires a closed effect summary",
        ));
    }
    for operation in &input.effect_operation_identities {
        validate_domain(
            operation,
            IdentityDomain::EffectOperation,
            "effect_operation_identities",
            REQUIREMENT_EFFECTS,
            diagnostics,
        );
    }
}

fn validate_expected_cohort(
    expected: Option<&WireIdentity>,
    declared: Option<&WireIdentity>,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let (Some(expected), Some(declared)) = (expected, declared)
        && expected != declared
    {
        push_stale_cohort_diagnostic(expected, declared, field, diagnostics);
    }
}

fn push_stale_cohort_diagnostic(
    expected: &WireIdentity,
    declared: &WireIdentity,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    diagnostics.push(
        Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_COHORTS,
            field,
            "binding cohort identity differs from the expected compatible cohort",
        )
        .with_detail("expected", expected.hex.as_str().to_string())
        .with_detail("declared", declared.hex.as_str().to_string()),
    );
}

fn validate_cohorts(input: &ApplicationHostBindingInput, diagnostics: &mut Vec<Diagnostic>) {
    validate_expected_cohort(
        input.expected_adapter_cohort_identity.as_ref(),
        Some(&input.adapter_cohort_identity),
        "adapter_cohort_identity",
        diagnostics,
    );
    validate_expected_cohort(
        input.expected_resource_profile_identity.as_ref(),
        Some(&input.resource_profile_identity),
        "resource_profile_identity",
        diagnostics,
    );
    validate_expected_cohort(
        input.expected_host_profile_identity.as_ref(),
        input.host_profile_identity.as_ref(),
        "host_profile_identity",
        diagnostics,
    );
}

fn validate_claims(input: &ApplicationHostBindingInput, diagnostics: &mut Vec<Diagnostic>) {
    for claim in &input.semantic_claims {
        let lowered = claim.to_lowercase();
        if FORBIDDEN_CLAIMS
            .iter()
            .any(|forbidden| lowered.contains(forbidden))
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_BOUNDARY,
                    "semantic_claims",
                    "binding must not claim authority, execution, correctness, or release status",
                )
                .with_detail("claim", claim.clone()),
            );
        }
    }
    for required in REQUIRED_NON_CLAIMS {
        if !input.non_claims.iter().any(|claim| claim == required) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingNonClaim,
                    REQUIREMENT_BOUNDARY,
                    "non_claims",
                    "binding lacks a required non-claim",
                )
                .with_detail("missing", required.to_string()),
            );
        }
    }
}

fn canonical_binding_bytes(input: &ApplicationHostBindingInput) -> Result<Vec<u8>, ReceiptError> {
    let Some(host_profile_identity) = input.host_profile_identity.clone() else {
        return Err(ReceiptError::Serialize(
            "application-host binding canonicalization requires one host profile".to_string(),
        ));
    };
    let canonical = CanonicalApplicationHostBinding {
        schema_version: input.schema_version.clone(),
        application_identity: input.application_identity.clone(),
        entrypoint_identity: input.entrypoint_identity.clone(),
        input_schema_identity: input.input_schema_identity.clone(),
        output_schema_identity: input.output_schema_identity.clone(),
        effect_summary_identity: input.effect_summary_identity.clone(),
        effect_operation_identities: sorted_wire(input.effect_operation_identities.clone()),
        host_profile_identity,
        adapter_cohort_identity: input.adapter_cohort_identity.clone(),
        resource_profile_identity: input.resource_profile_identity.clone(),
        policy_identity: input.policy_identity.clone(),
        compatibility_identity: input.compatibility_identity.clone(),
        non_claims: sorted_unique(input.non_claims.clone()),
    };
    encode_typed_preserves(
        &canonical,
        PreservesSchema {
            schema_version: APPLICATION_HOST_BINDING_SCHEMA_VERSION,
            root_label: APPLICATION_HOST_BINDING_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ReceiptError::Serialize(error.to_string()))
}

fn sorted_wire(mut identities: Vec<WireIdentity>) -> Vec<WireIdentity> {
    identities.sort();
    identities.dedup();
    identities
}

// r[impl kamacite.application_host_binding.artifact]
// r[impl kamacite.application_host_binding.host_selection]
// r[impl kamacite.application_host_binding.effects]
// r[impl kamacite.application_host_binding.cohorts]
// r[impl kamacite.application_host_binding.boundary]
pub fn validate_application_host_binding(
    input: &ApplicationHostBindingInput,
) -> ApplicationHostBindingValidation {
    let mut diagnostics = Vec::new();
    if input.schema_version != APPLICATION_HOST_BINDING_SCHEMA_VERSION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_ARTIFACT,
                "schema_version",
                "application-host binding schema version is unsupported",
            )
            .with_detail("actual", input.schema_version.clone())
            .with_detail(
                "expected",
                APPLICATION_HOST_BINDING_SCHEMA_VERSION.to_string(),
            ),
        );
    }
    validate_domain(
        &input.application_identity,
        IdentityDomain::TargetArtifact,
        "application_identity",
        REQUIREMENT_ARTIFACT,
        &mut diagnostics,
    );
    validate_domain(
        &input.entrypoint_identity,
        IdentityDomain::TargetArtifact,
        "entrypoint_identity",
        REQUIREMENT_ARTIFACT,
        &mut diagnostics,
    );
    validate_domain(
        &input.input_schema_identity,
        IdentityDomain::Adapter,
        "input_schema_identity",
        REQUIREMENT_ARTIFACT,
        &mut diagnostics,
    );
    validate_domain(
        &input.output_schema_identity,
        IdentityDomain::Adapter,
        "output_schema_identity",
        REQUIREMENT_ARTIFACT,
        &mut diagnostics,
    );
    validate_domain(
        &input.adapter_cohort_identity,
        IdentityDomain::Adapter,
        "adapter_cohort_identity",
        REQUIREMENT_COHORTS,
        &mut diagnostics,
    );
    validate_domain(
        &input.resource_profile_identity,
        IdentityDomain::Adapter,
        "resource_profile_identity",
        REQUIREMENT_COHORTS,
        &mut diagnostics,
    );
    if let Some(policy) = &input.policy_identity {
        validate_domain(
            policy,
            IdentityDomain::Adapter,
            "policy_identity",
            REQUIREMENT_ARTIFACT,
            &mut diagnostics,
        );
    }
    if let Some(compatibility) = &input.compatibility_identity {
        validate_domain(
            compatibility,
            IdentityDomain::EvidenceChain,
            "compatibility_identity",
            REQUIREMENT_ARTIFACT,
            &mut diagnostics,
        );
    }
    validate_host_selection(input, &mut diagnostics);
    validate_effect_closure(input, &mut diagnostics);
    validate_cohorts(input, &mut diagnostics);
    validate_claims(input, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    if !diagnostics.is_empty() {
        return ApplicationHostBindingValidation {
            passed: false,
            diagnostics,
            binding: None,
            receipt: None,
        };
    }
    match build_binding(input) {
        Ok((binding, receipt)) => ApplicationHostBindingValidation {
            passed: true,
            diagnostics,
            binding: Some(binding),
            receipt: Some(receipt),
        },
        Err(diagnostic) => ApplicationHostBindingValidation {
            passed: false,
            diagnostics: vec![diagnostic],
            binding: None,
            receipt: None,
        },
    }
}

fn build_binding(
    input: &ApplicationHostBindingInput,
) -> Result<
    (
        ValidatedApplicationHostBinding,
        ApplicationHostBindingReceipt,
    ),
    Diagnostic,
> {
    let canonical_bytes = canonical_binding_bytes(input).map_err(|error| {
        Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_ARTIFACT,
            "canonical_bytes",
            format!("application-host binding canonicalization failed: {error}"),
        )
    })?;
    let Some(host_wire) = input.host_profile_identity.clone() else {
        return Err(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_HOST_SELECTION,
            "host_profile_identity",
            "executable binding requires exactly one host-profile identity",
        ));
    };
    let binding = ValidatedApplicationHostBinding {
        application_identity: admit_field(
            &input.application_identity,
            "application_identity",
            REQUIREMENT_ARTIFACT,
        )?,
        entrypoint_identity: admit_field(
            &input.entrypoint_identity,
            "entrypoint_identity",
            REQUIREMENT_ARTIFACT,
        )?,
        input_schema_identity: admit_field(
            &input.input_schema_identity,
            "input_schema_identity",
            REQUIREMENT_ARTIFACT,
        )?,
        output_schema_identity: admit_field(
            &input.output_schema_identity,
            "output_schema_identity",
            REQUIREMENT_ARTIFACT,
        )?,
        effect_summary_identity: admit_field(
            &input.effect_summary_identity,
            "effect_summary_identity",
            REQUIREMENT_EFFECTS,
        )?,
        effect_operation_identities: input
            .effect_operation_identities
            .iter()
            .map(|operation| {
                admit_field(
                    operation,
                    "effect_operation_identities",
                    REQUIREMENT_EFFECTS,
                )
            })
            .collect::<Result<Vec<_>, _>>()?,
        host_profile_identity: admit_field(
            &host_wire,
            "host_profile_identity",
            REQUIREMENT_HOST_SELECTION,
        )?,
        adapter_cohort_identity: admit_field(
            &input.adapter_cohort_identity,
            "adapter_cohort_identity",
            REQUIREMENT_COHORTS,
        )?,
        resource_profile_identity: admit_field(
            &input.resource_profile_identity,
            "resource_profile_identity",
            REQUIREMENT_COHORTS,
        )?,
        policy_identity: input
            .policy_identity
            .as_ref()
            .map(|policy| admit_field(policy, "policy_identity", REQUIREMENT_ARTIFACT))
            .transpose()?,
        compatibility_identity: input
            .compatibility_identity
            .as_ref()
            .map(|compatibility| {
                admit_field(
                    compatibility,
                    "compatibility_identity",
                    REQUIREMENT_ARTIFACT,
                )
            })
            .transpose()?,
        binding_identity: ApplicationHostBindingIdentity::compute(&canonical_bytes),
        canonical_bytes,
    };
    let body = ApplicationHostBindingReceiptBody {
        schema_version: APPLICATION_HOST_BINDING_RECEIPT_SCHEMA.to_string(),
        verdict: VERDICT_ACCEPTED.to_string(),
        binding_identity: binding.binding_identity.clone(),
        application_identity: binding.application_identity.clone(),
        entrypoint_identity: binding.entrypoint_identity.clone(),
        input_schema_identity: binding.input_schema_identity.clone(),
        output_schema_identity: binding.output_schema_identity.clone(),
        effect_summary_identity: binding.effect_summary_identity.clone(),
        effect_operation_identities: binding.effect_operation_identities.clone(),
        host_profile_identity: binding.host_profile_identity.clone(),
        adapter_cohort_identity: binding.adapter_cohort_identity.clone(),
        resource_profile_identity: binding.resource_profile_identity.clone(),
        policy_identity: binding.policy_identity.clone(),
        compatibility_identity: binding.compatibility_identity.clone(),
        checked_laws: vec![
            "canonical-identity-stable".to_string(),
            "exactly-one-host".to_string(),
            "closed-effect-summary".to_string(),
            "exact-cohort".to_string(),
            "no-authority-transfer".to_string(),
        ],
        diagnostics: Vec::new(),
        non_claims: sorted_unique(input.non_claims.clone()),
    };
    let body_bytes = canonical_json_bytes(&body).map_err(|error| {
        Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_RECEIPT,
            "receipt",
            format!("application-host binding receipt encoding failed: {error}"),
        )
    })?;
    let receipt = ApplicationHostBindingReceipt {
        body,
        receipt_identity: ReceiptIdentity::compute(&body_bytes),
    };
    Ok((binding, receipt))
}

// r[impl kamacite.application_host_binding.projection]
pub fn application_host_binding_valence_projection(
    receipt: &ApplicationHostBindingReceipt,
) -> Result<ApplicationHostBindingValenceProjection, ReceiptError> {
    let body = &receipt.body;
    let mut artifacts = vec![
        ApplicationHostBindingValenceArtifact {
            role: "binding".to_string(),
            identity: AnyIdentity::ApplicationHostBinding(body.binding_identity.clone()),
        },
        ApplicationHostBindingValenceArtifact {
            role: "application".to_string(),
            identity: AnyIdentity::TargetArtifact(body.application_identity.clone()),
        },
        ApplicationHostBindingValenceArtifact {
            role: "entrypoint".to_string(),
            identity: AnyIdentity::TargetArtifact(body.entrypoint_identity.clone()),
        },
        ApplicationHostBindingValenceArtifact {
            role: "host".to_string(),
            identity: AnyIdentity::Evaluator(body.host_profile_identity.clone()),
        },
        ApplicationHostBindingValenceArtifact {
            role: "effect".to_string(),
            identity: AnyIdentity::AlgebraicEffect(body.effect_summary_identity.clone()),
        },
        ApplicationHostBindingValenceArtifact {
            role: "adapter".to_string(),
            identity: AnyIdentity::Adapter(body.adapter_cohort_identity.clone()),
        },
        ApplicationHostBindingValenceArtifact {
            role: "resource".to_string(),
            identity: AnyIdentity::Adapter(body.resource_profile_identity.clone()),
        },
    ];
    if let Some(policy) = &body.policy_identity {
        artifacts.push(ApplicationHostBindingValenceArtifact {
            role: "policy".to_string(),
            identity: AnyIdentity::Adapter(policy.clone()),
        });
    }
    if let Some(compatibility) = &body.compatibility_identity {
        artifacts.push(ApplicationHostBindingValenceArtifact {
            role: "compatibility".to_string(),
            identity: AnyIdentity::EvidenceChain(compatibility.clone()),
        });
    }
    artifacts.sort();
    let projection_body = ApplicationHostBindingValenceProjectionBody {
        schema_version: APPLICATION_HOST_BINDING_VALENCE_SCHEMA.to_string(),
        binding_identity: body.binding_identity.clone(),
        artifacts,
        non_claims: body.non_claims.clone(),
    };
    let bytes = serde_json::to_vec(&projection_body)
        .map_err(|error| ReceiptError::Serialize(error.to_string()))?;
    Ok(ApplicationHostBindingValenceProjection {
        body: projection_body,
        projection_identity: ProjectionIdentity::compute(&bytes),
    })
}

/// Compute the exact canonical Preserves bytes for a binding input.
pub fn application_host_binding_canonical_bytes(
    input: &ApplicationHostBindingInput,
) -> Result<Vec<u8>, ReceiptError> {
    canonical_binding_bytes(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::compute_identity;

    fn wire(domain: IdentityDomain, bytes: &[u8]) -> WireIdentity {
        compute_identity(domain, bytes)
    }

    fn required_non_claims() -> Vec<String> {
        REQUIRED_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect()
    }

    fn valid_input() -> ApplicationHostBindingInput {
        ApplicationHostBindingInput {
            schema_version: APPLICATION_HOST_BINDING_SCHEMA_VERSION.to_string(),
            application_identity: wire(IdentityDomain::TargetArtifact, b"application"),
            entrypoint_identity: wire(IdentityDomain::TargetArtifact, b"entrypoint"),
            input_schema_identity: wire(IdentityDomain::Adapter, b"input-schema"),
            output_schema_identity: wire(IdentityDomain::Adapter, b"output-schema"),
            effect_summary_identity: wire(IdentityDomain::AlgebraicEffect, b"effect-summary"),
            effect_summary_closed: true,
            effect_operation_identities: vec![
                wire(IdentityDomain::EffectOperation, b"op-read"),
                wire(IdentityDomain::EffectOperation, b"op-write"),
            ],
            host_profile_identity: Some(wire(IdentityDomain::Evaluator, b"host-profile")),
            alternative_host_profile_identities: Vec::new(),
            fallback_host_profile_identity: None,
            adapter_cohort_identity: wire(IdentityDomain::Adapter, b"adapter-cohort"),
            resource_profile_identity: wire(IdentityDomain::Adapter, b"resource-profile"),
            policy_identity: None,
            compatibility_identity: None,
            expected_adapter_cohort_identity: None,
            expected_resource_profile_identity: None,
            expected_host_profile_identity: None,
            metadata: ApplicationHostBindingMetadata::default(),
            semantic_claims: Vec::new(),
            non_claims: required_non_claims(),
        }
    }

    #[test]
    fn valid_capability_free_binding_passes_with_stable_identity() {
        let first = validate_application_host_binding(&valid_input());
        let second = validate_application_host_binding(&valid_input());
        assert!(first.passed);
        assert!(first.diagnostics.is_empty());
        assert!(second.passed);
        let (Some(first_binding), Some(second_binding)) = (first.binding, second.binding) else {
            return;
        };
        assert_eq!(
            first_binding.binding_identity,
            second_binding.binding_identity
        );
        assert_eq!(
            first_binding.canonical_bytes,
            second_binding.canonical_bytes
        );
    }

    #[test]
    fn closed_effect_binding_with_policy_and_compatibility_passes() {
        let mut input = valid_input();
        input.policy_identity = Some(wire(IdentityDomain::Adapter, b"policy"));
        input.compatibility_identity = Some(wire(IdentityDomain::EvidenceChain, b"compat"));
        let result = validate_application_host_binding(&input);
        assert!(result.passed);
        assert!(result.receipt.is_some());
    }

    #[test]
    fn metadata_rename_does_not_change_binding_identity() {
        let mut renamed = valid_input();
        renamed.metadata = ApplicationHostBindingMetadata {
            display_name: Some("Read configuration".to_string()),
            aliases: vec!["read-config".to_string()],
            path: Some("src/config.rs".to_string()),
            source_span: Some("0:42".to_string()),
            documentation: Some("reads the configuration".to_string()),
        };
        let base = validate_application_host_binding(&valid_input());
        let renamed = validate_application_host_binding(&renamed);
        let (Some(base), Some(renamed)) = (base.binding, renamed.binding) else {
            return;
        };
        assert_eq!(base.binding_identity, renamed.binding_identity);
        assert_eq!(base.canonical_bytes, renamed.canonical_bytes);
    }

    #[test]
    fn valence_projection_keeps_roles_distinct() {
        let result = validate_application_host_binding(&valid_input());
        assert!(result.passed);
        let Some(receipt) = result.receipt else {
            return;
        };
        let projection = application_host_binding_valence_projection(&receipt);
        let Ok(projection) = projection else {
            return;
        };
        let roles: Vec<&str> = projection
            .body
            .artifacts
            .iter()
            .map(|artifact| artifact.role.as_str())
            .collect();
        assert!(roles.contains(&"binding"));
        assert!(roles.contains(&"application"));
        assert!(roles.contains(&"entrypoint"));
        assert!(roles.contains(&"host"));
        assert!(roles.contains(&"effect"));
        assert!(roles.contains(&"adapter"));
        assert!(roles.contains(&"resource"));
        assert_eq!(
            projection.projection_identity.domain(),
            IdentityDomain::Projection
        );
    }

    #[test]
    fn missing_host_is_rejected() {
        let mut input = valid_input();
        input.host_profile_identity = None;
        let result = validate_application_host_binding(&input);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingInput)
        );
    }

    #[test]
    fn alternative_and_fallback_hosts_are_rejected() {
        let mut alternative = valid_input();
        alternative
            .alternative_host_profile_identities
            .push(wire(IdentityDomain::Evaluator, b"second-host"));
        let result = validate_application_host_binding(&alternative);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::AmbiguousClaim)
        );
        let mut fallback = valid_input();
        fallback.fallback_host_profile_identity =
            Some(wire(IdentityDomain::Evaluator, b"fallback-host"));
        let result = validate_application_host_binding(&fallback);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::AmbiguousClaim)
        );
    }

    #[test]
    fn open_effect_summary_is_rejected() {
        let mut input = valid_input();
        input.effect_summary_closed = false;
        let result = validate_application_host_binding(&input);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::OpenRowNotPure)
        );
    }

    #[test]
    fn wrong_domain_operation_is_rejected() {
        let mut input = valid_input();
        input
            .effect_operation_identities
            .push(wire(IdentityDomain::Source, b"undeclared"));
        let result = validate_application_host_binding(&input);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::IdentityDomainMismatch)
        );
    }

    #[test]
    fn stale_cohort_is_rejected() {
        let mut input = valid_input();
        input.expected_adapter_cohort_identity =
            Some(wire(IdentityDomain::Adapter, b"expected-cohort"));
        let result = validate_application_host_binding(&input);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
    }

    #[test]
    fn matching_expected_cohort_passes() {
        let mut input = valid_input();
        input.expected_adapter_cohort_identity = Some(input.adapter_cohort_identity.clone());
        input.expected_resource_profile_identity = Some(input.resource_profile_identity.clone());
        input.expected_host_profile_identity = input.host_profile_identity.clone();
        let result = validate_application_host_binding(&input);
        assert!(result.passed);
    }

    #[test]
    fn unsupported_version_is_rejected() {
        let mut input = valid_input();
        input.schema_version = "kamacite.application-host-binding.v0".to_string();
        let result = validate_application_host_binding(&input);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::UnsupportedSchemaVersion)
        );
    }

    #[test]
    fn authority_overclaim_is_rejected() {
        let mut input = valid_input();
        input
            .semantic_claims
            .push("binding grants host authorization".to_string());
        let result = validate_application_host_binding(&input);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::SemanticOverclaim)
        );
    }

    #[test]
    fn missing_non_claims_are_rejected() {
        let mut input = valid_input();
        input.non_claims = vec!["not host capability or authorization".to_string()];
        let result = validate_application_host_binding(&input);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
    }

    #[test]
    fn wrong_application_domain_is_rejected() {
        let mut input = valid_input();
        input.application_identity = wire(IdentityDomain::Source, b"application");
        let result = validate_application_host_binding(&input);
        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::IdentityDomainMismatch)
        );
    }

    #[test]
    fn canonical_ordering_normalizes_operation_order() {
        let mut reordered = valid_input();
        reordered.effect_operation_identities.reverse();
        let base = validate_application_host_binding(&valid_input());
        let reordered = validate_application_host_binding(&reordered);
        let (Some(base), Some(reordered)) = (base.binding, reordered.binding) else {
            return;
        };
        assert_eq!(base.binding_identity, reordered.binding_identity);
    }

    #[test]
    fn single_field_change_rekeys_binding_identity() {
        let base = validate_application_host_binding(&valid_input());
        let Some(base) = base.binding else {
            return;
        };
        let mutations: Vec<fn(&mut ApplicationHostBindingInput)> = vec![
            |input| {
                input.application_identity = wire(IdentityDomain::TargetArtifact, b"other-app");
            },
            |input| {
                input.entrypoint_identity = wire(IdentityDomain::TargetArtifact, b"other-entry");
            },
            |input| {
                input.input_schema_identity = wire(IdentityDomain::Adapter, b"other-input");
            },
            |input| {
                input.output_schema_identity = wire(IdentityDomain::Adapter, b"other-output");
            },
            |input| {
                input.effect_summary_identity =
                    wire(IdentityDomain::AlgebraicEffect, b"other-effects");
            },
            |input| {
                input.host_profile_identity = Some(wire(IdentityDomain::Evaluator, b"other-host"));
            },
            |input| {
                input.adapter_cohort_identity = wire(IdentityDomain::Adapter, b"other-cohort");
            },
            |input| {
                input.resource_profile_identity = wire(IdentityDomain::Adapter, b"other-resource");
            },
        ];
        for mutate in mutations {
            let mut changed = valid_input();
            mutate(&mut changed);
            let result = validate_application_host_binding(&changed);
            let Some(changed) = result.binding else {
                return;
            };
            assert_ne!(base.binding_identity, changed.binding_identity);
        }
    }
}
