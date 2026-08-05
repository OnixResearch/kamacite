use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    AdapterIdentity, CanonicalValueIdentity, DependencyClosureIdentity, EffectFamilyIdentity,
    EvaluatorIdentity, Identity, IdentityDomain, IdentityDomainMarker, PreservesLimits,
    PreservesSchema, PreservesSyntax, ProjectionIdentity, ReceiptIdentity, SourceIdentity,
    TargetArtifactIdentity, WireIdentity, canonicalize_preserves, compute_identity,
    encode_typed_preserves,
};

pub const SPACEWASM_PROFILE_SCHEMA: &str = "kamacite.spacewasm-mvp-profile.v1";
pub const SPACEWASM_PROFILE_ID: &str = "kamacite-spacewasm-mvp";
pub const SPACEWASM_ADAPTER_KIND: &str = "spacewasm-core-module";
pub const SPACEWASM_ARTIFACT_KIND: &str = "webassembly-1.0-core-module";
pub const SPACEWASM_ABI_PROFILE_ID: &str = "kamacite.spacewasm.bytes.v1";
pub const SPACEWASM_DESCRIPTOR_SCHEMA: &str = "kamacite.spacewasm-descriptor.v1";
pub const SPACEWASM_STATIC_RECEIPT_SCHEMA: &str = "kamacite.spacewasm-static-receipt.v1";
pub const SPACEWASM_MANTLE_REVISION: &str = "a141fcbaafe41f9a413a81275a33fe915bfca370";
pub const SPACEWASM_OCTET_REVISION: &str = "4367300e10740ecc99ba4b2171ace561b4787327";
pub const SPACEWASM_SOURCE_REVISION: &str = "e24cf09355a90497148eb5029fdb8e3400bd63e3";
pub const SPACEWASM_MANTLE_BUNDLE_ID: &str =
    "7775e57a4d07da89ca57ff282ab0d321f1adf70e9d650f3de4e3a329c92a71e6";
pub const SPACEWASM_MODULE_BLAKE3: &str =
    "b2568cffcad3c442a90dd65ccb82d6b7e285bf71c2c9908ce539b6b515217bde";

const SPACEWASM_MANTLE_PROFILE_ID: &str = "nasa-spacewasm-e24cf093-reference-v1";
const SPACEWASM_STATIC_RECEIPT_ROOT: &str = "SpaceWasmStaticReceiptBody";
const SPACEWASM_MANTLE_BUNDLE_SCHEMA: &str = "mantle-spacewasm-reference-bundle-v1";
const SPACEWASM_MANTLE_REPORT_SCHEMA: &str = "mantle-spacewasm-materialization-report-v1";
const SPACEWASM_OCTET_RECEIPT_SCHEMA: &str = "octet-evidence-rail-receipt/v1";
const SPACEWASM_OCTET_PROFILE_ID: &str = "spacewasm-mvp";
const SPACEWASM_OCTET_RAIL_ID: &str = "wasm-artifact";
const SPACEWASM_OCTET_STATUS: &str = "passed";
const SPACEWASM_RUNTIME_ID: &str = "nasa-spacewasm";
const SPACEWASM_EXPECTED_EXPORT: &str = "module::run::Func";
const SPACEWASM_EXPECTED_PROPOSAL: &str = "wasm1";
const SPACEWASM_PROMOTION_STATE: &str = "experimental-blocked";
const SPACEWASM_CANONICAL_IR: &str = "packed-preserves-bytes";
const SPACEWASM_MANTLE_MODULE_PATH: &str = "fixtures/wasm/mvp-positive.wasm";
const SPACEWASM_BLAKE3_PREFIX: &str = "b3:";

const SPACEWASM_EXECUTION_BLOCKERS: &[&str] = &[
    "mantle-reference-module-does-not-implement-kamacite-byte-abi",
    "octet-spacewasm-mvp-profile-permits-only-hostless-run-export",
    "kamacite-guest-materialization-bundle-unavailable",
    "consumer-spacewasm-shell-unavailable",
];
const SPACEWASM_RESULT_CLASSES: &[&str] = &[
    "success",
    "guest-error",
    "invalid-pointer-length",
    "malformed-output",
    "trap",
    "out-of-fuel",
    "resource-denial",
    "harness-failure",
];
const SPACEWASM_DENIED_AMBIENT_AUTHORITY: &[&str] = &[
    "wasi",
    "filesystem",
    "network",
    "process",
    "environment",
    "clock",
    "random",
    "credential",
    "device",
    "compiler",
    "evaluator",
    "package-manager",
    "mutation",
];
const SPACEWASM_DENIED_PROPOSALS: &[&str] = &[
    "bulk-memory",
    "component-model",
    "custom-page-sizes",
    "memory64",
    "multi-memory",
    "multi-value",
    "non-trapping-float-to-int",
    "sign-extension-ops",
    "threads",
    "wit",
    "wasi",
];
const SPACEWASM_REQUIRED_PARENT_ROLES: &[&str] = &[
    "source-envelope",
    "canonical-input",
    "input-schema",
    "output-schema",
    "adapter-profile",
    "abi-profile",
    "module",
    "spacewasm-runtime",
    "mantle-bundle",
    "octet-static-report",
    "capability-plan",
    "effect-plan",
    "resource-plan",
];
const SPACEWASM_CHECKED_LAWS: &[&str] = &[
    "profile-selection-is-explicit",
    "component-and-wit-fallback-rejected",
    "exact-external-bytes-bound",
    "module-and-canonical-value-identities-remain-distinct",
    "external-static-pass-cannot-create-execution-plan",
    "packed-preserves-remains-authoritative",
];
const SPACEWASM_NON_CLAIMS: &[&str] = &[
    "not WIT or Component Model compatibility",
    "not source-language semantic equivalence",
    "not SpaceWasm correctness or memory safety",
    "not WebAssembly conformance",
    "not sandbox completeness or host authority",
    "not portable interpreter state",
    "not flight qualification or certification",
    "not production readiness",
    "not release eligibility",
];
const SPACEWASM_MAXIMUM_ARTIFACT_BYTES: u64 = 1_048_576;
const SPACEWASM_MAXIMUM_HOSTCALL_BYTES: u64 = 65_536;
const SPACEWASM_MAXIMUM_EXPORTS: u64 = 16;
const SPACEWASM_MAXIMUM_FUNCTIONS: u64 = 4_096;
const SPACEWASM_MAXIMUM_MEMORIES: u64 = 1;
const SPACEWASM_MAXIMUM_TABLES: u64 = 1;
const SPACEWASM_MAXIMUM_GUEST_MEMORY_PAGES: u64 = 256;
const SPACEWASM_MAXIMUM_GUEST_TABLE_ELEMENTS: u64 = 1_024;
const SPACEWASM_INTERPRETER_CODE_PAGES: u64 = 64;
const SPACEWASM_INTERPRETER_CONTROL_FRAMES: u64 = 64;
const SPACEWASM_INTERPRETER_STACK_WORDS: u64 = 1_024;
const SPACEWASM_INTERPRETER_DEFAULT_FUEL: u64 = 1_024;
const SPACEWASM_INTERPRETER_STREAM_CHUNKS: u64 = 4_096;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmSourcePin {
    pub repository: String,
    pub revision: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmArtifactPin {
    pub path: String,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmMantleEvidence {
    pub source: SpaceWasmSourcePin,
    pub bundle_schema: String,
    pub bundle_identity_blake3: String,
    pub bundle_manifest: SpaceWasmArtifactPin,
    pub materialization_report: SpaceWasmArtifactPin,
    pub profile: SpaceWasmArtifactPin,
    pub module: SpaceWasmArtifactPin,
    pub verifier_summary: SpaceWasmArtifactPin,
    pub materialization_profile_id: String,
    pub materialization_profile_identity: String,
    pub materialization_cohort_identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmOctetEvidence {
    pub source: SpaceWasmSourcePin,
    pub profile_id: String,
    pub profile_identity: String,
    pub registry_identity: String,
    pub cohort_identity: String,
    pub runtime_cohort_identity: String,
    pub registry: SpaceWasmArtifactPin,
    pub support_projection: SpaceWasmArtifactPin,
    pub report: SpaceWasmArtifactPin,
    pub module: SpaceWasmArtifactPin,
    pub verification_role: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmAbiProfile {
    pub profile_id: String,
    pub memory_export: String,
    pub allocation_export: String,
    pub deallocation_export: String,
    pub entry_export: String,
    pub pointer_type: String,
    pub length_type: String,
    pub result_type: String,
    pub result_encoding: String,
    pub input_encoding: String,
    pub output_encoding: String,
    pub max_input_bytes: u64,
    pub max_output_bytes: u64,
    pub max_hostcall_bytes: u64,
    pub result_classes: Vec<String>,
    pub allowed_host_functions: Vec<String>,
    pub denied_ambient_authority: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmResources {
    pub max_artifact_bytes: u64,
    pub max_exports: u64,
    pub max_functions: u64,
    pub max_memories: u64,
    pub max_tables: u64,
    pub max_guest_memory_pages: u64,
    pub max_guest_table_elements: u64,
    pub allow_memory_grow: bool,
    pub allow_start: bool,
    pub interpreter_code_pages: u64,
    pub interpreter_control_frames: u64,
    pub interpreter_stack_words: u64,
    pub interpreter_default_fuel: u64,
    pub interpreter_stream_chunks: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmObservedModule {
    pub artifact_kind: String,
    pub proposals: Vec<String>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub has_memory: bool,
    pub abi_compatible: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmProfile {
    pub schema_version: String,
    pub profile_id: String,
    pub adapter_kind: String,
    pub artifact_kind: String,
    pub promotion_state: String,
    pub canonical_ir: String,
    pub mantle: SpaceWasmMantleEvidence,
    pub octet: SpaceWasmOctetEvidence,
    pub abi: SpaceWasmAbiProfile,
    pub admitted_proposals: Vec<String>,
    pub denied_proposals: Vec<String>,
    pub resources: SpaceWasmResources,
    pub observed_external_module: SpaceWasmObservedModule,
    pub required_parent_roles: Vec<String>,
    pub execution_blockers: Vec<String>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmSchemaRef {
    pub schema_version: String,
    pub root_label: String,
    pub identity: WireIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpaceWasmParentRole {
    SourceEnvelope,
    CanonicalInput,
    InputSchema,
    OutputSchema,
    AdapterProfile,
    AbiProfile,
    CapabilityPlan,
    EffectPlan,
    ResourcePlan,
    Module,
    SpacewasmRuntime,
    MantleBundle,
    OctetStaticReport,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmParentEdge {
    pub role: SpaceWasmParentRole,
    pub parent_identity: WireIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmDescriptor {
    pub schema_version: String,
    pub adapter_kind: String,
    pub artifact_kind: String,
    pub profile_identity: WireIdentity,
    pub abi_profile_identity: WireIdentity,
    pub capability_plan_identity: WireIdentity,
    pub effect_plan_identity: WireIdentity,
    pub resource_plan_identity: WireIdentity,
    pub source_identity: WireIdentity,
    pub canonical_input_identity: WireIdentity,
    pub input_schema: SpaceWasmSchemaRef,
    pub output_schema: SpaceWasmSchemaRef,
    pub module_identity: WireIdentity,
    pub runtime_identity: WireIdentity,
    pub mantle_bundle_identity: WireIdentity,
    pub octet_report_identity: WireIdentity,
    pub allowed_host_functions: Vec<String>,
    pub component_fallback: bool,
    pub wit_world: Option<String>,
    pub parent_edges: Vec<SpaceWasmParentEdge>,
    pub claims: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub struct SpaceWasmStaticRequest<'a> {
    pub profile_bytes: &'a [u8],
    pub source_envelope_bytes: &'a [u8],
    pub mantle_bundle_manifest_bytes: &'a [u8],
    pub mantle_materialization_report_bytes: &'a [u8],
    pub mantle_profile_bytes: &'a [u8],
    pub mantle_bundle_verification_bytes: &'a [u8],
    pub octet_registry_bytes: &'a [u8],
    pub octet_support_projection_bytes: &'a [u8],
    pub octet_report_bytes: &'a [u8],
    pub module_bytes: &'a [u8],
    pub canonical_input_bytes: &'a [u8],
    pub descriptor: &'a SpaceWasmDescriptor,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpaceWasmBlocker {
    MantleReferenceModuleDoesNotImplementKamaciteByteAbi,
    OctetProfilePermitsOnlyHostlessRunExport,
    KamaciteGuestMaterializationBundleUnavailable,
    ConsumerSpacewasmShellUnavailable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpaceWasmPlanningState {
    StaticOnly,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SpaceWasmStaticValidation {
    pub state: SpaceWasmPlanningState,
    pub profile_identity: AdapterIdentity,
    pub descriptor_identity: AdapterIdentity,
    pub abi_profile_identity: AdapterIdentity,
    pub capability_plan_identity: AdapterIdentity,
    pub effect_plan_identity: EffectFamilyIdentity,
    pub resource_plan_identity: AdapterIdentity,
    pub source_identity: SourceIdentity,
    pub canonical_input_identity: CanonicalValueIdentity,
    pub input_schema_identity: ProjectionIdentity,
    pub output_schema_identity: ProjectionIdentity,
    pub module_identity: TargetArtifactIdentity,
    pub runtime_identity: EvaluatorIdentity,
    pub mantle_bundle_identity: DependencyClosureIdentity,
    pub octet_report_identity: ProjectionIdentity,
    pub blockers: Vec<SpaceWasmBlocker>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmStaticReceiptBody {
    pub schema_version: String,
    pub state: SpaceWasmPlanningState,
    pub profile_identity: AdapterIdentity,
    pub descriptor_identity: AdapterIdentity,
    pub abi_profile_identity: AdapterIdentity,
    pub capability_plan_identity: AdapterIdentity,
    pub effect_plan_identity: EffectFamilyIdentity,
    pub resource_plan_identity: AdapterIdentity,
    pub source_identity: SourceIdentity,
    pub canonical_input_identity: CanonicalValueIdentity,
    pub input_schema_identity: ProjectionIdentity,
    pub output_schema_identity: ProjectionIdentity,
    pub module_identity: TargetArtifactIdentity,
    pub runtime_identity: EvaluatorIdentity,
    pub mantle_bundle_identity: DependencyClosureIdentity,
    pub octet_report_identity: ProjectionIdentity,
    pub blockers: Vec<SpaceWasmBlocker>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWasmStaticReceipt {
    pub body: SpaceWasmStaticReceiptBody,
    pub receipt_identity: ReceiptIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaceWasmProfilePlanIdentities {
    pub abi_profile_identity: AdapterIdentity,
    pub capability_plan_identity: AdapterIdentity,
    pub effect_plan_identity: EffectFamilyIdentity,
    pub resource_plan_identity: AdapterIdentity,
}

#[derive(Serialize)]
struct SpaceWasmCapabilityPlanMaterial<'a> {
    allowed_host_functions: &'a [String],
    denied_ambient_authority: &'a [String],
}

#[derive(Serialize)]
struct SpaceWasmEffectPlanMaterial<'a> {
    allowed_operations: &'a [String],
}

struct SpaceWasmAdmittedIdentityRefs<'a> {
    profile: &'a AdapterIdentity,
    plans: &'a SpaceWasmProfilePlanIdentities,
    source: &'a SourceIdentity,
    canonical_input: &'a CanonicalValueIdentity,
    module: &'a TargetArtifactIdentity,
    runtime: &'a EvaluatorIdentity,
    mantle_bundle: &'a DependencyClosureIdentity,
    octet_report: &'a ProjectionIdentity,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SpaceWasmDiagnostic {
    pub code: &'static str,
    pub field: &'static str,
    pub message: String,
}

impl SpaceWasmDiagnostic {
    fn new(code: &'static str, field: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            field,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpaceWasmValidationError {
    pub diagnostics: Vec<SpaceWasmDiagnostic>,
}

impl Display for SpaceWasmValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "SpaceWasm static validation failed with {} diagnostic(s)",
            self.diagnostics.len()
        )
    }
}

impl std::error::Error for SpaceWasmValidationError {}

pub fn spacewasm_profile_plan_identities(
    profile: &SpaceWasmProfile,
) -> Result<SpaceWasmProfilePlanIdentities, SpaceWasmValidationError> {
    let abi_profile_identity = encoded_identity("abi", &profile.abi)?;
    let capability_plan_identity = encoded_identity(
        "capability_plan",
        &SpaceWasmCapabilityPlanMaterial {
            allowed_host_functions: &profile.abi.allowed_host_functions,
            denied_ambient_authority: &profile.abi.denied_ambient_authority,
        },
    )?;
    let effect_plan_identity = encoded_identity(
        "effect_plan",
        &SpaceWasmEffectPlanMaterial {
            allowed_operations: &profile.abi.allowed_host_functions,
        },
    )?;
    let resource_plan_identity = encoded_identity("resource_plan", &profile.resources)?;
    Ok(SpaceWasmProfilePlanIdentities {
        abi_profile_identity,
        capability_plan_identity,
        effect_plan_identity,
        resource_plan_identity,
    })
}

// r[impl kamacite.spacewasm_adapter.boundary]
// r[impl kamacite.spacewasm_adapter.profile]
// r[impl kamacite.spacewasm_adapter.abi]
// r[impl kamacite.spacewasm_adapter.identity]
// r[impl kamacite.spacewasm_adapter.functional_core]
// r[impl kamacite.spacewasm_adapter.preserves_io]
pub fn validate_spacewasm_static(
    request: &SpaceWasmStaticRequest<'_>,
) -> Result<SpaceWasmStaticValidation, SpaceWasmValidationError> {
    let mut diagnostics = Vec::new();
    let profile = decode_profile(request.profile_bytes, &mut diagnostics);
    let Some(profile) = profile else {
        return Err(validation_error(diagnostics));
    };
    validate_profile(&profile, request.profile_bytes, &mut diagnostics);
    validate_external_bytes(&profile, request, &mut diagnostics);
    validate_mantle_evidence(&profile, request, &mut diagnostics);
    validate_octet_evidence(&profile, request, &mut diagnostics);

    let profile_identity = AdapterIdentity::compute(request.profile_bytes);
    let plan_identities = spacewasm_profile_plan_identities(&profile)?;
    let source_identity = SourceIdentity::compute(request.source_envelope_bytes);
    let module_identity = TargetArtifactIdentity::compute(request.module_bytes);
    let runtime_identity = EvaluatorIdentity::compute(request.mantle_profile_bytes);
    let mantle_bundle_identity = identity_from_hex(
        &profile.mantle.bundle_identity_blake3,
        "mantle.bundle_identity_blake3",
        &mut diagnostics,
    );
    let octet_report_identity = ProjectionIdentity::compute(request.octet_report_bytes);
    let canonical_input_identity = validate_canonical_input(
        request.canonical_input_bytes,
        &request.descriptor.input_schema,
        profile.abi.max_input_bytes,
        &mut diagnostics,
    );

    if let (Some(mantle_bundle_identity), Some(canonical_input_identity)) =
        (&mantle_bundle_identity, &canonical_input_identity)
    {
        let identities = SpaceWasmAdmittedIdentityRefs {
            profile: &profile_identity,
            plans: &plan_identities,
            source: &source_identity,
            canonical_input: canonical_input_identity,
            module: &module_identity,
            runtime: &runtime_identity,
            mantle_bundle: mantle_bundle_identity,
            octet_report: &octet_report_identity,
        };
        validate_descriptor(request.descriptor, &profile, &identities, &mut diagnostics);
    }

    if !diagnostics.is_empty() {
        return Err(validation_error(diagnostics));
    }
    let Some(canonical_input_identity) = canonical_input_identity else {
        return Err(single_error(SpaceWasmDiagnostic::new(
            "internal-invariant",
            "canonical_input_identity",
            "successful validation did not retain canonical input identity",
        )));
    };
    let Some(mantle_bundle_identity) = mantle_bundle_identity else {
        return Err(single_error(SpaceWasmDiagnostic::new(
            "internal-invariant",
            "mantle_bundle_identity",
            "successful validation did not retain Mantle bundle identity",
        )));
    };
    let descriptor_bytes = serde_json::to_vec(request.descriptor).map_err(|error| {
        single_error(SpaceWasmDiagnostic::new(
            "encoding",
            "descriptor",
            error.to_string(),
        ))
    })?;
    Ok(SpaceWasmStaticValidation {
        state: SpaceWasmPlanningState::StaticOnly,
        profile_identity,
        descriptor_identity: AdapterIdentity::compute(&descriptor_bytes),
        abi_profile_identity: plan_identities.abi_profile_identity,
        capability_plan_identity: plan_identities.capability_plan_identity,
        effect_plan_identity: plan_identities.effect_plan_identity,
        resource_plan_identity: plan_identities.resource_plan_identity,
        source_identity,
        canonical_input_identity,
        input_schema_identity: ProjectionIdentity::from_wire(
            request.descriptor.input_schema.identity.clone(),
        )
        .map_err(|_| {
            single_error(SpaceWasmDiagnostic::new(
                "internal-invariant",
                "descriptor.input_schema",
                "validated schema identity failed admission",
            ))
        })?,
        output_schema_identity: ProjectionIdentity::from_wire(
            request.descriptor.output_schema.identity.clone(),
        )
        .map_err(|_| {
            single_error(SpaceWasmDiagnostic::new(
                "internal-invariant",
                "descriptor.output_schema",
                "validated schema identity failed admission",
            ))
        })?,
        module_identity,
        runtime_identity,
        mantle_bundle_identity,
        octet_report_identity,
        blockers: vec![
            SpaceWasmBlocker::MantleReferenceModuleDoesNotImplementKamaciteByteAbi,
            SpaceWasmBlocker::OctetProfilePermitsOnlyHostlessRunExport,
            SpaceWasmBlocker::KamaciteGuestMaterializationBundleUnavailable,
            SpaceWasmBlocker::ConsumerSpacewasmShellUnavailable,
        ],
        checked_laws: profile.checked_laws,
        non_claims: profile.non_claims,
    })
}

// r[impl kamacite.spacewasm_adapter.receipt]
pub fn build_spacewasm_static_receipt(
    request: &SpaceWasmStaticRequest<'_>,
) -> Result<SpaceWasmStaticReceipt, SpaceWasmValidationError> {
    let validation = validate_spacewasm_static(request)?;
    let body = SpaceWasmStaticReceiptBody {
        schema_version: SPACEWASM_STATIC_RECEIPT_SCHEMA.to_string(),
        state: validation.state,
        profile_identity: validation.profile_identity,
        descriptor_identity: validation.descriptor_identity,
        abi_profile_identity: validation.abi_profile_identity,
        capability_plan_identity: validation.capability_plan_identity,
        effect_plan_identity: validation.effect_plan_identity,
        resource_plan_identity: validation.resource_plan_identity,
        source_identity: validation.source_identity,
        canonical_input_identity: validation.canonical_input_identity,
        input_schema_identity: validation.input_schema_identity,
        output_schema_identity: validation.output_schema_identity,
        module_identity: validation.module_identity,
        runtime_identity: validation.runtime_identity,
        mantle_bundle_identity: validation.mantle_bundle_identity,
        octet_report_identity: validation.octet_report_identity,
        blockers: validation.blockers,
        checked_laws: validation.checked_laws,
        non_claims: validation.non_claims,
    };
    let artifact = encode_typed_preserves(
        &body,
        PreservesSchema {
            schema_version: SPACEWASM_STATIC_RECEIPT_SCHEMA,
            root_label: SPACEWASM_STATIC_RECEIPT_ROOT,
        },
        PreservesLimits::default(),
    )
    .map_err(|error| {
        single_error(SpaceWasmDiagnostic::new(
            "receipt-encoding",
            "receipt",
            error.to_string(),
        ))
    })?;
    Ok(SpaceWasmStaticReceipt {
        body,
        receipt_identity: ReceiptIdentity::compute(&artifact.packed_bytes),
    })
}

pub fn validate_spacewasm_static_receipt(
    request: &SpaceWasmStaticRequest<'_>,
    receipt: &SpaceWasmStaticReceipt,
) -> Result<(), SpaceWasmValidationError> {
    let expected = build_spacewasm_static_receipt(request)?;
    if receipt == &expected {
        Ok(())
    } else {
        Err(single_error(SpaceWasmDiagnostic::new(
            "stale-receipt",
            "receipt",
            "receipt differs from deterministic validation over exact external evidence",
        )))
    }
}

pub fn validate_spacewasm_canonical_output(
    bytes: &[u8],
    schema: &SpaceWasmSchemaRef,
) -> Result<CanonicalValueIdentity, SpaceWasmValidationError> {
    let mut diagnostics = Vec::new();
    validate_schema_ref(&mut diagnostics, "output_schema", schema);
    let identity = validate_canonical_value(
        bytes,
        schema,
        SPACEWASM_MAXIMUM_ARTIFACT_BYTES,
        "canonical_output",
        &mut diagnostics,
    );
    if !diagnostics.is_empty() {
        return Err(validation_error(diagnostics));
    }
    identity.ok_or_else(|| {
        single_error(SpaceWasmDiagnostic::new(
            "internal-invariant",
            "canonical_output",
            "successful output validation did not retain canonical identity",
        ))
    })
}

fn decode_profile(
    bytes: &[u8],
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) -> Option<SpaceWasmProfile> {
    match serde_json::from_slice(bytes) {
        Ok(profile) => Some(profile),
        Err(error) => {
            diagnostics.push(SpaceWasmDiagnostic::new(
                "decode",
                "profile",
                error.to_string(),
            ));
            None
        }
    }
}

fn validate_profile(
    profile: &SpaceWasmProfile,
    bytes: &[u8],
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) {
    const REVIEWED_PROFILE: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/generated/profile.json");
    check_equal_bytes(diagnostics, "profile", bytes, REVIEWED_PROFILE);
    check_equal(
        diagnostics,
        "schema_version",
        &profile.schema_version,
        SPACEWASM_PROFILE_SCHEMA,
    );
    check_equal(
        diagnostics,
        "profile_id",
        &profile.profile_id,
        SPACEWASM_PROFILE_ID,
    );
    check_equal(
        diagnostics,
        "adapter_kind",
        &profile.adapter_kind,
        SPACEWASM_ADAPTER_KIND,
    );
    check_equal(
        diagnostics,
        "artifact_kind",
        &profile.artifact_kind,
        SPACEWASM_ARTIFACT_KIND,
    );
    check_equal(
        diagnostics,
        "promotion_state",
        &profile.promotion_state,
        SPACEWASM_PROMOTION_STATE,
    );
    check_equal(
        diagnostics,
        "canonical_ir",
        &profile.canonical_ir,
        SPACEWASM_CANONICAL_IR,
    );
    check_equal(
        diagnostics,
        "abi.profile_id",
        &profile.abi.profile_id,
        SPACEWASM_ABI_PROFILE_ID,
    );
    check_equal(
        diagnostics,
        "abi.memory_export",
        &profile.abi.memory_export,
        "memory",
    );
    check_equal(
        diagnostics,
        "abi.allocation_export",
        &profile.abi.allocation_export,
        "kamacite_alloc",
    );
    check_equal(
        diagnostics,
        "abi.deallocation_export",
        &profile.abi.deallocation_export,
        "kamacite_dealloc",
    );
    check_equal(
        diagnostics,
        "abi.entry_export",
        &profile.abi.entry_export,
        "kamacite_invoke",
    );
    check_equal_vec(
        diagnostics,
        "abi.result_classes",
        &profile.abi.result_classes,
        &string_values(SPACEWASM_RESULT_CLASSES),
    );
    check_equal_vec(
        diagnostics,
        "abi.denied_ambient_authority",
        &profile.abi.denied_ambient_authority,
        &string_values(SPACEWASM_DENIED_AMBIENT_AUTHORITY),
    );
    if profile.abi.max_input_bytes != SPACEWASM_MAXIMUM_ARTIFACT_BYTES
        || profile.abi.max_output_bytes != SPACEWASM_MAXIMUM_ARTIFACT_BYTES
        || profile.abi.max_hostcall_bytes != SPACEWASM_MAXIMUM_HOSTCALL_BYTES
    {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "unsupported-profile",
            "abi.byte_limits",
            "ABI byte limits differ from the reviewed bounded profile",
        ));
    }
    check_equal_vec(
        diagnostics,
        "admitted_proposals",
        &profile.admitted_proposals,
        &[SPACEWASM_EXPECTED_PROPOSAL.to_string()],
    );
    check_equal_vec(
        diagnostics,
        "denied_proposals",
        &profile.denied_proposals,
        &string_values(SPACEWASM_DENIED_PROPOSALS),
    );
    check_equal_vec(
        diagnostics,
        "required_parent_roles",
        &profile.required_parent_roles,
        &string_values(SPACEWASM_REQUIRED_PARENT_ROLES),
    );
    check_equal_vec(
        diagnostics,
        "checked_laws",
        &profile.checked_laws,
        &string_values(SPACEWASM_CHECKED_LAWS),
    );
    check_equal_vec(
        diagnostics,
        "non_claims",
        &profile.non_claims,
        &string_values(SPACEWASM_NON_CLAIMS),
    );
    validate_resource_profile(&profile.resources, diagnostics);
    check_equal(
        diagnostics,
        "mantle.source.revision",
        &profile.mantle.source.revision,
        SPACEWASM_MANTLE_REVISION,
    );
    check_equal(
        diagnostics,
        "octet.source.revision",
        &profile.octet.source.revision,
        SPACEWASM_OCTET_REVISION,
    );
    check_equal(
        diagnostics,
        "mantle.bundle_schema",
        &profile.mantle.bundle_schema,
        SPACEWASM_MANTLE_BUNDLE_SCHEMA,
    );
    check_equal(
        diagnostics,
        "mantle.materialization_profile_id",
        &profile.mantle.materialization_profile_id,
        SPACEWASM_MANTLE_PROFILE_ID,
    );
    check_equal(
        diagnostics,
        "octet.profile_id",
        &profile.octet.profile_id,
        SPACEWASM_OCTET_PROFILE_ID,
    );
    check_equal(
        diagnostics,
        "octet.verification_role",
        &profile.octet.verification_role,
        "boundary",
    );
    check_equal_vec(
        diagnostics,
        "execution_blockers",
        &profile.execution_blockers,
        &string_values(SPACEWASM_EXECUTION_BLOCKERS),
    );
    check_equal_vec(
        diagnostics,
        "observed_external_module.exports",
        &profile.observed_external_module.exports,
        &[SPACEWASM_EXPECTED_EXPORT.to_string()],
    );
    check_equal_vec(
        diagnostics,
        "observed_external_module.proposals",
        &profile.observed_external_module.proposals,
        &[SPACEWASM_EXPECTED_PROPOSAL.to_string()],
    );
    if profile.observed_external_module.abi_compatible
        || profile.observed_external_module.has_memory
        || !profile.observed_external_module.imports.is_empty()
    {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "unsupported-external-module",
            "observed_external_module",
            "reviewed external module must remain hostless and ABI-incompatible",
        ));
    }
    if profile.resources.allow_memory_grow || profile.resources.allow_start {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "unsupported-profile",
            "resources",
            "memory growth and start functions must remain denied",
        ));
    }
    if !profile.abi.allowed_host_functions.is_empty() {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "unsupported-profile",
            "abi.allowed_host_functions",
            "initial exact Octet profile admits no imports",
        ));
    }
}

fn validate_resource_profile(
    resources: &SpaceWasmResources,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) {
    let expected = SpaceWasmResources {
        max_artifact_bytes: SPACEWASM_MAXIMUM_ARTIFACT_BYTES,
        max_exports: SPACEWASM_MAXIMUM_EXPORTS,
        max_functions: SPACEWASM_MAXIMUM_FUNCTIONS,
        max_memories: SPACEWASM_MAXIMUM_MEMORIES,
        max_tables: SPACEWASM_MAXIMUM_TABLES,
        max_guest_memory_pages: SPACEWASM_MAXIMUM_GUEST_MEMORY_PAGES,
        max_guest_table_elements: SPACEWASM_MAXIMUM_GUEST_TABLE_ELEMENTS,
        allow_memory_grow: false,
        allow_start: false,
        interpreter_code_pages: SPACEWASM_INTERPRETER_CODE_PAGES,
        interpreter_control_frames: SPACEWASM_INTERPRETER_CONTROL_FRAMES,
        interpreter_stack_words: SPACEWASM_INTERPRETER_STACK_WORDS,
        interpreter_default_fuel: SPACEWASM_INTERPRETER_DEFAULT_FUEL,
        interpreter_stream_chunks: SPACEWASM_INTERPRETER_STREAM_CHUNKS,
    };
    if resources != &expected {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "unsupported-profile",
            "resources",
            "resource bounds differ from the reviewed Mantle/Octet intersection",
        ));
    }
}

fn validate_external_bytes(
    profile: &SpaceWasmProfile,
    request: &SpaceWasmStaticRequest<'_>,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) {
    validate_raw_hash(
        diagnostics,
        "mantle.bundle_manifest",
        request.mantle_bundle_manifest_bytes,
        &profile.mantle.bundle_manifest.blake3,
    );
    validate_raw_hash(
        diagnostics,
        "mantle.materialization_report",
        request.mantle_materialization_report_bytes,
        &profile.mantle.materialization_report.blake3,
    );
    validate_raw_hash(
        diagnostics,
        "mantle.profile",
        request.mantle_profile_bytes,
        &profile.mantle.profile.blake3,
    );
    validate_raw_hash(
        diagnostics,
        "mantle.bundle_verification",
        request.mantle_bundle_verification_bytes,
        &profile.mantle.verifier_summary.blake3,
    );
    validate_raw_hash(
        diagnostics,
        "octet.registry",
        request.octet_registry_bytes,
        &profile.octet.registry.blake3,
    );
    validate_raw_hash(
        diagnostics,
        "octet.support_projection",
        request.octet_support_projection_bytes,
        &profile.octet.support_projection.blake3,
    );
    validate_raw_hash(
        diagnostics,
        "octet.report",
        request.octet_report_bytes,
        &profile.octet.report.blake3,
    );
    validate_raw_hash(
        diagnostics,
        "module",
        request.module_bytes,
        &profile.mantle.module.blake3,
    );
    validate_raw_hash(
        diagnostics,
        "octet.module",
        request.module_bytes,
        &profile.octet.module.blake3,
    );
}

fn validate_mantle_evidence(
    profile: &SpaceWasmProfile,
    request: &SpaceWasmStaticRequest<'_>,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) {
    let manifest = decode_json(
        request.mantle_bundle_manifest_bytes,
        "mantle.bundle_manifest",
        diagnostics,
    );
    let materialization = decode_json(
        request.mantle_materialization_report_bytes,
        "mantle.materialization_report",
        diagnostics,
    );
    let verification = decode_json(
        request.mantle_bundle_verification_bytes,
        "mantle.bundle_verification",
        diagnostics,
    );
    let (Some(manifest), Some(materialization), Some(verification)) =
        (manifest, materialization, verification)
    else {
        return;
    };
    check_json_string(
        diagnostics,
        &manifest,
        &["schema"],
        SPACEWASM_MANTLE_BUNDLE_SCHEMA,
        "mantle.bundle_manifest.schema",
    );
    check_json_string(
        diagnostics,
        &manifest,
        &["profile_id"],
        SPACEWASM_MANTLE_PROFILE_ID,
        "mantle.bundle_manifest.profile_id",
    );
    check_json_string(
        diagnostics,
        &manifest,
        &["profile_identity_blake3"],
        &profile.mantle.materialization_profile_identity,
        "mantle.bundle_manifest.profile_identity",
    );
    check_json_string(
        diagnostics,
        &manifest,
        &["cohort_identity_blake3"],
        &profile.mantle.materialization_cohort_identity,
        "mantle.bundle_manifest.cohort_identity",
    );
    validate_manifest_member(
        &manifest,
        SPACEWASM_MANTLE_MODULE_PATH,
        "fixture-artifact",
        SPACEWASM_MODULE_BLAKE3,
        diagnostics,
    );
    check_json_string(
        diagnostics,
        &materialization,
        &["schema"],
        SPACEWASM_MANTLE_REPORT_SCHEMA,
        "mantle.materialization.schema",
    );
    check_json_string(
        diagnostics,
        &materialization,
        &["disposition"],
        "complete",
        "mantle.materialization.disposition",
    );
    check_json_bool(
        diagnostics,
        &materialization,
        &["source_admitted"],
        true,
        "mantle.materialization.source_admitted",
    );
    check_json_bool(
        diagnostics,
        &materialization,
        &["support_matches_profile"],
        true,
        "mantle.materialization.support_matches_profile",
    );
    check_json_bool(
        diagnostics,
        &verification,
        &["valid"],
        true,
        "mantle.bundle_verification.valid",
    );
    check_json_string(
        diagnostics,
        &verification,
        &["bundle_identity_blake3"],
        SPACEWASM_MANTLE_BUNDLE_ID,
        "mantle.bundle_verification.bundle_identity",
    );
    check_json_empty_array(
        diagnostics,
        &verification,
        &["diagnostics"],
        "mantle.bundle_verification.diagnostics",
    );
}

fn validate_octet_evidence(
    profile: &SpaceWasmProfile,
    request: &SpaceWasmStaticRequest<'_>,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) {
    let report = decode_json(request.octet_report_bytes, "octet.report", diagnostics);
    let Some(report) = report else {
        return;
    };
    check_json_string(
        diagnostics,
        &report,
        &["schema_version"],
        SPACEWASM_OCTET_RECEIPT_SCHEMA,
        "octet.schema_version",
    );
    check_json_string(
        diagnostics,
        &report,
        &["rail_id"],
        SPACEWASM_OCTET_RAIL_ID,
        "octet.rail_id",
    );
    check_json_string(
        diagnostics,
        &report,
        &["status"],
        SPACEWASM_OCTET_STATUS,
        "octet.status",
    );
    check_json_string(
        diagnostics,
        &report,
        &["profile"],
        SPACEWASM_OCTET_PROFILE_ID,
        "octet.profile",
    );
    check_json_empty_array(diagnostics, &report, &["findings"], "octet.findings");
    check_json_empty_array(diagnostics, &report, &["diagnostics"], "octet.diagnostics");
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "profile_identity"],
        &with_b3_prefix(&profile.octet.profile_identity),
        "octet.wasm_artifact.profile_identity",
    );
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "registry_identity"],
        &with_b3_prefix(&profile.octet.registry_identity),
        "octet.wasm_artifact.registry_identity",
    );
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "cohort_identity"],
        &with_b3_prefix(&profile.octet.cohort_identity),
        "octet.wasm_artifact.cohort_identity",
    );
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "exact_artifact_identity"],
        &with_b3_prefix(SPACEWASM_MODULE_BLAKE3),
        "octet.wasm_artifact.exact_artifact_identity",
    );
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "verification_role"],
        "boundary",
        "octet.wasm_artifact.verification_role",
    );
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "target_runtime", "runtime_id"],
        SPACEWASM_RUNTIME_ID,
        "octet.target_runtime.runtime_id",
    );
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "target_runtime", "source_revision"],
        SPACEWASM_SOURCE_REVISION,
        "octet.target_runtime.source_revision",
    );
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "target_runtime", "cohort_identity"],
        &with_b3_prefix(&profile.octet.runtime_cohort_identity),
        "octet.target_runtime.cohort_identity",
    );
    check_json_string(
        diagnostics,
        &report,
        &["wasm_artifact", "facts", "artifact_kind"],
        "module",
        "octet.facts.artifact_kind",
    );
    check_json_string_array(
        diagnostics,
        &report,
        &["wasm_artifact", "facts", "proposals"],
        &[SPACEWASM_EXPECTED_PROPOSAL.to_string()],
        "octet.facts.proposals",
    );
    check_json_empty_array(
        diagnostics,
        &report,
        &["wasm_artifact", "facts", "imports"],
        "octet.facts.imports",
    );
    let exports = report
        .pointer("/wasm_artifact/facts/exports")
        .and_then(Value::as_array);
    let export_matches = exports.is_some_and(|exports| {
        exports.len() == 1
            && exports[0].get("namespace").and_then(Value::as_str) == Some("module")
            && exports[0].get("name").and_then(Value::as_str) == Some("run")
            && exports[0].get("kind").and_then(Value::as_str) == Some("Func")
    });
    if !export_matches {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "external-evidence-mismatch",
            "octet.facts.exports",
            "expected exactly module::run::Func",
        ));
    }
    check_json_u64(
        diagnostics,
        &report,
        &["wasm_artifact", "facts", "memory_count"],
        0,
        "octet.facts.memory_count",
    );
    check_json_bool(
        diagnostics,
        &report,
        &["wasm_artifact", "facts", "has_start"],
        false,
        "octet.facts.has_start",
    );
}

fn validate_descriptor(
    descriptor: &SpaceWasmDescriptor,
    profile: &SpaceWasmProfile,
    identities: &SpaceWasmAdmittedIdentityRefs<'_>,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) {
    check_equal(
        diagnostics,
        "descriptor.schema_version",
        &descriptor.schema_version,
        SPACEWASM_DESCRIPTOR_SCHEMA,
    );
    check_equal(
        diagnostics,
        "descriptor.adapter_kind",
        &descriptor.adapter_kind,
        SPACEWASM_ADAPTER_KIND,
    );
    check_equal(
        diagnostics,
        "descriptor.artifact_kind",
        &descriptor.artifact_kind,
        SPACEWASM_ARTIFACT_KIND,
    );
    validate_identity(
        diagnostics,
        "descriptor.profile_identity",
        &descriptor.profile_identity,
        identities.profile,
    );
    validate_identity(
        diagnostics,
        "descriptor.abi_profile_identity",
        &descriptor.abi_profile_identity,
        &identities.plans.abi_profile_identity,
    );
    validate_identity(
        diagnostics,
        "descriptor.capability_plan_identity",
        &descriptor.capability_plan_identity,
        &identities.plans.capability_plan_identity,
    );
    validate_identity(
        diagnostics,
        "descriptor.effect_plan_identity",
        &descriptor.effect_plan_identity,
        &identities.plans.effect_plan_identity,
    );
    validate_identity(
        diagnostics,
        "descriptor.resource_plan_identity",
        &descriptor.resource_plan_identity,
        &identities.plans.resource_plan_identity,
    );
    validate_identity(
        diagnostics,
        "descriptor.source_identity",
        &descriptor.source_identity,
        identities.source,
    );
    validate_identity(
        diagnostics,
        "descriptor.canonical_input_identity",
        &descriptor.canonical_input_identity,
        identities.canonical_input,
    );
    validate_schema_ref(
        diagnostics,
        "descriptor.input_schema",
        &descriptor.input_schema,
    );
    validate_schema_ref(
        diagnostics,
        "descriptor.output_schema",
        &descriptor.output_schema,
    );
    validate_identity(
        diagnostics,
        "descriptor.module_identity",
        &descriptor.module_identity,
        identities.module,
    );
    validate_identity(
        diagnostics,
        "descriptor.runtime_identity",
        &descriptor.runtime_identity,
        identities.runtime,
    );
    validate_identity(
        diagnostics,
        "descriptor.mantle_bundle_identity",
        &descriptor.mantle_bundle_identity,
        identities.mantle_bundle,
    );
    validate_identity(
        diagnostics,
        "descriptor.octet_report_identity",
        &descriptor.octet_report_identity,
        identities.octet_report,
    );
    check_equal_vec(
        diagnostics,
        "descriptor.allowed_host_functions",
        &descriptor.allowed_host_functions,
        &profile.abi.allowed_host_functions,
    );
    if descriptor.component_fallback || descriptor.wit_world.is_some() {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "profile-confusion",
            "descriptor",
            "SpaceWasm core profile cannot accept WIT, components, or fallback",
        ));
    }
    if !descriptor.claims.is_empty() {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "semantic-overclaim",
            "descriptor.claims",
            "experimental static descriptor cannot add positive semantic claims",
        ));
    }
    check_equal_vec(
        diagnostics,
        "descriptor.non_claims",
        &descriptor.non_claims,
        &profile.non_claims,
    );
    let expected_edges = expected_parent_edges(descriptor);
    if sorted_unique(&descriptor.parent_edges) != expected_edges {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "parent-edge-mismatch",
            "descriptor.parent_edges",
            "parent edges do not match the exact typed identity layers",
        ));
    }
}

pub fn expected_spacewasm_parent_edges(
    descriptor: &SpaceWasmDescriptor,
) -> Vec<SpaceWasmParentEdge> {
    expected_parent_edges(descriptor)
}

fn expected_parent_edges(descriptor: &SpaceWasmDescriptor) -> Vec<SpaceWasmParentEdge> {
    sorted_unique(&[
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::SourceEnvelope,
            parent_identity: descriptor.source_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::CanonicalInput,
            parent_identity: descriptor.canonical_input_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::InputSchema,
            parent_identity: descriptor.input_schema.identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::OutputSchema,
            parent_identity: descriptor.output_schema.identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::AdapterProfile,
            parent_identity: descriptor.profile_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::AbiProfile,
            parent_identity: descriptor.abi_profile_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::CapabilityPlan,
            parent_identity: descriptor.capability_plan_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::EffectPlan,
            parent_identity: descriptor.effect_plan_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::ResourcePlan,
            parent_identity: descriptor.resource_plan_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::Module,
            parent_identity: descriptor.module_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::SpacewasmRuntime,
            parent_identity: descriptor.runtime_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::MantleBundle,
            parent_identity: descriptor.mantle_bundle_identity.clone(),
        },
        SpaceWasmParentEdge {
            role: SpaceWasmParentRole::OctetStaticReport,
            parent_identity: descriptor.octet_report_identity.clone(),
        },
    ])
}

fn validate_canonical_input(
    bytes: &[u8],
    schema: &SpaceWasmSchemaRef,
    maximum_bytes: u64,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) -> Option<CanonicalValueIdentity> {
    validate_canonical_value(bytes, schema, maximum_bytes, "canonical_input", diagnostics)
}

fn validate_canonical_value(
    bytes: &[u8],
    schema: &SpaceWasmSchemaRef,
    maximum_bytes: u64,
    field: &'static str,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) -> Option<CanonicalValueIdentity> {
    let observed_bytes = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if observed_bytes > maximum_bytes {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "resource-limit",
            field,
            "canonical value exceeds the profile byte bound",
        ));
        return None;
    }
    match canonicalize_preserves(
        bytes,
        PreservesSyntax::Packed,
        PreservesSchema {
            schema_version: &schema.schema_version,
            root_label: &schema.root_label,
        },
        PreservesLimits::default(),
    ) {
        Ok(artifact) if artifact.packed_bytes == bytes => {
            Some(CanonicalValueIdentity::compute(bytes))
        }
        Ok(_) => {
            diagnostics.push(SpaceWasmDiagnostic::new(
                "noncanonical-value",
                field,
                "packed bytes are not canonical",
            ));
            None
        }
        Err(error) => {
            diagnostics.push(SpaceWasmDiagnostic::new(
                "invalid-canonical-value",
                field,
                error.to_string(),
            ));
            None
        }
    }
}

fn validate_manifest_member(
    manifest: &Value,
    path: &str,
    role: &str,
    digest: &str,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) {
    let member = manifest
        .get("members")
        .and_then(Value::as_array)
        .and_then(|members| {
            members
                .iter()
                .find(|member| member.get("path").and_then(Value::as_str) == Some(path))
        });
    let matches = member.is_some_and(|member| {
        member.get("role").and_then(Value::as_str) == Some(role)
            && member.get("digest_blake3").and_then(Value::as_str) == Some(digest)
    });
    if !matches {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "external-evidence-mismatch",
            "mantle.bundle_manifest.members",
            format!("missing exact member {path:?} with role {role:?}"),
        ));
    }
}

fn decode_json(
    bytes: &[u8],
    field: &'static str,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) -> Option<Value> {
    match serde_json::from_slice(bytes) {
        Ok(value) => Some(value),
        Err(error) => {
            diagnostics.push(SpaceWasmDiagnostic::new("decode", field, error.to_string()));
            None
        }
    }
}

fn identity_from_hex<D: IdentityDomainMarker>(
    hex: &str,
    field: &'static str,
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
) -> Option<Identity<D>> {
    match crate::Blake3Hex::parse(hex.to_string()) {
        Ok(hex) => Some(Identity::<D>::from_hex(hex)),
        Err(error) => {
            diagnostics.push(SpaceWasmDiagnostic::new(
                "invalid-identity",
                field,
                error.to_string(),
            ));
            None
        }
    }
}

fn validate_raw_hash(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    field: &'static str,
    bytes: &[u8],
    expected_hex: &str,
) {
    let observed = compute_identity(IdentityDomain::Projection, bytes);
    if observed.hex.as_str() != expected_hex {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "stale-identity",
            field,
            "raw bytes do not match the reviewed BLAKE3 identity",
        ));
    }
}

fn encoded_identity<D: IdentityDomainMarker, T: Serialize>(
    field: &'static str,
    value: &T,
) -> Result<Identity<D>, SpaceWasmValidationError> {
    let bytes = serde_json::to_vec(value).map_err(|error| {
        single_error(SpaceWasmDiagnostic::new(
            "encoding",
            field,
            error.to_string(),
        ))
    })?;
    Ok(Identity::<D>::compute(&bytes))
}

pub fn spacewasm_schema_identity(schema_version: &str, root_label: &str) -> ProjectionIdentity {
    let material = format!("{schema_version}\0{root_label}");
    ProjectionIdentity::compute(material.as_bytes())
}

fn validate_schema_ref(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    field: &'static str,
    schema: &SpaceWasmSchemaRef,
) {
    if schema.schema_version.is_empty()
        || schema.root_label.is_empty()
        || schema.schema_version.contains('\0')
        || schema.root_label.contains('\0')
    {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "invalid-schema",
            field,
            "schema version and root label must be nonempty and contain no NUL",
        ));
        return;
    }
    let expected = spacewasm_schema_identity(&schema.schema_version, &schema.root_label);
    validate_identity(diagnostics, field, &schema.identity, &expected);
}

fn validate_identity<D: IdentityDomainMarker>(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    field: &'static str,
    identity: &WireIdentity,
    expected: &Identity<D>,
) {
    validate_identity_domain(diagnostics, field, identity, D::DOMAIN);
    if identity != &expected.to_wire() {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "stale-identity",
            field,
            "identity does not match the exact admitted bytes",
        ));
    }
}

fn validate_identity_domain(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    field: &'static str,
    identity: &WireIdentity,
    domain: IdentityDomain,
) {
    if let Err(error) = identity.validate_domain(domain) {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "identity-domain",
            field,
            error.to_string(),
        ));
    }
}

fn check_json_string(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    value: &Value,
    path: &[&str],
    expected: &str,
    field: &'static str,
) {
    if json_path(value, path).and_then(Value::as_str) != Some(expected) {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "external-evidence-mismatch",
            field,
            format!("expected {expected:?}"),
        ));
    }
}

fn check_json_bool(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    value: &Value,
    path: &[&str],
    expected: bool,
    field: &'static str,
) {
    if json_path(value, path).and_then(Value::as_bool) != Some(expected) {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "external-evidence-mismatch",
            field,
            format!("expected {expected}"),
        ));
    }
}

fn check_json_u64(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    value: &Value,
    path: &[&str],
    expected: u64,
    field: &'static str,
) {
    if json_path(value, path).and_then(Value::as_u64) != Some(expected) {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "external-evidence-mismatch",
            field,
            format!("expected {expected}"),
        ));
    }
}

fn check_json_empty_array(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    value: &Value,
    path: &[&str],
    field: &'static str,
) {
    if !json_path(value, path)
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty)
    {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "external-evidence-mismatch",
            field,
            "expected an empty array",
        ));
    }
}

fn check_json_string_array(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    value: &Value,
    path: &[&str],
    expected: &[String],
    field: &'static str,
) {
    let actual = json_path(value, path)
        .and_then(Value::as_array)
        .and_then(|values| {
            values
                .iter()
                .map(Value::as_str)
                .map(|value| value.map(str::to_string))
                .collect::<Option<Vec<_>>>()
        });
    if actual.as_deref() != Some(expected) {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "external-evidence-mismatch",
            field,
            "ordered string array differs from the reviewed evidence",
        ));
    }
}

fn json_path<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for segment in path {
        current = current.get(*segment)?;
    }
    Some(current)
}

fn check_equal_bytes(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    field: &'static str,
    actual: &[u8],
    expected: &[u8],
) {
    if actual != expected {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "stale-profile",
            field,
            "bytes differ from the reviewed generated profile",
        ));
    }
}

fn check_equal(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    field: &'static str,
    actual: &str,
    expected: &str,
) {
    if actual != expected {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "unsupported-profile",
            field,
            format!("expected {expected:?}, got {actual:?}"),
        ));
    }
}

fn check_equal_vec(
    diagnostics: &mut Vec<SpaceWasmDiagnostic>,
    field: &'static str,
    actual: &[String],
    expected: &[String],
) {
    if actual != expected {
        diagnostics.push(SpaceWasmDiagnostic::new(
            "unsupported-profile",
            field,
            "ordered values differ from the reviewed profile",
        ));
    }
}

fn sorted_unique<T>(values: &[T]) -> Vec<T>
where
    T: Clone + Ord,
{
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

fn string_values(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn with_b3_prefix(value: &str) -> String {
    format!("{SPACEWASM_BLAKE3_PREFIX}{value}")
}

fn validation_error(diagnostics: Vec<SpaceWasmDiagnostic>) -> SpaceWasmValidationError {
    SpaceWasmValidationError {
        diagnostics: diagnostics
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
    }
}

fn single_error(diagnostic: SpaceWasmDiagnostic) -> SpaceWasmValidationError {
    SpaceWasmValidationError {
        diagnostics: vec![diagnostic],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PreservesSchema, encode_typed_preserves};

    const PROFILE_BYTES: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/generated/profile.json");
    const MANTLE_MANIFEST_BYTES: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/upstream/mantle/bundle-manifest.json");
    const MANTLE_MATERIALIZATION_BYTES: &[u8] = include_bytes!(
        "../../../adapters/spacewasm-mvp/upstream/mantle/materialization-report.json"
    );
    const MANTLE_PROFILE_BYTES: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/upstream/mantle/profile.json");
    const MANTLE_VERIFICATION_BYTES: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/upstream/mantle/bundle-verification.json");
    const OCTET_REGISTRY_BYTES: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/upstream/octet/profiles.json");
    const OCTET_SUPPORT_BYTES: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/upstream/octet/support-projection.json");
    const OCTET_REPORT_BYTES: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/upstream/octet/evidence-rail-receipt.json");
    const MODULE_BYTES: &[u8] =
        include_bytes!("../../../adapters/spacewasm-mvp/upstream/mantle/mvp-positive.wasm");
    const TEST_SCHEMA_VERSION: &str = "kamacite.spacewasm-test-value.v1";
    const TEST_ROOT_LABEL: &str = "TestValue";
    const SOURCE_BYTES: &[u8] = b"reviewed test source";

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct TestValue {
        message: String,
    }

    fn canonical_input() -> Result<Vec<u8>, crate::PreservesCodecError> {
        encode_typed_preserves(
            &TestValue {
                message: "hello".to_string(),
            },
            PreservesSchema {
                schema_version: TEST_SCHEMA_VERSION,
                root_label: TEST_ROOT_LABEL,
            },
            PreservesLimits::default(),
        )
        .map(|artifact| artifact.packed_bytes)
    }

    fn descriptor(input: &[u8]) -> Result<SpaceWasmDescriptor, Box<dyn std::error::Error>> {
        let profile: SpaceWasmProfile = serde_json::from_slice(PROFILE_BYTES)?;
        let plan_identities = spacewasm_profile_plan_identities(&profile)?;
        let source_identity = compute_identity(IdentityDomain::Source, SOURCE_BYTES);
        let input_schema = SpaceWasmSchemaRef {
            schema_version: TEST_SCHEMA_VERSION.to_string(),
            root_label: TEST_ROOT_LABEL.to_string(),
            identity: spacewasm_schema_identity(TEST_SCHEMA_VERSION, TEST_ROOT_LABEL).to_wire(),
        };
        let output_schema = input_schema.clone();
        let mut descriptor = SpaceWasmDescriptor {
            schema_version: SPACEWASM_DESCRIPTOR_SCHEMA.to_string(),
            adapter_kind: SPACEWASM_ADAPTER_KIND.to_string(),
            artifact_kind: SPACEWASM_ARTIFACT_KIND.to_string(),
            profile_identity: compute_identity(IdentityDomain::Adapter, PROFILE_BYTES),
            abi_profile_identity: plan_identities.abi_profile_identity.to_wire(),
            capability_plan_identity: plan_identities.capability_plan_identity.to_wire(),
            effect_plan_identity: plan_identities.effect_plan_identity.to_wire(),
            resource_plan_identity: plan_identities.resource_plan_identity.to_wire(),
            source_identity,
            canonical_input_identity: compute_identity(IdentityDomain::CanonicalValue, input),
            input_schema,
            output_schema,
            module_identity: compute_identity(IdentityDomain::TargetArtifact, MODULE_BYTES),
            runtime_identity: compute_identity(IdentityDomain::Evaluator, MANTLE_PROFILE_BYTES),
            mantle_bundle_identity: WireIdentity::new(
                IdentityDomain::DependencyClosure,
                crate::Blake3Hex::parse(SPACEWASM_MANTLE_BUNDLE_ID.to_string())?,
            ),
            octet_report_identity: compute_identity(IdentityDomain::Projection, OCTET_REPORT_BYTES),
            allowed_host_functions: Vec::new(),
            component_fallback: false,
            wit_world: None,
            parent_edges: Vec::new(),
            claims: Vec::new(),
            non_claims: profile.non_claims,
        };
        descriptor.parent_edges = expected_spacewasm_parent_edges(&descriptor);
        Ok(descriptor)
    }

    fn request<'a>(
        descriptor: &'a SpaceWasmDescriptor,
        input: &'a [u8],
    ) -> SpaceWasmStaticRequest<'a> {
        SpaceWasmStaticRequest {
            profile_bytes: PROFILE_BYTES,
            source_envelope_bytes: SOURCE_BYTES,
            mantle_bundle_manifest_bytes: MANTLE_MANIFEST_BYTES,
            mantle_materialization_report_bytes: MANTLE_MATERIALIZATION_BYTES,
            mantle_profile_bytes: MANTLE_PROFILE_BYTES,
            mantle_bundle_verification_bytes: MANTLE_VERIFICATION_BYTES,
            octet_registry_bytes: OCTET_REGISTRY_BYTES,
            octet_support_projection_bytes: OCTET_SUPPORT_BYTES,
            octet_report_bytes: OCTET_REPORT_BYTES,
            module_bytes: MODULE_BYTES,
            canonical_input_bytes: input,
            descriptor,
        }
    }

    // r[verify kamacite.spacewasm_adapter.boundary]
    // r[verify kamacite.spacewasm_adapter.profile]
    // r[verify kamacite.spacewasm_adapter.abi]
    // r[verify kamacite.spacewasm_adapter.identity]
    // r[verify kamacite.spacewasm_adapter.functional_core]
    // r[verify kamacite.spacewasm_adapter.preserves_io]
    // r[verify kamacite.spacewasm_adapter.receipt]
    // r[verify kamacite.spacewasm_adapter.valence_export]
    #[test]
    fn exact_external_evidence_is_static_only_when_guest_abi_is_absent()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = canonical_input()?;
        let descriptor = descriptor(&input)?;
        let result = validate_spacewasm_static(&request(&descriptor, &input));
        assert!(result.is_ok(), "{result:?}");
        if let Ok(result) = result {
            assert_eq!(result.state, SpaceWasmPlanningState::StaticOnly);
            assert_eq!(
                result.blockers,
                vec![
                    SpaceWasmBlocker::MantleReferenceModuleDoesNotImplementKamaciteByteAbi,
                    SpaceWasmBlocker::OctetProfilePermitsOnlyHostlessRunExport,
                    SpaceWasmBlocker::KamaciteGuestMaterializationBundleUnavailable,
                    SpaceWasmBlocker::ConsumerSpacewasmShellUnavailable,
                ]
            );
            assert_eq!(
                result.module_identity.hex().as_str(),
                SPACEWASM_MODULE_BLAKE3
            );
        }

        let exact_request = request(&descriptor, &input);
        let receipt = build_spacewasm_static_receipt(&exact_request)?;
        assert_eq!(receipt.receipt_identity.domain(), IdentityDomain::Receipt);
        assert!(validate_spacewasm_static_receipt(&exact_request, &receipt).is_ok());
        let projection = crate::project_spacewasm_static_to_valence(&exact_request)?;
        let repeated_projection = crate::project_spacewasm_static_to_valence(&exact_request)?;
        assert_eq!(projection, repeated_projection);
        assert_eq!(
            projection.projection_identity.domain(),
            IdentityDomain::Projection
        );
        assert_eq!(
            projection.body.parent_edges.len(),
            descriptor.parent_edges.len() + 1
        );
        let source_artifact = projection
            .body
            .artifacts
            .iter()
            .find(|artifact| artifact.role == "source-envelope");
        assert!(source_artifact.is_some());
        if let Some(source_artifact) = source_artifact {
            assert!(matches!(
                source_artifact.spans,
                crate::SourceSpanSet::Present { .. }
            ));
        }
        let mut stale_receipt = receipt;
        stale_receipt.body.schema_version = "substituted".to_string();
        assert!(validate_spacewasm_static_receipt(&exact_request, &stale_receipt).is_err());

        let output_identity =
            validate_spacewasm_canonical_output(&input, &descriptor.output_schema)?;
        assert_eq!(output_identity.domain(), IdentityDomain::CanonicalValue);
        assert!(
            validate_spacewasm_canonical_output(b"malformed", &descriptor.output_schema).is_err()
        );
        Ok(())
    }

    #[test]
    fn profile_component_and_identity_confusion_fail_closed()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = canonical_input()?;
        let mut descriptor = descriptor(&input)?;
        descriptor.component_fallback = true;
        descriptor.wit_world = Some("onix:runtime/runtime".to_string());
        descriptor.module_identity = descriptor.canonical_input_identity.clone();
        descriptor.parent_edges = expected_spacewasm_parent_edges(&descriptor);
        let error = validate_spacewasm_static(&request(&descriptor, &input));
        assert!(error.is_err());
        if let Err(error) = error {
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code == "profile-confusion")
            );
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code == "identity-domain")
            );
        }
        Ok(())
    }

    #[test]
    fn stale_external_report_and_noncanonical_input_fail_closed()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = canonical_input()?;
        let exact_descriptor = descriptor(&input)?;
        let mut stale_report = OCTET_REPORT_BYTES.to_vec();
        stale_report.push(b' ');
        let mut stale_request = request(&exact_descriptor, &input);
        stale_request.octet_report_bytes = &stale_report;
        assert!(validate_spacewasm_static(&stale_request).is_err());

        let mut substituted_source_request = request(&exact_descriptor, &input);
        substituted_source_request.source_envelope_bytes = b"substituted source";
        assert!(validate_spacewasm_static(&substituted_source_request).is_err());

        let invalid_input = b"not packed Preserves";
        let invalid_descriptor = descriptor(invalid_input)?;
        let invalid_request = request(&invalid_descriptor, invalid_input);
        let error = validate_spacewasm_static(&invalid_request);
        assert!(error.is_err());
        if let Err(error) = error {
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code == "invalid-canonical-value")
            );
        }
        Ok(())
    }

    #[test]
    fn parent_substitution_and_semantic_claims_fail_closed()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = canonical_input()?;
        let mut descriptor = descriptor(&input)?;
        descriptor.claims.push("SpaceWasm correctness".to_string());
        descriptor.parent_edges.pop();
        let error = validate_spacewasm_static(&request(&descriptor, &input));
        assert!(error.is_err());
        if let Err(error) = error {
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code == "semantic-overclaim")
            );
            assert!(
                error
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code == "parent-edge-mismatch")
            );
        }
        Ok(())
    }
}
