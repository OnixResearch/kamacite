use crate::{
    AdapterIdentity, CanonicalPreservesArtifact, CanonicalValueIdentity, IdentityDomain,
    PRESERVES_CANONICAL_CODEC, PreservesLimits, PreservesSchema, ProjectionIdentity,
    ReceiptIdentity, WireIdentity, encode_typed_preserves,
};
use serde::de::{Error as DeserializeError, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Number, Value};
use std::collections::BTreeMap;
use std::fmt::{self, Display};

pub const NICKEL_PRESERVES_PROFILE_SCHEMA: &str = "kamacite.nickel-json-to-preserves-profile.v1";
pub const NICKEL_PRESERVES_MAPPING_PROFILE: &str = "nickel-json-to-preserves-v1";
pub const NICKEL_PRESERVES_SCHEMA_VERSION: &str = "kamacite.nickel-json-value.v1";
pub const NICKEL_PRESERVES_ROOT_LABEL: &str = "kamacite-nickel-json-value-v1";
pub const NICKEL_EXPORT_MANIFEST_SCHEMA_V3: &str = "onix-nickel-export-manifest/v3";
pub const NICKEL_EXPORT_RECEIPT_SCHEMA_V3: &str = "onix-nickel-export-receipt/v3";
pub const NICKEL_PRESERVES_RECEIPT_SCHEMA: &str = "kamacite.nickel-preserves-adapter-receipt.v1";

const B3_PREFIX: &str = "b3:";
const BLAKE3_HEX_BYTES: usize = 64;
const ROOT_DEPTH: usize = 1;
const NICKEL_PRESERVES_OUTPUT_CLASSES: &[&str] = &[
    "canonical-packed-preserves",
    "preserves-review-text",
    "adapter-receipt",
];
const NICKEL_PRESERVES_NUMERIC_POLICY: &str = "signed-or-unsigned-64-bit-integers-only";
const NICKEL_PRESERVES_OBJECT_ORDER: &str = "unicode-codepoint-lexicographic";
const NICKEL_PRESERVES_UNICODE_POLICY: &str = "decoded-unicode-scalar-values";
const NICKEL_PRESERVES_CHECKED_LAWS: &[&str] = &[
    "exact-admitted-json-bytes-bound",
    "closed-json-alternatives",
    "deterministic-object-order",
    "one-typed-value-owns-canonical-output",
    "packed-and-text-value-equivalence",
];
const NICKEL_PRESERVES_NON_CLAIMS: &[&str] = &[
    "preserves only the decoded JSON-compatible export value",
    "not arbitrary Nickel source losslessness or semantic equivalence",
    "not evaluator correctness or import-closure completeness",
    "not authorization, runtime behavior, or release eligibility",
    "review text is non-authoritative and Markdown/AsciiDoc are outside this adapter",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NickelPreservesLimits {
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub max_depth: usize,
    pub max_collection_items: usize,
    pub max_total_values: usize,
    pub max_string_bytes: usize,
    pub max_numeric_lexeme_bytes: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NickelPreservesProfile {
    pub schema_version: String,
    pub profile_id: String,
    pub accepted_manifest_schemas: Vec<String>,
    pub accepted_receipt_schemas: Vec<String>,
    pub mapping_profile: String,
    pub preserves_schema_version: String,
    pub preserves_root_label: String,
    pub canonical_codec: String,
    pub output_classes: Vec<String>,
    pub limits: NickelPreservesLimits,
    pub numeric_policy: String,
    pub object_order: String,
    pub unicode_policy: String,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NickelExportIdentityKind {
    Manifest,
    ExportReceipt,
    DeclaredInput,
    Source,
    EvaluatorArtifact,
    EvaluationPlan,
    JsonOutput,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NickelExportIdentity {
    pub kind: NickelExportIdentityKind,
    pub b3: String,
}

impl NickelExportIdentity {
    pub fn new(kind: NickelExportIdentityKind, b3: String) -> Result<Self, NickelPreservesError> {
        validate_external_b3(&b3)?;
        Ok(Self { kind, b3 })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmittedNickelJsonExport {
    pub manifest_schema: String,
    pub receipt_schema: String,
    pub destination: String,
    pub manifest_identity: NickelExportIdentity,
    pub manifest_wire_identity: WireIdentity,
    pub export_receipt_identity: NickelExportIdentity,
    pub declared_input_identity: NickelExportIdentity,
    pub source_identity: NickelExportIdentity,
    pub evaluator_label: String,
    pub evaluator_version: String,
    pub evaluator_artifact_identity: NickelExportIdentity,
    pub evaluation_plan_identity: NickelExportIdentity,
    pub json_output_identity: NickelExportIdentity,
    pub json_bytes: Vec<u8>,
    pub upstream_non_claim: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-nickel-json-value-v1")]
pub struct NickelJsonPreservesRoot {
    pub mapping_profile: String,
    pub value: NickelJsonPreservesValue,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum NickelJsonPreservesValue {
    Null,
    Boolean(bool),
    String(String),
    SignedInteger(i64),
    UnsignedInteger(u64),
    Array(Vec<NickelJsonPreservesValue>),
    Object(BTreeMap<String, NickelJsonPreservesValue>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NickelPreservesReceiptBody {
    pub schema_version: String,
    pub manifest_identity: NickelExportIdentity,
    pub manifest_wire_identity: AdapterIdentity,
    pub export_receipt_identity: NickelExportIdentity,
    pub declared_input_identity: NickelExportIdentity,
    pub source_identity: NickelExportIdentity,
    pub evaluator_label: String,
    pub evaluator_version: String,
    pub evaluator_artifact_identity: NickelExportIdentity,
    pub evaluation_plan_identity: NickelExportIdentity,
    pub json_output_identity: NickelExportIdentity,
    pub json_projection_identity: ProjectionIdentity,
    pub mapping_profile_identity: AdapterIdentity,
    pub canonical_codec_identity: AdapterIdentity,
    pub packed_preserves_identity: CanonicalValueIdentity,
    pub packed_output_bytes: u64,
    pub review_text_identity: ProjectionIdentity,
    pub checked_laws: Vec<String>,
    pub diagnostics: Vec<String>,
    pub upstream_non_claim: String,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NickelPreservesReceipt {
    pub body: NickelPreservesReceiptBody,
    pub receipt_identity: ReceiptIdentity,
}

#[derive(Clone, Debug)]
pub struct NickelPreservesAdaptation {
    pub profile: NickelPreservesProfile,
    pub profile_identity: AdapterIdentity,
    pub canonical_artifact: CanonicalPreservesArtifact,
    pub packed_preserves_identity: CanonicalValueIdentity,
    pub review_text_identity: ProjectionIdentity,
    pub receipt: NickelPreservesReceipt,
}

struct StrictJsonValue(Value);

impl<'de> Deserialize<'de> for StrictJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonVisitor)
    }
}

struct StrictJsonVisitor;

impl<'de> Visitor<'de> for StrictJsonVisitor {
    type Value = StrictJsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON-compatible Nickel export value without duplicate object keys")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        Ok(StrictJsonValue(Value::Null))
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        Ok(StrictJsonValue(Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        Ok(StrictJsonValue(Value::Number(Number::from(value))))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        Ok(StrictJsonValue(Value::Number(Number::from(value))))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        Number::from_f64(value)
            .map(Value::Number)
            .map(StrictJsonValue)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        Ok(StrictJsonValue(Value::String(value.to_string())))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        Ok(StrictJsonValue(Value::String(value)))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(StrictJsonValue(value)) = sequence.next_element::<StrictJsonValue>()? {
            values.push(value);
        }
        Ok(StrictJsonValue(Value::Array(values)))
    }

    fn visit_map<A>(self, mut object: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Map::new();
        while let Some((key, StrictJsonValue(value))) =
            object.next_entry::<String, StrictJsonValue>()?
        {
            if values.contains_key(&key) {
                return Err(A::Error::custom(format!(
                    "duplicate JSON object key {key:?}"
                )));
            }
            values.insert(key, value);
        }
        Ok(StrictJsonValue(Value::Object(values)))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NickelPreservesError {
    MalformedProfile(String),
    UnsupportedProfile(String),
    InvalidLimit(&'static str),
    InvalidExternalIdentity(String),
    IdentityKindMismatch {
        field: &'static str,
        expected: NickelExportIdentityKind,
        actual: NickelExportIdentityKind,
    },
    StaleJsonIdentity,
    InputLimit {
        actual: usize,
        limit: usize,
    },
    DepthLimit {
        actual: usize,
        limit: usize,
    },
    CollectionLimit {
        actual: usize,
        limit: usize,
    },
    TotalValueLimit {
        actual: usize,
        limit: usize,
    },
    StringLimit {
        actual: usize,
        limit: usize,
    },
    NumericLimit {
        actual: usize,
        limit: usize,
    },
    UnsupportedNumber(String),
    MalformedJson(String),
    Codec(String),
    SizeOverflow,
    ReceiptEncoding(String),
}

impl Display for NickelPreservesError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedProfile(error) => {
                write!(formatter, "malformed Nickel Preserves profile: {error}")
            }
            Self::UnsupportedProfile(field) => write!(
                formatter,
                "unsupported Nickel Preserves profile field: {field}"
            ),
            Self::InvalidLimit(field) => {
                write!(formatter, "Nickel Preserves limit {field} must be nonzero")
            }
            Self::InvalidExternalIdentity(identity) => write!(
                formatter,
                "invalid nickel-export BLAKE3 identity {identity:?}"
            ),
            Self::IdentityKindMismatch {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "Nickel export identity kind mismatch for {field}: expected {expected:?}, got {actual:?}"
            ),
            Self::StaleJsonIdentity => write!(
                formatter,
                "verified nickel-export output identity does not match exact JSON bytes"
            ),
            Self::InputLimit { actual, limit } => write!(
                formatter,
                "Nickel JSON input size {actual} exceeds limit {limit}"
            ),
            Self::DepthLimit { actual, limit } => write!(
                formatter,
                "Nickel JSON depth {actual} exceeds limit {limit}"
            ),
            Self::CollectionLimit { actual, limit } => write!(
                formatter,
                "Nickel JSON collection size {actual} exceeds limit {limit}"
            ),
            Self::TotalValueLimit { actual, limit } => write!(
                formatter,
                "Nickel JSON value count {actual} exceeds limit {limit}"
            ),
            Self::StringLimit { actual, limit } => write!(
                formatter,
                "Nickel JSON string size {actual} exceeds limit {limit}"
            ),
            Self::NumericLimit { actual, limit } => write!(
                formatter,
                "Nickel JSON number lexeme size {actual} exceeds limit {limit}"
            ),
            Self::UnsupportedNumber(number) => write!(
                formatter,
                "Nickel JSON number {number:?} is outside the integer-only v1 profile"
            ),
            Self::MalformedJson(error) => {
                write!(formatter, "malformed Nickel JSON export: {error}")
            }
            Self::Codec(error) => write!(formatter, "Nickel Preserves codec failed: {error}"),
            Self::SizeOverflow => write!(
                formatter,
                "Nickel Preserves byte length cannot be represented as u64"
            ),
            Self::ReceiptEncoding(error) => write!(
                formatter,
                "Nickel Preserves receipt encoding failed: {error}"
            ),
        }
    }
}

impl std::error::Error for NickelPreservesError {}

struct MappingState<'a> {
    limits: &'a NickelPreservesLimits,
    total_values: usize,
}

// r[impl kamacite.nickel_preserves_adapter.mapping_profile]
// r[impl kamacite.nickel_preserves_adapter.canonical_output]
// r[impl kamacite.nickel_preserves_adapter.receipt]
// r[impl kamacite.nickel_preserves_adapter.non_claim]
// r[impl kamacite.nickel_preserves_adapter.text_projection]
pub fn adapt_admitted_nickel_json_export(
    admitted: &AdmittedNickelJsonExport,
    profile_bytes: &[u8],
    canonical_codec: &str,
) -> Result<NickelPreservesAdaptation, NickelPreservesError> {
    let profile = parse_and_validate_profile(profile_bytes, canonical_codec)?;
    validate_admitted_export(admitted)?;
    if admitted.json_bytes.len() > profile.limits.max_input_bytes {
        return Err(NickelPreservesError::InputLimit {
            actual: admitted.json_bytes.len(),
            limit: profile.limits.max_input_bytes,
        });
    }
    validate_numeric_lexeme_lengths(
        &admitted.json_bytes,
        profile.limits.max_numeric_lexeme_bytes,
    )?;
    let StrictJsonValue(json) = serde_json::from_slice::<StrictJsonValue>(&admitted.json_bytes)
        .map_err(|error| NickelPreservesError::MalformedJson(error.to_string()))?;
    let mut state = MappingState {
        limits: &profile.limits,
        total_values: 0,
    };
    let value = map_json_value(&json, ROOT_DEPTH, &mut state)?;
    let root = NickelJsonPreservesRoot {
        mapping_profile: profile.mapping_profile.clone(),
        value,
    };
    let canonical_artifact = encode_typed_preserves(
        &root,
        PreservesSchema {
            schema_version: &profile.preserves_schema_version,
            root_label: &profile.preserves_root_label,
        },
        preserves_limits(&profile.limits),
    )
    .map_err(|error| NickelPreservesError::Codec(error.to_string()))?;
    let profile_identity = AdapterIdentity::compute(profile_bytes);
    let json_projection_identity = ProjectionIdentity::compute(&admitted.json_bytes);
    let canonical_codec_identity = AdapterIdentity::compute(canonical_codec.as_bytes());
    let packed_preserves_identity =
        CanonicalValueIdentity::compute(&canonical_artifact.packed_bytes);
    let review_text_identity =
        ProjectionIdentity::compute(canonical_artifact.text_projection.as_bytes());
    let packed_output_bytes = u64::try_from(canonical_artifact.packed_bytes.len())
        .map_err(|_| NickelPreservesError::SizeOverflow)?;
    let body = NickelPreservesReceiptBody {
        schema_version: NICKEL_PRESERVES_RECEIPT_SCHEMA.to_string(),
        manifest_identity: admitted.manifest_identity.clone(),
        manifest_wire_identity: AdapterIdentity::from_wire(admitted.manifest_wire_identity.clone())
            .map_err(|_| {
                NickelPreservesError::UnsupportedProfile(
                    "manifest wire identity domain".to_string(),
                )
            })?,
        export_receipt_identity: admitted.export_receipt_identity.clone(),
        declared_input_identity: admitted.declared_input_identity.clone(),
        source_identity: admitted.source_identity.clone(),
        evaluator_label: admitted.evaluator_label.clone(),
        evaluator_version: admitted.evaluator_version.clone(),
        evaluator_artifact_identity: admitted.evaluator_artifact_identity.clone(),
        evaluation_plan_identity: admitted.evaluation_plan_identity.clone(),
        json_output_identity: admitted.json_output_identity.clone(),
        json_projection_identity,
        mapping_profile_identity: profile_identity.clone(),
        canonical_codec_identity,
        packed_preserves_identity: packed_preserves_identity.clone(),
        packed_output_bytes,
        review_text_identity: review_text_identity.clone(),
        checked_laws: profile.checked_laws.clone(),
        diagnostics: Vec::new(),
        upstream_non_claim: admitted.upstream_non_claim.clone(),
        non_claims: profile.non_claims.clone(),
    };
    let receipt_bytes = serde_json::to_vec(&body)
        .map_err(|error| NickelPreservesError::ReceiptEncoding(error.to_string()))?;
    let receipt = NickelPreservesReceipt {
        body,
        receipt_identity: ReceiptIdentity::compute(&receipt_bytes),
    };
    Ok(NickelPreservesAdaptation {
        profile,
        profile_identity,
        canonical_artifact,
        packed_preserves_identity,
        review_text_identity,
        receipt,
    })
}

fn parse_and_validate_profile(
    profile_bytes: &[u8],
    canonical_codec: &str,
) -> Result<NickelPreservesProfile, NickelPreservesError> {
    let profile = serde_json::from_slice::<NickelPreservesProfile>(profile_bytes)
        .map_err(|error| NickelPreservesError::MalformedProfile(error.to_string()))?;
    if profile.schema_version != NICKEL_PRESERVES_PROFILE_SCHEMA
        || profile.profile_id != NICKEL_PRESERVES_MAPPING_PROFILE
        || profile.mapping_profile != NICKEL_PRESERVES_MAPPING_PROFILE
        || profile.preserves_schema_version != NICKEL_PRESERVES_SCHEMA_VERSION
        || profile.preserves_root_label != NICKEL_PRESERVES_ROOT_LABEL
        || profile.canonical_codec != PRESERVES_CANONICAL_CODEC
        || canonical_codec != PRESERVES_CANONICAL_CODEC
        || profile.accepted_manifest_schemas != [NICKEL_EXPORT_MANIFEST_SCHEMA_V3]
        || profile.accepted_receipt_schemas != [NICKEL_EXPORT_RECEIPT_SCHEMA_V3]
        || profile.output_classes != string_values(NICKEL_PRESERVES_OUTPUT_CLASSES)
        || profile.numeric_policy != NICKEL_PRESERVES_NUMERIC_POLICY
        || profile.object_order != NICKEL_PRESERVES_OBJECT_ORDER
        || profile.unicode_policy != NICKEL_PRESERVES_UNICODE_POLICY
        || profile.checked_laws != string_values(NICKEL_PRESERVES_CHECKED_LAWS)
        || profile.non_claims != string_values(NICKEL_PRESERVES_NON_CLAIMS)
    {
        return Err(NickelPreservesError::UnsupportedProfile(
            "schema, mapping, codec, cohort, laws, or non-claims".to_string(),
        ));
    }
    validate_mapping_limits(&profile.limits)?;
    Ok(profile)
}

fn validate_mapping_limits(limits: &NickelPreservesLimits) -> Result<(), NickelPreservesError> {
    let fields = [
        ("max_input_bytes", limits.max_input_bytes),
        ("max_output_bytes", limits.max_output_bytes),
        ("max_depth", limits.max_depth),
        ("max_collection_items", limits.max_collection_items),
        ("max_total_values", limits.max_total_values),
        ("max_string_bytes", limits.max_string_bytes),
        ("max_numeric_lexeme_bytes", limits.max_numeric_lexeme_bytes),
    ];
    for (field, value) in fields {
        if value == 0 {
            return Err(NickelPreservesError::InvalidLimit(field));
        }
    }
    Ok(())
}

fn validate_admitted_export(
    admitted: &AdmittedNickelJsonExport,
) -> Result<(), NickelPreservesError> {
    if admitted.manifest_schema != NICKEL_EXPORT_MANIFEST_SCHEMA_V3
        || admitted.receipt_schema != NICKEL_EXPORT_RECEIPT_SCHEMA_V3
        || admitted.destination.is_empty()
        || admitted.evaluator_label.is_empty()
        || admitted.evaluator_version.is_empty()
        || admitted.upstream_non_claim.is_empty()
    {
        return Err(NickelPreservesError::UnsupportedProfile(
            "admitted export cohort".to_string(),
        ));
    }
    let identities = [
        (
            "manifest_identity",
            &admitted.manifest_identity,
            NickelExportIdentityKind::Manifest,
        ),
        (
            "export_receipt_identity",
            &admitted.export_receipt_identity,
            NickelExportIdentityKind::ExportReceipt,
        ),
        (
            "declared_input_identity",
            &admitted.declared_input_identity,
            NickelExportIdentityKind::DeclaredInput,
        ),
        (
            "source_identity",
            &admitted.source_identity,
            NickelExportIdentityKind::Source,
        ),
        (
            "evaluator_artifact_identity",
            &admitted.evaluator_artifact_identity,
            NickelExportIdentityKind::EvaluatorArtifact,
        ),
        (
            "evaluation_plan_identity",
            &admitted.evaluation_plan_identity,
            NickelExportIdentityKind::EvaluationPlan,
        ),
        (
            "json_output_identity",
            &admitted.json_output_identity,
            NickelExportIdentityKind::JsonOutput,
        ),
    ];
    for (field, identity, expected) in identities {
        validate_external_b3(&identity.b3)?;
        if identity.kind != expected {
            return Err(NickelPreservesError::IdentityKindMismatch {
                field,
                expected,
                actual: identity.kind,
            });
        }
    }
    if admitted.manifest_wire_identity.domain != IdentityDomain::Adapter {
        return Err(NickelPreservesError::UnsupportedProfile(
            "manifest wire identity domain".to_string(),
        ));
    }
    if admitted.json_output_identity.b3 != external_b3(&admitted.json_bytes) {
        return Err(NickelPreservesError::StaleJsonIdentity);
    }
    Ok(())
}

fn map_json_value(
    value: &Value,
    depth: usize,
    state: &mut MappingState<'_>,
) -> Result<NickelJsonPreservesValue, NickelPreservesError> {
    if depth > state.limits.max_depth {
        return Err(NickelPreservesError::DepthLimit {
            actual: depth,
            limit: state.limits.max_depth,
        });
    }
    state.total_values = state
        .total_values
        .checked_add(1)
        .ok_or(NickelPreservesError::SizeOverflow)?;
    if state.total_values > state.limits.max_total_values {
        return Err(NickelPreservesError::TotalValueLimit {
            actual: state.total_values,
            limit: state.limits.max_total_values,
        });
    }
    match value {
        Value::Null => Ok(NickelJsonPreservesValue::Null),
        Value::Bool(value) => Ok(NickelJsonPreservesValue::Boolean(*value)),
        Value::String(value) => {
            validate_string(value, state.limits)?;
            Ok(NickelJsonPreservesValue::String(value.clone()))
        }
        Value::Number(value) => map_number(value, state.limits),
        Value::Array(values) => {
            validate_collection(values.len(), state.limits)?;
            let next_depth = depth
                .checked_add(1)
                .ok_or(NickelPreservesError::SizeOverflow)?;
            values
                .iter()
                .map(|value| map_json_value(value, next_depth, state))
                .collect::<Result<Vec<_>, _>>()
                .map(NickelJsonPreservesValue::Array)
        }
        Value::Object(values) => {
            validate_collection(values.len(), state.limits)?;
            let next_depth = depth
                .checked_add(1)
                .ok_or(NickelPreservesError::SizeOverflow)?;
            let mut mapped = BTreeMap::new();
            for (key, value) in values {
                validate_string(key, state.limits)?;
                mapped.insert(key.clone(), map_json_value(value, next_depth, state)?);
            }
            Ok(NickelJsonPreservesValue::Object(mapped))
        }
    }
}

fn validate_numeric_lexeme_lengths(json: &[u8], limit: usize) -> Result<(), NickelPreservesError> {
    let mut index = 0;
    let mut in_string = false;
    let mut escaped = false;
    while index < json.len() {
        let byte = json[index];
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            index += 1;
            continue;
        }
        if byte == b'"' {
            in_string = true;
            index += 1;
            continue;
        }
        if byte == b'-' || byte.is_ascii_digit() {
            let start = index;
            index += 1;
            while index < json.len()
                && matches!(json[index], b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-')
            {
                index += 1;
            }
            let actual = index - start;
            if actual > limit {
                return Err(NickelPreservesError::NumericLimit { actual, limit });
            }
            continue;
        }
        index += 1;
    }
    Ok(())
}

fn map_number(
    number: &Number,
    limits: &NickelPreservesLimits,
) -> Result<NickelJsonPreservesValue, NickelPreservesError> {
    let lexeme = number.to_string();
    if lexeme.len() > limits.max_numeric_lexeme_bytes {
        return Err(NickelPreservesError::NumericLimit {
            actual: lexeme.len(),
            limit: limits.max_numeric_lexeme_bytes,
        });
    }
    if let Some(value) = number.as_i64() {
        return Ok(NickelJsonPreservesValue::SignedInteger(value));
    }
    if let Some(value) = number.as_u64() {
        return Ok(NickelJsonPreservesValue::UnsignedInteger(value));
    }
    Err(NickelPreservesError::UnsupportedNumber(lexeme))
}

fn validate_collection(
    actual: usize,
    limits: &NickelPreservesLimits,
) -> Result<(), NickelPreservesError> {
    if actual > limits.max_collection_items {
        Err(NickelPreservesError::CollectionLimit {
            actual,
            limit: limits.max_collection_items,
        })
    } else {
        Ok(())
    }
}

fn validate_string(
    value: &str,
    limits: &NickelPreservesLimits,
) -> Result<(), NickelPreservesError> {
    if value.len() > limits.max_string_bytes {
        Err(NickelPreservesError::StringLimit {
            actual: value.len(),
            limit: limits.max_string_bytes,
        })
    } else {
        Ok(())
    }
}

fn preserves_limits(limits: &NickelPreservesLimits) -> PreservesLimits {
    PreservesLimits {
        max_input_bytes: limits.max_input_bytes,
        max_output_bytes: limits.max_output_bytes,
        max_depth: limits.max_depth,
        max_collection_items: limits.max_collection_items,
        max_atom_bytes: limits.max_string_bytes,
        max_values: limits.max_total_values,
    }
}

fn validate_external_b3(identity: &str) -> Result<(), NickelPreservesError> {
    let Some(hex) = identity.strip_prefix(B3_PREFIX) else {
        return Err(NickelPreservesError::InvalidExternalIdentity(
            identity.to_string(),
        ));
    };
    if hex.len() != BLAKE3_HEX_BYTES
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(NickelPreservesError::InvalidExternalIdentity(
            identity.to_string(),
        ));
    }
    Ok(())
}

fn external_b3(bytes: &[u8]) -> String {
    format!("{B3_PREFIX}{}", blake3::hash(bytes).to_hex())
}

fn string_values(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_identity;

    const PROFILE_BYTES: &[u8] =
        include_bytes!("../../../adapters/nickel-preserves/generated/profile.json");
    const JSON_ONE: &[u8] = b"{\"z\":[null,true,\"\xce\xbb\"],\"a\":1}";
    const JSON_TWO: &[u8] = b"{ \"a\" : 1, \"z\" : [ null, true, \"\xce\xbb\" ] }";

    fn external(kind: NickelExportIdentityKind, bytes: &[u8]) -> NickelExportIdentity {
        NickelExportIdentity {
            kind,
            b3: external_b3(bytes),
        }
    }

    fn admitted(json_bytes: &[u8]) -> AdmittedNickelJsonExport {
        AdmittedNickelJsonExport {
            manifest_schema: NICKEL_EXPORT_MANIFEST_SCHEMA_V3.to_string(),
            receipt_schema: NICKEL_EXPORT_RECEIPT_SCHEMA_V3.to_string(),
            destination: "generated/value.json".to_string(),
            manifest_identity: external(NickelExportIdentityKind::Manifest, b"manifest"),
            manifest_wire_identity: compute_identity(IdentityDomain::Adapter, b"manifest wire"),
            export_receipt_identity: external(NickelExportIdentityKind::ExportReceipt, b"receipt"),
            declared_input_identity: external(
                NickelExportIdentityKind::DeclaredInput,
                b"declared input",
            ),
            source_identity: external(NickelExportIdentityKind::Source, b"source"),
            evaluator_label: "nixpkgs:nickel".to_string(),
            evaluator_version: "nickel-lang-cli-1.17.0".to_string(),
            evaluator_artifact_identity: external(
                NickelExportIdentityKind::EvaluatorArtifact,
                b"evaluator",
            ),
            evaluation_plan_identity: external(NickelExportIdentityKind::EvaluationPlan, b"plan"),
            json_output_identity: external(NickelExportIdentityKind::JsonOutput, json_bytes),
            json_bytes: json_bytes.to_vec(),
            upstream_non_claim: "upstream export evidence is bounded".to_string(),
        }
    }

    fn modified_profile(update: impl FnOnce(&mut NickelPreservesProfile)) -> Option<Vec<u8>> {
        let mut profile = serde_json::from_slice::<NickelPreservesProfile>(PROFILE_BYTES).ok()?;
        update(&mut profile);
        serde_json::to_vec(&profile).ok()
    }

    #[test]
    // r[verify kamacite.nickel_preserves_adapter.mapping_profile]
    // r[verify kamacite.nickel_preserves_adapter.canonical_output]
    // r[verify kamacite.nickel_preserves_adapter.receipt]
    // r[verify kamacite.nickel_preserves_adapter.non_claim]
    // r[verify kamacite.nickel_preserves_adapter.text_projection]
    fn equivalent_json_values_converge_to_one_canonical_identity() {
        let first = adapt_admitted_nickel_json_export(
            &admitted(JSON_ONE),
            PROFILE_BYTES,
            PRESERVES_CANONICAL_CODEC,
        );
        let second = adapt_admitted_nickel_json_export(
            &admitted(JSON_TWO),
            PROFILE_BYTES,
            PRESERVES_CANONICAL_CODEC,
        );
        assert!(first.is_ok());
        assert!(second.is_ok());
        if let (Ok(first), Ok(second)) = (first, second) {
            assert_ne!(
                first.receipt.body.json_projection_identity,
                second.receipt.body.json_projection_identity
            );
            assert_eq!(
                first.packed_preserves_identity,
                second.packed_preserves_identity
            );
            assert_eq!(
                first.canonical_artifact.packed_bytes,
                second.canonical_artifact.packed_bytes
            );
        }
    }

    #[test]
    fn maps_every_admitted_json_alternative() {
        const ALTERNATIVES: &[u8] = br#"[null,false,"text",-1,18446744073709551615,[1],{"key":2}]"#;
        let result = adapt_admitted_nickel_json_export(
            &admitted(ALTERNATIVES),
            PROFILE_BYTES,
            PRESERVES_CANONICAL_CODEC,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn enforces_every_named_mapping_limit() {
        let input = modified_profile(|profile| profile.limits.max_input_bytes = 1);
        assert!(input.is_some());
        if let Some(input) = input {
            assert!(matches!(
                adapt_admitted_nickel_json_export(
                    &admitted(br#"{"value":1}"#),
                    &input,
                    PRESERVES_CANONICAL_CODEC
                ),
                Err(NickelPreservesError::InputLimit { .. })
            ));
        }

        let output = modified_profile(|profile| profile.limits.max_output_bytes = 1);
        assert!(output.is_some());
        if let Some(output) = output {
            assert!(matches!(
                adapt_admitted_nickel_json_export(
                    &admitted(br#"{"value":1}"#),
                    &output,
                    PRESERVES_CANONICAL_CODEC
                ),
                Err(NickelPreservesError::Codec(_))
            ));
        }

        let depth = modified_profile(|profile| profile.limits.max_depth = 1);
        assert!(depth.is_some());
        if let Some(depth) = depth {
            assert!(matches!(
                adapt_admitted_nickel_json_export(
                    &admitted(br#"[[]]"#),
                    &depth,
                    PRESERVES_CANONICAL_CODEC
                ),
                Err(NickelPreservesError::DepthLimit { .. })
            ));
        }

        let collection = modified_profile(|profile| profile.limits.max_collection_items = 1);
        assert!(collection.is_some());
        if let Some(collection) = collection {
            assert!(matches!(
                adapt_admitted_nickel_json_export(
                    &admitted(br#"[1,2]"#),
                    &collection,
                    PRESERVES_CANONICAL_CODEC
                ),
                Err(NickelPreservesError::CollectionLimit { .. })
            ));
        }

        let total = modified_profile(|profile| profile.limits.max_total_values = 1);
        assert!(total.is_some());
        if let Some(total) = total {
            assert!(matches!(
                adapt_admitted_nickel_json_export(
                    &admitted(br#"[1]"#),
                    &total,
                    PRESERVES_CANONICAL_CODEC
                ),
                Err(NickelPreservesError::TotalValueLimit { .. })
            ));
        }

        let string = modified_profile(|profile| profile.limits.max_string_bytes = 1);
        assert!(string.is_some());
        if let Some(string) = string {
            assert!(matches!(
                adapt_admitted_nickel_json_export(
                    &admitted(br#""ab""#),
                    &string,
                    PRESERVES_CANONICAL_CODEC
                ),
                Err(NickelPreservesError::StringLimit { .. })
            ));
        }

        let numeric = modified_profile(|profile| profile.limits.max_numeric_lexeme_bytes = 1);
        assert!(numeric.is_some());
        if let Some(numeric) = numeric {
            assert!(matches!(
                adapt_admitted_nickel_json_export(
                    &admitted(b"10"),
                    &numeric,
                    PRESERVES_CANONICAL_CODEC
                ),
                Err(NickelPreservesError::NumericLimit { .. })
            ));
            assert!(
                adapt_admitted_nickel_json_export(
                    &admitted(br#""123 and \"456\"""#),
                    &numeric,
                    PRESERVES_CANONICAL_CODEC
                )
                .is_ok()
            );
        }
    }

    #[test]
    // r[verify kamacite.nickel_preserves_adapter.fixtures]
    fn rejects_malformed_numbers_stale_identity_wrong_kind_overclaims_and_limits() {
        assert!(matches!(
            adapt_admitted_nickel_json_export(
                &admitted(b"{"),
                PROFILE_BYTES,
                PRESERVES_CANONICAL_CODEC
            ),
            Err(NickelPreservesError::MalformedJson(_))
        ));
        assert!(matches!(
            adapt_admitted_nickel_json_export(
                &admitted(br#"{"key":1,"key":2}"#),
                PROFILE_BYTES,
                PRESERVES_CANONICAL_CODEC
            ),
            Err(NickelPreservesError::MalformedJson(error))
                if error.contains("duplicate JSON object key")
        ));
        assert!(matches!(
            adapt_admitted_nickel_json_export(
                &admitted(br#"{"value":1}"#),
                PROFILE_BYTES,
                "unsupported-codec"
            ),
            Err(NickelPreservesError::UnsupportedProfile(_))
        ));

        for invalid_number in [
            br#"{"value":1.5}"#.as_slice(),
            br#"{"value":1e3}"#.as_slice(),
            br#"{"value":18446744073709551616}"#.as_slice(),
            br#"{"value":-9223372036854775809}"#.as_slice(),
        ] {
            assert!(matches!(
                adapt_admitted_nickel_json_export(
                    &admitted(invalid_number),
                    PROFILE_BYTES,
                    PRESERVES_CANONICAL_CODEC
                ),
                Err(NickelPreservesError::UnsupportedNumber(_))
            ));
        }

        let mut stale = admitted(br#"{"value":1}"#);
        stale.json_output_identity.b3 = external_b3(b"other bytes");
        assert!(matches!(
            adapt_admitted_nickel_json_export(&stale, PROFILE_BYTES, PRESERVES_CANONICAL_CODEC),
            Err(NickelPreservesError::StaleJsonIdentity)
        ));

        let mut wrong_kind = admitted(br#"{"value":1}"#);
        wrong_kind.source_identity.kind = NickelExportIdentityKind::JsonOutput;
        assert!(matches!(
            adapt_admitted_nickel_json_export(
                &wrong_kind,
                PROFILE_BYTES,
                PRESERVES_CANONICAL_CODEC
            ),
            Err(NickelPreservesError::IdentityKindMismatch { .. })
        ));

        let unsupported_profiles = [
            modified_profile(|profile| profile.schema_version = "unsupported-schema".to_string()),
            modified_profile(|profile| profile.mapping_profile = "unsupported-mapping".to_string()),
        ];
        for unsupported in unsupported_profiles {
            assert!(unsupported.is_some());
            if let Some(unsupported) = unsupported {
                assert!(matches!(
                    adapt_admitted_nickel_json_export(
                        &admitted(br#"{"value":1}"#),
                        &unsupported,
                        PRESERVES_CANONICAL_CODEC
                    ),
                    Err(NickelPreservesError::UnsupportedProfile(_))
                ));
            }
        }

        let profile = serde_json::from_slice::<NickelPreservesProfile>(PROFILE_BYTES).ok();
        assert!(profile.is_some());
        if let Some(profile) = profile {
            let mut overclaim = profile.clone();
            overclaim
                .non_claims
                .push("proves arbitrary Nickel semantic equivalence".to_string());
            let overclaim_bytes = serde_json::to_vec(&overclaim);
            assert!(overclaim_bytes.is_ok());
            if let Ok(overclaim_bytes) = overclaim_bytes {
                assert!(matches!(
                    adapt_admitted_nickel_json_export(
                        &admitted(br#"{"value":1}"#),
                        &overclaim_bytes,
                        PRESERVES_CANONICAL_CODEC
                    ),
                    Err(NickelPreservesError::UnsupportedProfile(_))
                ));
            }

            let mut bounded = profile;
            bounded.limits.max_collection_items = 1;
            let bounded_bytes = serde_json::to_vec(&bounded);
            assert!(bounded_bytes.is_ok());
            if let Ok(bounded_bytes) = bounded_bytes {
                assert!(matches!(
                    adapt_admitted_nickel_json_export(
                        &admitted(br#"[1,2]"#),
                        &bounded_bytes,
                        PRESERVES_CANONICAL_CODEC
                    ),
                    Err(NickelPreservesError::CollectionLimit { .. })
                ));
            }
        }
    }
}
