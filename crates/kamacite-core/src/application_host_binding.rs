use crate::identity::{Identity, IdentityDomain, compute_identity};
use crate::preserves_codec::{PreservesLimits, PreservesSchema, encode_typed_preserves};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

pub const APPLICATION_HOST_BINDING_SCHEMA_VERSION: u32 = 1;
pub const APPLICATION_HOST_BINDING_PRESERVES_SCHEMA: &str = "kamacite.application-host-binding.v1";
pub const APPLICATION_HOST_BINDING_ROOT_LABEL: &str = "kamacite-application-host-binding-v1";
pub const APPLICATION_HOST_BINDING_RECEIPT_SCHEMA: &str =
    "kamacite.application-host-binding-validation-receipt.v1";
pub const APPLICATION_HOST_BINDING_RECEIPT_ROOT_LABEL: &str =
    "kamacite-application-host-binding-validation-receipt-v1";
pub const APPLICATION_HOST_BINDING_VALENCE_SCHEMA: &str =
    "kamacite.application-host-binding-valence.v1";
pub const APPLICATION_HOST_BINDING_VALENCE_ROOT_LABEL: &str =
    "kamacite-application-host-binding-valence-v1";
pub const MAX_BINDING_HOST_PROFILES: usize = 8;
pub const MAX_BINDING_OPERATIONS: usize = 256;
pub const MAX_BINDING_COMPATIBILITY_IDENTITIES: usize = 64;
pub const MAX_BINDING_NON_CLAIMS: usize = 32;
pub const MAX_BINDING_METADATA_ITEMS: usize = 32;
pub const MAX_BINDING_DIAGNOSTIC_BYTES: usize = 512;

const REQUIREMENT_ARTIFACT: &str = "r[kamacite.application_host_binding.artifact]";
const REQUIREMENT_HOST_SELECTION: &str = "r[kamacite.application_host_binding.host_selection]";
const REQUIREMENT_EFFECTS: &str = "r[kamacite.application_host_binding.effects]";
const REQUIREMENT_COHORTS: &str = "r[kamacite.application_host_binding.cohorts]";
const REQUIREMENT_BOUNDARY: &str = "r[kamacite.application_host_binding.boundary]";
const REQUIREMENT_RECEIPT: &str = "r[kamacite.application_host_binding.receipt]";
const REQUIREMENT_PROJECTION: &str = "r[kamacite.application_host_binding.projection]";

pub const APPLICATION_HOST_BINDING_REQUIRED_NON_CLAIMS: &[&str] = &[
    "not application correctness",
    "not effect completeness beyond supplied evidence",
    "not host authorization",
    "not host behavior",
    "not release eligibility",
    "not resource enforcement",
    "not runtime success",
    "not sandbox enforcement",
];

const FORBIDDEN_DIAGNOSTIC_TERMS: &[&str] = &[
    "credential",
    "environment",
    "physical path",
    "raw_prompt",
    "secret",
    "stderr",
    "stdout",
    "token",
];

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationHostBindingMetadata {
    pub display_name: String,
    pub aliases: Vec<String>,
    pub help_text: String,
    pub source_spans: Vec<String>,
    pub diagnostics: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationEffectSummaryInput {
    pub identity: Identity,
    pub closed: bool,
    pub unknown: bool,
    pub operation_identities: Vec<Identity>,
    pub declared_operation_identities: Vec<Identity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationHostBindingDraft {
    pub schema_version: u32,
    pub application_artifact_identity: Identity,
    pub entrypoint_identity: Identity,
    pub input_schema_identity: Identity,
    pub output_schema_identity: Identity,
    pub effect_summary: ApplicationEffectSummaryInput,
    pub host_profile_identities: Vec<Identity>,
    pub adapter_cohort_identity: Identity,
    pub resource_profile_identity: Identity,
    pub policy_identity: Identity,
    pub compatibility_identities: Vec<Identity>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationHostBindingCohort {
    pub application_artifact_identity: Identity,
    pub entrypoint_identity: Identity,
    pub input_schema_identity: Identity,
    pub output_schema_identity: Identity,
    pub effect_summary_identity: Identity,
    pub effect_operation_identities: Vec<Identity>,
    pub host_profile_identity: Identity,
    pub adapter_cohort_identity: Identity,
    pub resource_profile_identity: Identity,
    pub policy_identity: Identity,
    pub compatibility_identities: Vec<Identity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationHostBindingValidationInput {
    pub binding: ApplicationHostBindingDraft,
    pub reviewed_cohort: ApplicationHostBindingCohort,
    #[serde(default)]
    pub metadata: ApplicationHostBindingMetadata,
    #[serde(default)]
    pub claimed_authorities: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBinding {
    pub schema_version: u32,
    pub application_artifact_identity: Identity,
    pub entrypoint_identity: Identity,
    pub input_schema_identity: Identity,
    pub output_schema_identity: Identity,
    pub effect_summary_identity: Identity,
    pub effect_operation_identities: Vec<Identity>,
    pub host_profile_identity: Identity,
    pub adapter_cohort_identity: Identity,
    pub resource_profile_identity: Identity,
    pub policy_identity: Identity,
    pub compatibility_identities: Vec<Identity>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IdentifiedApplicationHostBinding {
    pub binding: ApplicationHostBinding,
    pub canonical_bytes: Vec<u8>,
    pub identity: Identity,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationHostBindingIssueCode {
    UnsupportedSchemaVersion,
    WrongIdentityDomain,
    InvalidIdentity,
    HostSelectionMissing,
    HostSelectionAmbiguous,
    EffectSummaryOpen,
    EffectSummaryUnknown,
    EffectOperationUndeclared,
    CohortMismatch,
    DuplicateItem,
    ItemLimitExceeded,
    MissingNonClaim,
    UnsafeDiagnostic,
    AuthorityOverclaim,
    CanonicalizationFailed,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ApplicationHostBindingDiagnostic {
    pub code: ApplicationHostBindingIssueCode,
    pub requirement_id: String,
    pub field: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationHostBindingVerdict {
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingValidation {
    pub verdict: ApplicationHostBindingVerdict,
    pub binding: Option<IdentifiedApplicationHostBinding>,
    pub diagnostics: Vec<ApplicationHostBindingDiagnostic>,
    pub checked_rules: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationHostBindingRole {
    Application,
    Entrypoint,
    InputSchema,
    OutputSchema,
    EffectSummary,
    EffectOperation,
    HostProfile,
    AdapterCohort,
    ResourceProfile,
    Policy,
    Compatibility,
    Binding,
    ValidationReceipt,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ApplicationHostBindingIdentityRef {
    pub role: ApplicationHostBindingRole,
    pub identity: Identity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingReceiptBody {
    pub schema_version: String,
    pub input_identities: Vec<ApplicationHostBindingIdentityRef>,
    pub binding_identity: Option<Identity>,
    pub verdict: ApplicationHostBindingVerdict,
    pub diagnostics: Vec<ApplicationHostBindingDiagnostic>,
    pub checked_rules: Vec<String>,
    pub policy_identity: Identity,
    pub source_spans: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingReceipt {
    pub body: ApplicationHostBindingReceiptBody,
    pub canonical_bytes: Vec<u8>,
    pub receipt_identity: Identity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingValenceArtifact {
    pub role: ApplicationHostBindingRole,
    pub identity: Identity,
    pub checked_claim: String,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingValenceProjectionBody {
    pub schema_version: String,
    pub subject_identity: Identity,
    pub artifacts: Vec<ApplicationHostBindingValenceArtifact>,
    pub diagnostic_role: String,
    pub diagnostics: Vec<ApplicationHostBindingDiagnostic>,
    pub non_claim_role: String,
    pub non_claims: Vec<String>,
    pub source_spans: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ApplicationHostBindingValenceProjection {
    pub body: ApplicationHostBindingValenceProjectionBody,
    pub canonical_bytes: Vec<u8>,
    pub projection_identity: Identity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicationHostBindingError {
    Canonicalization(String),
    ReceiptIdentityMismatch,
    RejectedProjection,
}

impl Display for ApplicationHostBindingError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Canonicalization(message) => {
                write!(
                    formatter,
                    "application-host binding canonicalization failed: {message}"
                )
            }
            Self::ReceiptIdentityMismatch => {
                formatter.write_str("application-host binding receipt identity does not match")
            }
            Self::RejectedProjection => formatter.write_str(
                "rejected application-host binding receipt cannot produce a passing projection",
            ),
        }
    }
}

impl std::error::Error for ApplicationHostBindingError {}

#[derive(Serialize)]
#[serde(rename = "kamacite-application-host-binding-v1")]
struct CanonicalApplicationHostBinding<'a> {
    schema_version: u32,
    application_artifact_identity: &'a Identity,
    entrypoint_identity: &'a Identity,
    input_schema_identity: &'a Identity,
    output_schema_identity: &'a Identity,
    effect_summary_identity: &'a Identity,
    effect_operation_identities: &'a [Identity],
    host_profile_identity: &'a Identity,
    adapter_cohort_identity: &'a Identity,
    resource_profile_identity: &'a Identity,
    policy_identity: &'a Identity,
    compatibility_identities: &'a [Identity],
    non_claims: &'a [String],
}

#[derive(Serialize)]
#[serde(rename = "kamacite-application-host-binding-validation-receipt-v1")]
struct CanonicalApplicationHostBindingReceipt<'a> {
    body: &'a ApplicationHostBindingReceiptBody,
}

#[derive(Serialize)]
#[serde(rename = "kamacite-application-host-binding-valence-v1")]
struct CanonicalApplicationHostBindingValenceProjection<'a> {
    body: &'a ApplicationHostBindingValenceProjectionBody,
}

pub fn validate_application_host_binding(
    input: &ApplicationHostBindingValidationInput,
) -> ApplicationHostBindingValidation {
    let mut diagnostics = Vec::new();
    validate_schema(input.binding.schema_version, &mut diagnostics);
    validate_identity_fields(input, &mut diagnostics);
    validate_host_selection(input, &mut diagnostics);
    let effect_operations = validate_effect_summary(input, &mut diagnostics);
    let compatibility_identities = normalized_identities(
        &input.binding.compatibility_identities,
        IdentityDomain::EvidenceChain,
        "binding.compatibility_identities",
        MAX_BINDING_COMPATIBILITY_IDENTITIES,
        REQUIREMENT_COHORTS,
        &mut diagnostics,
    );
    let non_claims = validate_non_claims(&input.binding.non_claims, &mut diagnostics);
    validate_metadata(&input.metadata, &mut diagnostics);
    validate_authority_claims(&input.claimed_authorities, &mut diagnostics);
    validate_cohort(
        input,
        &effect_operations,
        &compatibility_identities,
        &mut diagnostics,
    );
    diagnostics.sort();
    diagnostics.dedup();

    let binding = if diagnostics.is_empty() {
        match identified_binding(
            input,
            effect_operations,
            compatibility_identities,
            non_claims,
        ) {
            Ok(binding) => Some(binding),
            Err(error) => {
                diagnostics.push(diagnostic(
                    ApplicationHostBindingIssueCode::CanonicalizationFailed,
                    REQUIREMENT_ARTIFACT,
                    "binding",
                    error.to_string(),
                ));
                None
            }
        }
    } else {
        None
    };
    diagnostics.sort();
    let verdict = if binding.is_some() && diagnostics.is_empty() {
        ApplicationHostBindingVerdict::Accepted
    } else {
        ApplicationHostBindingVerdict::Rejected
    };
    ApplicationHostBindingValidation {
        verdict,
        binding,
        diagnostics,
        checked_rules: checked_rules(),
    }
}

pub fn build_application_host_binding_receipt(
    input: &ApplicationHostBindingValidationInput,
) -> Result<
    (
        ApplicationHostBindingValidation,
        ApplicationHostBindingReceipt,
    ),
    ApplicationHostBindingError,
> {
    let validation = validate_application_host_binding(input);
    let body = ApplicationHostBindingReceiptBody {
        schema_version: APPLICATION_HOST_BINDING_RECEIPT_SCHEMA.to_string(),
        input_identities: binding_identity_refs(&input.binding),
        binding_identity: validation
            .binding
            .as_ref()
            .map(|binding| binding.identity.clone()),
        verdict: validation.verdict.clone(),
        diagnostics: validation.diagnostics.clone(),
        checked_rules: validation.checked_rules.clone(),
        policy_identity: input.binding.policy_identity.clone(),
        source_spans: normalized_strings_for_receipt(&input.metadata.source_spans),
        non_claims: normalized_strings_for_receipt(&input.binding.non_claims),
    };
    let canonical_bytes = encode_typed_preserves(
        &CanonicalApplicationHostBindingReceipt { body: &body },
        PreservesSchema {
            schema_version: APPLICATION_HOST_BINDING_RECEIPT_SCHEMA,
            root_label: APPLICATION_HOST_BINDING_RECEIPT_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ApplicationHostBindingError::Canonicalization(error.to_string()))?;
    let receipt_identity = compute_identity(IdentityDomain::Receipt, &canonical_bytes);
    Ok((
        validation,
        ApplicationHostBindingReceipt {
            body,
            canonical_bytes,
            receipt_identity,
        },
    ))
}

pub fn project_application_host_binding_to_valence(
    receipt: &ApplicationHostBindingReceipt,
) -> Result<ApplicationHostBindingValenceProjection, ApplicationHostBindingError> {
    verify_application_host_binding_receipt(receipt)?;
    if receipt.body.verdict != ApplicationHostBindingVerdict::Accepted {
        return Err(ApplicationHostBindingError::RejectedProjection);
    }
    let Some(subject_identity) = receipt.body.binding_identity.clone() else {
        return Err(ApplicationHostBindingError::RejectedProjection);
    };
    let mut artifacts = receipt
        .body
        .input_identities
        .iter()
        .map(|reference| valence_artifact(reference.role.clone(), reference.identity.clone()))
        .collect::<Vec<_>>();
    artifacts.push(valence_artifact(
        ApplicationHostBindingRole::Binding,
        subject_identity.clone(),
    ));
    artifacts.push(valence_artifact(
        ApplicationHostBindingRole::ValidationReceipt,
        receipt.receipt_identity.clone(),
    ));
    artifacts.sort_by(|left, right| {
        left.role
            .cmp(&right.role)
            .then_with(|| left.identity.cmp(&right.identity))
    });
    artifacts.dedup_by(|left, right| left.role == right.role && left.identity == right.identity);
    let body = ApplicationHostBindingValenceProjectionBody {
        schema_version: APPLICATION_HOST_BINDING_VALENCE_SCHEMA.to_string(),
        subject_identity,
        artifacts,
        diagnostic_role: "application-host-binding-validation-diagnostic".to_string(),
        diagnostics: receipt.body.diagnostics.clone(),
        non_claim_role: "application-host-binding-explicit-non-claim".to_string(),
        non_claims: receipt.body.non_claims.clone(),
        source_spans: receipt.body.source_spans.clone(),
    };
    let canonical_bytes = encode_typed_preserves(
        &CanonicalApplicationHostBindingValenceProjection { body: &body },
        PreservesSchema {
            schema_version: APPLICATION_HOST_BINDING_VALENCE_SCHEMA,
            root_label: APPLICATION_HOST_BINDING_VALENCE_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ApplicationHostBindingError::Canonicalization(error.to_string()))?;
    let projection_identity = compute_identity(IdentityDomain::Projection, &canonical_bytes);
    Ok(ApplicationHostBindingValenceProjection {
        body,
        canonical_bytes,
        projection_identity,
    })
}

pub fn verify_application_host_binding_receipt(
    receipt: &ApplicationHostBindingReceipt,
) -> Result<(), ApplicationHostBindingError> {
    let canonical_bytes = encode_typed_preserves(
        &CanonicalApplicationHostBindingReceipt {
            body: &receipt.body,
        },
        PreservesSchema {
            schema_version: APPLICATION_HOST_BINDING_RECEIPT_SCHEMA,
            root_label: APPLICATION_HOST_BINDING_RECEIPT_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ApplicationHostBindingError::Canonicalization(error.to_string()))?;
    let identity = compute_identity(IdentityDomain::Receipt, &canonical_bytes);
    if identity != receipt.receipt_identity || canonical_bytes != receipt.canonical_bytes {
        return Err(ApplicationHostBindingError::ReceiptIdentityMismatch);
    }
    Ok(())
}

fn validate_schema(schema_version: u32, diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>) {
    if schema_version != APPLICATION_HOST_BINDING_SCHEMA_VERSION {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::UnsupportedSchemaVersion,
            REQUIREMENT_ARTIFACT,
            "binding.schema_version",
            "application-host binding schema version is unsupported",
        ));
    }
}

fn validate_identity_fields(
    input: &ApplicationHostBindingValidationInput,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    let binding = &input.binding;
    validate_identity(
        &binding.application_artifact_identity,
        IdentityDomain::TargetArtifact,
        "binding.application_artifact_identity",
        REQUIREMENT_ARTIFACT,
        diagnostics,
    );
    validate_identity(
        &binding.entrypoint_identity,
        IdentityDomain::FunctionAddress,
        "binding.entrypoint_identity",
        REQUIREMENT_ARTIFACT,
        diagnostics,
    );
    validate_identity(
        &binding.input_schema_identity,
        IdentityDomain::CanonicalValue,
        "binding.input_schema_identity",
        REQUIREMENT_ARTIFACT,
        diagnostics,
    );
    validate_identity(
        &binding.output_schema_identity,
        IdentityDomain::CanonicalValue,
        "binding.output_schema_identity",
        REQUIREMENT_ARTIFACT,
        diagnostics,
    );
    validate_identity(
        &binding.effect_summary.identity,
        IdentityDomain::AlgebraicEffect,
        "binding.effect_summary.identity",
        REQUIREMENT_EFFECTS,
        diagnostics,
    );
    validate_identity(
        &binding.adapter_cohort_identity,
        IdentityDomain::Adapter,
        "binding.adapter_cohort_identity",
        REQUIREMENT_COHORTS,
        diagnostics,
    );
    validate_identity(
        &binding.resource_profile_identity,
        IdentityDomain::Adapter,
        "binding.resource_profile_identity",
        REQUIREMENT_COHORTS,
        diagnostics,
    );
    validate_identity(
        &binding.policy_identity,
        IdentityDomain::CanonicalValue,
        "binding.policy_identity",
        REQUIREMENT_BOUNDARY,
        diagnostics,
    );
    validate_cohort_identity_fields(&input.reviewed_cohort, diagnostics);
}

fn validate_cohort_identity_fields(
    cohort: &ApplicationHostBindingCohort,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    let fields = [
        (
            &cohort.application_artifact_identity,
            IdentityDomain::TargetArtifact,
            "reviewed_cohort.application_artifact_identity",
        ),
        (
            &cohort.entrypoint_identity,
            IdentityDomain::FunctionAddress,
            "reviewed_cohort.entrypoint_identity",
        ),
        (
            &cohort.input_schema_identity,
            IdentityDomain::CanonicalValue,
            "reviewed_cohort.input_schema_identity",
        ),
        (
            &cohort.output_schema_identity,
            IdentityDomain::CanonicalValue,
            "reviewed_cohort.output_schema_identity",
        ),
        (
            &cohort.effect_summary_identity,
            IdentityDomain::AlgebraicEffect,
            "reviewed_cohort.effect_summary_identity",
        ),
        (
            &cohort.host_profile_identity,
            IdentityDomain::Adapter,
            "reviewed_cohort.host_profile_identity",
        ),
        (
            &cohort.adapter_cohort_identity,
            IdentityDomain::Adapter,
            "reviewed_cohort.adapter_cohort_identity",
        ),
        (
            &cohort.resource_profile_identity,
            IdentityDomain::Adapter,
            "reviewed_cohort.resource_profile_identity",
        ),
        (
            &cohort.policy_identity,
            IdentityDomain::CanonicalValue,
            "reviewed_cohort.policy_identity",
        ),
    ];
    for (identity, domain, field) in fields {
        validate_identity(identity, domain, field, REQUIREMENT_COHORTS, diagnostics);
    }
    let _ = normalized_identities(
        &cohort.effect_operation_identities,
        IdentityDomain::EffectOperation,
        "reviewed_cohort.effect_operation_identities",
        MAX_BINDING_OPERATIONS,
        REQUIREMENT_COHORTS,
        diagnostics,
    );
    let _ = normalized_identities(
        &cohort.compatibility_identities,
        IdentityDomain::EvidenceChain,
        "reviewed_cohort.compatibility_identities",
        MAX_BINDING_COMPATIBILITY_IDENTITIES,
        REQUIREMENT_COHORTS,
        diagnostics,
    );
}

fn validate_host_selection(
    input: &ApplicationHostBindingValidationInput,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    let hosts = &input.binding.host_profile_identities;
    if hosts.is_empty() {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::HostSelectionMissing,
            REQUIREMENT_HOST_SELECTION,
            "binding.host_profile_identities",
            "one explicit host profile is required",
        ));
        return;
    }
    if hosts.len() != 1 {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::HostSelectionAmbiguous,
            REQUIREMENT_HOST_SELECTION,
            "binding.host_profile_identities",
            "multiple, alternative, repeated, implicit, or fallback host profiles are denied",
        ));
    }
    if hosts.len() > MAX_BINDING_HOST_PROFILES {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::ItemLimitExceeded,
            REQUIREMENT_HOST_SELECTION,
            "binding.host_profile_identities",
            "host profile collection exceeds its bounded limit",
        ));
    }
    for host in hosts {
        validate_identity(
            host,
            IdentityDomain::Adapter,
            "binding.host_profile_identities",
            REQUIREMENT_HOST_SELECTION,
            diagnostics,
        );
    }
}

fn validate_effect_summary(
    input: &ApplicationHostBindingValidationInput,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) -> Vec<Identity> {
    let summary = &input.binding.effect_summary;
    if !summary.closed {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::EffectSummaryOpen,
            REQUIREMENT_EFFECTS,
            "binding.effect_summary.closed",
            "executable bindings require a closed effect summary",
        ));
    }
    if summary.unknown {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::EffectSummaryUnknown,
            REQUIREMENT_EFFECTS,
            "binding.effect_summary.unknown",
            "unknown effect requirements deny executable binding admission",
        ));
    }
    let operations = normalized_identities(
        &summary.operation_identities,
        IdentityDomain::EffectOperation,
        "binding.effect_summary.operation_identities",
        MAX_BINDING_OPERATIONS,
        REQUIREMENT_EFFECTS,
        diagnostics,
    );
    let declared = normalized_identities(
        &summary.declared_operation_identities,
        IdentityDomain::EffectOperation,
        "binding.effect_summary.declared_operation_identities",
        MAX_BINDING_OPERATIONS,
        REQUIREMENT_EFFECTS,
        diagnostics,
    );
    for operation in &operations {
        if !declared.contains(operation) {
            diagnostics.push(diagnostic(
                ApplicationHostBindingIssueCode::EffectOperationUndeclared,
                REQUIREMENT_EFFECTS,
                "binding.effect_summary.operation_identities",
                "effect summary references an undeclared semantic operation identity",
            ));
        }
    }
    operations
}

fn validate_non_claims(
    values: &[String],
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) -> Vec<String> {
    let normalized = normalized_strings(
        values,
        "binding.non_claims",
        MAX_BINDING_NON_CLAIMS,
        REQUIREMENT_BOUNDARY,
        diagnostics,
    );
    for required in APPLICATION_HOST_BINDING_REQUIRED_NON_CLAIMS {
        if !normalized.iter().any(|value| value == required) {
            diagnostics.push(diagnostic(
                ApplicationHostBindingIssueCode::MissingNonClaim,
                REQUIREMENT_BOUNDARY,
                "binding.non_claims",
                format!("required non-claim is missing: {required}"),
            ));
        }
    }
    normalized
}

fn validate_metadata(
    metadata: &ApplicationHostBindingMetadata,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    validate_metadata_collection(&metadata.aliases, "metadata.aliases", diagnostics);
    validate_metadata_collection(&metadata.source_spans, "metadata.source_spans", diagnostics);
    validate_metadata_collection(&metadata.diagnostics, "metadata.diagnostics", diagnostics);
    for value in &metadata.diagnostics {
        let normalized = value.to_ascii_lowercase();
        let unsafe_term = FORBIDDEN_DIAGNOSTIC_TERMS
            .iter()
            .any(|term| normalized.contains(term));
        if unsafe_term || value.len() > MAX_BINDING_DIAGNOSTIC_BYTES {
            diagnostics.push(diagnostic(
                ApplicationHostBindingIssueCode::UnsafeDiagnostic,
                REQUIREMENT_BOUNDARY,
                "metadata.diagnostics",
                "binding metadata contains an unsafe diagnostic",
            ));
        }
    }
}

fn validate_metadata_collection(
    values: &[String],
    field: &str,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    if values.len() > MAX_BINDING_METADATA_ITEMS {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::ItemLimitExceeded,
            REQUIREMENT_BOUNDARY,
            field,
            "binding metadata collection exceeds its bounded limit",
        ));
    }
}

fn validate_authority_claims(
    claimed_authorities: &[String],
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    if !claimed_authorities.is_empty() {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::AuthorityOverclaim,
            REQUIREMENT_BOUNDARY,
            "claimed_authorities",
            "binding identity cannot grant or claim host authority",
        ));
    }
}

fn validate_cohort(
    input: &ApplicationHostBindingValidationInput,
    operations: &[Identity],
    compatibility_identities: &[Identity],
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    let binding = &input.binding;
    let cohort = &input.reviewed_cohort;
    compare_identity(
        &binding.application_artifact_identity,
        &cohort.application_artifact_identity,
        "binding.application_artifact_identity",
        diagnostics,
    );
    compare_identity(
        &binding.entrypoint_identity,
        &cohort.entrypoint_identity,
        "binding.entrypoint_identity",
        diagnostics,
    );
    compare_identity(
        &binding.input_schema_identity,
        &cohort.input_schema_identity,
        "binding.input_schema_identity",
        diagnostics,
    );
    compare_identity(
        &binding.output_schema_identity,
        &cohort.output_schema_identity,
        "binding.output_schema_identity",
        diagnostics,
    );
    compare_identity(
        &binding.effect_summary.identity,
        &cohort.effect_summary_identity,
        "binding.effect_summary.identity",
        diagnostics,
    );
    if let Some(host) = binding.host_profile_identities.first() {
        compare_identity(
            host,
            &cohort.host_profile_identity,
            "binding.host_profile_identities",
            diagnostics,
        );
    }
    compare_identity(
        &binding.adapter_cohort_identity,
        &cohort.adapter_cohort_identity,
        "binding.adapter_cohort_identity",
        diagnostics,
    );
    compare_identity(
        &binding.resource_profile_identity,
        &cohort.resource_profile_identity,
        "binding.resource_profile_identity",
        diagnostics,
    );
    compare_identity(
        &binding.policy_identity,
        &cohort.policy_identity,
        "binding.policy_identity",
        diagnostics,
    );
    let expected_operations = normalized_identity_set(&cohort.effect_operation_identities);
    if operations != expected_operations {
        diagnostics.push(cohort_mismatch(
            "binding.effect_summary.operation_identities",
        ));
    }
    let expected_compatibility = normalized_identity_set(&cohort.compatibility_identities);
    if compatibility_identities != expected_compatibility {
        diagnostics.push(cohort_mismatch("binding.compatibility_identities"));
    }
}

fn compare_identity(
    actual: &Identity,
    expected: &Identity,
    field: &str,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    if actual != expected {
        diagnostics.push(cohort_mismatch(field));
    }
}

fn cohort_mismatch(field: &str) -> ApplicationHostBindingDiagnostic {
    diagnostic(
        ApplicationHostBindingIssueCode::CohortMismatch,
        REQUIREMENT_COHORTS,
        field,
        "binding identity does not match the reviewed producer cohort",
    )
}

fn identified_binding(
    input: &ApplicationHostBindingValidationInput,
    effect_operation_identities: Vec<Identity>,
    compatibility_identities: Vec<Identity>,
    non_claims: Vec<String>,
) -> Result<IdentifiedApplicationHostBinding, ApplicationHostBindingError> {
    let Some(host_profile_identity) = input.binding.host_profile_identities.first().cloned() else {
        return Err(ApplicationHostBindingError::Canonicalization(
            "host profile disappeared after validation".to_string(),
        ));
    };
    let binding = ApplicationHostBinding {
        schema_version: input.binding.schema_version,
        application_artifact_identity: input.binding.application_artifact_identity.clone(),
        entrypoint_identity: input.binding.entrypoint_identity.clone(),
        input_schema_identity: input.binding.input_schema_identity.clone(),
        output_schema_identity: input.binding.output_schema_identity.clone(),
        effect_summary_identity: input.binding.effect_summary.identity.clone(),
        effect_operation_identities,
        host_profile_identity,
        adapter_cohort_identity: input.binding.adapter_cohort_identity.clone(),
        resource_profile_identity: input.binding.resource_profile_identity.clone(),
        policy_identity: input.binding.policy_identity.clone(),
        compatibility_identities,
        non_claims,
    };
    let canonical_bytes = encode_binding(&binding)?;
    let identity = compute_identity(IdentityDomain::ApplicationHostBinding, &canonical_bytes);
    Ok(IdentifiedApplicationHostBinding {
        binding,
        canonical_bytes,
        identity,
    })
}

fn encode_binding(
    binding: &ApplicationHostBinding,
) -> Result<Vec<u8>, ApplicationHostBindingError> {
    encode_typed_preserves(
        &CanonicalApplicationHostBinding {
            schema_version: binding.schema_version,
            application_artifact_identity: &binding.application_artifact_identity,
            entrypoint_identity: &binding.entrypoint_identity,
            input_schema_identity: &binding.input_schema_identity,
            output_schema_identity: &binding.output_schema_identity,
            effect_summary_identity: &binding.effect_summary_identity,
            effect_operation_identities: &binding.effect_operation_identities,
            host_profile_identity: &binding.host_profile_identity,
            adapter_cohort_identity: &binding.adapter_cohort_identity,
            resource_profile_identity: &binding.resource_profile_identity,
            policy_identity: &binding.policy_identity,
            compatibility_identities: &binding.compatibility_identities,
            non_claims: &binding.non_claims,
        },
        PreservesSchema {
            schema_version: APPLICATION_HOST_BINDING_PRESERVES_SCHEMA,
            root_label: APPLICATION_HOST_BINDING_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ApplicationHostBindingError::Canonicalization(error.to_string()))
}

fn validate_identity(
    identity: &Identity,
    expected: IdentityDomain,
    field: &str,
    requirement_id: &str,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) {
    if identity.validate().is_err() {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::InvalidIdentity,
            requirement_id,
            field,
            "identity algorithm or digest is invalid",
        ));
        return;
    }
    if identity.domain != expected {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::WrongIdentityDomain,
            requirement_id,
            field,
            format!("identity must use the {expected} domain"),
        ));
    }
}

fn normalized_identities(
    values: &[Identity],
    expected: IdentityDomain,
    field: &str,
    maximum: usize,
    requirement_id: &str,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) -> Vec<Identity> {
    if values.len() > maximum {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::ItemLimitExceeded,
            requirement_id,
            field,
            "identity collection exceeds its bounded limit",
        ));
    }
    for identity in values {
        validate_identity(
            identity,
            expected.clone(),
            field,
            requirement_id,
            diagnostics,
        );
    }
    let normalized = values.iter().cloned().collect::<BTreeSet<_>>();
    if normalized.len() != values.len() {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::DuplicateItem,
            requirement_id,
            field,
            "identity collection contains a duplicate item",
        ));
    }
    normalized.into_iter().collect()
}

fn normalized_strings(
    values: &[String],
    field: &str,
    maximum: usize,
    requirement_id: &str,
    diagnostics: &mut Vec<ApplicationHostBindingDiagnostic>,
) -> Vec<String> {
    if values.len() > maximum {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::ItemLimitExceeded,
            requirement_id,
            field,
            "string collection exceeds its bounded limit",
        ));
    }
    if values.iter().any(String::is_empty) {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::MissingNonClaim,
            requirement_id,
            field,
            "string collection contains an empty item",
        ));
    }
    let normalized = values.iter().cloned().collect::<BTreeSet<_>>();
    if normalized.len() != values.len() {
        diagnostics.push(diagnostic(
            ApplicationHostBindingIssueCode::DuplicateItem,
            requirement_id,
            field,
            "string collection contains a duplicate item",
        ));
    }
    normalized.into_iter().collect()
}

fn normalized_identity_set(values: &[Identity]) -> Vec<Identity> {
    values
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn normalized_strings_for_receipt(values: &[String]) -> Vec<String> {
    values
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn binding_identity_refs(
    binding: &ApplicationHostBindingDraft,
) -> Vec<ApplicationHostBindingIdentityRef> {
    let mut references = vec![
        identity_ref(
            ApplicationHostBindingRole::Application,
            binding.application_artifact_identity.clone(),
        ),
        identity_ref(
            ApplicationHostBindingRole::Entrypoint,
            binding.entrypoint_identity.clone(),
        ),
        identity_ref(
            ApplicationHostBindingRole::InputSchema,
            binding.input_schema_identity.clone(),
        ),
        identity_ref(
            ApplicationHostBindingRole::OutputSchema,
            binding.output_schema_identity.clone(),
        ),
        identity_ref(
            ApplicationHostBindingRole::EffectSummary,
            binding.effect_summary.identity.clone(),
        ),
        identity_ref(
            ApplicationHostBindingRole::AdapterCohort,
            binding.adapter_cohort_identity.clone(),
        ),
        identity_ref(
            ApplicationHostBindingRole::ResourceProfile,
            binding.resource_profile_identity.clone(),
        ),
        identity_ref(
            ApplicationHostBindingRole::Policy,
            binding.policy_identity.clone(),
        ),
    ];
    references.extend(
        binding
            .host_profile_identities
            .iter()
            .cloned()
            .map(|identity| identity_ref(ApplicationHostBindingRole::HostProfile, identity)),
    );
    references.extend(
        binding
            .effect_summary
            .operation_identities
            .iter()
            .cloned()
            .map(|identity| identity_ref(ApplicationHostBindingRole::EffectOperation, identity)),
    );
    references.extend(
        binding
            .compatibility_identities
            .iter()
            .cloned()
            .map(|identity| identity_ref(ApplicationHostBindingRole::Compatibility, identity)),
    );
    references.sort();
    references.dedup();
    references
}

fn identity_ref(
    role: ApplicationHostBindingRole,
    identity: Identity,
) -> ApplicationHostBindingIdentityRef {
    ApplicationHostBindingIdentityRef { role, identity }
}

fn valence_artifact(
    role: ApplicationHostBindingRole,
    identity: Identity,
) -> ApplicationHostBindingValenceArtifact {
    ApplicationHostBindingValenceArtifact {
        role,
        identity,
        checked_claim: "canonical identity linkage for one application-host composition role"
            .to_string(),
        non_claims: vec![
            "not behavior correctness".to_string(),
            "not host authority".to_string(),
        ],
    }
}

fn checked_rules() -> Vec<String> {
    vec![
        REQUIREMENT_ARTIFACT.to_string(),
        REQUIREMENT_HOST_SELECTION.to_string(),
        REQUIREMENT_EFFECTS.to_string(),
        REQUIREMENT_COHORTS.to_string(),
        REQUIREMENT_BOUNDARY.to_string(),
        REQUIREMENT_RECEIPT.to_string(),
        REQUIREMENT_PROJECTION.to_string(),
    ]
}

fn diagnostic(
    code: ApplicationHostBindingIssueCode,
    requirement_id: impl Into<String>,
    field: impl Into<String>,
    message: impl Into<String>,
) -> ApplicationHostBindingDiagnostic {
    ApplicationHostBindingDiagnostic {
        code,
        requirement_id: requirement_id.into(),
        field: field.into(),
        message: message.into(),
    }
}

// r[verify kamacite.application_host_binding.fixtures]
#[cfg(test)]
mod tests {
    use super::*;

    const APPLICATION_BYTES: &[u8] = b"application";
    const ENTRYPOINT_BYTES: &[u8] = b"entrypoint";
    const INPUT_SCHEMA_BYTES: &[u8] = b"input-schema";
    const OUTPUT_SCHEMA_BYTES: &[u8] = b"output-schema";
    const EFFECT_SUMMARY_BYTES: &[u8] = b"effect-summary";
    const EFFECT_OPERATION_BYTES: &[u8] = b"effect-operation";
    const HOST_PROFILE_BYTES: &[u8] = b"host-profile";
    const ADAPTER_COHORT_BYTES: &[u8] = b"adapter-cohort";
    const RESOURCE_PROFILE_BYTES: &[u8] = b"resource-profile";
    const POLICY_BYTES: &[u8] = b"policy";
    const COMPATIBILITY_BYTES: &[u8] = b"compatibility";

    fn identity(domain: IdentityDomain, bytes: &[u8]) -> Identity {
        compute_identity(domain, bytes)
    }

    fn valid_input() -> ApplicationHostBindingValidationInput {
        let operation = identity(IdentityDomain::EffectOperation, EFFECT_OPERATION_BYTES);
        let binding = ApplicationHostBindingDraft {
            schema_version: APPLICATION_HOST_BINDING_SCHEMA_VERSION,
            application_artifact_identity: identity(
                IdentityDomain::TargetArtifact,
                APPLICATION_BYTES,
            ),
            entrypoint_identity: identity(IdentityDomain::FunctionAddress, ENTRYPOINT_BYTES),
            input_schema_identity: identity(IdentityDomain::CanonicalValue, INPUT_SCHEMA_BYTES),
            output_schema_identity: identity(IdentityDomain::CanonicalValue, OUTPUT_SCHEMA_BYTES),
            effect_summary: ApplicationEffectSummaryInput {
                identity: identity(IdentityDomain::AlgebraicEffect, EFFECT_SUMMARY_BYTES),
                closed: true,
                unknown: false,
                operation_identities: vec![operation.clone()],
                declared_operation_identities: vec![operation.clone()],
            },
            host_profile_identities: vec![identity(IdentityDomain::Adapter, HOST_PROFILE_BYTES)],
            adapter_cohort_identity: identity(IdentityDomain::Adapter, ADAPTER_COHORT_BYTES),
            resource_profile_identity: identity(IdentityDomain::Adapter, RESOURCE_PROFILE_BYTES),
            policy_identity: identity(IdentityDomain::CanonicalValue, POLICY_BYTES),
            compatibility_identities: vec![identity(
                IdentityDomain::EvidenceChain,
                COMPATIBILITY_BYTES,
            )],
            non_claims: APPLICATION_HOST_BINDING_REQUIRED_NON_CLAIMS
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        };
        let reviewed_cohort = ApplicationHostBindingCohort {
            application_artifact_identity: binding.application_artifact_identity.clone(),
            entrypoint_identity: binding.entrypoint_identity.clone(),
            input_schema_identity: binding.input_schema_identity.clone(),
            output_schema_identity: binding.output_schema_identity.clone(),
            effect_summary_identity: binding.effect_summary.identity.clone(),
            effect_operation_identities: binding.effect_summary.operation_identities.clone(),
            host_profile_identity: binding.host_profile_identities[0].clone(),
            adapter_cohort_identity: binding.adapter_cohort_identity.clone(),
            resource_profile_identity: binding.resource_profile_identity.clone(),
            policy_identity: binding.policy_identity.clone(),
            compatibility_identities: binding.compatibility_identities.clone(),
        };
        ApplicationHostBindingValidationInput {
            binding,
            reviewed_cohort,
            metadata: ApplicationHostBindingMetadata {
                display_name: "Deterministic fake-backed headless entrypoint".to_string(),
                aliases: vec!["headless".to_string()],
                help_text: "Runs one deterministic fixture-backed turn.".to_string(),
                source_spans: vec!["animus:fixture".to_string()],
                diagnostics: vec!["producer fixture is frozen".to_string()],
            },
            claimed_authorities: Vec::new(),
        }
    }

    fn align_cohort(input: &mut ApplicationHostBindingValidationInput) {
        input.reviewed_cohort = ApplicationHostBindingCohort {
            application_artifact_identity: input.binding.application_artifact_identity.clone(),
            entrypoint_identity: input.binding.entrypoint_identity.clone(),
            input_schema_identity: input.binding.input_schema_identity.clone(),
            output_schema_identity: input.binding.output_schema_identity.clone(),
            effect_summary_identity: input.binding.effect_summary.identity.clone(),
            effect_operation_identities: input.binding.effect_summary.operation_identities.clone(),
            host_profile_identity: input
                .binding
                .host_profile_identities
                .first()
                .cloned()
                .unwrap_or_else(|| identity(IdentityDomain::Adapter, HOST_PROFILE_BYTES)),
            adapter_cohort_identity: input.binding.adapter_cohort_identity.clone(),
            resource_profile_identity: input.binding.resource_profile_identity.clone(),
            policy_identity: input.binding.policy_identity.clone(),
            compatibility_identities: input.binding.compatibility_identities.clone(),
        };
    }

    fn accepted_identity(input: &ApplicationHostBindingValidationInput) -> Option<Identity> {
        validate_application_host_binding(input)
            .binding
            .map(|binding| binding.identity)
    }

    fn has_issue(
        validation: &ApplicationHostBindingValidation,
        code: ApplicationHostBindingIssueCode,
    ) -> bool {
        validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code)
    }

    #[test]
    fn admits_closed_effect_and_capability_free_bindings() {
        let effectful = validate_application_host_binding(&valid_input());
        assert_eq!(effectful.verdict, ApplicationHostBindingVerdict::Accepted);
        assert!(effectful.binding.is_some());
        assert!(effectful.diagnostics.is_empty());

        let mut capability_free = valid_input();
        capability_free
            .binding
            .effect_summary
            .operation_identities
            .clear();
        capability_free
            .binding
            .effect_summary
            .declared_operation_identities
            .clear();
        align_cohort(&mut capability_free);
        let capability_free = validate_application_host_binding(&capability_free);
        assert_eq!(
            capability_free.verdict,
            ApplicationHostBindingVerdict::Accepted
        );
    }

    #[test]
    fn metadata_rename_does_not_change_binding_identity() {
        let first = valid_input();
        let mut renamed = first.clone();
        renamed.metadata.display_name = "Renamed entrypoint".to_string();
        renamed.metadata.aliases = vec!["renamed".to_string()];
        renamed.metadata.help_text = "Different help text.".to_string();
        renamed.metadata.source_spans = vec!["animus:other-span".to_string()];
        assert_eq!(accepted_identity(&first), accepted_identity(&renamed));
    }

    #[test]
    fn canonical_order_is_deterministic() {
        let first_operation = identity(IdentityDomain::EffectOperation, b"first-operation");
        let second_operation = identity(IdentityDomain::EffectOperation, b"second-operation");
        let first_compatibility = identity(IdentityDomain::EvidenceChain, b"first-compatibility");
        let second_compatibility = identity(IdentityDomain::EvidenceChain, b"second-compatibility");
        let mut first = valid_input();
        first.binding.effect_summary.operation_identities =
            vec![second_operation.clone(), first_operation.clone()];
        first.binding.effect_summary.declared_operation_identities =
            vec![first_operation.clone(), second_operation.clone()];
        first.binding.compatibility_identities =
            vec![second_compatibility.clone(), first_compatibility.clone()];
        align_cohort(&mut first);
        let mut second = first.clone();
        second.binding.effect_summary.operation_identities.reverse();
        second
            .binding
            .effect_summary
            .declared_operation_identities
            .reverse();
        second.binding.compatibility_identities.reverse();
        second.binding.non_claims.reverse();
        second.reviewed_cohort.effect_operation_identities.reverse();
        second.reviewed_cohort.compatibility_identities.reverse();
        assert_eq!(accepted_identity(&first), accepted_identity(&second));
    }

    #[test]
    fn each_semantic_field_change_rekeys_binding() {
        let base = valid_input();
        let base_identity = accepted_identity(&base);
        assert!(base_identity.is_some());
        let Some(base_identity) = base_identity else {
            return;
        };
        let mut variants = Vec::new();

        let mut changed = base.clone();
        changed.binding.application_artifact_identity =
            identity(IdentityDomain::TargetArtifact, b"other-application");
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.entrypoint_identity =
            identity(IdentityDomain::FunctionAddress, b"other-entrypoint");
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.input_schema_identity =
            identity(IdentityDomain::CanonicalValue, b"other-input");
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.output_schema_identity =
            identity(IdentityDomain::CanonicalValue, b"other-output");
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.effect_summary.identity =
            identity(IdentityDomain::AlgebraicEffect, b"other-summary");
        variants.push(changed);
        let mut changed = base.clone();
        let operation = identity(IdentityDomain::EffectOperation, b"other-operation");
        changed.binding.effect_summary.operation_identities = vec![operation.clone()];
        changed.binding.effect_summary.declared_operation_identities = vec![operation];
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.host_profile_identities =
            vec![identity(IdentityDomain::Adapter, b"other-host")];
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.adapter_cohort_identity =
            identity(IdentityDomain::Adapter, b"other-adapter");
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.resource_profile_identity =
            identity(IdentityDomain::Adapter, b"other-resource");
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.policy_identity = identity(IdentityDomain::CanonicalValue, b"other-policy");
        variants.push(changed);
        let mut changed = base.clone();
        changed.binding.compatibility_identities = vec![identity(
            IdentityDomain::EvidenceChain,
            b"other-compatibility",
        )];
        variants.push(changed);
        let mut changed = base;
        changed
            .binding
            .non_claims
            .push("not deployment readiness".to_string());
        variants.push(changed);

        for mut variant in variants {
            align_cohort(&mut variant);
            let variant_identity = accepted_identity(&variant);
            assert!(variant_identity.is_some());
            assert_ne!(variant_identity, Some(base_identity.clone()));
        }
    }

    #[test]
    fn rejects_missing_and_multiple_hosts() {
        let mut missing = valid_input();
        missing.binding.host_profile_identities.clear();
        let missing = validate_application_host_binding(&missing);
        assert!(has_issue(
            &missing,
            ApplicationHostBindingIssueCode::HostSelectionMissing
        ));

        let mut multiple = valid_input();
        multiple
            .binding
            .host_profile_identities
            .push(identity(IdentityDomain::Adapter, b"fallback-host"));
        let multiple = validate_application_host_binding(&multiple);
        assert!(has_issue(
            &multiple,
            ApplicationHostBindingIssueCode::HostSelectionAmbiguous
        ));
    }

    #[test]
    fn rejects_open_unknown_and_undeclared_effects() {
        let mut open = valid_input();
        open.binding.effect_summary.closed = false;
        let open = validate_application_host_binding(&open);
        assert!(has_issue(
            &open,
            ApplicationHostBindingIssueCode::EffectSummaryOpen
        ));

        let mut unknown = valid_input();
        unknown.binding.effect_summary.unknown = true;
        let unknown = validate_application_host_binding(&unknown);
        assert!(has_issue(
            &unknown,
            ApplicationHostBindingIssueCode::EffectSummaryUnknown
        ));

        let mut undeclared = valid_input();
        undeclared
            .binding
            .effect_summary
            .declared_operation_identities
            .clear();
        let undeclared = validate_application_host_binding(&undeclared);
        assert!(has_issue(
            &undeclared,
            ApplicationHostBindingIssueCode::EffectOperationUndeclared
        ));
    }

    #[test]
    fn rejects_stale_and_wrong_role_cohorts() {
        let mut stale = valid_input();
        stale.reviewed_cohort.application_artifact_identity =
            identity(IdentityDomain::TargetArtifact, b"stale-application");
        let stale = validate_application_host_binding(&stale);
        assert!(has_issue(
            &stale,
            ApplicationHostBindingIssueCode::CohortMismatch
        ));

        let mut wrong_adapter = valid_input();
        wrong_adapter.reviewed_cohort.adapter_cohort_identity =
            identity(IdentityDomain::Adapter, b"wrong-adapter");
        let wrong_adapter = validate_application_host_binding(&wrong_adapter);
        assert!(has_issue(
            &wrong_adapter,
            ApplicationHostBindingIssueCode::CohortMismatch
        ));

        let mut wrong_domain = valid_input();
        wrong_domain.binding.resource_profile_identity =
            identity(IdentityDomain::Receipt, RESOURCE_PROFILE_BYTES);
        let wrong_domain = validate_application_host_binding(&wrong_domain);
        assert!(has_issue(
            &wrong_domain,
            ApplicationHostBindingIssueCode::WrongIdentityDomain
        ));
    }

    #[test]
    fn rejects_unsupported_version_unsafe_diagnostic_and_authority_overclaim() {
        let mut unsupported = valid_input();
        unsupported.binding.schema_version = APPLICATION_HOST_BINDING_SCHEMA_VERSION + 1;
        let unsupported = validate_application_host_binding(&unsupported);
        assert!(has_issue(
            &unsupported,
            ApplicationHostBindingIssueCode::UnsupportedSchemaVersion
        ));

        let mut unsafe_diagnostic = valid_input();
        unsafe_diagnostic.metadata.diagnostics = vec!["credential leaked".to_string()];
        let unsafe_diagnostic = validate_application_host_binding(&unsafe_diagnostic);
        assert!(has_issue(
            &unsafe_diagnostic,
            ApplicationHostBindingIssueCode::UnsafeDiagnostic
        ));

        let mut overclaim = valid_input();
        overclaim.claimed_authorities = vec!["host filesystem authority".to_string()];
        let overclaim = validate_application_host_binding(&overclaim);
        assert!(has_issue(
            &overclaim,
            ApplicationHostBindingIssueCode::AuthorityOverclaim
        ));
    }

    #[test]
    fn receipts_and_valence_projection_preserve_distinct_roles() {
        let result = build_application_host_binding_receipt(&valid_input());
        assert!(result.is_ok());
        let Ok((validation, receipt)) = result else {
            return;
        };
        assert_eq!(validation.verdict, ApplicationHostBindingVerdict::Accepted);
        assert!(verify_application_host_binding_receipt(&receipt).is_ok());
        let projection = project_application_host_binding_to_valence(&receipt);
        assert!(projection.is_ok());
        if let Ok(projection) = projection {
            let roles = projection
                .body
                .artifacts
                .iter()
                .map(|artifact| artifact.role.clone())
                .collect::<BTreeSet<_>>();
            assert!(roles.contains(&ApplicationHostBindingRole::Application));
            assert!(roles.contains(&ApplicationHostBindingRole::Entrypoint));
            assert!(roles.contains(&ApplicationHostBindingRole::HostProfile));
            assert!(roles.contains(&ApplicationHostBindingRole::EffectSummary));
            assert!(roles.contains(&ApplicationHostBindingRole::AdapterCohort));
            assert!(roles.contains(&ApplicationHostBindingRole::ResourceProfile));
            assert!(roles.contains(&ApplicationHostBindingRole::Binding));
            assert!(roles.contains(&ApplicationHostBindingRole::ValidationReceipt));
            assert_eq!(
                projection.body.diagnostic_role,
                "application-host-binding-validation-diagnostic"
            );
            assert_eq!(
                projection.body.non_claim_role,
                "application-host-binding-explicit-non-claim"
            );
            assert_eq!(projection.body.source_spans, vec!["animus:fixture"]);
        }
    }

    #[test]
    fn rejected_receipt_cannot_produce_passing_projection() {
        let mut input = valid_input();
        input.binding.host_profile_identities.clear();
        let result = build_application_host_binding_receipt(&input);
        assert!(result.is_ok());
        if let Ok((validation, receipt)) = result {
            assert_eq!(validation.verdict, ApplicationHostBindingVerdict::Rejected);
            assert_eq!(
                project_application_host_binding_to_valence(&receipt),
                Err(ApplicationHostBindingError::RejectedProjection)
            );
        }
    }

    #[test]
    fn receipt_tampering_fails_closed() {
        let result = build_application_host_binding_receipt(&valid_input());
        assert!(result.is_ok());
        if let Ok((_validation, mut receipt)) = result {
            receipt.body.non_claims.push("tampered".to_string());
            assert_eq!(
                verify_application_host_binding_receipt(&receipt),
                Err(ApplicationHostBindingError::ReceiptIdentityMismatch)
            );
        }
    }
}
