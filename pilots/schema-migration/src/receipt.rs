use crate::{SCHEMA_MIGRATION_CORE_REVISION, SchemaMigrationPilotError};
use kamacite_core::{PreservesLimits, PreservesSchema, ReceiptIdentity, encode_typed_preserves};
use schema_migration_core::{
    BoundReason, InvalidCatalogReason, PlanReceipt, PlanningDecisionClass, PlanningOutcome,
    PreconditionDenial, ReceiptReason, project_receipt,
};
use serde::{Deserialize, Serialize};

pub const SCHEMA_MIGRATION_RECEIPT_SCHEMA_VERSION: u32 = 1;
pub const SCHEMA_MIGRATION_RECEIPT_PRESERVES_SCHEMA: &str =
    "kamacite.schema-migration-plan-receipt.v1";
pub const SCHEMA_MIGRATION_RECEIPT_ROOT_LABEL: &str = "kamacite-schema-migration-plan-receipt-v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MigrationCandidateReceipt {
    pub path_id: String,
    pub entry_ids: Vec<String>,
    pub risk_cost: u64,
    pub effort_cost: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "kamacite-schema-migration-plan-receipt-v1")]
pub struct SchemaMigrationPlanReceiptBody {
    pub schema_version: u32,
    pub planner_revision: String,
    pub decision: String,
    pub reason: String,
    pub plan_id: Option<String>,
    pub source_data: Option<String>,
    pub lineage: Option<String>,
    pub source_schema: Option<String>,
    pub target_schema: Option<String>,
    pub catalog_id: Option<String>,
    pub policy_id: Option<String>,
    pub limits_id: Option<String>,
    pub observations_id: Option<String>,
    pub path_id: Option<String>,
    pub step_count: u16,
    pub candidates: Vec<MigrationCandidateReceipt>,
    pub executes_migrations: bool,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifiedSchemaMigrationPlanReceipt {
    pub body: SchemaMigrationPlanReceiptBody,
    pub canonical_bytes: Vec<u8>,
    pub identity: ReceiptIdentity,
}

// r[impl kamacite.schema_migration.receipt]
// r[impl kamacite.schema_migration.pilot_boundary]
pub fn identify_planning_outcome(
    outcome: &PlanningOutcome,
) -> Result<IdentifiedSchemaMigrationPlanReceipt, SchemaMigrationPilotError> {
    let body = receipt_body(&project_receipt(outcome));
    let canonical_bytes = encode_typed_preserves(
        &body,
        PreservesSchema {
            schema_version: SCHEMA_MIGRATION_RECEIPT_PRESERVES_SCHEMA,
            root_label: SCHEMA_MIGRATION_RECEIPT_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| SchemaMigrationPilotError::Canonicalization(error.to_string()))?;
    let identity = ReceiptIdentity::compute(&canonical_bytes);
    Ok(IdentifiedSchemaMigrationPlanReceipt {
        body,
        canonical_bytes,
        identity,
    })
}

fn receipt_body(receipt: &PlanReceipt) -> SchemaMigrationPlanReceiptBody {
    SchemaMigrationPlanReceiptBody {
        schema_version: SCHEMA_MIGRATION_RECEIPT_SCHEMA_VERSION,
        planner_revision: SCHEMA_MIGRATION_CORE_REVISION.to_string(),
        decision: decision_label(receipt.decision()).to_string(),
        reason: reason_label(receipt.reason()).to_string(),
        plan_id: receipt.plan_id().map(|identity| identity.to_hex()),
        source_data: receipt
            .source_data()
            .map(|identity| identity.as_str().to_string()),
        lineage: receipt.lineage().map(|identity| identity.to_hex()),
        source_schema: receipt.source_schema().map(|identity| identity.to_hex()),
        target_schema: receipt.target_schema().map(|identity| identity.to_hex()),
        catalog_id: receipt.catalog_id().map(|identity| identity.to_hex()),
        policy_id: receipt.policy_id().map(|identity| identity.to_hex()),
        limits_id: receipt.limits_id().map(|identity| identity.to_hex()),
        observations_id: receipt.observations_id().map(|identity| identity.to_hex()),
        path_id: receipt.path_id().map(|identity| identity.to_hex()),
        step_count: receipt.step_count(),
        candidates: receipt
            .candidates()
            .iter()
            .map(|candidate| MigrationCandidateReceipt {
                path_id: candidate.path_id().to_hex(),
                entry_ids: candidate
                    .entry_ids()
                    .iter()
                    .map(|identity| identity.to_hex())
                    .collect(),
                risk_cost: candidate.cost().risk(),
                effort_cost: candidate.cost().effort(),
            })
            .collect(),
        executes_migrations: false,
        non_claims: receipt
            .non_claims()
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
    }
}

const fn decision_label(decision: PlanningDecisionClass) -> &'static str {
    match decision {
        PlanningDecisionClass::NoOp => "no-op",
        PlanningDecisionClass::Selected => "selected",
        PlanningDecisionClass::NoPath => "no-path",
        PlanningDecisionClass::Ambiguous => "ambiguous",
        PlanningDecisionClass::BoundExhausted => "bound-exhausted",
        PlanningDecisionClass::PreconditionDenied => "precondition-denied",
        PlanningDecisionClass::InvalidCatalog => "invalid-catalog",
    }
}

const fn reason_label(reason: ReceiptReason) -> &'static str {
    match reason {
        ReceiptReason::NoOp => "no-op",
        ReceiptReason::UniquePath => "unique-path",
        ReceiptReason::NoPath => "no-path",
        ReceiptReason::Ambiguous => "ambiguous",
        ReceiptReason::Bound(BoundReason::PathSteps) => "bound-path-steps",
        ReceiptReason::Bound(BoundReason::CandidatePaths) => "bound-candidate-paths",
        ReceiptReason::Bound(BoundReason::SearchStates) => "bound-search-states",
        ReceiptReason::Preconditions(PreconditionDenial::RequiredClassMissing) => {
            "precondition-required-class-missing"
        }
        ReceiptReason::Preconditions(PreconditionDenial::ObservationMissing) => {
            "precondition-observation-missing"
        }
        ReceiptReason::Preconditions(PreconditionDenial::ObservationDenied) => {
            "precondition-observation-denied"
        }
        ReceiptReason::Preconditions(PreconditionDenial::ImplementationCohortMismatch) => {
            "precondition-implementation-cohort-mismatch"
        }
        ReceiptReason::Preconditions(PreconditionDenial::ObservationSetInvalid) => {
            "precondition-observation-set-invalid"
        }
        ReceiptReason::InvalidCatalog(InvalidCatalogReason::EntryLimit) => {
            "invalid-catalog-entry-limit"
        }
        ReceiptReason::InvalidCatalog(InvalidCatalogReason::OutgoingEdgeLimit) => {
            "invalid-catalog-outgoing-edge-limit"
        }
        ReceiptReason::InvalidCatalog(InvalidCatalogReason::PreconditionLimit) => {
            "invalid-catalog-precondition-limit"
        }
        ReceiptReason::InvalidCatalog(InvalidCatalogReason::DuplicateEntry) => {
            "invalid-catalog-duplicate-entry"
        }
        ReceiptReason::InvalidCatalog(InvalidCatalogReason::SelfEdge) => {
            "invalid-catalog-self-edge"
        }
        ReceiptReason::InvalidCatalog(InvalidCatalogReason::CostOverflow) => {
            "invalid-catalog-cost-overflow"
        }
        ReceiptReason::InvalidCatalog(InvalidCatalogReason::IdentityFailure) => {
            "invalid-catalog-identity-failure"
        }
    }
}
