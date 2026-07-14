#![forbid(unsafe_code)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::unwrap_used)]

mod diagnostic;
mod dialect_roundtrip;
mod docs_generation;
mod evidence_chain;
mod fixture;
mod function_address;
mod identity;
mod marigold_analysis;
mod proof_evidence;
mod receipt;
mod valence_export;

pub use diagnostic::{Diagnostic, DiagnosticCode};
pub use dialect_roundtrip::{
    DIALECT_ROUNDTRIP_CANONICAL_OWNER, DIALECT_ROUNDTRIP_JSON_PROJECTION,
    DIALECT_ROUNDTRIP_PROFILE_SCHEMA_VERSION, DialectRoundtripInput,
    DialectRoundtripJsonProjection, DialectRoundtripProfile, DialectRoundtripValenceProjection,
    DialectSourceSpan, dialect_payload_identity, dialect_roundtrip_json_projection,
    dialect_roundtrip_profile, dialect_roundtrip_valence_projection,
    validate_dialect_roundtrip_input, validate_dialect_roundtrip_json_projection,
};
pub use docs_generation::{
    DOCS_CONFIG_SCHEMA_VERSION, DOCUMENTATION_IR_SCHEMA_VERSION, DocsAssemblyInput,
    LANGUAGE_DOCUMENTATION_FACTS_SCHEMA_VERSION, LanguageDocumentationFactsArtifact,
    LanguageDocumentationFactsInput, LanguageDocumentationItem, RunnableCellReference,
    SourceByteSpan, WasmExecutionRecordInput, WasmRunnerProfileInput, WasmTargetBinding,
    assemble_documentation_ir, docs_config_identity, language_documentation_facts_artifact,
    validate_language_documentation_facts, validate_wasm_execution_record,
    validate_wasm_runner_profile,
};
pub use evidence_chain::{
    EVIDENCE_CHAIN_CANONICAL_CODEC, EVIDENCE_CHAIN_ENVELOPE_SCHEMA_VERSION,
    EVIDENCE_CHAIN_JSON_PROJECTION, EvidenceChainEnvelope, EvidenceChainEnvelopeBody,
    EvidenceChainEnvelopeInput, EvidenceChainJsonProjection, EvidenceChainParentLink,
    EvidenceChainProfileRoot, EvidenceChainProjectionRecord, EvidenceKind, ParentLinkDomain,
    build_evidence_chain_envelope, evidence_chain_canonical_bytes, evidence_chain_collection_root,
    evidence_chain_json_projection, function_address_evidence_chain_input,
    validate_evidence_chain_envelope_input, validate_evidence_chain_json_projection,
};
pub use fixture::{
    AdapterReceiptInput, AlgebraicEffectInput, ArtifactClass, CanonicalValueInput,
    CapabilityShellInput, DocumentationCellDescriptor, DocumentationCellInput,
    DocumentationIrInput, DocumentationOutputInput, DocumentationPageInput,
    DocumentationReceiptInput, DocumentationSectionInput, EffectOperation, FixtureCase,
    FixtureClass, FixturePayload, FixtureResult, FixtureSuite, FixtureSuiteReport,
    FsmDenialRowInput, FsmEventInput, FsmGuardInput, FsmReceiptInput, FsmSideEffectInput,
    FsmSpecInput, FsmStateInput, FsmTraceStepInput, FsmTransitionRowInput, HandlerDescriptorInput,
    HandlerReceiptInput, IdentityUseInput, NickelAdapterReceiptInput, ReplayProfileInput,
    SourceEnvelopeInput, StackEvidenceInput, SteelAdapterReceiptInput, SteelHostFunctionInput,
    SteelOutputClass, TargetArtifactInput, UnsupportedClaimInput, ValidationVerdict,
    render_documentation_projection, validate_fixture, validate_fixture_suite,
};
pub use function_address::{
    FUNCTION_ADDRESS_CANONICAL_CODEC, FUNCTION_ADDRESS_JSON_PROJECTION,
    FUNCTION_ADDRESS_PRODUCER_PROFILE_SCHEMA_VERSION, FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION,
    FUNCTION_ADDRESS_VALENCE_PROJECTION_SCHEMA_VERSION, FunctionAddressCaveat,
    FunctionAddressDependencyEdge, FunctionAddressDependencyIdentityKind,
    FunctionAddressJsonProjection, FunctionAddressProducer, FunctionAddressProducerProfile,
    FunctionAddressReceiptBody, FunctionAddressReceiptEnvelope, FunctionAddressReceiptInput,
    FunctionAddressReceiptResult, FunctionAddressReceiptTrustClass, FunctionAddressRecord,
    FunctionAddressSourceEnvelope, FunctionAddressSpan, FunctionAddressValenceArtifact,
    FunctionAddressValenceProjection, LEGACY_FUNCTION_ADDRESS_CANONICAL_CODEC,
    LEGACY_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION, LEGACY_TOOL_VERSION_UNBOUND_CAVEAT,
    classify_function_address_receipt, export_function_address_valence_projection,
    function_address_canonical_bytes, function_address_json_projection,
    function_address_producer_profile, function_address_projection_json,
    validate_function_address_json_projection, validate_function_address_producer,
    validate_function_address_receipt,
};
pub use identity::{
    BLAKE3_HEX_LENGTH, Blake3Hex, Identity, IdentityDomain, IdentityError, compute_identity,
};
pub use marigold_analysis::{
    MARIGOLD_ANALYSIS_CANONICAL_CODEC, MARIGOLD_ANALYSIS_EVIDENCE_ROLE,
    MARIGOLD_ANALYSIS_JSON_PROJECTION, MARIGOLD_ANALYSIS_PROFILE, MARIGOLD_ANALYSIS_SCHEMA_VERSION,
    MARIGOLD_ANALYZER_TOOL, MARIGOLD_ANALYZER_VERSION, MARIGOLD_MAX_SOURCE_BYTES,
    MARIGOLD_REQUIRED_CHECKED_LAWS, MARIGOLD_REQUIRED_NON_CLAIMS, MarigoldAnalysisInput,
    MarigoldAnalysisReceiptBody, MarigoldAnalysisReceiptEnvelope, MarigoldAnalysisReceiptResult,
    MarigoldAnalyzerDescriptor, MarigoldProgramAnalysis, MarigoldStreamAnalysis,
    build_marigold_analysis_receipt, marigold_analysis_projection_identity,
    marigold_evaluator_identity, validate_marigold_analysis,
};
pub use proof_evidence::{
    TRELLIS_PROOF_PROFILE_CANONICAL_CODEC, TRELLIS_PROOF_PROFILE_JSON_PROJECTION,
    TRELLIS_PROOF_PROFILE_SCHEMA_VERSION, TrellisProofEvidenceEnvelope, TrellisProofEvidenceInput,
    TrellisProofEvidenceJsonProjection, TrellisProofEvidenceProfile, TrellisProofEvidenceRole,
    build_trellis_proof_evidence_envelope, trellis_proof_assumptions_root,
    trellis_proof_dependency_root, trellis_proof_evidence_json_projection,
    trellis_proof_profile_canonical_bytes, validate_trellis_proof_evidence_input,
    validate_trellis_proof_evidence_json_projection,
};
pub use receipt::{ReceiptBody, ReceiptEnvelope, ReceiptError, canonical_json_bytes, receipt_json};
pub use valence_export::{
    SourceSpan, SourceSpanSet, VALENCE_CONFORMANCE_CANONICAL_CODEC,
    VALENCE_CONFORMANCE_SCHEMA_VERSION, VALENCE_EXPORT_CANONICAL_CODEC,
    VALENCE_EXPORT_DEPENDENCY_PROFILE, VALENCE_EXPORT_PROFILE_VERSION,
    VALENCE_EXPORT_SCHEMA_VERSION, ValenceArtifactIdentity, ValenceExportConformance,
    ValenceReceiptExportBody, ValenceReceiptExportEnvelope, ValenceReceiptExportInput,
    ValenceReceiptExportResult, ValidatedValenceExportConformance, admit_valence_conformance,
    export_identity_key, export_valence_receipt, valence_conformance_identity,
};
