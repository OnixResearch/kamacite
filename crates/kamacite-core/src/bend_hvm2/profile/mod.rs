#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "Profile admission accumulates complete deterministic diagnostics instead of asserting on untrusted profile and tool facts."
    )
)]

mod fields;

use super::model::{
    BEND_LICENSE_BLAKE3, BendHvm2Diagnostic, BendHvm2Profile, HVM2_LICENSE_BLAKE3, PilotBoundary,
    ToolchainObservation,
};
use fields::{validate_profile_fields, validate_toolchain};

const REVIEWED_PROFILE: &[u8] =
    include_bytes!("../../../../../adapters/bend-hvm2/generated/profile.json");

pub fn admit_profile(
    bytes: &[u8],
    bend_license_bytes: &[u8],
    hvm2_license_bytes: &[u8],
    toolchain: &ToolchainObservation,
) -> Result<BendHvm2Profile, Vec<BendHvm2Diagnostic>> {
    let mut diagnostics = Vec::new();
    let profile = match serde_json::from_slice::<BendHvm2Profile>(bytes) {
        Ok(profile) => profile,
        Err(error) => {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "profile-decode",
                "profile",
                format!("Bend/HVM2 profile decoding failed: {error}"),
            ));
            return Err(diagnostics);
        }
    };
    if bytes != REVIEWED_PROFILE {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "profile-drift",
            "profile",
            "profile bytes differ from the reviewed Nickel projection",
        ));
    }
    validate_profile_fields(&profile, &mut diagnostics);
    validate_license(
        bend_license_bytes,
        &profile.cohort.bend.license_blake3,
        BEND_LICENSE_BLAKE3,
        "cohort.bend.license",
        &mut diagnostics,
    );
    validate_license(
        hvm2_license_bytes,
        &profile.cohort.hvm2.license_blake3,
        HVM2_LICENSE_BLAKE3,
        "cohort.hvm2.license",
        &mut diagnostics,
    );
    validate_toolchain(&profile, toolchain, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    if diagnostics.is_empty() {
        Ok(profile)
    } else {
        Err(diagnostics)
    }
}

fn validate_license(
    bytes: &[u8],
    declared: &str,
    expected: &str,
    field: &str,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let observed = blake3::hash(bytes).to_hex().to_string();
    if declared != expected || observed != expected {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "license-drift",
            field,
            "license bytes or declared identity differ from the reviewed Apache-2.0 artifact",
        ));
    }
}

pub(crate) fn diagnostic(
    boundary: PilotBoundary,
    code: impl Into<String>,
    field: impl Into<String>,
    message: impl Into<String>,
) -> BendHvm2Diagnostic {
    BendHvm2Diagnostic {
        boundary,
        code: code.into(),
        field: field.into(),
        message: message.into(),
    }
}
