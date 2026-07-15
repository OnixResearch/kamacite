use crate::{
    Identity, IdentityDomain, PRESERVES_CANONICAL_CODEC, PreservesLimits, PreservesSchema,
    PreservesSyntax, canonicalize_preserves, compute_identity, encode_typed_preserves,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

pub const WASM_COMPONENT_PROFILE_SCHEMA: &str = "kamacite.wasm-component-static-profile.v1";
pub const WASM_COMPONENT_PROFILE_ID: &str = "kamacite.wit-component-adapter.v1";
pub const WASM_COMPONENT_DESCRIPTOR_SCHEMA: &str = "kamacite.wasm-component-descriptor.v1";
pub const WASM_COMPONENT_MATERIALIZATION_SCHEMA: &str =
    "mantle.component-materialization-bundle.v1";
pub const WASM_COMPONENT_RUNTIME_PROFILE_ID: &str = "molten.wasm.component.v1";
pub const WASM_COMPONENT_STATIC_RECEIPT_SCHEMA: &str = "kamacite.wasm-component-static-receipt.v1";
pub const WASM_COMPONENT_WIT_PACKAGE: &str = "molten:component-runtime@1.0.0";
pub const WASM_COMPONENT_WIT_WORLD: &str = "actor";
pub const WASM_COMPONENT_INVOKE_EXPORT: &str = "invoke";
pub const WASM_COMPONENT_ARTIFACT_KIND: &str = "component";
pub const WASM_COMPONENT_INPUT_CARRIER: &str = "canonical-preserves-bytes";
pub const WASM_COMPONENT_OUTPUT_CARRIER: &str = "canonical-preserves-bytes";
pub const WASM_COMPONENT_ERROR_CARRIER: &str = "utf8-string";
pub const WASM_COMPONENT_INITIAL_IMPORT_POLICY: &str = "deny-all";
pub const WASM_COMPONENT_MATERIALIZATION_POLICY: &str = "exact-mantle-bundle-required";
pub const WASM_COMPONENT_SOURCE_REPOSITORY: &str = "https://github.com/OnixResearch/aspen";
pub const WASM_COMPONENT_SOURCE_REVISION: &str = "62b02a3435a7000a85bbb6988a88f8afe5effbbb";
pub const WASM_COMPONENT_RUNTIME_PROFILE_BLAKE3: &str =
    "c2ef4db1ab1010512b0d7b023e6a64f80c00c11850e041fa5347d91028d8d96f";
pub const WASM_COMPONENT_WIT_BLAKE3: &str =
    "83be349bb27975ada30dbe60817c5404df7862babdf02ae229b399886e76d5e8";

const WASM_COMPONENT_RUNTIME_PROFILE_PATH: &str =
    "docs/wasm-component-runtime/generated/profile.json";
const WASM_COMPONENT_WIT_PATH: &str = "wit/molten-component-runtime/runtime.wit";
const WASM_COMPONENT_MOLTEN_PROFILE_SCHEMA: &str = "molten.wasm-component-profile.v1";
const WASM_COMPONENT_MOLTEN_PROFILE_SCHEMA_VERSION: u32 = 1;
const WASM_COMPONENT_PROFILE_SOURCE_LANGUAGE: &str = "nickel";
const WASM_COMPONENT_RUNTIME_STRATEGY: &str = "portable-component";
const WASM_COMPONENT_EVIDENCE_SCOPE: &str = "production";
const WASM_COMPONENT_PRODUCER: &str = "mantle";
const WASM_COMPONENT_STATIC_RECEIPT_ROOT: &str = "WasmComponentStaticReceiptBody";
const WASM_COMPONENT_FIXED_GROWTH: &str = "fixed";
const WASM_COMPONENT_RECORDED_INPUTS: &str = "recorded-effects-only";
const WASM_COMPONENT_BLAKE3_PREFIX: &str = "blake3:";

const WASM_COMPONENT_RESULT_CLASSES: &[&str] = &["denied", "error", "success", "trap"];
const WASM_COMPONENT_REQUIRED_FEATURES: &[&str] = &[
    "bulk-memory",
    "component-model",
    "multi-value",
    "reference-types",
    "simd",
];
const WASM_COMPONENT_CHECKED_LAWS: &[&str] = &[
    "wit-source-identity-bound",
    "runtime-profile-identity-bound",
    "descriptor-layers-distinct",
    "materialization-remeasured",
    "canonical-input-before-planning",
    "ambient-imports-denied",
    "missing-production-evidence-denies-execution",
];
const WASM_COMPONENT_NON_CLAIMS: &[&str] = &[
    "WIT and WebAssembly are target artifacts, not Kamacite canonical IR",
    "static validation is not component execution or behavioral correctness",
    "profile compatibility is not Wasmtime correctness or sandbox completeness",
    "materialization references do not transfer Mantle build trust",
    "capability declarations do not grant host authority",
    "no arbitrary Nickel, Rust, Steel, or Wasm semantic equivalence",
    "no release eligibility or production readiness",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentProfile {
    pub schema_version: String,
    pub profile_id: String,
    pub source: WasmComponentProfileSource,
    pub runtime: WasmComponentRuntimeProfileExport,
    pub adapter: WasmComponentAdapterProfile,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentProfileSource {
    pub repository: String,
    pub revision: String,
    pub runtime_profile_path: String,
    pub runtime_profile_blake3: String,
    pub wit_path: String,
    pub wit_blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentRuntimeProfileExport {
    pub schema_id: String,
    pub schema_version: u32,
    pub source_language: String,
    pub profile: WasmComponentRuntimeProfile,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentRuntimeProfile {
    pub profile_id: String,
    pub evidence_scope: String,
    pub runtime_strategy: String,
    pub toolchain: WasmComponentToolchain,
    pub wit: WasmComponentWitCohort,
    pub features: WasmComponentFeatures,
    pub determinism: WasmComponentDeterminism,
    pub resources: WasmComponentResources,
    pub allowed_imports: Vec<String>,
    pub allowed_wasi_interfaces: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentToolchain {
    pub wasmtime: String,
    pub wasm_tools: String,
    pub wasmparser: String,
    pub wit_bindgen: String,
    pub wasmtime_wasi: String,
    pub wasi_package: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentWitCohort {
    pub package: String,
    pub world: String,
    pub source_ref: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentFeatures {
    pub component_model: bool,
    pub multi_value: bool,
    pub bulk_memory: bool,
    pub reference_types: bool,
    pub simd: bool,
    pub relaxed_simd: bool,
    pub threads: bool,
    pub tail_call: bool,
    pub multi_memory: bool,
    pub exceptions: bool,
    pub gc: bool,
    pub memory64: bool,
    pub extended_const: bool,
    pub function_references: bool,
    pub custom_page_sizes: bool,
    pub wide_arithmetic: bool,
    pub component_async: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentDeterminism {
    pub fuel_interruption: bool,
    pub nan_canonicalization: bool,
    pub relaxed_simd_deterministic: bool,
    pub memory_growth: String,
    pub table_growth: String,
    pub host_inputs: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentResources {
    pub fuel: u64,
    pub max_component_bytes: u64,
    pub max_wit_bytes: u64,
    pub max_memory_bytes: u64,
    pub max_table_elements: u64,
    pub max_instances: u64,
    pub max_memories: u64,
    pub max_tables: u64,
    pub max_stack_bytes: u64,
    pub max_hostcall_bytes: u64,
    pub max_result_bytes: u64,
    pub max_concurrency: u64,
    pub max_imports: u64,
    pub max_exports: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentAdapterProfile {
    pub descriptor_schema: String,
    pub materialization_schema: String,
    pub artifact_kind: String,
    pub invoke_export: String,
    pub input_carrier: String,
    pub output_carrier: String,
    pub error_carrier: String,
    pub canonical_codec: String,
    pub production_materialization: String,
    pub initial_import_policy: String,
    pub result_classes: Vec<String>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WasmComponentParentRole {
    Component,
    InputSchema,
    MaterializationBundle,
    OutputSchema,
    ResourceProfile,
    RuntimeProfile,
    ToolchainCohort,
    WitSource,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentParentEdge {
    pub role: WasmComponentParentRole,
    pub identity: Identity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentSchemaRef {
    pub identity: Identity,
    pub schema_version: String,
    pub root_label: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WasmComponentReplayClass {
    Denied,
    RecordedEffectRequired,
    Recompute,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentImportBinding {
    pub import: String,
    pub capability_identity: Identity,
    pub effect_operation_identity: Identity,
    pub handler_profile_identity: Identity,
    pub policy_identity: Identity,
    pub authority_identity: Identity,
    pub resource_identity: Identity,
    pub replay_class: WasmComponentReplayClass,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentDescriptor {
    pub schema_version: String,
    pub adapter_id: String,
    pub adapter_version: String,
    pub artifact_kind: String,
    pub wit_package: String,
    pub wit_world: String,
    pub wit_source_identity: Identity,
    pub component_identity: Identity,
    pub runtime_profile_identity: Identity,
    pub toolchain_cohort_identity: Identity,
    pub resource_profile_identity: Identity,
    pub input_schema: WasmComponentSchemaRef,
    pub output_schema: WasmComponentSchemaRef,
    pub imports: Vec<WasmComponentImportBinding>,
    pub exports: Vec<String>,
    pub materialization_bundle_identity: Option<Identity>,
    pub parent_edges: Vec<WasmComponentParentEdge>,
    pub semantic_claims: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WasmComponentEvidenceScope {
    Production,
    TestOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentMaterializationBundle {
    pub schema_version: String,
    pub producer: String,
    pub evidence_scope: WasmComponentEvidenceScope,
    pub component_identity: Identity,
    pub component_byte_length: u64,
    pub wit_identity: Identity,
    pub wit_byte_length: u64,
    pub runtime_profile_identity: Identity,
    pub toolchain_cohort_identity: Identity,
    pub octet_report_identity: Identity,
    pub stage_receipt_identities: Vec<Identity>,
    pub portable_component: bool,
    pub contains_precompiled_bytes: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentInspection {
    pub artifact_kind: String,
    pub component_identity: Identity,
    pub wit_world: String,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub enabled_features: Vec<String>,
    pub fixed_memory: bool,
    pub initial_memory_bytes: u64,
    pub maximum_memory_bytes: u64,
    pub fixed_table: bool,
    pub initial_table_elements: u64,
    pub maximum_table_elements: u64,
    pub instances: u64,
    pub memories: u64,
    pub tables: u64,
}

#[derive(Clone, Debug)]
pub struct WasmComponentStaticRequest<'a> {
    pub profile_bytes: &'a [u8],
    pub runtime_profile_bytes: &'a [u8],
    pub wit_bytes: &'a [u8],
    pub descriptor: &'a WasmComponentDescriptor,
    pub canonical_input_bytes: &'a [u8],
    pub materialization_manifest_bytes: Option<&'a [u8]>,
    pub component_bytes: Option<&'a [u8]>,
    pub inspection: Option<&'a WasmComponentInspection>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WasmComponentPlanningState {
    Executable,
    StaticOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WasmComponentBlocker {
    ExternalMaterializationVerifierUnavailable,
    MissingComponentBytes,
    MissingInspection,
    MissingMaterializationBundle,
    TestOnlyMaterialization,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WasmComponentExecutionPlan {
    pub descriptor_identity: Identity,
    pub component_identity: Identity,
    pub wit_identity: Identity,
    pub runtime_profile_identity: Identity,
    pub materialization_bundle_identity: Identity,
    pub inspection_identity: Identity,
    pub canonical_input_identity: Identity,
    pub input_schema_identity: Identity,
    pub output_schema_identity: Identity,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub capability_identities: Vec<Identity>,
    pub effect_operation_identities: Vec<Identity>,
    pub authority_identities: Vec<Identity>,
    pub resource_identities: Vec<Identity>,
    pub resource_limits: WasmComponentResources,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WasmComponentStaticValidation {
    pub state: WasmComponentPlanningState,
    pub profile_identity: Identity,
    pub runtime_profile_identity: Identity,
    pub wit_identity: Identity,
    pub descriptor_identity: Identity,
    pub canonical_input_identity: Identity,
    pub component_identity: Option<Identity>,
    pub inspection_identity: Option<Identity>,
    pub materialization_bundle_identity: Option<Identity>,
    pub execution_plan_identity: Option<Identity>,
    pub execution_plan: Option<WasmComponentExecutionPlan>,
    pub blockers: Vec<WasmComponentBlocker>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentStaticReceiptBody {
    pub schema_version: String,
    pub state: WasmComponentPlanningState,
    pub source_repository: String,
    pub source_revision: String,
    pub profile_identity: Identity,
    pub runtime_profile_identity: Identity,
    pub wit_identity: Identity,
    pub descriptor_identity: Identity,
    pub canonical_input_identity: Identity,
    pub component_identity: Option<Identity>,
    pub inspection_identity: Option<Identity>,
    pub materialization_bundle_identity: Option<Identity>,
    pub blockers: Vec<WasmComponentBlocker>,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WasmComponentStaticReceipt {
    pub body: WasmComponentStaticReceiptBody,
    pub receipt_identity: Identity,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WasmComponentDiagnostic {
    pub code: &'static str,
    pub field: &'static str,
    pub message: String,
}

impl WasmComponentDiagnostic {
    fn new(code: &'static str, field: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            field,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WasmComponentValidationError {
    pub diagnostics: Vec<WasmComponentDiagnostic>,
}

impl Display for WasmComponentValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let rendered = self
            .diagnostics
            .iter()
            .map(|diagnostic| {
                format!(
                    "{}:{}:{}",
                    diagnostic.code, diagnostic.field, diagnostic.message
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        formatter.write_str(&rendered)
    }
}

impl std::error::Error for WasmComponentValidationError {}

struct AdmittedProfile {
    profile: WasmComponentProfile,
    profile_identity: Identity,
    runtime_profile_identity: Identity,
    wit_identity: Identity,
    wit_byte_length: u64,
}

// r[impl kamacite.wasm_adapter.boundary]
// r[impl kamacite.wasm_adapter.wit_identity]
// r[impl kamacite.wasm_adapter.descriptor]
// r[impl kamacite.wasm_adapter.profile]
// r[impl kamacite.wasm_adapter.materialization]
// r[impl kamacite.wasm_adapter.capabilities]
// r[impl kamacite.wasm_adapter.preserves_io]
// r[impl kamacite.wasm_adapter.functional_core]
pub fn validate_wasm_component_static(
    request: &WasmComponentStaticRequest<'_>,
) -> Result<WasmComponentStaticValidation, WasmComponentValidationError> {
    let admitted = admit_profile(request)?;
    let descriptor_identity = descriptor_identity(request.descriptor)?;
    let mut diagnostics = validate_descriptor(&admitted, request.descriptor);
    let canonical_input_identity = validate_canonical_input(
        &admitted.profile,
        request.descriptor,
        request.canonical_input_bytes,
    )
    .map_err(single_error)?;
    if !diagnostics.is_empty() {
        return Err(validation_error(diagnostics));
    }

    let mut blockers = Vec::new();
    let mut materialization_bundle_identity = None;
    let component_identity = request
        .component_bytes
        .map(|bytes| compute_identity(IdentityDomain::TargetArtifact, bytes));
    let inspection_identity = request
        .inspection
        .map(|inspection| serialized_identity(IdentityDomain::Adapter, "inspection", inspection))
        .transpose()?;
    let execution_plan: Option<WasmComponentExecutionPlan> = None;
    let execution_plan_identity: Option<Identity> = None;
    match request.materialization_manifest_bytes {
        None => {
            blockers.push(WasmComponentBlocker::MissingMaterializationBundle);
            if request.component_bytes.is_some() || request.inspection.is_some() {
                diagnostics.push(WasmComponentDiagnostic::new(
                    "orphan-artifact",
                    "materialization",
                    "component bytes and inspection require a materialization manifest",
                ));
            }
        }
        Some(manifest_bytes) => {
            let bundle = parse_materialization(manifest_bytes)?;
            let bundle_identity =
                compute_identity(IdentityDomain::DependencyClosure, manifest_bytes);
            materialization_bundle_identity = Some(bundle_identity.clone());
            diagnostics.extend(validate_materialization(
                &admitted,
                request.descriptor,
                &bundle,
                &bundle_identity,
                request.component_bytes,
            ));
            if request.component_bytes.is_none() {
                blockers.push(WasmComponentBlocker::MissingComponentBytes);
            }
            if request.inspection.is_none() {
                blockers.push(WasmComponentBlocker::MissingInspection);
            }
            if bundle.evidence_scope == WasmComponentEvidenceScope::TestOnly {
                blockers.push(WasmComponentBlocker::TestOnlyMaterialization);
            }
            if let (Some(component_bytes), Some(inspection)) =
                (request.component_bytes, request.inspection)
            {
                diagnostics.extend(validate_inspection(
                    &admitted,
                    request.descriptor,
                    &bundle,
                    component_bytes,
                    inspection,
                ));
                if diagnostics.is_empty()
                    && bundle.evidence_scope == WasmComponentEvidenceScope::Production
                {
                    blockers.push(WasmComponentBlocker::ExternalMaterializationVerifierUnavailable);
                }
            }
        }
    }
    if !diagnostics.is_empty() {
        return Err(validation_error(diagnostics));
    }
    blockers.sort();
    blockers.dedup();
    let state = WasmComponentPlanningState::StaticOnly;
    Ok(WasmComponentStaticValidation {
        state,
        profile_identity: admitted.profile_identity,
        runtime_profile_identity: admitted.runtime_profile_identity,
        wit_identity: admitted.wit_identity,
        descriptor_identity,
        canonical_input_identity,
        component_identity,
        inspection_identity,
        materialization_bundle_identity,
        execution_plan_identity,
        execution_plan,
        blockers,
        checked_laws: admitted.profile.adapter.checked_laws,
        non_claims: admitted.profile.adapter.non_claims,
    })
}

// r[impl kamacite.wasm_adapter.receipt]
pub fn build_wasm_component_static_receipt(
    request: &WasmComponentStaticRequest<'_>,
) -> Result<WasmComponentStaticReceipt, WasmComponentValidationError> {
    let validation = validate_wasm_component_static(request)?;
    let body = WasmComponentStaticReceiptBody {
        schema_version: WASM_COMPONENT_STATIC_RECEIPT_SCHEMA.to_string(),
        state: validation.state,
        source_repository: WASM_COMPONENT_SOURCE_REPOSITORY.to_string(),
        source_revision: WASM_COMPONENT_SOURCE_REVISION.to_string(),
        profile_identity: validation.profile_identity,
        runtime_profile_identity: validation.runtime_profile_identity,
        wit_identity: validation.wit_identity,
        descriptor_identity: validation.descriptor_identity,
        canonical_input_identity: validation.canonical_input_identity,
        component_identity: validation.component_identity,
        inspection_identity: validation.inspection_identity,
        materialization_bundle_identity: validation.materialization_bundle_identity,
        blockers: validation.blockers,
        checked_laws: validation.checked_laws,
        non_claims: validation.non_claims,
    };
    let artifact = encode_typed_preserves(
        &body,
        PreservesSchema {
            schema_version: WASM_COMPONENT_STATIC_RECEIPT_SCHEMA,
            root_label: WASM_COMPONENT_STATIC_RECEIPT_ROOT,
        },
        PreservesLimits::default(),
    )
    .map_err(|error| {
        single_error(WasmComponentDiagnostic::new(
            "receipt-encoding",
            "receipt",
            error.to_string(),
        ))
    })?;
    Ok(WasmComponentStaticReceipt {
        body,
        receipt_identity: compute_identity(IdentityDomain::Receipt, &artifact.packed_bytes),
    })
}

pub fn validate_wasm_component_static_receipt(
    request: &WasmComponentStaticRequest<'_>,
    receipt: &WasmComponentStaticReceipt,
) -> Result<(), WasmComponentValidationError> {
    let expected = build_wasm_component_static_receipt(request)?;
    if receipt == &expected {
        Ok(())
    } else {
        Err(single_error(WasmComponentDiagnostic::new(
            "stale-receipt",
            "receipt",
            "static receipt differs from the deterministic validation evidence",
        )))
    }
}

fn admit_profile(
    request: &WasmComponentStaticRequest<'_>,
) -> Result<AdmittedProfile, WasmComponentValidationError> {
    let profile =
        serde_json::from_slice::<WasmComponentProfile>(request.profile_bytes).map_err(|error| {
            single_error(WasmComponentDiagnostic::new(
                "decode",
                "profile",
                error.to_string(),
            ))
        })?;
    let runtime_export =
        serde_json::from_slice::<WasmComponentRuntimeProfileExport>(request.runtime_profile_bytes)
            .map_err(|error| {
                single_error(WasmComponentDiagnostic::new(
                    "decode",
                    "runtime_profile",
                    error.to_string(),
                ))
            })?;
    let profile_identity = compute_identity(IdentityDomain::Adapter, request.profile_bytes);
    let runtime_profile_identity =
        compute_identity(IdentityDomain::Evaluator, request.runtime_profile_bytes);
    let wit_identity = compute_identity(IdentityDomain::Source, request.wit_bytes);
    let wit_byte_length = u64::try_from(request.wit_bytes.len()).map_err(|error| {
        single_error(WasmComponentDiagnostic::new(
            "limit",
            "wit_bytes",
            error.to_string(),
        ))
    })?;
    let mut diagnostics = validate_profile(
        &profile,
        &runtime_export,
        &runtime_profile_identity,
        &wit_identity,
    );
    if wit_byte_length > profile.runtime.profile.resources.max_wit_bytes {
        diagnostics.push(WasmComponentDiagnostic::new(
            "limit",
            "wit_bytes",
            "WIT source exceeds the admitted byte limit",
        ));
    }
    if diagnostics.is_empty() {
        Ok(AdmittedProfile {
            profile,
            profile_identity,
            runtime_profile_identity,
            wit_identity,
            wit_byte_length,
        })
    } else {
        Err(validation_error(diagnostics))
    }
}

fn validate_profile(
    profile: &WasmComponentProfile,
    runtime_export: &WasmComponentRuntimeProfileExport,
    runtime_identity: &Identity,
    wit_identity: &Identity,
) -> Vec<WasmComponentDiagnostic> {
    let mut diagnostics = Vec::new();
    check_equal(
        &mut diagnostics,
        "profile",
        profile.schema_version.as_str(),
        WASM_COMPONENT_PROFILE_SCHEMA,
    );
    check_equal(
        &mut diagnostics,
        "profile",
        profile.profile_id.as_str(),
        WASM_COMPONENT_PROFILE_ID,
    );
    check_equal(
        &mut diagnostics,
        "profile.source.repository",
        profile.source.repository.as_str(),
        WASM_COMPONENT_SOURCE_REPOSITORY,
    );
    check_equal(
        &mut diagnostics,
        "profile.source.revision",
        profile.source.revision.as_str(),
        WASM_COMPONENT_SOURCE_REVISION,
    );
    check_equal(
        &mut diagnostics,
        "profile.source.runtime_profile_path",
        profile.source.runtime_profile_path.as_str(),
        WASM_COMPONENT_RUNTIME_PROFILE_PATH,
    );
    check_equal(
        &mut diagnostics,
        "profile.source.wit_path",
        profile.source.wit_path.as_str(),
        WASM_COMPONENT_WIT_PATH,
    );
    check_equal(
        &mut diagnostics,
        "profile.source.runtime_profile_blake3",
        profile.source.runtime_profile_blake3.as_str(),
        WASM_COMPONENT_RUNTIME_PROFILE_BLAKE3,
    );
    check_equal(
        &mut diagnostics,
        "profile.source.wit_blake3",
        profile.source.wit_blake3.as_str(),
        WASM_COMPONENT_WIT_BLAKE3,
    );
    if runtime_identity.hex.as_str() != profile.source.runtime_profile_blake3 {
        diagnostics.push(WasmComponentDiagnostic::new(
            "stale-identity",
            "runtime_profile",
            "runtime profile bytes differ from the pinned source identity",
        ));
    }
    if wit_identity.hex.as_str() != profile.source.wit_blake3 {
        diagnostics.push(WasmComponentDiagnostic::new(
            "stale-identity",
            "wit_source",
            "WIT bytes differ from the pinned source identity",
        ));
    }
    if runtime_export.schema_id != WASM_COMPONENT_MOLTEN_PROFILE_SCHEMA
        || runtime_export.schema_version != WASM_COMPONENT_MOLTEN_PROFILE_SCHEMA_VERSION
        || runtime_export.source_language != WASM_COMPONENT_PROFILE_SOURCE_LANGUAGE
        || runtime_export != &profile.runtime
    {
        diagnostics.push(WasmComponentDiagnostic::new(
            "profile-mismatch",
            "runtime_profile",
            "Kamacite profile does not exactly match the pinned Molten profile export",
        ));
    }
    validate_runtime_profile(profile, &mut diagnostics);
    validate_adapter_profile(profile, &mut diagnostics);
    diagnostics
}

fn validate_runtime_profile(
    profile: &WasmComponentProfile,
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
) {
    let runtime = &profile.runtime.profile;
    check_equal(
        diagnostics,
        "runtime.profile_id",
        runtime.profile_id.as_str(),
        WASM_COMPONENT_RUNTIME_PROFILE_ID,
    );
    check_equal(
        diagnostics,
        "runtime.evidence_scope",
        runtime.evidence_scope.as_str(),
        WASM_COMPONENT_EVIDENCE_SCOPE,
    );
    check_equal(
        diagnostics,
        "runtime.runtime_strategy",
        runtime.runtime_strategy.as_str(),
        WASM_COMPONENT_RUNTIME_STRATEGY,
    );
    check_equal(
        diagnostics,
        "runtime.wit.package",
        runtime.wit.package.as_str(),
        WASM_COMPONENT_WIT_PACKAGE,
    );
    check_equal(
        diagnostics,
        "runtime.wit.world",
        runtime.wit.world.as_str(),
        WASM_COMPONENT_WIT_WORLD,
    );
    let expected_source_ref = format!("{WASM_COMPONENT_BLAKE3_PREFIX}{WASM_COMPONENT_WIT_BLAKE3}");
    check_equal(
        diagnostics,
        "runtime.wit.source_ref",
        runtime.wit.source_ref.as_str(),
        expected_source_ref.as_str(),
    );
    if !runtime.allowed_imports.is_empty() || !runtime.allowed_wasi_interfaces.is_empty() {
        diagnostics.push(WasmComponentDiagnostic::new(
            "ambient-authority",
            "runtime.allowed_imports",
            "initial runtime cohort must deny all host and WASI imports",
        ));
    }
    if !runtime.determinism.fuel_interruption
        || !runtime.determinism.nan_canonicalization
        || !runtime.determinism.relaxed_simd_deterministic
        || runtime.determinism.memory_growth != WASM_COMPONENT_FIXED_GROWTH
        || runtime.determinism.table_growth != WASM_COMPONENT_FIXED_GROWTH
        || runtime.determinism.host_inputs != WASM_COMPONENT_RECORDED_INPUTS
    {
        diagnostics.push(WasmComponentDiagnostic::new(
            "nondeterministic-profile",
            "runtime.determinism",
            "runtime profile does not enforce the reviewed deterministic posture",
        ));
    }
    if runtime.resources.max_concurrency != 1 {
        diagnostics.push(WasmComponentDiagnostic::new(
            "unsupported-profile",
            "runtime.resources.max_concurrency",
            "initial runtime profile requires single-concurrency execution",
        ));
    }
}

fn validate_adapter_profile(
    profile: &WasmComponentProfile,
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
) {
    let adapter = &profile.adapter;
    for (field, actual, expected) in [
        (
            "adapter.descriptor_schema",
            adapter.descriptor_schema.as_str(),
            WASM_COMPONENT_DESCRIPTOR_SCHEMA,
        ),
        (
            "adapter.materialization_schema",
            adapter.materialization_schema.as_str(),
            WASM_COMPONENT_MATERIALIZATION_SCHEMA,
        ),
        (
            "adapter.artifact_kind",
            adapter.artifact_kind.as_str(),
            WASM_COMPONENT_ARTIFACT_KIND,
        ),
        (
            "adapter.invoke_export",
            adapter.invoke_export.as_str(),
            WASM_COMPONENT_INVOKE_EXPORT,
        ),
        (
            "adapter.input_carrier",
            adapter.input_carrier.as_str(),
            WASM_COMPONENT_INPUT_CARRIER,
        ),
        (
            "adapter.output_carrier",
            adapter.output_carrier.as_str(),
            WASM_COMPONENT_OUTPUT_CARRIER,
        ),
        (
            "adapter.error_carrier",
            adapter.error_carrier.as_str(),
            WASM_COMPONENT_ERROR_CARRIER,
        ),
        (
            "adapter.canonical_codec",
            adapter.canonical_codec.as_str(),
            PRESERVES_CANONICAL_CODEC,
        ),
        (
            "adapter.production_materialization",
            adapter.production_materialization.as_str(),
            WASM_COMPONENT_MATERIALIZATION_POLICY,
        ),
        (
            "adapter.initial_import_policy",
            adapter.initial_import_policy.as_str(),
            WASM_COMPONENT_INITIAL_IMPORT_POLICY,
        ),
    ] {
        check_equal(diagnostics, field, actual, expected);
    }
    compare_string_set(
        diagnostics,
        "adapter.result_classes",
        &adapter.result_classes,
        WASM_COMPONENT_RESULT_CLASSES,
    );
    compare_string_set(
        diagnostics,
        "adapter.checked_laws",
        &adapter.checked_laws,
        WASM_COMPONENT_CHECKED_LAWS,
    );
    compare_string_set(
        diagnostics,
        "adapter.non_claims",
        &adapter.non_claims,
        WASM_COMPONENT_NON_CLAIMS,
    );
}

fn descriptor_identity(
    descriptor: &WasmComponentDescriptor,
) -> Result<Identity, WasmComponentValidationError> {
    let bytes = serde_json::to_vec(descriptor).map_err(|error| {
        single_error(WasmComponentDiagnostic::new(
            "encoding",
            "descriptor",
            error.to_string(),
        ))
    })?;
    Ok(compute_identity(IdentityDomain::Adapter, &bytes))
}

fn validate_descriptor(
    admitted: &AdmittedProfile,
    descriptor: &WasmComponentDescriptor,
) -> Vec<WasmComponentDiagnostic> {
    let mut diagnostics = Vec::new();
    let runtime = &admitted.profile.runtime.profile;
    let adapter = &admitted.profile.adapter;
    for (field, actual, expected) in [
        (
            "descriptor.schema_version",
            descriptor.schema_version.as_str(),
            adapter.descriptor_schema.as_str(),
        ),
        (
            "descriptor.artifact_kind",
            descriptor.artifact_kind.as_str(),
            WASM_COMPONENT_ARTIFACT_KIND,
        ),
        (
            "descriptor.wit_package",
            descriptor.wit_package.as_str(),
            runtime.wit.package.as_str(),
        ),
        (
            "descriptor.wit_world",
            descriptor.wit_world.as_str(),
            runtime.wit.world.as_str(),
        ),
    ] {
        check_equal(&mut diagnostics, field, actual, expected);
    }
    if descriptor.adapter_id.trim().is_empty() || descriptor.adapter_version.trim().is_empty() {
        diagnostics.push(WasmComponentDiagnostic::new(
            "missing-field",
            "descriptor.adapter_id",
            "adapter id and version must be nonempty",
        ));
    }
    validate_identity(
        &mut diagnostics,
        "descriptor.wit_source_identity",
        &descriptor.wit_source_identity,
        IdentityDomain::Source,
        Some(&admitted.wit_identity),
    );
    validate_identity(
        &mut diagnostics,
        "descriptor.component_identity",
        &descriptor.component_identity,
        IdentityDomain::TargetArtifact,
        None,
    );
    validate_identity(
        &mut diagnostics,
        "descriptor.runtime_profile_identity",
        &descriptor.runtime_profile_identity,
        IdentityDomain::Evaluator,
        Some(&admitted.runtime_profile_identity),
    );
    validate_encoded_identity(
        &mut diagnostics,
        "descriptor.toolchain_cohort_identity",
        &descriptor.toolchain_cohort_identity,
        IdentityDomain::Compiler,
        &runtime.toolchain,
    );
    validate_encoded_identity(
        &mut diagnostics,
        "descriptor.resource_profile_identity",
        &descriptor.resource_profile_identity,
        IdentityDomain::Adapter,
        &runtime.resources,
    );
    validate_schema_ref(
        &mut diagnostics,
        "descriptor.input_schema",
        &descriptor.input_schema,
    );
    validate_schema_ref(
        &mut diagnostics,
        "descriptor.output_schema",
        &descriptor.output_schema,
    );
    if descriptor.exports != [WASM_COMPONENT_INVOKE_EXPORT] {
        diagnostics.push(WasmComponentDiagnostic::new(
            "unsupported-export",
            "descriptor.exports",
            "descriptor must export exactly the reviewed invoke function",
        ));
    }
    validate_import_bindings(runtime, descriptor, &mut diagnostics);
    if descriptor.semantic_claims.is_empty() {
        compare_string_set(
            &mut diagnostics,
            "descriptor.non_claims",
            &descriptor.non_claims,
            WASM_COMPONENT_NON_CLAIMS,
        );
    } else {
        diagnostics.push(WasmComponentDiagnostic::new(
            "semantic-overclaim",
            "descriptor.semantic_claims",
            "Wasm descriptor cannot claim semantic equivalence or behavioral correctness",
        ));
    }
    validate_parent_edges(descriptor, &mut diagnostics);
    diagnostics
}

fn validate_schema_ref(
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
    field: &'static str,
    schema: &WasmComponentSchemaRef,
) {
    validate_identity(
        diagnostics,
        field,
        &schema.identity,
        IdentityDomain::Adapter,
        None,
    );
    if schema.schema_version.trim().is_empty() || schema.root_label.trim().is_empty() {
        diagnostics.push(WasmComponentDiagnostic::new(
            "missing-field",
            field,
            "canonical schema version and root label must be nonempty",
        ));
    }
}

fn validate_import_bindings(
    runtime: &WasmComponentRuntimeProfile,
    descriptor: &WasmComponentDescriptor,
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
) {
    let imports = descriptor
        .imports
        .iter()
        .map(|binding| binding.import.clone())
        .collect::<Vec<_>>();
    if sorted_unique(&imports) != imports {
        diagnostics.push(WasmComponentDiagnostic::new(
            "unordered-collection",
            "descriptor.imports",
            "component imports must be sorted and unique",
        ));
    }
    if imports != runtime.allowed_imports {
        diagnostics.push(WasmComponentDiagnostic::new(
            "undeclared-import",
            "descriptor.imports",
            "component imports differ from the admitted runtime allowlist",
        ));
    }
    for binding in &descriptor.imports {
        for (field, identity, domain) in [
            (
                "descriptor.imports.capability_identity",
                &binding.capability_identity,
                IdentityDomain::AlgebraicEffect,
            ),
            (
                "descriptor.imports.effect_operation_identity",
                &binding.effect_operation_identity,
                IdentityDomain::EffectOperation,
            ),
            (
                "descriptor.imports.handler_profile_identity",
                &binding.handler_profile_identity,
                IdentityDomain::Adapter,
            ),
            (
                "descriptor.imports.policy_identity",
                &binding.policy_identity,
                IdentityDomain::Adapter,
            ),
            (
                "descriptor.imports.authority_identity",
                &binding.authority_identity,
                IdentityDomain::Adapter,
            ),
            (
                "descriptor.imports.resource_identity",
                &binding.resource_identity,
                IdentityDomain::Adapter,
            ),
        ] {
            validate_identity(diagnostics, field, identity, domain, None);
        }
        if binding.import.starts_with("wasi:") {
            diagnostics.push(WasmComponentDiagnostic::new(
                "ambient-authority",
                "descriptor.imports",
                "ambient WASI imports are denied by the initial profile",
            ));
        }
    }
}

fn validate_parent_edges(
    descriptor: &WasmComponentDescriptor,
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
) {
    let mut expected = vec![
        WasmComponentParentEdge {
            role: WasmComponentParentRole::Component,
            identity: descriptor.component_identity.clone(),
        },
        WasmComponentParentEdge {
            role: WasmComponentParentRole::InputSchema,
            identity: descriptor.input_schema.identity.clone(),
        },
        WasmComponentParentEdge {
            role: WasmComponentParentRole::OutputSchema,
            identity: descriptor.output_schema.identity.clone(),
        },
        WasmComponentParentEdge {
            role: WasmComponentParentRole::ResourceProfile,
            identity: descriptor.resource_profile_identity.clone(),
        },
        WasmComponentParentEdge {
            role: WasmComponentParentRole::RuntimeProfile,
            identity: descriptor.runtime_profile_identity.clone(),
        },
        WasmComponentParentEdge {
            role: WasmComponentParentRole::ToolchainCohort,
            identity: descriptor.toolchain_cohort_identity.clone(),
        },
        WasmComponentParentEdge {
            role: WasmComponentParentRole::WitSource,
            identity: descriptor.wit_source_identity.clone(),
        },
    ];
    if let Some(identity) = &descriptor.materialization_bundle_identity {
        validate_identity(
            diagnostics,
            "descriptor.materialization_bundle_identity",
            identity,
            IdentityDomain::DependencyClosure,
            None,
        );
        expected.push(WasmComponentParentEdge {
            role: WasmComponentParentRole::MaterializationBundle,
            identity: identity.clone(),
        });
    }
    expected.sort();
    let actual_is_sorted_unique =
        sorted_unique(&descriptor.parent_edges) == descriptor.parent_edges;
    let mut actual = descriptor.parent_edges.clone();
    actual.sort();
    if actual != expected || !actual_is_sorted_unique {
        diagnostics.push(WasmComponentDiagnostic::new(
            "parent-edge-mismatch",
            "descriptor.parent_edges",
            "descriptor parent edges do not exactly cover each typed layer",
        ));
    }
}

fn validate_canonical_input(
    profile: &WasmComponentProfile,
    descriptor: &WasmComponentDescriptor,
    input: &[u8],
) -> Result<Identity, WasmComponentDiagnostic> {
    validate_canonical_value(
        profile,
        &descriptor.input_schema,
        input,
        "canonical_input",
        "invalid-canonical-input",
    )
}

pub fn validate_wasm_component_canonical_output(
    request: &WasmComponentStaticRequest<'_>,
    output: &[u8],
) -> Result<Identity, WasmComponentValidationError> {
    validate_wasm_component_static(request)?;
    let profile =
        serde_json::from_slice::<WasmComponentProfile>(request.profile_bytes).map_err(|error| {
            single_error(WasmComponentDiagnostic::new(
                "decode",
                "profile",
                error.to_string(),
            ))
        })?;
    validate_canonical_value(
        &profile,
        &request.descriptor.output_schema,
        output,
        "canonical_output",
        "invalid-canonical-output",
    )
    .map_err(single_error)
}

fn validate_canonical_value(
    profile: &WasmComponentProfile,
    schema: &WasmComponentSchemaRef,
    bytes: &[u8],
    field: &'static str,
    error_code: &'static str,
) -> Result<Identity, WasmComponentDiagnostic> {
    let mut limits = PreservesLimits::default();
    let byte_limit = usize::try_from(profile.runtime.profile.resources.max_hostcall_bytes)
        .map_err(|error| WasmComponentDiagnostic::new("limit", field, error.to_string()))?;
    limits.max_input_bytes = byte_limit;
    limits.max_output_bytes = byte_limit;
    let artifact = canonicalize_preserves(
        bytes,
        PreservesSyntax::Packed,
        PreservesSchema {
            schema_version: &schema.schema_version,
            root_label: &schema.root_label,
        },
        limits,
    )
    .map_err(|error| WasmComponentDiagnostic::new(error_code, field, error.to_string()))?;
    Ok(compute_identity(
        IdentityDomain::CanonicalValue,
        &artifact.packed_bytes,
    ))
}

fn parse_materialization(
    bytes: &[u8],
) -> Result<WasmComponentMaterializationBundle, WasmComponentValidationError> {
    serde_json::from_slice(bytes).map_err(|error| {
        single_error(WasmComponentDiagnostic::new(
            "decode",
            "materialization_bundle",
            error.to_string(),
        ))
    })
}

fn validate_materialization(
    admitted: &AdmittedProfile,
    descriptor: &WasmComponentDescriptor,
    bundle: &WasmComponentMaterializationBundle,
    bundle_identity: &Identity,
    component_bytes: Option<&[u8]>,
) -> Vec<WasmComponentDiagnostic> {
    let mut diagnostics = Vec::new();
    check_equal(
        &mut diagnostics,
        "materialization.schema_version",
        bundle.schema_version.as_str(),
        WASM_COMPONENT_MATERIALIZATION_SCHEMA,
    );
    check_equal(
        &mut diagnostics,
        "materialization.producer",
        bundle.producer.as_str(),
        WASM_COMPONENT_PRODUCER,
    );
    validate_identity(
        &mut diagnostics,
        "materialization.component_identity",
        &bundle.component_identity,
        IdentityDomain::TargetArtifact,
        Some(&descriptor.component_identity),
    );
    validate_identity(
        &mut diagnostics,
        "materialization.wit_identity",
        &bundle.wit_identity,
        IdentityDomain::Source,
        Some(&admitted.wit_identity),
    );
    validate_identity(
        &mut diagnostics,
        "materialization.runtime_profile_identity",
        &bundle.runtime_profile_identity,
        IdentityDomain::Evaluator,
        Some(&admitted.runtime_profile_identity),
    );
    validate_identity(
        &mut diagnostics,
        "materialization.toolchain_cohort_identity",
        &bundle.toolchain_cohort_identity,
        IdentityDomain::Compiler,
        Some(&descriptor.toolchain_cohort_identity),
    );
    validate_identity(
        &mut diagnostics,
        "materialization.octet_report_identity",
        &bundle.octet_report_identity,
        IdentityDomain::Receipt,
        None,
    );
    if bundle.stage_receipt_identities.is_empty()
        || sorted_unique(&bundle.stage_receipt_identities) != bundle.stage_receipt_identities
    {
        diagnostics.push(WasmComponentDiagnostic::new(
            "evidence-collection",
            "materialization.stage_receipt_identities",
            "Mantle stage receipt identities must be nonempty, sorted, and unique",
        ));
    }
    for identity in &bundle.stage_receipt_identities {
        validate_identity(
            &mut diagnostics,
            "materialization.stage_receipt_identities",
            identity,
            IdentityDomain::Receipt,
            None,
        );
    }
    if !bundle.portable_component || bundle.contains_precompiled_bytes {
        diagnostics.push(WasmComponentDiagnostic::new(
            "unsupported-artifact",
            "materialization.portable_component",
            "initial cohort requires portable component bytes and rejects precompiled bytes",
        ));
    }
    if descriptor.materialization_bundle_identity.as_ref() != Some(bundle_identity) {
        diagnostics.push(WasmComponentDiagnostic::new(
            "stale-identity",
            "descriptor.materialization_bundle_identity",
            "descriptor does not bind the exact materialization manifest bytes",
        ));
    }
    if let Some(component_bytes) = component_bytes {
        let measured_identity = compute_identity(IdentityDomain::TargetArtifact, component_bytes);
        let measured_length = u64::try_from(component_bytes.len());
        if measured_identity != bundle.component_identity
            || measured_length != Ok(bundle.component_byte_length)
        {
            diagnostics.push(WasmComponentDiagnostic::new(
                "stale-identity",
                "component_bytes",
                "component bytes differ from the materialization identity or length",
            ));
        }
        if measured_length
            .map(|length| {
                length
                    > admitted
                        .profile
                        .runtime
                        .profile
                        .resources
                        .max_component_bytes
            })
            .unwrap_or(true)
        {
            diagnostics.push(WasmComponentDiagnostic::new(
                "limit",
                "component_bytes",
                "component bytes exceed the admitted profile bound",
            ));
        }
    }
    if bundle.wit_byte_length != admitted.wit_byte_length {
        diagnostics.push(WasmComponentDiagnostic::new(
            "stale-identity",
            "materialization.wit_byte_length",
            "WIT byte length differs from the admitted source bytes",
        ));
    }
    diagnostics
}

fn validate_inspection(
    admitted: &AdmittedProfile,
    descriptor: &WasmComponentDescriptor,
    bundle: &WasmComponentMaterializationBundle,
    component_bytes: &[u8],
    inspection: &WasmComponentInspection,
) -> Vec<WasmComponentDiagnostic> {
    let mut diagnostics = Vec::new();
    let measured = compute_identity(IdentityDomain::TargetArtifact, component_bytes);
    validate_identity(
        &mut diagnostics,
        "inspection.component_identity",
        &inspection.component_identity,
        IdentityDomain::TargetArtifact,
        Some(&measured),
    );
    if inspection.component_identity != bundle.component_identity
        || inspection.component_identity != descriptor.component_identity
    {
        diagnostics.push(WasmComponentDiagnostic::new(
            "stale-identity",
            "inspection.component_identity",
            "inspection is bound to a different component",
        ));
    }
    check_equal(
        &mut diagnostics,
        "inspection.artifact_kind",
        inspection.artifact_kind.as_str(),
        WASM_COMPONENT_ARTIFACT_KIND,
    );
    check_equal(
        &mut diagnostics,
        "inspection.wit_world",
        inspection.wit_world.as_str(),
        WASM_COMPONENT_WIT_WORLD,
    );
    let expected_imports = descriptor
        .imports
        .iter()
        .map(|binding| binding.import.clone())
        .collect::<Vec<_>>();
    if inspection.imports != expected_imports
        || sorted_unique(&inspection.imports) != inspection.imports
    {
        diagnostics.push(WasmComponentDiagnostic::new(
            "import-mismatch",
            "inspection.imports",
            "inspected imports differ from the descriptor or are not sorted",
        ));
    }
    if inspection.exports != descriptor.exports
        || sorted_unique(&inspection.exports) != inspection.exports
    {
        diagnostics.push(WasmComponentDiagnostic::new(
            "export-mismatch",
            "inspection.exports",
            "inspected exports differ from the descriptor or are not sorted",
        ));
    }
    compare_string_set(
        &mut diagnostics,
        "inspection.enabled_features",
        &inspection.enabled_features,
        WASM_COMPONENT_REQUIRED_FEATURES,
    );
    let resources = &admitted.profile.runtime.profile.resources;
    if !inspection.fixed_memory
        || inspection.initial_memory_bytes != inspection.maximum_memory_bytes
        || inspection.maximum_memory_bytes > resources.max_memory_bytes
    {
        diagnostics.push(WasmComponentDiagnostic::new(
            "resource-profile",
            "inspection.memory",
            "component memory must be fixed and within the admitted bound",
        ));
    }
    if !inspection.fixed_table
        || inspection.initial_table_elements != inspection.maximum_table_elements
        || inspection.maximum_table_elements > resources.max_table_elements
    {
        diagnostics.push(WasmComponentDiagnostic::new(
            "resource-profile",
            "inspection.table",
            "component table must be fixed and within the admitted bound",
        ));
    }
    for (field, actual, maximum) in [
        (
            "inspection.instances",
            inspection.instances,
            resources.max_instances,
        ),
        (
            "inspection.memories",
            inspection.memories,
            resources.max_memories,
        ),
        ("inspection.tables", inspection.tables, resources.max_tables),
    ] {
        if actual > maximum {
            diagnostics.push(WasmComponentDiagnostic::new(
                "limit",
                field,
                "component resource count exceeds the admitted bound",
            ));
        }
    }
    diagnostics
}

fn serialized_identity<T: Serialize>(
    domain: IdentityDomain,
    field: &'static str,
    value: &T,
) -> Result<Identity, WasmComponentValidationError> {
    let bytes = serde_json::to_vec(value).map_err(|error| {
        single_error(WasmComponentDiagnostic::new(
            "encoding",
            field,
            error.to_string(),
        ))
    })?;
    Ok(compute_identity(domain, &bytes))
}

fn validate_encoded_identity<T: Serialize>(
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
    field: &'static str,
    identity: &Identity,
    domain: IdentityDomain,
    value: &T,
) {
    match serde_json::to_vec(value) {
        Ok(bytes) => {
            let expected = compute_identity(domain.clone(), &bytes);
            validate_identity(diagnostics, field, identity, domain, Some(&expected));
        }
        Err(error) => diagnostics.push(WasmComponentDiagnostic::new(
            "encoding",
            field,
            error.to_string(),
        )),
    }
}

fn validate_identity(
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
    field: &'static str,
    identity: &Identity,
    domain: IdentityDomain,
    expected: Option<&Identity>,
) {
    if let Err(error) = identity.validate_domain(domain) {
        diagnostics.push(WasmComponentDiagnostic::new(
            "identity-domain",
            field,
            error.to_string(),
        ));
    }
    if expected.is_some_and(|expected| identity != expected) {
        diagnostics.push(WasmComponentDiagnostic::new(
            "stale-identity",
            field,
            "identity does not match the admitted bytes or parent",
        ));
    }
}

fn check_equal(
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
    field: &'static str,
    actual: &str,
    expected: &str,
) {
    if actual != expected {
        diagnostics.push(WasmComponentDiagnostic::new(
            "unsupported-profile",
            field,
            format!("expected {expected:?}, got {actual:?}"),
        ));
    }
}

fn compare_string_set(
    diagnostics: &mut Vec<WasmComponentDiagnostic>,
    field: &'static str,
    actual: &[String],
    expected: &[&str],
) {
    let expected = expected
        .iter()
        .map(|value| (*value).to_string())
        .collect::<Vec<_>>();
    if actual != expected {
        diagnostics.push(WasmComponentDiagnostic::new(
            "profile-mismatch",
            field,
            "values differ from the reviewed ordered set",
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

fn validation_error(diagnostics: Vec<WasmComponentDiagnostic>) -> WasmComponentValidationError {
    let diagnostics = diagnostics
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    WasmComponentValidationError { diagnostics }
}

fn single_error(diagnostic: WasmComponentDiagnostic) -> WasmComponentValidationError {
    WasmComponentValidationError {
        diagnostics: vec![diagnostic],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PreservesSchema, encode_typed_preserves};

    const PROFILE_BYTES: &[u8] =
        include_bytes!("../../../adapters/wasm-component/generated/profile.json");
    const RUNTIME_PROFILE_BYTES: &[u8] =
        include_bytes!("../../../adapters/wasm-component/upstream/molten-profile.json");
    const WIT_BYTES: &[u8] =
        include_bytes!("../../../adapters/wasm-component/upstream/runtime.wit");
    const TEST_SCHEMA_VERSION: &str = "kamacite.wasm-component-test-value.v1";
    const TEST_ROOT_LABEL: &str = "TestValue";

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct TestValue {
        message: String,
    }

    fn runtime_profile() -> Result<WasmComponentRuntimeProfileExport, serde_json::Error> {
        serde_json::from_slice(RUNTIME_PROFILE_BYTES)
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

    fn schema_ref(label: &[u8]) -> WasmComponentSchemaRef {
        WasmComponentSchemaRef {
            identity: compute_identity(IdentityDomain::Adapter, label),
            schema_version: TEST_SCHEMA_VERSION.to_string(),
            root_label: TEST_ROOT_LABEL.to_string(),
        }
    }

    fn descriptor(
        materialization: Option<Identity>,
    ) -> Result<WasmComponentDescriptor, Box<dyn std::error::Error>> {
        let profile = runtime_profile()?.profile;
        let toolchain = serde_json::to_vec(&profile.toolchain)?;
        let resources = serde_json::to_vec(&profile.resources)?;
        let mut descriptor = WasmComponentDescriptor {
            schema_version: WASM_COMPONENT_DESCRIPTOR_SCHEMA.to_string(),
            adapter_id: "fixture.identity".to_string(),
            adapter_version: "v1".to_string(),
            artifact_kind: WASM_COMPONENT_ARTIFACT_KIND.to_string(),
            wit_package: WASM_COMPONENT_WIT_PACKAGE.to_string(),
            wit_world: WASM_COMPONENT_WIT_WORLD.to_string(),
            wit_source_identity: compute_identity(IdentityDomain::Source, WIT_BYTES),
            component_identity: compute_identity(IdentityDomain::TargetArtifact, b"component"),
            runtime_profile_identity: compute_identity(
                IdentityDomain::Evaluator,
                RUNTIME_PROFILE_BYTES,
            ),
            toolchain_cohort_identity: compute_identity(IdentityDomain::Compiler, &toolchain),
            resource_profile_identity: compute_identity(IdentityDomain::Adapter, &resources),
            input_schema: schema_ref(b"input-schema"),
            output_schema: schema_ref(b"output-schema"),
            imports: Vec::new(),
            exports: vec![WASM_COMPONENT_INVOKE_EXPORT.to_string()],
            materialization_bundle_identity: materialization,
            parent_edges: Vec::new(),
            semantic_claims: Vec::new(),
            non_claims: string_values(WASM_COMPONENT_NON_CLAIMS),
        };
        descriptor.parent_edges = expected_edges(&descriptor);
        Ok(descriptor)
    }

    fn expected_edges(descriptor: &WasmComponentDescriptor) -> Vec<WasmComponentParentEdge> {
        let mut edges = vec![
            edge(
                WasmComponentParentRole::Component,
                &descriptor.component_identity,
            ),
            edge(
                WasmComponentParentRole::InputSchema,
                &descriptor.input_schema.identity,
            ),
            edge(
                WasmComponentParentRole::OutputSchema,
                &descriptor.output_schema.identity,
            ),
            edge(
                WasmComponentParentRole::ResourceProfile,
                &descriptor.resource_profile_identity,
            ),
            edge(
                WasmComponentParentRole::RuntimeProfile,
                &descriptor.runtime_profile_identity,
            ),
            edge(
                WasmComponentParentRole::ToolchainCohort,
                &descriptor.toolchain_cohort_identity,
            ),
            edge(
                WasmComponentParentRole::WitSource,
                &descriptor.wit_source_identity,
            ),
        ];
        if let Some(identity) = &descriptor.materialization_bundle_identity {
            edges.push(edge(
                WasmComponentParentRole::MaterializationBundle,
                identity,
            ));
        }
        edges.sort();
        edges
    }

    fn edge(role: WasmComponentParentRole, identity: &Identity) -> WasmComponentParentEdge {
        WasmComponentParentEdge {
            role,
            identity: identity.clone(),
        }
    }

    fn request<'a>(
        descriptor: &'a WasmComponentDescriptor,
        input: &'a [u8],
    ) -> WasmComponentStaticRequest<'a> {
        WasmComponentStaticRequest {
            profile_bytes: PROFILE_BYTES,
            runtime_profile_bytes: RUNTIME_PROFILE_BYTES,
            wit_bytes: WIT_BYTES,
            descriptor,
            canonical_input_bytes: input,
            materialization_manifest_bytes: None,
            component_bytes: None,
            inspection: None,
        }
    }

    // r[verify kamacite.wasm_adapter.boundary]
    // r[verify kamacite.wasm_adapter.wit_identity]
    // r[verify kamacite.wasm_adapter.profile]
    // r[verify kamacite.wasm_adapter.functional_core]
    // r[verify kamacite.wasm_adapter.receipt]
    // r[verify kamacite.wasm_adapter.valence_export]
    #[test]
    fn exact_profile_descriptor_and_canonical_input_validate_without_execution()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = canonical_input()?;
        let descriptor = descriptor(None)?;
        let exact_request = request(&descriptor, &input);
        let result = validate_wasm_component_static(&exact_request);
        assert!(result.is_ok(), "{result:?}");
        if let Ok(result) = result {
            assert_eq!(result.state, WasmComponentPlanningState::StaticOnly);
            assert_eq!(
                result.blockers,
                vec![WasmComponentBlocker::MissingMaterializationBundle]
            );
            assert!(result.execution_plan.is_none());
            assert!(result.execution_plan_identity.is_none());
            assert_eq!(result.wit_identity.domain, IdentityDomain::Source);
            assert_eq!(
                result.runtime_profile_identity.domain,
                IdentityDomain::Evaluator
            );
            let receipt = build_wasm_component_static_receipt(&exact_request)?;
            assert_eq!(receipt.receipt_identity.domain, IdentityDomain::Receipt);
            assert!(validate_wasm_component_static_receipt(&exact_request, &receipt).is_ok());
            let projection = crate::project_wasm_component_static_to_valence(&exact_request)?;
            let repeated_projection =
                crate::project_wasm_component_static_to_valence(&exact_request)?;
            assert_eq!(projection, repeated_projection);
            assert_eq!(
                projection.projection_identity.domain,
                IdentityDomain::Projection
            );
            assert!(projection.body.parent_edges.len() > descriptor.parent_edges.len());
            let wit_artifact = projection
                .body
                .artifacts
                .iter()
                .find(|artifact| artifact.role == "wit-source");
            assert!(wit_artifact.is_some());
            if let Some(wit_artifact) = wit_artifact {
                assert!(matches!(
                    wit_artifact.spans,
                    crate::SourceSpanSet::Present { .. }
                ));
            }
            let output_identity = validate_wasm_component_canonical_output(&exact_request, &input)?;
            assert_eq!(output_identity.domain, IdentityDomain::CanonicalValue);
            assert!(
                validate_wasm_component_canonical_output(&exact_request, b"malformed").is_err()
            );
            let mut stale = receipt;
            stale.body.source_revision = "substituted".to_string();
            assert!(validate_wasm_component_static_receipt(&exact_request, &stale).is_err());
        }
        Ok(())
    }

    // r[verify kamacite.wasm_adapter.wit_identity]
    // r[verify kamacite.wasm_adapter.fixtures]
    #[test]
    fn profile_wit_runtime_and_descriptor_drift_fail_closed()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = canonical_input()?;
        let base = descriptor(None)?;
        let mut stale_wit = request(&base, &input);
        stale_wit.wit_bytes = b"stale WIT";
        assert!(validate_wasm_component_static(&stale_wit).is_err());

        let mut stale_runtime = request(&base, &input);
        stale_runtime.runtime_profile_bytes = br#"{}"#;
        assert!(validate_wasm_component_static(&stale_runtime).is_err());

        let mut orphan_component = request(&base, &input);
        orphan_component.component_bytes = Some(b"orphan component");
        assert!(validate_wasm_component_static(&orphan_component).is_err());

        let mut wrong_world = base.clone();
        wrong_world.wit_world = "other".to_string();
        assert!(validate_wasm_component_static(&request(&wrong_world, &input)).is_err());

        let mut overclaim = base;
        overclaim
            .semantic_claims
            .push("arbitrary Wasm semantic equivalence".to_string());
        assert!(validate_wasm_component_static(&request(&overclaim, &input)).is_err());
        Ok(())
    }

    // r[verify kamacite.wasm_adapter.preserves_io]
    // r[verify kamacite.wasm_adapter.capabilities]
    // r[verify kamacite.wasm_adapter.fixtures]
    #[test]
    fn noncanonical_input_undeclared_import_and_parent_confusion_are_rejected()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = canonical_input()?;
        let base = descriptor(None)?;
        let malformed = validate_wasm_component_static(&request(&base, b"not preserves"));
        assert!(malformed.is_err());

        let mut import = base.clone();
        import.imports.push(WasmComponentImportBinding {
            import: "wasi:filesystem/types".to_string(),
            capability_identity: compute_identity(IdentityDomain::AlgebraicEffect, b"capability"),
            effect_operation_identity: compute_identity(
                IdentityDomain::EffectOperation,
                b"operation",
            ),
            handler_profile_identity: compute_identity(IdentityDomain::Adapter, b"handler"),
            policy_identity: compute_identity(IdentityDomain::Adapter, b"policy"),
            authority_identity: compute_identity(IdentityDomain::Adapter, b"authority"),
            resource_identity: compute_identity(IdentityDomain::Adapter, b"resource"),
            replay_class: WasmComponentReplayClass::RecordedEffectRequired,
        });
        assert!(validate_wasm_component_static(&request(&import, &input)).is_err());

        let mut parent = base;
        parent.parent_edges.pop();
        assert!(validate_wasm_component_static(&request(&parent, &input)).is_err());
        Ok(())
    }

    #[test]
    fn test_only_materialization_never_yields_an_executable_plan()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = canonical_input()?;
        let component_bytes = b"test-only component bytes";
        let component_identity = compute_identity(IdentityDomain::TargetArtifact, component_bytes);
        let mut descriptor = descriptor(None)?;
        descriptor.component_identity = component_identity.clone();
        descriptor.parent_edges = expected_edges(&descriptor);
        let profile = runtime_profile()?.profile;
        let toolchain = serde_json::to_vec(&profile.toolchain)?;
        let bundle = WasmComponentMaterializationBundle {
            schema_version: WASM_COMPONENT_MATERIALIZATION_SCHEMA.to_string(),
            producer: WASM_COMPONENT_PRODUCER.to_string(),
            evidence_scope: WasmComponentEvidenceScope::TestOnly,
            component_identity: component_identity.clone(),
            component_byte_length: u64::try_from(component_bytes.len())?,
            wit_identity: compute_identity(IdentityDomain::Source, WIT_BYTES),
            wit_byte_length: u64::try_from(WIT_BYTES.len())?,
            runtime_profile_identity: compute_identity(
                IdentityDomain::Evaluator,
                RUNTIME_PROFILE_BYTES,
            ),
            toolchain_cohort_identity: compute_identity(IdentityDomain::Compiler, &toolchain),
            octet_report_identity: compute_identity(IdentityDomain::Receipt, b"octet report"),
            stage_receipt_identities: vec![compute_identity(
                IdentityDomain::Receipt,
                b"mantle stage",
            )],
            portable_component: true,
            contains_precompiled_bytes: false,
        };
        let bundle_bytes = serde_json::to_vec(&bundle)?;
        let bundle_identity = compute_identity(IdentityDomain::DependencyClosure, &bundle_bytes);
        descriptor.materialization_bundle_identity = Some(bundle_identity);
        descriptor.parent_edges = expected_edges(&descriptor);

        let inspection = WasmComponentInspection {
            artifact_kind: WASM_COMPONENT_ARTIFACT_KIND.to_string(),
            component_identity,
            wit_world: WASM_COMPONENT_WIT_WORLD.to_string(),
            imports: Vec::new(),
            exports: vec![WASM_COMPONENT_INVOKE_EXPORT.to_string()],
            enabled_features: string_values(WASM_COMPONENT_REQUIRED_FEATURES),
            fixed_memory: true,
            initial_memory_bytes: 1,
            maximum_memory_bytes: 1,
            fixed_table: true,
            initial_table_elements: 0,
            maximum_table_elements: 0,
            instances: 1,
            memories: 1,
            tables: 0,
        };
        let request = WasmComponentStaticRequest {
            profile_bytes: PROFILE_BYTES,
            runtime_profile_bytes: RUNTIME_PROFILE_BYTES,
            wit_bytes: WIT_BYTES,
            descriptor: &descriptor,
            canonical_input_bytes: &input,
            materialization_manifest_bytes: Some(&bundle_bytes),
            component_bytes: Some(component_bytes),
            inspection: Some(&inspection),
        };
        let result = validate_wasm_component_static(&request);
        assert!(result.is_ok(), "{result:?}");
        if let Ok(result) = result {
            assert_eq!(result.state, WasmComponentPlanningState::StaticOnly);
            assert_eq!(
                result.blockers,
                vec![WasmComponentBlocker::TestOnlyMaterialization]
            );
            assert!(result.execution_plan.is_none());
            assert!(result.component_identity.is_some());
            assert!(result.inspection_identity.is_some());
        }
        let test_receipt = build_wasm_component_static_receipt(&request)?;
        assert!(test_receipt.body.component_identity.is_some());
        assert!(test_receipt.body.inspection_identity.is_some());
        let test_projection = crate::project_wasm_component_static_to_valence(&request)?;
        assert!(
            test_projection
                .body
                .artifacts
                .iter()
                .any(|artifact| artifact.role == "portable-component")
        );
        assert!(
            test_projection
                .body
                .artifacts
                .iter()
                .any(|artifact| artifact.role == "component-inspection")
        );

        let mut production_bundle = bundle;
        production_bundle.evidence_scope = WasmComponentEvidenceScope::Production;
        let production_bytes = serde_json::to_vec(&production_bundle)?;
        let production_identity =
            compute_identity(IdentityDomain::DependencyClosure, &production_bytes);
        let mut production_descriptor = descriptor;
        production_descriptor.materialization_bundle_identity = Some(production_identity);
        production_descriptor.parent_edges = expected_edges(&production_descriptor);
        let production_request = WasmComponentStaticRequest {
            profile_bytes: PROFILE_BYTES,
            runtime_profile_bytes: RUNTIME_PROFILE_BYTES,
            wit_bytes: WIT_BYTES,
            descriptor: &production_descriptor,
            canonical_input_bytes: &input,
            materialization_manifest_bytes: Some(&production_bytes),
            component_bytes: Some(component_bytes),
            inspection: Some(&inspection),
        };
        let production_result = validate_wasm_component_static(&production_request);
        assert!(production_result.is_ok(), "{production_result:?}");
        if let Ok(production_result) = production_result {
            assert_eq!(
                production_result.state,
                WasmComponentPlanningState::StaticOnly
            );
            assert_eq!(
                production_result.blockers,
                vec![WasmComponentBlocker::ExternalMaterializationVerifierUnavailable]
            );
            assert!(production_result.execution_plan.is_none());
        }
        Ok(())
    }

    fn string_values(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }
}
