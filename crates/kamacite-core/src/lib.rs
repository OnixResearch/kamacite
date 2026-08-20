#![forbid(unsafe_code)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::unwrap_used)]

mod application_host_binding;
mod ast_grep_audit;
mod diagnostic;
mod dialect_roundtrip;
mod docs_generation;
mod evidence_chain;
mod fixture;
mod function_address;
mod identity;
mod manifest_freshness;
mod nickel_preserves;
mod nickel_sandbox;
mod preserves_codec;
mod proof_evidence;
mod receipt;
mod semantic_operation;
mod semantic_operation_valence;
mod spacewasm;
mod spacewasm_valence;
mod steel_execution;
mod valence_export;
mod wasm_component;
mod wasm_component_valence;

pub use application_host_binding::{
    APPLICATION_HOST_BINDING_RECEIPT_ROOT_LABEL, APPLICATION_HOST_BINDING_RECEIPT_SCHEMA,
    APPLICATION_HOST_BINDING_ROOT_LABEL, APPLICATION_HOST_BINDING_SCHEMA_VERSION,
    APPLICATION_HOST_BINDING_VALENCE_SCHEMA, ApplicationHostBindingInput,
    ApplicationHostBindingMetadata, ApplicationHostBindingReceipt,
    ApplicationHostBindingReceiptBody, ApplicationHostBindingValenceArtifact,
    ApplicationHostBindingValenceProjection, ApplicationHostBindingValenceProjectionBody,
    ApplicationHostBindingValidation, ValidatedApplicationHostBinding,
    application_host_binding_canonical_bytes, application_host_binding_valence_projection,
    validate_application_host_binding,
};
pub use ast_grep_audit::{
    AST_GREP_AUDIT_SCHEMA_VERSION, AST_GREP_BOUNDARY_ROLE, AST_GREP_RECORDED_ONLY_ROLE,
    AST_GREP_VALENCE_SCHEMA_VERSION, AstGrepAuditInput, AstGrepCodemodArtifact, AstGrepFinding,
    AstGrepLanguageProfile, AstGrepRuleCoverage, AstGrepSourceEnvelope, AstGrepValenceProjection,
    validate_ast_grep_audit,
};
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
    DOCS_CONFIG_SCHEMA_VERSION, DOCUMENTATION_IR_SCHEMA_VERSION, DocsActionEvidenceInput,
    DocsActionKind, DocsActionOutcome, DocsAssemblyInput, DocsCompletionState,
    DocsConfigProjection, DocsOutputConfig, DocsPageGroupingConfig, DocsSourceRootConfig,
    DocsWasmRunnerConfig, LANGUAGE_DOCUMENTATION_FACTS_SCHEMA_VERSION,
    LanguageDocumentationFactsArtifact, LanguageDocumentationFactsInput, LanguageDocumentationItem,
    RunnableCellReference, SourceByteSpan, WasmExecutionRecordInput, WasmRunnerProfileInput,
    WasmTargetBinding, assemble_documentation_ir, docs_config_identity, evaluate_docs_action,
    language_documentation_facts_artifact, validate_docs_config,
    validate_documentation_render_output, validate_language_documentation_facts,
    validate_wasm_execution_record, validate_wasm_runner_profile,
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
    FsmCoverageEvidence, FsmDenialRowInput, FsmEventInput, FsmGuardDecisionInput, FsmGuardInput,
    FsmMatrixCell, FsmReceiptInput, FsmSideEffectInput, FsmSpecInput, FsmStateInput,
    FsmTraceStepInput, FsmTransitionRowInput, HandlerDescriptorInput, HandlerReceiptInput,
    IdentityUseInput, NickelAdapterReceiptInput, ReplayProfileInput, SourceEnvelopeInput,
    StackEvidenceInput, SteelAdapterReceiptInput, SteelHostFunctionInput, SteelOutputClass,
    TargetArtifactInput, UnsupportedClaimInput, ValidatedFsmDescriptor, ValidationVerdict,
    fixture_expected_canonical_artifact, render_documentation_projection, validate_fixture,
    validate_fixture_suite, validate_fsm_descriptor,
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
    LEGACY_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION, LEGACY_PSEUDO_FUNCTION_ADDRESS_CANONICAL_CODEC,
    LEGACY_PSEUDO_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION, LEGACY_TOOL_VERSION_UNBOUND_CAVEAT,
    classify_function_address_receipt, export_function_address_valence_projection,
    function_address_canonical_bytes, function_address_json_projection,
    function_address_producer_profile, function_address_projection_json,
    validate_function_address_json_projection, validate_function_address_producer,
    validate_function_address_receipt,
};
pub use identity::{
    AdapterDomain, AdapterIdentity, AdapterIdentityRef, AlgebraicEffectDomain, AnyIdentity,
    ApplicationHostBindingDomain, ApplicationHostBindingIdentity,
    ApplicationHostBindingIdentityRef, BLAKE3_HEX_LENGTH, Blake3Hex, CanonicalValueDomain,
    CanonicalValueIdentity, CanonicalValueIdentityRef, CompatibilityDomain, CompatibilityIdentity,
    CompatibilityIdentityRef, CompilerDomain, CompilerIdentity, CompilerIdentityRef,
    DependencyClosureDomain, DependencyClosureIdentity, DependencyClosureIdentityRef,
    DialectPayloadDomain, DialectPayloadIdentity, DialectPayloadIdentityRef,
    DocumentationFactsDomain, DocumentationFactsIdentity, DocumentationFactsIdentityRef,
    DocumentationIrDomain, DocumentationIrIdentity, DocumentationIrIdentityRef,
    DocumentationOutputDomain, DocumentationOutputIdentity, DocumentationOutputIdentityRef,
    EffectFamilyIdentity, EffectFamilyIdentityRef, EffectLogDomain, EffectLogIdentity,
    EffectLogIdentityRef, EffectOperationDomain, EffectOperationIdentity,
    EffectOperationIdentityRef, EvaluatorDomain, EvaluatorIdentity, EvaluatorIdentityRef,
    EvidenceChainDomain, EvidenceChainIdentity, EvidenceChainIdentityRef, FsmCoverageDomain,
    FsmCoverageIdentity, FsmCoverageIdentityRef, FsmSpecDomain, FsmSpecIdentity,
    FsmSpecIdentityRef, FsmUniverseDomain, FsmUniverseIdentity, FsmUniverseIdentityRef,
    FunctionAddressDomain, FunctionAddressIdentity, FunctionAddressIdentityRef, Identity,
    IdentityDomain, IdentityDomainMarker, IdentityError, IdentityRef, ProjectionDomain,
    ProjectionIdentity, ProjectionIdentityRef, ReceiptDomain, ReceiptIdentity, ReceiptIdentityRef,
    SourceDomain, SourceIdentity, SourceIdentityRef, TargetArtifactDomain, TargetArtifactIdentity,
    TargetArtifactIdentityRef, WireIdentity, compute_identity,
};
pub use manifest_freshness::{
    MANIFEST_EVALUATOR_DESCRIPTOR, MANIFEST_EVALUATOR_PROFILE,
    MANIFEST_GENERATION_PRESERVES_ROOT_LABEL, MANIFEST_GENERATION_PRESERVES_VERSION,
    MANIFEST_GENERATION_RECEIPT_SCHEMA, ManifestGenerationInput, ManifestGenerationReceipt,
    ManifestGenerationReceiptBody, ManifestGenerationValidation, build_manifest_generation_receipt,
    validate_manifest_generation_receipt,
};
pub use nickel_preserves::{
    AdmittedNickelJsonExport, NICKEL_EXPORT_MANIFEST_SCHEMA_V3, NICKEL_EXPORT_RECEIPT_SCHEMA_V3,
    NICKEL_PRESERVES_MAPPING_PROFILE, NICKEL_PRESERVES_PROFILE_SCHEMA,
    NICKEL_PRESERVES_RECEIPT_SCHEMA, NICKEL_PRESERVES_ROOT_LABEL, NICKEL_PRESERVES_SCHEMA_VERSION,
    NickelExportIdentity, NickelExportIdentityKind, NickelJsonPreservesRoot,
    NickelJsonPreservesValue, NickelPreservesAdaptation, NickelPreservesError,
    NickelPreservesLimits, NickelPreservesProfile, NickelPreservesReceipt,
    NickelPreservesReceiptBody, adapt_admitted_nickel_json_export,
};
pub use nickel_sandbox::{
    NICKEL_SANDBOX_CPU_SECONDS, NICKEL_SANDBOX_DIAGNOSTIC_LIMIT_BYTES,
    NICKEL_SANDBOX_EVALUATOR_POLICY, NICKEL_SANDBOX_IMPORT_ROOT, NICKEL_SANDBOX_KIND,
    NICKEL_SANDBOX_MEMORY_LIMIT_BYTES, NICKEL_SANDBOX_OUTPUT_LIMIT_BYTES,
    NICKEL_SANDBOX_SCHEMA_VERSION, NICKEL_SANDBOX_SOURCE_PATH, NICKEL_SANDBOX_TIMEOUT_MILLISECONDS,
    NickelSandboxEvidence, NickelSandboxLimits, NickelSandboxOutcome, NickelSandboxProfile,
    nickel_sandbox_evidence_identity, nickel_sandbox_observed_facts, nickel_sandbox_profile,
    validate_nickel_sandbox, validate_nickel_sandbox_profile,
};
pub use preserves_codec::{
    CanonicalPreservesArtifact, PRESERVES_CANONICAL_CODEC, PRESERVES_LEGACY_PREFIXED_JSON_CODEC,
    PRESERVES_PACKAGE_VERSION, PRESERVES_SPEC_VERSION, PreservesCodecError, PreservesLimits,
    PreservesSchema, PreservesSyntax, canonicalize_preserves, decode_typed_preserves,
    encode_typed_preserves, verify_typed_preserves,
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
pub use receipt::{
    ReceiptBody, ReceiptEnvelope, ReceiptError, WireReceiptEnvelope, canonical_json_bytes,
    receipt_json,
};
pub use semantic_operation::{
    CompatibilityContext, ContinuationContract, IdentifiedSemanticOperation,
    IdentifiedSemanticOperationCompatibility, LEGACY_EFFECT_OPERATION_SCHEMA_VERSION,
    LegacyEffectOperationReference, MAX_COMPATIBILITY_CONTEXTS, MAX_MIGRATION_CHANGED_FIELDS,
    MAX_NON_CLAIMS, MAX_OPERATION_DEPENDENCIES, MAX_OPERATION_REFERENCES_PER_SURFACE,
    MAX_SUPPORTING_REFS, OperationImplementation, ReplayClass,
    SEMANTIC_OPERATION_COMPATIBILITY_PRESERVES_SCHEMA, SEMANTIC_OPERATION_COMPATIBILITY_ROOT_LABEL,
    SEMANTIC_OPERATION_COMPATIBILITY_SCHEMA_VERSION, SEMANTIC_OPERATION_PRESERVES_SCHEMA,
    SEMANTIC_OPERATION_ROOT_LABEL, SEMANTIC_OPERATION_SCHEMA_VERSION,
    SemanticOperationCompatibility, SemanticOperationDescriptor, SemanticOperationDescriptorInput,
    SemanticOperationError, SemanticOperationMetadata, SemanticOperationMigrationPreview,
    SemanticOperationReferenceBundle, admit_semantic_operation_compatibility,
    identify_semantic_operation, identify_semantic_operation_compatibility,
    preview_semantic_operation_migration, require_exact_semantic_operation,
    semantic_operation_descriptor, validate_semantic_operation_reference_bundle,
};
pub use semantic_operation_valence::{
    SEMANTIC_OPERATION_VALENCE_SCHEMA, SemanticOperationValenceArtifact,
    SemanticOperationValenceEdge, SemanticOperationValenceInput,
    SemanticOperationValenceProjection, SemanticOperationValenceProjectionBody,
    SemanticOperationValenceRole, project_semantic_operation_to_valence,
};
pub use spacewasm::{
    SPACEWASM_ABI_PROFILE_ID, SPACEWASM_ADAPTER_KIND, SPACEWASM_ARTIFACT_KIND,
    SPACEWASM_DESCRIPTOR_SCHEMA, SPACEWASM_MANTLE_BUNDLE_ID, SPACEWASM_MANTLE_REVISION,
    SPACEWASM_MODULE_BLAKE3, SPACEWASM_OCTET_REVISION, SPACEWASM_PROFILE_ID,
    SPACEWASM_PROFILE_SCHEMA, SPACEWASM_SOURCE_REVISION, SPACEWASM_STATIC_RECEIPT_SCHEMA,
    SpaceWasmAbiProfile, SpaceWasmArtifactPin, SpaceWasmBlocker, SpaceWasmDescriptor,
    SpaceWasmDiagnostic, SpaceWasmMantleEvidence, SpaceWasmObservedModule, SpaceWasmOctetEvidence,
    SpaceWasmParentEdge, SpaceWasmParentRole, SpaceWasmPlanningState, SpaceWasmProfile,
    SpaceWasmProfilePlanIdentities, SpaceWasmResources, SpaceWasmSchemaRef, SpaceWasmSourcePin,
    SpaceWasmStaticReceipt, SpaceWasmStaticReceiptBody, SpaceWasmStaticRequest,
    SpaceWasmStaticValidation, SpaceWasmValidationError, build_spacewasm_static_receipt,
    expected_spacewasm_parent_edges, spacewasm_profile_plan_identities, spacewasm_schema_identity,
    validate_spacewasm_canonical_output, validate_spacewasm_static,
    validate_spacewasm_static_receipt,
};
pub use spacewasm_valence::{
    SPACEWASM_VALENCE_PROJECTION_SCHEMA, SpaceWasmValenceArtifact, SpaceWasmValenceEdge,
    SpaceWasmValenceProjection, SpaceWasmValenceProjectionBody, SpaceWasmValenceProjectionError,
    SpaceWasmValenceVerificationClass, project_spacewasm_static_to_valence,
};
pub use steel_execution::{
    STEEL_DTO_BYTECODE_SCHEMA, STEEL_DTO_CALLABLE_DESCRIPTOR_SCHEMA, STEEL_DTO_INTEGER_SCHEMA,
    STEEL_DTO_MODULE_OUTPUT_SCHEMA, STEEL_EXECUTED_PROBE_KIND, STEEL_STORED_EVIDENCE_KIND,
    STEEL_SUCCESS_EXIT_CLASS, SteelDtoContract, SteelDtoContractBody, SteelDtoShape, SteelDtoValue,
    SteelDtoViolation, SteelExecutedProbeEvidence, SteelExecutionEvidence,
    render_steel_probe_output, resolve_steel_dto_contract, validate_steel_dto,
};
pub use valence_export::{
    SourceSpan, SourceSpanSet, VALENCE_CONFORMANCE_CANONICAL_CODEC,
    VALENCE_CONFORMANCE_SCHEMA_VERSION, VALENCE_EXPORT_CANONICAL_CODEC,
    VALENCE_EXPORT_DEPENDENCY_PROFILE, VALENCE_EXPORT_PROFILE_VERSION,
    VALENCE_EXPORT_SCHEMA_VERSION, ValenceArtifactIdentity, ValenceExportConformance,
    ValenceReceiptExportBody, ValenceReceiptExportEnvelope, ValenceReceiptExportInput,
    ValenceReceiptExportResult, ValidatedValenceExportConformance, admit_valence_conformance,
    export_identity_key, export_valence_receipt, valence_conformance_identity,
};
pub use wasm_component::{
    WASM_COMPONENT_ARTIFACT_KIND, WASM_COMPONENT_DESCRIPTOR_SCHEMA,
    WASM_COMPONENT_MATERIALIZATION_SCHEMA, WASM_COMPONENT_PROFILE_ID,
    WASM_COMPONENT_PROFILE_SCHEMA, WASM_COMPONENT_RUNTIME_PROFILE_ID,
    WASM_COMPONENT_SOURCE_REPOSITORY, WASM_COMPONENT_SOURCE_REVISION,
    WASM_COMPONENT_STATIC_RECEIPT_SCHEMA, WASM_COMPONENT_WIT_BLAKE3, WASM_COMPONENT_WIT_PACKAGE,
    WASM_COMPONENT_WIT_WORLD, WasmComponentAdapterProfile, WasmComponentBlocker,
    WasmComponentDescriptor, WasmComponentDeterminism, WasmComponentDiagnostic,
    WasmComponentEvidenceScope, WasmComponentExecutionPlan, WasmComponentFeatures,
    WasmComponentImportBinding, WasmComponentInspection, WasmComponentMantleHandoff,
    WasmComponentMantleHandoffRequest, WasmComponentMantlePackageInput,
    WasmComponentMantleStageReceipt, WasmComponentMantleStoreObject,
    WasmComponentMaterializationBundle, WasmComponentParentEdge, WasmComponentParentRole,
    WasmComponentPlanningState, WasmComponentProfile, WasmComponentProfileSource,
    WasmComponentReplayClass, WasmComponentResources, WasmComponentRuntimeProfile,
    WasmComponentRuntimeProfileExport, WasmComponentSchemaRef, WasmComponentStaticReceipt,
    WasmComponentStaticReceiptBody, WasmComponentStaticRequest, WasmComponentStaticValidation,
    WasmComponentToolchain, WasmComponentValidationError, WasmComponentWitCohort,
    build_wasm_component_static_receipt, validate_wasm_component_canonical_output,
    validate_wasm_component_mantle_handoff, validate_wasm_component_static,
    validate_wasm_component_static_receipt,
};
pub use wasm_component_valence::{
    WASM_COMPONENT_VALENCE_PROJECTION_SCHEMA, WasmComponentValenceArtifact,
    WasmComponentValenceEdge, WasmComponentValenceProjection, WasmComponentValenceProjectionBody,
    WasmComponentValenceProjectionError, WasmComponentValenceVerificationClass,
    project_wasm_component_static_to_valence,
};
