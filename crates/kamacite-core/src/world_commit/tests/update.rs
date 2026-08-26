use crate::identity::SourceIdentity;
use crate::world_commit::{
    MOLTEN_REVISION, WorldCohortUpdateRequest, plan_world_commit_cohort_update,
};

const NEXT_REVISION: &str = "1111111111111111111111111111111111111111";
const CONVERGENCE_PASSES: u32 = 2;

#[test]
fn unchanged_known_cohort_is_admitted_without_regeneration() {
    let identity = SourceIdentity::compute(b"source");
    let request = WorldCohortUpdateRequest {
        current_revision: MOLTEN_REVISION.to_string(),
        proposed_revision: MOLTEN_REVISION.to_string(),
        current_schema_identity: identity.clone(),
        proposed_schema_identity: identity.clone(),
        current_codec_identity: identity.clone(),
        proposed_codec_identity: identity,
        explicit_regeneration: false,
        convergence_passes: 0,
        removed_negative_vectors: Vec::new(),
    };
    let plan = plan_world_commit_cohort_update(&request);
    assert!(plan.admitted);
    assert!(!plan.requires_new_cohort);
}

// r[verify kamacite.world_commit_conformance.update]
#[test]
fn changed_cohort_requires_explicit_convergent_regeneration() {
    let identity = SourceIdentity::compute(b"source");
    let mut request = WorldCohortUpdateRequest {
        current_revision: MOLTEN_REVISION.to_string(),
        proposed_revision: NEXT_REVISION.to_string(),
        current_schema_identity: identity.clone(),
        proposed_schema_identity: SourceIdentity::compute(b"new-schema"),
        current_codec_identity: identity.clone(),
        proposed_codec_identity: identity,
        explicit_regeneration: false,
        convergence_passes: 0,
        removed_negative_vectors: Vec::new(),
    };
    let denied = plan_world_commit_cohort_update(&request);
    assert!(!denied.admitted);
    assert!(denied.requires_new_cohort);
    request.explicit_regeneration = true;
    request.convergence_passes = CONVERGENCE_PASSES;
    let admitted = plan_world_commit_cohort_update(&request);
    assert!(admitted.admitted);
}

#[test]
fn malformed_revisions_and_removed_negative_vectors_are_denied() {
    let identity = SourceIdentity::compute(b"source");
    let mut request = WorldCohortUpdateRequest {
        current_revision: MOLTEN_REVISION.to_string(),
        proposed_revision: "future".to_string(),
        current_schema_identity: identity.clone(),
        proposed_schema_identity: identity.clone(),
        current_codec_identity: identity.clone(),
        proposed_codec_identity: identity,
        explicit_regeneration: true,
        convergence_passes: CONVERGENCE_PASSES,
        removed_negative_vectors: Vec::new(),
    };
    assert!(!plan_world_commit_cohort_update(&request).admitted);
    request.proposed_revision = MOLTEN_REVISION.to_string();
    request
        .removed_negative_vectors
        .push("hidden-secret-field".to_string());
    assert!(!plan_world_commit_cohort_update(&request).admitted);
}
