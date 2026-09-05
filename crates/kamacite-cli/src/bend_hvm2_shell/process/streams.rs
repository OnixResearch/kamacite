use kamacite_core::bend_hvm2::{ProcessOutcome, ResourceProfile};
use rustix::fs::{MemfdFlags, memfd_create};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::Instant;

const READ_CHUNK_BYTES: usize = 8_192;
const SIGNAL_EXIT_CODE_BASE: i32 = 128;

pub(super) fn capture_file(name: &str) -> Result<File, String> {
    memfd_create(name, MemfdFlags::empty())
        .map(File::from)
        .map_err(|error| format!("stream capture memfd creation failed: {error}"))
}

pub(super) fn stream_length(file: &File, label: &str) -> Result<u64, String> {
    file.metadata()
        .map(|metadata| metadata.len())
        .map_err(|error| format!("{label} capture metadata failed: {error}"))
}

pub(super) fn read_bounded(file: &mut File, limit: u64, label: &str) -> Result<Vec<u8>, String> {
    file.seek(SeekFrom::Start(0))
        .map_err(|error| format!("{label} capture seek failed: {error}"))?;
    let mut output = Vec::new();
    let mut bounded = file.take(limit);
    let mut chunk = [0_u8; READ_CHUNK_BYTES];
    loop {
        let read = bounded
            .read(&mut chunk)
            .map_err(|error| format!("{label} capture read failed: {error}"))?;
        if read == 0 {
            break;
        }
        output.extend_from_slice(&chunk[..read]);
    }
    Ok(output)
}

pub(super) fn classify_outcome(
    timed_out: bool,
    success: bool,
    exit_code: Option<i32>,
    stdout_length: u64,
    stderr_length: u64,
    resources: &ResourceProfile,
) -> ProcessOutcome {
    if timed_out {
        return ProcessOutcome::Timeout;
    }
    if stdout_length > resources.output_limit_bytes {
        return ProcessOutcome::OutputLimit;
    }
    if stderr_length > resources.diagnostic_limit_bytes {
        return ProcessOutcome::DiagnosticLimit;
    }
    if success {
        return ProcessOutcome::Success;
    }
    if exit_code.is_none() || exit_code.is_some_and(|code| code >= SIGNAL_EXIT_CODE_BASE) {
        return ProcessOutcome::ResourceLimit;
    }
    ProcessOutcome::ExitFailure
}

pub(super) fn elapsed_nanoseconds(start: Instant) -> Result<u64, String> {
    u64::try_from(start.elapsed().as_nanos())
        .map_err(|_| "stage wall duration exceeded the u64 nanosecond domain".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome_classification_is_fail_closed() {
        let resources = ResourceProfile {
            timeout_milliseconds: 1,
            cpu_seconds: 1,
            memory_limit_bytes: 1,
            output_limit_bytes: 1,
            diagnostic_limit_bytes: 1,
            artifact_limit_bytes: 1,
            file_limit_bytes: 1,
            process_limit: 1,
        };
        assert_eq!(
            classify_outcome(true, false, None, 0, 0, &resources),
            ProcessOutcome::Timeout
        );
        assert_eq!(
            classify_outcome(false, false, None, 2, 0, &resources),
            ProcessOutcome::OutputLimit
        );
        assert_eq!(
            classify_outcome(false, false, None, 0, 2, &resources),
            ProcessOutcome::DiagnosticLimit
        );
        assert_eq!(
            classify_outcome(false, false, None, 0, 0, &resources),
            ProcessOutcome::ResourceLimit
        );
    }
}
