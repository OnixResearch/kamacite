mod adjacency;
mod error;
mod receipt;

pub use adjacency::{
    AdjacentMigrationCatalog, SchemaVersion, VersionedMigrationEntry, plan_adjacent_migration,
};
pub use error::SchemaMigrationPilotError;
pub use receipt::{
    IdentifiedSchemaMigrationPlanReceipt, MigrationCandidateReceipt,
    SCHEMA_MIGRATION_RECEIPT_PRESERVES_SCHEMA, SCHEMA_MIGRATION_RECEIPT_ROOT_LABEL,
    SCHEMA_MIGRATION_RECEIPT_SCHEMA_VERSION, SchemaMigrationPlanReceiptBody,
    identify_planning_outcome,
};

pub const SCHEMA_MIGRATION_CORE_REVISION: &str = "4fe90e130f2871cf69a6febcdc70785adca98aea";
