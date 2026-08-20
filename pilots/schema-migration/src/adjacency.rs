use crate::SchemaMigrationPilotError;
use schema_migration_core::{
    CatalogSnapshot, Direction, MigrationEntry, PlanningLimits, PlanningOutcome, PlanningPolicy,
    PlanningRequest, SchemaId, plan_migration,
};

const MINIMUM_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SchemaVersion(u32);

impl SchemaVersion {
    pub const fn new(value: u32) -> Result<Self, SchemaMigrationPilotError> {
        if value < MINIMUM_SCHEMA_VERSION {
            return Err(SchemaMigrationPilotError::InvalidVersion { value });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionedMigrationEntry {
    entry: MigrationEntry,
    source_version: SchemaVersion,
    target_version: SchemaVersion,
}

impl VersionedMigrationEntry {
    pub fn new(
        entry: MigrationEntry,
        source_version: SchemaVersion,
        target_version: SchemaVersion,
    ) -> Result<Self, SchemaMigrationPilotError> {
        validate_adjacency(source_version, target_version, entry.metadata().direction())?;
        Ok(Self {
            entry,
            source_version,
            target_version,
        })
    }

    #[must_use]
    pub const fn entry(&self) -> &MigrationEntry {
        &self.entry
    }

    #[must_use]
    pub const fn source_version(&self) -> SchemaVersion {
        self.source_version
    }

    #[must_use]
    pub const fn target_version(&self) -> SchemaVersion {
        self.target_version
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjacentMigrationCatalog {
    entries: Vec<VersionedMigrationEntry>,
}

impl AdjacentMigrationCatalog {
    // r[impl kamacite.schema_migration.adjacency]
    pub fn new(entries: Vec<VersionedMigrationEntry>) -> Result<Self, SchemaMigrationPilotError> {
        let mut bindings: Vec<(SchemaId, SchemaVersion)> = Vec::new();
        for versioned in &entries {
            let reference = versioned.entry().binding().reference();
            admit_binding(
                &mut bindings,
                reference.source_schema(),
                versioned.source_version(),
            )?;
            admit_binding(
                &mut bindings,
                reference.target_schema(),
                versioned.target_version(),
            )?;
        }
        Ok(Self { entries })
    }

    #[must_use]
    pub fn entries(&self) -> &[VersionedMigrationEntry] {
        &self.entries
    }

    #[must_use]
    pub fn snapshot(&self) -> CatalogSnapshot {
        CatalogSnapshot::new(
            self.entries
                .iter()
                .map(|versioned| versioned.entry().clone())
                .collect(),
        )
    }
}

fn validate_adjacency(
    source: SchemaVersion,
    target: SchemaVersion,
    direction: Direction,
) -> Result<(), SchemaMigrationPilotError> {
    let source_value = source.value();
    let target_value = target.value();
    let expected = match direction {
        Direction::Forward => {
            source_value
                .checked_add(1)
                .ok_or(SchemaMigrationPilotError::VersionOverflow {
                    version: source_value,
                    direction,
                })?
        }
        Direction::Reverse => {
            target_value
                .checked_add(1)
                .ok_or(SchemaMigrationPilotError::VersionOverflow {
                    version: target_value,
                    direction,
                })?
        }
    };
    let adjacent = match direction {
        Direction::Forward => expected == target_value,
        Direction::Reverse => expected == source_value,
    };
    if adjacent {
        return Ok(());
    }
    let opposite = match direction {
        Direction::Forward => target_value.checked_add(1) == Some(source_value),
        Direction::Reverse => source_value.checked_add(1) == Some(target_value),
    };
    if opposite {
        return Err(SchemaMigrationPilotError::DirectionMismatch {
            source_version: source_value,
            target_version: target_value,
            direction,
        });
    }
    Err(SchemaMigrationPilotError::NonAdjacent {
        source_version: source_value,
        target_version: target_value,
        direction,
    })
}

fn admit_binding(
    bindings: &mut Vec<(SchemaId, SchemaVersion)>,
    schema: SchemaId,
    version: SchemaVersion,
) -> Result<(), SchemaMigrationPilotError> {
    for (prior_schema, prior_version) in bindings.iter() {
        if *prior_schema == schema && *prior_version != version {
            return Err(SchemaMigrationPilotError::SchemaVersionConflict {
                schema: schema.to_hex(),
                first_version: prior_version.value(),
                second_version: version.value(),
            });
        }
        if *prior_version == version && *prior_schema != schema {
            return Err(SchemaMigrationPilotError::VersionSchemaConflict {
                version: version.value(),
                first_schema: prior_schema.to_hex(),
                second_schema: schema.to_hex(),
            });
        }
    }
    bindings.push((schema, version));
    Ok(())
}

// r[impl kamacite.schema_migration.shared_planner]
#[must_use]
pub fn plan_adjacent_migration(
    catalog: &AdjacentMigrationCatalog,
    request: &PlanningRequest,
    policy: &PlanningPolicy,
    limits: PlanningLimits,
) -> PlanningOutcome {
    plan_migration(&catalog.snapshot(), request, policy, limits)
}
