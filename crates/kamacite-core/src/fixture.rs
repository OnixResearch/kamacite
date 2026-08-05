use crate::application_host_binding::ApplicationHostBindingInput;
use crate::ast_grep_audit::{AstGrepAuditInput, validate_ast_grep_audit};
use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{
    FsmCoverageIdentity, FsmSpecIdentity, FsmUniverseIdentity, Identity, IdentityDomain,
    IdentityDomainMarker, WireIdentity, compute_identity,
};
use crate::nickel_sandbox::{
    NickelSandboxEvidence, NickelSandboxProfile, nickel_sandbox_evidence_identity,
    validate_nickel_sandbox,
};
#[cfg(test)]
use crate::nickel_sandbox::{nickel_sandbox_observed_facts, nickel_sandbox_profile};
use crate::preserves_codec::{
    CanonicalPreservesArtifact, PreservesCodecError, PreservesLimits, PreservesSchema,
    PreservesSyntax, canonicalize_preserves, encode_typed_preserves,
};
use crate::receipt::{ReceiptBody, ReceiptEnvelope};
use crate::steel_execution::{
    STEEL_SUCCESS_EXIT_CLASS, SteelDtoValue, SteelExecutionEvidence, resolve_steel_dto_contract,
    validate_steel_dto,
};
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
const REQUIREMENT_PRESERVES_REAL_CODEC: &str =
    "r[kamacite.findings.preserves_authority.real_codec]";
const REQUIREMENT_PRESERVES_TYPED_BINDING: &str =
    "r[kamacite.findings.preserves_authority.typed_binding]";
const REQUIREMENT_PRESERVES_VALIDATION: &str =
    "r[kamacite.findings.preserves_authority.validation]";
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
const REQUIREMENT_STEEL_FRESH_PROBE: &str = "r[kamacite.findings.steel_execution.fresh_probe]";
const REQUIREMENT_STEEL_RUNNER_IDENTITY: &str =
    "r[kamacite.findings.steel_execution.runner_identity]";
const REQUIREMENT_STEEL_SCHEMA_CONFORMANCE: &str =
    "r[kamacite.findings.steel_execution.schema_conformance]";
const REQUIREMENT_STEEL_CAPABILITY_ENFORCEMENT: &str =
    "r[kamacite.findings.steel_execution.capability_enforcement]";
const REQUIREMENT_FSM_SPEC_VERSION: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.unsupported_version]";
const REQUIREMENT_FSM_SPEC_IDENTITY: &str = "r[kamacite.lossless_ir.fsm_spec_vocabulary.complete]";
const REQUIREMENT_FSM_CLOSED_UNIVERSES: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.closed_universes]";
const REQUIREMENT_FSM_DUPLICATE_UNIVERSE: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.closed_universes.duplicates]";
const REQUIREMENT_FSM_UNKNOWN_ENDPOINT: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.closed_universes.references]";
const REQUIREMENT_FSM_COVERAGE: &str = "r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage]";
const REQUIREMENT_FSM_MISSING_DENIAL: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage.missing_denial]";
const REQUIREMENT_FSM_DUPLICATE_COVERAGE: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.coverage.duplicate]";
const REQUIREMENT_FSM_NO_WILDCARDS: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.no_wildcards.reject]";
const REQUIREMENT_FSM_GUARDS_EFFECTS: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.guards_side_effects]";
const REQUIREMENT_FSM_UNDECLARED_GUARD: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.guards_side_effects.undeclared_guard]";
const REQUIREMENT_FSM_DENIAL_BOUNDARY: &str =
    "r[kamacite.lossless_ir.fsm_spec_vocabulary.guards_side_effects.denial]";
const REQUIREMENT_FSM_RECEIPT: &str =
    "r[kamacite.lossless_ir.fsm_receipt_vocabulary.checked_claim]";
const REQUIREMENT_FSM_RECEIPT_IDENTITY: &str =
    "r[kamacite.lossless_ir.fsm_receipt_vocabulary.identity_domain]";
const REQUIREMENT_FSM_VALIDATION: &str = "r[kamacite.lossless_ir.fsm_validation.pure]";
const REQUIREMENT_FSM_REACHABILITY: &str =
    "r[kamacite.fsms.reachability_terminal_policy.unreachable_unmarked]";
const REQUIREMENT_FSM_TERMINAL: &str =
    "r[kamacite.fsms.reachability_terminal_policy.terminal_outgoing]";
const REQUIREMENT_FSM_TRACE: &str = "r[kamacite.fsms.trace_receipts.valid_path]";
const REQUIREMENT_FSM_TRACE_IMPOSSIBLE: &str =
    "r[kamacite.fsms.trace_receipts.impossible_transition]";
const REQUIREMENT_FSM_DESCRIPTOR_BINDING: &str =
    "r[kamacite.findings.fsm_receipts.descriptor_binding]";
const REQUIREMENT_FSM_COVERAGE_RECOMPUTATION: &str =
    "r[kamacite.findings.fsm_receipts.coverage_recomputation]";
const REQUIREMENT_FSM_TRACE_CONSISTENCY: &str =
    "r[kamacite.findings.fsm_receipts.trace_consistency]";
const REQUIREMENT_FSM_STACK_VALENCE: &str =
    "r[kamacite.stack_integration.fsm_valence_export.non_claims]";
const REQUIREMENT_FSM_STACK_OCTET: &str =
    "r[kamacite.stack_integration.fsm_octet_interop.source_shape]";
const REQUIREMENT_DOCS_VERSION: &str = "r[kamacite.docs.ir_projection.unsupported_version]";
const REQUIREMENT_DOCS_IR: &str = "r[kamacite.docs.ir_projection]";
const REQUIREMENT_DOCS_SOURCE: &str = "r[kamacite.docs.ir_projection.missing_source]";
const REQUIREMENT_DOCS_STALE_SOURCE: &str = "r[kamacite.docs.ir_projection.stale_source_hash]";
const REQUIREMENT_DOCS_DUPLICATE_ID: &str = "r[kamacite.docs.ir_projection.duplicate_id]";
const REQUIREMENT_DOCS_RENDERERS: &str = "r[kamacite.docs.renderers]";
const REQUIREMENT_DOCS_RENDERER_FORMAT: &str = "r[kamacite.docs.renderers.unsupported_format]";
const REQUIREMENT_DOCS_RENDERER_DRIFT: &str = "r[kamacite.docs.renderers.drift]";
const REQUIREMENT_DOCS_UNKNOWN_LINK: &str = "r[kamacite.docs.renderers.unknown_link]";
const REQUIREMENT_DOCS_OUTPUT_BOUNDARY: &str =
    "r[kamacite.docs.renderers.projection_not_canonical]";
const REQUIREMENT_DOCS_RUNNABLE: &str = "r[kamacite.docs.runnable_cells]";
const REQUIREMENT_DOCS_DEFAULT_DENY: &str = "r[kamacite.docs.runnable_cells.denied_by_default]";
const REQUIREMENT_DOCS_UNSUPPORTED_LANGUAGE: &str =
    "r[kamacite.docs.runnable_cells.unsupported_language]";
const REQUIREMENT_DOCS_UNDECLARED_CAPABILITY: &str =
    "r[kamacite.docs.runnable_cells.undeclared_capability]";
const REQUIREMENT_DOCS_MISSING_EVALUATOR: &str =
    "r[kamacite.docs.runnable_cells.missing_evaluator]";
const REQUIREMENT_DOCS_OUTPUT_MISMATCH: &str = "r[kamacite.docs.runnable_cells.mismatched_output]";
const REQUIREMENT_DOCS_UNEXPECTED_SUCCESS: &str =
    "r[kamacite.docs.runnable_cells.unexpected_success]";
const REQUIREMENT_DOCS_AMBIENT_AUTHORITY: &str =
    "r[kamacite.docs.runnable_cells.ambient_authority]";
const REQUIREMENT_DOCS_RECEIPT: &str = "r[kamacite.docs.receipts]";
const REQUIREMENT_DOCS_NON_CLAIMS: &str = "r[kamacite.docs.receipts.non_claims]";
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
const FSM_DETERMINISTIC_POLICY: &str = "deterministic";
const FSM_EXHAUSTIVENESS_RECEIPT_KIND: &str = "exhaustiveness";
const FSM_TRACE_RECEIPT_KIND: &str = "trace";
const FSM_NO_SIDE_EFFECT_CLASS: &str = "no-side-effect";
const FSM_DENIAL_SIDE_EFFECT_CLASS: &str = "denial";
const FSM_WILDCARD_MARKERS: &[&str] = &["*", "default", "otherwise", "catch-all"];
const FSM_FORBIDDEN_CLAIMS: &[&str] = &[
    "semantic correctness",
    "semantic equivalence",
    "semantic-equivalence",
    "runtime correctness",
    "protocol completeness",
    "liveness",
    "verifier soundness",
    "release eligibility",
];
const DOC_RENDER_HTML: &str = "html";
const DOC_RENDER_MARKDOWN: &str = "markdown";
const DOC_RENDER_ASCIIDOC: &str = "asciidoc";
const DOC_SUPPORTED_RENDER_FORMATS: &[&str] =
    &[DOC_RENDER_HTML, DOC_RENDER_MARKDOWN, DOC_RENDER_ASCIIDOC];
const DOC_CELL_LANGUAGES: &[&str] = &["nickel", "steel", "rust-validator", "preserves"];
const DOC_STORED_EVIDENCE_MODE: &str = "stored-evidence";
const DOC_EXPLICIT_RUN_MODE: &str = "explicit-run";
const DOC_EXPECTED_SUCCESS: &str = "success";
const DOC_EXPECTED_FAILURE: &str = "failure";
const CANONICAL_VALUE_SCHEMA_VERSION: &str = "kamacite.canonical-value.v1";
const CANONICAL_VALUE_ROOT_LABEL: &str = "kamacite-canonical-value-v1";
const ALGEBRAIC_EFFECT_SCHEMA_VERSION: &str = "kamacite.algebraic-effect.v2";
const ALGEBRAIC_EFFECT_ROOT_LABEL: &str = "kamacite-algebraic-effect-v2";
const FSM_SPEC_SCHEMA_VERSION: &str = "kamacite.fsm-spec.v2";
const FSM_SPEC_ROOT_LABEL: &str = "kamacite-fsm-spec-v2";
const FSM_RECEIPT_SCHEMA_VERSION: &str = "kamacite.fsm-receipt.v3";
const FSM_RECEIPT_ROOT_LABEL: &str = "kamacite-fsm-receipt-v3";
const FSM_COVERAGE_SCHEMA_VERSION: &str = "kamacite.fsm-coverage.v1";
const FSM_COVERAGE_ROOT_LABEL: &str = "FsmCoverageMaterial";
const FSM_TRANSITION_KIND: &str = "transition";
const FSM_STATE_UNIVERSE_KIND: &str = "state-universe";
const FSM_EVENT_UNIVERSE_KIND: &str = "event-universe";
const FSM_TRANSITION_COVERAGE_KIND: &str = "transition-coverage";
const FSM_DENIAL_COVERAGE_KIND: &str = "denial-coverage";
const FSM_EXCLUSION_COVERAGE_KIND: &str = "exclusion-coverage";
const FSM_MISSING_CELLS_KIND: &str = "missing-cells";
const DOCUMENTATION_IR_SCHEMA_VERSION: &str = "kamacite.documentation-ir.v2";
const DOCUMENTATION_IR_ROOT_LABEL: &str = "kamacite-documentation-ir-v2";
const DOCUMENTATION_RECEIPT_SCHEMA_VERSION: &str = "kamacite.documentation-receipt.v2";
const DOCUMENTATION_RECEIPT_ROOT_LABEL: &str = "kamacite-documentation-receipt-v2";
const DOC_FORBIDDEN_CLAIMS: &[&str] = &[
    "canonical authority",
    "canonical ir",
    "semantic correctness",
    "semantic equivalence",
    "runtime correctness",
    "source losslessness",
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
    FsmSpec,
    FsmReceipt,
    DocumentationIr,
    DocumentationOutput,
    DocumentationCell,
    DocumentationReceipt,
    ApplicationHostBinding,
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
            Self::FsmSpec => "fsm-spec",
            Self::FsmReceipt => "fsm-receipt",
            Self::DocumentationIr => "documentation-ir",
            Self::DocumentationOutput => "documentation-output",
            Self::DocumentationCell => "documentation-cell",
            Self::DocumentationReceipt => "documentation-receipt",
            Self::ApplicationHostBinding => "application-host-binding",
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
    AstGrepAudit(Box<AstGrepAuditInput>),
    NickelAdapterReceipt(Box<NickelAdapterReceiptInput>),
    SteelAdapterReceipt(Box<SteelAdapterReceiptInput>),
    TargetArtifact(TargetArtifactInput),
    UnsupportedClaim(UnsupportedClaimInput),
    IdentityUse(IdentityUseInput),
    CapabilityShell(CapabilityShellInput),
    StackEvidence(StackEvidenceInput),
    AlgebraicEffect(Box<AlgebraicEffectInput>),
    ValenceReceiptExport(Box<ValenceReceiptExportInput>),
    FsmSpec(Box<FsmSpecInput>),
    FsmReceipt(Box<FsmReceiptInput>),
    DocumentationIr(Box<DocumentationIrInput>),
    DocumentationOutput(Box<DocumentationOutputInput>),
    DocumentationCell(Box<DocumentationCellInput>),
    DocumentationReceipt(Box<DocumentationReceiptInput>),
    ApplicationHostBinding(Box<ApplicationHostBindingInput>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceEnvelopeInput {
    pub schema_version: u32,
    pub language: String,
    pub media_type: String,
    pub parser_identity: String,
    pub source_bytes: Vec<u8>,
    pub declared_identity: WireIdentity,
    pub aliases: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CanonicalValueInput {
    pub schema_version: u32,
    pub canonical_bytes: Vec<u8>,
    pub declared_identity: WireIdentity,
    pub projection_identity: Option<WireIdentity>,
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
    pub execution_mode: String,
    pub source_bytes: Vec<u8>,
    pub source_identity: Option<WireIdentity>,
    pub evaluator_profile: String,
    pub evaluator_descriptor: String,
    pub evaluator_identity: Option<WireIdentity>,
    pub export_bytes: Vec<u8>,
    pub projection_identity: Option<WireIdentity>,
    pub declared_capabilities: Vec<String>,
    pub expected_effects: Vec<String>,
    pub sandbox_profile: Option<NickelSandboxProfile>,
    pub sandbox_evidence: Option<NickelSandboxEvidence>,
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
    pub source_identity: Option<WireIdentity>,
    pub evaluator_profile: String,
    pub evaluator_descriptor: String,
    pub evaluator_identity: Option<WireIdentity>,
    pub module_descriptor_bytes: Vec<u8>,
    pub module_identity: Option<WireIdentity>,
    pub input_bytes: Vec<u8>,
    pub input_identity: Option<WireIdentity>,
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
    pub output_projection_identity: Option<WireIdentity>,
    pub output_class: SteelOutputClass,
    pub claims_canonical_ir: bool,
    pub execution_evidence: SteelExecutionEvidence,
    pub checked_laws: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TargetArtifactInput {
    pub target_kind: String,
    pub output_bytes: Vec<u8>,
    pub declared_identity: WireIdentity,
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
    pub supplied_identity: WireIdentity,
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
    pub referenced_identities: Vec<WireIdentity>,
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
    pub effect_log_identity: Option<WireIdentity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AlgebraicEffectInput {
    pub schema_version: u32,
    pub canonical_bytes: Vec<u8>,
    pub declared_identity: WireIdentity,
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
pub struct FsmStateInput {
    pub state_id: String,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmEventInput {
    pub event_id: String,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmGuardInput {
    pub guard_id: String,
    pub required_evidence_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmSideEffectInput {
    pub effect_id: String,
    pub class: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmTransitionRowInput {
    pub from_state: String,
    pub event: String,
    pub to_state: String,
    pub guards: Vec<String>,
    pub side_effect: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmDenialRowInput {
    pub from_state: String,
    pub event: String,
    pub reason: String,
    pub guards: Vec<String>,
    pub side_effect: String,
    pub preserves_state: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmSpecInput {
    pub schema_version: u32,
    pub fsm_id: String,
    pub canonical_bytes: Vec<u8>,
    pub source_envelope_identity: Option<WireIdentity>,
    pub declared_identity: WireIdentity,
    pub states: Vec<FsmStateInput>,
    pub events: Vec<FsmEventInput>,
    pub initial_state: String,
    pub terminal_states: Vec<String>,
    pub intentionally_unreachable_states: Vec<String>,
    pub determinism_policy: String,
    pub totality_policy: String,
    pub guards: Vec<FsmGuardInput>,
    pub side_effects: Vec<FsmSideEffectInput>,
    pub transitions: Vec<FsmTransitionRowInput>,
    pub denials: Vec<FsmDenialRowInput>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmGuardDecisionInput {
    pub guard_id: String,
    pub allowed: bool,
    pub evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmTraceStepInput {
    pub from_state: String,
    pub event: String,
    pub to_state: String,
    pub transition_identity: Option<WireIdentity>,
    pub guard_decisions: Vec<FsmGuardDecisionInput>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FsmReceiptInput {
    pub schema_version: u32,
    pub receipt_kind: String,
    pub canonical_bytes: Vec<u8>,
    pub spec_identity: WireIdentity,
    pub state_universe_identity: WireIdentity,
    pub event_universe_identity: WireIdentity,
    pub transition_coverage_identity: WireIdentity,
    pub denial_coverage_identity: WireIdentity,
    pub adapter_identity: WireIdentity,
    pub dependency_identities: Vec<WireIdentity>,
    pub octet_receipt_identities: Vec<WireIdentity>,
    pub valence_export_identity: Option<WireIdentity>,
    pub checked_laws: Vec<String>,
    pub diagnostics: Vec<String>,
    pub non_claims: Vec<String>,
    pub semantic_claims: Vec<String>,
    pub declared_receipt_identity: Option<WireIdentity>,
    pub trace_start_state: Option<String>,
    pub trace_events: Vec<String>,
    pub trace_steps: Vec<FsmTraceStepInput>,
    pub trace_final_state: Option<String>,
    pub descriptor: Option<Box<FsmSpecInput>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocumentationSectionInput {
    pub section_id: String,
    pub anchor: String,
    pub title: String,
    pub prose_blocks: Vec<String>,
    pub code_blocks: Vec<String>,
    pub runnable_cell_ids: Vec<String>,
    pub links: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocumentationCellDescriptor {
    pub cell_id: String,
    pub language: String,
    pub source_identity: Option<WireIdentity>,
    pub required_capabilities: Vec<String>,
    pub expected_result: String,
    pub evaluator_identity: Option<WireIdentity>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocumentationPageInput {
    pub page_id: String,
    pub title: String,
    pub source_envelope_identity: Option<WireIdentity>,
    pub sections: Vec<DocumentationSectionInput>,
    pub cells: Vec<DocumentationCellDescriptor>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocumentationIrInput {
    pub schema_version: u32,
    pub canonical_bytes: Vec<u8>,
    pub declared_identity: WireIdentity,
    pub collector_identity: WireIdentity,
    pub pages: Vec<DocumentationPageInput>,
    pub target_formats: Vec<String>,
    pub dependency_identities: Vec<WireIdentity>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocumentationOutputInput {
    pub schema_version: u32,
    pub documentation_ir_identity: WireIdentity,
    pub renderer_identity: WireIdentity,
    pub target_format: String,
    pub output_bytes: Vec<u8>,
    pub declared_output_identity: WireIdentity,
    pub claims_canonical_ir: bool,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocumentationCellInput {
    pub schema_version: u32,
    pub cell_id: String,
    pub language: String,
    pub source_bytes: Vec<u8>,
    pub source_identity: Option<WireIdentity>,
    pub run_profile: String,
    pub execution_mode: String,
    pub evaluator_identity: Option<WireIdentity>,
    pub declared_capabilities: Vec<String>,
    pub observed_effects: Vec<String>,
    pub expected_result: String,
    pub expected_output_identity: Option<WireIdentity>,
    pub observed_output_bytes: Vec<u8>,
    pub observed_output_identity: Option<WireIdentity>,
    pub expected_diagnostic: Option<String>,
    pub observed_diagnostic: Option<String>,
    pub claims_semantic_equivalence: bool,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DocumentationReceiptInput {
    pub schema_version: u32,
    pub receipt_kind: String,
    pub canonical_bytes: Vec<u8>,
    pub documentation_ir_identity: WireIdentity,
    pub renderer_identity: Option<WireIdentity>,
    pub output_identities: Vec<WireIdentity>,
    pub cell_receipt_identities: Vec<WireIdentity>,
    pub valence_export_identity: Option<WireIdentity>,
    pub declared_receipt_identity: Option<WireIdentity>,
    pub diagnostics: Vec<String>,
    pub semantic_claims: Vec<String>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-algebraic-effect-v2")]
struct CanonicalAlgebraicEffect {
    schema_version: u32,
    signatures: Vec<EffectOperation>,
    input_row: Vec<String>,
    row_open: bool,
    claimed_pure: bool,
    handler: Option<HandlerDescriptorInput>,
    receipt: Option<HandlerReceiptInput>,
    replay: Option<ReplayProfileInput>,
    claimed_host_authority: Vec<String>,
    admitted_host_capabilities: Vec<String>,
}

impl From<&AlgebraicEffectInput> for CanonicalAlgebraicEffect {
    fn from(input: &AlgebraicEffectInput) -> Self {
        Self {
            schema_version: input.schema_version,
            signatures: input.signatures.clone(),
            input_row: input.input_row.clone(),
            row_open: input.row_open,
            claimed_pure: input.claimed_pure,
            handler: input.handler.clone(),
            receipt: input.receipt.clone(),
            replay: input.replay.clone(),
            claimed_host_authority: input.claimed_host_authority.clone(),
            admitted_host_capabilities: input.admitted_host_capabilities.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-fsm-spec-v2")]
struct CanonicalFsmSpec {
    schema_version: u32,
    fsm_id: String,
    source_envelope_identity: Option<WireIdentity>,
    states: Vec<FsmStateInput>,
    events: Vec<FsmEventInput>,
    initial_state: String,
    terminal_states: Vec<String>,
    intentionally_unreachable_states: Vec<String>,
    determinism_policy: String,
    totality_policy: String,
    guards: Vec<FsmGuardInput>,
    side_effects: Vec<FsmSideEffectInput>,
    transitions: Vec<FsmTransitionRowInput>,
    denials: Vec<FsmDenialRowInput>,
    checked_laws: Vec<String>,
    non_claims: Vec<String>,
}

impl From<&FsmSpecInput> for CanonicalFsmSpec {
    fn from(input: &FsmSpecInput) -> Self {
        Self {
            schema_version: input.schema_version,
            fsm_id: input.fsm_id.clone(),
            source_envelope_identity: input.source_envelope_identity.clone(),
            states: input.states.clone(),
            events: input.events.clone(),
            initial_state: input.initial_state.clone(),
            terminal_states: input.terminal_states.clone(),
            intentionally_unreachable_states: input.intentionally_unreachable_states.clone(),
            determinism_policy: input.determinism_policy.clone(),
            totality_policy: input.totality_policy.clone(),
            guards: input.guards.clone(),
            side_effects: input.side_effects.clone(),
            transitions: input.transitions.clone(),
            denials: input.denials.clone(),
            checked_laws: input.checked_laws.clone(),
            non_claims: input.non_claims.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-fsm-receipt-v3")]
struct CanonicalFsmReceipt {
    schema_version: u32,
    receipt_kind: String,
    spec_identity: WireIdentity,
    state_universe_identity: WireIdentity,
    event_universe_identity: WireIdentity,
    transition_coverage_identity: WireIdentity,
    denial_coverage_identity: WireIdentity,
    adapter_identity: WireIdentity,
    dependency_identities: Vec<WireIdentity>,
    octet_receipt_identities: Vec<WireIdentity>,
    valence_export_identity: Option<WireIdentity>,
    checked_laws: Vec<String>,
    diagnostics: Vec<String>,
    non_claims: Vec<String>,
    semantic_claims: Vec<String>,
    trace_start_state: Option<String>,
    trace_events: Vec<String>,
    trace_steps: Vec<FsmTraceStepInput>,
    trace_final_state: Option<String>,
}

impl From<&FsmReceiptInput> for CanonicalFsmReceipt {
    fn from(input: &FsmReceiptInput) -> Self {
        Self {
            schema_version: input.schema_version,
            receipt_kind: input.receipt_kind.clone(),
            spec_identity: input.spec_identity.clone(),
            state_universe_identity: input.state_universe_identity.clone(),
            event_universe_identity: input.event_universe_identity.clone(),
            transition_coverage_identity: input.transition_coverage_identity.clone(),
            denial_coverage_identity: input.denial_coverage_identity.clone(),
            adapter_identity: input.adapter_identity.clone(),
            dependency_identities: input.dependency_identities.clone(),
            octet_receipt_identities: input.octet_receipt_identities.clone(),
            valence_export_identity: input.valence_export_identity.clone(),
            checked_laws: input.checked_laws.clone(),
            diagnostics: input.diagnostics.clone(),
            non_claims: input.non_claims.clone(),
            semantic_claims: input.semantic_claims.clone(),
            trace_start_state: input.trace_start_state.clone(),
            trace_events: input.trace_events.clone(),
            trace_steps: input.trace_steps.clone(),
            trace_final_state: input.trace_final_state.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct FsmCoverageMaterial<T> {
    kind: String,
    entries: Vec<T>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FsmMatrixCell {
    pub state: String,
    pub event: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FsmCoverageEvidence {
    pub state_universe_identity: FsmUniverseIdentity,
    pub event_universe_identity: FsmUniverseIdentity,
    pub transition_coverage_identity: FsmCoverageIdentity,
    pub denial_coverage_identity: FsmCoverageIdentity,
    pub exclusion_coverage_identity: FsmCoverageIdentity,
    pub missing_cells_identity: FsmCoverageIdentity,
    pub missing_cells: Vec<FsmMatrixCell>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedFsmDescriptor {
    pub spec_identity: FsmSpecIdentity,
    pub initial_state: String,
    pub states: BTreeSet<String>,
    pub events: BTreeSet<String>,
    pub coverage: FsmCoverageEvidence,
    transitions: BTreeMap<(String, String, String), (FsmTransitionRowInput, FsmCoverageIdentity)>,
    guards: BTreeMap<String, FsmGuardInput>,
}

impl ValidatedFsmDescriptor {
    #[must_use]
    pub fn transition_identity(
        &self,
        from_state: &str,
        event: &str,
        to_state: &str,
    ) -> Option<&FsmCoverageIdentity> {
        self.transitions
            .get(&(
                from_state.to_string(),
                event.to_string(),
                to_state.to_string(),
            ))
            .map(|(_, identity)| identity)
    }
}

// r[impl kamacite.findings.fsm_receipts.descriptor_binding]
// r[impl kamacite.findings.fsm_receipts.coverage_recomputation]
pub fn validate_fsm_descriptor(
    input: &FsmSpecInput,
) -> Result<ValidatedFsmDescriptor, Vec<Diagnostic>> {
    let mut accumulator = ValidationAccumulator::default();
    validate_fsm_spec(input, &mut accumulator);
    if !accumulator.diagnostics.is_empty() {
        accumulator.diagnostics.sort();
        accumulator.diagnostics.dedup();
        return Err(accumulator.diagnostics);
    }
    decode_fsm_descriptor(input).map_err(|diagnostic| vec![diagnostic])?;
    let evidence = fsm_coverage_evidence(input).map_err(|diagnostic| vec![diagnostic])?;
    let mut transitions = BTreeMap::new();
    for transition in &input.transitions {
        let identity =
            fsm_transition_identity(transition).map_err(|diagnostic| vec![diagnostic])?;
        transitions.insert(
            (
                transition.from_state.clone(),
                transition.event.clone(),
                transition.to_state.clone(),
            ),
            (transition.clone(), identity),
        );
    }
    let guards = input
        .guards
        .iter()
        .map(|guard| (guard.guard_id.clone(), guard.clone()))
        .collect();
    Ok(ValidatedFsmDescriptor {
        spec_identity: FsmSpecIdentity::from_wire(input.declared_identity.clone()).map_err(
            |error| {
                vec![Diagnostic::new(
                    DiagnosticCode::IdentityDomainMismatch,
                    REQUIREMENT_TYPED_DOMAIN,
                    "fsm.declared_identity",
                    format!("FSM spec identity admission failed: {error}"),
                )]
            },
        )?,
        initial_state: input.initial_state.clone(),
        states: input
            .states
            .iter()
            .map(|state| state.state_id.clone())
            .collect(),
        events: input
            .events
            .iter()
            .map(|event| event.event_id.clone())
            .collect(),
        coverage: evidence,
        transitions,
        guards,
    })
}

fn decode_fsm_descriptor(input: &FsmSpecInput) -> Result<(), Diagnostic> {
    let artifact = canonicalize_preserves(
        &input.canonical_bytes,
        PreservesSyntax::Text,
        PreservesSchema {
            schema_version: FSM_SPEC_SCHEMA_VERSION,
            root_label: FSM_SPEC_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map_err(|error| {
        Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_FSM_DESCRIPTOR_BINDING,
            "fsm_receipt.descriptor.canonical_bytes",
            format!("FSM descriptor canonical decode failed: {error}"),
        )
    })?;
    let expected = encode_typed_preserves(
        &CanonicalFsmSpec::from(input),
        PreservesSchema {
            schema_version: FSM_SPEC_SCHEMA_VERSION,
            root_label: FSM_SPEC_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map_err(|error| {
        Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_FSM_DESCRIPTOR_BINDING,
            "fsm_receipt.descriptor",
            format!("FSM descriptor typed encoding failed: {error}"),
        )
    })?;
    if artifact.value != expected.value {
        return Err(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_FSM_DESCRIPTOR_BINDING,
            "fsm_receipt.descriptor",
            "decoded FSM descriptor value does not match the validated descriptor DTO",
        ));
    }
    Ok(())
}

fn fsm_coverage_evidence(input: &FsmSpecInput) -> Result<FsmCoverageEvidence, Diagnostic> {
    let mut states: Vec<String> = input
        .states
        .iter()
        .map(|state| state.state_id.clone())
        .collect();
    states.sort();
    let mut events: Vec<String> = input
        .events
        .iter()
        .map(|event| event.event_id.clone())
        .collect();
    events.sort();
    let mut transitions = input.transitions.clone();
    transitions.sort_by(|left, right| {
        (
            &left.from_state,
            &left.event,
            &left.to_state,
            &left.guards,
            &left.side_effect,
        )
            .cmp(&(
                &right.from_state,
                &right.event,
                &right.to_state,
                &right.guards,
                &right.side_effect,
            ))
    });
    let mut denials = input.denials.clone();
    denials.sort_by(|left, right| {
        (
            &left.from_state,
            &left.event,
            &left.reason,
            &left.guards,
            &left.side_effect,
            left.preserves_state,
        )
            .cmp(&(
                &right.from_state,
                &right.event,
                &right.reason,
                &right.guards,
                &right.side_effect,
                right.preserves_state,
            ))
    });
    let mut exclusions = input.intentionally_unreachable_states.clone();
    exclusions.sort();
    exclusions.dedup();

    let covered: BTreeSet<(String, String)> = input
        .transitions
        .iter()
        .map(|row| (row.from_state.clone(), row.event.clone()))
        .chain(
            input
                .denials
                .iter()
                .map(|row| (row.from_state.clone(), row.event.clone())),
        )
        .collect();
    let mut missing_cells = Vec::new();
    for state in &states {
        for event in &events {
            if !covered.contains(&(state.clone(), event.clone())) {
                missing_cells.push(FsmMatrixCell {
                    state: state.clone(),
                    event: event.clone(),
                });
            }
        }
    }

    Ok(FsmCoverageEvidence {
        state_universe_identity: fsm_material_identity(FSM_STATE_UNIVERSE_KIND, states)?,
        event_universe_identity: fsm_material_identity(FSM_EVENT_UNIVERSE_KIND, events)?,
        transition_coverage_identity: fsm_material_identity(
            FSM_TRANSITION_COVERAGE_KIND,
            transitions,
        )?,
        denial_coverage_identity: fsm_material_identity(FSM_DENIAL_COVERAGE_KIND, denials)?,
        exclusion_coverage_identity: fsm_material_identity(
            FSM_EXCLUSION_COVERAGE_KIND,
            exclusions,
        )?,
        missing_cells_identity: fsm_material_identity(
            FSM_MISSING_CELLS_KIND,
            missing_cells.clone(),
        )?,
        missing_cells,
    })
}

fn fsm_transition_identity(
    transition: &FsmTransitionRowInput,
) -> Result<FsmCoverageIdentity, Diagnostic> {
    fsm_material_identity(FSM_TRANSITION_KIND, vec![transition.clone()])
}

fn fsm_material_identity<D: IdentityDomainMarker, T: Serialize>(
    kind: &str,
    entries: Vec<T>,
) -> Result<Identity<D>, Diagnostic> {
    let material = FsmCoverageMaterial {
        kind: kind.to_string(),
        entries,
    };
    let encoded = encode_typed_preserves(
        &material,
        PreservesSchema {
            schema_version: FSM_COVERAGE_SCHEMA_VERSION,
            root_label: FSM_COVERAGE_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map_err(|error| {
        Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_FSM_COVERAGE_RECOMPUTATION,
            "fsm.coverage",
            format!("FSM coverage canonicalization failed: {error}"),
        )
    })?;
    Ok(Identity::<D>::compute(&encoded.packed_bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-documentation-ir-v2")]
struct CanonicalDocumentationIr {
    schema_version: u32,
    collector_identity: WireIdentity,
    pages: Vec<DocumentationPageInput>,
    target_formats: Vec<String>,
    dependency_identities: Vec<WireIdentity>,
    checked_laws: Vec<String>,
    non_claims: Vec<String>,
}

impl From<&DocumentationIrInput> for CanonicalDocumentationIr {
    fn from(input: &DocumentationIrInput) -> Self {
        Self {
            schema_version: input.schema_version,
            collector_identity: input.collector_identity.clone(),
            pages: input.pages.clone(),
            target_formats: input.target_formats.clone(),
            dependency_identities: input.dependency_identities.clone(),
            checked_laws: input.checked_laws.clone(),
            non_claims: input.non_claims.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-documentation-receipt-v2")]
struct CanonicalDocumentationReceipt {
    schema_version: u32,
    receipt_kind: String,
    documentation_ir_identity: WireIdentity,
    renderer_identity: Option<WireIdentity>,
    output_identities: Vec<WireIdentity>,
    cell_receipt_identities: Vec<WireIdentity>,
    valence_export_identity: Option<WireIdentity>,
    diagnostics: Vec<String>,
    semantic_claims: Vec<String>,
    checked_laws: Vec<String>,
    non_claims: Vec<String>,
}

impl From<&DocumentationReceiptInput> for CanonicalDocumentationReceipt {
    fn from(input: &DocumentationReceiptInput) -> Self {
        Self {
            schema_version: input.schema_version,
            receipt_kind: input.receipt_kind.clone(),
            documentation_ir_identity: input.documentation_ir_identity.clone(),
            renderer_identity: input.renderer_identity.clone(),
            output_identities: input.output_identities.clone(),
            cell_receipt_identities: input.cell_receipt_identities.clone(),
            valence_export_identity: input.valence_export_identity.clone(),
            diagnostics: input.diagnostics.clone(),
            semantic_claims: input.semantic_claims.clone(),
            checked_laws: input.checked_laws.clone(),
            non_claims: input.non_claims.clone(),
        }
    }
}

pub fn fixture_expected_canonical_artifact(
    payload: &FixturePayload,
) -> Result<Option<CanonicalPreservesArtifact>, PreservesCodecError> {
    let limits = PreservesLimits::default();
    match payload {
        FixturePayload::CanonicalValue(input) => canonicalize_preserves(
            &input.canonical_bytes,
            PreservesSyntax::Text,
            PreservesSchema {
                schema_version: CANONICAL_VALUE_SCHEMA_VERSION,
                root_label: CANONICAL_VALUE_ROOT_LABEL,
            },
            limits,
        )
        .map(Some),
        FixturePayload::AlgebraicEffect(input) => encode_typed_preserves(
            &CanonicalAlgebraicEffect::from(input.as_ref()),
            PreservesSchema {
                schema_version: ALGEBRAIC_EFFECT_SCHEMA_VERSION,
                root_label: ALGEBRAIC_EFFECT_ROOT_LABEL,
            },
            limits,
        )
        .map(Some),
        FixturePayload::FsmSpec(input) => encode_typed_preserves(
            &CanonicalFsmSpec::from(input.as_ref()),
            PreservesSchema {
                schema_version: FSM_SPEC_SCHEMA_VERSION,
                root_label: FSM_SPEC_ROOT_LABEL,
            },
            limits,
        )
        .map(Some),
        FixturePayload::FsmReceipt(input) => encode_typed_preserves(
            &CanonicalFsmReceipt::from(input.as_ref()),
            PreservesSchema {
                schema_version: FSM_RECEIPT_SCHEMA_VERSION,
                root_label: FSM_RECEIPT_ROOT_LABEL,
            },
            limits,
        )
        .map(Some),
        FixturePayload::DocumentationIr(input) => encode_typed_preserves(
            &CanonicalDocumentationIr::from(input.as_ref()),
            PreservesSchema {
                schema_version: DOCUMENTATION_IR_SCHEMA_VERSION,
                root_label: DOCUMENTATION_IR_ROOT_LABEL,
            },
            limits,
        )
        .map(Some),
        FixturePayload::DocumentationReceipt(input) => encode_typed_preserves(
            &CanonicalDocumentationReceipt::from(input.as_ref()),
            PreservesSchema {
                schema_version: DOCUMENTATION_RECEIPT_SCHEMA_VERSION,
                root_label: DOCUMENTATION_RECEIPT_ROOT_LABEL,
            },
            limits,
        )
        .map(Some),
        _ => Ok(None),
    }
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
    input_identities: BTreeMap<String, WireIdentity>,
    output_identities: BTreeMap<String, WireIdentity>,
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
        FixturePayload::AstGrepAudit(input) => {
            validate_ast_grep_audit_fixture(input, &mut accumulator)
        }
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
        FixturePayload::FsmSpec(input) => validate_fsm_spec(input, &mut accumulator),
        FixturePayload::FsmReceipt(input) => validate_fsm_receipt(input, &mut accumulator),
        FixturePayload::DocumentationIr(input) => {
            validate_documentation_ir(input, &mut accumulator)
        }
        FixturePayload::DocumentationOutput(input) => {
            validate_documentation_output(input, &mut accumulator)
        }
        FixturePayload::DocumentationCell(input) => {
            validate_documentation_cell(input, &mut accumulator)
        }
        FixturePayload::DocumentationReceipt(input) => {
            validate_documentation_receipt(input, &mut accumulator)
        }
        FixturePayload::ApplicationHostBinding(input) => {
            validate_application_host_binding_fixture(input, &mut accumulator)
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

fn canonicalize_fixture_bytes(
    input: &[u8],
    schema: PreservesSchema<'_>,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) -> Option<Vec<u8>> {
    match canonicalize_preserves(
        input,
        PreservesSyntax::Text,
        schema,
        PreservesLimits::default(),
    ) {
        Ok(artifact) => Some(artifact.packed_bytes),
        Err(error) => {
            push_preserves_diagnostic(error, field, accumulator);
            None
        }
    }
}

fn verify_fixture_bytes(
    input: &[u8],
    expected: Result<CanonicalPreservesArtifact, PreservesCodecError>,
    schema: PreservesSchema<'_>,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) -> Option<Vec<u8>> {
    let expected = match expected {
        Ok(expected) => expected,
        Err(error) => {
            push_preserves_diagnostic(error, field, accumulator);
            return None;
        }
    };
    let actual = canonicalize_fixture_bytes(input, schema, field, accumulator)?;
    if actual != expected.packed_bytes {
        push_preserves_diagnostic(
            PreservesCodecError::TypedContentMismatch,
            field,
            accumulator,
        );
        return None;
    }
    Some(actual)
}

fn push_preserves_diagnostic(
    error: PreservesCodecError,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) {
    let (code, requirement_id) = match &error {
        PreservesCodecError::InputLimit { .. }
        | PreservesCodecError::OutputLimit { .. }
        | PreservesCodecError::DepthLimit { .. }
        | PreservesCodecError::CollectionLimit { .. }
        | PreservesCodecError::AtomLimit { .. }
        | PreservesCodecError::ValueLimit { .. }
        | PreservesCodecError::InvalidPolicy(_) => (
            DiagnosticCode::ResourceLimitExceeded,
            REQUIREMENT_PRESERVES_VALIDATION,
        ),
        PreservesCodecError::NonCanonicalPacked => (
            DiagnosticCode::NonCanonicalInput,
            REQUIREMENT_PRESERVES_VALIDATION,
        ),
        PreservesCodecError::UnsupportedSchema { .. } => (
            DiagnosticCode::UnsupportedCanonicalCodec,
            REQUIREMENT_PRESERVES_REAL_CODEC,
        ),
        PreservesCodecError::TypedContentMismatch => (
            DiagnosticCode::CanonicalContentMismatch,
            REQUIREMENT_PRESERVES_TYPED_BINDING,
        ),
        PreservesCodecError::Malformed(_)
        | PreservesCodecError::UnsupportedAnnotation
        | PreservesCodecError::UnsupportedEmbeddedValue
        | PreservesCodecError::TypedDecode(_) => (
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_PRESERVES_VALIDATION,
        ),
    };
    accumulator.diagnostics.push(
        Diagnostic::new(
            code,
            requirement_id,
            field,
            "canonical Preserves validation failed",
        )
        .with_detail("error", error.to_string()),
    );
}

fn validate_canonical_value(input: &CanonicalValueInput, accumulator: &mut ValidationAccumulator) {
    validate_supported_schema(input.schema_version, accumulator);
    validate_identity_domain(
        &input.declared_identity,
        IdentityDomain::CanonicalValue,
        "canonical.declared_identity",
        accumulator,
    );
    accumulator.input_identities.insert(
        "declared-canonical".to_string(),
        input.declared_identity.clone(),
    );
    if let Some(canonical_bytes) = canonicalize_fixture_bytes(
        &input.canonical_bytes,
        PreservesSchema {
            schema_version: CANONICAL_VALUE_SCHEMA_VERSION,
            root_label: CANONICAL_VALUE_ROOT_LABEL,
        },
        "canonical.canonical_bytes",
        accumulator,
    ) {
        let computed = compute_identity(IdentityDomain::CanonicalValue, &canonical_bytes);
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
                "declared canonical identity does not match canonical Preserves packed bytes",
            ));
        }
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

fn validate_ast_grep_audit_fixture(
    input: &AstGrepAuditInput,
    accumulator: &mut ValidationAccumulator,
) {
    accumulator.input_identities.insert(
        "ast-grep-profile".to_string(),
        input.profile_identity.clone(),
    );
    accumulator.input_identities.insert(
        "ast-grep-rule-bundle".to_string(),
        input.rule_bundle_identity.clone(),
    );
    accumulator.input_identities.insert(
        "ast-grep-source-scope".to_string(),
        input.source_scope_identity.clone(),
    );
    for source in &input.sources {
        accumulator.input_identities.insert(
            format!("ast-grep-source:{}", source.path),
            source.identity.clone(),
        );
    }
    accumulator.output_identities.insert(
        "ast-grep-findings".to_string(),
        input.findings_identity.clone(),
    );
    accumulator.output_identities.insert(
        "ast-grep-valence-projection".to_string(),
        input.valence_projection_identity.clone(),
    );
    if let Some(codemod) = &input.codemod {
        accumulator.output_identities.insert(
            "ast-grep-codemod-recipe".to_string(),
            codemod.recipe_identity.clone(),
        );
    }
    accumulator
        .diagnostics
        .extend(validate_ast_grep_audit(input));
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
    validate_nickel_sandbox_evidence(input, accumulator);
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

fn validate_nickel_sandbox_evidence(
    input: &NickelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.declared_capabilities.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_NICKEL_CAPABILITY_BOUNDARY,
            "nickel.declared_capabilities",
            "Nickel adapter receipts must declare requested shell capabilities",
        ));
    }
    let (Some(profile), Some(evidence)) = (&input.sandbox_profile, &input.sandbox_evidence) else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UndeclaredAuthority,
            REQUIREMENT_NICKEL_CAPABILITY_BOUNDARY,
            "nickel.sandbox_evidence",
            "Nickel adapter receipt lacks shell-enforced sandbox evidence",
        ));
        return;
    };
    accumulator.input_identities.insert(
        "nickel-sandbox-profile".to_string(),
        profile.profile_identity.to_wire(),
    );
    match nickel_sandbox_evidence_identity(evidence) {
        Ok(identity) => {
            accumulator
                .input_identities
                .insert("nickel-sandbox-evidence".to_string(), identity.to_wire());
        }
        Err(error) => accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_NICKEL_CAPABILITY_BOUNDARY,
            "nickel.sandbox_evidence",
            format!("Nickel sandbox evidence identity failed: {error}"),
        )),
    }
    for (path, identity) in &evidence.import_identities {
        accumulator
            .input_identities
            .insert(format!("nickel-import:{path}"), identity.clone());
    }
    if let Some(identity) = &evidence.stdout_identity {
        accumulator
            .output_identities
            .insert("nickel-sandbox-stdout".to_string(), identity.clone());
    }
    accumulator.output_identities.insert(
        "nickel-sandbox-stderr".to_string(),
        evidence.stderr_identity.clone(),
    );
    accumulator.diagnostics.extend(validate_nickel_sandbox(
        profile,
        evidence,
        &input.declared_capabilities,
        input.projection_identity.as_ref(),
        input.evaluator_identity.as_ref(),
    ));
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
    validate_steel_execution_evidence(input, accumulator);
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
    let input_contract = resolve_steel_dto_contract(&input.input_schema);
    let output_contract = resolve_steel_dto_contract(&input.output_schema);
    match &input_contract {
        Ok(contract) => {
            accumulator.input_identities.insert(
                "steel-input-contract".to_string(),
                contract.identity.to_wire(),
            );
        }
        Err(error) => accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_STEEL_SCHEMA_CONFORMANCE,
                "steel.input_schema",
                "Steel input schema must resolve to an admitted content-addressed contract",
            )
            .with_detail("error", error.to_string()),
        ),
    }
    match &output_contract {
        Ok(contract) => {
            accumulator.input_identities.insert(
                "steel-output-contract".to_string(),
                contract.identity.to_wire(),
            );
        }
        Err(error) => accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_STEEL_SCHEMA_CONFORMANCE,
                "steel.output_schema",
                "Steel output schema must resolve to an admitted content-addressed contract",
            )
            .with_detail("error", error.to_string()),
        ),
    }
    if !input.input_bytes.is_empty() {
        validate_steel_input_value(input, accumulator);
    }
    if let Err(error) = validate_steel_dto(&input.output_schema, &input.output_bytes) {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_STEEL_SCHEMA_CONFORMANCE,
                "steel.output_bytes",
                "Steel output bytes do not satisfy the admitted content-addressed contract",
            )
            .with_detail("error", error.to_string()),
        );
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

fn validate_steel_input_value(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    let Some(input_identity) = &input.input_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_STEEL_SCHEMA_CONFORMANCE,
            "steel.input_identity",
            "Steel input bytes must bind a projection-domain identity",
        ));
        return;
    };
    validate_identity_domain(
        input_identity,
        IdentityDomain::Projection,
        "steel.input_identity",
        accumulator,
    );
    let computed = compute_identity(IdentityDomain::Projection, &input.input_bytes);
    accumulator
        .output_identities
        .insert("computed-steel-input".to_string(), computed.clone());
    if input_identity != &computed {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_STEEL_SCHEMA_CONFORMANCE,
            "steel.input_identity",
            "Steel input identity does not match exact input bytes",
        ));
    }
    match validate_steel_dto(&input.input_schema, &input.input_bytes) {
        Ok(SteelDtoValue::Integer(_)) => {}
        Ok(_) => accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_STEEL_SCHEMA_CONFORMANCE,
            "steel.input_bytes",
            "executed Steel DTO probes require the admitted integer input contract",
        )),
        Err(error) => accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_STEEL_SCHEMA_CONFORMANCE,
                "steel.input_bytes",
                "Steel input bytes do not satisfy the admitted content-addressed contract",
            )
            .with_detail("error", error.to_string()),
        ),
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

// r[impl kamacite.findings.steel_execution.fresh_probe]
// r[impl kamacite.findings.steel_execution.runner_identity]
// r[impl kamacite.findings.steel_execution.capability_enforcement]
fn validate_steel_execution_evidence(
    input: &SteelAdapterReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    let evidence_bytes = match serde_json::to_vec(&input.execution_evidence) {
        Ok(bytes) => bytes,
        Err(error) => {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MalformedProjectionOutput,
                    REQUIREMENT_STEEL_FRESH_PROBE,
                    "steel.execution_evidence",
                    "Steel execution evidence must have a deterministic encoding",
                )
                .with_detail("error", error.to_string()),
            );
            return;
        }
    };
    let evidence_identity = compute_identity(IdentityDomain::Adapter, &evidence_bytes);
    match &input.execution_evidence {
        SteelExecutionEvidence::StoredEvidence {
            fresh,
            output_identity,
        } => {
            accumulator
                .checked_laws
                .push("steel-stored-evidence-explicitly-nonfresh".to_string());
            accumulator
                .output_identities
                .insert("steel-stored-evidence".to_string(), evidence_identity);
            if *fresh || output_identity.as_ref() != input.output_projection_identity.as_ref() {
                accumulator.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::StaleIdentity,
                    REQUIREMENT_STEEL_FRESH_PROBE,
                    "steel.execution_evidence",
                    "stored Steel evidence must remain explicitly non-fresh and bind its prerecorded output",
                ));
            }
        }
        SteelExecutionEvidence::ExecutedProbe(evidence) => {
            accumulator
                .checked_laws
                .push("steel-executed-probe".to_string());
            accumulator.output_identities.insert(
                "steel-executed-probe-evidence".to_string(),
                evidence_identity,
            );
            validate_executed_steel_evidence(input, evidence, accumulator);
        }
    }
}

fn validate_executed_steel_evidence(
    input: &SteelAdapterReceiptInput,
    evidence: &crate::steel_execution::SteelExecutedProbeEvidence,
    accumulator: &mut ValidationAccumulator,
) {
    let runner_identity = compute_identity(
        IdentityDomain::Evaluator,
        evidence.runner_descriptor.as_bytes(),
    );
    let input_contract = resolve_steel_dto_contract(&input.input_schema).ok();
    let output_contract = resolve_steel_dto_contract(&input.output_schema).ok();
    let output_byte_length = u64::try_from(input.output_bytes.len()).ok();
    let declared_host_functions = input
        .host_functions
        .iter()
        .map(|function| function.function_id.clone())
        .collect::<Vec<_>>();
    let identity_edges_match = input.source_identity.as_ref() == Some(&evidence.source_identity)
        && input.module_identity.as_ref() == Some(&evidence.module_identity)
        && input.input_identity.as_ref() == Some(&evidence.input_identity)
        && input.output_projection_identity.as_ref() == Some(&evidence.output_identity)
        && input_contract.as_ref().is_some_and(|contract| {
            contract.identity.to_wire() == evidence.input_contract_identity
        })
        && output_contract.as_ref().is_some_and(|contract| {
            contract.identity.to_wire() == evidence.output_contract_identity
        });
    let runner_matches = evidence.runner_profile == input.evaluator_profile
        && evidence.runner_descriptor == input.evaluator_descriptor
        && evidence.runner_identity == runner_identity
        && input.evaluator_identity.as_ref() == Some(&evidence.runner_identity);
    let authority_matches = evidence.declared_builtin_modules == input.declared_builtin_modules
        && evidence.registered_host_functions == declared_host_functions
        && evidence.observed_host_functions == input.observed_host_functions
        && evidence.enforced_capabilities == input.declared_capabilities
        && evidence.observed_effects == input.observed_effects
        && evidence.authority_enforced;
    let completion_matches = evidence.completed
        && evidence.exit_class == STEEL_SUCCESS_EXIT_CLASS
        && output_byte_length == Some(evidence.output_byte_length);
    if input.input_bytes.is_empty() || !identity_edges_match {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_STEEL_FRESH_PROBE,
            "steel.execution_evidence.identity_edges",
            "executed Steel evidence must bind exact source, module, input, contracts, and output",
        ));
    }
    if !runner_matches {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedEvaluatorIdentity,
            REQUIREMENT_STEEL_RUNNER_IDENTITY,
            "steel.execution_evidence.runner",
            "executed Steel evidence must derive evaluator identity from the admitted runner",
        ));
    }
    if !authority_matches {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UndeclaredAuthority,
            REQUIREMENT_STEEL_CAPABILITY_ENFORCEMENT,
            "steel.execution_evidence.authority",
            "executed Steel evidence does not match the shell-enforced module, function, and capability surface",
        ));
    }
    if !completion_matches {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_STEEL_FRESH_PROBE,
            "steel.execution_evidence.completion",
            "executed Steel evidence must record a completed successful run and exact output length",
        ));
    }
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

fn validate_application_host_binding_fixture(
    input: &ApplicationHostBindingInput,
    accumulator: &mut ValidationAccumulator,
) {
    let result = crate::application_host_binding::validate_application_host_binding(input);
    accumulator
        .diagnostics
        .extend(result.diagnostics.iter().cloned());
    accumulator.input_identities.insert(
        "application".to_string(),
        input.application_identity.clone(),
    );
    if let Some(host) = &input.host_profile_identity {
        accumulator
            .input_identities
            .insert("host-profile".to_string(), host.clone());
    }
    accumulator.input_identities.insert(
        "adapter-cohort".to_string(),
        input.adapter_cohort_identity.clone(),
    );
    accumulator.input_identities.insert(
        "resource-profile".to_string(),
        input.resource_profile_identity.clone(),
    );
    if let Some(receipt) = &result.receipt {
        accumulator.output_identities.insert(
            "application-host-binding".to_string(),
            receipt.body.binding_identity.to_wire(),
        );
        accumulator.output_identities.insert(
            "application-host-binding-receipt".to_string(),
            receipt.receipt_identity.to_wire(),
        );
        accumulator
            .checked_laws
            .extend(receipt.body.checked_laws.iter().cloned());
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
            .insert(export_identity_key(), export.export_identity.to_wire());
    }
}

fn validate_fsm_spec(input: &FsmSpecInput, accumulator: &mut ValidationAccumulator) {
    if input.schema_version != SUPPORTED_SCHEMA_VERSION {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedSchemaVersion,
            REQUIREMENT_FSM_SPEC_VERSION,
            "fsm.schema_version",
            "unsupported FSM spec schema version rejected",
        ));
    }
    validate_identity_domain(
        &input.declared_identity,
        IdentityDomain::FsmSpec,
        "fsm.declared_identity",
        accumulator,
    );
    validate_optional_source_identity(
        &input.source_envelope_identity,
        "fsm.source_envelope_identity",
        accumulator,
    );
    accumulator.input_identities.insert(
        "declared-fsm-spec".to_string(),
        input.declared_identity.clone(),
    );
    if let Some(canonical_bytes) = verify_fixture_bytes(
        &input.canonical_bytes,
        encode_typed_preserves(
            &CanonicalFsmSpec::from(input),
            PreservesSchema {
                schema_version: FSM_SPEC_SCHEMA_VERSION,
                root_label: FSM_SPEC_ROOT_LABEL,
            },
            PreservesLimits::default(),
        ),
        PreservesSchema {
            schema_version: FSM_SPEC_SCHEMA_VERSION,
            root_label: FSM_SPEC_ROOT_LABEL,
        },
        "fsm.canonical_bytes",
        accumulator,
    ) {
        let computed = compute_identity(IdentityDomain::FsmSpec, &canonical_bytes);
        accumulator
            .output_identities
            .insert("computed-fsm-spec".to_string(), computed.clone());
        if input.declared_identity.domain == IdentityDomain::FsmSpec
            && input.declared_identity.hex != computed.hex
        {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_FSM_SPEC_IDENTITY,
                "fsm.declared_identity.hex",
                "declared FSM spec identity does not match canonical Preserves packed bytes",
            ));
        }
    }
    validate_fsm_metadata(input, accumulator);
    let states = collect_fsm_states(&input.states, accumulator);
    let events = collect_fsm_events(&input.events, accumulator);
    let guards = collect_fsm_guards(&input.guards, accumulator);
    let side_effects = collect_fsm_side_effects(&input.side_effects, accumulator);
    validate_fsm_state_policy(input, &states, accumulator);
    let transition_pairs =
        validate_fsm_transitions(input, &states, &events, &guards, &side_effects, accumulator);
    let denial_pairs =
        validate_fsm_denials(input, &states, &events, &guards, &side_effects, accumulator);
    validate_fsm_pair_coverage(
        input,
        &states,
        &events,
        &transition_pairs,
        &denial_pairs,
        accumulator,
    );
    validate_fsm_reachability(input, &states, accumulator);
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_optional_source_identity(
    identity: &Option<WireIdentity>,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) {
    if let Some(identity) = identity {
        validate_identity_domain(identity, IdentityDomain::Source, field, accumulator);
        accumulator
            .input_identities
            .insert(field.to_string(), identity.clone());
    }
}

fn validate_fsm_metadata(input: &FsmSpecInput, accumulator: &mut ValidationAccumulator) {
    if input.fsm_id.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_FSM_SPEC_IDENTITY,
            "fsm.fsm_id",
            "FSM spec requires a stable machine identity label",
        ));
    }
    if input.canonical_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_SPEC_IDENTITY,
            "fsm.canonical_bytes",
            "FSM spec identity requires canonical Preserves bytes or supported canonical content",
        ));
    }
    if input.checked_laws.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingCheckedLaw,
            REQUIREMENT_FSM_VALIDATION,
            "fsm.checked_laws",
            "FSM validation evidence must name checked laws",
        ));
    }
    if input.non_claims.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_FSM_SPEC_IDENTITY,
            "fsm.non_claims",
            "FSM artifacts require explicit non-claims before trust",
        ));
    }
    if input.determinism_policy != FSM_DETERMINISTIC_POLICY {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedArtifactClass,
                "r[kamacite.fsms.determinism_policy.nondeterminism_explicit]",
                "fsm.determinism_policy",
                "only the deterministic FSM profile is trusted in this slice",
            )
            .with_detail("policy", input.determinism_policy.clone()),
        );
    }
    if input.totality_policy.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_FSM_COVERAGE,
            "fsm.totality_policy",
            "FSM specs must name the totality policy used for pair coverage",
        ));
    }
}

fn collect_fsm_states(
    states: &[FsmStateInput],
    accumulator: &mut ValidationAccumulator,
) -> BTreeSet<String> {
    let mut declared = BTreeSet::new();
    for state in states {
        if state.state_id.is_empty() || state.label.is_empty() {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_FSM_CLOSED_UNIVERSES,
                "fsm.states",
                "FSM state entries require identity and metadata label",
            ));
        }
        if !declared.insert(state.state_id.clone()) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::DuplicateArtifactIdentity,
                    REQUIREMENT_FSM_DUPLICATE_UNIVERSE,
                    "fsm.states",
                    "duplicate FSM state identity rejected before trust",
                )
                .with_detail("state", state.state_id.clone()),
            );
        }
    }
    if declared.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_CLOSED_UNIVERSES,
            "fsm.states",
            "FSM state universe must be closed and non-empty",
        ));
    }
    declared
}

fn collect_fsm_events(
    events: &[FsmEventInput],
    accumulator: &mut ValidationAccumulator,
) -> BTreeSet<String> {
    let mut declared = BTreeSet::new();
    for event in events {
        if event.event_id.is_empty() || event.label.is_empty() {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_FSM_CLOSED_UNIVERSES,
                "fsm.events",
                "FSM event entries require identity and metadata label",
            ));
        }
        if !declared.insert(event.event_id.clone()) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::DuplicateArtifactIdentity,
                    REQUIREMENT_FSM_DUPLICATE_UNIVERSE,
                    "fsm.events",
                    "duplicate FSM event identity rejected before trust",
                )
                .with_detail("event", event.event_id.clone()),
            );
        }
    }
    if declared.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_CLOSED_UNIVERSES,
            "fsm.events",
            "FSM event universe must be closed and non-empty",
        ));
    }
    declared
}

fn collect_fsm_guards(
    guards: &[FsmGuardInput],
    accumulator: &mut ValidationAccumulator,
) -> BTreeSet<String> {
    let mut declared = BTreeSet::new();
    for guard in guards {
        if guard.guard_id.is_empty() {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_FSM_GUARDS_EFFECTS,
                "fsm.guards",
                "guard vocabulary entries require stable identities",
            ));
        }
        if guard.non_claims.is_empty() {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::MissingNonClaim,
                REQUIREMENT_FSM_GUARDS_EFFECTS,
                "fsm.guards.non_claims",
                "guard descriptors require non-claims before they can partition coverage",
            ));
        }
        if !declared.insert(guard.guard_id.clone()) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::DuplicateArtifactIdentity,
                    REQUIREMENT_FSM_DUPLICATE_UNIVERSE,
                    "fsm.guards",
                    "duplicate FSM guard identity rejected before trust",
                )
                .with_detail("guard", guard.guard_id.clone()),
            );
        }
    }
    declared
}

fn collect_fsm_side_effects(
    effects: &[FsmSideEffectInput],
    accumulator: &mut ValidationAccumulator,
) -> BTreeMap<String, String> {
    let mut declared = BTreeMap::new();
    for effect in effects {
        if effect.effect_id.is_empty() || effect.class.is_empty() {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_FSM_GUARDS_EFFECTS,
                "fsm.side_effects",
                "side-effect entries require identity and class",
            ));
        }
        if declared
            .insert(effect.effect_id.clone(), effect.class.clone())
            .is_some()
        {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::DuplicateArtifactIdentity,
                    REQUIREMENT_FSM_DUPLICATE_UNIVERSE,
                    "fsm.side_effects",
                    "duplicate FSM side-effect identity rejected before trust",
                )
                .with_detail("effect", effect.effect_id.clone()),
            );
        }
        if is_ambient_steel_authority(&effect.class) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::HostCapabilityConfusion,
                    "r[kamacite.fsms.fixture_corpus.negative_host_confusion]",
                    "fsm.side_effects.class",
                    "FSM side-effect class is symbolic data and does not grant host authority",
                )
                .with_detail("effect", effect.effect_id.clone())
                .with_detail("class", effect.class.clone()),
            );
        }
    }
    if declared.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_GUARDS_EFFECTS,
            "fsm.side_effects",
            "FSM specs must declare side-effect classes before rows reference them",
        ));
    }
    declared
}

fn validate_fsm_state_policy(
    input: &FsmSpecInput,
    states: &BTreeSet<String>,
    accumulator: &mut ValidationAccumulator,
) {
    if input.initial_state.is_empty() || !states.contains(&input.initial_state) {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            "r[kamacite.fsms.descriptor_vocabulary.initial_state_required]",
            "fsm.initial_state",
            "FSM descriptor initial state must be present in the closed state set",
        ));
    }
    for state in input
        .terminal_states
        .iter()
        .chain(input.intentionally_unreachable_states.iter())
    {
        if !states.contains(state) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_FSM_UNKNOWN_ENDPOINT,
                    "fsm.state_policy",
                    "FSM state policy references a state outside the closed universe",
                )
                .with_detail("state", state.clone()),
            );
        }
    }
}

fn validate_fsm_transitions(
    input: &FsmSpecInput,
    states: &BTreeSet<String>,
    events: &BTreeSet<String>,
    guards: &BTreeSet<String>,
    side_effects: &BTreeMap<String, String>,
    accumulator: &mut ValidationAccumulator,
) -> BTreeSet<(String, String)> {
    let terminals: BTreeSet<&String> = input.terminal_states.iter().collect();
    let mut seen_keys = BTreeSet::new();
    let mut covered_pairs = BTreeSet::new();
    for transition in &input.transitions {
        validate_fsm_row_symbols(
            &transition.from_state,
            &transition.event,
            states,
            events,
            "fsm.transitions",
            accumulator,
        );
        if is_fsm_wildcard(&transition.to_state) {
            push_fsm_wildcard_diagnostic(
                "fsm.transitions.to_state",
                &transition.to_state,
                accumulator,
            );
        } else if !states.contains(&transition.to_state) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_FSM_UNKNOWN_ENDPOINT,
                    "fsm.transitions.to_state",
                    "FSM transition target state must be declared",
                )
                .with_detail("state", transition.to_state.clone()),
            );
        }
        validate_fsm_row_guards(
            &transition.guards,
            guards,
            "fsm.transitions.guards",
            accumulator,
        );
        validate_fsm_row_side_effect(
            &transition.side_effect,
            side_effects,
            "fsm.transitions.side_effect",
            accumulator,
        );
        if terminals.contains(&transition.from_state) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UnsupportedArtifactClass,
                    REQUIREMENT_FSM_TERMINAL,
                    "fsm.transitions.from_state",
                    "terminal FSM states cannot have outgoing transitions without a separate accepted restart policy",
                )
                .with_detail("state", transition.from_state.clone()),
            );
        }
        let mut guard_partition = transition.guards.clone();
        guard_partition.sort();
        guard_partition.dedup();
        let key = (
            transition.from_state.clone(),
            transition.event.clone(),
            guard_partition.join("+"),
        );
        if !seen_keys.insert(key) {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::DuplicateArtifactIdentity,
                "r[kamacite.fsms.determinism_policy.conflict]",
                "fsm.transitions",
                "deterministic FSM profile rejects duplicate or conflicting transition rows",
            ));
        }
        covered_pairs.insert((transition.from_state.clone(), transition.event.clone()));
    }
    covered_pairs
}

fn validate_fsm_denials(
    input: &FsmSpecInput,
    states: &BTreeSet<String>,
    events: &BTreeSet<String>,
    guards: &BTreeSet<String>,
    side_effects: &BTreeMap<String, String>,
    accumulator: &mut ValidationAccumulator,
) -> BTreeSet<(String, String)> {
    let mut covered_pairs = BTreeSet::new();
    let mut seen_pairs = BTreeSet::new();
    for denial in &input.denials {
        validate_fsm_row_symbols(
            &denial.from_state,
            &denial.event,
            states,
            events,
            "fsm.denials",
            accumulator,
        );
        validate_fsm_row_guards(&denial.guards, guards, "fsm.denials.guards", accumulator);
        validate_fsm_denial_side_effect(denial, side_effects, accumulator);
        if denial.reason.is_empty() || !denial.preserves_state {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_FSM_DENIAL_BOUNDARY,
                "fsm.denials",
                "denial rows must name a reason and preserve prior state",
            ));
        }
        let pair = (denial.from_state.clone(), denial.event.clone());
        if !seen_pairs.insert(pair.clone()) {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::DuplicateArtifactIdentity,
                REQUIREMENT_FSM_DUPLICATE_COVERAGE,
                "fsm.denials",
                "duplicate denial coverage for the same state/event pair rejected",
            ));
        }
        covered_pairs.insert(pair);
    }
    covered_pairs
}

fn validate_fsm_row_symbols(
    state: &str,
    event: &str,
    states: &BTreeSet<String>,
    events: &BTreeSet<String>,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) {
    if is_fsm_wildcard(state) {
        push_fsm_wildcard_diagnostic(field, state, accumulator);
    } else if !states.contains(state) {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_FSM_UNKNOWN_ENDPOINT,
                field,
                "FSM row source state must be declared",
            )
            .with_detail("state", state.to_string()),
        );
    }
    if is_fsm_wildcard(event) {
        push_fsm_wildcard_diagnostic(field, event, accumulator);
    } else if !events.contains(event) {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_FSM_UNKNOWN_ENDPOINT,
                field,
                "FSM row event must be declared",
            )
            .with_detail("event", event.to_string()),
        );
    }
}

fn validate_fsm_row_guards(
    row_guards: &[String],
    guards: &BTreeSet<String>,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) {
    for guard in row_guards {
        if !guards.contains(guard) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredAuthority,
                    REQUIREMENT_FSM_UNDECLARED_GUARD,
                    field,
                    "FSM row references an undeclared guard descriptor",
                )
                .with_detail("guard", guard.clone()),
            );
        }
    }
}

fn validate_fsm_row_side_effect(
    effect: &str,
    side_effects: &BTreeMap<String, String>,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) {
    if !side_effects.contains_key(effect) {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UndeclaredCapability,
                REQUIREMENT_FSM_GUARDS_EFFECTS,
                field,
                "FSM row references an undeclared side-effect class",
            )
            .with_detail("effect", effect.to_string()),
        );
    }
}

fn validate_fsm_denial_side_effect(
    denial: &FsmDenialRowInput,
    side_effects: &BTreeMap<String, String>,
    accumulator: &mut ValidationAccumulator,
) {
    validate_fsm_row_side_effect(
        &denial.side_effect,
        side_effects,
        "fsm.denials.side_effect",
        accumulator,
    );
    let Some(effect_class) = side_effects.get(&denial.side_effect) else {
        return;
    };
    if effect_class != FSM_NO_SIDE_EFFECT_CLASS && effect_class != FSM_DENIAL_SIDE_EFFECT_CLASS {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::HostCapabilityConfusion,
                REQUIREMENT_FSM_DENIAL_BOUNDARY,
                "fsm.denials.side_effect",
                "denial rows must use a declared no-side-effect or denial side-effect class",
            )
            .with_detail("effect", denial.side_effect.clone())
            .with_detail("class", effect_class.clone()),
        );
    }
}

fn is_fsm_wildcard(value: &str) -> bool {
    FSM_WILDCARD_MARKERS.contains(&value)
}

fn push_fsm_wildcard_diagnostic(field: &str, value: &str, accumulator: &mut ValidationAccumulator) {
    accumulator.diagnostics.push(
        Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_FSM_NO_WILDCARDS,
            field,
            "FSM specs reject wildcard, default, otherwise, or catch-all rows",
        )
        .with_detail("value", value.to_string()),
    );
}

fn validate_fsm_pair_coverage(
    input: &FsmSpecInput,
    states: &BTreeSet<String>,
    events: &BTreeSet<String>,
    transition_pairs: &BTreeSet<(String, String)>,
    denial_pairs: &BTreeSet<(String, String)>,
    accumulator: &mut ValidationAccumulator,
) {
    let intentionally_unreachable: BTreeSet<&String> =
        input.intentionally_unreachable_states.iter().collect();
    for state in states {
        if intentionally_unreachable.contains(state) {
            continue;
        }
        for event in events {
            let pair = (state.clone(), event.clone());
            let transitioned = transition_pairs.contains(&pair);
            let denied = denial_pairs.contains(&pair);
            match (transitioned, denied) {
                (false, false) => accumulator.diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::MissingInput,
                        REQUIREMENT_FSM_MISSING_DENIAL,
                        "fsm.coverage",
                        "every reachable state/event pair must be covered by transition or explicit denial",
                    )
                    .with_detail("state", state.clone())
                    .with_detail("event", event.clone()),
                ),
                (true, true) => accumulator.diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::DuplicateArtifactIdentity,
                        REQUIREMENT_FSM_DUPLICATE_COVERAGE,
                        "fsm.coverage",
                        "state/event pair cannot be both admitted transition and explicit denial",
                    )
                    .with_detail("state", state.clone())
                    .with_detail("event", event.clone()),
                ),
                (true, false) | (false, true) => {}
            }
        }
    }
}

fn validate_fsm_reachability(
    input: &FsmSpecInput,
    states: &BTreeSet<String>,
    accumulator: &mut ValidationAccumulator,
) {
    if input.initial_state.is_empty() || !states.contains(&input.initial_state) {
        return;
    }
    let mut reachable = BTreeSet::new();
    let mut frontier = vec![input.initial_state.clone()];
    while let Some(state) = frontier.pop() {
        if !reachable.insert(state.clone()) {
            continue;
        }
        for transition in &input.transitions {
            if transition.from_state == state
                && states.contains(&transition.to_state)
                && !reachable.contains(&transition.to_state)
            {
                frontier.push(transition.to_state.clone());
            }
        }
    }
    let intentionally_unreachable: BTreeSet<&String> =
        input.intentionally_unreachable_states.iter().collect();
    for state in states {
        if !reachable.contains(state) && !intentionally_unreachable.contains(state) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_FSM_REACHABILITY,
                    "fsm.states",
                    "FSM state is unreachable from the initial state and lacks reviewed unreachable metadata",
                )
                .with_detail("state", state.clone()),
            );
        }
    }
}

fn validate_fsm_receipt(input: &FsmReceiptInput, accumulator: &mut ValidationAccumulator) {
    if input.schema_version != SUPPORTED_SCHEMA_VERSION {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedSchemaVersion,
            REQUIREMENT_FSM_SPEC_VERSION,
            "fsm_receipt.schema_version",
            "unsupported FSM receipt schema version rejected",
        ));
    }
    let descriptor = validate_bound_fsm_descriptor(input, accumulator);
    validate_fsm_receipt_identities(input, accumulator);
    validate_fsm_receipt_claims(input, accumulator);
    match input.receipt_kind.as_str() {
        FSM_EXHAUSTIVENESS_RECEIPT_KIND => {}
        FSM_TRACE_RECEIPT_KIND => {
            if let Some(descriptor) = descriptor.as_ref() {
                validate_fsm_trace_receipt(input, descriptor, accumulator);
            }
        }
        _ => accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedArtifactClass,
                REQUIREMENT_FSM_RECEIPT,
                "fsm_receipt.receipt_kind",
                "FSM receipt kind must be exhaustiveness or trace",
            )
            .with_detail("kind", input.receipt_kind.clone()),
        ),
    }
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_bound_fsm_descriptor(
    input: &FsmReceiptInput,
    accumulator: &mut ValidationAccumulator,
) -> Option<ValidatedFsmDescriptor> {
    let Some(descriptor_input) = input.descriptor.as_deref() else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_DESCRIPTOR_BINDING,
            "fsm_receipt.descriptor",
            "FSM receipt validation requires the bound descriptor value",
        ));
        return None;
    };
    let descriptor = match validate_fsm_descriptor(descriptor_input) {
        Ok(descriptor) => descriptor,
        Err(diagnostics) => {
            accumulator.diagnostics.extend(diagnostics);
            return None;
        }
    };
    compare_fsm_evidence_identity(
        &input.spec_identity,
        &descriptor.spec_identity,
        REQUIREMENT_FSM_DESCRIPTOR_BINDING,
        "fsm_receipt.spec_identity",
        accumulator,
    );
    compare_fsm_evidence_identity(
        &input.state_universe_identity,
        &descriptor.coverage.state_universe_identity,
        REQUIREMENT_FSM_COVERAGE_RECOMPUTATION,
        "fsm_receipt.state_universe_identity",
        accumulator,
    );
    compare_fsm_evidence_identity(
        &input.event_universe_identity,
        &descriptor.coverage.event_universe_identity,
        REQUIREMENT_FSM_COVERAGE_RECOMPUTATION,
        "fsm_receipt.event_universe_identity",
        accumulator,
    );
    compare_fsm_evidence_identity(
        &input.transition_coverage_identity,
        &descriptor.coverage.transition_coverage_identity,
        REQUIREMENT_FSM_COVERAGE_RECOMPUTATION,
        "fsm_receipt.transition_coverage_identity",
        accumulator,
    );
    compare_fsm_evidence_identity(
        &input.denial_coverage_identity,
        &descriptor.coverage.denial_coverage_identity,
        REQUIREMENT_FSM_COVERAGE_RECOMPUTATION,
        "fsm_receipt.denial_coverage_identity",
        accumulator,
    );
    if !descriptor.coverage.missing_cells.is_empty() {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_FSM_COVERAGE_RECOMPUTATION,
                "fsm_receipt.missing_cells",
                "bound FSM descriptor has uncovered state/event cells",
            )
            .with_detail(
                "missing_cells_identity",
                descriptor
                    .coverage
                    .missing_cells_identity
                    .hex()
                    .as_str()
                    .to_string(),
            ),
        );
    }
    accumulator.input_identities.insert(
        "fsm-exclusion-coverage".to_string(),
        descriptor.coverage.exclusion_coverage_identity.to_wire(),
    );
    accumulator.input_identities.insert(
        "fsm-missing-cells".to_string(),
        descriptor.coverage.missing_cells_identity.to_wire(),
    );
    Some(descriptor)
}

fn compare_fsm_evidence_identity<D: IdentityDomainMarker>(
    supplied: &WireIdentity,
    expected: &Identity<D>,
    requirement_id: &str,
    field: &str,
    accumulator: &mut ValidationAccumulator,
) {
    if supplied != &expected.to_wire() {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                requirement_id,
                field,
                "FSM receipt identity does not match evidence recomputed from the bound descriptor",
            )
            .with_detail("expected", expected.hex().as_str().to_string())
            .with_detail("supplied", supplied.hex.as_str().to_string()),
        );
    }
}

fn validate_fsm_receipt_identities(
    input: &FsmReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    validate_identity_domain(
        &input.spec_identity,
        IdentityDomain::FsmSpec,
        "fsm_receipt.spec_identity",
        accumulator,
    );
    validate_identity_domain(
        &input.state_universe_identity,
        IdentityDomain::FsmUniverse,
        "fsm_receipt.state_universe_identity",
        accumulator,
    );
    validate_identity_domain(
        &input.event_universe_identity,
        IdentityDomain::FsmUniverse,
        "fsm_receipt.event_universe_identity",
        accumulator,
    );
    validate_identity_domain(
        &input.transition_coverage_identity,
        IdentityDomain::FsmCoverage,
        "fsm_receipt.transition_coverage_identity",
        accumulator,
    );
    validate_identity_domain(
        &input.denial_coverage_identity,
        IdentityDomain::FsmCoverage,
        "fsm_receipt.denial_coverage_identity",
        accumulator,
    );
    validate_identity_domain(
        &input.adapter_identity,
        IdentityDomain::Adapter,
        "fsm_receipt.adapter_identity",
        accumulator,
    );
    accumulator
        .input_identities
        .insert("fsm-spec".to_string(), input.spec_identity.clone());
    accumulator.input_identities.insert(
        "fsm-state-universe".to_string(),
        input.state_universe_identity.clone(),
    );
    accumulator.input_identities.insert(
        "fsm-event-universe".to_string(),
        input.event_universe_identity.clone(),
    );
    accumulator.input_identities.insert(
        "fsm-transition-coverage".to_string(),
        input.transition_coverage_identity.clone(),
    );
    accumulator.input_identities.insert(
        "fsm-denial-coverage".to_string(),
        input.denial_coverage_identity.clone(),
    );
    accumulator
        .input_identities
        .insert("fsm-adapter".to_string(), input.adapter_identity.clone());
    for dependency in &input.dependency_identities {
        accumulator.input_identities.insert(
            format!("fsm-dependency:{}", dependency.domain),
            dependency.clone(),
        );
    }
    for octet_receipt in &input.octet_receipt_identities {
        validate_identity_domain(
            octet_receipt,
            IdentityDomain::Receipt,
            "fsm_receipt.octet_receipt_identities",
            accumulator,
        );
        accumulator.input_identities.insert(
            "octet-source-shape-receipt".to_string(),
            octet_receipt.clone(),
        );
    }
    if let Some(valence_export_identity) = &input.valence_export_identity {
        validate_identity_domain(
            valence_export_identity,
            IdentityDomain::Projection,
            "fsm_receipt.valence_export_identity",
            accumulator,
        );
        accumulator.input_identities.insert(
            "valence-fsm-export-projection".to_string(),
            valence_export_identity.clone(),
        );
    }
    let Some(receipt_identity) = &input.declared_receipt_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_RECEIPT_IDENTITY,
            "fsm_receipt.declared_receipt_identity",
            "FSM receipt must bind a separate receipt-domain identity",
        ));
        return;
    };
    validate_identity_domain(
        receipt_identity,
        IdentityDomain::Receipt,
        "fsm_receipt.declared_receipt_identity",
        accumulator,
    );
    accumulator
        .input_identities
        .insert("declared-fsm-receipt".to_string(), receipt_identity.clone());
    if let Some(canonical_bytes) = verify_fixture_bytes(
        &input.canonical_bytes,
        encode_typed_preserves(
            &CanonicalFsmReceipt::from(input),
            PreservesSchema {
                schema_version: FSM_RECEIPT_SCHEMA_VERSION,
                root_label: FSM_RECEIPT_ROOT_LABEL,
            },
            PreservesLimits::default(),
        ),
        PreservesSchema {
            schema_version: FSM_RECEIPT_SCHEMA_VERSION,
            root_label: FSM_RECEIPT_ROOT_LABEL,
        },
        "fsm_receipt.canonical_bytes",
        accumulator,
    ) {
        let computed = compute_identity(IdentityDomain::Receipt, &canonical_bytes);
        accumulator
            .output_identities
            .insert("computed-fsm-receipt".to_string(), computed.clone());
        if receipt_identity.domain == IdentityDomain::Receipt
            && receipt_identity.hex != computed.hex
        {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_FSM_RECEIPT_IDENTITY,
                "fsm_receipt.declared_receipt_identity.hex",
                "declared FSM receipt identity does not match canonical Preserves packed bytes",
            ));
        }
    }
    if receipt_identity.hex == input.spec_identity.hex
        || receipt_identity.hex == input.adapter_identity.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::ReceiptIdentityReuse,
            REQUIREMENT_FSM_RECEIPT_IDENTITY,
            "fsm_receipt.declared_receipt_identity",
            "FSM receipt identity must not reuse spec or adapter identity bytes",
        ));
    }
}

fn validate_fsm_receipt_claims(input: &FsmReceiptInput, accumulator: &mut ValidationAccumulator) {
    if input.canonical_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_RECEIPT,
            "fsm_receipt.canonical_bytes",
            "FSM receipts require canonical receipt bytes before identity can be checked",
        ));
    }
    if input.checked_laws.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingCheckedLaw,
            REQUIREMENT_FSM_RECEIPT,
            "fsm_receipt.checked_laws",
            "FSM receipts must bind checked laws",
        ));
    }
    if input.non_claims.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_FSM_RECEIPT,
            "fsm_receipt.non_claims",
            "FSM receipts require explicit non-claims",
        ));
    }
    for claim in &input.semantic_claims {
        if fsm_claim_is_forbidden(claim) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_FSM_STACK_VALENCE,
                    "fsm_receipt.semantic_claims",
                    "FSM receipt rejected runtime, liveness, release, or semantic-correctness overclaim",
                )
                .with_detail("claim", claim.clone()),
            );
        }
    }
    if !input.octet_receipt_identities.is_empty()
        && input
            .semantic_claims
            .iter()
            .any(|claim| claim.to_ascii_lowercase().contains("octet proves"))
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::SemanticOverclaim,
            REQUIREMENT_FSM_STACK_OCTET,
            "fsm_receipt.semantic_claims",
            "Octet-linked FSM receipts remain source-shape/provenance evidence only",
        ));
    }
}

fn fsm_claim_is_forbidden(claim: &str) -> bool {
    let normalized = claim.to_ascii_lowercase();
    FSM_FORBIDDEN_CLAIMS
        .iter()
        .any(|forbidden| normalized.contains(forbidden))
}

// r[impl kamacite.findings.fsm_receipts.trace_consistency]
fn validate_fsm_trace_receipt(
    input: &FsmReceiptInput,
    descriptor: &ValidatedFsmDescriptor,
    accumulator: &mut ValidationAccumulator,
) {
    let Some(start_state) = &input.trace_start_state else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_TRACE,
            "fsm_receipt.trace_start_state",
            "FSM trace receipts must bind a start state",
        ));
        return;
    };
    let Some(final_state) = &input.trace_final_state else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_FSM_TRACE,
            "fsm_receipt.trace_final_state",
            "FSM trace receipts must bind a final state",
        ));
        return;
    };
    if start_state != &descriptor.initial_state {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_FSM_TRACE_CONSISTENCY,
                "fsm_receipt.trace_start_state",
                "FSM trace start state does not match the descriptor initial state",
            )
            .with_detail("expected", descriptor.initial_state.clone())
            .with_detail("supplied", start_state.clone()),
        );
        return;
    }
    if input.trace_events.len() != input.trace_steps.len() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_FSM_TRACE_IMPOSSIBLE,
            "fsm_receipt.trace_steps",
            "FSM trace event sequence and observed transition sequence must align",
        ));
        return;
    }
    let mut current_state = start_state.clone();
    for (event, step) in input.trace_events.iter().zip(input.trace_steps.iter()) {
        if !descriptor.events.contains(event) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_FSM_TRACE_CONSISTENCY,
                    "fsm_receipt.trace_events",
                    "FSM trace event is not declared by the bound descriptor",
                )
                .with_detail("event", event.clone()),
            );
            return;
        }
        if step.from_state != current_state || step.event != *event {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_FSM_TRACE_IMPOSSIBLE,
                    "fsm_receipt.trace_steps",
                    "FSM trace transition is impossible from the current state and event",
                )
                .with_detail("current_state", current_state.clone())
                .with_detail("event", event.clone()),
            );
            return;
        }
        let key = (
            step.from_state.clone(),
            step.event.clone(),
            step.to_state.clone(),
        );
        let Some((transition, expected_identity)) = descriptor.transitions.get(&key) else {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_FSM_TRACE_CONSISTENCY,
                    "fsm_receipt.trace_steps",
                    "FSM trace step is not an enabled transition in the bound descriptor",
                )
                .with_detail("from_state", step.from_state.clone())
                .with_detail("event", step.event.clone())
                .with_detail("to_state", step.to_state.clone()),
            );
            return;
        };
        let Some(transition_identity) = &step.transition_identity else {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_FSM_TRACE_CONSISTENCY,
                "fsm_receipt.trace_steps.transition_identity",
                "FSM trace step must bind its descriptor transition identity",
            ));
            return;
        };
        compare_fsm_evidence_identity(
            transition_identity,
            expected_identity,
            REQUIREMENT_FSM_TRACE_CONSISTENCY,
            "fsm_receipt.trace_steps.transition_identity",
            accumulator,
        );
        validate_fsm_guard_decisions(step, transition, descriptor, accumulator);
        current_state = step.to_state.clone();
    }
    if &current_state != final_state {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_FSM_TRACE_IMPOSSIBLE,
                "fsm_receipt.trace_final_state",
                "FSM trace final state does not match observed transition path",
            )
            .with_detail("observed", current_state)
            .with_detail("declared", final_state.clone()),
        );
    }
}

fn validate_fsm_guard_decisions(
    step: &FsmTraceStepInput,
    transition: &FsmTransitionRowInput,
    descriptor: &ValidatedFsmDescriptor,
    accumulator: &mut ValidationAccumulator,
) {
    let decisions: BTreeMap<&str, &FsmGuardDecisionInput> = step
        .guard_decisions
        .iter()
        .map(|decision| (decision.guard_id.as_str(), decision))
        .collect();
    if decisions.len() != step.guard_decisions.len() || decisions.len() != transition.guards.len() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_FSM_TRACE_CONSISTENCY,
            "fsm_receipt.trace_steps.guard_decisions",
            "FSM trace guard decisions must uniquely cover the descriptor transition guards",
        ));
        return;
    }
    for guard_id in &transition.guards {
        let Some(guard) = descriptor.guards.get(guard_id) else {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::UndeclaredAuthority,
                REQUIREMENT_FSM_TRACE_CONSISTENCY,
                "fsm_receipt.trace_steps.guard_decisions",
                "FSM trace references an undeclared descriptor guard",
            ));
            continue;
        };
        let Some(decision) = decisions.get(guard_id.as_str()) else {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_FSM_TRACE_CONSISTENCY,
                "fsm_receipt.trace_steps.guard_decisions",
                "FSM trace is missing required guard-decision evidence",
            ));
            continue;
        };
        let mut expected_refs = guard.required_evidence_refs.clone();
        expected_refs.sort();
        expected_refs.dedup();
        let mut supplied_refs = decision.evidence_refs.clone();
        supplied_refs.sort();
        supplied_refs.dedup();
        if !decision.allowed || supplied_refs != expected_refs {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredAuthority,
                    REQUIREMENT_FSM_TRACE_CONSISTENCY,
                    "fsm_receipt.trace_steps.guard_decisions",
                    "FSM trace guard decision is denied or does not bind exact required evidence refs",
                )
                .with_detail("guard", guard_id.clone()),
            );
        }
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
    accumulator.input_identities.insert(
        "declared-algebraic-effect".to_string(),
        input.declared_identity.clone(),
    );
    if let Some(canonical_bytes) = verify_fixture_bytes(
        &input.canonical_bytes,
        encode_typed_preserves(
            &CanonicalAlgebraicEffect::from(input),
            PreservesSchema {
                schema_version: ALGEBRAIC_EFFECT_SCHEMA_VERSION,
                root_label: ALGEBRAIC_EFFECT_ROOT_LABEL,
            },
            PreservesLimits::default(),
        ),
        PreservesSchema {
            schema_version: ALGEBRAIC_EFFECT_SCHEMA_VERSION,
            root_label: ALGEBRAIC_EFFECT_ROOT_LABEL,
        },
        "algebraic.canonical_bytes",
        accumulator,
    ) {
        let computed = compute_identity(IdentityDomain::AlgebraicEffect, &canonical_bytes);
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
                "declared algebraic-effect identity does not match canonical Preserves packed bytes",
            ));
        }
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
        "non-replayable" if replay.effect_log_identity.is_some() => {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::NonReplayableEvidenceMisuse,
                "r[kamacite.algebraic_effects.replay_profile.non_replayable]",
                "algebraic.replay.effect_log_identity",
                "non-replayable effects cannot be accepted as replay evidence",
            ));
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
    identity: &WireIdentity,
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

pub fn render_documentation_projection(
    input: &DocumentationIrInput,
    target_format: &str,
) -> Result<Vec<u8>, Diagnostic> {
    if !DOC_SUPPORTED_RENDER_FORMATS.contains(&target_format) {
        return Err(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_DOCS_RENDERER_FORMAT,
            "documentation.target_format",
            "documentation renderer target format is unsupported",
        ));
    }

    let mut rendered = String::new();
    rendered.push_str("kamacite-doc-projection-v1\n");
    rendered.push_str("format:");
    rendered.push_str(target_format);
    rendered.push('\n');
    rendered.push_str("ir:");
    rendered.push_str(input.declared_identity.hex.as_str());
    rendered.push('\n');
    for page in &input.pages {
        rendered.push_str("page:");
        rendered.push_str(&page.page_id);
        rendered.push(':');
        rendered.push_str(&page.title);
        rendered.push('\n');
        for section in &page.sections {
            rendered.push_str("section:");
            rendered.push_str(&section.section_id);
            rendered.push(':');
            rendered.push_str(&section.anchor);
            rendered.push(':');
            rendered.push_str(&section.title);
            rendered.push('\n');
            for block in &section.prose_blocks {
                rendered.push_str("prose:");
                rendered.push_str(block);
                rendered.push('\n');
            }
            for cell in &section.runnable_cell_ids {
                rendered.push_str("cell:");
                rendered.push_str(cell);
                rendered.push('\n');
            }
        }
    }
    Ok(rendered.into_bytes())
}

fn validate_documentation_ir(
    input: &DocumentationIrInput,
    accumulator: &mut ValidationAccumulator,
) {
    validate_documentation_schema(input.schema_version, accumulator);
    validate_identity_domain(
        &input.declared_identity,
        IdentityDomain::DocumentationIr,
        "documentation_ir.declared_identity",
        accumulator,
    );
    validate_identity_domain(
        &input.collector_identity,
        IdentityDomain::Adapter,
        "documentation_ir.collector_identity",
        accumulator,
    );
    validate_documentation_ir_identity(input, accumulator);
    validate_documentation_ir_pages(input, accumulator);
    validate_documentation_target_formats(&input.target_formats, accumulator);
    for identity in &input.dependency_identities {
        if identity.algorithm != "blake3" {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_DOCS_IR,
                "documentation_ir.dependency_identities",
                "documentation dependencies must use BLAKE3 identities",
            ));
        }
    }
    if input.non_claims.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_DOCS_NON_CLAIMS,
            "documentation_ir.non_claims",
            "documentation IR must preserve non-claims",
        ));
    }
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_documentation_ir_identity(
    input: &DocumentationIrInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.canonical_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_IR,
            "documentation_ir.canonical_bytes",
            "documentation IR canonical bytes are required",
        ));
        return;
    }
    accumulator.input_identities.insert(
        "declared-documentation-ir".to_string(),
        input.declared_identity.clone(),
    );
    if let Some(canonical_bytes) = verify_fixture_bytes(
        &input.canonical_bytes,
        encode_typed_preserves(
            &CanonicalDocumentationIr::from(input),
            PreservesSchema {
                schema_version: DOCUMENTATION_IR_SCHEMA_VERSION,
                root_label: DOCUMENTATION_IR_ROOT_LABEL,
            },
            PreservesLimits::default(),
        ),
        PreservesSchema {
            schema_version: DOCUMENTATION_IR_SCHEMA_VERSION,
            root_label: DOCUMENTATION_IR_ROOT_LABEL,
        },
        "documentation_ir.canonical_bytes",
        accumulator,
    ) {
        let computed = compute_identity(IdentityDomain::DocumentationIr, &canonical_bytes);
        accumulator
            .output_identities
            .insert("computed-documentation-ir".to_string(), computed.clone());
        if input.declared_identity.domain == IdentityDomain::DocumentationIr
            && input.declared_identity.hex != computed.hex
        {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_DOCS_RENDERER_DRIFT,
                "documentation_ir.declared_identity.hex",
                "declared documentation IR identity does not match canonical Preserves packed bytes",
            ));
        }
    }
}

fn validate_documentation_ir_pages(
    input: &DocumentationIrInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.pages.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_IR,
            "documentation_ir.pages",
            "documentation IR must contain at least one page",
        ));
        return;
    }

    let mut declared_targets = BTreeSet::new();
    let mut declared_cells = BTreeSet::new();
    let mut duplicate_ids = BTreeSet::new();

    for page in &input.pages {
        record_documentation_id(&page.page_id, &mut declared_targets, &mut duplicate_ids);
        if page.source_envelope_identity.is_none() {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_DOCS_SOURCE,
                "documentation_ir.pages.source_envelope_identity",
                "documentation page source envelope identity is required",
            ));
        }
        if let Some(identity) = &page.source_envelope_identity {
            validate_identity_domain(
                identity,
                IdentityDomain::Source,
                "documentation_ir.pages.source_envelope_identity",
                accumulator,
            );
        }
        for section in &page.sections {
            record_documentation_id(
                &section.section_id,
                &mut declared_targets,
                &mut duplicate_ids,
            );
            record_documentation_id(&section.anchor, &mut declared_targets, &mut duplicate_ids);
            if section.anchor.is_empty() {
                accumulator.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::IncompleteMetadata,
                    REQUIREMENT_DOCS_RENDERERS,
                    "documentation_ir.sections.anchor",
                    "documentation section anchors must be explicit",
                ));
            }
            for cell_id in &section.runnable_cell_ids {
                if cell_id.is_empty() {
                    accumulator.diagnostics.push(Diagnostic::new(
                        DiagnosticCode::IncompleteMetadata,
                        REQUIREMENT_DOCS_RUNNABLE,
                        "documentation_ir.sections.runnable_cell_ids",
                        "runnable cell references must be explicit",
                    ));
                }
            }
        }
        for cell in &page.cells {
            record_documentation_id(&cell.cell_id, &mut declared_cells, &mut duplicate_ids);
            validate_documentation_cell_descriptor(cell, accumulator);
        }
    }

    if !duplicate_ids.is_empty() {
        accumulator.diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::DuplicateArtifactIdentity,
                REQUIREMENT_DOCS_DUPLICATE_ID,
                "documentation_ir.ids",
                "documentation page, section, anchor, or cell identity is duplicated",
            )
            .with_detail(
                "ids",
                sorted_strings(duplicate_ids.into_iter().collect()).join(","),
            ),
        );
    }

    for page in &input.pages {
        for section in &page.sections {
            for link in &section.links {
                if !declared_targets.contains(link) {
                    accumulator.diagnostics.push(
                        Diagnostic::new(
                            DiagnosticCode::MissingInput,
                            REQUIREMENT_DOCS_UNKNOWN_LINK,
                            "documentation_ir.sections.links",
                            "documentation link target is unknown",
                        )
                        .with_detail("target", link.clone()),
                    );
                }
            }
            for cell_id in &section.runnable_cell_ids {
                if !declared_cells.contains(cell_id) {
                    accumulator.diagnostics.push(
                        Diagnostic::new(
                            DiagnosticCode::MissingInput,
                            REQUIREMENT_DOCS_RUNNABLE,
                            "documentation_ir.sections.runnable_cell_ids",
                            "documentation runnable cell target is unknown",
                        )
                        .with_detail("cell", cell_id.clone()),
                    );
                }
            }
        }
    }
}

fn record_documentation_id(
    value: &str,
    seen: &mut BTreeSet<String>,
    duplicates: &mut BTreeSet<String>,
) {
    if value.is_empty() {
        return;
    }
    if !seen.insert(value.to_string()) {
        duplicates.insert(value.to_string());
    }
}

fn validate_documentation_cell_descriptor(
    cell: &DocumentationCellDescriptor,
    accumulator: &mut ValidationAccumulator,
) {
    if !DOC_CELL_LANGUAGES.contains(&cell.language.as_str()) {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_DOCS_UNSUPPORTED_LANGUAGE,
            "documentation_ir.cells.language",
            "documentation runnable cell language is unsupported",
        ));
    }
    let Some(source_identity) = &cell.source_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_SOURCE,
            "documentation_ir.cells.source_identity",
            "documentation runnable cell source identity is required",
        ));
        return;
    };
    validate_identity_domain(
        source_identity,
        IdentityDomain::Source,
        "documentation_ir.cells.source_identity",
        accumulator,
    );
    validate_documentation_expected_result(&cell.expected_result, accumulator);
    if !cell.required_capabilities.is_empty() && cell.evaluator_identity.is_none() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedEvaluatorIdentity,
            REQUIREMENT_DOCS_MISSING_EVALUATOR,
            "documentation_ir.cells.evaluator_identity",
            "capability-backed runnable cells require evaluator identity",
        ));
    }
    if cell.non_claims.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_DOCS_NON_CLAIMS,
            "documentation_ir.cells.non_claims",
            "runnable cell descriptors must preserve non-claims",
        ));
    }
}

fn validate_documentation_target_formats(
    formats: &[String],
    accumulator: &mut ValidationAccumulator,
) {
    if formats.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_RENDERERS,
            "documentation_ir.target_formats",
            "documentation IR must declare at least one renderer target",
        ));
    }
    for format in formats {
        if !DOC_SUPPORTED_RENDER_FORMATS.contains(&format.as_str()) {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::UnsupportedArtifactClass,
                REQUIREMENT_DOCS_RENDERER_FORMAT,
                "documentation_ir.target_formats",
                "documentation renderer target format is unsupported",
            ));
        }
    }
}

fn validate_documentation_output(
    input: &DocumentationOutputInput,
    accumulator: &mut ValidationAccumulator,
) {
    validate_documentation_schema(input.schema_version, accumulator);
    validate_identity_domain(
        &input.documentation_ir_identity,
        IdentityDomain::DocumentationIr,
        "documentation_output.documentation_ir_identity",
        accumulator,
    );
    validate_identity_domain(
        &input.renderer_identity,
        IdentityDomain::Adapter,
        "documentation_output.renderer_identity",
        accumulator,
    );
    validate_identity_domain(
        &input.declared_output_identity,
        IdentityDomain::DocumentationOutput,
        "documentation_output.declared_output_identity",
        accumulator,
    );
    if !DOC_SUPPORTED_RENDER_FORMATS.contains(&input.target_format.as_str()) {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_DOCS_RENDERER_FORMAT,
            "documentation_output.target_format",
            "documentation output target format is unsupported",
        ));
    }
    if input.output_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_RENDERERS,
            "documentation_output.output_bytes",
            "documentation output bytes are required",
        ));
    }
    let computed = compute_identity(IdentityDomain::DocumentationOutput, &input.output_bytes);
    accumulator.input_identities.insert(
        "declared-documentation-output".to_string(),
        input.declared_output_identity.clone(),
    );
    accumulator.output_identities.insert(
        "computed-documentation-output".to_string(),
        computed.clone(),
    );
    if input.declared_output_identity.domain == IdentityDomain::DocumentationOutput
        && input.declared_output_identity.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::GoldenReceiptMismatch,
            REQUIREMENT_DOCS_RENDERER_DRIFT,
            "documentation_output.declared_output_identity.hex",
            "declared documentation output identity does not match output bytes",
        ));
    }
    if input.claims_canonical_ir {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::CanonicalIrOverclaim,
            REQUIREMENT_DOCS_OUTPUT_BOUNDARY,
            "documentation_output.claims_canonical_ir",
            "rendered documentation output must not claim canonical IR authority",
        ));
    }
    if input.non_claims.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_DOCS_NON_CLAIMS,
            "documentation_output.non_claims",
            "documentation output must preserve non-claims",
        ));
    }
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_documentation_cell(
    input: &DocumentationCellInput,
    accumulator: &mut ValidationAccumulator,
) {
    validate_documentation_schema(input.schema_version, accumulator);
    if input.cell_id.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_DOCS_RUNNABLE,
            "documentation_cell.cell_id",
            "documentation cell identity is required",
        ));
    }
    if !DOC_CELL_LANGUAGES.contains(&input.language.as_str()) {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_DOCS_UNSUPPORTED_LANGUAGE,
            "documentation_cell.language",
            "documentation runnable cell language is unsupported",
        ));
    }
    validate_documentation_cell_source(input, accumulator);
    validate_documentation_execution_mode(input, accumulator);
    validate_documentation_observed_effects(input, accumulator);
    validate_documentation_expected_result(&input.expected_result, accumulator);
    validate_documentation_observed_result(input, accumulator);
    if input.claims_semantic_equivalence {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::SemanticOverclaim,
            REQUIREMENT_DOCS_NON_CLAIMS,
            "documentation_cell.claims_semantic_equivalence",
            "documentation cells must not claim arbitrary semantic equivalence",
        ));
    }
    if input.non_claims.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_DOCS_NON_CLAIMS,
            "documentation_cell.non_claims",
            "documentation cell receipts must preserve non-claims",
        ));
    }
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_documentation_cell_source(
    input: &DocumentationCellInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.source_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingSourceBytes,
            REQUIREMENT_DOCS_SOURCE,
            "documentation_cell.source_bytes",
            "runnable documentation cell source bytes are required",
        ));
        return;
    }
    let Some(source_identity) = &input.source_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_SOURCE,
            "documentation_cell.source_identity",
            "runnable documentation cell source identity is required",
        ));
        return;
    };
    validate_identity_domain(
        source_identity,
        IdentityDomain::Source,
        "documentation_cell.source_identity",
        accumulator,
    );
    let computed = compute_identity(IdentityDomain::Source, &input.source_bytes);
    if source_identity.domain == IdentityDomain::Source && source_identity.hex != computed.hex {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_DOCS_STALE_SOURCE,
            "documentation_cell.source_identity.hex",
            "runnable documentation cell source identity does not match source bytes",
        ));
    }
}

fn validate_documentation_execution_mode(
    input: &DocumentationCellInput,
    accumulator: &mut ValidationAccumulator,
) {
    match input.execution_mode.as_str() {
        DOC_STORED_EVIDENCE_MODE => {
            if !input.observed_effects.is_empty() {
                accumulator.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::UndeclaredAuthority,
                    REQUIREMENT_DOCS_DEFAULT_DENY,
                    "documentation_cell.observed_effects",
                    "stored-evidence documentation mode must not execute cells",
                ));
            }
        }
        DOC_EXPLICIT_RUN_MODE => {
            if input.evaluator_identity.is_none() {
                accumulator.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::UnsupportedEvaluatorIdentity,
                    REQUIREMENT_DOCS_MISSING_EVALUATOR,
                    "documentation_cell.evaluator_identity",
                    "explicit runnable documentation execution requires evaluator identity",
                ));
            }
        }
        _ => accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedArtifactClass,
            REQUIREMENT_DOCS_DEFAULT_DENY,
            "documentation_cell.execution_mode",
            "documentation cell execution mode is unsupported",
        )),
    }
    if let Some(identity) = &input.evaluator_identity {
        validate_identity_domain(
            identity,
            IdentityDomain::Evaluator,
            "documentation_cell.evaluator_identity",
            accumulator,
        );
    }
}

fn validate_documentation_observed_effects(
    input: &DocumentationCellInput,
    accumulator: &mut ValidationAccumulator,
) {
    let declared: BTreeSet<_> = input.declared_capabilities.iter().cloned().collect();
    for effect in &input.observed_effects {
        if !declared.contains(effect) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredCapability,
                    REQUIREMENT_DOCS_UNDECLARED_CAPABILITY,
                    "documentation_cell.observed_effects",
                    "runnable documentation cell observed an undeclared capability",
                )
                .with_detail("effect", effect.clone()),
            );
        }
        if AMBIENT_STEEL_AUTHORITIES.contains(&effect.as_str()) && !declared.contains(effect) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UndeclaredAuthority,
                    REQUIREMENT_DOCS_AMBIENT_AUTHORITY,
                    "documentation_cell.observed_effects",
                    "runnable documentation cell requested ambient authority",
                )
                .with_detail("effect", effect.clone()),
            );
        }
    }
}

fn validate_documentation_expected_result(
    expected_result: &str,
    accumulator: &mut ValidationAccumulator,
) {
    if expected_result != DOC_EXPECTED_SUCCESS && expected_result != DOC_EXPECTED_FAILURE {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_DOCS_RUNNABLE,
            "documentation_cell.expected_result",
            "documentation cell expected result must be success or failure",
        ));
    }
}

fn validate_documentation_observed_result(
    input: &DocumentationCellInput,
    accumulator: &mut ValidationAccumulator,
) {
    if let Some(observed_identity) = &input.observed_output_identity {
        validate_identity_domain(
            observed_identity,
            IdentityDomain::DocumentationOutput,
            "documentation_cell.observed_output_identity",
            accumulator,
        );
        let computed = compute_identity(
            IdentityDomain::DocumentationOutput,
            &input.observed_output_bytes,
        );
        if observed_identity.domain == IdentityDomain::DocumentationOutput
            && observed_identity.hex != computed.hex
        {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::GoldenReceiptMismatch,
                REQUIREMENT_DOCS_OUTPUT_MISMATCH,
                "documentation_cell.observed_output_identity.hex",
                "runnable documentation observed output identity does not match bytes",
            ));
        }
    }
    match input.expected_result.as_str() {
        DOC_EXPECTED_SUCCESS => validate_documentation_success_result(input, accumulator),
        DOC_EXPECTED_FAILURE => validate_documentation_failure_result(input, accumulator),
        _ => {}
    }
}

fn validate_documentation_success_result(
    input: &DocumentationCellInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.observed_output_identity.is_none() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_OUTPUT_MISMATCH,
            "documentation_cell.observed_output_identity",
            "expected-success documentation cell requires observed output identity",
        ));
    }
    if let (Some(expected), Some(observed)) = (
        &input.expected_output_identity,
        &input.observed_output_identity,
    ) {
        validate_identity_domain(
            expected,
            IdentityDomain::DocumentationOutput,
            "documentation_cell.expected_output_identity",
            accumulator,
        );
        if expected.domain == IdentityDomain::DocumentationOutput
            && observed.domain == IdentityDomain::DocumentationOutput
            && expected.hex != observed.hex
        {
            accumulator.diagnostics.push(Diagnostic::new(
                DiagnosticCode::GoldenReceiptMismatch,
                REQUIREMENT_DOCS_OUTPUT_MISMATCH,
                "documentation_cell.expected_output_identity",
                "runnable documentation expected output did not match observed output",
            ));
        }
    }
}

fn validate_documentation_failure_result(
    input: &DocumentationCellInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.observed_output_identity.is_some() && input.observed_diagnostic.is_none() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::WrongExpectedVerdict,
            REQUIREMENT_DOCS_UNEXPECTED_SUCCESS,
            "documentation_cell.expected_result",
            "expected-failure documentation cell unexpectedly produced output",
        ));
    }
    if input.expected_diagnostic.is_none() || input.observed_diagnostic.is_none() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_RUNNABLE,
            "documentation_cell.observed_diagnostic",
            "expected-failure documentation cell requires expected and observed diagnostics",
        ));
        return;
    }
    if input.expected_diagnostic != input.observed_diagnostic {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::GoldenReceiptMismatch,
            REQUIREMENT_DOCS_OUTPUT_MISMATCH,
            "documentation_cell.observed_diagnostic",
            "runnable documentation observed diagnostic did not match expected diagnostic",
        ));
    }
}

fn validate_documentation_receipt(
    input: &DocumentationReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    validate_documentation_schema(input.schema_version, accumulator);
    if input.receipt_kind.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_DOCS_RECEIPT,
            "documentation_receipt.receipt_kind",
            "documentation receipt kind is required",
        ));
    }
    validate_identity_domain(
        &input.documentation_ir_identity,
        IdentityDomain::DocumentationIr,
        "documentation_receipt.documentation_ir_identity",
        accumulator,
    );
    if let Some(renderer_identity) = &input.renderer_identity {
        validate_identity_domain(
            renderer_identity,
            IdentityDomain::Adapter,
            "documentation_receipt.renderer_identity",
            accumulator,
        );
    }
    for output_identity in &input.output_identities {
        validate_identity_domain(
            output_identity,
            IdentityDomain::DocumentationOutput,
            "documentation_receipt.output_identities",
            accumulator,
        );
    }
    for cell_receipt_identity in &input.cell_receipt_identities {
        validate_identity_domain(
            cell_receipt_identity,
            IdentityDomain::Receipt,
            "documentation_receipt.cell_receipt_identities",
            accumulator,
        );
    }
    if let Some(valence_export_identity) = &input.valence_export_identity {
        validate_identity_domain(
            valence_export_identity,
            IdentityDomain::Projection,
            "documentation_receipt.valence_export_identity",
            accumulator,
        );
    }
    validate_documentation_receipt_identity(input, accumulator);
    validate_documentation_receipt_claims(input, accumulator);
    accumulator.checked_laws.extend(input.checked_laws.clone());
}

fn validate_documentation_receipt_identity(
    input: &DocumentationReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.canonical_bytes.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_RECEIPT,
            "documentation_receipt.canonical_bytes",
            "documentation receipt canonical bytes are required",
        ));
        return;
    }
    let computed = verify_fixture_bytes(
        &input.canonical_bytes,
        encode_typed_preserves(
            &CanonicalDocumentationReceipt::from(input),
            PreservesSchema {
                schema_version: DOCUMENTATION_RECEIPT_SCHEMA_VERSION,
                root_label: DOCUMENTATION_RECEIPT_ROOT_LABEL,
            },
            PreservesLimits::default(),
        ),
        PreservesSchema {
            schema_version: DOCUMENTATION_RECEIPT_SCHEMA_VERSION,
            root_label: DOCUMENTATION_RECEIPT_ROOT_LABEL,
        },
        "documentation_receipt.canonical_bytes",
        accumulator,
    )
    .map(|canonical_bytes| compute_identity(IdentityDomain::Receipt, &canonical_bytes));
    if let Some(computed) = &computed {
        accumulator.output_identities.insert(
            "computed-documentation-receipt".to_string(),
            computed.clone(),
        );
    }
    let Some(declared) = &input.declared_receipt_identity else {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_DOCS_RECEIPT,
            "documentation_receipt.declared_receipt_identity",
            "documentation receipt identity is required",
        ));
        return;
    };
    validate_identity_domain(
        declared,
        IdentityDomain::Receipt,
        "documentation_receipt.declared_receipt_identity",
        accumulator,
    );
    accumulator.input_identities.insert(
        "declared-documentation-receipt".to_string(),
        declared.clone(),
    );
    if let Some(computed) = computed
        && declared.domain == IdentityDomain::Receipt
        && declared.hex != computed.hex
    {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_DOCS_RECEIPT,
            "documentation_receipt.declared_receipt_identity.hex",
            "declared documentation receipt identity does not match canonical Preserves packed bytes",
        ));
    }
}

fn validate_documentation_receipt_claims(
    input: &DocumentationReceiptInput,
    accumulator: &mut ValidationAccumulator,
) {
    if input.non_claims.is_empty() {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingNonClaim,
            REQUIREMENT_DOCS_NON_CLAIMS,
            "documentation_receipt.non_claims",
            "documentation receipts must preserve non-claims",
        ));
    }
    for claim in &input.semantic_claims {
        if contains_forbidden_doc_claim(claim) {
            accumulator.diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_DOCS_NON_CLAIMS,
                    "documentation_receipt.semantic_claims",
                    "documentation receipt semantic claim is outside the bounded docs contract",
                )
                .with_detail("claim", claim.clone()),
            );
        }
    }
}

fn contains_forbidden_doc_claim(claim: &str) -> bool {
    let normalized = claim.to_ascii_lowercase();
    DOC_FORBIDDEN_CLAIMS
        .iter()
        .any(|forbidden| normalized.contains(forbidden))
}

fn validate_documentation_schema(schema_version: u32, accumulator: &mut ValidationAccumulator) {
    if schema_version != SUPPORTED_SCHEMA_VERSION {
        accumulator.diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedSchemaVersion,
            REQUIREMENT_DOCS_VERSION,
            "documentation.schema_version",
            "documentation artifact schema version is unsupported",
        ));
    }
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
    const STEEL_INPUT: &[u8] = b"{\"value\":1}\n";
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
            input.declared_identity = WireIdentity::new(IdentityDomain::Source, stale_hex);
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

    fn valid_nickel_adapter_fixture() -> Option<FixtureCase> {
        let profile = nickel_sandbox_profile().ok()?;
        let declared_capabilities = vec![
            FIXTURE_READ_CAPABILITY.to_string(),
            NICKEL_EXPORT_CAPABILITY.to_string(),
            CAPTURE_STDOUT_CAPABILITY.to_string(),
        ];
        let mut import_identities = BTreeMap::new();
        import_identities.insert(
            "source.ncl".to_string(),
            compute_identity(IdentityDomain::Source, VALID_SOURCE),
        );
        let evidence = NickelSandboxEvidence {
            profile_identity: profile.profile_identity.to_wire(),
            evaluator_identity: compute_identity(
                IdentityDomain::Evaluator,
                NICKEL_EVALUATOR_DESCRIPTOR.as_bytes(),
            ),
            import_identities,
            requested_capabilities: declared_capabilities.clone(),
            admitted_capabilities: profile.admitted_capabilities.clone(),
            expected_effects: declared_capabilities.clone(),
            observed_execution_facts: nickel_sandbox_observed_facts(),
            outcome: crate::nickel_sandbox::NickelSandboxOutcome::Success,
            exit_code: Some(0),
            stdout_identity: Some(compute_identity(IdentityDomain::Projection, VALID_EXPORT)),
            stderr_identity: compute_identity(IdentityDomain::Projection, b""),
            sandbox_enforced: true,
        };
        Some(FixtureCase {
            id: NICKEL_FIXTURE_ID.to_string(),
            fixture_class: FixtureClass::Positive,
            artifact_class: ArtifactClass::AdapterReceipt,
            requirement_ids: vec![NICKEL_REQUIREMENT_ID.to_string()],
            non_claims: vec![NICKEL_NON_CLAIM.to_string()],
            expected_verdict: ValidationVerdict::Accepted,
            checked_laws: vec![NICKEL_ADAPTER_LAW.to_string()],
            payload: FixturePayload::NickelAdapterReceipt(Box::new(NickelAdapterReceiptInput {
                schema_version: SUPPORTED_SCHEMA_VERSION,
                adapter_kind: NICKEL_ADAPTER_KIND.to_string(),
                execution_mode: "sandboxed-reference-export".to_string(),
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
                declared_capabilities,
                expected_effects: vec![
                    FIXTURE_READ_CAPABILITY.to_string(),
                    NICKEL_EXPORT_CAPABILITY.to_string(),
                    CAPTURE_STDOUT_CAPABILITY.to_string(),
                ],
                sandbox_profile: Some(profile),
                sandbox_evidence: Some(evidence),
                claims_canonical_ir: false,
                checked_laws: vec![NICKEL_ADAPTER_LAW.to_string()],
            })),
        })
    }

    #[test]
    fn accepts_nickel_adapter_export_evidence() {
        let Some(fixture) = valid_nickel_adapter_fixture() else {
            return;
        };
        let result = validate_fixture(&fixture);
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
        assert!(result.expectation_met);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn rejects_nickel_adapter_missing_source() {
        let Some(mut fixture) = valid_nickel_adapter_fixture() else {
            return;
        };
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
        let Some(mut fixture) = valid_nickel_adapter_fixture() else {
            return;
        };
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
        let Some(mut fixture) = valid_nickel_adapter_fixture() else {
            return;
        };
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
        let Some(mut fixture) = valid_nickel_adapter_fixture() else {
            return;
        };
        fixture.id = "unit.nickel.adapter.undeclared-effect".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::NickelAdapterReceipt(input) = &mut fixture.payload
            && let Some(evidence) = &mut input.sandbox_evidence
        {
            evidence
                .admitted_capabilities
                .push(NETWORK_CAPABILITY.to_string());
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
                input_bytes: STEEL_INPUT.to_vec(),
                input_identity: Some(compute_identity(IdentityDomain::Projection, STEEL_INPUT)),
                input_schema: crate::steel_execution::STEEL_DTO_INTEGER_SCHEMA.to_string(),
                output_schema: crate::steel_execution::STEEL_DTO_INTEGER_SCHEMA.to_string(),
                host_functions: vec![SteelHostFunctionInput {
                    function_id: STEEL_HOST_FUNCTION_ID.to_string(),
                    input_schema: crate::steel_execution::STEEL_DTO_INTEGER_SCHEMA.to_string(),
                    output_schema: crate::steel_execution::STEEL_DTO_INTEGER_SCHEMA.to_string(),
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
                execution_evidence: SteelExecutionEvidence::StoredEvidence {
                    fresh: false,
                    output_identity: Some(compute_identity(
                        IdentityDomain::Projection,
                        STEEL_OUTPUT,
                    )),
                },
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
    fn rejects_stored_steel_evidence_relabelled_as_fresh() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.stored-relabeled-fresh".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload
            && let SteelExecutionEvidence::StoredEvidence { fresh, .. } =
                &mut input.execution_evidence
        {
            *fresh = true;
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.requirement_id == REQUIREMENT_STEEL_FRESH_PROBE })
        );
    }

    #[test]
    fn rejects_steel_output_with_valid_json_but_wrong_contract_shape() {
        let mut fixture = valid_steel_adapter_fixture();
        fixture.id = "unit.steel.adapter.wrong-output-shape".to_string();
        fixture.fixture_class = FixtureClass::Negative;
        fixture.expected_verdict = ValidationVerdict::Rejected;
        if let FixturePayload::SteelAdapterReceipt(input) = &mut fixture.payload {
            input.output_bytes = br#"{"value":{"nested":2}}"#.to_vec();
            input.output_projection_identity = Some(compute_identity(
                IdentityDomain::Projection,
                &input.output_bytes,
            ));
            input.execution_evidence = SteelExecutionEvidence::StoredEvidence {
                fresh: false,
                output_identity: input.output_projection_identity.clone(),
            };
        }

        let result = validate_fixture(&fixture);

        assert!(result.expectation_met);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.requirement_id == REQUIREMENT_STEEL_SCHEMA_CONFORMANCE
        }));
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

    fn bind_algebraic_effect_canonical(input: &mut AlgebraicEffectInput) {
        let encoded = encode_typed_preserves(
            &CanonicalAlgebraicEffect::from(&*input),
            PreservesSchema {
                schema_version: ALGEBRAIC_EFFECT_SCHEMA_VERSION,
                root_label: ALGEBRAIC_EFFECT_ROOT_LABEL,
            },
            PreservesLimits::default(),
        );
        assert!(encoded.is_ok(), "test algebraic effect must encode");
        let Ok(encoded) = encoded else {
            return;
        };
        input.canonical_bytes = encoded.text_projection.into_bytes();
        input.declared_identity =
            compute_identity(IdentityDomain::AlgebraicEffect, &encoded.packed_bytes);
    }

    #[test]
    fn accepts_algebraic_exact_discharge() {
        let bytes = b"<effect positive>".to_vec();
        let identity = compute_identity(IdentityDomain::AlgebraicEffect, &bytes);
        let mut fixture = FixtureCase {
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
        if let FixturePayload::AlgebraicEffect(input) = &mut fixture.payload {
            bind_algebraic_effect_canonical(input);
        }
        let result = validate_fixture(&fixture);
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
        assert!(result.expectation_met);
    }

    #[test]
    fn rejects_host_capability_confusion() {
        let bytes = b"<effect host confusion>".to_vec();
        let identity = compute_identity(IdentityDomain::AlgebraicEffect, &bytes);
        let mut fixture = FixtureCase {
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
        if let FixturePayload::AlgebraicEffect(input) = &mut fixture.payload {
            bind_algebraic_effect_canonical(input);
        }
        let result = validate_fixture(&fixture);
        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::HostCapabilityConfusion)
        );
    }

    const FSM_SPEC_BYTES: &[u8] = b"(fsm-spec unit.lifecycle v1 complete)\n";
    const FSM_RECEIPT_BYTES: &[u8] = b"(fsm-receipt unit.lifecycle exhaustiveness v1)\n";
    const FSM_TRACE_BYTES: &[u8] = b"(fsm-trace unit.lifecycle start-stop v1)\n";

    fn bind_fsm_spec_canonical(input: &mut FsmSpecInput) {
        let encoded = encode_typed_preserves(
            &CanonicalFsmSpec::from(&*input),
            PreservesSchema {
                schema_version: FSM_SPEC_SCHEMA_VERSION,
                root_label: FSM_SPEC_ROOT_LABEL,
            },
            PreservesLimits::default(),
        );
        assert!(encoded.is_ok(), "test FSM spec must encode");
        let Ok(encoded) = encoded else {
            return;
        };
        input.canonical_bytes = encoded.text_projection.into_bytes();
        input.declared_identity = compute_identity(IdentityDomain::FsmSpec, &encoded.packed_bytes);
    }

    fn bind_fsm_receipt_canonical(input: &mut FsmReceiptInput) {
        let encoded = encode_typed_preserves(
            &CanonicalFsmReceipt::from(&*input),
            PreservesSchema {
                schema_version: FSM_RECEIPT_SCHEMA_VERSION,
                root_label: FSM_RECEIPT_ROOT_LABEL,
            },
            PreservesLimits::default(),
        );
        assert!(encoded.is_ok(), "test FSM receipt must encode");
        let Ok(encoded) = encoded else {
            return;
        };
        input.canonical_bytes = encoded.text_projection.into_bytes();
        input.declared_receipt_identity = Some(compute_identity(
            IdentityDomain::Receipt,
            &encoded.packed_bytes,
        ));
    }

    fn valid_fsm_spec_input() -> FsmSpecInput {
        let mut input = FsmSpecInput {
            schema_version: SUPPORTED_SCHEMA_VERSION,
            fsm_id: "unit.lifecycle".to_string(),
            canonical_bytes: FSM_SPEC_BYTES.to_vec(),
            source_envelope_identity: Some(compute_identity(IdentityDomain::Source, b"fsm-source")),
            declared_identity: compute_identity(IdentityDomain::FsmSpec, FSM_SPEC_BYTES),
            states: vec![
                FsmStateInput {
                    state_id: "idle".to_string(),
                    label: "Idle".to_string(),
                },
                FsmStateInput {
                    state_id: "active".to_string(),
                    label: "Active".to_string(),
                },
            ],
            events: vec![
                FsmEventInput {
                    event_id: "start".to_string(),
                    label: "Start".to_string(),
                },
                FsmEventInput {
                    event_id: "stop".to_string(),
                    label: "Stop".to_string(),
                },
            ],
            initial_state: "idle".to_string(),
            terminal_states: Vec::new(),
            intentionally_unreachable_states: Vec::new(),
            determinism_policy: FSM_DETERMINISTIC_POLICY.to_string(),
            totality_policy: "all-state-event-pairs-explicit".to_string(),
            guards: vec![FsmGuardInput {
                guard_id: "has-authority".to_string(),
                required_evidence_refs: vec!["authority-ref".to_string()],
                non_claims: vec!["guard evidence is not host authority".to_string()],
            }],
            side_effects: vec![
                FsmSideEffectInput {
                    effect_id: "none".to_string(),
                    class: FSM_NO_SIDE_EFFECT_CLASS.to_string(),
                },
                FsmSideEffectInput {
                    effect_id: "admit".to_string(),
                    class: "symbolic".to_string(),
                },
            ],
            transitions: vec![
                FsmTransitionRowInput {
                    from_state: "idle".to_string(),
                    event: "start".to_string(),
                    to_state: "active".to_string(),
                    guards: vec!["has-authority".to_string()],
                    side_effect: "admit".to_string(),
                },
                FsmTransitionRowInput {
                    from_state: "active".to_string(),
                    event: "stop".to_string(),
                    to_state: "idle".to_string(),
                    guards: Vec::new(),
                    side_effect: "admit".to_string(),
                },
            ],
            denials: vec![
                FsmDenialRowInput {
                    from_state: "idle".to_string(),
                    event: "stop".to_string(),
                    reason: "already-idle".to_string(),
                    guards: Vec::new(),
                    side_effect: "none".to_string(),
                    preserves_state: true,
                },
                FsmDenialRowInput {
                    from_state: "active".to_string(),
                    event: "start".to_string(),
                    reason: "already-active".to_string(),
                    guards: Vec::new(),
                    side_effect: "none".to_string(),
                    preserves_state: true,
                },
            ],
            checked_laws: vec!["total-pair-coverage".to_string()],
            non_claims: vec!["not runtime correctness".to_string()],
        };
        bind_fsm_spec_canonical(&mut input);
        input
    }

    fn fsm_fixture(input: FsmSpecInput, expected_verdict: ValidationVerdict) -> FixtureCase {
        FixtureCase {
            id: "unit.fsm.spec".to_string(),
            fixture_class: FixtureClass::Positive,
            artifact_class: ArtifactClass::FsmSpec,
            requirement_ids: vec![REQUIREMENT_FSM_SPEC_IDENTITY.to_string()],
            non_claims: vec!["not runtime correctness".to_string()],
            expected_verdict,
            checked_laws: vec!["total-pair-coverage".to_string()],
            payload: FixturePayload::FsmSpec(Box::new(input)),
        }
    }

    #[test]
    fn accepts_complete_fsm_spec() {
        let result = validate_fixture(&fsm_fixture(
            valid_fsm_spec_input(),
            ValidationVerdict::Accepted,
        ));
        assert!(result.expectation_met);
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
    }

    #[test]
    fn rejects_missing_denial_coverage_and_wildcards() {
        let mut input = valid_fsm_spec_input();
        input.denials.pop();
        input.transitions.push(FsmTransitionRowInput {
            from_state: "idle".to_string(),
            event: "*".to_string(),
            to_state: "active".to_string(),
            guards: Vec::new(),
            side_effect: "admit".to_string(),
        });
        bind_fsm_spec_canonical(&mut input);
        let mut fixture = fsm_fixture(input, ValidationVerdict::Rejected);
        fixture.fixture_class = FixtureClass::Negative;
        let result = validate_fixture(&fixture);
        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.requirement_id == REQUIREMENT_FSM_MISSING_DENIAL)
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.requirement_id == REQUIREMENT_FSM_NO_WILDCARDS)
        );
    }

    fn valid_fsm_receipt_input() -> Result<FsmReceiptInput, Vec<Diagnostic>> {
        let spec = valid_fsm_spec_input();
        let descriptor = validate_fsm_descriptor(&spec)?;
        let mut input = FsmReceiptInput {
            schema_version: SUPPORTED_SCHEMA_VERSION,
            receipt_kind: FSM_EXHAUSTIVENESS_RECEIPT_KIND.to_string(),
            canonical_bytes: FSM_RECEIPT_BYTES.to_vec(),
            spec_identity: spec.declared_identity.clone(),
            state_universe_identity: descriptor.coverage.state_universe_identity.to_wire(),
            event_universe_identity: descriptor.coverage.event_universe_identity.to_wire(),
            transition_coverage_identity: descriptor
                .coverage
                .transition_coverage_identity
                .to_wire(),
            denial_coverage_identity: descriptor.coverage.denial_coverage_identity.to_wire(),
            adapter_identity: compute_identity(IdentityDomain::Adapter, b"fsm-adapter"),
            dependency_identities: Vec::new(),
            octet_receipt_identities: vec![compute_identity(IdentityDomain::Receipt, b"octet-fsm")],
            valence_export_identity: Some(compute_identity(
                IdentityDomain::Projection,
                b"valence-fsm",
            )),
            checked_laws: vec!["total-pair-coverage".to_string()],
            diagnostics: Vec::new(),
            non_claims: vec!["not runtime correctness".to_string()],
            semantic_claims: Vec::new(),
            declared_receipt_identity: Some(compute_identity(
                IdentityDomain::Receipt,
                FSM_RECEIPT_BYTES,
            )),
            trace_start_state: None,
            trace_events: Vec::new(),
            trace_steps: Vec::new(),
            trace_final_state: None,
            descriptor: Some(Box::new(spec)),
        };
        bind_fsm_receipt_canonical(&mut input);
        Ok(input)
    }

    fn test_transition_identity(input: &FsmReceiptInput, index: usize) -> Option<WireIdentity> {
        let descriptor = input.descriptor.as_deref()?;
        let transition = descriptor.transitions.get(index)?;
        fsm_transition_identity(transition)
            .ok()
            .map(|typed| typed.to_wire())
    }

    fn fsm_receipt_fixture(
        input: FsmReceiptInput,
        expected_verdict: ValidationVerdict,
    ) -> FixtureCase {
        FixtureCase {
            id: "unit.fsm.receipt".to_string(),
            fixture_class: FixtureClass::Positive,
            artifact_class: ArtifactClass::FsmReceipt,
            requirement_ids: vec![REQUIREMENT_FSM_RECEIPT.to_string()],
            non_claims: vec!["not runtime correctness".to_string()],
            expected_verdict,
            checked_laws: vec!["total-pair-coverage".to_string()],
            payload: FixturePayload::FsmReceipt(Box::new(input)),
        }
    }

    #[test]
    fn accepts_fsm_receipt_with_octet_and_valence_links() {
        let input = valid_fsm_receipt_input();
        assert!(input.is_ok(), "{:?}", input.as_ref().err());
        let Ok(input) = input else {
            return;
        };
        let result = validate_fixture(&fsm_receipt_fixture(input, ValidationVerdict::Accepted));
        assert!(result.expectation_met);
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
    }

    #[test]
    fn rejects_fsm_receipt_overclaim() {
        let input = valid_fsm_receipt_input();
        assert!(input.is_ok());
        let Ok(mut input) = input else {
            return;
        };
        input
            .semantic_claims
            .push("proves protocol completeness and liveness".to_string());
        bind_fsm_receipt_canonical(&mut input);
        let mut fixture = fsm_receipt_fixture(input, ValidationVerdict::Rejected);
        fixture.fixture_class = FixtureClass::Negative;
        let result = validate_fixture(&fixture);
        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::SemanticOverclaim)
        );
    }

    fn valid_fsm_trace_receipt_input() -> Option<FsmReceiptInput> {
        let mut input = valid_fsm_receipt_input().ok()?;
        let start_identity = test_transition_identity(&input, 0)?;
        let stop_identity = test_transition_identity(&input, 1)?;
        input.receipt_kind = FSM_TRACE_RECEIPT_KIND.to_string();
        input.canonical_bytes = FSM_TRACE_BYTES.to_vec();
        input.declared_receipt_identity =
            Some(compute_identity(IdentityDomain::Receipt, FSM_TRACE_BYTES));
        input.trace_start_state = Some("idle".to_string());
        input.trace_events = vec!["start".to_string(), "stop".to_string()];
        input.trace_steps = vec![
            FsmTraceStepInput {
                from_state: "idle".to_string(),
                event: "start".to_string(),
                to_state: "active".to_string(),
                transition_identity: Some(start_identity),
                guard_decisions: vec![FsmGuardDecisionInput {
                    guard_id: "has-authority".to_string(),
                    allowed: true,
                    evidence_refs: vec!["authority-ref".to_string()],
                }],
            },
            FsmTraceStepInput {
                from_state: "active".to_string(),
                event: "stop".to_string(),
                to_state: "idle".to_string(),
                transition_identity: Some(stop_identity),
                guard_decisions: Vec::new(),
            },
        ];
        input.trace_final_state = Some("idle".to_string());
        bind_fsm_receipt_canonical(&mut input);
        Some(input)
    }

    #[test]
    fn accepts_fsm_trace_receipt_path() {
        let input = valid_fsm_trace_receipt_input();
        assert!(input.is_some());
        let Some(input) = input else {
            return;
        };
        let result = validate_fixture(&fsm_receipt_fixture(input, ValidationVerdict::Accepted));
        assert!(result.expectation_met);
        assert_eq!(result.observed_verdict, ValidationVerdict::Accepted);
    }

    #[test]
    fn rejects_impossible_fsm_trace_transition() {
        let input = valid_fsm_receipt_input();
        assert!(input.is_ok());
        let Ok(mut input) = input else {
            return;
        };
        input.receipt_kind = FSM_TRACE_RECEIPT_KIND.to_string();
        input.canonical_bytes = FSM_TRACE_BYTES.to_vec();
        input.declared_receipt_identity =
            Some(compute_identity(IdentityDomain::Receipt, FSM_TRACE_BYTES));
        input.trace_start_state = Some("idle".to_string());
        input.trace_events = vec!["stop".to_string()];
        input.trace_steps = vec![FsmTraceStepInput {
            from_state: "active".to_string(),
            event: "stop".to_string(),
            to_state: "idle".to_string(),
            transition_identity: None,
            guard_decisions: Vec::new(),
        }];
        input.trace_final_state = Some("idle".to_string());
        bind_fsm_receipt_canonical(&mut input);
        let mut fixture = fsm_receipt_fixture(input, ValidationVerdict::Rejected);
        fixture.fixture_class = FixtureClass::Negative;
        let result = validate_fixture(&fixture);
        assert!(result.expectation_met);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.requirement_id == REQUIREMENT_FSM_TRACE_IMPOSSIBLE)
        );
    }

    // r[verify kamacite.findings.fsm_receipts.descriptor_binding]
    // r[verify kamacite.findings.fsm_receipts.coverage_recomputation]
    // r[verify kamacite.findings.fsm_receipts.trace_consistency]
    // r[verify kamacite.findings.fsm_receipts.negative_coverage]
    #[test]
    fn rejects_descriptor_substitution_and_stale_coverage_roots() {
        let input = valid_fsm_receipt_input();
        assert!(input.is_ok());
        let Ok(mut substituted) = input else {
            return;
        };
        let Some(descriptor) = substituted.descriptor.as_deref_mut() else {
            return;
        };
        descriptor.fsm_id = "substituted-machine".to_string();
        bind_fsm_spec_canonical(descriptor);
        let fixture = fsm_receipt_fixture(substituted, ValidationVerdict::Rejected);
        let result = validate_fixture(&fixture);
        assert!(result.expectation_met);
        assert!(
            result.diagnostics.iter().any(|diagnostic| {
                diagnostic.requirement_id == REQUIREMENT_FSM_DESCRIPTOR_BINDING
            })
        );

        let input = valid_fsm_receipt_input();
        assert!(input.is_ok());
        let Ok(mut stale) = input else {
            return;
        };
        stale.transition_coverage_identity =
            compute_identity(IdentityDomain::FsmCoverage, b"stale-transition-root");
        bind_fsm_receipt_canonical(&mut stale);
        let fixture = fsm_receipt_fixture(stale, ValidationVerdict::Rejected);
        let result = validate_fixture(&fixture);
        assert!(result.expectation_met);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.requirement_id == REQUIREMENT_FSM_COVERAGE_RECOMPUTATION
        }));
    }

    #[test]
    fn rejects_wrong_start_undeclared_event_invented_transition_and_guard_evidence() {
        let input = valid_fsm_trace_receipt_input();
        assert!(input.is_some());
        let Some(mut wrong_start) = input else {
            return;
        };
        wrong_start.trace_start_state = Some("active".to_string());
        bind_fsm_receipt_canonical(&mut wrong_start);
        let result = validate_fixture(&fsm_receipt_fixture(
            wrong_start,
            ValidationVerdict::Rejected,
        ));
        assert!(result.expectation_met);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.requirement_id == REQUIREMENT_FSM_TRACE_CONSISTENCY
                && diagnostic.field == "fsm_receipt.trace_start_state"
        }));

        let input = valid_fsm_trace_receipt_input();
        assert!(input.is_some());
        let Some(mut undeclared_event) = input else {
            return;
        };
        let Some(event) = undeclared_event.trace_events.get_mut(0) else {
            return;
        };
        *event = "undeclared".to_string();
        let Some(step) = undeclared_event.trace_steps.get_mut(0) else {
            return;
        };
        step.event = "undeclared".to_string();
        bind_fsm_receipt_canonical(&mut undeclared_event);
        let result = validate_fixture(&fsm_receipt_fixture(
            undeclared_event,
            ValidationVerdict::Rejected,
        ));
        assert!(result.expectation_met);

        let input = valid_fsm_trace_receipt_input();
        assert!(input.is_some());
        let Some(mut invented) = input else {
            return;
        };
        let Some(first_step) = invented.trace_steps.get_mut(0) else {
            return;
        };
        first_step.to_state = "idle".to_string();
        let Some(second_step) = invented.trace_steps.get_mut(1) else {
            return;
        };
        second_step.from_state = "idle".to_string();
        bind_fsm_receipt_canonical(&mut invented);
        let result = validate_fixture(&fsm_receipt_fixture(invented, ValidationVerdict::Rejected));
        assert!(result.expectation_met);

        let input = valid_fsm_trace_receipt_input();
        assert!(input.is_some());
        let Some(mut missing_guard) = input else {
            return;
        };
        let Some(step) = missing_guard.trace_steps.get_mut(0) else {
            return;
        };
        step.guard_decisions.clear();
        bind_fsm_receipt_canonical(&mut missing_guard);
        let result = validate_fixture(&fsm_receipt_fixture(
            missing_guard,
            ValidationVerdict::Rejected,
        ));
        assert!(result.expectation_met);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.requirement_id == REQUIREMENT_FSM_TRACE_CONSISTENCY
                && diagnostic.field == "fsm_receipt.trace_steps.guard_decisions"
        }));
    }

    #[test]
    fn rejects_missing_descriptor_and_descriptor_missing_matrix_cell() {
        let input = valid_fsm_receipt_input();
        assert!(input.is_ok());
        let Ok(mut missing_descriptor) = input else {
            return;
        };
        missing_descriptor.descriptor = None;
        let result = validate_fixture(&fsm_receipt_fixture(
            missing_descriptor,
            ValidationVerdict::Rejected,
        ));
        assert!(result.expectation_met);

        let input = valid_fsm_receipt_input();
        assert!(input.is_ok());
        let Ok(mut missing_cell) = input else {
            return;
        };
        let Some(descriptor) = missing_cell.descriptor.as_deref_mut() else {
            return;
        };
        descriptor.denials.pop();
        bind_fsm_spec_canonical(descriptor);
        let result = validate_fixture(&fsm_receipt_fixture(
            missing_cell,
            ValidationVerdict::Rejected,
        ));
        assert!(result.expectation_met);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.requirement_id == REQUIREMENT_FSM_MISSING_DENIAL
                || diagnostic.requirement_id == REQUIREMENT_FSM_COVERAGE_RECOMPUTATION
        }));
    }
}
