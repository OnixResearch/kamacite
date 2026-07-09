use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{Identity, IdentityDomain, compute_identity};
use crate::receipt::{ReceiptBody, ReceiptEnvelope};
use crate::valence_export::{
    ValenceReceiptExportInput, export_identity_key, export_valence_receipt,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const SUPPORTED_SCHEMA_VERSION: u32 = 1;
const REQUIREMENT_SCHEMA_VERSION: &str =
    "r[kamacite.lossless_ir.schema_vocabulary.unsupported_version]";
const REQUIREMENT_SOURCE_HASH: &str = "r[kamacite.lossless_ir.source_preservation.hash_mismatch]";
const REQUIREMENT_SOURCE_BYTES: &str =
    "r[kamacite.lossless_ir.source_envelope_vocabulary.missing_bytes]";
const REQUIREMENT_NON_CLAIMS: &str = "r[kamacite.lossless_ir.non_claims.receipt]";
const REQUIREMENT_TYPED_DOMAIN: &str =
    "r[kamacite.lossless_ir.typed_identity_domains.projection_not_canonical]";
const REQUIREMENT_CAPABILITIES: &str =
    "r[kamacite.rust_determinism.explicit_capabilities.filesystem]";
const REQUIREMENT_TARGET_OVERCLAIM: &str = "r[kamacite.lossless_ir.wasm_target_boundary.not_ir]";
const REQUIREMENT_UNSUPPORTED_CLAIM: &str =
    "r[kamacite.lossless_ir.losslessness_classes.arbitrary_source_denied]";
const REQUIREMENT_FIXTURE_METADATA: &str =
    "r[kamacite.lossless_ir.fixture_corpus.requirement_coverage]";
const REQUIREMENT_EXPECTED_VERDICT: &str = "r[kamacite.lossless_ir.negative_fixtures]";
const REQUIREMENT_PATH_ESCAPE: &str =
    "r[kamacite.rust_determinism.capability_shells.escape_fixtures]";
const REQUIREMENT_ALGEBRAIC_VERSION: &str =
    "r[kamacite.algebraic_effects.ir_boundary.unsupported_version]";
const REQUIREMENT_EFFECT_SCHEMA: &str =
    "r[kamacite.algebraic_effects.signature_vocabulary.missing_schema]";
const REQUIREMENT_UNDECLARED_EFFECT: &str =
    "r[kamacite.algebraic_effects.row_summary_vocabulary.undeclared_operation]";
const REQUIREMENT_OPEN_ROW: &str =
    "r[kamacite.algebraic_effects.row_summary_vocabulary.open_row_not_pure]";
const REQUIREMENT_HANDLER_NAME: &str =
    "r[kamacite.algebraic_effects.handler_descriptor.name_not_discharge]";
const REQUIREMENT_UNKNOWN_EFFECT: &str =
    "r[kamacite.algebraic_effects.handler_receipts.unknown_effect_denied]";
const REQUIREMENT_CONTINUATION: &str = "r[kamacite.algebraic_effects.continuation_policy.mismatch]";
const REQUIREMENT_HOST_CONFUSION: &str =
    "r[kamacite.algebraic_effects.host_boundary.operation_not_authority]";
const REQUIREMENT_REPLAY: &str = "r[kamacite.algebraic_effects.replay_profile.recorded_log]";
const REQUIREMENT_STACK_BOUNDARY: &str =
    "r[kamacite.lossless_ir.stack_evidence_boundary.octet_checks]";
const REQUIREMENT_NICKEL_MISSING_SOURCE: &str =
    "r[kamacite.nickel_adapter_suite.negative_fixtures.missing_source]";
const REQUIREMENT_NICKEL_STALE_PROJECTION: &str =
    "r[kamacite.nickel_adapter_suite.negative_fixtures.stale_projection]";
const REQUIREMENT_NICKEL_JSON_NOT_CANONICAL: &str =
    "r[kamacite.nickel_adapter_suite.source_export_separation.json_not_canonical]";
const REQUIREMENT_NICKEL_MISSING_EVALUATOR: &str =
    "r[kamacite.nickel_adapter_suite.evaluator_identity.missing]";
const REQUIREMENT_NICKEL_UNSUPPORTED_EVALUATOR: &str =
    "r[kamacite.nickel_adapter_suite.evaluator_identity.unsupported]";
const REQUIREMENT_NICKEL_MALFORMED_OUTPUT: &str =
    "r[kamacite.nickel_adapter_suite.negative_fixtures.malformed_export]";
const REQUIREMENT_NICKEL_UNDECLARED_EFFECT: &str =
    "r[kamacite.nickel_adapter_suite.negative_fixtures.undeclared_effect]";
const REQUIREMENT_NICKEL_SOURCE_EXPORT: &str =
    "r[kamacite.nickel_adapter_suite.source_export_separation]";
const REQUIREMENT_NICKEL_CAPABILITY_BOUNDARY: &str =
    "r[kamacite.nickel_adapter_suite.capability_boundary]";
const REQUIREMENT_STEEL_RECEIPT_PROFILE: &str = "r[kamacite.steel_adapter_suite.receipt_profile]";
const REQUIREMENT_STEEL_EVALUATOR_IDENTITY: &str =
    "r[kamacite.steel_adapter_suite.receipt_profile.evaluator_identity]";
const REQUIREMENT_STEEL_SOURCE_MODULE_IDENTITY: &str =
    "r[kamacite.steel_adapter_suite.receipt_profile.source_module_identity]";
const REQUIREMENT_STEEL_OUTPUT_PROJECTION: &str =
    "r[kamacite.steel_adapter_suite.receipt_profile.output_projection]";
const REQUIREMENT_STEEL_DECLARED_HOST_FUNCTIONS: &str =
    "r[kamacite.steel_adapter_suite.host_function_boundary.declared_host_functions]";
const REQUIREMENT_STEEL_UNDECLARED_HOST_FUNCTION: &str =
    "r[kamacite.steel_adapter_suite.host_function_boundary.undeclared_host_function]";
const REQUIREMENT_STEEL_AMBIENT_IMPORT: &str =
    "r[kamacite.steel_adapter_suite.host_function_boundary.ambient_import]";
const REQUIREMENT_STEEL_DTO_OUTPUT: &str =
    "r[kamacite.steel_adapter_suite.output_boundary.dto_only]";
const REQUIREMENT_STEEL_OPAQUE_CLOSURE: &str =
    "r[kamacite.steel_adapter_suite.output_boundary.opaque_closure]";
const REQUIREMENT_STEEL_BYTECODE_NOT_IR: &str =
    "r[kamacite.steel_adapter_suite.output_boundary.bytecode_not_ir]";
const REQUIREMENT_STEEL_CAPABILITY_BOUNDARY: &str =
    "r[kamacite.steel_adapter_suite.capability_boundary]";
const NICKEL_ADAPTER_KIND: &str = "nickel-export";
const NICKEL_REFERENCE_EVALUATOR_PROFILE: &str = "nickel-reference-cli";
const STEEL_ADAPTER_KIND: &str = "steel-dto-probe";
const STEEL_DTO_PROBE_EVALUATOR_PROFILE: &str = "steel-dto-probe-shell";
const AMBIENT_STEEL_AUTHORITIES: &[&str] = &[
    "ambient-filesystem",
    "clock",
    "credential",
    "dylib",
    "filesystem",
    "mutation",
    "network",
    "package",
    "package-manager",
    "process",
    "random",
    "tcp",
    "time",
];

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
    ValenceReceiptExport,
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
            Self::ValenceReceiptExport => "valence-receipt-export",
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
    NickelAdapterReceipt(NickelAdapterReceiptInput),
    SteelAdapterReceipt(Box<SteelAdapterReceiptInput>),
    TargetArtifact(TargetArtifactInput),
    UnsupportedClaim(UnsupportedClaimInput),
    IdentityUse(IdentityUseInput),
    CapabilityShell(CapabilityShellInput),
    StackEvidence(StackEvidenceInput),
    AlgebraicEffect(Box<AlgebraicEffectInput>),
    ValenceReceiptExport(Box<ValenceReceiptExportInput>),
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
pub struct NickelAdapterReceiptInput {
    pub schema_version: u32,
    pub adapter_kind: String,
    pub source_bytes: Vec<u8>,
    pub source_identity: Option<Identity>,
    pub evaluator_profile: String,
    pub evaluator_descriptor: String,
    pub evaluator_identity: Option<Identity>,
    pub export_bytes: Vec<u8>,
    pub projection_identity: Option<Identity>,
    pub declared_capabilities: Vec<String>,
    pub observed_effects: Vec<String>,
    pub claims_canonical_ir: bool,
    pub checked_laws: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteelHostFunctionInput {
    pub function_id: String,
    pub input_schema: String,
    pub output_schema: String,
    pub effect_class: String,
    pub required_capabilities: Vec<String>,
    pub checked_laws: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SteelOutputClass {
    Dto,
    Projection,
    Descriptor,
    Diagnostic,
    Receipt,
    OpaqueClosure,
    Bytecode,
    JsonString,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteelAdapterReceiptInput {
    pub schema_version: u32,
    pub adapter_kind: String,
    pub source_bytes: Vec<u8>,
    pub source_identity: Option<Identity>,
    pub evaluator_profile: String,
    pub evaluator_descriptor: String,
    pub evaluator_identity: Option<Identity>,
    pub module_descriptor_bytes: Vec<u8>,
    pub module_identity: Option<Identity>,
    pub input_schema: String,
    pub output_schema: String,
    pub host_functions: Vec<SteelHostFunctionInput>,
    pub observed_host_functions: Vec<String>,
    pub declared_builtin_modules: Vec<String>,
    pub declared_imports: Vec<String>,
    pub requested_imports: Vec<String>,
    pub declared_capabilities: Vec<String>,
    pub observed_effects: Vec<String>,
    pub effect_class: String,
    pub output_bytes: Vec<u8>,
    pub output_projection_identity: Option<Identity>,
    pub output_class: SteelOutputClass,
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
        FixturePayload::NickelAdapterReceipt(input) => {
            validate_nickel_adapter_receipt(input, &mut accumulator)
        }
        FixturePayload::SteelAdapterReceipt(input) => {
            validate_steel_adapter_receipt(input, &mut accumulator)
        }
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
        FixturePayload::ValenceReceiptExport(input) => {
            validate_valence_receipt_export(input, &mut accumulator)
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
            "r[kamacite.lossless_ir.canonical_value_vocabulary.stale_hash]",
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
            "r[kamacite.lossless_ir.format_strategy.json_projection]",
            "canonical.projection_claims_authority",
            "compatibility projection cannot replace canonical Preserves bytes",
        ));
    }
}

fn validate_adapter_receipt(input: &AdapterReceiptInput, accumulator: &mut ValidationAccumulator) {
    if !input.has_conformance_entry {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            "r[kamacite.lossless_ir.adapter_conformance.untrusted_without_entry]",
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

fn validate_nickel_adapter_receipt(
    input: &NickelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    validate_supported_schema(input.schema_version, accumulator);
    validate_nickel_adapter_kind(input, accumulator);
    validate_nickel_source(input, accumulator);
    validate_nickel_evaluator(input, accumulator);
    validate_nickel_projection(input, accumulator);
    validate_nickel_capabilities(input, accumulator);
    if input.claims_canonical_ir {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::CanonicalIrOverclaim,
            REQUIREMENT_NICKEL_JSON_NOT_CANONICAL,
            "nickel.claims_canonical_ir",
            "Nickel JSON export is derived projection evidence, not canonical Kamacite IR",
        ));
    }
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_nickel_adapter_kind(
    input: &NickelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.adapter_kind != NICKEL_ADAPTER_KIND {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedArtifactClass,
                REQUIREMENT_NICKEL_SOURCE_EXPORT,
                "nickel.adapter_kind",
                "Nickel adapter receipt must name the reference export adapter kind",
            )
            .with_detail("adapter_kind", input.adapter_kind.clone()),
        );
    }
}

fn validate_nickel_source(
    input: &NickelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    let Some(source_identity) = &input.source_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_NICKEL_MISSING_SOURCE,
            "nickel.source_identity",
            "Nickel adapter receipt must bind preserved source-envelope identity",
        ));
        if input.source_bytes.is_empty() {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::MissingSourceBytes,
                REQUIREMENT_NICKEL_MISSING_SOURCE,
                "nickel.source_bytes",
                "Nickel adapter receipt must preserve source bytes before trusting export evidence",
            ));
        }
        return;
    };
    validate_identity_domain(
        source_identity,
        IdentityDomain::Source,
        "nickel.source_identity",
        accumulator,
    );
    accumulator
        .input_identities
        .insert("nickel-source".to_string(), source_identity.clone());
    if input.source_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingSourceBytes,
            REQUIREMENT_NICKEL_MISSING_SOURCE,
            "nickel.source_bytes",
            "Nickel adapter receipt must preserve source bytes before trusting export evidence",
        ));
        return;
    }
    let computed = compute_identity(IdentityDomain::Source, &input.source_bytes);
    accumulator
        .output_identities
        .insert("computed-nickel-source".to_string(), computed.clone());
    if source_identity.domain == IdentityDomain::Source && source_identity.hex != computed.hex {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_NICKEL_MISSING_SOURCE,
            "nickel.source_identity.hex",
            "declared Nickel source identity does not match source bytes",
        ));
    }
}

fn validate_nickel_evaluator(
    input: &NickelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.evaluator_profile != NICKEL_REFERENCE_EVALUATOR_PROFILE {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedEvaluatorIdentity,
                REQUIREMENT_NICKEL_UNSUPPORTED_EVALUATOR,
                "nickel.evaluator_profile",
                "Nickel export evidence must use the accepted reference CLI evaluator profile",
            )
            .with_detail("profile", input.evaluator_profile.clone()),
        );
    }
    if input.evaluator_descriptor.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_NICKEL_MISSING_EVALUATOR,
            "nickel.evaluator_descriptor",
            "Nickel evaluator identity must include deterministic version/profile data",
        ));
    }
    let Some(evaluator_identity) = &input.evaluator_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_NICKEL_MISSING_EVALUATOR,
            "nickel.evaluator_identity",
            "trusted Nickel export evidence must bind evaluator identity",
        ));
        return;
    };
    validate_identity_domain(
        evaluator_identity,
        IdentityDomain::Evaluator,
        "nickel.evaluator_identity",
        accumulator,
    );
    accumulator
        .input_identities
        .insert("nickel-evaluator".to_string(), evaluator_identity.clone());
    let computed = compute_identity(
        IdentityDomain::Evaluator,
        input.evaluator_descriptor.as_bytes(),
    );
    accumulator
        .output_identities
        .insert("computed-nickel-evaluator".to_string(), computed.clone());
    if evaluator_identity.domain == IdentityDomain::Evaluator
        && evaluator_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_NICKEL_MISSING_EVALUATOR,
            "nickel.evaluator_identity.hex",
            "declared Nickel evaluator identity does not match descriptor bytes",
        ));
    }
}

fn validate_nickel_projection(
    input: &NickelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    let Some(projection_identity) = &input.projection_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_NICKEL_STALE_PROJECTION,
            "nickel.projection_identity",
            "Nickel adapter receipt must bind export projection identity",
        ));
        return;
    };
    validate_identity_domain(
        projection_identity,
        IdentityDomain::Projection,
        "nickel.projection_identity",
        accumulator,
    );
    accumulator.input_identities.insert(
        "nickel-export-projection".to_string(),
        projection_identity.clone(),
    );
    let computed = compute_identity(IdentityDomain::Projection, &input.export_bytes);
    accumulator.output_identities.insert(
        "computed-nickel-export-projection".to_string(),
        computed.clone(),
    );
    if projection_identity.domain == IdentityDomain::Projection
        && projection_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_NICKEL_STALE_PROJECTION,
            "nickel.projection_identity.hex",
            "declared Nickel export projection identity does not match export bytes",
        ));
    }
    if serde_json::from_slice::<serde_json::Value>(&input.export_bytes).is_err() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_NICKEL_MALFORMED_OUTPUT,
            "nickel.export_bytes",
            "Nickel export bytes must decode under the declared JSON projection schema",
        ));
    }
}

fn validate_nickel_capabilities(
    input: &NickelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.declared_capabilities.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_NICKEL_CAPABILITY_BOUNDARY,
            "nickel.declared_capabilities",
            "Nickel adapter receipts must declare shell capabilities before execution evidence is trusted",
        ));
    }
    let declared: BTreeSet<&String> = input.declared_capabilities.iter().collect();
    for observed in &input.observed_effects {
        if !declared.contains(observed) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredCapability,
                    REQUIREMENT_NICKEL_UNDECLARED_EFFECT,
                    "nickel.observed_effects",
                    "observed Nickel adapter host effect was not declared",
                )
                .with_detail("effect", observed.clone()),
            );
        }
    }
}

fn validate_steel_adapter_receipt(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    validate_supported_schema(input.schema_version, accumulator);
    validate_steel_adapter_kind(input, accumulator);
    validate_steel_source(input, accumulator);
    validate_steel_evaluator(input, accumulator);
    validate_steel_module_identity(input, accumulator);
    validate_steel_dto_schemas(input, accumulator);
    let declared_host_functions = validate_steel_host_functions(input, accumulator);
    validate_steel_host_function_usage(input, &declared_host_functions, accumulator);
    validate_steel_imports(input, accumulator);
    validate_steel_capabilities(input, accumulator);
    validate_steel_output(input, accumulator);
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_steel_adapter_kind(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.adapter_kind != STEEL_ADAPTER_KIND {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedArtifactClass,
                REQUIREMENT_STEEL_RECEIPT_PROFILE,
                "steel.adapter_kind",
                "Steel adapter receipt must name the DTO-only probe adapter kind",
            )
            .with_detail("adapter_kind", input.adapter_kind.clone()),
        );
    }
}

fn validate_steel_source(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.source_bytes.is_empty() {
        if input.source_identity.is_some() {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::MissingSourceBytes,
                REQUIREMENT_STEEL_SOURCE_MODULE_IDENTITY,
                "steel.source_bytes",
                "Steel source identity cannot be trusted without source bytes",
            ));
        }
        return;
    }
    let Some(source_identity) = &input.source_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_STEEL_SOURCE_MODULE_IDENTITY,
            "steel.source_identity",
            "Steel source bytes must bind a source-envelope identity",
        ));
        return;
    };
    validate_identity_domain(
        source_identity,
        IdentityDomain::Source,
        "steel.source_identity",
        accumulator,
    );
    accumulator
        .input_identities
        .insert("steel-source".to_string(), source_identity.clone());
    let computed = compute_identity(IdentityDomain::Source, &input.source_bytes);
    accumulator
        .output_identities
        .insert("computed-steel-source".to_string(), computed.clone());
    if source_identity.domain == IdentityDomain::Source && source_identity.hex != computed.hex {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_STEEL_SOURCE_MODULE_IDENTITY,
            "steel.source_identity.hex",
            "declared Steel source identity does not match source bytes",
        ));
    }
}

fn validate_steel_evaluator(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.evaluator_profile != STEEL_DTO_PROBE_EVALUATOR_PROFILE {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedEvaluatorIdentity,
                REQUIREMENT_STEEL_EVALUATOR_IDENTITY,
                "steel.evaluator_profile",
                "Steel adapter evidence must use the accepted DTO-probe evaluator profile",
            )
            .with_detail("profile", input.evaluator_profile.clone()),
        );
    }
    if input.evaluator_descriptor.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_STEEL_EVALUATOR_IDENTITY,
            "steel.evaluator_descriptor",
            "Steel evaluator identity must include deterministic profile data",
        ));
    }
    let Some(evaluator_identity) = &input.evaluator_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_STEEL_EVALUATOR_IDENTITY,
            "steel.evaluator_identity",
            "trusted Steel adapter evidence must bind evaluator identity",
        ));
        return;
    };
    validate_identity_domain(
        evaluator_identity,
        IdentityDomain::Evaluator,
        "steel.evaluator_identity",
        accumulator,
    );
    accumulator
        .input_identities
        .insert("steel-evaluator".to_string(), evaluator_identity.clone());
    let computed = compute_identity(
        IdentityDomain::Evaluator,
        input.evaluator_descriptor.as_bytes(),
    );
    accumulator
        .output_identities
        .insert("computed-steel-evaluator".to_string(), computed.clone());
    if evaluator_identity.domain == IdentityDomain::Evaluator
        && evaluator_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_STEEL_EVALUATOR_IDENTITY,
            "steel.evaluator_identity.hex",
            "declared Steel evaluator identity does not match descriptor bytes",
        ));
    }
}

fn validate_steel_module_identity(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.module_descriptor_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_STEEL_SOURCE_MODULE_IDENTITY,
            "steel.module_descriptor_bytes",
            "Steel adapter receipts must bind module or descriptor bytes separately",
        ));
        return;
    }
    let Some(module_identity) = &input.module_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_STEEL_SOURCE_MODULE_IDENTITY,
            "steel.module_identity",
            "Steel module or descriptor bytes must bind an adapter-domain identity",
        ));
        return;
    };
    validate_identity_domain(
        module_identity,
        IdentityDomain::Adapter,
        "steel.module_identity",
        accumulator,
    );
    accumulator.input_identities.insert(
        "steel-module-descriptor".to_string(),
        module_identity.clone(),
    );
    let computed = compute_identity(IdentityDomain::Adapter, &input.module_descriptor_bytes);
    accumulator.output_identities.insert(
        "computed-steel-module-descriptor".to_string(),
        computed.clone(),
    );
    if module_identity.domain == IdentityDomain::Adapter && module_identity.hex != computed.hex {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_STEEL_SOURCE_MODULE_IDENTITY,
            "steel.module_identity.hex",
            "declared Steel module identity does not match descriptor bytes",
        ));
    }
}

fn validate_steel_dto_schemas(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.input_schema.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_STEEL_RECEIPT_PROFILE,
            "steel.input_schema",
            "Steel adapter receipts must declare an input DTO schema",
        ));
    }
    if input.output_schema.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_STEEL_RECEIPT_PROFILE,
            "steel.output_schema",
            "Steel adapter receipts must declare an output DTO schema",
        ));
    }
    if input.effect_class.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_STEEL_RECEIPT_PROFILE,
            "steel.effect_class",
            "Steel adapter receipts must declare an effect class",
        ));
    }
}

fn validate_steel_host_functions(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) -> BTreeSet<String> {
    let declared_capabilities: BTreeSet<&String> = input.declared_capabilities.iter().collect();
    let mut declared_host_functions = BTreeSet::new();
    if input.host_functions.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_STEEL_DECLARED_HOST_FUNCTIONS,
            "steel.host_functions",
            "Steel adapter receipts must declare trusted host functions before use",
        ));
    }
    for function in &input.host_functions {
        declared_host_functions.insert(function.function_id.clone());
        if function.function_id.is_empty()
            || function.input_schema.is_empty()
            || function.output_schema.is_empty()
            || function.effect_class.is_empty()
            || function.checked_laws.is_empty()
        {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_STEEL_DECLARED_HOST_FUNCTIONS,
                "steel.host_functions",
                "host-function declarations require id, DTO schemas, effect class, and laws",
            ));
        }
        for capability in &function.required_capabilities {
            if !declared_capabilities.contains(capability) {
                accumulator.diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::UndeclaredCapability,
                        REQUIREMENT_STEEL_DECLARED_HOST_FUNCTIONS,
                        "steel.host_functions.required_capabilities",
                        "host-function required capability was not declared by adapter",
                    )
                    .with_detail("capability", capability.clone())
                    .with_detail("function", function.function_id.clone()),
                );
            }
        }
        accumulator
            .checked_laws
            .extend(function.checked_laws.clone());
    }
    declared_host_functions
}

fn validate_steel_host_function_usage(
    input: &SteelAdapterReceiptInput,
    declared_host_functions: &BTreeSet<String>,
    accumulator: &mut ValidationAccumulator,
) {
    for observed in &input.observed_host_functions {
        if !declared_host_functions.contains(observed) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredAuthority,
                    REQUIREMENT_STEEL_UNDECLARED_HOST_FUNCTION,
                    "steel.observed_host_functions",
                    "observed Steel host function was not declared by adapter descriptor",
                )
                .with_detail("function", observed.clone()),
            );
        }
    }
}

fn validate_steel_imports(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    let declared_imports: BTreeSet<&String> = input
        .declared_imports
        .iter()
        .chain(input.declared_builtin_modules.iter())
        .collect();
    let declared_capabilities: BTreeSet<&String> = input.declared_capabilities.iter().collect();
    for requested in &input.requested_imports {
        let ambient = is_ambient_steel_authority(requested);
        if ambient && !declared_capabilities.contains(requested) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredAuthority,
                    REQUIREMENT_STEEL_AMBIENT_IMPORT,
                    "steel.requested_imports",
                    "Steel requested ambient authority outside the declared adapter contract",
                )
                .with_detail("import", requested.clone()),
            );
            continue;
        }
        if !declared_imports.contains(requested) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredAuthority,
                    REQUIREMENT_STEEL_AMBIENT_IMPORT,
                    "steel.requested_imports",
                    "Steel requested an import or built-in module not declared by descriptor",
                )
                .with_detail("import", requested.clone()),
            );
        }
    }
}

fn is_ambient_steel_authority(authority: &str) -> bool {
    AMBIENT_STEEL_AUTHORITIES.contains(&authority)
}

fn validate_steel_capabilities(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.declared_capabilities.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_STEEL_CAPABILITY_BOUNDARY,
            "steel.declared_capabilities",
            "Steel adapter receipts must declare shell capabilities before evidence is trusted",
        ));
    }
    let declared: BTreeSet<&String> = input.declared_capabilities.iter().collect();
    for observed in &input.observed_effects {
        if !declared.contains(observed) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredCapability,
                    REQUIREMENT_STEEL_CAPABILITY_BOUNDARY,
                    "steel.observed_effects",
                    "observed Steel adapter effect exceeded declared capabilities",
                )
                .with_detail("effect", observed.clone()),
            );
        }
    }
}

fn validate_steel_output(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    let Some(projection_identity) = &input.output_projection_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_STEEL_OUTPUT_PROJECTION,
            "steel.output_projection_identity",
            "Steel adapter receipts must bind output projection identity",
        ));
        return;
    };
    validate_identity_domain(
        projection_identity,
        IdentityDomain::Projection,
        "steel.output_projection_identity",
        accumulator,
    );
    accumulator.input_identities.insert(
        "steel-output-projection".to_string(),
        projection_identity.clone(),
    );
    let computed = compute_identity(IdentityDomain::Projection, &input.output_bytes);
    accumulator.output_identities.insert(
        "computed-steel-output-projection".to_string(),
        computed.clone(),
    );
    if projection_identity.domain == IdentityDomain::Projection
        && projection_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_STEEL_OUTPUT_PROJECTION,
            "steel.output_projection_identity.hex",
            "declared Steel output projection identity does not match output bytes",
        ));
    }
    validate_steel_output_class(input, accumulator);
}

fn validate_steel_output_class(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    match input.output_class {
        SteelOutputClass::Dto | SteelOutputClass::Projection => {
            if input.output_bytes.is_empty()
                || serde_json::from_slice::<serde_json::Value>(&input.output_bytes).is_err()
            {
                accumulator.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::MalformedProjectionOutput,
                    REQUIREMENT_STEEL_DTO_OUTPUT,
                    "steel.output_bytes",
                    "Steel DTO or projection output must decode under the declared schema",
                ));
            }
        }
        SteelOutputClass::Descriptor | SteelOutputClass::Diagnostic | SteelOutputClass::Receipt => {
        }
        SteelOutputClass::OpaqueClosure => accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_STEEL_OPAQUE_CLOSURE,
            "steel.output_class",
            "opaque Steel closures must not be exported as portable Kamacite values",
        )),
        SteelOutputClass::Bytecode | SteelOutputClass::JsonString => {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::CanonicalIrOverclaim,
                REQUIREMENT_STEEL_BYTECODE_NOT_IR,
                "steel.output_class",
                "Steel bytecode, JSON strings, or evaluator-internal values are not canonical IR",
            ));
        }
    }
    if input.claims_canonical_ir {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::CanonicalIrOverclaim,
            REQUIREMENT_STEEL_BYTECODE_NOT_IR,
            "steel.claims_canonical_ir",
            "Steel adapter output cannot replace canonical Kamacite Preserves IR",
        ));
    }
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
            "r[kamacite.lossless_ir.target_artifact_vocabulary.toolchain_provenance]",
            "target.declared_identity.hex",
            "declared target identity does not match target artifact bytes",
        ));
    }
    if input.metadata.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            "r[kamacite.lossless_ir.llvm_target_boundary.artifact_receipt]",
            "target.metadata",
            "target artifact metadata is required for bounded provenance",
        ));
    }
    if input.claims_canonical_ir {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::CanonicalIrOverclaim,
            REQUIREMENT_TARGET_OVERCLAIM,
            "target.claims_canonical_ir",
            "compiled target artifact cannot be Kamacite canonical IR",
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
            "r[kamacite.lossless_ir.total_diagnostics.malformed]",
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
            "stack evidence export broadened Kamacite bounded claim",
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

fn validate_valence_receipt_export(
    input: &ValenceReceiptExportInput,
    accumulator: &mut ValidationAccumulator,
) {
    accumulator.input_identities.insert(
        "valence-source-receipt".to_string(),
        input.source_receipt.receipt_identity.clone(),
    );
    accumulator.input_identities.insert(
        "valence-conformance".to_string(),
        input.conformance.conformance_identity.clone(),
    );
    accumulator
        .checked_laws
        .extend(input.source_receipt.body.checked_laws.clone());
    accumulator
        .checked_laws
        .extend(input.conformance.checked_laws.clone());

    let result = export_valence_receipt(input);
    accumulator.diagnostics.extend(result.diagnostics);
    if let Some(export) = result.export {
        accumulator
            .output_identities
            .insert(export_identity_key(), export.export_identity);
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
            "r[kamacite.algebraic_effects.fixture_corpus.negative_stale_identity]",
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
                    "r[kamacite.algebraic_effects.handler_descriptor.residual_row]",
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
                "r[kamacite.algebraic_effects.continuation_policy.single_use_evidence]",
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
                    "r[kamacite.algebraic_effects.replay_profile.non_replayable]",
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
        "r[kamacite.validation_harness.seed_fixtures.valid_source_envelope]";
    const NON_CLAIM: &str = "not Nickel evaluation semantics";
    const LAW: &str = "source-envelope-preservation";
    const NICKEL_FIXTURE_ID: &str = "unit.nickel.adapter.valid";
    const NICKEL_REQUIREMENT_ID: &str =
        "r[kamacite.nickel_adapter_suite.positive_fixtures.valid_export]";
    const NICKEL_NON_CLAIM: &str = "Nickel export is derived projection only";
    const NICKEL_ADAPTER_LAW: &str = "source-export-separation";
    const NICKEL_EVALUATOR_DESCRIPTOR: &str =
        "nickel-reference-cli:nickel-lang-cli nickel test-version";
    const NICKEL_EVALUATOR_PROFILE: &str = "nickel-reference-cli";
    const UNSUPPORTED_EVALUATOR_PROFILE: &str = "third-party-nickel-wrapper";
    const FIXTURE_READ_CAPABILITY: &str = "fixture-read";
    const NICKEL_EXPORT_CAPABILITY: &str = "nickel-reference-export";
    const CAPTURE_STDOUT_CAPABILITY: &str = "capture-stdout";
    const NETWORK_CAPABILITY: &str = "network";
    const VALID_EXPORT: &[u8] = b"1\n";
    const STALE_EXPORT: &[u8] = b"0\n";
    const STEEL_FIXTURE_ID: &str = "unit.steel.adapter.valid";
    const STEEL_REQUIREMENT_ID: &str = "r[kamacite.steel_adapter_suite.positive_fixtures.dto_only]";
    const STEEL_NON_CLAIM: &str = "Steel output is bounded DTO evidence only";
    const STEEL_ADAPTER_LAW: &str = "dto-only-output-boundary";
    const STEEL_HOST_FUNCTION_ID: &str = "kamacite.add-one";
    const STEEL_UNDECLARED_HOST_FUNCTION_ID: &str = "kamacite.read-file";
    const STEEL_EVALUATOR_DESCRIPTOR: &str = "steel-dto-probe-shell:kamacite-cli-dto-profile-v1";
    const STEEL_SOURCE: &[u8] = b"(define (probe input) (kamacite.add-one input))\n";
    const STEEL_MODULE_DESCRIPTOR: &[u8] =
        b"{\"module\":\"kamacite.dto.probe\",\"imports\":[\"steel/base\"]}\n";
    const STEEL_OUTPUT: &[u8] = b"{\"value\":2}\n";
    const MALFORMED_STEEL_OUTPUT: &[u8] = b"not-json\n";
    const STALE_STEEL_OUTPUT: &[u8] = b"{\"value\":1}\n";
    const DTO_EVALUATION_CAPABILITY: &str = "dto-evaluation";
    const CAPTURE_OUTPUT_CAPABILITY: &str = "capture-stdout";
    const STEEL_BASE_MODULE: &str = "steel/base";
    const FILESYSTEM_IMPORT: &str = "filesystem";

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
                "r[kamacite.rust_determinism.typed_identity_domains.source_not_receipt]"
                    .to_string(),
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

    fn valid_nickel_adapter_fixture() -> FixtureCase {
        FixtureCase {
            id: NICKEL_FIXTURE_ID.to_string(),
            fixture_class: FixtureClass::Positive,
            artifact_class: ArtifactClass::AdapterReceipt,
            requirement_ids: vec![NICKEL_REQUIREMENT_ID.to_string()],
            non_claims: vec![NICKEL_NON_CLAIM.to_string()],
            expected_verdict: ValidationVerdict::Accepted,
            checked_laws: vec![NICKEL_ADAPTER_LAW.to_string()],
            payload: FixturePayload::NickelAdapterReceipt(NickelAdapterReceiptInput {
                schema_version: SUPPORTED_SCHEMA_VERSION,
                adapter_kind: NICKEL_ADAPTER_KIND.to_string(),
                source_bytes: VALID_SOURCE.to_vec(),
                source_identity: Some(compute_identity(IdentityDomain::Source, VALID_SOURCE)),
                evaluator_profile: NICKEL_EVALUATOR_PROFILE.to_string(),
                evaluator_descriptor: NICKEL_EVALUATOR_DESCRIPTOR.to_string(),
                evaluator_identity: Some(compute_identity(
                    IdentityDomain::Evaluator,
                    NICKEL_EVALUATOR_DESCRIPTOR.as_bytes(),
                )),
                export_bytes: VALID_EXPORT.to_vec(),
                projection_identity: Some(compute_identity(
                    IdentityDomain::Projection,
                    VALID_EXPORT,
                )),
                declared_capabilities: vec![
                    FIXTURE_READ_CAPABILITY.to_string(),
                    NICKEL_EXPORT_CAPABILITY.to_string(),
                    CAPTURE_STDOUT_CAPABILITY.to_string(),
                ],
                observed_effects: vec![
                    FIXTURE_READ_CAPABILITY.to_string(),
                    NICKEL_EXPORT_CAPABILITY.to_string(),
                    CAPTURE_STDOUT_CAPABILITY.to_string(),
                ],
                claims_canonical_ir: false,
                checked_laws: vec![NICKEL_ADAPTER_LAW.to_string()],
            }),
        }
    }

    #[test]
    fn accepts_nickel_adapter_export_evidence() {
        let result = validate_fixture(&valid_nickel_adapter_fixture());
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
        assert!(result.expectation_met);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn rejects_nickel_adapter_missing_source() {
        let mut fixture = valid_nickel_adapter_fixture();
        fixture.id = "unit.nickel.adapter.missing-source".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::NickelAdapterReceipt(input) = &mut fixture.payload {
            input.source_bytes = Vec::new();
            input.source_identity = None;
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingSourceBytes)
        );
    }

    #[test]
    fn rejects_nickel_adapter_stale_projection_identity() {
        let mut fixture = valid_nickel_adapter_fixture();
        fixture.id = "unit.nickel.adapter.stale-projection".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::NickelAdapterReceipt(input) = &mut fixture.payload {
            input.projection_identity =
                Some(compute_identity(IdentityDomain::Projection, STALE_EXPORT));
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
    }

    #[test]
    fn rejects_nickel_adapter_unsupported_evaluator_profile() {
        let mut fixture = valid_nickel_adapter_fixture();
        fixture.id = "unit.nickel.adapter.unsupported-evaluator".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::NickelAdapterReceipt(input) = &mut fixture.payload {
            input.evaluator_profile = UNSUPPORTED_EVALUATOR_PROFILE.to_string();
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == DiagnosticCode::UnsupportedEvaluatorIdentity
            })
        );
    }

    #[test]
    fn rejects_nickel_adapter_undeclared_effect() {
        let mut fixture = valid_nickel_adapter_fixture();
        fixture.id = "unit.nickel.adapter.undeclared-effect".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::NickelAdapterReceipt(input) = &mut fixture.payload {
            input.observed_effects.push(NETWORK_CAPABILITY.to_string());
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::UndeclaredCapability)
        );
    }

    fn valid_steel_adapter_fixture() -> FixtureCase {
        FixtureCase {
            id: STEEL_FIXTURE_ID.to_string(),
            fixture_class: FixtureClass::Positive,
            artifact_class: ArtifactClass::AdapterReceipt,
            requirement_ids: vec![STEEL_REQUIREMENT_ID.to_string()],
            non_claims: vec![STEEL_NON_CLAIM.to_string()],
            expected_verdict: ValidationVerdict::Accepted,
            checked_laws: vec![STEEL_ADAPTER_LAW.to_string()],
            payload: FixturePayload::SteelAdapterReceipt(Box::new(SteelAdapterReceiptInput {
                schema_version: SUPPORTED_SCHEMA_VERSION,
                adapter_kind: STEEL_ADAPTER_KIND.to_string(),
                source_bytes: STEEL_SOURCE.to_vec(),
                source_identity: Some(compute_identity(IdentityDomain::Source, STEEL_SOURCE)),
                evaluator_profile: STEEL_DTO_PROBE_EVALUATOR_PROFILE.to_string(),
                evaluator_descriptor: STEEL_EVALUATOR_DESCRIPTOR.to_string(),
                evaluator_identity: Some(compute_identity(
                    IdentityDomain::Evaluator,
                    STEEL_EVALUATOR_DESCRIPTOR.as_bytes(),
                )),
                module_descriptor_bytes: STEEL_MODULE_DESCRIPTOR.to_vec(),
                module_identity: Some(compute_identity(
                    IdentityDomain::Adapter,
                    STEEL_MODULE_DESCRIPTOR,
                )),
                input_schema: "kamacite.dto.input.integer-v1".to_string(),
                output_schema: "kamacite.dto.output.integer-v1".to_string(),
                host_functions: vec![SteelHostFunctionInput {
                    function_id: STEEL_HOST_FUNCTION_ID.to_string(),
                    input_schema: "kamacite.dto.input.integer-v1".to_string(),
                    output_schema: "kamacite.dto.output.integer-v1".to_string(),
                    effect_class: "pure-dto".to_string(),
                    required_capabilities: vec![DTO_EVALUATION_CAPABILITY.to_string()],
                    checked_laws: vec!["host-function-dto-contract".to_string()],
                }],
                observed_host_functions: vec![STEEL_HOST_FUNCTION_ID.to_string()],
                declared_builtin_modules: vec![STEEL_BASE_MODULE.to_string()],
                declared_imports: Vec::new(),
                requested_imports: vec![STEEL_BASE_MODULE.to_string()],
                declared_capabilities: vec![
                    DTO_EVALUATION_CAPABILITY.to_string(),
                    CAPTURE_OUTPUT_CAPABILITY.to_string(),
                ],
                observed_effects: vec![
                    DTO_EVALUATION_CAPABILITY.to_string(),
                    CAPTURE_OUTPUT_CAPABILITY.to_string(),
                ],
                effect_class: "pure-dto".to_string(),
                output_bytes: STEEL_OUTPUT.to_vec(),
                output_projection_identity: Some(compute_identity(
                    IdentityDomain::Projection,
                    STEEL_OUTPUT,
                )),
                output_class: SteelOutputClass::Dto,
                claims_canonical_ir: false,
                checked_laws: vec![STEEL_ADAPTER_LAW.to_string()],
            })),
        }
    }

    #[test]
    fn accepts_steel_adapter_dto_evidence() {
        let result = validate_fixture(&valid_steel_adapter_fixture());
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
        assert!(result.expectation_met);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn rejects_steel_adapter_missing_evaluator_identity() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.missing-evaluator".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload {
            input.evaluator_identity = None;
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingInput)
        );
    }

    #[test]
    fn rejects_steel_adapter_undeclared_host_function() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.undeclared-host-function".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload {
            input
                .observed_host_functions
                .push(STEEL_UNDECLARED_HOST_FUNCTION_ID.to_string());
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::UndeclaredAuthority)
        );
    }

    #[test]
    fn rejects_steel_adapter_ambient_import() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.ambient-import".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload {
            input.requested_imports.push(FILESYSTEM_IMPORT.to_string());
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::UndeclaredAuthority)
        );
    }

    #[test]
    fn rejects_steel_adapter_stale_output_identity() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.stale-output".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload {
            input.output_projection_identity = Some(compute_identity(
                IdentityDomain::Projection,
                STALE_STEEL_OUTPUT,
            ));
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
    }

    #[test]
    fn rejects_steel_adapter_malformed_output() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.malformed-output".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload {
            input.output_bytes = MALFORMED_STEEL_OUTPUT.to_vec();
            input.output_projection_identity = Some(compute_identity(
                IdentityDomain::Projection,
                MALFORMED_STEEL_OUTPUT,
            ));
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::MalformedProjectionOutput })
        );
    }

    #[test]
    fn rejects_steel_adapter_opaque_closure_output() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.opaque-closure".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload {
            input.output_class = SteelOutputClass::OpaqueClosure;
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::UnsupportedArtifactClass })
        );
    }

    #[test]
    fn rejects_steel_adapter_bytecode_as_canonical_ir() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.bytecode-canonical-ir".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload {
            input.output_class = SteelOutputClass::Bytecode;
            input.claims_canonical_ir = true;
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::CanonicalIrOverclaim)
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
                "r[kamacite.algebraic_effects.fixture_corpus.positive_discharge]".to_string(),
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
                "r[kamacite.algebraic_effects.fixture_corpus.negative_host_confusion]".to_string(),
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
