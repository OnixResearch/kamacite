use crate::identity::{
    CompatibilityIdentity, EffectOperationIdentity, IdentityDomain, WireIdentity,
};
use crate::preserves_codec::{PreservesLimits, PreservesSchema, encode_typed_preserves};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

pub const SEMANTIC_OPERATION_SCHEMA_VERSION: u32 = 2;
pub const SEMANTIC_OPERATION_PRESERVES_SCHEMA: &str = "kamacite.semantic-effect-operation.v2";
pub const SEMANTIC_OPERATION_ROOT_LABEL: &str = "kamacite-semantic-effect-operation-v2";
pub const SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA_VERSION: u32 = 1;
pub const SEMANTIC_OPERATION_COMPATIBILITY_PRESERVES_SCHEMA: &str =
    "kamacite.semantic-effect-operation-compatibility.v1";
pub const SEMANTIC_OPERATION_COMPATIBILITY_ROOT_LABEL: &str =
    "kamacite-semantic-effect-operation-compatibility-v1";
pub const MAX_OPERATION_DEPENDENCIES: usize = 128;
pub const MAX_COMPATIBILITY_CONTEXTS: usize = 32;
pub const MAX_SUPPORTING_REFS: usize = 128;
pub const MAX_NON_CLAIMS: usize = 32;
pub const LEGACY_EFFECT_OPERATION_SCHEMA_VERSION: u32 = 1;
pub const MAX_MIGRATION_CHANGED_FIELDS: usize = 32;
pub const MAX_OPERATION_REFERENCES_PER_SURFACE: usize = 256;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContinuationContract {
    None,
    SingleUse,
    MultiUse,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReplayClass {
    Deterministic,
    Recorded,
    NonReplayable,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationImplementation {
    DefaultBehavior(WireIdentity),
    Abstract,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationDescriptor {
    pub schema_version: u32,
    pub effect_family_identity: WireIdentity,
    pub parameter_schema_identity: WireIdentity,
    pub result_schema_identity: WireIdentity,
    pub continuation_contract: ContinuationContract,
    pub replay_class: ReplayClass,
    pub dependency_identities: Vec<WireIdentity>,
    pub implementation: OperationImplementation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationDescriptorInput {
    pub schema_version: u32,
    pub effect_family_identity: WireIdentity,
    pub parameter_schema_identity: WireIdentity,
    pub result_schema_identity: WireIdentity,
    pub continuation_contract: ContinuationContract,
    pub replay_class: ReplayClass,
    pub dependency_identities: Vec<WireIdentity>,
    pub default_behavior_identity: Option<WireIdentity>,
    pub abstract_operation: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationMetadata {
    pub semantic_operation_identity: EffectOperationIdentity,
    pub display_name: String,
    pub aliases: Vec<String>,
    pub path: Option<String>,
    pub source_span: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompatibilityContext {
    Replay,
    CacheRead,
    RemoteRequest,
    LiveHostExecution,
    Upgrade,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationCompatibility {
    pub schema_version: u32,
    pub source_operation_identity: WireIdentity,
    pub target_operation_identity: WireIdentity,
    pub allowed_contexts: Vec<CompatibilityContext>,
    pub supporting_refs: Vec<WireIdentity>,
    pub diagnostics: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifiedSemanticOperation {
    pub descriptor: SemanticOperationDescriptor,
    pub canonical_bytes: Vec<u8>,
    pub identity: EffectOperationIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifiedSemanticOperationCompatibility {
    pub compatibility: SemanticOperationCompatibility,
    pub canonical_bytes: Vec<u8>,
    pub identity: CompatibilityIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LegacyEffectOperationReference {
    pub schema_version: u32,
    pub operation_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationReferenceBundle {
    pub declared_operations: Vec<WireIdentity>,
    pub effect_row: Vec<WireIdentity>,
    pub effect_summary: Vec<WireIdentity>,
    pub handler_descriptor: Vec<WireIdentity>,
    pub handler_receipt: Vec<WireIdentity>,
    pub effect_log: Vec<WireIdentity>,
    pub adapter_bindings: Vec<WireIdentity>,
    pub target_imports: Vec<WireIdentity>,
    pub wasm_imports: Vec<WireIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticOperationMigrationPreview {
    pub legacy: LegacyEffectOperationReference,
    pub semantic_operation_identity: EffectOperationIdentity,
    pub semantic_descriptor: SemanticOperationDescriptor,
    pub changed_fields: Vec<String>,
    pub mutation_performed: bool,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticOperationError {
    UnsupportedDescriptorVersion {
        actual: u32,
    },
    UnsupportedCompatibilityVersion {
        actual: u32,
    },
    UnsupportedLegacyVersion {
        actual: u32,
    },
    EmptyLegacyOperationId,
    WrongIdentityDomain {
        field: &'static str,
        expected: IdentityDomain,
    },
    InvalidIdentity {
        field: &'static str,
    },
    TooManyItems {
        field: &'static str,
        actual: usize,
        maximum: usize,
    },
    DuplicateItem {
        field: &'static str,
    },
    EmptyCollection {
        field: &'static str,
    },
    MissingImplementation,
    ConflictingImplementation,
    SameCompatibilityIdentity,
    UnsupportedCompatibilityUse {
        context: CompatibilityContext,
    },
    OperationMismatch {
        requested: WireIdentity,
        handled: WireIdentity,
    },
    UndeclaredOperation {
        surface: &'static str,
        identity: WireIdentity,
    },
    Canonicalization(String),
}

impl Display for SemanticOperationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedDescriptorVersion { actual } => write!(
                formatter,
                "semantic operation descriptor version {actual} is unsupported"
            ),
            Self::UnsupportedCompatibilityVersion { actual } => write!(
                formatter,
                "semantic operation compatibility version {actual} is unsupported"
            ),
            Self::UnsupportedLegacyVersion { actual } => write!(
                formatter,
                "legacy effect operation version {actual} is unsupported"
            ),
            Self::EmptyLegacyOperationId => {
                formatter.write_str("legacy effect operation id must not be empty")
            }
            Self::WrongIdentityDomain { field, expected } => {
                write!(formatter, "{field} must use the {expected} identity domain")
            }
            Self::InvalidIdentity { field } => {
                write!(formatter, "{field} contains an invalid identity")
            }
            Self::TooManyItems {
                field,
                actual,
                maximum,
            } => write!(
                formatter,
                "{field} has {actual} items; maximum is {maximum}"
            ),
            Self::DuplicateItem { field } => write!(formatter, "{field} contains a duplicate item"),
            Self::EmptyCollection { field } => write!(formatter, "{field} must not be empty"),
            Self::MissingImplementation => formatter.write_str(
                "semantic operation requires a default behavior identity or an abstract marker",
            ),
            Self::ConflictingImplementation => formatter.write_str(
                "semantic operation cannot have both a default behavior identity and an abstract marker",
            ),
            Self::SameCompatibilityIdentity => formatter.write_str(
                "semantic operation compatibility source and target identities must differ",
            ),
            Self::UnsupportedCompatibilityUse { context } => {
                write!(
                    formatter,
                    "semantic operation compatibility does not admit {context:?}"
                )
            }
            Self::OperationMismatch { requested, handled } => write!(
                formatter,
                "semantic operation mismatch: requested {} but handler provides {}",
                requested.hex.as_str(),
                handled.hex.as_str()
            ),
            Self::UndeclaredOperation { surface, identity } => write!(
                formatter,
                "{surface} references undeclared semantic operation {}",
                identity.hex.as_str()
            ),
            Self::Canonicalization(message) => {
                write!(
                    formatter,
                    "semantic operation canonicalization failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SemanticOperationError {}

#[derive(Serialize)]
#[serde(rename = "kamacite-semantic-effect-operation-v2")]
struct CanonicalSemanticOperation {
    schema_version: u32,
    effect_family_identity: WireIdentity,
    parameter_schema_identity: WireIdentity,
    result_schema_identity: WireIdentity,
    continuation_contract: ContinuationContract,
    replay_class: ReplayClass,
    dependency_identities: Vec<WireIdentity>,
    implementation: OperationImplementation,
}

#[derive(Serialize)]
#[serde(rename = "kamacite-semantic-effect-operation-compatibility-v1")]
struct CanonicalSemanticOperationCompatibility {
    schema_version: u32,
    source_operation_identity: WireIdentity,
    target_operation_identity: WireIdentity,
    allowed_contexts: Vec<CompatibilityContext>,
    supporting_refs: Vec<WireIdentity>,
    diagnostics: Vec<String>,
    non_claims: Vec<String>,
}

pub fn semantic_operation_descriptor(
    input: &SemanticOperationDescriptorInput,
) -> Result<SemanticOperationDescriptor, SemanticOperationError> {
    let implementation = match (&input.default_behavior_identity, input.abstract_operation) {
        (Some(identity), false) => OperationImplementation::DefaultBehavior(identity.clone()),
        (None, true) => OperationImplementation::Abstract,
        (None, false) => return Err(SemanticOperationError::MissingImplementation),
        (Some(_), true) => return Err(SemanticOperationError::ConflictingImplementation),
    };
    normalize_and_validate_descriptor(&SemanticOperationDescriptor {
        schema_version: input.schema_version,
        effect_family_identity: input.effect_family_identity.clone(),
        parameter_schema_identity: input.parameter_schema_identity.clone(),
        result_schema_identity: input.result_schema_identity.clone(),
        continuation_contract: input.continuation_contract.clone(),
        replay_class: input.replay_class.clone(),
        dependency_identities: input.dependency_identities.clone(),
        implementation,
    })
}

pub fn identify_semantic_operation(
    descriptor: &SemanticOperationDescriptor,
) -> Result<IdentifiedSemanticOperation, SemanticOperationError> {
    let descriptor = normalize_and_validate_descriptor(descriptor)?;
    let body = CanonicalSemanticOperation {
        schema_version: descriptor.schema_version,
        effect_family_identity: descriptor.effect_family_identity.clone(),
        parameter_schema_identity: descriptor.parameter_schema_identity.clone(),
        result_schema_identity: descriptor.result_schema_identity.clone(),
        continuation_contract: descriptor.continuation_contract.clone(),
        replay_class: descriptor.replay_class.clone(),
        dependency_identities: descriptor.dependency_identities.clone(),
        implementation: descriptor.implementation.clone(),
    };
    let canonical_bytes = encode_typed_preserves(
        &body,
        PreservesSchema {
            schema_version: SEMANTIC_OPERATION_PRESERVES_SCHEMA,
            root_label: SEMANTIC_OPERATION_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| SemanticOperationError::Canonicalization(error.to_string()))?;
    let identity = EffectOperationIdentity::compute(&canonical_bytes);
    Ok(IdentifiedSemanticOperation {
        descriptor,
        canonical_bytes,
        identity,
    })
}

pub fn identify_semantic_operation_compatibility(
    compatibility: &SemanticOperationCompatibility,
) -> Result<IdentifiedSemanticOperationCompatibility, SemanticOperationError> {
    let compatibility = normalize_and_validate_compatibility(compatibility)?;
    let body = CanonicalSemanticOperationCompatibility {
        schema_version: compatibility.schema_version,
        source_operation_identity: compatibility.source_operation_identity.clone(),
        target_operation_identity: compatibility.target_operation_identity.clone(),
        allowed_contexts: compatibility.allowed_contexts.clone(),
        supporting_refs: compatibility.supporting_refs.clone(),
        diagnostics: compatibility.diagnostics.clone(),
        non_claims: compatibility.non_claims.clone(),
    };
    let canonical_bytes = encode_typed_preserves(
        &body,
        PreservesSchema {
            schema_version: SEMANTIC_OPERATION_COMPATIBILITY_PRESERVES_SCHEMA,
            root_label: SEMANTIC_OPERATION_COMPATIBILITY_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| SemanticOperationError::Canonicalization(error.to_string()))?;
    let identity = CompatibilityIdentity::compute(&canonical_bytes);
    Ok(IdentifiedSemanticOperationCompatibility {
        compatibility,
        canonical_bytes,
        identity,
    })
}

pub fn validate_semantic_operation_reference_bundle(
    bundle: &SemanticOperationReferenceBundle,
) -> Result<SemanticOperationReferenceBundle, SemanticOperationError> {
    let declared_operations =
        normalize_operation_references(&bundle.declared_operations, "declared_operations", true)?;
    let surfaces = [
        ("effect_row", &bundle.effect_row),
        ("effect_summary", &bundle.effect_summary),
        ("handler_descriptor", &bundle.handler_descriptor),
        ("handler_receipt", &bundle.handler_receipt),
        ("effect_log", &bundle.effect_log),
        ("adapter_bindings", &bundle.adapter_bindings),
        ("target_imports", &bundle.target_imports),
        ("wasm_imports", &bundle.wasm_imports),
    ];
    let mut normalized = Vec::with_capacity(surfaces.len());
    for (surface, references) in surfaces {
        let references = normalize_operation_references(references, surface, false)?;
        if let Some(identity) = references
            .iter()
            .find(|identity| !declared_operations.contains(identity))
        {
            return Err(SemanticOperationError::UndeclaredOperation {
                surface,
                identity: identity.clone(),
            });
        }
        normalized.push(references);
    }
    let mut normalized = normalized.into_iter();
    Ok(SemanticOperationReferenceBundle {
        declared_operations,
        effect_row: normalized.next().unwrap_or_default(),
        effect_summary: normalized.next().unwrap_or_default(),
        handler_descriptor: normalized.next().unwrap_or_default(),
        handler_receipt: normalized.next().unwrap_or_default(),
        effect_log: normalized.next().unwrap_or_default(),
        adapter_bindings: normalized.next().unwrap_or_default(),
        target_imports: normalized.next().unwrap_or_default(),
        wasm_imports: normalized.next().unwrap_or_default(),
    })
}

pub fn preview_semantic_operation_migration(
    legacy: &LegacyEffectOperationReference,
    descriptor: &SemanticOperationDescriptor,
    changed_fields: &[String],
) -> Result<SemanticOperationMigrationPreview, SemanticOperationError> {
    if legacy.schema_version != LEGACY_EFFECT_OPERATION_SCHEMA_VERSION {
        return Err(SemanticOperationError::UnsupportedLegacyVersion {
            actual: legacy.schema_version,
        });
    }
    if legacy.operation_id.is_empty() {
        return Err(SemanticOperationError::EmptyLegacyOperationId);
    }
    let identified = identify_semantic_operation(descriptor)?;
    let changed_fields = normalize_strings(
        changed_fields,
        "changed_fields",
        MAX_MIGRATION_CHANGED_FIELDS,
        true,
    )?;
    Ok(SemanticOperationMigrationPreview {
        legacy: legacy.clone(),
        semantic_operation_identity: identified.identity,
        semantic_descriptor: identified.descriptor,
        changed_fields,
        mutation_performed: false,
        non_claims: vec![
            "legacy identity was not reinterpreted".to_string(),
            "preview does not mutate governed fixtures".to_string(),
            "mapping does not prove semantic equivalence".to_string(),
        ],
    })
}

pub fn require_exact_semantic_operation(
    requested: &EffectOperationIdentity,
    handled: &EffectOperationIdentity,
) -> Result<(), SemanticOperationError> {
    if requested != handled {
        return Err(SemanticOperationError::OperationMismatch {
            requested: requested.to_wire(),
            handled: handled.to_wire(),
        });
    }
    Ok(())
}

pub fn admit_semantic_operation_compatibility(
    compatibility: &SemanticOperationCompatibility,
    source: &WireIdentity,
    target: &WireIdentity,
    context: CompatibilityContext,
) -> Result<(), SemanticOperationError> {
    let compatibility = normalize_and_validate_compatibility(compatibility)?;
    validate_domain(
        source,
        IdentityDomain::EffectOperation,
        "source_operation_identity",
    )?;
    validate_domain(
        target,
        IdentityDomain::EffectOperation,
        "target_operation_identity",
    )?;
    let source = EffectOperationIdentity::from_wire(source.clone()).map_err(|_| {
        SemanticOperationError::InvalidIdentity {
            field: "source_operation_identity",
        }
    })?;
    let target = EffectOperationIdentity::from_wire(target.clone()).map_err(|_| {
        SemanticOperationError::InvalidIdentity {
            field: "target_operation_identity",
        }
    })?;
    let compatibility_source =
        EffectOperationIdentity::from_wire(compatibility.source_operation_identity.clone())
            .map_err(|_| SemanticOperationError::InvalidIdentity {
                field: "source_operation_identity",
            })?;
    let compatibility_target =
        EffectOperationIdentity::from_wire(compatibility.target_operation_identity.clone())
            .map_err(|_| SemanticOperationError::InvalidIdentity {
                field: "target_operation_identity",
            })?;
    require_exact_semantic_operation(&source, &compatibility_source)?;
    require_exact_semantic_operation(&target, &compatibility_target)?;
    if !compatibility.allowed_contexts.contains(&context) {
        return Err(SemanticOperationError::UnsupportedCompatibilityUse { context });
    }
    Ok(())
}

fn normalize_and_validate_descriptor(
    descriptor: &SemanticOperationDescriptor,
) -> Result<SemanticOperationDescriptor, SemanticOperationError> {
    if descriptor.schema_version != SEMANTIC_OPERATION_SCHEMA_VERSION {
        return Err(SemanticOperationError::UnsupportedDescriptorVersion {
            actual: descriptor.schema_version,
        });
    }
    validate_domain(
        &descriptor.effect_family_identity,
        IdentityDomain::AlgebraicEffect,
        "effect_family_identity",
    )?;
    validate_domain(
        &descriptor.parameter_schema_identity,
        IdentityDomain::CanonicalValue,
        "parameter_schema_identity",
    )?;
    validate_domain(
        &descriptor.result_schema_identity,
        IdentityDomain::CanonicalValue,
        "result_schema_identity",
    )?;
    if let OperationImplementation::DefaultBehavior(identity) = &descriptor.implementation {
        validate_domain(
            identity,
            IdentityDomain::CanonicalValue,
            "default_behavior_identity",
        )?;
    }
    let dependency_identities = normalize_identities(
        &descriptor.dependency_identities,
        "dependency_identities",
        MAX_OPERATION_DEPENDENCIES,
        false,
    )?;
    Ok(SemanticOperationDescriptor {
        schema_version: descriptor.schema_version,
        effect_family_identity: descriptor.effect_family_identity.clone(),
        parameter_schema_identity: descriptor.parameter_schema_identity.clone(),
        result_schema_identity: descriptor.result_schema_identity.clone(),
        continuation_contract: descriptor.continuation_contract.clone(),
        replay_class: descriptor.replay_class.clone(),
        dependency_identities,
        implementation: descriptor.implementation.clone(),
    })
}

fn normalize_and_validate_compatibility(
    compatibility: &SemanticOperationCompatibility,
) -> Result<SemanticOperationCompatibility, SemanticOperationError> {
    if compatibility.schema_version != SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA_VERSION {
        return Err(SemanticOperationError::UnsupportedCompatibilityVersion {
            actual: compatibility.schema_version,
        });
    }
    validate_domain(
        &compatibility.source_operation_identity,
        IdentityDomain::EffectOperation,
        "source_operation_identity",
    )?;
    validate_domain(
        &compatibility.target_operation_identity,
        IdentityDomain::EffectOperation,
        "target_operation_identity",
    )?;
    if compatibility.source_operation_identity == compatibility.target_operation_identity {
        return Err(SemanticOperationError::SameCompatibilityIdentity);
    }
    let allowed_contexts = normalize_values(
        &compatibility.allowed_contexts,
        "allowed_contexts",
        MAX_COMPATIBILITY_CONTEXTS,
        true,
    )?;
    let supporting_refs = normalize_identities(
        &compatibility.supporting_refs,
        "supporting_refs",
        MAX_SUPPORTING_REFS,
        true,
    )?;
    let diagnostics = normalize_strings(
        &compatibility.diagnostics,
        "diagnostics",
        MAX_SUPPORTING_REFS,
        false,
    )?;
    let non_claims = normalize_strings(
        &compatibility.non_claims,
        "non_claims",
        MAX_NON_CLAIMS,
        true,
    )?;
    Ok(SemanticOperationCompatibility {
        schema_version: compatibility.schema_version,
        source_operation_identity: compatibility.source_operation_identity.clone(),
        target_operation_identity: compatibility.target_operation_identity.clone(),
        allowed_contexts,
        supporting_refs,
        diagnostics,
        non_claims,
    })
}

fn validate_domain(
    identity: &WireIdentity,
    expected: IdentityDomain,
    field: &'static str,
) -> Result<(), SemanticOperationError> {
    identity
        .validate_domain(expected.clone())
        .map_err(|_| SemanticOperationError::WrongIdentityDomain { field, expected })
}

fn normalize_identities(
    values: &[WireIdentity],
    field: &'static str,
    maximum: usize,
    require_non_empty: bool,
) -> Result<Vec<WireIdentity>, SemanticOperationError> {
    if values.iter().any(|identity| identity.validate().is_err()) {
        return Err(SemanticOperationError::InvalidIdentity { field });
    }
    normalize_values(values, field, maximum, require_non_empty)
}

fn normalize_operation_references(
    values: &[WireIdentity],
    field: &'static str,
    require_non_empty: bool,
) -> Result<Vec<WireIdentity>, SemanticOperationError> {
    for identity in values {
        validate_domain(identity, IdentityDomain::EffectOperation, field)?;
    }
    normalize_identities(
        values,
        field,
        MAX_OPERATION_REFERENCES_PER_SURFACE,
        require_non_empty,
    )
}

fn normalize_strings(
    values: &[String],
    field: &'static str,
    maximum: usize,
    require_non_empty: bool,
) -> Result<Vec<String>, SemanticOperationError> {
    if values.iter().any(String::is_empty) {
        return Err(SemanticOperationError::EmptyCollection { field });
    }
    normalize_values(values, field, maximum, require_non_empty)
}

fn normalize_values<T>(
    values: &[T],
    field: &'static str,
    maximum: usize,
    require_non_empty: bool,
) -> Result<Vec<T>, SemanticOperationError>
where
    T: Clone + Ord,
{
    if require_non_empty && values.is_empty() {
        return Err(SemanticOperationError::EmptyCollection { field });
    }
    if values.len() > maximum {
        return Err(SemanticOperationError::TooManyItems {
            field,
            actual: values.len(),
            maximum,
        });
    }
    let normalized = values.iter().cloned().collect::<BTreeSet<_>>();
    if normalized.len() != values.len() {
        return Err(SemanticOperationError::DuplicateItem { field });
    }
    Ok(normalized.into_iter().collect())
}

// r[verify kamacite.algebraic_effects.semantic_identity_fixtures]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::compute_identity;

    const FAMILY_BYTES: &[u8] = b"family";
    const PARAMETER_BYTES: &[u8] = b"parameter";
    const RESULT_BYTES: &[u8] = b"result";
    const BEHAVIOR_BYTES: &[u8] = b"behavior";
    const OTHER_BEHAVIOR_BYTES: &[u8] = b"other-behavior";
    const SUPPORT_BYTES: &[u8] = b"support";

    fn descriptor() -> SemanticOperationDescriptor {
        SemanticOperationDescriptor {
            schema_version: SEMANTIC_OPERATION_SCHEMA_VERSION,
            effect_family_identity: compute_identity(IdentityDomain::AlgebraicEffect, FAMILY_BYTES),
            parameter_schema_identity: compute_identity(
                IdentityDomain::CanonicalValue,
                PARAMETER_BYTES,
            ),
            result_schema_identity: compute_identity(IdentityDomain::CanonicalValue, RESULT_BYTES),
            continuation_contract: ContinuationContract::SingleUse,
            replay_class: ReplayClass::Recorded,
            dependency_identities: Vec::new(),
            implementation: OperationImplementation::DefaultBehavior(compute_identity(
                IdentityDomain::CanonicalValue,
                BEHAVIOR_BYTES,
            )),
        }
    }

    #[test]
    fn metadata_rename_does_not_change_semantic_identity() {
        let result = identify_semantic_operation(&descriptor());
        assert!(result.is_ok());
        let Ok(identified) = result else {
            return;
        };
        let first = SemanticOperationMetadata {
            semantic_operation_identity: identified.identity.clone(),
            display_name: "ReadConfig".to_string(),
            aliases: Vec::new(),
            path: None,
            source_span: None,
            documentation: None,
        };
        let mut renamed = first.clone();
        renamed.display_name = "LoadConfig".to_string();
        assert_eq!(
            first.semantic_operation_identity,
            renamed.semantic_operation_identity
        );
    }

    #[test]
    fn behavior_change_rekeys_operation() {
        let first = identify_semantic_operation(&descriptor());
        assert!(first.is_ok());
        let Ok(first) = first else {
            return;
        };
        let mut changed = descriptor();
        changed.implementation = OperationImplementation::DefaultBehavior(compute_identity(
            IdentityDomain::CanonicalValue,
            OTHER_BEHAVIOR_BYTES,
        ));
        let second = identify_semantic_operation(&changed);
        assert!(second.is_ok());
        if let Ok(second) = second {
            assert_ne!(first.identity, second.identity);
        }
    }

    #[test]
    fn every_semantic_field_change_rekeys_operation() {
        let base = descriptor();
        let identified_base = identify_semantic_operation(&base);
        assert!(identified_base.is_ok());
        let Ok(identified_base) = identified_base else {
            return;
        };
        let mut variants = Vec::new();
        let mut changed = base.clone();
        changed.effect_family_identity =
            compute_identity(IdentityDomain::AlgebraicEffect, b"other-family");
        variants.push(changed);
        let mut changed = base.clone();
        changed.parameter_schema_identity =
            compute_identity(IdentityDomain::CanonicalValue, b"other-parameter");
        variants.push(changed);
        let mut changed = base.clone();
        changed.result_schema_identity =
            compute_identity(IdentityDomain::CanonicalValue, b"other-result");
        variants.push(changed);
        let mut changed = base.clone();
        changed.continuation_contract = ContinuationContract::MultiUse;
        variants.push(changed);
        let mut changed = base.clone();
        changed.replay_class = ReplayClass::NonReplayable;
        variants.push(changed);
        let mut changed = base.clone();
        changed.dependency_identities =
            vec![compute_identity(IdentityDomain::Source, b"dependency")];
        variants.push(changed);
        let mut changed = base.clone();
        changed.implementation = OperationImplementation::Abstract;
        variants.push(changed);
        for variant in variants {
            let identified = identify_semantic_operation(&variant);
            assert!(identified.is_ok());
            if let Ok(identified) = identified {
                assert_ne!(identified_base.identity, identified.identity);
            }
        }
    }

    #[test]
    fn canonical_preserves_and_identity_are_deterministic() {
        let first = identify_semantic_operation(&descriptor());
        let second = identify_semantic_operation(&descriptor());
        assert!(first.is_ok());
        assert!(second.is_ok());
        if let (Ok(first), Ok(second)) = (first, second) {
            assert_eq!(first.canonical_bytes, second.canonical_bytes);
            assert_eq!(first.identity, second.identity);
            assert_eq!(
                first.identity,
                EffectOperationIdentity::compute(&first.canonical_bytes)
            );
        }
    }

    #[test]
    fn abstract_operation_has_canonical_identity() {
        let mut abstract_descriptor = descriptor();
        abstract_descriptor.implementation = OperationImplementation::Abstract;
        let result = identify_semantic_operation(&abstract_descriptor);
        assert!(result.is_ok());
    }

    #[test]
    fn descriptor_input_rejects_missing_or_conflicting_implementation() {
        let descriptor = descriptor();
        let mut input = SemanticOperationDescriptorInput {
            schema_version: descriptor.schema_version,
            effect_family_identity: descriptor.effect_family_identity,
            parameter_schema_identity: descriptor.parameter_schema_identity,
            result_schema_identity: descriptor.result_schema_identity,
            continuation_contract: descriptor.continuation_contract,
            replay_class: descriptor.replay_class,
            dependency_identities: descriptor.dependency_identities,
            default_behavior_identity: None,
            abstract_operation: false,
        };
        assert_eq!(
            semantic_operation_descriptor(&input),
            Err(SemanticOperationError::MissingImplementation)
        );
        input.default_behavior_identity = Some(compute_identity(
            IdentityDomain::CanonicalValue,
            BEHAVIOR_BYTES,
        ));
        input.abstract_operation = true;
        assert_eq!(
            semantic_operation_descriptor(&input),
            Err(SemanticOperationError::ConflictingImplementation)
        );
    }

    #[test]
    fn migration_preview_keeps_legacy_and_semantic_identities_separate() {
        let legacy = LegacyEffectOperationReference {
            schema_version: LEGACY_EFFECT_OPERATION_SCHEMA_VERSION,
            operation_id: "op.read-config".to_string(),
        };
        let changed_fields = vec!["identity-domain".to_string(), "nominal-family".to_string()];
        let preview = preview_semantic_operation_migration(&legacy, &descriptor(), &changed_fields);
        assert!(preview.is_ok());
        if let Ok(preview) = preview {
            assert_eq!(preview.legacy, legacy);
            assert!(!preview.mutation_performed);
            assert_eq!(
                preview.semantic_operation_identity.domain(),
                IdentityDomain::EffectOperation
            );
            assert!(
                preview
                    .non_claims
                    .iter()
                    .any(|claim| { claim == "legacy identity was not reinterpreted" })
            );
        }
    }

    #[test]
    fn migration_preview_rejects_unknown_legacy_version_and_empty_id() {
        let unknown = LegacyEffectOperationReference {
            schema_version: SEMANTIC_OPERATION_SCHEMA_VERSION,
            operation_id: "op.read-config".to_string(),
        };
        assert!(matches!(
            preview_semantic_operation_migration(
                &unknown,
                &descriptor(),
                &["identity-domain".to_string()],
            ),
            Err(SemanticOperationError::UnsupportedLegacyVersion { .. })
        ));
        let empty = LegacyEffectOperationReference {
            schema_version: LEGACY_EFFECT_OPERATION_SCHEMA_VERSION,
            operation_id: String::new(),
        };
        assert_eq!(
            preview_semantic_operation_migration(
                &empty,
                &descriptor(),
                &["identity-domain".to_string()],
            ),
            Err(SemanticOperationError::EmptyLegacyOperationId)
        );
    }

    #[test]
    fn exact_handler_match_passes_and_stale_match_fails() {
        let requested = identify_semantic_operation(&descriptor());
        assert!(requested.is_ok());
        let Ok(requested) = requested else {
            return;
        };
        let requested = requested.identity;
        assert!(require_exact_semantic_operation(&requested, &requested).is_ok());
        let stale = EffectOperationIdentity::compute(b"stale");
        assert!(matches!(
            require_exact_semantic_operation(&requested, &stale),
            Err(SemanticOperationError::OperationMismatch { .. })
        ));
    }

    #[test]
    fn exact_operation_references_cover_all_artifact_surfaces() {
        let operation = identify_semantic_operation(&descriptor());
        assert!(operation.is_ok());
        let Ok(operation) = operation else {
            return;
        };
        let references = vec![operation.identity.to_wire()];
        let bundle = SemanticOperationReferenceBundle {
            declared_operations: references.clone(),
            effect_row: references.clone(),
            effect_summary: references.clone(),
            handler_descriptor: references.clone(),
            handler_receipt: references.clone(),
            effect_log: references.clone(),
            adapter_bindings: references.clone(),
            target_imports: references.clone(),
            wasm_imports: references,
        };
        assert!(validate_semantic_operation_reference_bundle(&bundle).is_ok());
    }

    #[test]
    fn stale_operation_reference_fails_before_artifact_use() {
        let declared = identify_semantic_operation(&descriptor());
        assert!(declared.is_ok());
        let Ok(declared) = declared else {
            return;
        };
        let stale = compute_identity(IdentityDomain::EffectOperation, b"stale-reference");
        let bundle = SemanticOperationReferenceBundle {
            declared_operations: vec![declared.identity.to_wire()],
            effect_row: vec![stale.clone()],
            effect_summary: Vec::new(),
            handler_descriptor: Vec::new(),
            handler_receipt: Vec::new(),
            effect_log: Vec::new(),
            adapter_bindings: Vec::new(),
            target_imports: Vec::new(),
            wasm_imports: Vec::new(),
        };
        assert_eq!(
            validate_semantic_operation_reference_bundle(&bundle),
            Err(SemanticOperationError::UndeclaredOperation {
                surface: "effect_row",
                identity: stale,
            })
        );
    }

    #[test]
    fn directional_replay_compatibility_does_not_admit_live_execution() {
        let source = compute_identity(IdentityDomain::EffectOperation, b"old");
        let target = identify_semantic_operation(&descriptor());
        assert!(target.is_ok());
        let Ok(target) = target else {
            return;
        };
        let target = target.identity;
        let compatibility = SemanticOperationCompatibility {
            schema_version: SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA_VERSION,
            source_operation_identity: source.clone(),
            target_operation_identity: target.to_wire(),
            allowed_contexts: vec![CompatibilityContext::Replay],
            supporting_refs: vec![compute_identity(
                IdentityDomain::EvidenceChain,
                SUPPORT_BYTES,
            )],
            diagnostics: vec!["reviewed replay substitution".to_string()],
            non_claims: vec!["not host authority".to_string()],
        };
        assert!(identify_semantic_operation_compatibility(&compatibility).is_ok());
        let identified = identify_semantic_operation_compatibility(&compatibility);
        if let Ok(identified) = identified {
            assert_eq!(identified.identity.domain(), IdentityDomain::Compatibility);
        }
        assert!(
            admit_semantic_operation_compatibility(
                &compatibility,
                &source,
                &target.to_wire(),
                CompatibilityContext::Replay,
            )
            .is_ok()
        );
        assert!(matches!(
            admit_semantic_operation_compatibility(
                &compatibility,
                &source,
                &target.to_wire(),
                CompatibilityContext::LiveHostExecution,
            ),
            Err(SemanticOperationError::UnsupportedCompatibilityUse { .. })
        ));
        assert!(matches!(
            admit_semantic_operation_compatibility(
                &compatibility,
                &target.to_wire(),
                &source,
                CompatibilityContext::Replay,
            ),
            Err(SemanticOperationError::OperationMismatch { .. })
        ));
    }

    #[test]
    fn rejects_wrong_nominal_family_domain() {
        let mut invalid = descriptor();
        invalid.effect_family_identity = compute_identity(IdentityDomain::CanonicalValue, b"wrong");
        assert!(matches!(
            identify_semantic_operation(&invalid),
            Err(SemanticOperationError::WrongIdentityDomain {
                field: "effect_family_identity",
                ..
            })
        ));
    }

    #[test]
    fn dependency_order_is_canonical_and_duplicates_fail() {
        let first_dependency = compute_identity(IdentityDomain::Source, b"a");
        let second_dependency = compute_identity(IdentityDomain::Source, b"b");
        let mut first = descriptor();
        first.dependency_identities = vec![second_dependency.clone(), first_dependency.clone()];
        let mut second = descriptor();
        second.dependency_identities = vec![first_dependency.clone(), second_dependency];
        let first_identity = identify_semantic_operation(&first);
        let second_identity = identify_semantic_operation(&second);
        assert!(first_identity.is_ok());
        assert!(second_identity.is_ok());
        if let (Ok(first_identity), Ok(second_identity)) = (first_identity, second_identity) {
            assert_eq!(first_identity.identity, second_identity.identity);
        }
        second.dependency_identities = vec![first_dependency.clone(), first_dependency];
        assert!(matches!(
            identify_semantic_operation(&second),
            Err(SemanticOperationError::DuplicateItem {
                field: "dependency_identities"
            })
        ));
        let mut invalid_dependency = compute_identity(IdentityDomain::Source, b"invalid");
        invalid_dependency.algorithm = "sha256".to_string();
        second.dependency_identities = vec![invalid_dependency];
        assert_eq!(
            identify_semantic_operation(&second),
            Err(SemanticOperationError::InvalidIdentity {
                field: "dependency_identities"
            })
        );
    }
}
