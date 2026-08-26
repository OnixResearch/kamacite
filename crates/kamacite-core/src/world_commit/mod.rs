//! Bounded Molten world-commit interchange conformance.
//!
//! This module owns byte-level admission, pinned cohort checks, independent
//! adapter parity, and deterministic receipts. Molten retains world semantics,
//! runtime behavior, restore compatibility, and authority.

mod cohort;
mod model;
mod parity;
mod parse;
mod update;

#[cfg(test)]
mod tests;

pub use cohort::{
    compute_world_commit_cohort_identity, source_identity, validate_world_commit_cohort,
};
pub use model::*;
pub use parity::{evaluate_world_commit_parity, project_world_commit_receipt_for_valence};
pub use parse::{
    admit_world_commit_packed, canonicalize_world_commit_text, parse_world_commit_value,
    validate_world_commit_value,
};
pub use update::plan_world_commit_cohort_update;
