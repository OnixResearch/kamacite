#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::ambiguous_params,
        tigerstyle::assertion_density,
        tigerstyle::compound_condition,
        tigerstyle::function_length,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "The exact cohort table stays contiguous for review, and helper labels plus ordered diagnostics disambiguate every compared field."
    )
)]

use super::diagnostic;
use crate::bend_hvm2::model::{
    BEND_HVM2_PROFILE_ID, BEND_HVM2_PROFILE_SCHEMA, BEND_REVISION, BEND_VERSION,
    BendHvm2Diagnostic, BendHvm2Profile, HVM2_REVISION, HVM2_VERSION, PILOT_LEAF_LIMIT,
    PILOT_MEASURED_RUNS, PILOT_RESULT, PILOT_WARMUP_RUNS, PilotBoundary, TRELLIS_ALGORITHM_PROFILE,
    TRELLIS_BUNDLE_BLAKE3, TRELLIS_ORACLE_SYMBOL, TRELLIS_REVISION, TRELLIS_SCOPE,
    TRELLIS_VERIFIER_PROVIDER_REVISION, ToolchainObservation, U24_MODULUS,
};

const EXACT_LICENSE: &str = "Apache-2.0";
const EXACT_DEPENDENCY: &str = "=2.0.22";
const EXACT_DISPOSITION: &str = "evaluation-only";
const EXACT_CANONICAL_IR: &str = "kamacite-packed-preserves";
const EXACT_COMPILER_FAMILY: &str = "gcc";
const EXACT_COMPILER_VERSION: &str = "15.2.0";
const EXACT_TARGET: &str = "x86_64-unknown-linux-gnu";
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "not Bend correctness",
    "not HVM2 correctness",
    "not generated-C semantic equivalence",
    "not host-compiler correctness",
    "not verifier soundness",
    "not sandbox completeness",
    "not portable performance",
    "not runtime adoption",
    "not production readiness",
    "not release eligibility",
];
const REQUIRED_DENIED_AUTHORITY: &[&str] = &[
    "network",
    "runtime-io",
    "filesystem-outside-scratch",
    "host-process-namespace",
    "environment-inheritance",
    "clock-input",
    "random",
    "credential",
    "device",
    "package-manager",
    "mutation-outside-scratch-and-receipt-output",
];

pub(super) fn validate_profile_fields(
    profile: &BendHvm2Profile,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    for (field, actual, expected) in [
        (
            "schema_version",
            profile.schema_version.as_str(),
            BEND_HVM2_PROFILE_SCHEMA,
        ),
        (
            "profile_id",
            profile.profile_id.as_str(),
            BEND_HVM2_PROFILE_ID,
        ),
        (
            "disposition",
            profile.disposition.as_str(),
            EXACT_DISPOSITION,
        ),
        (
            "canonical_ir",
            profile.canonical_ir.as_str(),
            EXACT_CANONICAL_IR,
        ),
        (
            "cohort.bend.revision",
            profile.cohort.bend.revision.as_str(),
            BEND_REVISION,
        ),
        (
            "cohort.bend.version",
            profile.cohort.bend.version.as_str(),
            BEND_VERSION,
        ),
        (
            "cohort.bend.license",
            profile.cohort.bend.license.as_str(),
            EXACT_LICENSE,
        ),
        (
            "cohort.hvm2.revision",
            profile.cohort.hvm2.revision.as_str(),
            HVM2_REVISION,
        ),
        (
            "cohort.hvm2.version",
            profile.cohort.hvm2.version.as_str(),
            HVM2_VERSION,
        ),
        (
            "cohort.hvm2.license",
            profile.cohort.hvm2.license.as_str(),
            EXACT_LICENSE,
        ),
        (
            "cohort.bend_hvm_dependency",
            profile.cohort.bend_hvm_dependency.as_str(),
            EXACT_DEPENDENCY,
        ),
        (
            "trellis.revision",
            profile.trellis.revision.as_str(),
            TRELLIS_REVISION,
        ),
        (
            "trellis.verifier_provider_revision",
            profile.trellis.verifier_provider_revision.as_str(),
            TRELLIS_VERIFIER_PROVIDER_REVISION,
        ),
        (
            "trellis.proof_bundle_blake3",
            profile.trellis.proof_bundle_blake3.as_str(),
            TRELLIS_BUNDLE_BLAKE3,
        ),
        (
            "trellis.scope",
            profile.trellis.scope.as_str(),
            TRELLIS_SCOPE,
        ),
        (
            "trellis.algorithm_profile",
            profile.trellis.algorithm_profile.as_str(),
            TRELLIS_ALGORITHM_PROFILE,
        ),
        (
            "trellis.executable_oracle_symbol",
            profile.trellis.executable_oracle_symbol.as_str(),
            TRELLIS_ORACLE_SYMBOL,
        ),
    ] {
        check_text(actual, expected, field, diagnostics);
    }
    validate_workload_and_benchmark(profile, diagnostics);
    validate_required_values(
        &profile.denied_authority,
        REQUIRED_DENIED_AUTHORITY,
        "denied_authority",
        diagnostics,
    );
    validate_required_values(
        &profile.non_claims,
        REQUIRED_NON_CLAIMS,
        "non_claims",
        diagnostics,
    );
}

pub(super) fn validate_toolchain(
    profile: &BendHvm2Profile,
    toolchain: &ToolchainObservation,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let expected = &profile.cohort.compiler;
    for (field, actual, required) in [
        (
            "toolchain.bend_version",
            toolchain.bend_version.as_str(),
            BEND_VERSION,
        ),
        (
            "toolchain.hvm2_version",
            toolchain.hvm2_version.as_str(),
            HVM2_VERSION,
        ),
        (
            "toolchain.compiler_family",
            toolchain.compiler_family.as_str(),
            EXACT_COMPILER_FAMILY,
        ),
        (
            "toolchain.compiler_version",
            toolchain.compiler_version.as_str(),
            EXACT_COMPILER_VERSION,
        ),
        (
            "toolchain.compiler_target",
            toolchain.compiler_target.as_str(),
            EXACT_TARGET,
        ),
        (
            "cohort.compiler.family",
            expected.family.as_str(),
            EXACT_COMPILER_FAMILY,
        ),
        (
            "cohort.compiler.version",
            expected.version.as_str(),
            EXACT_COMPILER_VERSION,
        ),
        (
            "cohort.compiler.target",
            expected.target.as_str(),
            EXACT_TARGET,
        ),
    ] {
        check_text(actual, required, field, diagnostics);
    }
}

fn validate_workload_and_benchmark(
    profile: &BendHvm2Profile,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let workload = &profile.workload;
    if workload.leaves.is_empty()
        || !workload.leaves.len().is_power_of_two()
        || workload.leaf_limit != PILOT_LEAF_LIMIT
        || workload.modulus != U24_MODULUS
        || workload.result != PILOT_RESULT
    {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "workload-policy",
            "workload",
            "workload differs from the reviewed non-empty power-of-two u24 profile",
        ));
    }
    if workload.leaves.iter().any(|leaf| *leaf >= U24_MODULUS) {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "u24-domain",
            "workload.leaves",
            "one or more leaves are outside the u24 domain",
        ));
    }
    let benchmark = &profile.benchmark;
    if benchmark.warmup_runs != PILOT_WARMUP_RUNS
        || benchmark.measured_runs != PILOT_MEASURED_RUNS
        || benchmark.maximum_stage_wall_milliseconds == 0
    {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "benchmark-policy",
            "benchmark",
            "benchmark repetitions or wall limit differ from the reviewed policy",
        ));
    }
}

fn validate_required_values(
    values: &[String],
    required: &[&str],
    field: &str,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    for item in required {
        if !values.iter().any(|value| value == item) {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "missing-boundary",
                field,
                format!("required value is absent: {item}"),
            ));
        }
    }
}

fn check_text(
    actual: &str,
    required: &str,
    field: &str,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    if actual != required {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "cohort-drift",
            field,
            format!("expected `{required}`, observed `{actual}`"),
        ));
    }
}
