//! Semantic effect-operation descriptors and compatibility artifacts.
//!
//! Kamacite derives one canonical effect-operation identity from a versioned
//! semantic descriptor. The identity commits to the nominal effect-family
//! identity, parameter and result schema identities, the continuation
//! contract, the replay class, the ordered dependency identities, and exactly
//! one of a default-behavior identity or an explicit abstract-operation
//! marker. Display names, paths, aliases, and documentation stay outside the
//! identity material.
//!
//! Handlers, rows, logs, receipts, and bindings reference exact semantic
//! operation identities. A name or a same-shaped schema never discharges an
//! operation. Intentional substitution between two different identities
//! requires a versioned directional compatibility artifact with bounded
//! contexts.
//!
//! # Claim boundary
//!
//! A valid semantic operation identity or compatibility artifact does not
//! grant host capability, runtime authority, handler correctness,
//! default-behavior correctness, semantic equivalence, replay truth, or
//! release eligibility.

use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{Identity, IdentityDomain};
use crate::identity_model::{
    CompatibilityIdentity, EffectOperationIdentity, TypedIdentity, WireIdentity,
};
use crate::preserves_codec::{PreservesLimits, PreservesSchema, encode_typed_preserves};
use serde::{Deserialize, Serialize};

/// Schema identity of the semantic effect-operation descriptor.
pub const SEMANTIC_OPERATION_DESCRIPTOR_SCHEMA: &str =
    "kamacite.algebraic-effect.semantic-operation-descriptor.v1";
/// Preserves root label of the canonical semantic descriptor record.
pub const SEMANTIC_OPERATION_DESCRIPTOR_ROOT_LABEL: &str = "kamacite-semantic-effect-operation-v1";
/// Schema identity of the semantic-operation compatibility artifact.
pub const SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA: &str =
    "kamacite.algebraic-effect.semantic-operation-compatibility.v1";
/// Preserves root label of the canonical compatibility record.
pub const SEMANTIC_OPERATION_COMPATIBILITY_ROOT_LABEL: &str =
    "kamacite-semantic-operation-compatibility-v1";

/// Marker string for a default-behavior operation inside canonical data.
const DEFAULT_BEHAVIOR_MARKER: &str = "default-behavior";
/// Marker string for an abstract operation inside canonical data.
const ABSTRACT_OPERATION_MARKER: &str = "abstract";

/// Continuation contracts a semantic descriptor may declare.
pub const SUPPORTED_CONTINUATION_CONTRACTS: [&str; 3] = [
    "resumable-one-shot",
    "linear-continuation",
    "tail-continuation",
];
/// Replay classes a semantic descriptor may declare.
pub const SUPPORTED_SEMANTIC_REPLAY_CLASSES: [&str; 4] = [
    "deterministic",
    "recorded-replay",
    "denied",
    "non-replayable",
];
/// Contexts a compatibility artifact may admit.
pub const SUPPORTED_COMPATIBILITY_CONTEXTS: [&str; 2] = ["replay-only", "live-host"];

/// Non-claims every semantic descriptor must carry verbatim.
pub const REQUIRED_DESCRIPTOR_NON_CLAIMS: [&str; 3] = [
    "semantic operation identity is not host authority",
    "default-behavior ref is not behavior correctness",
    "matching keys do not prove semantic equivalence",
];
/// Non-claims every compatibility artifact must carry verbatim.
pub const REQUIRED_COMPATIBILITY_NON_CLAIMS: [&str; 2] = [
    "compatibility is not semantic equivalence",
    "compatibility is not host authority",
];
/// Claim phrases a compatibility artifact must never make.
const FORBIDDEN_COMPATIBILITY_CLAIMS: [&str; 9] = [
    "semantic equivalence",
    "semantic-equivalence",
    "equivalent semantics",
    "semantically equivalent",
    "host authority",
    "authorization",
    "authorizes",
    "grants authority",
    "release eligibility",
];

const REQUIREMENT_DESCRIPTOR: &str = "r[kamacite.algebraic_effects.semantic_operation_descriptor]";
const REQUIREMENT_NAMES: &str = "r[kamacite.algebraic_effects.semantic_operation_names]";
const REQUIREMENT_HANDLER: &str = "r[kamacite.algebraic_effects.semantic_handler_matching]";
const REQUIREMENT_COMPATIBILITY: &str =
    "r[kamacite.algebraic_effects.semantic_operation_compatibility]";
const REQUIREMENT_BOUNDARY: &str = "r[kamacite.algebraic_effects.semantic_identity_boundary]";

/// Direction of an intentional semantic-operation substitution.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticOperationCompatibilityDirection {
    /// Substitution from the older identity to the newer identity.
    OldToNew,
    /// Substitution from the newer identity to the older identity.
    NewToOld,
}

impl SemanticOperationCompatibilityDirection {
    /// The canonical wire label of this direction.
    pub fn label(self) -> &'static str {
        match self {
            Self::OldToNew => "old-to-new",
            Self::NewToOld => "new-to-old",
        }
    }
}

/// Input DTO of one semantic effect-operation descriptor.
///
/// `operation_name` and `aliases` are review metadata. They never enter the
/// canonical identity material.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticEffectOperationDescriptorInput {
    /// Must equal [`SEMANTIC_OPERATION_DESCRIPTOR_SCHEMA`].
    pub schema_version: String,
    /// Nominal effect-family identity. Must use the algebraic-effect domain.
    pub effect_family_identity: Identity,
    /// Parameter schema identity. Must use the adapter domain.
    pub parameter_schema_identity: Identity,
    /// Result schema identity. Must use the adapter domain.
    pub result_schema_identity: Identity,
    /// Continuation contract label. Metadata: display name of the operation.
    pub continuation_contract: String,
    /// Replay class label.
    pub replay_class: String,
    /// Ordered dependency identities. Order is identity material.
    pub dependency_identities: Vec<Identity>,
    /// Default-behavior identity in the canonical-value domain.
    pub default_behavior_identity: Option<Identity>,
    /// Explicit abstract-operation marker.
    pub abstract_operation_marker: bool,
    /// Display name. Metadata only.
    pub operation_name: String,
    /// Aliases. Metadata only.
    pub aliases: Vec<String>,
    /// Required non-claims.
    pub non_claims: Vec<String>,
}

/// Canonical identity material of one semantic effect-operation descriptor.
#[derive(Serialize)]
#[serde(rename = "kamacite-semantic-effect-operation-v1")]
struct CanonicalSemanticOperation<'a> {
    schema_version: &'a str,
    effect_family_identity: &'a Identity,
    parameter_schema_identity: &'a Identity,
    result_schema_identity: &'a Identity,
    continuation_contract: &'a str,
    replay_class: &'a str,
    dependency_identities: &'a [Identity],
    default_marker: &'a str,
    default_behavior_identity: Option<&'a Identity>,
}

/// Input DTO of one directional semantic-operation compatibility artifact.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticOperationCompatibilityInput {
    /// Must equal [`SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA`].
    pub schema_version: String,
    /// Source operation identity in the effect-operation domain.
    pub source_operation_identity: Identity,
    /// Target operation identity in the effect-operation domain.
    pub target_operation_identity: Identity,
    /// Substitution direction.
    pub direction: SemanticOperationCompatibilityDirection,
    /// Allowed use contexts. Must be a non-empty subset of
    /// [`SUPPORTED_COMPATIBILITY_CONTEXTS`].
    pub allowed_contexts: Vec<String>,
    /// Ordered supporting evidence identities.
    pub supporting_evidence_identities: Vec<Identity>,
    /// Reviewer claims about the substitution. Equivalence and authority
    /// language is rejected.
    pub semantic_claims: Vec<String>,
    /// Required non-claims.
    pub non_claims: Vec<String>,
}

/// Canonical identity material of one compatibility artifact.
#[derive(Serialize)]
#[serde(rename = "kamacite-semantic-operation-compatibility-v1")]
struct CanonicalSemanticCompatibility<'a> {
    schema_version: &'a str,
    source_operation_identity: &'a Identity,
    target_operation_identity: &'a Identity,
    direction: &'a str,
    allowed_contexts: &'a [String],
    supporting_evidence_identities: &'a [Identity],
}

/// Validate one semantic effect-operation descriptor.
pub fn validate_semantic_operation_descriptor(
    input: &SemanticEffectOperationDescriptorInput,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if input.schema_version != SEMANTIC_OPERATION_DESCRIPTOR_SCHEMA {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_DESCRIPTOR,
                "descriptor.schema_version",
                "semantic operation descriptor schema version is unsupported",
            )
            .with_detail("actual", input.schema_version.clone())
            .with_detail("expected", SEMANTIC_OPERATION_DESCRIPTOR_SCHEMA.to_string()),
        );
    }
    push_domain_diagnostic(
        &input.effect_family_identity,
        IdentityDomain::AlgebraicEffect,
        "descriptor.effect_family_identity",
        REQUIREMENT_DESCRIPTOR,
        &mut diagnostics,
    );
    push_domain_diagnostic(
        &input.parameter_schema_identity,
        IdentityDomain::Adapter,
        "descriptor.parameter_schema_identity",
        REQUIREMENT_DESCRIPTOR,
        &mut diagnostics,
    );
    push_domain_diagnostic(
        &input.result_schema_identity,
        IdentityDomain::Adapter,
        "descriptor.result_schema_identity",
        REQUIREMENT_DESCRIPTOR,
        &mut diagnostics,
    );
    if !SUPPORTED_CONTINUATION_CONTRACTS.contains(&input.continuation_contract.as_str()) {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::ContinuationPolicyMismatch,
                REQUIREMENT_DESCRIPTOR,
                "descriptor.continuation_contract",
                "semantic operation continuation contract is unsupported",
            )
            .with_detail("actual", input.continuation_contract.clone()),
        );
    }
    if !SUPPORTED_SEMANTIC_REPLAY_CLASSES.contains(&input.replay_class.as_str()) {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedArtifactClass,
                REQUIREMENT_DESCRIPTOR,
                "descriptor.replay_class",
                "semantic operation replay class is unsupported",
            )
            .with_detail("actual", input.replay_class.clone()),
        );
    }
    match (
        &input.default_behavior_identity,
        input.abstract_operation_marker,
    ) {
        (Some(_), true) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteEffectSignature,
            REQUIREMENT_DESCRIPTOR,
            "descriptor.default_behavior_identity",
            "semantic operation must not declare both a default-behavior identity and an abstract marker",
        )),
        (None, false) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteEffectSignature,
            REQUIREMENT_DESCRIPTOR,
            "descriptor.default_behavior_identity",
            "semantic operation must declare a default-behavior identity or an abstract marker",
        )),
        (Some(identity), false) => push_domain_diagnostic(
            identity,
            IdentityDomain::CanonicalValue,
            "descriptor.default_behavior_identity",
            REQUIREMENT_DESCRIPTOR,
            &mut diagnostics,
        ),
        (None, true) => {}
    }
    for missing in REQUIRED_DESCRIPTOR_NON_CLAIMS
        .iter()
        .filter(|required| !input.non_claims.contains(&(*required).to_string()))
    {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MissingNonClaim,
                REQUIREMENT_BOUNDARY,
                "descriptor.non_claims",
                "semantic operation descriptor must preserve the bounded claim",
            )
            .with_detail("missing", (*missing).to_string()),
        );
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

/// Derive the canonical semantic operation identity of a valid descriptor.
///
/// The identity uses the maintained effect-operation domain. Names, aliases,
/// and non-claims never enter the canonical bytes.
pub fn semantic_operation_identity(
    input: &SemanticEffectOperationDescriptorInput,
) -> Result<Identity, Vec<Diagnostic>> {
    let diagnostics = validate_semantic_operation_descriptor(input);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    let canonical = CanonicalSemanticOperation {
        schema_version: SEMANTIC_OPERATION_DESCRIPTOR_SCHEMA,
        effect_family_identity: &input.effect_family_identity,
        parameter_schema_identity: &input.parameter_schema_identity,
        result_schema_identity: &input.result_schema_identity,
        continuation_contract: &input.continuation_contract,
        replay_class: &input.replay_class,
        dependency_identities: &input.dependency_identities,
        default_marker: if input.abstract_operation_marker {
            ABSTRACT_OPERATION_MARKER
        } else {
            DEFAULT_BEHAVIOR_MARKER
        },
        default_behavior_identity: input.default_behavior_identity.as_ref(),
    };
    let artifact = encode_typed_preserves(
        &canonical,
        PreservesSchema {
            schema_version: SEMANTIC_OPERATION_DESCRIPTOR_SCHEMA,
            root_label: SEMANTIC_OPERATION_DESCRIPTOR_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map_err(|error| {
        vec![
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_DESCRIPTOR,
                "descriptor.canonical_bytes",
                "semantic operation descriptor could not be canonically encoded",
            )
            .with_detail("error", error.to_string()),
        ]
    })?;
    Ok(EffectOperationIdentity::compute(&artifact.packed_bytes)
        .as_wire()
        .into())
}

/// Admit an operation reference that must carry an exact semantic identity.
///
/// A trusted-looking name alone is denied. The supplied identity must admit
/// into the effect-operation domain.
pub fn require_semantic_operation_identity(
    operation_name: &str,
    identity: Option<&Identity>,
) -> Result<Identity, Vec<Diagnostic>> {
    let Some(identity) = identity else {
        return Err(vec![
            Diagnostic::new(
                DiagnosticCode::SemanticOperationMismatch,
                REQUIREMENT_NAMES,
                "operation_reference.identity",
                "an operation name alone cannot discharge an operation reference",
            )
            .with_detail("operation_name", operation_name.to_string()),
        ]);
    };
    if TypedIdentity::<crate::identity_model::EffectOperation>::from_wire(
        WireIdentity::from_legacy(identity),
    )
    .is_err()
    {
        return Err(vec![
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_HANDLER,
                "operation_reference.identity",
                "operation reference must use the effect-operation identity domain",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("operation_name", operation_name.to_string()),
        ]);
    }
    Ok(identity.clone())
}

/// Check that a handler or binding covers the exact requested operation.
///
/// Same-named or same-shaped identities never match.
pub fn exact_semantic_operation_match(
    required: &Identity,
    supplied: &Identity,
) -> Result<(), Vec<Diagnostic>> {
    for (label, identity) in [("required", required), ("supplied", supplied)] {
        if TypedIdentity::<crate::identity_model::EffectOperation>::from_wire(
            WireIdentity::from_legacy(identity),
        )
        .is_err()
        {
            return Err(vec![
                Diagnostic::new(
                    DiagnosticCode::IdentityDomainMismatch,
                    REQUIREMENT_HANDLER,
                    "handler_match.identity",
                    "handler matching requires effect-operation domain identities",
                )
                .with_detail("which", label.to_string())
                .with_detail("actual", identity.domain.to_string()),
            ]);
        }
    }
    if required.hex != supplied.hex {
        return Err(vec![Diagnostic::new(
            DiagnosticCode::SemanticOperationMismatch,
            REQUIREMENT_HANDLER,
            "handler_match.operation_identity",
            "handler covers an older or different same-named operation; exact semantic operation identity required",
        )
        .with_detail("required", required.hex.as_str().to_string())
        .with_detail("supplied", supplied.hex.as_str().to_string())]);
    }
    Ok(())
}

/// Validate one directional semantic-operation compatibility artifact.
pub fn validate_semantic_operation_compatibility(
    input: &SemanticOperationCompatibilityInput,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if input.schema_version != SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_COMPATIBILITY,
                "compatibility.schema_version",
                "semantic operation compatibility schema version is unsupported",
            )
            .with_detail("actual", input.schema_version.clone())
            .with_detail(
                "expected",
                SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA.to_string(),
            ),
        );
    }
    push_domain_diagnostic(
        &input.source_operation_identity,
        IdentityDomain::EffectOperation,
        "compatibility.source_operation_identity",
        REQUIREMENT_COMPATIBILITY,
        &mut diagnostics,
    );
    push_domain_diagnostic(
        &input.target_operation_identity,
        IdentityDomain::EffectOperation,
        "compatibility.target_operation_identity",
        REQUIREMENT_COMPATIBILITY,
        &mut diagnostics,
    );
    if input.source_operation_identity.hex == input.target_operation_identity.hex {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::AmbiguousClaim,
            REQUIREMENT_COMPATIBILITY,
            "compatibility.source_operation_identity",
            "compatibility requires two different semantic operation identities",
        ));
    }
    if input.allowed_contexts.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_COMPATIBILITY,
            "compatibility.allowed_contexts",
            "compatibility must declare at least one allowed context",
        ));
    }
    let mut seen_contexts = std::collections::BTreeSet::new();
    for context in &input.allowed_contexts {
        if !SUPPORTED_COMPATIBILITY_CONTEXTS.contains(&context.as_str()) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UnsupportedArtifactClass,
                    REQUIREMENT_COMPATIBILITY,
                    "compatibility.allowed_contexts",
                    "compatibility context is unsupported",
                )
                .with_detail("actual", context.clone()),
            );
        }
        if !seen_contexts.insert(context.clone()) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::AmbiguousClaim,
                    REQUIREMENT_COMPATIBILITY,
                    "compatibility.allowed_contexts",
                    "compatibility contexts must be unique",
                )
                .with_detail("duplicate", context.clone()),
            );
        }
    }
    for claim in &input.semantic_claims {
        let normalized = claim.to_ascii_lowercase();
        if FORBIDDEN_COMPATIBILITY_CLAIMS
            .iter()
            .any(|forbidden| normalized.contains(forbidden))
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_BOUNDARY,
                    "compatibility.semantic_claims",
                    "compatibility claim oversteps the bounded substitution claim",
                )
                .with_detail("claim", claim.clone()),
            );
        }
    }
    for missing in REQUIRED_COMPATIBILITY_NON_CLAIMS
        .iter()
        .filter(|required| !input.non_claims.contains(&(*required).to_string()))
    {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MissingNonClaim,
                REQUIREMENT_BOUNDARY,
                "compatibility.non_claims",
                "compatibility artifact must preserve the bounded claim",
            )
            .with_detail("missing", (*missing).to_string()),
        );
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

/// Derive the canonical compatibility identity of a valid artifact.
pub fn semantic_operation_compatibility_identity(
    input: &SemanticOperationCompatibilityInput,
) -> Result<Identity, Vec<Diagnostic>> {
    let diagnostics = validate_semantic_operation_compatibility(input);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    let canonical = CanonicalSemanticCompatibility {
        schema_version: SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA,
        source_operation_identity: &input.source_operation_identity,
        target_operation_identity: &input.target_operation_identity,
        direction: input.direction.label(),
        allowed_contexts: &input.allowed_contexts,
        supporting_evidence_identities: &input.supporting_evidence_identities,
    };
    let artifact = encode_typed_preserves(
        &canonical,
        PreservesSchema {
            schema_version: SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA,
            root_label: SEMANTIC_OPERATION_COMPATIBILITY_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map_err(|error| {
        vec![
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_COMPATIBILITY,
                "compatibility.canonical_bytes",
                "compatibility artifact could not be canonically encoded",
            )
            .with_detail("error", error.to_string()),
        ]
    })?;
    Ok(CompatibilityIdentity::compute(&artifact.packed_bytes)
        .as_wire()
        .into())
}

/// Check one compatibility use request against the reviewed artifact.
///
/// The request direction must match the declared direction and the use
/// context must be admitted.
pub fn validate_compatibility_use(
    compatibility: &SemanticOperationCompatibilityInput,
    requested_from: &Identity,
    requested_to: &Identity,
    context: &str,
) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = validate_semantic_operation_compatibility(compatibility);
    for (label, identity) in [
        ("requested_from", requested_from),
        ("requested_to", requested_to),
    ] {
        if TypedIdentity::<crate::identity_model::EffectOperation>::from_wire(
            WireIdentity::from_legacy(identity),
        )
        .is_err()
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::IdentityDomainMismatch,
                    REQUIREMENT_COMPATIBILITY,
                    "compatibility_use.identity",
                    "compatibility use requires effect-operation domain identities",
                )
                .with_detail("which", label.to_string())
                .with_detail("actual", identity.domain.to_string()),
            );
        }
    }
    let direction_ok = match compatibility.direction {
        SemanticOperationCompatibilityDirection::OldToNew => {
            requested_from.hex == compatibility.source_operation_identity.hex
                && requested_to.hex == compatibility.target_operation_identity.hex
        }
        SemanticOperationCompatibilityDirection::NewToOld => {
            requested_from.hex == compatibility.target_operation_identity.hex
                && requested_to.hex == compatibility.source_operation_identity.hex
        }
    };
    if !direction_ok {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::SemanticOperationMismatch,
            REQUIREMENT_COMPATIBILITY,
            "compatibility_use.direction",
            "compatibility artifact does not permit substitution in this direction",
        ));
    }
    if !compatibility
        .allowed_contexts
        .iter()
        .any(|allowed| allowed == context)
    {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::SemanticOperationMismatch,
                REQUIREMENT_COMPATIBILITY,
                "compatibility_use.context",
                "compatibility artifact does not permit this use context",
            )
            .with_detail("requested", context.to_string()),
        );
    }
    diagnostics.sort();
    diagnostics.dedup();
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn push_domain_diagnostic(
    identity: &Identity,
    expected: IdentityDomain,
    field: &str,
    requirement: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if identity.domain != expected {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                requirement,
                field,
                "identity domain did not match the required semantic descriptor domain",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

    use super::*;
    use crate::identity::{BLAKE3_HEX_LENGTH, compute_identity};

    const FAMILY_BYTES: &[u8] = b"semantic family log";
    const OTHER_FAMILY_BYTES: &[u8] = b"semantic family clock";
    const PARAM_BYTES: &[u8] = b"parameter schema";
    const OTHER_PARAM_BYTES: &[u8] = b"parameter schema v2";
    const RESULT_BYTES: &[u8] = b"result schema";
    const BEHAVIOR_BYTES: &[u8] = b"default behavior material";
    const OTHER_BEHAVIOR_BYTES: &[u8] = b"default behavior material v2";
    const DEP_BYTES: &[u8] = b"dependency";
    const OTHER_DEP_BYTES: &[u8] = b"dependency two";

    fn family_identity() -> Identity {
        compute_identity(IdentityDomain::AlgebraicEffect, FAMILY_BYTES)
    }

    fn behavior_identity() -> Identity {
        compute_identity(IdentityDomain::CanonicalValue, BEHAVIOR_BYTES)
    }

    fn descriptor() -> SemanticEffectOperationDescriptorInput {
        SemanticEffectOperationDescriptorInput {
            schema_version: SEMANTIC_OPERATION_DESCRIPTOR_SCHEMA.to_string(),
            effect_family_identity: family_identity(),
            parameter_schema_identity: compute_identity(IdentityDomain::Adapter, PARAM_BYTES),
            result_schema_identity: compute_identity(IdentityDomain::Adapter, RESULT_BYTES),
            continuation_contract: "resumable-one-shot".to_string(),
            replay_class: "deterministic".to_string(),
            dependency_identities: vec![compute_identity(
                IdentityDomain::DependencyClosure,
                DEP_BYTES,
            )],
            default_behavior_identity: Some(behavior_identity()),
            abstract_operation_marker: false,
            operation_name: "Log.write".to_string(),
            aliases: vec!["log-write".to_string()],
            non_claims: REQUIRED_DESCRIPTOR_NON_CLAIMS
                .iter()
                .map(|claim| (*claim).to_string())
                .collect(),
        }
    }

    fn operation_identity(input: &SemanticEffectOperationDescriptorInput) -> Identity {
        semantic_operation_identity(input).unwrap()
    }

    fn assert_rekey(
        base: &Identity,
        mutate: impl FnOnce(&mut SemanticEffectOperationDescriptorInput),
    ) {
        let mut input = descriptor();
        mutate(&mut input);
        let rekeyed = operation_identity(&input);
        assert_ne!(base, &rekeyed);
    }

    fn compatibility(source: Identity, target: Identity) -> SemanticOperationCompatibilityInput {
        SemanticOperationCompatibilityInput {
            schema_version: SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA.to_string(),
            source_operation_identity: source,
            target_operation_identity: target,
            direction: SemanticOperationCompatibilityDirection::OldToNew,
            allowed_contexts: vec!["replay-only".to_string()],
            supporting_evidence_identities: vec![compute_identity(
                IdentityDomain::Receipt,
                b"review receipt",
            )],
            semantic_claims: vec!["bounded replay substitution only".to_string()],
            non_claims: REQUIRED_COMPATIBILITY_NON_CLAIMS
                .iter()
                .map(|claim| (*claim).to_string())
                .collect(),
        }
    }

    #[test]
    fn complete_descriptor_derives_effect_operation_identity() {
        let identity = operation_identity(&descriptor());
        assert_eq!(identity.domain, IdentityDomain::EffectOperation);
        assert_eq!(identity.algorithm, "blake3");
        assert_eq!(identity.hex.as_str().len(), BLAKE3_HEX_LENGTH);
        assert_eq!(identity, operation_identity(&descriptor()));
    }

    #[test]
    fn rename_and_metadata_do_not_change_identity() {
        let base = operation_identity(&descriptor());
        let mut renamed = descriptor();
        renamed.operation_name = "Log.append".to_string();
        renamed.aliases = vec!["renamed-alias".to_string()];
        renamed.non_claims.push("extra non-claim".to_string());
        assert_eq!(base, operation_identity(&renamed));
    }

    #[test]
    fn semantic_field_changes_rekey_the_operation() {
        let base = operation_identity(&descriptor());
        assert_rekey(&base, |input| {
            input.effect_family_identity =
                compute_identity(IdentityDomain::AlgebraicEffect, OTHER_FAMILY_BYTES)
        });
        assert_rekey(&base, |input| {
            input.parameter_schema_identity =
                compute_identity(IdentityDomain::Adapter, OTHER_PARAM_BYTES)
        });
        assert_rekey(&base, |input| {
            input.result_schema_identity =
                compute_identity(IdentityDomain::Adapter, OTHER_PARAM_BYTES)
        });
        assert_rekey(&base, |input| {
            input.continuation_contract = "linear-continuation".to_string()
        });
        assert_rekey(&base, |input| {
            input.replay_class = "recorded-replay".to_string()
        });
        assert_rekey(&base, |input| {
            input.dependency_identities.push(compute_identity(
                IdentityDomain::DependencyClosure,
                OTHER_DEP_BYTES,
            ))
        });
        assert_rekey(&base, |input| {
            input.dependency_identities = vec![
                compute_identity(IdentityDomain::DependencyClosure, OTHER_DEP_BYTES),
                compute_identity(IdentityDomain::DependencyClosure, DEP_BYTES),
            ]
        });
        assert_rekey(&base, |input| {
            input.default_behavior_identity = Some(compute_identity(
                IdentityDomain::CanonicalValue,
                OTHER_BEHAVIOR_BYTES,
            ))
        });
        assert_rekey(&base, |input| {
            input.default_behavior_identity = None;
            input.abstract_operation_marker = true;
        });
    }

    #[test]
    fn same_shaped_operations_in_different_families_differ() {
        let first = operation_identity(&descriptor());
        let mut other_family = descriptor();
        other_family.effect_family_identity =
            compute_identity(IdentityDomain::AlgebraicEffect, OTHER_FAMILY_BYTES);
        let second = operation_identity(&other_family);
        assert_ne!(first, second);
    }

    #[test]
    fn abstract_operation_is_admitted() {
        let mut input = descriptor();
        input.default_behavior_identity = None;
        input.abstract_operation_marker = true;
        let identity = semantic_operation_identity(&input);
        assert!(identity.is_ok());
        assert_eq!(identity.unwrap().domain, IdentityDomain::EffectOperation);
    }

    #[test]
    fn neither_behavior_marker_is_rejected() {
        let mut input = descriptor();
        input.default_behavior_identity = None;
        input.abstract_operation_marker = false;
        let diagnostics = validate_semantic_operation_descriptor(&input);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code,
            DiagnosticCode::IncompleteEffectSignature
        );
        assert!(semantic_operation_identity(&input).is_err());
    }

    #[test]
    fn both_behavior_markers_are_rejected() {
        let mut input = descriptor();
        input.abstract_operation_marker = true;
        let diagnostics = validate_semantic_operation_descriptor(&input);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code,
            DiagnosticCode::IncompleteEffectSignature
        );
    }

    #[test]
    fn wrong_domain_references_are_rejected() {
        let mut input = descriptor();
        input.effect_family_identity = compute_identity(IdentityDomain::Receipt, FAMILY_BYTES);
        input.parameter_schema_identity = compute_identity(IdentityDomain::Source, PARAM_BYTES);
        input.result_schema_identity = compute_identity(IdentityDomain::Source, RESULT_BYTES);
        input.default_behavior_identity =
            Some(compute_identity(IdentityDomain::Adapter, BEHAVIOR_BYTES));
        let diagnostics = validate_semantic_operation_descriptor(&input);
        assert_eq!(diagnostics.len(), 4);
        assert!(
            diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code == DiagnosticCode::IdentityDomainMismatch)
        );
    }

    #[test]
    fn unsupported_contract_replay_and_schema_are_rejected() {
        let mut input = descriptor();
        input.continuation_contract = "restartable".to_string();
        input.replay_class = "chaotic".to_string();
        input.schema_version = "kamacite.semantic.v0".to_string();
        let diagnostics = validate_semantic_operation_descriptor(&input);
        assert!(diagnostics.len() >= 3);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::ContinuationPolicyMismatch)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::UnsupportedArtifactClass)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::UnsupportedSchemaVersion)
        );
    }

    #[test]
    fn missing_required_non_claims_are_rejected() {
        let mut input = descriptor();
        input.non_claims.clear();
        let diagnostics = validate_semantic_operation_descriptor(&input);
        assert_eq!(diagnostics.len(), REQUIRED_DESCRIPTOR_NON_CLAIMS.len());
        assert!(
            diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
    }

    #[test]
    fn name_only_reference_is_denied() {
        let denied = require_semantic_operation_identity("Log.write", None);
        let diagnostics = denied.unwrap_err();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code,
            DiagnosticCode::SemanticOperationMismatch
        );

        let wrong_domain = compute_identity(IdentityDomain::Receipt, FAMILY_BYTES);
        let denied = require_semantic_operation_identity("Log.write", Some(&wrong_domain));
        assert_eq!(
            denied.unwrap_err()[0].code,
            DiagnosticCode::IdentityDomainMismatch
        );

        let exact = operation_identity(&descriptor());
        let admitted = require_semantic_operation_identity("Log.write", Some(&exact));
        assert!(admitted.is_ok());
    }

    #[test]
    fn exact_match_is_accepted_and_drift_is_denied() {
        let identity = operation_identity(&descriptor());
        assert!(exact_semantic_operation_match(&identity, &identity).is_ok());

        let mut drifted = descriptor();
        drifted.default_behavior_identity = Some(compute_identity(
            IdentityDomain::CanonicalValue,
            OTHER_BEHAVIOR_BYTES,
        ));
        let older = operation_identity(&drifted);
        let diagnostics = exact_semantic_operation_match(&identity, &older).unwrap_err();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code,
            DiagnosticCode::SemanticOperationMismatch
        );
        assert_eq!(diagnostics[0].field, "handler_match.operation_identity");
    }

    #[test]
    fn compatibility_validates_and_uses_the_compatibility_domain() {
        let source = operation_identity(&descriptor());
        let mut drifted = descriptor();
        drifted.replay_class = "recorded-replay".to_string();
        let target = operation_identity(&drifted);
        let artifact = compatibility(source.clone(), target.clone());
        assert!(validate_semantic_operation_compatibility(&artifact).is_empty());
        let identity = semantic_operation_compatibility_identity(&artifact).unwrap();
        assert_eq!(identity.domain, IdentityDomain::Compatibility);

        let mut other = artifact.clone();
        other.direction = SemanticOperationCompatibilityDirection::NewToOld;
        assert_ne!(
            identity,
            semantic_operation_compatibility_identity(&other).unwrap()
        );
    }

    #[test]
    fn replay_only_use_is_permitted() {
        let source = operation_identity(&descriptor());
        let mut drifted = descriptor();
        drifted.replay_class = "recorded-replay".to_string();
        let target = operation_identity(&drifted);
        let artifact = compatibility(source.clone(), target.clone());
        assert!(validate_compatibility_use(&artifact, &source, &target, "replay-only").is_ok());
    }

    #[test]
    fn wrong_direction_use_is_denied() {
        let source = operation_identity(&descriptor());
        let mut drifted = descriptor();
        drifted.replay_class = "recorded-replay".to_string();
        let target = operation_identity(&drifted);
        let artifact = compatibility(source.clone(), target.clone());
        let diagnostics =
            validate_compatibility_use(&artifact, &target, &source, "replay-only").unwrap_err();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].field, "compatibility_use.direction");
    }

    #[test]
    fn undeclared_live_host_use_is_denied() {
        let source = operation_identity(&descriptor());
        let mut drifted = descriptor();
        drifted.replay_class = "recorded-replay".to_string();
        let target = operation_identity(&drifted);
        let artifact = compatibility(source.clone(), target.clone());
        let diagnostics =
            validate_compatibility_use(&artifact, &source, &target, "live-host").unwrap_err();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].field, "compatibility_use.context");

        let diagnostics =
            validate_compatibility_use(&artifact, &source, &target, "cache").unwrap_err();
        assert_eq!(
            diagnostics[0].code,
            DiagnosticCode::SemanticOperationMismatch
        );
    }

    #[test]
    fn same_identity_compatibility_is_denied() {
        let identity = operation_identity(&descriptor());
        let artifact = compatibility(identity.clone(), identity.clone());
        let diagnostics = validate_semantic_operation_compatibility(&artifact);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, DiagnosticCode::AmbiguousClaim);
    }

    #[test]
    fn overclaiming_compatibility_is_denied() {
        let source = operation_identity(&descriptor());
        let mut drifted = descriptor();
        drifted.replay_class = "recorded-replay".to_string();
        let target = operation_identity(&drifted);
        let mut artifact = compatibility(source, target);
        artifact.semantic_claims =
            vec!["the two operations are semantically equivalent for all callers".to_string()];
        let diagnostics = validate_semantic_operation_compatibility(&artifact);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, DiagnosticCode::SemanticOverclaim);
    }

    #[test]
    fn missing_compatibility_non_claims_and_contexts_are_denied() {
        let source = operation_identity(&descriptor());
        let mut drifted = descriptor();
        drifted.replay_class = "recorded-replay".to_string();
        let target = operation_identity(&drifted);
        let mut artifact = compatibility(source, target);
        artifact.non_claims.clear();
        artifact.allowed_contexts = Vec::new();
        let diagnostics = validate_semantic_operation_compatibility(&artifact);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingInput)
        );
    }

    #[test]
    fn dto_round_trips_through_json() {
        let input = descriptor();
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"effect_family_identity\""));
        let parsed: SemanticEffectOperationDescriptorInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, input);

        let source = operation_identity(&input);
        let mut drifted = input.clone();
        drifted.replay_class = "recorded-replay".to_string();
        let target = operation_identity(&drifted);
        let artifact = compatibility(source, target);
        let json = serde_json::to_string(&artifact).unwrap();
        assert!(json.contains("\"old-to-new\""));
        let parsed: SemanticOperationCompatibilityInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, artifact);
    }
}
