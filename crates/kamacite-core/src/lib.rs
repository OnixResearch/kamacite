#![forbid(unsafe_code)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::unwrap_used)]

mod diagnostic;
mod evidence_chain;
mod fixture;
mod function_address;
mod identity;
mod receipt;
mod valence_export;

pub use diagnostic::{Diagnostic, DiagnosticCode};
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
    FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION, FUNCTION_ADDRESS_VALENCE_PROJECTION_SCHEMA_VERSION,
    FunctionAddressCaveat, FunctionAddressDependencyEdge, FunctionAddressDependencyIdentityKind,
    FunctionAddressJsonProjection, FunctionAddressProducer, FunctionAddressReceiptBody,
    FunctionAddressReceiptEnvelope, FunctionAddressReceiptInput, FunctionAddressReceiptResult,
    FunctionAddressRecord, FunctionAddressSourceEnvelope, FunctionAddressSpan,
    FunctionAddressValenceArtifact, FunctionAddressValenceProjection,
    export_function_address_valence_projection, function_address_canonical_bytes,
    function_address_json_projection, function_address_projection_json,
    validate_function_address_json_projection, validate_function_address_receipt,
};
pub use identity::{
    BLAKE3_HEX_LENGTH, Blake3Hex, Identity, IdentityDomain, IdentityError, compute_identity,
};
pub use receipt::{ReceiptBody, ReceiptEnvelope, ReceiptError, canonical_json_bytes, receipt_json};
pub use valence_export::{
    SourceSpan, SourceSpanSet, VALENCE_EXPORT_CANONICAL_CODEC, VALENCE_EXPORT_DEPENDENCY_PROFILE,
    VALENCE_EXPORT_PROFILE_VERSION, VALENCE_EXPORT_SCHEMA_VERSION, ValenceArtifactIdentity,
    ValenceExportConformance, ValenceReceiptExportBody, ValenceReceiptExportEnvelope,
    ValenceReceiptExportInput, ValenceReceiptExportResult, export_identity_key,
    export_valence_receipt,
};
