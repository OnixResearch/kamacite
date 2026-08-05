use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{
    AdapterIdentity, AnyIdentity, IdentityDomain, ReceiptIdentity, WireIdentity, compute_identity,
};
use crate::preserves_codec::{
    PRESERVES_CANONICAL_CODEC, PreservesLimits, PreservesSchema, encode_typed_preserves,
};
use crate::receipt::ReceiptError;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION: &str = "kamacite.function-address-receipt.v3";
pub const FUNCTION_ADDRESS_CANONICAL_CODEC: &str = PRESERVES_CANONICAL_CODEC;
pub const FUNCTION_ADDRESS_JSON_PROJECTION: &str =
    "kamacite.function-address-receipt.v3.compat-json";
pub const FUNCTION_ADDRESS_VALENCE_PROJECTION_SCHEMA_VERSION: &str =
    "kamacite.function-address-valence-projection.v3";
pub const FUNCTION_ADDRESS_PRODUCER_PROFILE_SCHEMA_VERSION: &str =
    "kamacite.function-address-producer-profile.v2";
pub const LEGACY_PSEUDO_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION: &str =
    "kamacite.function-address-receipt.v2";
pub const LEGACY_PSEUDO_FUNCTION_ADDRESS_CANONICAL_CODEC: &str =
    "kamacite-preserves-canonical-binary-function-address-receipt-v2";
pub const LEGACY_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION: &str =
    "kamacite.function-address-receipt.v1";
pub const LEGACY_FUNCTION_ADDRESS_CANONICAL_CODEC: &str =
    "kamacite-preserves-canonical-binary-function-address-receipt-v1";
pub const LEGACY_TOOL_VERSION_UNBOUND_CAVEAT: &str = "tool-version-unbound";
const FUNCTION_ADDRESS_ROOT_LABEL: &str = "kamacite-function-address-receipt-v3";
const FUNCTION_ADDRESS_PRODUCER_ROOT_LABEL: &str = "kamacite-function-address-producer-v2";
const REQUIREMENT_CONTRACT: &str = "r[kamacite.function_address_receipts.contract]";
const REQUIREMENT_VALIDATION: &str = "r[kamacite.function_address_receipts.validation]";
const REQUIREMENT_OCTET_INTEROP: &str = "r[kamacite.function_address_receipts.octet_interop]";
const REQUIREMENT_DEPENDENCY_KIND: &str =
    "r[kamacite.function_address_receipts.dependency_vocabulary.kind]";
const REQUIREMENT_DEPENDENCY_COMPAT: &str =
    "r[kamacite.function_address_receipts.dependency_vocabulary.compat]";
const REQUIREMENT_DEPENDENCY_OVERCLAIM: &str =
    "r[kamacite.function_address_receipts.dependency_vocabulary.negative]";
const REQUIREMENT_PRESERVES_SCHEMA: &str =
    "r[kamacite.function_address_receipts.preserves_authority.schema]";
const REQUIREMENT_PRESERVES_HASH: &str =
    "r[kamacite.function_address_receipts.preserves_authority.hash]";
const REQUIREMENT_JSON_PROJECTION: &str =
    "r[kamacite.function_address_receipts.preserves_authority.json_projection]";
const REQUIREMENT_PRODUCER_PROFILE: &str =
    "r[kamacite.findings.function_address_producer.profile_identity]";
const REQUIREMENT_PRODUCER_BINDING: &str =
    "r[kamacite.findings.function_address_producer.receipt_binding]";
const REQUIREMENT_PRODUCER_MIGRATION: &str =
    "r[kamacite.findings.function_address_producer.migration]";
const NON_CLAIM_HOST_LANGUAGE_SEMANTICS: &str = "not host-language semantic correctness";
const NON_CLAIM_COMPILER_CORRECTNESS: &str = "not compiler correctness";
const NON_CLAIM_RUNTIME_BEHAVIOR: &str = "not runtime behavior";
const NON_CLAIM_WHOLE_PROGRAM_SAFETY: &str = "not whole-program safety";
const NON_CLAIM_VERIFIER_SOUNDNESS: &str = "not verifier soundness";
const NON_CLAIM_RELEASE_ELIGIBILITY: &str = "not release eligibility";
const NON_CLAIM_PRODUCER_CORRECTNESS: &str = "not producer correctness";
const NON_CLAIM_SEMANTIC_DEPENDENCY_CLOSURE: &str = "not semantic dependency closure";
const REQUIRED_PRODUCER_NON_CLAIMS: &[&str] = &[
    NON_CLAIM_PRODUCER_CORRECTNESS,
    NON_CLAIM_SEMANTIC_DEPENDENCY_CLOSURE,
];
const REQUIRED_NON_CLAIMS: &[&str] = &[
    NON_CLAIM_HOST_LANGUAGE_SEMANTICS,
    NON_CLAIM_COMPILER_CORRECTNESS,
    NON_CLAIM_RUNTIME_BEHAVIOR,
    NON_CLAIM_WHOLE_PROGRAM_SAFETY,
    NON_CLAIM_VERIFIER_SOUNDNESS,
    NON_CLAIM_RELEASE_ELIGIBILITY,
];
const FORBIDDEN_CLAIM_FRAGMENTS: &[&str] = &[
    "semantic correctness",
    "compiler correctness",
    "runtime behavior",
    "whole-program safety",
    "verifier soundness",
    "release eligibility",
    "semantic equivalence",
    "source equivalence",
    "semantic dependency closure",
    "compiler-resolved call graph",
    "semantic call graph",
    "whole-program dependency closure",
];
const OCTET_PRODUCER: &str = "octet";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressProducer {
    pub name: String,
    pub canonicalizer: String,
    pub tool_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_identity: Option<WireIdentity>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub normative_options: BTreeMap<String, String>,
    #[serde(default)]
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressProducerProfile {
    pub schema_version: String,
    pub producer_kind: String,
    pub tool_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_identity: Option<WireIdentity>,
    pub canonicalizer_profile: String,
    pub normative_options: BTreeMap<String, String>,
    pub non_claims: Vec<String>,
    pub profile_identity: AdapterIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FunctionAddressReceiptTrustClass {
    CurrentProducerBound,
    LegacyPseudoCodec,
    LegacyToolVersionUnbound,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressSourceEnvelope {
    pub key: String,
    pub display_path: String,
    pub language: String,
    pub source_bytes: String,
    pub source_identity: WireIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressSpan {
    pub source_key: String,
    pub start_byte: u64,
    pub end_byte: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FunctionAddressDependencyIdentityKind {
    SourceTokenDependencySet,
    SourceShapeDependencySet,
}

impl FunctionAddressDependencyIdentityKind {
    fn label(&self) -> &'static str {
        match self {
            Self::SourceTokenDependencySet => "source-token-dependency-set",
            Self::SourceShapeDependencySet => "source-shape-dependency-set",
        }
    }

    fn expected_domain(&self) -> IdentityDomain {
        IdentityDomain::DependencyClosure
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressRecord {
    pub locator: String,
    pub display_name: String,
    pub source_key: String,
    pub span: Option<FunctionAddressSpan>,
    pub function_address: WireIdentity,
    pub dependency_closure: Option<WireIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency_identity_kind: Option<FunctionAddressDependencyIdentityKind>,
    pub caveats: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressDependencyEdge {
    pub from_locator: String,
    pub to_locator: String,
    pub dependency_identity: Option<WireIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency_identity_kind: Option<FunctionAddressDependencyIdentityKind>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressCaveat {
    pub locator: String,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressReceiptInput {
    pub producer: FunctionAddressProducer,
    pub source_envelopes: Vec<FunctionAddressSourceEnvelope>,
    pub functions: Vec<FunctionAddressRecord>,
    pub dependency_edges: Vec<FunctionAddressDependencyEdge>,
    pub caveats: Vec<FunctionAddressCaveat>,
    pub semantic_claims: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressReceiptBody {
    pub schema_version: String,
    pub canonical_codec: String,
    pub producer: FunctionAddressProducer,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_profile: Option<FunctionAddressProducerProfile>,
    pub source_envelopes: Vec<FunctionAddressSourceEnvelope>,
    pub functions: Vec<FunctionAddressRecord>,
    pub dependency_edges: Vec<FunctionAddressDependencyEdge>,
    pub caveats: Vec<FunctionAddressCaveat>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressReceiptEnvelope {
    pub body: FunctionAddressReceiptBody,
    pub receipt_identity: ReceiptIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionAddressReceiptResult {
    pub passed: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub receipt: Option<FunctionAddressReceiptEnvelope>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressJsonProjection {
    pub schema_version: String,
    pub compatibility_projection: String,
    pub canonical_identity_owner: String,
    pub receipt_identity: ReceiptIdentity,
    pub receipt: FunctionAddressReceiptBody,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressValenceProjection {
    pub schema_version: String,
    pub source_receipt_identity: ReceiptIdentity,
    pub producer: String,
    pub producer_profile_identity: AdapterIdentity,
    pub artifact_identities: Vec<FunctionAddressValenceArtifact>,
    pub dependency_edges: Vec<FunctionAddressDependencyEdge>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressValenceArtifact {
    pub role: String,
    pub key: String,
    pub identity: AnyIdentity,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct CanonicalSourceEnvelope {
    source_identity: WireIdentity,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct CanonicalFunctionRecord {
    source_identity: WireIdentity,
    function_address: WireIdentity,
    dependency_closure: Option<WireIdentity>,
    dependency_identity_kind: Option<FunctionAddressDependencyIdentityKind>,
    caveats: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-function-address-receipt-v3")]
struct CanonicalReceipt<'a> {
    schema_version: &'a str,
    canonical_codec: &'a str,
    producer_profile_identity: WireIdentity,
    source_envelopes: Vec<CanonicalSourceEnvelope>,
    functions: Vec<CanonicalFunctionRecord>,
    dependency_edges: Vec<CanonicalDependencyEdge>,
    caveats: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct CanonicalDependencyEdge {
    from_address: WireIdentity,
    to_address: WireIdentity,
    dependency_identity: Option<WireIdentity>,
    dependency_identity_kind: Option<FunctionAddressDependencyIdentityKind>,
}

#[derive(Serialize)]
#[serde(rename = "kamacite-function-address-producer-v2")]
struct CanonicalProducerProfile<'a> {
    schema_version: &'a str,
    producer_kind: &'a str,
    tool_version: &'a str,
    build_identity: &'a Option<WireIdentity>,
    canonicalizer_profile: &'a str,
    normative_options: &'a BTreeMap<String, String>,
    non_claims: Vec<String>,
}

pub fn function_address_producer_profile(
    producer: &FunctionAddressProducer,
) -> Result<FunctionAddressProducerProfile, Vec<Diagnostic>> {
    let diagnostics = validate_function_address_producer(producer);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    function_address_producer_profile_unchecked(producer).map_err(|error| {
        vec![Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_PRODUCER_PROFILE,
            "producer",
            format!("producer profile canonicalization failed: {error}"),
        )]
    })
}

fn function_address_producer_profile_unchecked(
    producer: &FunctionAddressProducer,
) -> Result<FunctionAddressProducerProfile, ReceiptError> {
    let canonical = canonical_producer_profile(producer);
    let profile_bytes = typed_function_address_bytes(
        &canonical,
        FUNCTION_ADDRESS_PRODUCER_PROFILE_SCHEMA_VERSION,
        FUNCTION_ADDRESS_PRODUCER_ROOT_LABEL,
    )?;
    let profile_identity = AdapterIdentity::compute(&profile_bytes);
    Ok(FunctionAddressProducerProfile {
        schema_version: FUNCTION_ADDRESS_PRODUCER_PROFILE_SCHEMA_VERSION.to_string(),
        producer_kind: producer.name.clone(),
        tool_version: producer.tool_version.clone(),
        build_identity: producer.build_identity.clone(),
        canonicalizer_profile: producer.canonicalizer.clone(),
        normative_options: producer.normative_options.clone(),
        non_claims: sorted_strings(&producer.non_claims),
        profile_identity,
    })
}

pub fn classify_function_address_receipt(
    body: &FunctionAddressReceiptBody,
) -> FunctionAddressReceiptTrustClass {
    if body.schema_version == FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION
        && body.canonical_codec == FUNCTION_ADDRESS_CANONICAL_CODEC
        && body.producer_profile.is_some()
    {
        return FunctionAddressReceiptTrustClass::CurrentProducerBound;
    }
    if body.schema_version == LEGACY_PSEUDO_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION
        && body.canonical_codec == LEGACY_PSEUDO_FUNCTION_ADDRESS_CANONICAL_CODEC
        && body.producer_profile.is_some()
    {
        return FunctionAddressReceiptTrustClass::LegacyPseudoCodec;
    }
    if body.schema_version == LEGACY_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION
        && body.canonical_codec == LEGACY_FUNCTION_ADDRESS_CANONICAL_CODEC
        && body
            .caveats
            .iter()
            .any(|caveat| caveat.reason == LEGACY_TOOL_VERSION_UNBOUND_CAVEAT)
    {
        return FunctionAddressReceiptTrustClass::LegacyToolVersionUnbound;
    }
    FunctionAddressReceiptTrustClass::Unsupported
}

fn canonical_producer_profile(producer: &FunctionAddressProducer) -> CanonicalProducerProfile<'_> {
    CanonicalProducerProfile {
        schema_version: FUNCTION_ADDRESS_PRODUCER_PROFILE_SCHEMA_VERSION,
        producer_kind: &producer.name,
        tool_version: &producer.tool_version,
        build_identity: &producer.build_identity,
        canonicalizer_profile: &producer.canonicalizer,
        normative_options: &producer.normative_options,
        non_claims: sorted_strings(&producer.non_claims),
    }
}

pub fn validate_function_address_receipt(
    input: &FunctionAddressReceiptInput,
) -> FunctionAddressReceiptResult {
    let mut diagnostics = validate_function_address_producer(&input.producer);
    validate_non_claims(&input.non_claims, &mut diagnostics);
    validate_semantic_claims(&input.semantic_claims, &mut diagnostics);
    let source_identities = validate_sources(&input.source_envelopes, &mut diagnostics);
    let source_lengths = source_byte_lengths(&input.source_envelopes, &mut diagnostics);
    let mut dependency_kinds = BTreeMap::new();
    let function_addresses = validate_functions(
        input,
        &source_identities,
        &source_lengths,
        &mut dependency_kinds,
        &mut diagnostics,
    );
    validate_dependency_edges(
        input,
        &function_addresses,
        &mut dependency_kinds,
        &mut diagnostics,
    );
    validate_caveats(input, &function_addresses, &mut diagnostics);

    diagnostics.sort();
    diagnostics.dedup();
    if !diagnostics.is_empty() {
        return FunctionAddressReceiptResult {
            passed: false,
            diagnostics,
            receipt: None,
        };
    }

    match build_receipt(input, &source_identities) {
        Ok(receipt) => FunctionAddressReceiptResult {
            passed: true,
            diagnostics,
            receipt: Some(receipt),
        },
        Err(error) => FunctionAddressReceiptResult {
            passed: false,
            diagnostics: vec![Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PRESERVES_HASH,
                "receipt",
                format!("function-address receipt canonicalization failed: {error}"),
            )],
            receipt: None,
        },
    }
}

pub fn function_address_canonical_bytes(
    input: &FunctionAddressReceiptInput,
) -> Result<Vec<u8>, ReceiptError> {
    let source_identities = source_identity_map(&input.source_envelopes);
    let canonical = canonical_receipt(input, &source_identities)?;
    typed_function_address_bytes(
        &canonical,
        FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION,
        FUNCTION_ADDRESS_ROOT_LABEL,
    )
}

fn typed_function_address_bytes<T: Serialize>(
    value: &T,
    schema_version: &str,
    root_label: &str,
) -> Result<Vec<u8>, ReceiptError> {
    encode_typed_preserves(
        value,
        PreservesSchema {
            schema_version,
            root_label,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ReceiptError::Serialize(error.to_string()))
}

pub fn function_address_json_projection(
    receipt: &FunctionAddressReceiptEnvelope,
) -> FunctionAddressJsonProjection {
    FunctionAddressJsonProjection {
        schema_version: FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION.to_string(),
        compatibility_projection: FUNCTION_ADDRESS_JSON_PROJECTION.to_string(),
        canonical_identity_owner: FUNCTION_ADDRESS_CANONICAL_CODEC.to_string(),
        receipt_identity: receipt.receipt_identity.clone(),
        receipt: receipt.body.clone(),
    }
}

pub fn function_address_projection_json(
    receipt: &FunctionAddressReceiptEnvelope,
) -> Result<String, ReceiptError> {
    let projection = function_address_json_projection(receipt);
    serde_json::to_string_pretty(&projection)
        .map_err(|error| ReceiptError::Serialize(error.to_string()))
}

pub fn validate_function_address_json_projection(
    projection: &FunctionAddressJsonProjection,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if projection.compatibility_projection != FUNCTION_ADDRESS_JSON_PROJECTION {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_JSON_PROJECTION,
                "compatibility_projection",
                "function-address JSON projection declares an unsupported projection kind",
            )
            .with_detail("actual", projection.compatibility_projection.clone())
            .with_detail("expected", FUNCTION_ADDRESS_JSON_PROJECTION),
        );
    }
    if projection.canonical_identity_owner != FUNCTION_ADDRESS_CANONICAL_CODEC {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_JSON_PROJECTION,
                "canonical_identity_owner",
                "function-address JSON projection must bind to canonical Preserves receipt identity",
            )
            .with_detail("actual", projection.canonical_identity_owner.clone())
            .with_detail("expected", FUNCTION_ADDRESS_CANONICAL_CODEC),
        );
    }
    match classify_function_address_receipt(&projection.receipt) {
        FunctionAddressReceiptTrustClass::CurrentProducerBound => {}
        FunctionAddressReceiptTrustClass::LegacyPseudoCodec => diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_PRODUCER_MIGRATION,
                "receipt.canonical_codec",
                "legacy function-address receipt used labeled deterministic JSON rather than canonical Preserves packed bytes",
            )
            .with_detail("legacy_codec", LEGACY_PSEUDO_FUNCTION_ADDRESS_CANONICAL_CODEC)
            .with_detail("current_codec", FUNCTION_ADDRESS_CANONICAL_CODEC),
        ),
        FunctionAddressReceiptTrustClass::LegacyToolVersionUnbound => diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedSchemaVersion,
                REQUIREMENT_PRODUCER_MIGRATION,
                "receipt.schema_version",
                "legacy function-address receipt is recorded-only because tool version was not bound",
            )
            .with_detail("caveat", LEGACY_TOOL_VERSION_UNBOUND_CAVEAT),
        ),
        FunctionAddressReceiptTrustClass::Unsupported => diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedSchemaVersion,
            REQUIREMENT_PRODUCER_MIGRATION,
            "receipt.schema_version",
            "function-address receipt does not satisfy the current producer-bound profile",
        )),
    }
    match (
        function_address_producer_profile(&projection.receipt.producer),
        projection.receipt.producer_profile.as_ref(),
    ) {
        (Ok(computed), Some(declared)) if &computed == declared => {}
        (Ok(computed), Some(declared)) => diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_PRODUCER_BINDING,
                "receipt.producer_profile",
                "displayed producer fields do not match the bound producer profile",
            )
            .with_detail("declared", declared.profile_identity.hex().as_str())
            .with_detail("computed", computed.profile_identity.hex().as_str()),
        ),
        (Ok(_), None) => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_PRODUCER_BINDING,
            "receipt.producer_profile",
            "current function-address receipt requires a bound producer profile",
        )),
        (Err(mut inner), _) => diagnostics.append(&mut inner),
    }
    let input = FunctionAddressReceiptInput {
        producer: projection.receipt.producer.clone(),
        source_envelopes: projection.receipt.source_envelopes.clone(),
        functions: projection.receipt.functions.clone(),
        dependency_edges: projection.receipt.dependency_edges.clone(),
        caveats: projection.receipt.caveats.clone(),
        semantic_claims: Vec::new(),
        non_claims: projection.receipt.non_claims.clone(),
    };
    let result = validate_function_address_receipt(&input);
    diagnostics.extend(result.diagnostics);
    match result.receipt {
        Some(receipt) if receipt.receipt_identity == projection.receipt_identity => {}
        Some(receipt) => diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_JSON_PROJECTION,
                "receipt_identity",
                "function-address JSON projection receipt identity does not match canonical Preserves bytes",
            )
            .with_detail("declared", projection.receipt_identity.hex().as_str())
            .with_detail("computed", receipt.receipt_identity.hex().as_str()),
        ),
        None => diagnostics.push(Diagnostic::new(
            DiagnosticCode::MalformedProjectionOutput,
            REQUIREMENT_JSON_PROJECTION,
            "receipt",
            "function-address JSON projection receipt body is not schema-valid",
        )),
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

pub fn export_function_address_valence_projection(
    receipt: &FunctionAddressReceiptEnvelope,
) -> Result<FunctionAddressValenceProjection, Vec<Diagnostic>> {
    let Some(producer_profile) = &receipt.body.producer_profile else {
        return Err(vec![Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_PRODUCER_BINDING,
            "producer_profile",
            "Valence projection requires a current producer-bound function-address receipt",
        )]);
    };
    let mut artifact_identities = vec![FunctionAddressValenceArtifact {
        role: "producer-profile".to_string(),
        key: producer_profile.schema_version.clone(),
        identity: AnyIdentity::Adapter(producer_profile.profile_identity.clone()),
    }];
    for source in &receipt.body.source_envelopes {
        artifact_identities.push(FunctionAddressValenceArtifact {
            role: "source-envelope".to_string(),
            key: source.key.clone(),
            identity: AnyIdentity::admit(source.source_identity.clone()).map_err(|error| {
                vec![Diagnostic::new(
                    DiagnosticCode::IdentityDomainMismatch,
                    REQUIREMENT_PRODUCER_BINDING,
                    "valence_projection.artifact_identities",
                    format!("source-envelope artifact identity admission failed: {error}"),
                )]
            })?,
        });
    }
    for function in &receipt.body.functions {
        artifact_identities.push(FunctionAddressValenceArtifact {
            role: "function-address".to_string(),
            key: function.locator.clone(),
            identity: AnyIdentity::admit(function.function_address.clone()).map_err(|error| {
                vec![Diagnostic::new(
                    DiagnosticCode::IdentityDomainMismatch,
                    REQUIREMENT_PRODUCER_BINDING,
                    "valence_projection.artifact_identities",
                    format!("function-address artifact identity admission failed: {error}"),
                )]
            })?,
        });
    }
    artifact_identities
        .sort_by(|left, right| left.key.cmp(&right.key).then(left.role.cmp(&right.role)));
    Ok(FunctionAddressValenceProjection {
        schema_version: FUNCTION_ADDRESS_VALENCE_PROJECTION_SCHEMA_VERSION.to_string(),
        source_receipt_identity: receipt.receipt_identity.clone(),
        producer: receipt.body.producer.name.clone(),
        producer_profile_identity: producer_profile.profile_identity.clone(),
        artifact_identities,
        dependency_edges: receipt.body.dependency_edges.clone(),
        non_claims: receipt.body.non_claims.clone(),
    })
}

fn build_receipt(
    input: &FunctionAddressReceiptInput,
    source_identities: &BTreeMap<String, WireIdentity>,
) -> Result<FunctionAddressReceiptEnvelope, ReceiptError> {
    let producer_profile = function_address_producer_profile_unchecked(&input.producer)?;
    let body = FunctionAddressReceiptBody {
        schema_version: FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION.to_string(),
        canonical_codec: FUNCTION_ADDRESS_CANONICAL_CODEC.to_string(),
        producer: input.producer.clone(),
        producer_profile: Some(producer_profile),
        source_envelopes: input.source_envelopes.clone(),
        functions: input.functions.clone(),
        dependency_edges: input.dependency_edges.clone(),
        caveats: input.caveats.clone(),
        non_claims: input.non_claims.clone(),
    };
    let canonical = canonical_receipt(input, source_identities)?;
    let bytes = typed_function_address_bytes(
        &canonical,
        FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION,
        FUNCTION_ADDRESS_ROOT_LABEL,
    )?;
    let receipt_identity = ReceiptIdentity::compute(&bytes);
    Ok(FunctionAddressReceiptEnvelope {
        body,
        receipt_identity,
    })
}

fn canonical_receipt<'a>(
    input: &'a FunctionAddressReceiptInput,
    source_identities: &BTreeMap<String, WireIdentity>,
) -> Result<CanonicalReceipt<'a>, ReceiptError> {
    let mut source_envelopes = source_identities
        .values()
        .cloned()
        .map(|source_identity| CanonicalSourceEnvelope { source_identity })
        .collect::<Vec<_>>();
    source_envelopes.sort();
    source_envelopes.dedup();

    let mut function_address_by_locator = BTreeMap::new();
    for function in &input.functions {
        function_address_by_locator
            .insert(function.locator.clone(), function.function_address.clone());
    }

    let mut functions = input
        .functions
        .iter()
        .filter_map(|function| {
            source_identities
                .get(&function.source_key)
                .map(|source_identity| CanonicalFunctionRecord {
                    source_identity: source_identity.clone(),
                    function_address: function.function_address.clone(),
                    dependency_closure: function.dependency_closure.clone(),
                    dependency_identity_kind: function.dependency_identity_kind.clone(),
                    caveats: function.caveats.clone(),
                })
        })
        .collect::<Vec<_>>();
    for function in &mut functions {
        function.caveats.sort();
        function.caveats.dedup();
    }
    functions.sort();
    functions.dedup();

    let mut dependency_edges = input
        .dependency_edges
        .iter()
        .filter_map(|edge| {
            let from_address = function_address_by_locator.get(&edge.from_locator)?;
            let to_address = function_address_by_locator.get(&edge.to_locator)?;
            Some(CanonicalDependencyEdge {
                from_address: from_address.clone(),
                to_address: to_address.clone(),
                dependency_identity: edge.dependency_identity.clone(),
                dependency_identity_kind: edge.dependency_identity_kind.clone(),
            })
        })
        .collect::<Vec<_>>();
    dependency_edges.sort();
    dependency_edges.dedup();

    let mut caveats = input
        .caveats
        .iter()
        .map(|caveat| caveat.reason.clone())
        .collect::<Vec<_>>();
    caveats.sort();
    caveats.dedup();

    let mut non_claims = input.non_claims.clone();
    non_claims.sort();
    non_claims.dedup();

    Ok(CanonicalReceipt {
        schema_version: FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION,
        canonical_codec: FUNCTION_ADDRESS_CANONICAL_CODEC,
        producer_profile_identity: function_address_producer_profile_unchecked(&input.producer)?
            .profile_identity
            .to_wire(),
        source_envelopes,
        functions,
        dependency_edges,
        caveats,
        non_claims,
    })
}

pub fn validate_function_address_producer(producer: &FunctionAddressProducer) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_producer(producer, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

fn validate_producer(producer: &FunctionAddressProducer, diagnostics: &mut Vec<Diagnostic>) {
    if producer.name != OCTET_PRODUCER {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedClaim,
                REQUIREMENT_OCTET_INTEROP,
                "producer.name",
                "first function-address receipt slice only accepts Octet producer input",
            )
            .with_detail("actual", producer.name.clone())
            .with_detail("expected", OCTET_PRODUCER),
        );
    }
    if producer.canonicalizer.trim().is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_CONTRACT,
            "producer.canonicalizer",
            "function-address receipts must bind the producer canonicalizer profile and version",
        ));
    } else if !has_versioned_canonicalizer_profile(&producer.canonicalizer) {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::IncompleteMetadata,
            REQUIREMENT_PRODUCER_PROFILE,
            "producer.canonicalizer",
            "canonicalizer profile must end in an explicit numeric version such as -v1",
        ));
    }
    if producer.tool_version.trim().is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_PRODUCER_PROFILE,
            "producer.tool_version",
            "function-address producer profiles require a tool version or build cohort",
        ));
    }
    if let Some(build_identity) = &producer.build_identity {
        validate_identity_domain(
            build_identity,
            IdentityDomain::Adapter,
            "producer.build_identity",
            diagnostics,
        );
    }
    for (key, value) in &producer.normative_options {
        if key.trim().is_empty() || value.trim().is_empty() {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::IncompleteMetadata,
                REQUIREMENT_PRODUCER_PROFILE,
                "producer.normative_options",
                "producer normative option keys and values must be non-empty",
            ));
        }
    }
    for required in REQUIRED_PRODUCER_NON_CLAIMS {
        if !producer
            .non_claims
            .iter()
            .any(|non_claim| non_claim == required)
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingNonClaim,
                    REQUIREMENT_PRODUCER_PROFILE,
                    "producer.non_claims",
                    "producer profile is missing a required non-claim",
                )
                .with_detail("missing", *required),
            );
        }
    }
}

fn has_versioned_canonicalizer_profile(value: &str) -> bool {
    let Some((profile, version)) = value.rsplit_once("-v") else {
        return false;
    };
    !profile.is_empty() && !version.is_empty() && version.bytes().all(|byte| byte.is_ascii_digit())
}

fn validate_non_claims(non_claims: &[String], diagnostics: &mut Vec<Diagnostic>) {
    for required in REQUIRED_NON_CLAIMS {
        if !non_claims.iter().any(|non_claim| non_claim == required) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingNonClaim,
                    REQUIREMENT_VALIDATION,
                    "non_claims",
                    "function-address receipts must preserve required non-claim boundaries",
                )
                .with_detail("missing", *required),
            );
        }
    }
}

fn validate_semantic_claims(claims: &[String], diagnostics: &mut Vec<Diagnostic>) {
    for claim in claims {
        let lowered = claim.to_ascii_lowercase();
        if FORBIDDEN_CLAIM_FRAGMENTS
            .iter()
            .any(|fragment| lowered.contains(fragment))
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_DEPENDENCY_OVERCLAIM,
                    "semantic_claims",
                    "function-address receipt contains forbidden overclaim text",
                )
                .with_detail("claim", claim.clone()),
            );
        }
    }
}

fn validate_sources(
    sources: &[FunctionAddressSourceEnvelope],
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, WireIdentity> {
    let mut identities = BTreeMap::new();
    let mut keys = BTreeSet::new();
    for source in sources {
        if !keys.insert(source.key.clone()) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::DuplicateArtifactIdentity,
                    REQUIREMENT_VALIDATION,
                    "source_envelopes.key",
                    "source envelope keys must be unique within a receipt",
                )
                .with_detail("key", source.key.clone()),
            );
        }
        validate_identity_domain(
            &source.source_identity,
            IdentityDomain::Source,
            "source_envelopes.source_identity",
            diagnostics,
        );
        let computed = compute_identity(IdentityDomain::Source, source.source_bytes.as_bytes());
        if source.source_identity.domain == IdentityDomain::Source
            && source.source_identity.hex != computed.hex
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::StaleIdentity,
                    REQUIREMENT_VALIDATION,
                    "source_envelopes.source_identity",
                    "source envelope identity does not match preserved source bytes",
                )
                .with_detail("key", source.key.clone()),
            );
        }
        identities.insert(source.key.clone(), source.source_identity.clone());
    }
    if sources.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_CONTRACT,
            "source_envelopes",
            "function-address receipts require at least one source envelope",
        ));
    }
    identities
}

fn validate_functions(
    input: &FunctionAddressReceiptInput,
    source_identities: &BTreeMap<String, WireIdentity>,
    source_lengths: &BTreeMap<String, u64>,
    dependency_kinds: &mut BTreeMap<WireIdentity, FunctionAddressDependencyIdentityKind>,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, WireIdentity> {
    let mut function_addresses = BTreeMap::new();
    let mut locators = BTreeSet::new();
    for function in &input.functions {
        if !locators.insert(function.locator.clone()) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::DuplicateArtifactIdentity,
                    REQUIREMENT_VALIDATION,
                    "functions.locator",
                    "function locators must be unique within a receipt",
                )
                .with_detail("locator", function.locator.clone()),
            );
        }
        if !source_identities.contains_key(&function.source_key) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "functions.source_key",
                    "function record references an absent source envelope",
                )
                .with_detail("locator", function.locator.clone())
                .with_detail("source_key", function.source_key.clone()),
            );
        }
        validate_function_span(function, source_lengths, diagnostics);
        validate_identity_domain(
            &function.function_address,
            IdentityDomain::FunctionAddress,
            "functions.function_address",
            diagnostics,
        );
        validate_dependency_identity_metadata(
            &function.dependency_closure,
            &function.dependency_identity_kind,
            "functions.dependency_closure",
            "functions.dependency_identity_kind",
            dependency_kinds,
            diagnostics,
        );
        function_addresses.insert(function.locator.clone(), function.function_address.clone());
    }
    if input.functions.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_CONTRACT,
            "functions",
            "function-address receipts require at least one function record",
        ));
    }
    function_addresses
}

fn validate_function_span(
    function: &FunctionAddressRecord,
    source_lengths: &BTreeMap<String, u64>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(span) = function.span.as_ref() else {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MissingInput,
                REQUIREMENT_PRESERVES_SCHEMA,
                "functions.span",
                "canonical function-address Preserves receipts require byte spans from the Octet handoff",
            )
            .with_detail("locator", function.locator.clone()),
        );
        return;
    };
    if span.source_key != function.source_key {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::StaleIdentity,
                REQUIREMENT_PRESERVES_SCHEMA,
                "functions.span.source_key",
                "function-address span source key must match the function record source key",
            )
            .with_detail("locator", function.locator.clone())
            .with_detail("span_source_key", span.source_key.clone())
            .with_detail("function_source_key", function.source_key.clone()),
        );
    }
    if span.start_byte >= span.end_byte {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PRESERVES_SCHEMA,
                "functions.span",
                "function-address span must be a non-empty increasing byte range",
            )
            .with_detail("locator", function.locator.clone()),
        );
    }
    if let Some(source_len) = source_lengths.get(&span.source_key)
        && span.end_byte > *source_len
    {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PRESERVES_SCHEMA,
                "functions.span.end_byte",
                "function-address span must fit within the preserved source envelope bytes",
            )
            .with_detail("locator", function.locator.clone())
            .with_detail("source_len", source_len.to_string())
            .with_detail("end_byte", span.end_byte.to_string()),
        );
    }
}

fn validate_dependency_edges(
    input: &FunctionAddressReceiptInput,
    function_addresses: &BTreeMap<String, WireIdentity>,
    dependency_kinds: &mut BTreeMap<WireIdentity, FunctionAddressDependencyIdentityKind>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for edge in &input.dependency_edges {
        if !function_addresses.contains_key(&edge.from_locator) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "dependency_edges.from_locator",
                    "dependency edge source function is absent from receipt",
                )
                .with_detail("from_locator", edge.from_locator.clone()),
            );
        }
        if !function_addresses.contains_key(&edge.to_locator) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "dependency_edges.to_locator",
                    "dependency edge target function is absent from receipt",
                )
                .with_detail("to_locator", edge.to_locator.clone()),
            );
        }
        validate_dependency_identity_metadata(
            &edge.dependency_identity,
            &edge.dependency_identity_kind,
            "dependency_edges.dependency_identity",
            "dependency_edges.dependency_identity_kind",
            dependency_kinds,
            diagnostics,
        );
    }
}

fn validate_dependency_identity_metadata(
    identity: &Option<WireIdentity>,
    kind: &Option<FunctionAddressDependencyIdentityKind>,
    identity_field: &str,
    kind_field: &str,
    dependency_kinds: &mut BTreeMap<WireIdentity, FunctionAddressDependencyIdentityKind>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match (identity, kind) {
        (Some(identity), Some(kind)) => {
            validate_identity_domain(
                identity,
                kind.expected_domain(),
                identity_field,
                diagnostics,
            );
            validate_dependency_kind_consistency(
                identity,
                kind,
                kind_field,
                dependency_kinds,
                diagnostics,
            );
        }
        (Some(identity), None) => {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_DEPENDENCY_KIND,
                    kind_field,
                    "dependency identities must declare whether they are token/source-shape evidence or stronger closure evidence",
                )
                .with_detail("identity_domain", identity.domain.to_string())
                .with_detail("identity_hex", identity.hex.as_str()),
            );
        }
        (None, Some(kind)) => {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_DEPENDENCY_COMPAT,
                    identity_field,
                    "dependency identity kind cannot appear without a dependency identity slot",
                )
                .with_detail("kind", kind.label()),
            );
        }
        (None, None) => {}
    }
}

fn validate_dependency_kind_consistency(
    identity: &WireIdentity,
    kind: &FunctionAddressDependencyIdentityKind,
    kind_field: &str,
    dependency_kinds: &mut BTreeMap<WireIdentity, FunctionAddressDependencyIdentityKind>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(existing) = dependency_kinds.get(identity) {
        if existing != kind {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::UnsupportedClaim,
                    REQUIREMENT_DEPENDENCY_KIND,
                    kind_field,
                    "the same dependency identity cannot be assigned multiple dependency evidence kinds",
                )
                .with_detail("identity_hex", identity.hex.as_str())
                .with_detail("existing_kind", existing.label())
                .with_detail("actual_kind", kind.label()),
            );
        }
        return;
    }
    dependency_kinds.insert(identity.clone(), kind.clone());
}

fn validate_caveats(
    input: &FunctionAddressReceiptInput,
    function_addresses: &BTreeMap<String, WireIdentity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for caveat in &input.caveats {
        if !function_addresses.contains_key(&caveat.locator) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "caveats.locator",
                    "caveat references an absent function locator",
                )
                .with_detail("locator", caveat.locator.clone()),
            );
        }
        if caveat.reason.is_empty() {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "caveats.reason",
                    "caveats must include a reviewable reason",
                )
                .with_detail("locator", caveat.locator.clone()),
            );
        }
    }
}

fn validate_identity_domain(
    identity: &WireIdentity,
    expected: IdentityDomain,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if identity.validate_domain(expected.clone()).is_err() {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_VALIDATION,
                field,
                "function-address receipt identity uses the wrong domain or algorithm",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

fn source_identity_map(
    sources: &[FunctionAddressSourceEnvelope],
) -> BTreeMap<String, WireIdentity> {
    sources
        .iter()
        .map(|source| (source.key.clone(), source.source_identity.clone()))
        .collect()
}

fn sorted_strings(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

fn source_byte_lengths(
    sources: &[FunctionAddressSourceEnvelope],
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, u64> {
    let mut lengths = BTreeMap::new();
    for source in sources {
        match u64::try_from(source.source_bytes.len()) {
            Ok(length) => {
                lengths.insert(source.key.clone(), length);
            }
            Err(error) => diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MalformedProjectionOutput,
                    REQUIREMENT_PRESERVES_SCHEMA,
                    "source_envelopes.source_bytes",
                    "source envelope byte length exceeds the supported receipt range",
                )
                .with_detail("source_key", source.key.clone())
                .with_detail("error", error.to_string()),
            ),
        }
    }
    lengths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::{Blake3Hex, IdentityError};

    const SOURCE_KEY: &str = "src/lib.rs";
    const SOURCE_PATH: &str = "src/lib.rs";
    const RENAMED_SOURCE_PATH: &str = "src/renamed.rs";
    const SOURCE_BYTES: &str = "pub fn f() -> u32 { 1 }\npub fn g() -> u32 { f() }\n";
    const FUNCTION_F_LOCATOR: &str = "src/lib.rs::f";
    const FUNCTION_G_LOCATOR: &str = "src/lib.rs::g";
    const FUNCTION_F_RENAMED_LOCATOR: &str = "src/renamed.rs::renamed_f";
    const FUNCTION_F_NAME: &str = "f";
    const FUNCTION_G_NAME: &str = "g";
    const RENAMED_FUNCTION_NAME: &str = "renamed_f";
    const CANONICALIZER: &str = "octet-source-token-function-address-v1";
    const TOOL_VERSION: &str = "octet-test";
    const CLAIM_RELEASE_ELIGIBILITY: &str = "this proves release eligibility";
    const INVALID_SHORT_HEX: &str = "abc";
    const SPAN_START: u64 = 0;
    const SPAN_END: u64 = 23;
    const SPAN_PAST_SOURCE_INCREMENT: u64 = 1;

    fn valid_input() -> FunctionAddressReceiptInput {
        let source_identity = compute_identity(IdentityDomain::Source, SOURCE_BYTES.as_bytes());
        let function_f = compute_identity(IdentityDomain::FunctionAddress, b"function f");
        let function_g = compute_identity(IdentityDomain::FunctionAddress, b"function g");
        let dependency = compute_identity(IdentityDomain::DependencyClosure, b"f -> g");
        FunctionAddressReceiptInput {
            producer: FunctionAddressProducer {
                name: OCTET_PRODUCER.to_string(),
                canonicalizer: CANONICALIZER.to_string(),
                tool_version: TOOL_VERSION.to_string(),
                build_identity: None,
                normative_options: BTreeMap::new(),
                non_claims: REQUIRED_PRODUCER_NON_CLAIMS
                    .iter()
                    .map(|claim| (*claim).to_string())
                    .collect(),
            },
            source_envelopes: vec![FunctionAddressSourceEnvelope {
                key: SOURCE_KEY.to_string(),
                display_path: SOURCE_PATH.to_string(),
                language: "rust".to_string(),
                source_bytes: SOURCE_BYTES.to_string(),
                source_identity,
            }],
            functions: vec![
                FunctionAddressRecord {
                    locator: FUNCTION_F_LOCATOR.to_string(),
                    display_name: FUNCTION_F_NAME.to_string(),
                    source_key: SOURCE_KEY.to_string(),
                    span: Some(FunctionAddressSpan {
                        source_key: SOURCE_KEY.to_string(),
                        start_byte: SPAN_START,
                        end_byte: SPAN_END,
                    }),
                    function_address: function_f,
                    dependency_closure: Some(dependency.clone()),
                    dependency_identity_kind: Some(
                        FunctionAddressDependencyIdentityKind::SourceTokenDependencySet,
                    ),
                    caveats: Vec::new(),
                },
                FunctionAddressRecord {
                    locator: FUNCTION_G_LOCATOR.to_string(),
                    display_name: FUNCTION_G_NAME.to_string(),
                    source_key: SOURCE_KEY.to_string(),
                    span: Some(FunctionAddressSpan {
                        source_key: SOURCE_KEY.to_string(),
                        start_byte: SPAN_END,
                        end_byte: u64::try_from(SOURCE_BYTES.len()).unwrap_or(u64::MAX),
                    }),
                    function_address: function_g,
                    dependency_closure: Some(dependency.clone()),
                    dependency_identity_kind: Some(
                        FunctionAddressDependencyIdentityKind::SourceTokenDependencySet,
                    ),
                    caveats: vec!["dependency-closure-review-required".to_string()],
                },
            ],
            dependency_edges: vec![FunctionAddressDependencyEdge {
                from_locator: FUNCTION_G_LOCATOR.to_string(),
                to_locator: FUNCTION_F_LOCATOR.to_string(),
                dependency_identity: Some(dependency),
                dependency_identity_kind: Some(
                    FunctionAddressDependencyIdentityKind::SourceTokenDependencySet,
                ),
            }],
            caveats: vec![FunctionAddressCaveat {
                locator: FUNCTION_G_LOCATOR.to_string(),
                reason: "macro or unresolved dependency caveat retained".to_string(),
            }],
            semantic_claims: Vec::new(),
            non_claims: REQUIRED_NON_CLAIMS
                .iter()
                .map(|claim| (*claim).to_string())
                .collect(),
        }
    }

    fn valid_receipt() -> Result<FunctionAddressReceiptEnvelope, Vec<Diagnostic>> {
        let result = validate_function_address_receipt(&valid_input());
        result.receipt.ok_or(result.diagnostics)
    }

    #[test]
    fn valid_octet_receipt_preserves_inventory_and_projection_identity() {
        let input = valid_input();

        let result = validate_function_address_receipt(&input);

        assert!(
            result.passed,
            "unexpected diagnostics: {:?}",
            result.diagnostics
        );
        assert!(
            result.receipt.is_some(),
            "passing function-address validation must emit a receipt"
        );
        let Some(receipt) = result.receipt else {
            return;
        };
        assert_eq!(
            receipt.body.schema_version,
            FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION
        );
        assert_eq!(receipt.receipt_identity.domain(), IdentityDomain::Receipt);
        let projection = function_address_json_projection(&receipt);
        assert_eq!(projection.receipt_identity, receipt.receipt_identity);
        assert_eq!(
            projection.compatibility_projection,
            FUNCTION_ADDRESS_JSON_PROJECTION
        );
    }

    #[test]
    fn canonical_preserves_bytes_own_receipt_identity_and_projection_round_trips() {
        let input = valid_input();
        let receipt_result = valid_receipt();
        assert!(
            receipt_result.is_ok(),
            "valid receipt fixture must materialize"
        );
        let Ok(receipt) = receipt_result else {
            return;
        };
        let bytes_result = function_address_canonical_bytes(&input);
        assert!(
            bytes_result.is_ok(),
            "canonical function-address bytes must serialize"
        );
        let Ok(canonical_bytes) = bytes_result else {
            return;
        };
        let projection_json_result = function_address_projection_json(&receipt);
        assert!(
            projection_json_result.is_ok(),
            "function-address projection must serialize"
        );
        let Ok(projection_json) = projection_json_result else {
            return;
        };
        let projection_result =
            serde_json::from_str::<FunctionAddressJsonProjection>(&projection_json);
        assert!(
            projection_result.is_ok(),
            "function-address projection must round trip"
        );
        let Ok(projection) = projection_result else {
            return;
        };

        assert_eq!(
            ReceiptIdentity::compute(&canonical_bytes),
            receipt.receipt_identity
        );
        assert_eq!(projection.receipt_identity, receipt.receipt_identity);
        assert!(validate_function_address_json_projection(&projection).is_empty());
    }

    #[test]
    fn producer_profile_and_receipt_identity_bind_normative_provenance() {
        let original_input = valid_input();
        let original_profile_result = function_address_producer_profile(&original_input.producer);
        assert!(original_profile_result.is_ok());
        let Ok(original_profile) = original_profile_result else {
            return;
        };
        let original_receipt_result = validate_function_address_receipt(&original_input);
        assert!(original_receipt_result.receipt.is_some());
        let Some(original_receipt) = original_receipt_result.receipt else {
            return;
        };

        let mut changed_version = original_input.clone();
        changed_version.producer.tool_version = "octet-test-v2".to_string();
        let version_profile_result = function_address_producer_profile(&changed_version.producer);
        assert!(version_profile_result.is_ok());
        let Ok(version_profile) = version_profile_result else {
            return;
        };
        let version_receipt_result = validate_function_address_receipt(&changed_version);
        assert!(version_receipt_result.receipt.is_some());
        let Some(version_receipt) = version_receipt_result.receipt else {
            return;
        };

        let mut changed_canonicalizer = original_input.clone();
        changed_canonicalizer.producer.canonicalizer =
            "octet-source-token-function-address-v2".to_string();
        let canonicalizer_profile_result =
            function_address_producer_profile(&changed_canonicalizer.producer);
        assert!(canonicalizer_profile_result.is_ok());
        let Ok(canonicalizer_profile) = canonicalizer_profile_result else {
            return;
        };

        let mut changed_options = original_input.clone();
        changed_options
            .producer
            .normative_options
            .insert("macro-expansion".to_string(), "disabled".to_string());
        let options_profile_result = function_address_producer_profile(&changed_options.producer);
        assert!(options_profile_result.is_ok());
        let Ok(options_profile) = options_profile_result else {
            return;
        };

        assert_ne!(
            original_profile.profile_identity,
            version_profile.profile_identity
        );
        assert_ne!(
            original_profile.profile_identity,
            canonicalizer_profile.profile_identity
        );
        assert_ne!(
            original_profile.profile_identity,
            options_profile.profile_identity
        );
        assert_ne!(
            original_receipt.receipt_identity,
            version_receipt.receipt_identity
        );
    }

    #[test]
    fn projection_rejects_unbound_displayed_producer_version() {
        let receipt_result = valid_receipt();
        assert!(receipt_result.is_ok());
        let Ok(receipt) = receipt_result else {
            return;
        };
        let mut projection = function_address_json_projection(&receipt);
        projection.receipt.producer.tool_version = "unbound-version".to_string();

        let diagnostics = validate_function_address_json_projection(&projection);

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == DiagnosticCode::StaleIdentity
                && diagnostic.field == "receipt.producer_profile"
        }));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == DiagnosticCode::StaleIdentity
                && diagnostic.field == "receipt_identity"
        }));
    }

    #[test]
    fn legacy_v1_receipts_require_the_tool_version_unbound_caveat() {
        let receipt_result = valid_receipt();
        assert!(receipt_result.is_ok());
        let Ok(receipt) = receipt_result else {
            return;
        };
        let mut legacy = receipt.body;
        legacy.schema_version = LEGACY_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION.to_string();
        legacy.canonical_codec = LEGACY_FUNCTION_ADDRESS_CANONICAL_CODEC.to_string();
        legacy.producer_profile = None;

        assert_eq!(
            classify_function_address_receipt(&legacy),
            FunctionAddressReceiptTrustClass::Unsupported
        );

        legacy.caveats.push(FunctionAddressCaveat {
            locator: "*".to_string(),
            reason: LEGACY_TOOL_VERSION_UNBOUND_CAVEAT.to_string(),
        });
        assert_eq!(
            classify_function_address_receipt(&legacy),
            FunctionAddressReceiptTrustClass::LegacyToolVersionUnbound
        );
    }

    #[test]
    fn legacy_v2_pseudo_codec_is_compatibility_evidence_only() {
        let receipt_result = valid_receipt();
        assert!(receipt_result.is_ok());
        let Ok(receipt) = receipt_result else {
            return;
        };
        let mut projection = function_address_json_projection(&receipt);
        projection.receipt.schema_version =
            LEGACY_PSEUDO_FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION.to_string();
        projection.receipt.canonical_codec =
            LEGACY_PSEUDO_FUNCTION_ADDRESS_CANONICAL_CODEC.to_string();

        assert_eq!(
            classify_function_address_receipt(&projection.receipt),
            FunctionAddressReceiptTrustClass::LegacyPseudoCodec
        );
        let diagnostics = validate_function_address_json_projection(&projection);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == DiagnosticCode::UnsupportedSchemaVersion
                && diagnostic.field == "receipt.canonical_codec"
        }));
    }

    #[test]
    fn producer_profile_rejects_missing_provenance_and_non_claims() {
        let mut input = valid_input();
        input.producer.tool_version.clear();
        input.producer.canonicalizer = "unversioned-canonicalizer".to_string();
        input.producer.build_identity = Some(compute_identity(
            IdentityDomain::Receipt,
            b"wrong producer build domain",
        ));
        input.producer.non_claims.clear();
        input
            .producer
            .normative_options
            .insert(String::new(), String::new());

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.field == "producer.tool_version" })
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.field == "producer.non_claims" })
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.field == "producer.canonicalizer" })
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.field == "producer.build_identity" })
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.field == "producer.normative_options" })
        );
    }

    #[test]
    fn projection_drift_or_json_authority_is_rejected() {
        let receipt_result = valid_receipt();
        assert!(
            receipt_result.is_ok(),
            "projection drift base receipt must materialize"
        );
        let Ok(receipt) = receipt_result else {
            return;
        };
        let mut projection = function_address_json_projection(&receipt);
        projection.canonical_identity_owner = "json".to_string();
        projection.receipt.functions[0].function_address =
            compute_identity(IdentityDomain::FunctionAddress, b"drifted function");

        let diagnostics = validate_function_address_json_projection(&projection);

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == DiagnosticCode::MalformedProjectionOutput
                && diagnostic.field == "canonical_identity_owner"
        }));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == DiagnosticCode::StaleIdentity
                && diagnostic.field == "receipt_identity"
        }));
    }

    #[test]
    fn missing_or_out_of_range_spans_are_rejected() {
        let mut missing = valid_input();
        missing.functions[0].span = None;
        let mut out_of_range = valid_input();
        let source_len = u64::try_from(SOURCE_BYTES.len()).unwrap_or(u64::MAX);
        out_of_range.functions[0].span = Some(FunctionAddressSpan {
            source_key: SOURCE_KEY.to_string(),
            start_byte: SPAN_START,
            end_byte: source_len + SPAN_PAST_SOURCE_INCREMENT,
        });

        let missing_result = validate_function_address_receipt(&missing);
        let out_of_range_result = validate_function_address_receipt(&out_of_range);

        assert!(!missing_result.passed);
        assert!(
            missing_result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "functions.span")
        );
        assert!(!out_of_range_result.passed);
        assert!(
            out_of_range_result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "functions.span.end_byte")
        );
    }

    #[test]
    fn metadata_rename_and_path_churn_do_not_change_canonical_identity() {
        let original_result = valid_receipt();
        assert!(
            original_result.is_ok(),
            "metadata stability base receipt must materialize"
        );
        let Ok(original) = original_result else {
            return;
        };
        let mut renamed = valid_input();
        renamed.source_envelopes[0].display_path = RENAMED_SOURCE_PATH.to_string();
        renamed.functions[0].locator = FUNCTION_F_RENAMED_LOCATOR.to_string();
        renamed.functions[0].display_name = RENAMED_FUNCTION_NAME.to_string();
        renamed.dependency_edges[0].to_locator = FUNCTION_F_RENAMED_LOCATOR.to_string();

        let renamed_result = validate_function_address_receipt(&renamed);
        assert!(
            renamed_result.receipt.is_some(),
            "metadata-only rename must retain a valid receipt"
        );
        let Some(renamed_receipt) = renamed_result.receipt else {
            return;
        };

        assert_eq!(renamed_receipt.receipt_identity, original.receipt_identity);
    }

    #[test]
    fn valence_projection_preserves_dependencies_and_non_claims() {
        let receipt_result = valid_receipt();
        assert!(
            receipt_result.is_ok(),
            "Valence projection base receipt must materialize"
        );
        let Ok(receipt) = receipt_result else {
            return;
        };

        let projection_result = export_function_address_valence_projection(&receipt);
        assert!(
            projection_result.is_ok(),
            "valid receipt must export to Valence"
        );
        let Ok(projection) = projection_result else {
            return;
        };

        assert_eq!(projection.source_receipt_identity, receipt.receipt_identity);
        assert_eq!(
            projection.dependency_edges.len(),
            receipt.body.dependency_edges.len()
        );
        assert!(
            projection
                .non_claims
                .iter()
                .any(|claim| claim == NON_CLAIM_RELEASE_ELIGIBILITY)
        );
        assert_eq!(
            projection.dependency_edges[0].dependency_identity_kind,
            Some(FunctionAddressDependencyIdentityKind::SourceTokenDependencySet)
        );
        assert!(
            projection
                .artifact_identities
                .iter()
                .any(|artifact| artifact.role == "function-address")
        );
    }

    #[test]
    fn stale_source_identity_is_rejected() {
        let mut input = valid_input();
        input.source_envelopes[0].source_identity =
            compute_identity(IdentityDomain::Source, b"stale");

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
    }

    #[test]
    fn duplicate_locator_is_rejected() {
        let mut input = valid_input();
        input.functions[1].locator = FUNCTION_F_LOCATOR.to_string();

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::DuplicateArtifactIdentity)
        );
    }

    #[test]
    fn missing_dependency_target_is_rejected() {
        let mut input = valid_input();
        input.dependency_edges[0].to_locator = "src/lib.rs::absent".to_string();

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "dependency_edges.to_locator")
        );
    }

    #[test]
    fn missing_non_claim_and_overclaim_text_are_rejected() {
        let mut input = valid_input();
        input.non_claims.clear();
        input
            .semantic_claims
            .push(CLAIM_RELEASE_ELIGIBILITY.to_string());

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::SemanticOverclaim)
        );
    }

    #[test]
    fn wrong_identity_domain_is_rejected() {
        let mut input = valid_input();
        input.functions[0].function_address =
            compute_identity(IdentityDomain::Source, b"wrong-domain");

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::IdentityDomainMismatch)
        );
    }

    #[test]
    fn missing_dependency_identity_kind_is_rejected() {
        let mut input = valid_input();
        input.functions[0].dependency_identity_kind = None;

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.requirement_id == REQUIREMENT_DEPENDENCY_KIND
                && diagnostic.field == "functions.dependency_identity_kind"
        }));
    }

    #[test]
    fn dependency_identity_kind_mismatch_is_rejected() {
        let mut input = valid_input();
        input.dependency_edges[0].dependency_identity_kind =
            Some(FunctionAddressDependencyIdentityKind::SourceShapeDependencySet);

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.requirement_id == REQUIREMENT_DEPENDENCY_KIND
                && diagnostic.code == DiagnosticCode::UnsupportedClaim
        }));
    }

    #[test]
    fn semantic_dependency_closure_overclaim_is_rejected() {
        let mut input = valid_input();
        input
            .semantic_claims
            .push("token dependencies prove semantic dependency closure".to_string());

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::SemanticOverclaim)
        );
    }

    #[test]
    fn malformed_blake3_digest_fails_deserialization() {
        let malformed = Blake3Hex::parse(INVALID_SHORT_HEX);
        assert!(matches!(
            malformed,
            Err(IdentityError::InvalidLength { .. })
        ));
    }
}
