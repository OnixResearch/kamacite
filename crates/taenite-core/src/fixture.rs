use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{Identity, IdentityDomain, compute_identity};
use crate::receipt::{ReceiptBody, ReceiptEnvelope};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const SUPPORTED_SCHEMA_VERSION: u32 = 1;
const REQUIREMENT_SCHEMA_VERSION: &str =
    "r[taenite.lossless_ir.schema_vocabulary.unsupported_version]";
const REQUIREMENT_SOURCE_HASH: &str = "r[taenite.lossless_ir.source_preservation.hash_mismatch]";
const REQUIREMENT_SOURCE_BYTES: &str =
    "r[taenite.lossless_ir.source_envelope_vocabulary.missing_bytes]";
const REQUIREMENT_NON_CLAIMS: &str = "r[taenite.lossless_ir.non_claims.receipt]";
const REQUIREMENT_TYPED_DOMAIN: &str =
    "r[taenite.lossless_ir.typed_identity_domains.projection_not_canonical]";
const REQUIREMENT_CAPABILITIES: &str =
    "r[taenite.rust_determinism.explicit_capabilities.filesystem]";
const REQUIREMENT_TARGET_OVERCLAIM: &str = "r[taenite.lossless_ir.wasm_target_boundary.not_ir]";
const REQUIREMENT_UNSUPPORTED_CLAIM: &str =
    "r[taenite.lossless_ir.losslessness_classes.arbitrary_source_denied]";
const REQUIREMENT_FIXTURE_METADATA: &str =
    "r[taenite.lossless_ir.fixture_corpus.requirement_coverage]";
const REQUIREMENT_EXPECTED_VERDICT: &str = "r[taenite.lossless_ir.negative_fixtures]";
const REQUIREMENT_PATH_ESCAPE: &str =
    "r[taenite.rust_determinism.capability_shells.escape_fixtures]";
const REQUIREMENT_ALGEBRAIC_VERSION: &str =
    "r[taenite.algebraic_effects.ir_boundary.unsupported_version]";
const REQUIREMENT_EFFECT_SCHEMA: &str =
    "r[taenite.algebraic_effects.signature_vocabulary.missing_schema]";
const REQUIREMENT_UNDECLARED_EFFECT: &str =
    "r[taenite.algebraic_effects.row_summary_vocabulary.undeclared_operation]";
const REQUIREMENT_OPEN_ROW: &str =
    "r[taenite.algebraic_effects.row_summary_vocabulary.open_row_not_pure]";
const REQUIREMENT_HANDLER_NAME: &str =
    "r[taenite.algebraic_effects.handler_descriptor.name_not_discharge]";
const REQUIREMENT_UNKNOWN_EFFECT: &str =
    "r[taenite.algebraic_effects.handler_receipts.unknown_effect_denied]";
const REQUIREMENT_CONTINUATION: &str = "r[taenite.algebraic_effects.continuation_policy.mismatch]";
const REQUIREMENT_HOST_CONFUSION: &str =
    "r[taenite.algebraic_effects.host_boundary.operation_not_authority]";
const REQUIREMENT_REPLAY: &str = "r[taenite.algebraic_effects.replay_profile.recorded_log]";
const REQUIREMENT_STACK_BOUNDARY: &str =
    "r[taenite.lossless_ir.stack_evidence_boundary.octet_checks]";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValidationVerdict {
    Accepted,
    Rejected,
}

impl ValidationVerdict {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FixtureClass {
    Positive,
    Negative,
    Property,
    GoldenReceipt,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactClass {
    SourceEnvelope,
    CanonicalValue,
    AdapterDescriptor,
    AdapterReceipt,
    TargetArtifact,
    Receipt,
    UnsupportedClaim,
    CapabilityShell,
    AlgebraicEffect,
    StackEvidence,
}

impl ArtifactClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SourceEnvelope => "source-envelope",
            Self::CanonicalValue => "canonical-value",
            Self::AdapterDescriptor => "adapter-descriptor",
            Self::AdapterReceipt => "adapter-receipt",
            Self::TargetArtifact => "target-artifact",
            Self::Receipt => "receipt",
            Self::UnsupportedClaim => "unsupported-claim",
            Self::CapabilityShell => "capability-shell",
            Self::AlgebraicEffect => "algebraic-effect",
            Self::StackEvidence => "stack-evidence",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixtureCase {
    pub id: String,
    pub fixture_class: FixtureClass,
    pub artifact_class: ArtifactClass,
    pub requirement_ids: Vec<String>,
    pub non_claims: Vec<String>,
    pub expected_verdict: ValidationVerdict,
    pub checked_laws: Vec<String>,
    pub payload: FixturePayload,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum FixturePayload {
    SourceEnvelope(SourceEnvelopeInput),
    CanonicalValue(CanonicalValueInput),
    AdapterReceipt(AdapterReceiptInput),
    TargetArtifact(TargetArtifactInput),
    UnsupportedClaim(UnsupportedClaimInput),
    IdentityUse(IdentityUseInput),
    CapabilityShell(CapabilityShellInput),
    StackEvidence(StackEvidenceInput),
    AlgebraicEffect(Box<AlgebraicEffectInput>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceEnvelopeInput {
    pub schema_version: u32,
    pub language: String,
    pub media_type: String,
    pub parser_identity: String,
    pub source_bytes: Vec<u8>,
    pub declared_identity: Identity,
    pub aliases: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CanonicalValueInput {
    pub schema_version: u32,
    pub canonical_bytes: Vec<u8>,
    pub declared_identity: Identity,
    pub projection_identity: Option<Identity>,
    pub projection_claims_authority: bool,
    pub aliases: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdapterReceiptInput {
    pub adapter_kind: String,
    pub has_conformance_entry: bool,
    pub declared_capabilities: Vec<String>,
    pub observed_effects: Vec<String>,
    pub claims_canonical_ir: bool,
    pub checked_laws: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TargetArtifactInput {
    pub target_kind: String,
    pub output_bytes: Vec<u8>,
    pub declared_identity: Identity,
    pub claims_canonical_ir: bool,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnsupportedClaimInput {
    pub claim_class: String,
    pub claim_text: String,
    pub preserve_source_only: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IdentityUseInput {
    pub required_domain: IdentityDomain,
    pub supplied_identity: Identity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PathObservation {
    NormalRelative,
    ParentTraversal,
    AbsolutePath,
    SymlinkEscape,
    UndeclaredFilesystem,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CapabilityShellInput {
    pub declared_filesystem: bool,
    pub path: String,
    pub observation: PathObservation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StackEvidenceInput {
    pub role: String,
    pub bounded_claim: String,
    pub broadens_claim: bool,
    pub referenced_identities: Vec<Identity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EffectOperation {
    pub operation_id: String,
    pub display_name: String,
    pub parameter_schema: Option<String>,
    pub result_schema: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandlerDescriptorInput {
    pub name: String,
    pub handled_operations: Vec<String>,
    pub residual_operations: Vec<String>,
    pub continuation_policy: String,
    pub claims_discharge: bool,
    pub host_capability_requirements: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandlerReceiptInput {
    pub discharged_operations: Vec<String>,
    pub residual_operations: Vec<String>,
    pub continuation_policy: String,
    pub continuation_evidence: Option<String>,
    pub observed_operations: Vec<String>,
    pub checked_laws: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayProfileInput {
    pub replay_class: String,
    pub effect_log_bytes: Vec<u8>,
    pub effect_log_identity: Option<Identity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AlgebraicEffectInput {
    pub schema_version: u32,
    pub canonical_bytes: Vec<u8>,
    pub declared_identity: Identity,
    pub signatures: Vec<EffectOperation>,
    pub input_row: Vec<String>,
    pub row_open: bool,
    pub claimed_pure: bool,
    pub handler: Option<HandlerDescriptorInput>,
    pub receipt: Option<HandlerReceiptInput>,
    pub replay: Option<ReplayProfileInput>,
    pub claimed_host_authority: Vec<String>,
    pub admitted_host_capabilities: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixtureSuite {
    pub fixtures: Vec<FixtureCase>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FixtureResult {
    pub id: String,
    pub artifact_class: ArtifactClass,
    pub expected_verdict: ValidationVerdict,
    pub observed_verdict: ValidationVerdict,
    pub expectation_met: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub receipt: Option<ReceiptEnvelope>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FixtureSuiteReport {
    pub passed: bool,
    pub results: Vec<FixtureResult>,
}

#[derive(Clone, Debug, Default)]
struct ValidationAccumulator {
    diagnostics: Vec<Diagnostic>,
    input_identities: BTreeMap<String, Identity>,
    output_identities: BTreeMap<String, Identity>,
    checked_laws: Vec<String>,
}

pub fn validate_fixture_suite(suite: &FixtureSuite) -> FixtureSuiteReport {
    let mut seen_ids = BTreeSet::new();
    let mut duplicate_ids = BTreeSet::new();
    for fixture in &suite.fixtures {
        if !seen_ids.insert(fixture.id.clone()) {
            duplicate_ids.insert(fixture.id.clone());
        }
    }

    let mut results = Vec::new();
    for fixture in &suite.fixtures {
        let mut result = validate_fixture(fixture);
        if duplicate_ids.contains(&fixture.id) {
            result = result_with_added_diagnostic(
                fixture,
                result,
                Diagnostic::new(
                    DiagnosticCode::DuplicateFixture,
                    REQUIREMENT_FIXTURE_METADATA,
                    "fixture.id",
                    "duplicate fixture identity rejected before trust",
                ),
            );
        }
        results.push(result);
    }

    let passed = results.iter().all(|result| result.expectation_met);
    FixtureSuiteReport { passed, results }
}

pub fn validate_fixture(fixture: &FixtureCase) -> FixtureResult {
    let mut accumulator = ValidationAccumulator::default();
    validate_common_metadata(fixture, &mut accumulator);

    match &fixture.payload {
        FixturePayload::SourceEnvelope(input) => validate_source_envelope(input, &mut accumulator),
        FixturePayload::CanonicalValue(input) => validate_canonical_value(input, &mut accumulator),
        FixturePayload::AdapterReceipt(input) => validate_adapter_receipt(input, &mut accumulator),
        FixturePayload::TargetArtifact(input) => validate_target_artifact(input, &mut accumulator),
        FixturePayload::UnsupportedClaim(input) => {
            validate_unsupported_claim(input, &mut accumulator)
        }
        FixturePayload::IdentityUse(input) => validate_identity_use(input, &mut accumulator),
        FixturePayload::CapabilityShell(input) => {
            validate_capability_shell(input, &mut accumulator)
        }
        FixturePayload::StackEvidence(input) => validate_stack_evidence(input, &mut accumulator),
        FixturePayload::AlgebraicEffect(input) => {
            validate_algebraic_effect(input, &mut accumulator)
        }
    }

    accumulator
        .checked_laws
        .extend(fixture.checked_laws.clone());
    finish_result(fixture, accumulator)
}

fn validate_common_metadata(fixture: &FixtureCase, accumulator: &mut ValidationAccumulator) {
    if fixture.id.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_FIXTURE_METADATA,
            "fixture.id",
            "fixture id is required",
        ));
    }
    if fixture.requirement_ids.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_FIXTURE_METADATA,
            "requirement_ids",
            "at least one requirement id is required",
        ));
    }
    if fixture.non_claims.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_NON_CLAIMS,
            "non_claims",
            "fixture must carry explicit non-claims",
        ));
    }
}

fn validate_source_envelope(input: &SourceEnvelopeInput, accumulator: &mut ValidationAccumulator) {
    validate_supported_schema(input.schema_version, accumulator);
    validate_identity_domain(
        &input.declared_identity,
        IdentityDomain::Source,
        "source.declared_identity",
        accumulator,
    );

    if input.source_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingSourceBytes,
            REQUIREMENT_SOURCE_BYTES,
            "source.source_bytes",
            "source envelope must preserve original bytes",
        ));
        return;
    }

    let computed = compute_identity(IdentityDomain::Source, &input.source_bytes);
    accumulator.input_identities.insert(
        "declared-source".to_string(),
        input.declared_identity.clone(),
    );
    accumulator
        .output_identities
        .insert("computed-source".to_string(), computed.clone());
    if input.declared_identity.domain == IdentityDomain::Source
        && input.declared_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_SOURCE_HASH,
            "source.declared_identity.hex",
            "declared source identity does not match source bytes",
        ));
    }
}

fn validate_canonical_value(input: &CanonicalValueInput, accumulator: &mut ValidationAccumulator) {
    validate_supported_schema(input.schema_version, accumulator);
    validate_identity_domain(
        &input.declared_identity,
        IdentityDomain::CanonicalValue,
        "canonical.declared_identity",
        accumulator,
    );
    let computed = compute_identity(IdentityDomain::CanonicalValue, &input.canonical_bytes);
    accumulator.input_identities.insert(
        "declared-canonical".to_string(),
        input.declared_identity.clone(),
    );
    accumulator
        .output_identities
        .insert("computed-canonical".to_string(), computed.clone());
    if input.declared_identity.domain == IdentityDomain::CanonicalValue
        && input.declared_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            "r[taenite.lossless_ir.canonical_value_vocabulary.stale_hash]",
            "canonical.declared_identity.hex",
            "declared canonical identity does not match canonical bytes",
        ));
    }
    if let Some(projection_identity) = &input.projection_identity {
        validate_identity_domain(
            projection_identity,
            IdentityDomain::Projection,
            "canonical.projection_identity",
            accumulator,
        );
        accumulator
            .input_identities
            .insert("projection".to_string(), projection_identity.clone());
    }
    if input.projection_claims_authority {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::LossyProjectionOverclaim,
            "r[taenite.lossless_ir.format_strategy.json_projection]",
            "canonical.projection_claims_authority",
            "compatibility projection cannot replace canonical Preserves bytes",
        ));
    }
}

fn validate_adapter_receipt(input: &AdapterReceiptInput, accumulator: &mut ValidationAccumulator) {
    if !input.has_conformance_entry {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            "r[taenite.lossless_ir.adapter_conformance.untrusted_without_entry]",
            "adapter.has_conformance_entry",
            "adapter-specific claims require a conformance entry",
        ));
    }
    let declared: BTreeSet<&String> = input.declared_capabilities.iter().collect();
    for observed in &input.observed_effects {
        if !declared.contains(observed) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredCapability,
                    REQUIREMENT_CAPABILITIES,
                    "adapter.observed_effects",
                    "observed host effect was not declared by adapter descriptor",
                )
                .with_detail("effect", observed.clone()),
            );
        }
    }
    if input.claims_canonical_ir {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::CanonicalIrOverclaim,
            REQUIREMENT_TARGET_OVERCLAIM,
            "adapter.claims_canonical_ir",
            "adapter or target artifact cannot replace Preserves canonical IR",
        ));
    }
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_target_artifact(input: &TargetArtifactInput, accumulator: &mut ValidationAccumulator) {
    validate_identity_domain(
        &input.declared_identity,
        IdentityDomain::TargetArtifact,
        "target.declared_identity",
        accumulator,
    );
    let computed = compute_identity(IdentityDomain::TargetArtifact, &input.output_bytes);
    accumulator.input_identities.insert(
        "declared-target".to_string(),
        input.declared_identity.clone(),
    );
    accumulator
        .output_identities
        .insert("computed-target".to_string(), computed.clone());
    if input.declared_identity.domain == IdentityDomain::TargetArtifact
        && input.declared_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            "r[taenite.lossless_ir.target_artifact_vocabulary.toolchain_provenance]",
            "target.declared_identity.hex",
            "declared target identity does not match target artifact bytes",
        ));
    }
    if input.metadata.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            "r[taenite.lossless_ir.llvm_target_boundary.artifact_receipt]",
            "target.metadata",
            "target artifact metadata is required for bounded provenance",
        ));
    }
    if input.claims_canonical_ir {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::CanonicalIrOverclaim,
            REQUIREMENT_TARGET_OVERCLAIM,
            "target.claims_canonical_ir",
            "compiled target artifact cannot be Taenite canonical IR",
        ));
    }
}

fn validate_unsupported_claim(
    input: &UnsupportedClaimInput,
    accumulator: &mut ValidationAccumulator,
) {
    let unsupported = input.claim_class == "unsupported-source-translation"
        || input.claim_class == "semantic-equivalence"
        || input.claim_text.contains("lossless arbitrary");
    if unsupported {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedClaim,
            REQUIREMENT_UNSUPPORTED_CLAIM,
            "claim.claim_class",
            "unsupported lossless translation or semantic-equivalence claim rejected",
        ));
    }
    if !input.preserve_source_only && unsupported {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::AmbiguousClaim,
            "r[taenite.lossless_ir.total_diagnostics.malformed]",
            "claim.preserve_source_only",
            "unsupported source translation must remain source-envelope-only evidence",
        ));
    }
}

fn validate_identity_use(input: &IdentityUseInput, accumulator: &mut ValidationAccumulator) {
    validate_identity_domain(
        &input.supplied_identity,
        input.required_domain.clone(),
        "identity_use.supplied_identity",
        accumulator,
    );
    accumulator
        .input_identities
        .insert("supplied".to_string(), input.supplied_identity.clone());
}

fn validate_capability_shell(
    input: &CapabilityShellInput,
    accumulator: &mut ValidationAccumulator,
) {
    if !input.declared_filesystem {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UndeclaredCapability,
            REQUIREMENT_CAPABILITIES,
            "capability.declared_filesystem",
            "filesystem-backed shell fixture lacks declared filesystem authority",
        ));
    }
    match input.observation {
        PathObservation::NormalRelative => {}
        PathObservation::ParentTraversal
        | PathObservation::AbsolutePath
        | PathObservation::SymlinkEscape
        | PathObservation::UndeclaredFilesystem => accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::PathEscape,
                REQUIREMENT_PATH_ESCAPE,
                "capability.path",
                "shell path admission rejected deterministic escape or undeclared authority case",
            )
            .with_detail("path", input.path.clone()),
        ),
    }
}

fn validate_stack_evidence(input: &StackEvidenceInput, accumulator: &mut ValidationAccumulator) {
    for identity in &input.referenced_identities {
        accumulator.input_identities.insert(
            format!("{}:{}", input.role, identity.domain),
            identity.clone(),
        );
    }
    if input.broadens_claim {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::AmbiguousClaim,
            REQUIREMENT_STACK_BOUNDARY,
            "stack.broadens_claim",
            "stack evidence export broadened Taenite bounded claim",
        ));
    }
    if input.role.is_empty() || input.bounded_claim.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_STACK_BOUNDARY,
            "stack.role",
            "stack evidence role and bounded claim are required",
        ));
    }
}

fn validate_algebraic_effect(
    input: &AlgebraicEffectInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.schema_version != SUPPORTED_SCHEMA_VERSION {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedAlgebraicEffectVersion,
            REQUIREMENT_ALGEBRAIC_VERSION,
            "algebraic.schema_version",
            "unsupported algebraic-effect schema version rejected",
        ));
    }
    validate_identity_domain(
        &input.declared_identity,
        IdentityDomain::AlgebraicEffect,
        "algebraic.declared_identity",
        accumulator,
    );
    let computed = compute_identity(IdentityDomain::AlgebraicEffect, &input.canonical_bytes);
    accumulator.input_identities.insert(
        "declared-algebraic-effect".to_string(),
        input.declared_identity.clone(),
    );
    accumulator
        .output_identities
        .insert("computed-algebraic-effect".to_string(), computed.clone());
    if input.declared_identity.domain == IdentityDomain::AlgebraicEffect
        && input.declared_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::AlgebraicEffectHashMismatch,
            "r[taenite.algebraic_effects.fixture_corpus.negative_stale_identity]",
            "algebraic.declared_identity.hex",
            "declared algebraic-effect identity does not match canonical bytes",
        ));
    }

    let declared_operations = validate_effect_operations(&input.signatures, accumulator);
    validate_effect_row(input, &declared_operations, accumulator);
    validate_host_authority(input, accumulator);
    validate_handler(input, &declared_operations, accumulator);
    validate_handler_receipt(input, &declared_operations, accumulator);
    validate_replay(input, accumulator);
}

fn validate_effect_operations(
    signatures: &[EffectOperation],
    accumulator: &mut ValidationAccumulator,
) -> BTreeSet<String> {
    let mut declared = BTreeSet::new();
    for operation in signatures {
        declared.insert(operation.operation_id.clone());
        if operation.parameter_schema.is_none() || operation.result_schema.is_none() {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::IncompleteEffectSignature,
                    REQUIREMENT_EFFECT_SCHEMA,
                    "algebraic.signatures",
                    "effect operation requires parameter and result schemas",
                )
                .with_detail("operation", operation.operation_id.clone()),
            );
        }
    }
    declared
}

fn validate_effect_row(
    input: &AlgebraicEffectInput,
    declared_operations: &BTreeSet<String>,
    accumulator: &mut ValidationAccumulator,
) {
    for operation in &input.input_row {
        if !declared_operations.contains(operation) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredEffectOperation,
                    REQUIREMENT_UNDECLARED_EFFECT,
                    "algebraic.input_row",
                    "effect row references an undeclared operation",
                )
                .with_detail("operation", operation.clone()),
            );
        }
    }
    if input.row_open && input.claimed_pure {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::OpenRowNotPure,
            REQUIREMENT_OPEN_ROW,
            "algebraic.claimed_pure",
            "open effect rows cannot be inferred pure",
        ));
    }
}

fn validate_host_authority(input: &AlgebraicEffectInput, accumulator: &mut ValidationAccumulator) {
    let admitted: BTreeSet<&String> = input.admitted_host_capabilities.iter().collect();
    for authority in &input.claimed_host_authority {
        if !admitted.contains(authority) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::HostCapabilityConfusion,
                    REQUIREMENT_HOST_CONFUSION,
                    "algebraic.claimed_host_authority",
                    "algebraic operation declaration does not grant host authority",
                )
                .with_detail("authority", authority.clone()),
            );
        }
    }
}

fn validate_handler(
    input: &AlgebraicEffectInput,
    declared_operations: &BTreeSet<String>,
    accumulator: &mut ValidationAccumulator,
) {
    let Some(handler) = &input.handler else {
        return;
    };
    if handler.claims_discharge && handler.handled_operations.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::HandlerNameNotDischarge,
            REQUIREMENT_HANDLER_NAME,
            "algebraic.handler.handled_operations",
            "handler names cannot prove discharge without operation identities",
        ));
    }
    for operation in &handler.handled_operations {
        if !declared_operations.contains(operation) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredEffectOperation,
                    REQUIREMENT_UNDECLARED_EFFECT,
                    "algebraic.handler.handled_operations",
                    "handler references an undeclared operation",
                )
                .with_detail("operation", operation.clone()),
            );
        }
    }
    let handled: BTreeSet<&String> = handler.handled_operations.iter().collect();
    let residual: BTreeSet<&String> = handler.residual_operations.iter().collect();
    for operation in &input.input_row {
        if !handled.contains(operation) && !residual.contains(operation) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UnknownEffectOperation,
                    "r[taenite.algebraic_effects.handler_descriptor.residual_row]",
                    "algebraic.handler.residual_operations",
                    "unhandled input operation is missing from residual row",
                )
                .with_detail("operation", operation.clone()),
            );
        }
    }
}

fn validate_handler_receipt(
    input: &AlgebraicEffectInput,
    declared_operations: &BTreeSet<String>,
    accumulator: &mut ValidationAccumulator,
) {
    let Some(receipt) = &input.receipt else {
        return;
    };
    let input_row: BTreeSet<&String> = input.input_row.iter().collect();
    for operation in &receipt.observed_operations {
        if !declared_operations.contains(operation) || !input_row.contains(operation) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UnknownEffectOperation,
                    REQUIREMENT_UNKNOWN_EFFECT,
                    "algebraic.receipt.observed_operations",
                    "handler receipt observed an unknown effect operation",
                )
                .with_detail("operation", operation.clone()),
            );
        }
    }
    if let Some(handler) = &input.handler {
        let handled: BTreeSet<&String> = handler.handled_operations.iter().collect();
        for operation in &receipt.discharged_operations {
            if !handled.contains(operation) || !input_row.contains(operation) {
                accumulator.diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::UnknownEffectOperation,
                        REQUIREMENT_UNKNOWN_EFFECT,
                        "algebraic.receipt.discharged_operations",
                        "handler receipt discharged an operation outside the descriptor or row",
                    )
                    .with_detail("operation", operation.clone()),
                );
            }
        }
        if handler.continuation_policy != receipt.continuation_policy {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::ContinuationPolicyMismatch,
                REQUIREMENT_CONTINUATION,
                "algebraic.receipt.continuation_policy",
                "handler descriptor and receipt continuation policies differ",
            ));
        }
        if handler.continuation_policy == "no-continuation"
            && receipt.continuation_evidence.is_some()
        {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::ContinuationPolicyMismatch,
                REQUIREMENT_CONTINUATION,
                "algebraic.receipt.continuation_evidence",
                "no-continuation handler receipt recorded continuation evidence",
            ));
        }
        if handler.continuation_policy == "single-use-continuation"
            && receipt.continuation_evidence.is_none()
        {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::ContinuationPolicyMismatch,
                "r[taenite.algebraic_effects.continuation_policy.single_use_evidence]",
                "algebraic.receipt.continuation_evidence",
                "single-use continuation claim requires matching evidence",
            ));
        }
    }
    accumulator
        .checked_laws
        .extend(receipt.checked_laws.clone());
}

fn validate_replay(input: &AlgebraicEffectInput, accumulator: &mut ValidationAccumulator) {
    let Some(replay) = &input.replay else {
        return;
    };
    match replay.replay_class.as_str() {
        "recorded-replay" => {
            let Some(identity) = &replay.effect_log_identity else {
                accumulator.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::MissingReplayLogIdentity,
                    REQUIREMENT_REPLAY,
                    "algebraic.replay.effect_log_identity",
                    "recorded replay must bind effect-log identity",
                ));
                return;
            };
            validate_identity_domain(
                identity,
                IdentityDomain::EffectLog,
                "algebraic.replay.effect_log_identity",
                accumulator,
            );
            let computed = compute_identity(IdentityDomain::EffectLog, &replay.effect_log_bytes);
            accumulator
                .input_identities
                .insert("declared-effect-log".to_string(), identity.clone());
            accumulator
                .output_identities
                .insert("computed-effect-log".to_string(), computed.clone());
            if identity.domain == IdentityDomain::EffectLog && identity.hex != computed.hex {
                accumulator.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::StaleIdentity,
                    REQUIREMENT_REPLAY,
                    "algebraic.replay.effect_log_identity.hex",
                    "recorded replay log identity does not match log bytes",
                ));
            }
        }
        "non-replayable" => {
            if replay.effect_log_identity.is_some() {
                accumulator.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::NonReplayableEvidenceMisuse,
                    "r[taenite.algebraic_effects.replay_profile.non_replayable]",
                    "algebraic.replay.effect_log_identity",
                    "non-replayable effects cannot be accepted as replay evidence",
                ));
            }
        }
        _ => {}
    }
}

fn validate_supported_schema(schema_version: u32, accumulator: &mut ValidationAccumulator) {
    if schema_version != SUPPORTED_SCHEMA_VERSION {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedSchemaVersion,
            REQUIREMENT_SCHEMA_VERSION,
            "schema_version",
            "unsupported schema version rejected",
        ));
    }
}

fn validate_identity_domain(
    identity: &Identity,
    expected: IdentityDomain,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) {
    if identity.validate_domain(expected.clone()).is_err() {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_TYPED_DOMAIN,
                field,
                "identity domain did not match required typed domain",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

fn finish_result(fixture: &FixtureCase, mut accumulator: ValidationAccumulator) -> FixtureResult {
    accumulator.diagnostics.sort();
    accumulator.checked_laws.sort();
    accumulator.checked_laws.dedup();

    let observed_verdict = if accumulator.diagnostics.is_empty() {
        ValidationVerdict::Accepted
    } else {
        ValidationVerdict::Rejected
    };
    let mut expectation_met = observed_verdict == fixture.expected_verdict;
    if !expectation_met {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::WrongExpectedVerdict,
                REQUIREMENT_EXPECTED_VERDICT,
                "expected_verdict",
                "fixture observed verdict did not match expected verdict",
            )
            .with_detail("observed", observed_verdict.as_str())
            .with_detail("expected", fixture.expected_verdict.as_str()),
        );
        accumulator.diagnostics.sort();
    }

    let evidence_verdict = if expectation_met {
        observed_verdict.clone()
    } else {
        expectation_met = false;
        ValidationVerdict::Rejected
    };
    let receipt = build_receipt(fixture, &evidence_verdict, &accumulator);

    FixtureResult {
        id: fixture.id.clone(),
        artifact_class: fixture.artifact_class.clone(),
        expected_verdict: fixture.expected_verdict.clone(),
        observed_verdict: evidence_verdict,
        expectation_met,
        diagnostics: accumulator.diagnostics,
        receipt,
    }
}

fn result_with_added_diagnostic(
    fixture: &FixtureCase,
    mut result: FixtureResult,
    diagnostic: Diagnostic,
) -> FixtureResult {
    result.diagnostics.push(diagnostic);
    result.diagnostics.sort();
    result.observed_verdict = ValidationVerdict::Rejected;
    result.expectation_met = fixture.expected_verdict == ValidationVerdict::Rejected;

    let mut accumulator = ValidationAccumulator {
        diagnostics: result.diagnostics.clone(),
        input_identities: BTreeMap::new(),
        output_identities: BTreeMap::new(),
        checked_laws: fixture.checked_laws.clone(),
    };
    accumulator.checked_laws.sort();
    accumulator.checked_laws.dedup();
    result.receipt = build_receipt(fixture, &result.observed_verdict, &accumulator);
    result
}

fn build_receipt(
    fixture: &FixtureCase,
    verdict: &ValidationVerdict,
    accumulator: &ValidationAccumulator,
) -> Option<ReceiptEnvelope> {
    let mut body = ReceiptBody::new(
        fixture.id.clone(),
        fixture.artifact_class.as_str(),
        verdict.as_str(),
        sorted_strings(fixture.requirement_ids.clone()),
        sorted_strings(fixture.non_claims.clone()),
    );
    body.input_identities = accumulator.input_identities.clone();
    body.output_identities = accumulator.output_identities.clone();
    body.checked_laws = accumulator.checked_laws.clone();
    body.diagnostics = accumulator.diagnostics.clone();
    ReceiptEnvelope::finalize(body).ok()
}

fn sorted_strings(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Blake3Hex;

    const VALID_SOURCE: &[u8] = b"let value = 1 in value\n";
    const STALE_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const FIXTURE_ID: &str = "unit.source.valid";
    const REQUIREMENT_ID: &str =
        "r[taenite.validation_harness.seed_fixtures.valid_source_envelope]";
    const NON_CLAIM: &str = "not Nickel evaluation semantics";
    const LAW: &str = "source-envelope-preservation";

    fn valid_source_fixture() -> FixtureCase {
        let declared_identity = compute_identity(IdentityDomain::Source, VALID_SOURCE);
        FixtureCase {
            id: FIXTURE_ID.to_string(),
            fixture_class: FixtureClass::Positive,
            artifact_class: ArtifactClass::SourceEnvelope,
            requirement_ids: vec![REQUIREMENT_ID.to_string()],
            non_claims: vec![NON_CLAIM.to_string()],
            expected_verdict: ValidationVerdict::Accepted,
            checked_laws: vec![LAW.to_string()],
            payload: FixturePayload::SourceEnvelope(SourceEnvelopeInput {
                schema_version: SUPPORTED_SCHEMA_VERSION,
                language: "nickel".to_string(),
                media_type: "text/x-nickel".to_string(),
                parser_identity: "nickel-reference-deferred".to_string(),
                source_bytes: VALID_SOURCE.to_vec(),
                declared_identity,
                aliases: vec!["seed".to_string()],
            }),
        }
    }

    #[test]
    fn accepts_valid_source_envelope() {
        let result = validate_fixture(&valid_source_fixture());
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
        assert!(result.expectation_met);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn rejects_stale_source_identity() {
        let mut fixture = valid_source_fixture();
        fixture.id = "unit.source.stale".to_string();
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SourceEnvelope(input) = &mut fixture.payload {
            let stale_hex = match Blake3Hex::parse(STALE_HASH) {
                Ok(parsed) => parsed,
                Err(_) => Blake3Hex::from_bytes(VALID_SOURCE),
            };
            input.declared_identity = Identity::new(IdentityDomain::Source, stale_hex);
        }
        let result = validate_fixture(&fixture);
        assert_eq!(result.observed_verdict, ValidationVerdict::Rejected);
        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
    }

    #[test]
    fn rejects_wrong_identity_domain() {
        let supplied_identity = compute_identity(IdentityDomain::Projection, VALID_SOURCE);
        let fixture = FixtureCase {
            id: "unit.identity.wrong-domain".to_string(),
            fixture_class: FixtureClass::Negative,
            artifact_class: ArtifactClass::Receipt,
            requirement_ids: vec![
                "r[taenite.rust_determinism.typed_identity_domains.source_not_receipt]".to_string(),
            ],
            non_claims: vec!["not cross-domain coercion".to_string()],
            expected_verdict: ValidationVerdict::Rejected,
            checked_laws: Vec::new(),
            payload: FixturePayload::IdentityUse(IdentityUseInput {
                required_domain: IdentityDomain::Source,
                supplied_identity,
            }),
        };
        let result = validate_fixture(&fixture);
        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::IdentityDomainMismatch)
        );
    }

    #[test]
    fn accepts_algebraic_exact_discharge() {
        let bytes = b"<effect positive>".to_vec();
        let identity = compute_identity(IdentityDomain::AlgebraicEffect, &bytes);
        let fixture = FixtureCase {
            id: "unit.effect.discharge".to_string(),
            fixture_class: FixtureClass::Positive,
            artifact_class: ArtifactClass::AlgebraicEffect,
            requirement_ids: vec![
                "r[taenite.algebraic_effects.fixture_corpus.positive_discharge]".to_string(),
            ],
            non_claims: vec!["not runtime authorization".to_string()],
            expected_verdict: ValidationVerdict::Accepted,
            checked_laws: vec!["exact-discharge".to_string()],
            payload: FixturePayload::AlgebraicEffect(Box::new(AlgebraicEffectInput {
                schema_version: SUPPORTED_SCHEMA_VERSION,
                canonical_bytes: bytes,
                declared_identity: identity,
                signatures: vec![EffectOperation {
                    operation_id: "op.read-config".to_string(),
                    display_name: "ReadConfig".to_string(),
                    parameter_schema: Some("unit".to_string()),
                    result_schema: Some("string".to_string()),
                }],
                input_row: vec!["op.read-config".to_string()],
                row_open: false,
                claimed_pure: false,
                handler: Some(HandlerDescriptorInput {
                    name: "config-memory-handler".to_string(),
                    handled_operations: vec!["op.read-config".to_string()],
                    residual_operations: Vec::new(),
                    continuation_policy: "no-continuation".to_string(),
                    claims_discharge: true,
                    host_capability_requirements: Vec::new(),
                }),
                receipt: Some(HandlerReceiptInput {
                    discharged_operations: vec!["op.read-config".to_string()],
                    residual_operations: Vec::new(),
                    continuation_policy: "no-continuation".to_string(),
                    continuation_evidence: None,
                    observed_operations: vec!["op.read-config".to_string()],
                    checked_laws: vec!["exact-discharge".to_string()],
                }),
                replay: None,
                claimed_host_authority: Vec::new(),
                admitted_host_capabilities: Vec::new(),
            })),
        };
        let result = validate_fixture(&fixture);
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
        assert!(result.expectation_met);
    }

    #[test]
    fn rejects_host_capability_confusion() {
        let bytes = b"<effect host confusion>".to_vec();
        let identity = compute_identity(IdentityDomain::AlgebraicEffect, &bytes);
        let fixture = FixtureCase {
            id: "unit.effect.host-confusion".to_string(),
            fixture_class: FixtureClass::Negative,
            artifact_class: ArtifactClass::AlgebraicEffect,
            requirement_ids: vec![
                "r[taenite.algebraic_effects.fixture_corpus.negative_host_confusion]".to_string(),
            ],
            non_claims: vec!["operation identity is not authority".to_string()],
            expected_verdict: ValidationVerdict::Rejected,
            checked_laws: Vec::new(),
            payload: FixturePayload::AlgebraicEffect(Box::new(AlgebraicEffectInput {
                schema_version: SUPPORTED_SCHEMA_VERSION,
                canonical_bytes: bytes,
                declared_identity: identity,
                signatures: vec![EffectOperation {
                    operation_id: "op.current-time".to_string(),
                    display_name: "CurrentTime".to_string(),
                    parameter_schema: Some("unit".to_string()),
                    result_schema: Some("timestamp".to_string()),
                }],
                input_row: vec!["op.current-time".to_string()],
                row_open: false,
                claimed_pure: false,
                handler: None,
                receipt: None,
                replay: None,
                claimed_host_authority: vec!["clock".to_string()],
                admitted_host_capabilities: Vec::new(),
            })),
        };
        let result = validate_fixture(&fixture);
        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::HostCapabilityConfusion)
        );
    }
}
