use crate::root_io::RootCapability;
use std::path::Path;

const SCRATCH_WORKLOAD: &str = "workload.bend";
const SCRATCH_INPUT: &str = "input.json";

pub(super) fn prepare_scratch(root: &Path, source: &[u8], input: &[u8]) -> Result<(), String> {
    let mut entries =
        std::fs::read_dir(root).map_err(|error| format!("read scratch root: {error}"))?;
    if entries
        .next()
        .transpose()
        .map_err(|error| error.to_string())?
        .is_some()
    {
        return Err("scratch root must be empty before pilot execution".to_string());
    }
    let capability =
        RootCapability::open(root).map_err(|error| format!("open scratch capability: {error}"))?;
    capability
        .write(Path::new(SCRATCH_WORKLOAD), source)
        .map_err(|error| format!("write scratch workload: {error}"))?;
    capability
        .write(Path::new(SCRATCH_INPUT), input)
        .map_err(|error| format!("write scratch input: {error}"))
}
