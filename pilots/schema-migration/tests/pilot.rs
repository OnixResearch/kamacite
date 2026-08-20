use kamacite_core::{
    IdentityDomain, PreservesLimits, PreservesSchema, PreservesSyntax, decode_typed_preserves,
    verify_typed_preserves,
};
use kamacite_schema_migration_pilot::{
    AdjacentMigrationCatalog, SCHEMA_MIGRATION_CORE_REVISION,
    SCHEMA_MIGRATION_RECEIPT_PRESERVES_SCHEMA, SCHEMA_MIGRATION_RECEIPT_ROOT_LABEL,
    SchemaMigrationPilotError, SchemaMigrationPlanReceiptBody, SchemaVersion,
    VersionedMigrationEntry, identify_planning_outcome, plan_adjacent_migration,
};
use schema_identity_core::{
    AdmissionLimits, IdentityMode, MemberLimit, NodeId, NodeLimit, NominalScope, Openness,
    ScalarKind, SchemaDescriptor, SchemaNode, SchemaNodeKind, admit_schema,
};
use schema_migration_core::{
    CatalogEntryKey, CostFacts, CostOrder, CostValues, DataSubjectId, Direction,
    ImplementationCohortId, MigrationArtifactId, MigrationBinding, MigrationEntry,
    MigrationMetadata, MigrationMetadataInput, MigrationProfileId, ObservationSet,
    PlanningDecisionClass, PlanningLimitValues, PlanningLimits, PlanningPolicy,
    PlanningPolicyInput, PlanningRequest, PolicyProfileId, RECEIPT_NON_CLAIMS, ReversibilityClass,
    SchemaId, SchemaRoute, TextLimit, TiePolicy, TypeLineageId,
};

const TEXT_BYTES: u16 = 128;
const SCHEMA_NODES: u32 = 4;
const SCHEMA_MEMBERS: u32 = 4;
const MAXIMUM_ENTRIES: u16 = 16;
const MAXIMUM_OUTGOING_EDGES: u16 = 8;
const MAXIMUM_PATH_STEPS: u16 = 8;
const MAXIMUM_CANDIDATE_PATHS: u16 = 8;
const MAXIMUM_SEARCH_STATES: u32 = 64;
const MAXIMUM_PRECONDITIONS: u16 = 8;
const MAXIMUM_DIAGNOSTICS: u16 = 8;
const LOW_COST: u64 = 1;
const VERSION_ONE_VALUE: u32 = 1;
const VERSION_TWO_VALUE: u32 = 2;
const VERSION_THREE_VALUE: u32 = 3;
const VERSION_FOUR_VALUE: u32 = 4;

#[derive(Clone)]
struct Fixture {
    lineage: TypeLineageId,
    version_one_schema: SchemaId,
    version_two_schema: SchemaId,
    version_three_schema: SchemaId,
    cohort: ImplementationCohortId,
}

struct EntryInput<'a> {
    key: &'a str,
    source_schema: SchemaId,
    target_schema: SchemaId,
    direction: Direction,
}

#[test]
fn kamacite_maps_the_shared_planning_fixture_without_effects()
-> Result<(), Box<dyn std::error::Error>> {
    let report = schema_migration_conformance::shared_fixture_report()?;
    assert_eq!(report.decision(), "selected");
    assert_eq!(report.step_count(), VERSION_ONE_VALUE as u16);
    assert!(!report.has_conditional_recovery());
    assert!(report.plan_id().is_some());
    Ok(())
}

#[test]
fn kamacite_verifies_every_embedded_shared_vector() -> Result<(), Box<dyn std::error::Error>> {
    // r[verify kamacite.schema_migration.shared_planner]
    // r[verify kamacite.schema_migration.conformance]
    schema_migration_conformance::verify_embedded_vectors()?;
    Ok(())
}

#[test]
fn kamacite_rejects_unknown_planning_fixtures() {
    assert!(schema_migration_conformance::run_scenario("kamacite-unknown").is_err());
}

#[test]
fn adjacent_forward_plan_has_stable_canonical_receipt() -> Result<(), Box<dyn std::error::Error>> {
    // r[verify kamacite.schema_migration.adjacency]
    // r[verify kamacite.schema_migration.adjacency.forward]
    // r[verify kamacite.schema_migration.receipt]
    // r[verify kamacite.schema_migration.receipt.stable]
    let fixture = fixture()?;
    let catalog = catalog(vec![versioned_entry(
        entry(
            &fixture,
            EntryInput {
                key: "forward-one-two",
                source_schema: fixture.version_one_schema,
                target_schema: fixture.version_two_schema,
                direction: Direction::Forward,
            },
        )?,
        VERSION_ONE_VALUE,
        VERSION_TWO_VALUE,
    )?])?;
    let request = request(
        &fixture,
        fixture.version_one_schema,
        fixture.version_two_schema,
    )?;
    let outcome = plan_adjacent_migration(
        &catalog,
        &request,
        &policy(&fixture, true, false)?,
        limits()?,
    );
    assert_eq!(outcome.decision(), PlanningDecisionClass::Selected);

    let first = identify_planning_outcome(&outcome)?;
    let second = identify_planning_outcome(&outcome)?;
    assert_eq!(first, second);
    assert_eq!(first.identity.domain(), IdentityDomain::Receipt);
    assert!(!first.body.executes_migrations);
    assert_eq!(first.body.planner_revision, SCHEMA_MIGRATION_CORE_REVISION);
    assert_eq!(first.body.decision, "selected");
    assert_eq!(first.body.reason, "unique-path");
    assert_eq!(first.body.step_count, VERSION_ONE_VALUE as u16);
    assert!(first.body.plan_id.is_some());
    assert_eq!(
        first.body.non_claims,
        RECEIPT_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect::<Vec<_>>()
    );

    let artifact = verify_typed_preserves(
        &first.canonical_bytes,
        PreservesSyntax::Packed,
        &first.body,
        PreservesSchema {
            schema_version: SCHEMA_MIGRATION_RECEIPT_PRESERVES_SCHEMA,
            root_label: SCHEMA_MIGRATION_RECEIPT_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )?;
    let decoded: SchemaMigrationPlanReceiptBody = decode_typed_preserves(&artifact)?;
    assert_eq!(decoded, first.body);
    Ok(())
}

#[test]
fn declared_reverse_plan_is_independent() -> Result<(), Box<dyn std::error::Error>> {
    // r[verify kamacite.schema_migration.adjacency.reverse]
    let fixture = fixture()?;
    let catalog = catalog(vec![versioned_entry(
        entry(
            &fixture,
            EntryInput {
                key: "reverse-two-one",
                source_schema: fixture.version_two_schema,
                target_schema: fixture.version_one_schema,
                direction: Direction::Reverse,
            },
        )?,
        VERSION_TWO_VALUE,
        VERSION_ONE_VALUE,
    )?])?;
    let request = request(
        &fixture,
        fixture.version_two_schema,
        fixture.version_one_schema,
    )?;
    let outcome = plan_adjacent_migration(
        &catalog,
        &request,
        &policy(&fixture, false, true)?,
        limits()?,
    );
    assert_eq!(outcome.decision(), PlanningDecisionClass::Selected);
    Ok(())
}

#[test]
fn bounded_multi_step_plan_uses_only_adjacent_edges() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = fixture()?;
    let catalog = catalog(vec![
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "forward-one-two",
                    source_schema: fixture.version_one_schema,
                    target_schema: fixture.version_two_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_ONE_VALUE,
            VERSION_TWO_VALUE,
        )?,
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "forward-two-three",
                    source_schema: fixture.version_two_schema,
                    target_schema: fixture.version_three_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_TWO_VALUE,
            VERSION_THREE_VALUE,
        )?,
    ])?;
    let request = request(
        &fixture,
        fixture.version_one_schema,
        fixture.version_three_schema,
    )?;
    let outcome = plan_adjacent_migration(
        &catalog,
        &request,
        &policy(&fixture, true, false)?,
        limits()?,
    );
    assert_eq!(outcome.decision(), PlanningDecisionClass::Selected);
    let Some(plan) = outcome.plan() else {
        return Err("selected outcome did not contain a plan".into());
    };
    assert_eq!(plan.steps().len(), VERSION_TWO_VALUE as usize);
    Ok(())
}

#[test]
fn skipped_version_is_rejected_before_planning() -> Result<(), Box<dyn std::error::Error>> {
    // r[verify kamacite.schema_migration.adjacency.skipped]
    let fixture = fixture()?;
    let result = versioned_entry(
        entry(
            &fixture,
            EntryInput {
                key: "skipped-one-three",
                source_schema: fixture.version_one_schema,
                target_schema: fixture.version_three_schema,
                direction: Direction::Forward,
            },
        )?,
        VERSION_ONE_VALUE,
        VERSION_THREE_VALUE,
    );
    assert!(matches!(
        result,
        Err(SchemaMigrationPilotError::NonAdjacent { .. })
    ));
    Ok(())
}

#[test]
fn direction_mismatch_is_rejected_before_planning() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = fixture()?;
    let result = versioned_entry(
        entry(
            &fixture,
            EntryInput {
                key: "wrong-direction",
                source_schema: fixture.version_two_schema,
                target_schema: fixture.version_one_schema,
                direction: Direction::Forward,
            },
        )?,
        VERSION_TWO_VALUE,
        VERSION_ONE_VALUE,
    );
    assert!(matches!(
        result,
        Err(SchemaMigrationPilotError::DirectionMismatch { .. })
    ));
    Ok(())
}

#[test]
fn one_schema_cannot_bind_two_versions() -> Result<(), Box<dyn std::error::Error>> {
    // r[verify kamacite.schema_migration.adjacency.consistency]
    let fixture = fixture()?;
    let result = catalog(vec![
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "forward-one-two",
                    source_schema: fixture.version_one_schema,
                    target_schema: fixture.version_two_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_ONE_VALUE,
            VERSION_TWO_VALUE,
        )?,
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "conflicting-three-four",
                    source_schema: fixture.version_two_schema,
                    target_schema: fixture.version_three_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_THREE_VALUE,
            VERSION_FOUR_VALUE,
        )?,
    ]);
    assert!(matches!(
        result,
        Err(SchemaMigrationPilotError::SchemaVersionConflict { .. })
    ));
    Ok(())
}

#[test]
fn one_version_cannot_bind_two_schemas() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = fixture()?;
    let result = catalog(vec![
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "forward-one-two",
                    source_schema: fixture.version_one_schema,
                    target_schema: fixture.version_two_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_ONE_VALUE,
            VERSION_TWO_VALUE,
        )?,
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "alternate-one-two",
                    source_schema: fixture.version_three_schema,
                    target_schema: fixture.version_two_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_ONE_VALUE,
            VERSION_TWO_VALUE,
        )?,
    ]);
    assert!(matches!(
        result,
        Err(SchemaMigrationPilotError::VersionSchemaConflict { .. })
    ));
    Ok(())
}

#[test]
fn zero_and_overflow_versions_fail_closed() -> Result<(), Box<dyn std::error::Error>> {
    assert!(matches!(
        SchemaVersion::new(0),
        Err(SchemaMigrationPilotError::InvalidVersion { .. })
    ));
    let fixture = fixture()?;
    let result = versioned_entry(
        entry(
            &fixture,
            EntryInput {
                key: "overflow",
                source_schema: fixture.version_one_schema,
                target_schema: fixture.version_two_schema,
                direction: Direction::Forward,
            },
        )?,
        u32::MAX,
        VERSION_ONE_VALUE,
    );
    assert!(matches!(
        result,
        Err(SchemaMigrationPilotError::VersionOverflow { .. })
    ));
    Ok(())
}

#[test]
fn ambiguous_receipt_keeps_bounded_candidates() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = fixture()?;
    let catalog = catalog(vec![
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "ambiguous-a",
                    source_schema: fixture.version_one_schema,
                    target_schema: fixture.version_two_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_ONE_VALUE,
            VERSION_TWO_VALUE,
        )?,
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "ambiguous-b",
                    source_schema: fixture.version_one_schema,
                    target_schema: fixture.version_two_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_ONE_VALUE,
            VERSION_TWO_VALUE,
        )?,
    ])?;
    let request = request(
        &fixture,
        fixture.version_one_schema,
        fixture.version_two_schema,
    )?;
    let outcome = plan_adjacent_migration(
        &catalog,
        &request,
        &policy(&fixture, true, false)?,
        limits()?,
    );
    assert_eq!(outcome.decision(), PlanningDecisionClass::Ambiguous);
    let identified = identify_planning_outcome(&outcome)?;
    assert_eq!(identified.body.reason, "ambiguous");
    assert!(identified.body.plan_id.is_none());
    assert_eq!(identified.body.candidates.len(), VERSION_TWO_VALUE as usize);
    Ok(())
}

#[test]
fn path_step_bound_has_exact_terminal_reason() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = fixture()?;
    let catalog = catalog(vec![
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "bounded-one-two",
                    source_schema: fixture.version_one_schema,
                    target_schema: fixture.version_two_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_ONE_VALUE,
            VERSION_TWO_VALUE,
        )?,
        versioned_entry(
            entry(
                &fixture,
                EntryInput {
                    key: "bounded-two-three",
                    source_schema: fixture.version_two_schema,
                    target_schema: fixture.version_three_schema,
                    direction: Direction::Forward,
                },
            )?,
            VERSION_TWO_VALUE,
            VERSION_THREE_VALUE,
        )?,
    ])?;
    let request = request(
        &fixture,
        fixture.version_one_schema,
        fixture.version_three_schema,
    )?;
    let outcome = plan_adjacent_migration(
        &catalog,
        &request,
        &policy(&fixture, true, false)?,
        limits_with_path_steps(VERSION_ONE_VALUE as u16)?,
    );
    assert_eq!(outcome.decision(), PlanningDecisionClass::BoundExhausted);
    let identified = identify_planning_outcome(&outcome)?;
    assert_eq!(identified.body.reason, "bound-path-steps");
    assert!(identified.body.plan_id.is_none());
    Ok(())
}

#[test]
fn no_path_receipt_preserves_denial_without_plan_identity() -> Result<(), Box<dyn std::error::Error>>
{
    // r[verify kamacite.schema_migration.receipt.denial]
    let fixture = fixture()?;
    let catalog = catalog(Vec::new())?;
    let request = request(
        &fixture,
        fixture.version_one_schema,
        fixture.version_three_schema,
    )?;
    let outcome = plan_adjacent_migration(
        &catalog,
        &request,
        &policy(&fixture, true, false)?,
        limits()?,
    );
    assert_eq!(outcome.decision(), PlanningDecisionClass::NoPath);
    let identified = identify_planning_outcome(&outcome)?;
    assert_eq!(identified.body.decision, "no-path");
    assert_eq!(identified.body.reason, "no-path");
    assert!(identified.body.plan_id.is_none());
    assert_eq!(identified.body.step_count, 0);
    Ok(())
}

#[test]
fn cargo_nix_and_nickel_source_declarations_match() {
    // r[verify kamacite.schema_migration.pilot_boundary]
    // r[verify kamacite.schema_migration.pilot_boundary.msrv]
    let source = include_str!("../source.ncl");
    let flake = include_str!("../../../flake.nix");
    let root_manifest = include_str!("../../../Cargo.toml");
    let pilot_manifest = include_str!("../Cargo.toml");
    assert_eq!(source.matches(SCHEMA_MIGRATION_CORE_REVISION).count(), 1);
    assert_eq!(
        source.matches("= Revision").count(),
        VERSION_THREE_VALUE as usize
    );
    assert!(!source.contains("../schema-migration-core"));
    assert!(flake.contains(SCHEMA_MIGRATION_CORE_REVISION));
    assert!(flake.contains("schema-migration-core-src"));
    assert!(root_manifest.contains("rust-version = \"1.88\""));
    assert!(pilot_manifest.contains("rust-version = \"1.95.0\""));
}

fn fixture() -> Result<Fixture, Box<dyn std::error::Error>> {
    let text_limit = schema_identity_core::TextLimit::new(TEXT_BYTES)?;
    let scope = NominalScope::new("kamacite", "adjacent-schema-pilot", text_limit)?;
    let lineage = scope.lineage_id()?;
    Ok(Fixture {
        lineage,
        version_one_schema: schema(lineage, ScalarKind::Boolean)?,
        version_two_schema: schema(lineage, ScalarKind::Text)?,
        version_three_schema: schema(lineage, ScalarKind::Bytes)?,
        cohort: ImplementationCohortId::new("reviewed-pilot", TextLimit::new(TEXT_BYTES)?)?,
    })
}

fn schema(
    lineage: TypeLineageId,
    scalar: ScalarKind,
) -> Result<SchemaId, Box<dyn std::error::Error>> {
    let root = NodeId::new(VERSION_ONE_VALUE);
    let descriptor = SchemaDescriptor::new(
        IdentityMode::Nominal { lineage },
        Openness::Closed,
        root,
        vec![SchemaNode::new(root, SchemaNodeKind::Scalar(scalar))],
        None,
    );
    let limits = AdmissionLimits::new(
        NodeLimit::new(SCHEMA_NODES)?,
        MemberLimit::new(SCHEMA_MEMBERS)?,
    );
    Ok(admit_schema(descriptor, limits)?.schema_id())
}

fn entry(
    fixture: &Fixture,
    input: EntryInput<'_>,
) -> Result<MigrationEntry, Box<dyn std::error::Error>> {
    let text_limit = TextLimit::new(TEXT_BYTES)?;
    let reference = schema_migration_core::MigrationReference::new(
        input.source_schema,
        input.target_schema,
        MigrationArtifactId::new(
            format!("artifact-{}", input.key),
            schema_identity_core::TextLimit::new(TEXT_BYTES)?,
        )?,
        PolicyProfileId::new(
            "kamacite-adjacent",
            schema_identity_core::TextLimit::new(TEXT_BYTES)?,
        )?,
    );
    let metadata = MigrationMetadata::new(MigrationMetadataInput {
        direction: input.direction,
        implementation_cohort: fixture.cohort.clone(),
        profile: MigrationProfileId::new("adjacent-only", text_limit)?,
        preconditions: Vec::new(),
        reversibility: ReversibilityClass::Irreversible,
        cost: CostFacts::new(CostValues {
            risk: LOW_COST,
            effort: LOW_COST,
        }),
    })?;
    Ok(MigrationEntry::new(
        CatalogEntryKey::new(input.key, text_limit)?,
        MigrationBinding::new(fixture.lineage, reference),
        metadata,
    ))
}

fn versioned_entry(
    entry: MigrationEntry,
    source_version: u32,
    target_version: u32,
) -> Result<VersionedMigrationEntry, SchemaMigrationPilotError> {
    VersionedMigrationEntry::new(
        entry,
        SchemaVersion::new(source_version)?,
        SchemaVersion::new(target_version)?,
    )
}

fn catalog(
    entries: Vec<VersionedMigrationEntry>,
) -> Result<AdjacentMigrationCatalog, SchemaMigrationPilotError> {
    AdjacentMigrationCatalog::new(entries)
}

fn request(
    fixture: &Fixture,
    source: SchemaId,
    target: SchemaId,
) -> Result<PlanningRequest, schema_migration_core::MigrationError> {
    Ok(PlanningRequest::new(
        DataSubjectId::new("kamacite-pilot-data", TextLimit::new(TEXT_BYTES)?)?,
        SchemaRoute::new(fixture.lineage, source, target),
        ObservationSet::new(Vec::new()),
    ))
}

fn policy(
    fixture: &Fixture,
    allows_forward: bool,
    allows_reverse: bool,
) -> Result<PlanningPolicy, schema_migration_core::MigrationError> {
    PlanningPolicy::new(PlanningPolicyInput {
        allows_forward,
        allows_reverse,
        implementation_cohorts: vec![fixture.cohort.clone()],
        required_preconditions: Vec::new(),
        maximum_steps: MAXIMUM_PATH_STEPS,
        cost_order: CostOrder::RiskThenEffortThenSteps,
        tie_policy: TiePolicy::DenyAmbiguous,
    })
}

fn limits() -> Result<PlanningLimits, schema_migration_core::MigrationError> {
    limits_with_path_steps(MAXIMUM_PATH_STEPS)
}

fn limits_with_path_steps(
    maximum_path_steps: u16,
) -> Result<PlanningLimits, schema_migration_core::MigrationError> {
    PlanningLimits::new(PlanningLimitValues {
        maximum_entries: MAXIMUM_ENTRIES,
        maximum_outgoing_edges: MAXIMUM_OUTGOING_EDGES,
        maximum_path_steps,
        maximum_candidate_paths: MAXIMUM_CANDIDATE_PATHS,
        maximum_search_states: MAXIMUM_SEARCH_STATES,
        maximum_preconditions: MAXIMUM_PRECONDITIONS,
        maximum_diagnostics: MAXIMUM_DIAGNOSTICS,
    })
}
