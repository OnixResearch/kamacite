#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::raw_arithmetic_overflow,
        reason = "Test-only wall-duration multiplication uses small named constants under a fixed observation bound."
    )
)]

mod fixtures;
mod negative;
mod positive;
mod stored;

use super::*;
use crate::{IdentityDomain, ProjectionIdentity, compute_identity};

const PROFILE_BYTES: &[u8] =
    include_bytes!("../../../../../adapters/bend-hvm2/generated/profile.json");
const SOURCE_BYTES: &[u8] = include_bytes!("../../../../../fixtures/bend-hvm2/workload.bend");
const INPUT_BYTES: &[u8] = include_bytes!("../../../../../fixtures/bend-hvm2/input.json");
const BEND_LICENSE_BYTES: &[u8] =
    include_bytes!("../../../../../adapters/bend-hvm2/upstream/bend/LICENSE-APACHE");
const HVM2_LICENSE_BYTES: &[u8] =
    include_bytes!("../../../../../adapters/bend-hvm2/upstream/hvm2/LICENSE");
const PROOF_BUNDLE_BYTES: &[u8] =
    include_bytes!("../../../../../adapters/bend-hvm2/upstream/trellis/proof-evidence-bundle.json");
const STAGE_WALL_NANOSECONDS: u64 = 1_000_000;
const RUN_RS_WALL_MULTIPLIER: u64 = 2;
const GENERATED_C_WALL_MULTIPLIER: u64 = 3;
const RUN_RS_INTERACTIONS: u64 = 114;
const GENERATED_C_INTERACTIONS: u64 = 113;
const ORACLE_COMBINES: u64 = 3;
const SUCCESS_EXIT_CODE: i32 = 0;
const ROUTE_OUTPUT_BYTES: u64 = 12;
const EXPECTED_OBSERVATION_COUNT: usize = 20;

struct OwnedPilotRequest {
    profile_bytes: Vec<u8>,
    source_bytes: Vec<u8>,
    input_bytes: Vec<u8>,
    bend_license_bytes: Vec<u8>,
    hvm2_license_bytes: Vec<u8>,
    proof_bundle_bytes: Vec<u8>,
    proof_validation: ProofValidationObservation,
    toolchain: ToolchainObservation,
    observations: Vec<StageObservation>,
    generated_c_bytes: Vec<u8>,
    compiler_invocation_bytes: Vec<u8>,
    executable_bytes: Vec<u8>,
}

impl OwnedPilotRequest {
    fn valid() -> Option<Self> {
        let profile: BendHvm2Profile = serde_json::from_slice(PROFILE_BYTES).ok()?;
        Some(Self {
            profile_bytes: PROFILE_BYTES.to_vec(),
            source_bytes: SOURCE_BYTES.to_vec(),
            input_bytes: INPUT_BYTES.to_vec(),
            bend_license_bytes: BEND_LICENSE_BYTES.to_vec(),
            hvm2_license_bytes: HVM2_LICENSE_BYTES.to_vec(),
            proof_bundle_bytes: PROOF_BUNDLE_BYTES.to_vec(),
            proof_validation: ProofValidationObservation {
                trellis_revision: TRELLIS_REVISION.to_string(),
                verifier_provider_revision: TRELLIS_VERIFIER_PROVIDER_REVISION.to_string(),
                proof_check: "trellis-verus-proofs-exact-source-v1".to_string(),
                proof_result_present: true,
                proof_result_identity: compute_identity(
                    IdentityDomain::EvidenceChain,
                    b"exact Trellis proof result",
                ),
                verifier_identity: compute_identity(
                    IdentityDomain::Compiler,
                    b"exact Octet production Verus identity",
                ),
                passed: true,
                diagnostics: Vec::new(),
            },
            toolchain: ToolchainObservation {
                bend_version: BEND_VERSION.to_string(),
                hvm2_version: HVM2_VERSION.to_string(),
                compiler_family: "gcc".to_string(),
                compiler_version: "15.2.0".to_string(),
                compiler_target: "x86_64-unknown-linux-gnu".to_string(),
            },
            observations: valid_observations(&profile)?,
            generated_c_bytes: b"reviewed generated C bytes".to_vec(),
            compiler_invocation_bytes: b"cc generated.c -o generated -O2 -lm -lpthread".to_vec(),
            executable_bytes: b"reviewed executable bytes".to_vec(),
        })
    }

    fn request(&self) -> PilotEvaluationRequest<'_> {
        PilotEvaluationRequest {
            profile_bytes: &self.profile_bytes,
            source_bytes: &self.source_bytes,
            canonical_input_bytes: &self.input_bytes,
            bend_license_bytes: &self.bend_license_bytes,
            hvm2_license_bytes: &self.hvm2_license_bytes,
            proof_bundle_bytes: &self.proof_bundle_bytes,
            proof_validation: &self.proof_validation,
            toolchain: &self.toolchain,
            observations: &self.observations,
            generated_c_bytes: &self.generated_c_bytes,
            compiler_invocation_bytes: &self.compiler_invocation_bytes,
            executable_bytes: &self.executable_bytes,
        }
    }
}

fn valid_observations(profile: &BendHvm2Profile) -> Option<Vec<StageObservation>> {
    let mut observations = Vec::with_capacity(EXPECTED_OBSERVATION_COUNT);
    for stage in [
        PilotStage::TrellisOracle,
        PilotStage::BendRunRs,
        PilotStage::GeneratedC,
    ] {
        for sample_index in 0..super::observation::EXPECTED_ROUTE_OBSERVATION_COUNT {
            observations.push(route_observation(profile, stage, sample_index)?);
        }
    }
    observations.push(build_observation(profile, PilotStage::BendGenC)?);
    observations.push(build_observation(profile, PilotStage::CCompile)?);
    Some(observations)
}

fn route_observation(
    profile: &BendHvm2Profile,
    stage: PilotStage,
    sample_index: usize,
) -> Option<StageObservation> {
    let (interaction_count, combine_count, multiplier) = match stage {
        PilotStage::TrellisOracle => (None, Some(ORACLE_COMBINES), 1),
        PilotStage::BendRunRs => (Some(RUN_RS_INTERACTIONS), None, RUN_RS_WALL_MULTIPLIER),
        PilotStage::GeneratedC => (
            Some(GENERATED_C_INTERACTIONS),
            None,
            GENERATED_C_WALL_MULTIPLIER,
        ),
        PilotStage::BendGenC | PilotStage::CCompile => (None, None, 1),
    };
    Some(StageObservation {
        stage,
        sample_index,
        warmup: sample_index < PILOT_WARMUP_RUNS,
        command_profile_identity: command_profile_identity(profile, stage).ok()?,
        outcome: ProcessOutcome::Success,
        exit_code: Some(SUCCESS_EXIT_CODE),
        result: Some(PILOT_RESULT),
        interaction_count,
        combine_count,
        wall_nanoseconds: STAGE_WALL_NANOSECONDS * multiplier,
        peak_memory_bytes: None,
        stdout_identity: ProjectionIdentity::compute(b"route output"),
        stderr_identity: ProjectionIdentity::compute(b""),
        stdout_bytes: ROUTE_OUTPUT_BYTES,
        stderr_bytes: 0,
    })
}

fn build_observation(profile: &BendHvm2Profile, stage: PilotStage) -> Option<StageObservation> {
    Some(StageObservation {
        stage,
        sample_index: 0,
        warmup: false,
        command_profile_identity: command_profile_identity(profile, stage).ok()?,
        outcome: ProcessOutcome::Success,
        exit_code: Some(SUCCESS_EXIT_CODE),
        result: None,
        interaction_count: None,
        combine_count: None,
        wall_nanoseconds: STAGE_WALL_NANOSECONDS,
        peak_memory_bytes: None,
        stdout_identity: ProjectionIdentity::compute(b"build output"),
        stderr_identity: ProjectionIdentity::compute(b""),
        stdout_bytes: ROUTE_OUTPUT_BYTES,
        stderr_bytes: 0,
    })
}
