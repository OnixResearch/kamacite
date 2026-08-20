use schema_migration_core::Direction;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SchemaMigrationPilotError {
    InvalidVersion {
        value: u32,
    },
    VersionOverflow {
        version: u32,
        direction: Direction,
    },
    DirectionMismatch {
        source_version: u32,
        target_version: u32,
        direction: Direction,
    },
    NonAdjacent {
        source_version: u32,
        target_version: u32,
        direction: Direction,
    },
    SchemaVersionConflict {
        schema: String,
        first_version: u32,
        second_version: u32,
    },
    VersionSchemaConflict {
        version: u32,
        first_schema: String,
        second_schema: String,
    },
    Canonicalization(String),
}

impl Display for SchemaMigrationPilotError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidVersion { value } => {
                write!(formatter, "schema version must be nonzero, got {value}")
            }
            Self::VersionOverflow { version, direction } => write!(
                formatter,
                "schema version {version} overflows adjacency in {direction:?} direction"
            ),
            Self::DirectionMismatch {
                source_version,
                target_version,
                direction,
            } => write!(
                formatter,
                "schema versions {source_version} to {target_version} conflict with {direction:?} direction"
            ),
            Self::NonAdjacent {
                source_version,
                target_version,
                direction,
            } => write!(
                formatter,
                "schema versions {source_version} to {target_version} are not adjacent in {direction:?} direction"
            ),
            Self::SchemaVersionConflict {
                schema,
                first_version,
                second_version,
            } => write!(
                formatter,
                "schema {schema} is bound to versions {first_version} and {second_version}"
            ),
            Self::VersionSchemaConflict {
                version,
                first_schema,
                second_schema,
            } => write!(
                formatter,
                "version {version} is bound to schemas {first_schema} and {second_schema}"
            ),
            Self::Canonicalization(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for SchemaMigrationPilotError {}
