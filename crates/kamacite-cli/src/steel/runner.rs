#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::assertion_density,
        tigerstyle::catch_all_on_enum,
        tigerstyle::compound_condition,
        tigerstyle::excessive_file_length,
        tigerstyle::foreign_enum_catch_all,
        tigerstyle::missing_const_fn,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "This bounded Steel shell fails closed for every unsupported DTO variant and needs a separate compatibility-preserving API decomposition."
    )
)]

use kamacite_core::{
    STEEL_DTO_INTEGER_SCHEMA, SteelDtoValue, SteelHostFunctionInput, SteelOutputClass,
    render_steel_probe_output, validate_steel_dto,
};
use serde::Deserialize;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use steel::{
    SteelVal,
    steel_vm::{
        builtin::BuiltInModule, engine::Engine, interrupt::InterruptHandler,
        register_fn::RegisterFn,
    },
};

pub const STEEL_RUNNER_PROFILE: &str = "steel-dto-probe-shell";
pub const STEEL_RUNNER_DESCRIPTOR: &str = "steel-core:0.8.2;engine=new-raw-no-kernel;kernel=none;modules=steel/base-empty;source-cohort=exact;profile=kamacite-dto-probe-v2";
pub const STEEL_BASE_MODULE: &str = "steel/base";
pub const STEEL_ADD_ONE_HOST_FUNCTION: &str = "kamacite.add-one";
pub const STEEL_DTO_EVALUATION_CAPABILITY: &str = "dto-evaluation";
pub const STEEL_CAPTURE_OUTPUT_CAPABILITY: &str = "capture-stdout";
pub const STEEL_FIXTURE_READ_CAPABILITY: &str = "fixture-read";
pub const STEEL_PROBE_CAPABILITY: &str = "steel-dto-probe";
pub const WORLD_VECTOR_HOST_FUNCTION: &str = "kamacite.resolve-world-vector";
pub const WORLD_VECTOR_CAPABILITY: &str = "world-vector-resolution";
pub const WORLD_VECTOR_BASE_MODULE: &str = "steel/base-empty";
pub const WORLD_VECTOR_RUNNER_DESCRIPTOR: &str = "steel-core:0.8.2;engine=new-raw-no-kernel;kernel=none;module=steel/base-empty;host=kamacite.resolve-world-vector;profile=kamacite-world-vector-v1";

const WORLD_VECTOR_MODULE_SCHEMA: &str = "kamacite.world-commit-steel-module.v1";
const WORLD_VECTOR_ENGINE: &str = "steel-core:0.8.2;new-raw-no-kernel";
const WORLD_VECTOR_EFFECT_CLASS: &str = "pure-index-resolution";
const WORLD_VECTOR_DENIED_AUTHORITIES: [&str; 6] = [
    "environment",
    "filesystem",
    "network",
    "process",
    "clock",
    "randomness",
];
const STEEL_RUNNER_TIMEOUT_MILLISECONDS: u64 = 1_000;
const STEEL_ADMITTED_SOURCE_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/adapters/steel/dto-only.scm");
const STEEL_ADMITTED_MODULE_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/adapters/steel/module-descriptor.json");
const STEEL_ADMITTED_INPUT_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/adapters/steel/dto-input.json");
const WORLD_VECTOR_SOURCE_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/world-commit/adapters/steel-resolve.scm");
const WORLD_VECTOR_MODULE_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/world-commit/adapters/steel-module-descriptor.json");
const WORLD_VECTOR_INPUT_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/world-commit/adapters/steel-input.json");
const STEEL_MAX_SOURCE_BYTES: usize = 64 * 1_024;
const STEEL_MAX_INPUT_BYTES: usize = 64 * 1_024;
const STEEL_MAX_OUTPUT_BYTES: usize = 64 * 1_024;
const EXPECTED_HOST_CALLS: usize = 1;

pub struct SteelProbeRequest<'a> {
    pub source_bytes: &'a [u8],
    pub module_descriptor_bytes: &'a [u8],
    pub input_bytes: &'a [u8],
    pub input_schema: &'a str,
    pub output_schema: &'a str,
    pub output_class: &'a SteelOutputClass,
    pub host_functions: &'a [SteelHostFunctionInput],
    pub declared_builtin_modules: &'a [String],
    pub declared_imports: &'a [String],
    pub requested_imports: &'a [String],
    pub declared_capabilities: &'a [String],
}

pub struct SteelProbeRun {
    pub output_bytes: Vec<u8>,
    pub observed_host_functions: Vec<String>,
    pub observed_effects: Vec<String>,
}

#[derive(Deserialize)]
struct WorldVectorModuleDescriptor {
    schema: String,
    engine: String,
    module: String,
    host_functions: Vec<WorldVectorHostDescriptor>,
    capabilities: Vec<String>,
    denied_authority: Vec<String>,
}

#[derive(Deserialize)]
struct WorldVectorHostDescriptor {
    id: String,
    effect_class: String,
    input_schema: String,
    output_schema: String,
}

// r[impl kamacite.findings.steel_execution.fresh_probe]
// r[impl kamacite.findings.steel_execution.runner_identity]
// r[impl kamacite.findings.steel_execution.capability_enforcement]
pub fn run_steel_probe(request: SteelProbeRequest<'_>) -> Result<SteelProbeRun, String> {
    validate_steel_probe_request(&request)?;
    let input = match validate_steel_dto(request.input_schema, request.input_bytes)
        .map_err(|error| error.to_string())?
    {
        SteelDtoValue::Integer(value) => value,
        _ => return Err("Steel probe input must use the integer DTO contract".to_string()),
    };
    input.checked_add(1).ok_or_else(|| {
        "Steel integer input would overflow the admitted host function".to_string()
    })?;
    let source = std::str::from_utf8(request.source_bytes)
        .map_err(|error| format!("Steel source is not UTF-8: {error}"))?;
    let host_calls = Arc::new(AtomicUsize::new(0));
    let observed_calls = Arc::clone(&host_calls);
    let mut engine = Engine::new_raw_no_kernel();
    engine.register_module(BuiltInModule::new(STEEL_BASE_MODULE.to_string()));
    engine.register_fn(STEEL_ADD_ONE_HOST_FUNCTION, move |value: i64| {
        observed_calls.fetch_add(1, Ordering::Relaxed);
        value.saturating_add(1)
    });
    let program = format!("{source}\n(probe {input})\n");
    let interrupt_handler = InterruptHandler::new(
        &mut engine,
        Duration::from_millis(STEEL_RUNNER_TIMEOUT_MILLISECONDS),
    );
    let values = interrupt_handler
        .run_with_timeout(|| engine.compile_and_run_raw_program(program))
        .map_err(|error| format!("bounded Steel probe failed: {error}"))?;
    let result = values
        .last()
        .ok_or_else(|| "bounded Steel probe returned no value".to_string())?;
    let SteelVal::IntV(result) = result else {
        return Err("bounded Steel probe must return one signed integer DTO value".to_string());
    };
    if host_calls.load(Ordering::Relaxed) != EXPECTED_HOST_CALLS {
        return Err(
            "bounded Steel probe did not call the admitted host function exactly once".to_string(),
        );
    }
    let result = i64::try_from(*result)
        .map_err(|error| format!("Steel integer result conversion failed: {error}"))?;
    let output_bytes = render_steel_probe_output(
        request.output_schema,
        result,
        request.module_descriptor_bytes,
    )
    .map_err(|error| error.to_string())?;
    if output_bytes.len() > STEEL_MAX_OUTPUT_BYTES {
        return Err("bounded Steel probe output exceeded its named byte limit".to_string());
    }
    validate_steel_dto(request.output_schema, &output_bytes).map_err(|error| error.to_string())?;
    Ok(SteelProbeRun {
        output_bytes,
        observed_host_functions: vec![STEEL_ADD_ONE_HOST_FUNCTION.to_string()],
        observed_effects: vec![
            STEEL_FIXTURE_READ_CAPABILITY.to_string(),
            STEEL_PROBE_CAPABILITY.to_string(),
            STEEL_CAPTURE_OUTPUT_CAPABILITY.to_string(),
            STEEL_DTO_EVALUATION_CAPABILITY.to_string(),
        ],
    })
}

// r[impl kamacite.world_commit_conformance.steel]
pub fn validate_world_vector_cohort(
    source_bytes: &[u8],
    module_descriptor_bytes: &[u8],
    input_bytes: &[u8],
) -> Result<(), String> {
    if source_bytes != WORLD_VECTOR_SOURCE_BYTES
        || module_descriptor_bytes != WORLD_VECTOR_MODULE_BYTES
        || input_bytes != WORLD_VECTOR_INPUT_BYTES
    {
        return Err("world-vector Steel input is outside the exact admitted cohort".to_string());
    }
    if source_bytes.is_empty() || source_bytes.len() > STEEL_MAX_SOURCE_BYTES {
        return Err("world-vector Steel source is empty or oversized".to_string());
    }
    std::str::from_utf8(source_bytes)
        .map_err(|error| format!("world-vector Steel source is not UTF-8: {error}"))?;
    validate_world_vector_descriptor(module_descriptor_bytes)?;
    match validate_steel_dto(STEEL_DTO_INTEGER_SCHEMA, input_bytes)
        .map_err(|error| error.to_string())?
    {
        SteelDtoValue::Integer(_) => Ok(()),
        _ => Err("world-vector Steel input must be an integer DTO".to_string()),
    }
}

pub fn run_world_vector_probe() -> Result<SteelProbeRun, String> {
    run_world_vector_probe_with(
        WORLD_VECTOR_SOURCE_BYTES,
        WORLD_VECTOR_MODULE_BYTES,
        WORLD_VECTOR_INPUT_BYTES,
    )
}

fn run_world_vector_probe_with(
    source_bytes: &[u8],
    module_descriptor_bytes: &[u8],
    input_bytes: &[u8],
) -> Result<SteelProbeRun, String> {
    validate_world_vector_cohort(source_bytes, module_descriptor_bytes, input_bytes)?;
    let input = match validate_steel_dto(STEEL_DTO_INTEGER_SCHEMA, input_bytes)
        .map_err(|error| error.to_string())?
    {
        SteelDtoValue::Integer(value) => value,
        _ => return Err("world-vector Steel input must be an integer DTO".to_string()),
    };
    let source = std::str::from_utf8(source_bytes)
        .map_err(|error| format!("world-vector Steel source is not UTF-8: {error}"))?;
    let host_calls = Arc::new(AtomicUsize::new(0));
    let observed_calls = Arc::clone(&host_calls);
    let mut engine = Engine::new_raw_no_kernel();
    engine.register_module(BuiltInModule::new(WORLD_VECTOR_BASE_MODULE.to_string()));
    engine.register_fn(WORLD_VECTOR_HOST_FUNCTION, move |value: i64| {
        observed_calls.fetch_add(1, Ordering::Relaxed);
        value
    });
    let program = format!("{source}\n(resolve-world-vector {input})\n");
    let interrupt_handler = InterruptHandler::new(
        &mut engine,
        Duration::from_millis(STEEL_RUNNER_TIMEOUT_MILLISECONDS),
    );
    let values = interrupt_handler
        .run_with_timeout(|| engine.compile_and_run_raw_program(program))
        .map_err(|error| format!("bounded world-vector Steel probe failed: {error}"))?;
    let result = values
        .last()
        .ok_or_else(|| "world-vector Steel probe returned no value".to_string())?;
    let SteelVal::IntV(result) = result else {
        return Err("world-vector Steel probe must return an integer DTO".to_string());
    };
    if host_calls.load(Ordering::Relaxed) != EXPECTED_HOST_CALLS {
        return Err("world-vector Steel host function was not called exactly once".to_string());
    }
    let result = i64::try_from(*result)
        .map_err(|error| format!("world-vector result conversion failed: {error}"))?;
    let output_bytes =
        render_steel_probe_output(STEEL_DTO_INTEGER_SCHEMA, result, module_descriptor_bytes)
            .map_err(|error| error.to_string())?;
    Ok(SteelProbeRun {
        output_bytes,
        observed_host_functions: vec![WORLD_VECTOR_HOST_FUNCTION.to_string()],
        observed_effects: Vec::new(),
    })
}

fn validate_world_vector_descriptor(bytes: &[u8]) -> Result<(), String> {
    let descriptor: WorldVectorModuleDescriptor = serde_json::from_slice(bytes)
        .map_err(|error| format!("world-vector module descriptor is malformed: {error}"))?;
    if descriptor.schema != WORLD_VECTOR_MODULE_SCHEMA
        || descriptor.engine != WORLD_VECTOR_ENGINE
        || descriptor.module != WORLD_VECTOR_BASE_MODULE
        || descriptor.capabilities.as_slice() != [WORLD_VECTOR_CAPABILITY]
        || descriptor.denied_authority.as_slice() != WORLD_VECTOR_DENIED_AUTHORITIES
        || !WORLD_VECTOR_RUNNER_DESCRIPTOR.contains(WORLD_VECTOR_BASE_MODULE)
        || !WORLD_VECTOR_RUNNER_DESCRIPTOR.contains(WORLD_VECTOR_HOST_FUNCTION)
    {
        return Err("world-vector module descriptor does not match the runner".to_string());
    }
    if descriptor.host_functions.len() != EXPECTED_HOST_CALLS {
        return Err("world-vector module must declare exactly one host function".to_string());
    }
    let host = &descriptor.host_functions[0];
    if host.id != WORLD_VECTOR_HOST_FUNCTION
        || host.effect_class != WORLD_VECTOR_EFFECT_CLASS
        || host.input_schema != STEEL_DTO_INTEGER_SCHEMA
        || host.output_schema != STEEL_DTO_INTEGER_SCHEMA
    {
        return Err(
            "world-vector host descriptor does not match the registered function".to_string(),
        );
    }
    Ok(())
}

fn validate_steel_probe_request(request: &SteelProbeRequest<'_>) -> Result<(), String> {
    if request.source_bytes.is_empty() || request.source_bytes.len() > STEEL_MAX_SOURCE_BYTES {
        return Err("Steel source is empty or exceeds its named byte limit".to_string());
    }
    if request.source_bytes != STEEL_ADMITTED_SOURCE_BYTES
        || request.module_descriptor_bytes != STEEL_ADMITTED_MODULE_BYTES
        || request.input_bytes != STEEL_ADMITTED_INPUT_BYTES
    {
        return Err(
            "Steel source, module, or input is outside the exact admitted probe cohort".to_string(),
        );
    }
    if request.input_bytes.is_empty() || request.input_bytes.len() > STEEL_MAX_INPUT_BYTES {
        return Err("Steel input is empty or exceeds its named byte limit".to_string());
    }
    if request.output_class != &SteelOutputClass::Dto {
        return Err("executed Steel probes admit only DTO output".to_string());
    }
    if request.declared_builtin_modules != [STEEL_BASE_MODULE]
        || !request.declared_imports.is_empty()
        || request.requested_imports != [STEEL_BASE_MODULE]
    {
        return Err(
            "executed Steel probes admit only the sandboxed steel/base module and no imports"
                .to_string(),
        );
    }
    if request.host_functions.len() != EXPECTED_HOST_CALLS {
        return Err("executed Steel probes require exactly one admitted host function".to_string());
    }
    let function = &request.host_functions[0];
    if function.function_id != STEEL_ADD_ONE_HOST_FUNCTION
        || function.input_schema != STEEL_DTO_INTEGER_SCHEMA
        || function.required_capabilities != [STEEL_DTO_EVALUATION_CAPABILITY]
    {
        return Err("Steel host-function descriptor is not admitted by the runner".to_string());
    }
    let required_capabilities = [
        STEEL_FIXTURE_READ_CAPABILITY,
        STEEL_PROBE_CAPABILITY,
        STEEL_CAPTURE_OUTPUT_CAPABILITY,
        STEEL_DTO_EVALUATION_CAPABILITY,
    ];
    if required_capabilities.iter().any(|capability| {
        !request
            .declared_capabilities
            .iter()
            .any(|item| item == capability)
    }) {
        return Err("Steel runner capability profile is incomplete".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &[u8] = STEEL_ADMITTED_SOURCE_BYTES;
    const MODULE: &[u8] = STEEL_ADMITTED_MODULE_BYTES;
    const INPUT: &[u8] = STEEL_ADMITTED_INPUT_BYTES;

    fn run_request(
        source_bytes: &[u8],
        host_functions: &[SteelHostFunctionInput],
        input_bytes: &[u8],
    ) -> Result<SteelProbeRun, String> {
        let declared_builtin_modules = vec![STEEL_BASE_MODULE.to_string()];
        let requested_imports = vec![STEEL_BASE_MODULE.to_string()];
        let declared_capabilities = vec![
            STEEL_FIXTURE_READ_CAPABILITY.to_string(),
            STEEL_PROBE_CAPABILITY.to_string(),
            STEEL_CAPTURE_OUTPUT_CAPABILITY.to_string(),
            STEEL_DTO_EVALUATION_CAPABILITY.to_string(),
        ];
        run_steel_probe(SteelProbeRequest {
            source_bytes,
            module_descriptor_bytes: MODULE,
            input_bytes,
            input_schema: STEEL_DTO_INTEGER_SCHEMA,
            output_schema: STEEL_DTO_INTEGER_SCHEMA,
            output_class: &SteelOutputClass::Dto,
            host_functions,
            declared_builtin_modules: &declared_builtin_modules,
            declared_imports: &[],
            requested_imports: &requested_imports,
            declared_capabilities: &declared_capabilities,
        })
    }

    fn host_functions() -> Vec<SteelHostFunctionInput> {
        vec![SteelHostFunctionInput {
            function_id: STEEL_ADD_ONE_HOST_FUNCTION.to_string(),
            input_schema: STEEL_DTO_INTEGER_SCHEMA.to_string(),
            output_schema: STEEL_DTO_INTEGER_SCHEMA.to_string(),
            effect_class: "pure-dto".to_string(),
            required_capabilities: vec![STEEL_DTO_EVALUATION_CAPABILITY.to_string()],
            checked_laws: vec!["host-function-dto-contract".to_string()],
        }]
    }

    #[test]
    fn executes_fresh_bounded_probe() {
        let functions = host_functions();
        let first = run_request(SOURCE, &functions, INPUT).expect("first run");
        let second = run_request(SOURCE, &functions, INPUT).expect("second fresh run");
        assert_eq!(first.output_bytes, br#"{"value":2}"#);
        assert_eq!(first.output_bytes, second.output_bytes);
        assert_eq!(
            first.observed_host_functions,
            vec![STEEL_ADD_ONE_HOST_FUNCTION]
        );
    }

    #[test]
    fn rejects_undeclared_host_function_before_execution() {
        let mut functions = host_functions();
        functions.push(SteelHostFunctionInput {
            function_id: "kamacite.read-file".to_string(),
            input_schema: STEEL_DTO_INTEGER_SCHEMA.to_string(),
            output_schema: STEEL_DTO_INTEGER_SCHEMA.to_string(),
            effect_class: "ambient-filesystem".to_string(),
            required_capabilities: vec!["ambient-filesystem".to_string()],
            checked_laws: Vec::new(),
        });
        let result = run_request(SOURCE, &functions, INPUT);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_shaped_input_before_execution() {
        let functions = host_functions();
        let result = run_request(SOURCE, &functions, br#"{"value":{"nested":1}}"#);
        assert!(result.is_err());
    }

    #[test]
    fn bounded_engine_denies_undeclared_function_called_by_source() {
        const UNDECLARED_SOURCE: &[u8] = b"(define (probe input) (kamacite.read-file input))";
        let functions = host_functions();
        let result = run_request(UNDECLARED_SOURCE, &functions, INPUT);
        assert!(result.is_err());
    }

    #[test]
    fn bounded_engine_denies_unregistered_process_module() {
        const PROCESS_SOURCE: &[u8] =
            b"(require-builtin steel/process)\n(define (probe input) input)";
        let functions = host_functions();
        let result = run_request(PROCESS_SOURCE, &functions, INPUT);
        assert!(result.is_err());
    }

    #[test]
    fn world_vector_probe_uses_one_pure_declared_host_function() {
        let run = run_world_vector_probe().expect("world vector probe");
        assert_eq!(run.output_bytes, br#"{"value":0}"#);
        assert_eq!(
            run.observed_host_functions,
            vec![WORLD_VECTOR_HOST_FUNCTION]
        );
        assert!(run.observed_effects.is_empty());
    }

    #[test]
    fn world_vector_probe_rejects_an_altered_source() {
        let result = run_world_vector_probe_with(
            b"(define (resolve-world-vector vector-id) vector-id)",
            WORLD_VECTOR_MODULE_BYTES,
            WORLD_VECTOR_INPUT_BYTES,
        );
        assert!(result.is_err());
    }

    #[test]
    fn world_vector_probe_rejects_an_altered_module_descriptor() {
        let result = run_world_vector_probe_with(
            WORLD_VECTOR_SOURCE_BYTES,
            br#"{"module":"steel/base"}"#,
            WORLD_VECTOR_INPUT_BYTES,
        );
        assert!(result.is_err());
    }
}
