#![forbid(unsafe_code)]

mod root_io;

use kamacite_core::{
    ApplicationHostBindingError, ApplicationHostBindingReceipt,
    ApplicationHostBindingValenceProjection, ApplicationHostBindingValidation,
    ApplicationHostBindingValidationInput, ApplicationHostBindingVerdict,
    build_application_host_binding_receipt, project_application_host_binding_to_valence,
};
use root_io::{RootCapability, RootIoError};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const DEFAULT_ROOT: &str = ".";
const VALIDATE_COMMAND: &str = "validate";

#[derive(Debug)]
struct CliError(String);

impl Display for CliError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl From<RootIoError> for CliError {
    fn from(error: RootIoError) -> Self {
        Self(error.to_string())
    }
}

impl From<ApplicationHostBindingError> for CliError {
    fn from(error: ApplicationHostBindingError) -> Self {
        Self(error.to_string())
    }
}

#[derive(Clone, Debug)]
struct Args {
    root: PathBuf,
    input: PathBuf,
    receipt_output: Option<PathBuf>,
    projection_output: Option<PathBuf>,
    update: bool,
}

#[derive(Debug, Serialize)]
struct ApplicationHostBindingCliReport {
    passed: bool,
    validation: ApplicationHostBindingValidation,
    receipt: ApplicationHostBindingReceipt,
    valence_projection: Option<ApplicationHostBindingValenceProjection>,
    mutation_performed: bool,
}

fn main() {
    let outcome = parse_args(std::env::args().skip(1).collect()).and_then(run);
    let code = match outcome {
        Ok((code, report)) => match serde_json::to_string_pretty(&report) {
            Ok(rendered) => {
                println!("{rendered}");
                code
            }
            Err(error) => {
                eprintln!("application-host binding report serialization failed: {error}");
                EXIT_FAILURE
            }
        },
        Err(error) => {
            eprintln!("{error}");
            EXIT_FAILURE
        }
    };
    std::process::exit(code);
}

fn parse_args(values: Vec<String>) -> Result<Args, CliError> {
    let Some(command) = values.first() else {
        return Err(CliError("missing command: validate".to_string()));
    };
    if command != VALIDATE_COMMAND {
        return Err(CliError(format!("unsupported command: {command}")));
    }
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut input = None;
    let mut receipt_output = None;
    let mut projection_output = None;
    let mut update = false;
    let mut index = 1;
    while index < values.len() {
        match values[index].as_str() {
            "--root" => root = PathBuf::from(next_value(&values, &mut index, "--root")?),
            "--input" => {
                input = Some(PathBuf::from(next_value(&values, &mut index, "--input")?));
            }
            "--receipt-output" => {
                receipt_output = Some(PathBuf::from(next_value(
                    &values,
                    &mut index,
                    "--receipt-output",
                )?));
            }
            "--projection-output" => {
                projection_output = Some(PathBuf::from(next_value(
                    &values,
                    &mut index,
                    "--projection-output",
                )?));
            }
            "--update" => update = true,
            option => return Err(CliError(format!("unsupported option: {option}"))),
        }
        index += 1;
    }
    let Some(input) = input else {
        return Err(CliError("--input is required".to_string()));
    };
    if update && receipt_output.is_none() {
        return Err(CliError("--update requires --receipt-output".to_string()));
    }
    if !update && (receipt_output.is_some() || projection_output.is_some()) {
        return Err(CliError(
            "output paths require explicit --update".to_string(),
        ));
    }
    Ok(Args {
        root,
        input,
        receipt_output,
        projection_output,
        update,
    })
}

fn next_value<'a>(
    values: &'a [String],
    index: &mut usize,
    option: &str,
) -> Result<&'a str, CliError> {
    *index += 1;
    values
        .get(*index)
        .map(String::as_str)
        .ok_or_else(|| CliError(format!("{option} requires a value")))
}

fn run(args: Args) -> Result<(i32, ApplicationHostBindingCliReport), CliError> {
    let root = RootCapability::open(&args.root)?;
    let input_bytes = root.read(&args.input)?;
    let input: ApplicationHostBindingValidationInput = serde_json::from_slice(&input_bytes)
        .map_err(|error| CliError(format!("binding input is malformed: {error}")))?;
    let (validation, receipt) = build_application_host_binding_receipt(&input)?;
    let valence_projection = if validation.verdict == ApplicationHostBindingVerdict::Accepted {
        Some(project_application_host_binding_to_valence(&receipt)?)
    } else {
        None
    };
    if args.update {
        write_outputs(
            &root,
            args.receipt_output.as_ref(),
            args.projection_output.as_ref(),
            &receipt,
            valence_projection.as_ref(),
        )?;
    }
    let passed = validation.verdict == ApplicationHostBindingVerdict::Accepted;
    let code = if passed { EXIT_SUCCESS } else { EXIT_FAILURE };
    Ok((
        code,
        ApplicationHostBindingCliReport {
            passed,
            validation,
            receipt,
            valence_projection,
            mutation_performed: args.update,
        },
    ))
}

fn write_outputs(
    root: &RootCapability,
    receipt_output: Option<&PathBuf>,
    projection_output: Option<&PathBuf>,
    receipt: &ApplicationHostBindingReceipt,
    projection: Option<&ApplicationHostBindingValenceProjection>,
) -> Result<(), CliError> {
    let Some(receipt_output) = receipt_output else {
        return Err(CliError("receipt output disappeared".to_string()));
    };
    let receipt_bytes = serde_json::to_vec_pretty(receipt)
        .map_err(|error| CliError(format!("receipt serialization failed: {error}")))?;
    let _previous_receipt = root.read_optional(receipt_output)?;
    root.write(receipt_output, &receipt_bytes)?;
    if let Some(projection_output) = projection_output {
        let Some(projection) = projection else {
            return Err(CliError(
                "rejected binding cannot write a passing projection".to_string(),
            ));
        };
        let projection_bytes = serde_json::to_vec_pretty(projection)
            .map_err(|error| CliError(format!("projection serialization failed: {error}")))?;
        let _previous_projection = root.read_optional(projection_output)?;
        root.write(projection_output, &projection_bytes)?;
    }
    Ok(())
}
