#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::function_length,
        reason = "The exact Trellis record comparison stays contiguous so reviewers can audit every required field in one table."
    )
)]

use std::collections::BTreeSet;

use super::model::{
    BendHvm2Diagnostic, BendHvm2Profile, PilotBoundary, TRELLIS_ALGORITHM_PROFILE,
    TRELLIS_ORACLE_SYMBOL, TRELLIS_SCOPE,
};
use super::profile::diagnostic;
use serde::Deserialize;

const BUNDLE_SCHEMA: &str = "trellis-proof-evidence-v1";
const PROOF_OUTCOME: &str = "verified";
const TRACEY_COVERAGE: &str = "covered";
const DOWNSTREAM_REQUIREMENT: &str =
    "exact artifact pin plus downstream validation or rerun receipt";
const FORBIDDEN_CLAIMS: &[&str] = &[
    "bend correctness",
    "hvm2 correctness",
    "generated-c equivalence",
    "release approval",
    "production readiness",
];

#[derive(Deserialize)]
pub(super) struct TrellisBundle {
    schema_version: String,
    records: Vec<TrellisRecord>,
}

#[derive(Deserialize)]
struct TrellisRecord {
    scope: String,
    requirement_ids: Vec<String>,
    proof_functions: Vec<String>,
    algorithm_profile: String,
    executable_oracle_symbol: String,
    downstream_validation_requirement: String,
    proof_command: String,
    verifier_toolchain: String,
    proof_outcome: String,
    tracey_coverage: String,
    proof_gap_references: Vec<String>,
    content_identities: Vec<TrellisContentIdentity>,
    claims: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Deserialize)]
struct TrellisContentIdentity {
    path: String,
    algorithm: String,
    digest: String,
}

pub(super) fn validate(
    profile: &BendHvm2Profile,
    bundle: &TrellisBundle,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    if bundle.schema_version != BUNDLE_SCHEMA {
        diagnostics.push(proof_diagnostic(
            "proof-schema",
            "Trellis proof bundle uses an unsupported schema",
        ));
        return;
    }
    let record = bundle
        .records
        .iter()
        .find(|record| record.scope == TRELLIS_SCOPE);
    let Some(record) = record else {
        diagnostics.push(proof_diagnostic(
            "proof-scope",
            "Trellis proof bundle lacks the bounded modular tree-fold record",
        ));
        return;
    };
    validate_exact_fields(profile, record, diagnostics);
    compare_set(
        &record.requirement_ids,
        &profile.trellis.required_requirement_ids,
        "requirement_ids",
        diagnostics,
    );
    compare_set(
        &record.proof_functions,
        &profile.trellis.required_proof_functions,
        "proof_functions",
        diagnostics,
    );
    if !record.proof_gap_references.is_empty() {
        diagnostics.push(proof_diagnostic(
            "proof-gap-drift",
            "reviewed tree-fold proof record must retain an empty proof-gap set",
        ));
    }
    validate_content_identities(&record.content_identities, diagnostics);
    validate_claims(profile, record, diagnostics);
}

fn validate_exact_fields(
    profile: &BendHvm2Profile,
    record: &TrellisRecord,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    for (field, actual, expected) in [
        (
            "algorithm_profile",
            record.algorithm_profile.as_str(),
            TRELLIS_ALGORITHM_PROFILE,
        ),
        (
            "executable_oracle_symbol",
            record.executable_oracle_symbol.as_str(),
            TRELLIS_ORACLE_SYMBOL,
        ),
        (
            "downstream_validation_requirement",
            record.downstream_validation_requirement.as_str(),
            DOWNSTREAM_REQUIREMENT,
        ),
        (
            "proof_command",
            record.proof_command.as_str(),
            profile.trellis.proof_command.as_str(),
        ),
        (
            "verifier_toolchain",
            record.verifier_toolchain.as_str(),
            profile.trellis.verifier_toolchain.as_str(),
        ),
        (
            "proof_outcome",
            record.proof_outcome.as_str(),
            PROOF_OUTCOME,
        ),
        (
            "tracey_coverage",
            record.tracey_coverage.as_str(),
            TRACEY_COVERAGE,
        ),
    ] {
        if actual != expected {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "proof-fact-drift",
                format!("trellis.proof_bundle.{field}"),
                format!("expected `{expected}`, observed `{actual}`"),
            ));
        }
    }
}

fn validate_content_identities(
    identities: &[TrellisContentIdentity],
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    const EXPECTED_SOURCE_COUNT: usize = 5;
    const BLAKE3_HEX_LENGTH: usize = 64;
    if identities.len() != EXPECTED_SOURCE_COUNT
        || identities.iter().any(|identity| {
            identity.path.is_empty()
                || identity.algorithm != "blake3"
                || identity.digest.len() != BLAKE3_HEX_LENGTH
                || !identity.digest.bytes().all(|byte| byte.is_ascii_hexdigit())
        })
    {
        diagnostics.push(proof_diagnostic(
            "proof-source-identity",
            "Trellis proof record lacks the exact five-file BLAKE3 source closure",
        ));
    }
}

fn validate_claims(
    profile: &BendHvm2Profile,
    record: &TrellisRecord,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let claims = record.claims.join(" ").to_ascii_lowercase();
    if FORBIDDEN_CLAIMS.iter().any(|claim| claims.contains(claim)) {
        diagnostics.push(proof_diagnostic(
            "proof-overclaim",
            "Trellis proof record claims external correctness or release authority",
        ));
    }
    for non_claim in &profile.trellis.required_non_claims {
        if !record.non_claims.iter().any(|value| value == non_claim) {
            diagnostics.push(proof_diagnostic(
                "proof-non-claim",
                "Trellis proof record omits a required non-claim",
            ));
        }
    }
}

fn compare_set(
    actual: &[String],
    expected: &[String],
    field: &str,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let actual = actual.iter().collect::<BTreeSet<_>>();
    let expected = expected.iter().collect::<BTreeSet<_>>();
    if actual != expected {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "proof-set-drift",
            format!("trellis.proof_bundle.{field}"),
            "Trellis proof record set differs from the reviewed profile",
        ));
    }
}

fn proof_diagnostic(code: &str, message: &str) -> BendHvm2Diagnostic {
    diagnostic(
        PilotBoundary::PreExecution,
        code,
        "trellis.proof_handoff",
        message,
    )
}
