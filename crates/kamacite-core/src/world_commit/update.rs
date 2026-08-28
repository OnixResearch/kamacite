#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::assertion_density,
        tigerstyle::bool_naming,
        tigerstyle::missing_const_fn,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "The dry-run cohort update contract needs a separate compatibility-preserving API migration."
    )
)]

use crate::world_commit::model::{
    MOLTEN_REVISION, WorldCohortUpdatePlan, WorldCohortUpdateRequest,
};

const REVISION_HEX_LENGTH: usize = 40;
const REQUIRED_CONVERGENCE_PASSES: u32 = 2;

// r[impl kamacite.world_commit_conformance.update]
pub fn plan_world_commit_cohort_update(
    request: &WorldCohortUpdateRequest,
) -> WorldCohortUpdatePlan {
    let revision_changed = request.current_revision != request.proposed_revision;
    let schema_changed = request.current_schema_identity != request.proposed_schema_identity;
    let codec_changed = request.current_codec_identity != request.proposed_codec_identity;
    let mut changed_fields = Vec::new();
    if revision_changed {
        changed_fields.push("product-revision".to_string());
    }
    if schema_changed {
        changed_fields.push("product-schema-identity".to_string());
    }
    if codec_changed {
        changed_fields.push("canonical-codec-identity".to_string());
    }
    let proposed_revision_valid = is_revision(&request.proposed_revision);
    let current_cohort_known = request.current_revision == MOLTEN_REVISION;
    let convergence_required = !changed_fields.is_empty();
    let convergence_satisfied = !convergence_required
        || (request.explicit_regeneration
            && request.convergence_passes >= REQUIRED_CONVERGENCE_PASSES);
    let admitted = current_cohort_known
        && proposed_revision_valid
        && request.removed_negative_vectors.is_empty()
        && convergence_satisfied;
    WorldCohortUpdatePlan {
        changed_fields,
        requires_new_cohort: revision_changed || schema_changed || codec_changed,
        convergence_required,
        review_removed_negative_vectors: request.removed_negative_vectors.clone(),
        admitted,
    }
}

fn is_revision(value: &str) -> bool {
    value.len() == REVISION_HEX_LENGTH
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}
