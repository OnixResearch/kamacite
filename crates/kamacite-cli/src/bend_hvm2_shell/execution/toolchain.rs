use super::{ExecutionContext, require_success};
use kamacite_core::bend_hvm2::ToolchainObservation;

const BEND_VERSION_PREFIX: &str = "bend-lang ";
const HVM_VERSION_PREFIX: &str = "hvm ";
const COMPILER_FAMILY: &str = "gcc";

pub(super) fn observe_toolchain(
    context: &ExecutionContext<'_>,
) -> Result<ToolchainObservation, String> {
    let bend = context.run(&context.bend, &["--version".to_string()])?;
    let hvm2 = context.run(&context.hvm2, &["--version".to_string()])?;
    let compiler_version = context.run(&context.compiler, &["-dumpfullversion".to_string()])?;
    let compiler_target = context.run(&context.compiler, &["-dumpmachine".to_string()])?;
    for (label, run) in [
        ("bend version", &bend),
        ("HVM2 version", &hvm2),
        ("compiler version", &compiler_version),
        ("compiler target", &compiler_target),
    ] {
        require_success(run, label)?;
    }
    Ok(ToolchainObservation {
        bend_version: parse_version(&bend.stdout, BEND_VERSION_PREFIX, "Bend")?,
        hvm2_version: parse_version(&hvm2.stdout, HVM_VERSION_PREFIX, "HVM2")?,
        compiler_family: COMPILER_FAMILY.to_string(),
        compiler_version: trimmed_utf8(&compiler_version.stdout, "compiler version")?,
        compiler_target: trimmed_utf8(&compiler_target.stdout, "compiler target")?,
    })
}

fn parse_version(bytes: &[u8], prefix: &str, label: &str) -> Result<String, String> {
    let text = trimmed_utf8(bytes, label)?;
    text.strip_prefix(prefix)
        .map(str::to_string)
        .ok_or_else(|| format!("{label} output lacks the expected prefix"))
}

fn trimmed_utf8(bytes: &[u8], label: &str) -> Result<String, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("{label} output is not UTF-8: {error}"))?;
    let text = text.trim();
    if text.is_empty() {
        Err(format!("{label} output is empty"))
    } else {
        Ok(text.to_string())
    }
}
