#![forbid(unsafe_code)]

mod root_io;

use kamacite_core::{
    LegacyEffectOperationReference, SemanticOperationDescriptorInput, SemanticOperationError,
    identify_semantic_operation, preview_semantic_operation_migration,
    semantic_operation_descriptor,
};
use root_io::{RootCapability, RootIoError};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const DEFAULT_ROOT: &str = ".";

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

impl From<SemanticOperationError> for CliError {
    fn from(error: SemanticOperationError) -> Self {
        Self(error.to_string())
    }
}

#[derive(Clone, Debug)]
struct Args {
    command: String,
    root: PathBuf,
    input: PathBuf,
    output: Option<PathBuf>,
    update: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MigrationInput {
    legacy: LegacyEffectOperationReference,
    descriptor: SemanticOperationDescriptorInput,
    changed_fields: Vec<String>,
}

#[derive(Debug, Serialize)]
struct IdentifyOutput {
    descriptor: kamacite_core::SemanticOperationDescriptor,
    canonical_bytes: Vec<u8>,
    semantic_operation_identity: kamacite_core::EffectOperationIdentity,
}

fn main() {
    let code = match parse_args(std::env::args().skip(1).collect()).and_then(run) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error}");
            EXIT_FAILURE
        }
    };
    std::process::exit(code);
}

fn parse_args(values: Vec<String>) -> Result<Args, CliError> {
    let Some(command) = values.first().cloned() else {
        return Err(CliError("missing command: identify or migrate".to_string()));
    };
    if command != "identify" && command != "migrate" {
        return Err(CliError(format!("unsupported command: {command}")));
    }
    let mut root = PathBuf::from(DEFAULT_ROOT);
    let mut input = None;
    let mut output = None;
    let mut update = false;
    let mut index = 1;
    while index < values.len() {
        match values[index].as_str() {
            "--root" => root = PathBuf::from(next_value(&values, &mut index, "--root")?),
            "--input" => input = Some(PathBuf::from(next_value(&values, &mut index, "--input")?)),
            "--output" => {
                output = Some(PathBuf::from(next_value(&values, &mut index, "--output")?));
            }
            "--update" => update = true,
            option => return Err(CliError(format!("unsupported option: {option}"))),
        }
        index += 1;
    }
    let Some(input) = input else {
        return Err(CliError("--input is required".to_string()));
    };
    if update && output.is_none() {
        return Err(CliError("--update requires --output".to_string()));
    }
    Ok(Args {
        command,
        root,
        input,
        output,
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

fn run(args: Args) -> Result<i32, CliError> {
    let root = RootCapability::open(&args.root)?;
    let input = root.read(&args.input)?;
    let rendered = if args.command == "identify" {
        let descriptor_input: SemanticOperationDescriptorInput = serde_json::from_slice(&input)
            .map_err(|error| CliError(format!("descriptor input is malformed: {error}")))?;
        let descriptor = semantic_operation_descriptor(&descriptor_input)?;
        let identified = identify_semantic_operation(&descriptor)?;
        serde_json::to_vec_pretty(&IdentifyOutput {
            descriptor: identified.descriptor,
            canonical_bytes: identified.canonical_bytes,
            semantic_operation_identity: identified.identity,
        })
        .map_err(|error| CliError(format!("identify output serialization failed: {error}")))?
    } else {
        let migration: MigrationInput = serde_json::from_slice(&input)
            .map_err(|error| CliError(format!("migration input is malformed: {error}")))?;
        let descriptor = semantic_operation_descriptor(&migration.descriptor)?;
        let preview = preview_semantic_operation_migration(
            &migration.legacy,
            &descriptor,
            &migration.changed_fields,
        )?;
        serde_json::to_vec_pretty(&preview)
            .map_err(|error| CliError(format!("migration output serialization failed: {error}")))?
    };
    if args.update {
        let Some(output) = args.output else {
            return Err(CliError("update output disappeared".to_string()));
        };
        let _previous_output = root.read_optional(&output)?;
        root.write(&output, &rendered)?;
    } else {
        println!("{}", String::from_utf8_lossy(&rendered));
    }
    Ok(EXIT_SUCCESS)
}
