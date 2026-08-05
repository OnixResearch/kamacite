use crate::{AdapterIdentity, WireIdentity};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::{self, Display};

pub const STEEL_DTO_INTEGER_SCHEMA: &str = "kamacite.dto.integer-v1";
pub const STEEL_DTO_MODULE_OUTPUT_SCHEMA: &str = "kamacite.dto.module-output-v1";
pub const STEEL_DTO_CALLABLE_DESCRIPTOR_SCHEMA: &str = "kamacite.dto.callable-descriptor-v1";
pub const STEEL_DTO_BYTECODE_SCHEMA: &str = "kamacite.dto.bytecode-v1";
pub const STEEL_EXECUTED_PROBE_KIND: &str = "executed-probe";
pub const STEEL_STORED_EVIDENCE_KIND: &str = "stored-evidence";
pub const STEEL_SUCCESS_EXIT_CLASS: &str = "success";

const STEEL_DTO_CONTRACT_VERSION: u32 = 1;
const INTEGER_VALUE_FIELD: &str = "value";
const MODULE_FIELD: &str = "module";
const CALLABLE_FIELD: &str = "callable";
const BYTECODE_IDENTITY_FIELD: &str = "bytecode_identity";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SteelDtoShape {
    IntegerRecord,
    ModuleOutput,
    CallableDescriptor,
    BytecodeDescriptor,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteelDtoContractBody {
    pub schema_version: u32,
    pub schema_id: String,
    pub shape: SteelDtoShape,
    pub required_fields: Vec<String>,
    pub additional_fields: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteelDtoContract {
    pub body: SteelDtoContractBody,
    pub identity: AdapterIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SteelDtoValue {
    Integer(i64),
    ModuleOutput { module: String, value: i64 },
    CallableDescriptor(String),
    BytecodeDescriptor(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SteelDtoViolation {
    UnknownSchema(String),
    MalformedJson(String),
    WrongShape { schema_id: String, detail: String },
    UnsupportedExecutionOutput(String),
    MalformedModuleDescriptor(String),
}

impl Display for SteelDtoViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSchema(schema_id) => {
                write!(formatter, "unknown Steel DTO schema {schema_id:?}")
            }
            Self::MalformedJson(error) => write!(formatter, "malformed Steel DTO JSON: {error}"),
            Self::WrongShape { schema_id, detail } => {
                write!(
                    formatter,
                    "Steel DTO does not satisfy {schema_id:?}: {detail}"
                )
            }
            Self::UnsupportedExecutionOutput(schema_id) => write!(
                formatter,
                "Steel execution output schema {schema_id:?} is not executable by this profile"
            ),
            Self::MalformedModuleDescriptor(error) => {
                write!(formatter, "malformed Steel module descriptor: {error}")
            }
        }
    }
}

impl std::error::Error for SteelDtoViolation {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SteelExecutionEvidence {
    StoredEvidence {
        fresh: bool,
        output_identity: Option<WireIdentity>,
    },
    ExecutedProbe(Box<SteelExecutedProbeEvidence>),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SteelExecutedProbeEvidence {
    pub runner_profile: String,
    pub runner_descriptor: String,
    pub runner_identity: WireIdentity,
    pub source_identity: WireIdentity,
    pub module_identity: WireIdentity,
    pub input_identity: WireIdentity,
    pub input_contract_identity: WireIdentity,
    pub output_contract_identity: WireIdentity,
    pub output_identity: WireIdentity,
    pub declared_builtin_modules: Vec<String>,
    pub registered_host_functions: Vec<String>,
    pub observed_host_functions: Vec<String>,
    pub enforced_capabilities: Vec<String>,
    pub observed_effects: Vec<String>,
    pub exit_class: String,
    pub output_byte_length: u64,
    pub authority_enforced: bool,
    pub completed: bool,
}

// r[impl kamacite.findings.steel_execution.schema_conformance]
pub fn resolve_steel_dto_contract(schema_id: &str) -> Result<SteelDtoContract, SteelDtoViolation> {
    let (shape, required_fields) = match schema_id {
        STEEL_DTO_INTEGER_SCHEMA => (
            SteelDtoShape::IntegerRecord,
            vec![INTEGER_VALUE_FIELD.to_string()],
        ),
        STEEL_DTO_MODULE_OUTPUT_SCHEMA => (
            SteelDtoShape::ModuleOutput,
            vec![MODULE_FIELD.to_string(), INTEGER_VALUE_FIELD.to_string()],
        ),
        STEEL_DTO_CALLABLE_DESCRIPTOR_SCHEMA => (
            SteelDtoShape::CallableDescriptor,
            vec![CALLABLE_FIELD.to_string()],
        ),
        STEEL_DTO_BYTECODE_SCHEMA => (
            SteelDtoShape::BytecodeDescriptor,
            vec![BYTECODE_IDENTITY_FIELD.to_string()],
        ),
        _ => return Err(SteelDtoViolation::UnknownSchema(schema_id.to_string())),
    };
    let body = SteelDtoContractBody {
        schema_version: STEEL_DTO_CONTRACT_VERSION,
        schema_id: schema_id.to_string(),
        shape,
        required_fields,
        additional_fields: false,
    };
    let bytes = serde_json::to_vec(&body)
        .map_err(|error| SteelDtoViolation::MalformedJson(error.to_string()))?;
    let identity = AdapterIdentity::compute(&bytes);
    Ok(SteelDtoContract { body, identity })
}

pub fn validate_steel_dto(
    schema_id: &str,
    bytes: &[u8],
) -> Result<SteelDtoValue, SteelDtoViolation> {
    let contract = resolve_steel_dto_contract(schema_id)?;
    let value = serde_json::from_slice::<Value>(bytes)
        .map_err(|error| SteelDtoViolation::MalformedJson(error.to_string()))?;
    let object = value
        .as_object()
        .ok_or_else(|| SteelDtoViolation::WrongShape {
            schema_id: schema_id.to_string(),
            detail: "expected a JSON object".to_string(),
        })?;
    if object.len() != contract.body.required_fields.len() {
        return Err(SteelDtoViolation::WrongShape {
            schema_id: schema_id.to_string(),
            detail: "object fields do not exactly match the closed contract".to_string(),
        });
    }
    match contract.body.shape {
        SteelDtoShape::IntegerRecord => {
            integer_field(object, schema_id).map(SteelDtoValue::Integer)
        }
        SteelDtoShape::ModuleOutput => {
            let module = object
                .get(MODULE_FIELD)
                .and_then(Value::as_str)
                .filter(|module| !module.is_empty())
                .ok_or_else(|| SteelDtoViolation::WrongShape {
                    schema_id: schema_id.to_string(),
                    detail: "module must be a nonempty string".to_string(),
                })?;
            let value = integer_field(object, schema_id)?;
            Ok(SteelDtoValue::ModuleOutput {
                module: module.to_string(),
                value,
            })
        }
        SteelDtoShape::CallableDescriptor => {
            string_field(object, schema_id, CALLABLE_FIELD).map(SteelDtoValue::CallableDescriptor)
        }
        SteelDtoShape::BytecodeDescriptor => {
            string_field(object, schema_id, BYTECODE_IDENTITY_FIELD)
                .map(SteelDtoValue::BytecodeDescriptor)
        }
    }
}

pub fn render_steel_probe_output(
    schema_id: &str,
    value: i64,
    module_descriptor_bytes: &[u8],
) -> Result<Vec<u8>, SteelDtoViolation> {
    let output = match schema_id {
        STEEL_DTO_INTEGER_SCHEMA => {
            let mut output = BTreeMap::new();
            output.insert(INTEGER_VALUE_FIELD, Value::from(value));
            output
        }
        STEEL_DTO_MODULE_OUTPUT_SCHEMA => {
            let descriptor = serde_json::from_slice::<Value>(module_descriptor_bytes)
                .map_err(|error| SteelDtoViolation::MalformedModuleDescriptor(error.to_string()))?;
            let module = descriptor
                .get(MODULE_FIELD)
                .and_then(Value::as_str)
                .filter(|module| !module.is_empty())
                .ok_or_else(|| {
                    SteelDtoViolation::MalformedModuleDescriptor(
                        "module field must be a nonempty string".to_string(),
                    )
                })?;
            let mut output = BTreeMap::new();
            output.insert(MODULE_FIELD, Value::from(module));
            output.insert(INTEGER_VALUE_FIELD, Value::from(value));
            output
        }
        _ => {
            return Err(SteelDtoViolation::UnsupportedExecutionOutput(
                schema_id.to_string(),
            ));
        }
    };
    serde_json::to_vec(&output).map_err(|error| SteelDtoViolation::MalformedJson(error.to_string()))
}

fn integer_field(
    object: &serde_json::Map<String, Value>,
    schema_id: &str,
) -> Result<i64, SteelDtoViolation> {
    object
        .get(INTEGER_VALUE_FIELD)
        .and_then(Value::as_i64)
        .ok_or_else(|| SteelDtoViolation::WrongShape {
            schema_id: schema_id.to_string(),
            detail: "value must be an exactly represented signed integer".to_string(),
        })
}

fn string_field(
    object: &serde_json::Map<String, Value>,
    schema_id: &str,
    field: &str,
) -> Result<String, SteelDtoViolation> {
    object
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .ok_or_else(|| SteelDtoViolation::WrongShape {
            schema_id: schema_id.to_string(),
            detail: format!("{field} must be a nonempty string"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // r[verify kamacite.findings.steel_execution.schema_conformance]
    fn validates_closed_integer_and_module_contracts() {
        let integer = validate_steel_dto(STEEL_DTO_INTEGER_SCHEMA, br#"{"value":2}"#);
        assert_eq!(integer, Ok(SteelDtoValue::Integer(2)));
        let module = validate_steel_dto(
            STEEL_DTO_MODULE_OUTPUT_SCHEMA,
            br#"{"module":"kamacite.dto.probe","value":2}"#,
        );
        assert_eq!(
            module,
            Ok(SteelDtoValue::ModuleOutput {
                module: "kamacite.dto.probe".to_string(),
                value: 2,
            })
        );
    }

    #[test]
    // r[verify kamacite.findings.steel_execution.schema_conformance]
    fn rejects_wrong_shape_unknown_schema_and_extra_fields() {
        assert!(matches!(
            validate_steel_dto(STEEL_DTO_INTEGER_SCHEMA, br#"{"value":{},"extra":1}"#),
            Err(SteelDtoViolation::WrongShape { .. })
        ));
        assert!(matches!(
            validate_steel_dto("kamacite.dto.unknown-v1", br#"{"value":1}"#),
            Err(SteelDtoViolation::UnknownSchema(_))
        ));
    }

    #[test]
    fn contract_identity_changes_with_schema() {
        let integer = resolve_steel_dto_contract(STEEL_DTO_INTEGER_SCHEMA);
        let module = resolve_steel_dto_contract(STEEL_DTO_MODULE_OUTPUT_SCHEMA);
        assert!(integer.is_ok());
        assert!(module.is_ok());
        if let (Ok(integer), Ok(module)) = (integer, module) {
            assert_ne!(integer.identity, module.identity);
            assert_eq!(integer.identity.domain(), crate::IdentityDomain::Adapter);
        }
    }
}
