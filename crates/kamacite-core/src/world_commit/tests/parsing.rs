use crate::world_commit::{
    AdmittedWorldCommit, MAX_WORLD_COMMIT_PARENTS, MAX_WORLD_COMMIT_ROOTS, WorldConformanceIssue,
    WorldProfileKind, WorldRoot, WorldRootDomain, admit_world_commit_packed,
    canonicalize_world_commit_text, validate_world_commit_value,
};

use super::support::LOGICAL_TEXT;

const SYNTHETIC_COHORT_REFERENCE_INDEX: usize = 126;
const SYNTHETIC_PROFILE_REFERENCE_INDEX: usize = 127;
const OVERSIZED_ROOT_REFERENCE_BASE: usize = 128;
type TestResult = Result<(), String>;

// r[verify kamacite.world_commit_conformance.rust]
// r[verify kamacite.world_commit_conformance.domains]
#[test]
fn authoritative_logical_fixture_is_canonical_and_stable() -> TestResult {
    let admitted = logical_fixture()?;
    let read_back = admit_world_commit_packed(&admitted.packed_bytes)
        .map_err(|issues| format!("packed fixture denied: {issues:?}"))?;
    assert_eq!(read_back.value, admitted.value);
    assert_eq!(read_back.value_identity, admitted.value_identity);
    assert_eq!(
        read_back.molten_world_commit_ref,
        admitted.molten_world_commit_ref
    );
    assert_eq!(admitted.value.profile.kind, WorldProfileKind::Logical);
    Ok(())
}

#[test]
fn opaque_mixed_and_authority_observation_profiles_are_admitted() -> TestResult {
    let opaque = admit_text(&synthetic_world(WorldProfileKind::Opaque, false))?;
    assert_eq!(opaque.value.profile.kind, WorldProfileKind::Opaque);
    let mixed = admit_text(&synthetic_world(WorldProfileKind::Mixed, false))?;
    assert_eq!(mixed.value.profile.kind, WorldProfileKind::Mixed);
    let observed = admit_text(&synthetic_world(WorldProfileKind::Logical, true))?;
    assert!(
        observed
            .value
            .roots
            .iter()
            .any(|root| root.domain == WorldRootDomain::AuthorityObservation)
    );
    assert!(
        !observed
            .value
            .completeness
            .contains(&WorldRootDomain::AuthorityObservation)
    );
    Ok(())
}

#[test]
fn parent_bounds_accept_the_limit_and_reject_the_next_item() -> TestResult {
    let admitted = logical_fixture()?;
    let mut value = admitted.value;
    value.parents = (0..MAX_WORLD_COMMIT_PARENTS)
        .map(|index| content_ref(index + MAX_WORLD_COMMIT_ROOTS))
        .collect();
    assert!(validate_world_commit_value(&value).is_empty());
    value.parents.push(content_ref(
        MAX_WORLD_COMMIT_PARENTS + MAX_WORLD_COMMIT_ROOTS,
    ));
    assert!(
        validate_world_commit_value(&value)
            .iter()
            .any(|issue| matches!(
                issue,
                WorldConformanceIssue::BoundExceeded { field, .. } if field == "parents"
            ))
    );
    Ok(())
}

// r[verify kamacite.world_commit_conformance.verification]
#[test]
fn unknown_versions_schemas_profiles_and_domains_fail_closed() -> TestResult {
    let text = logical_text()?;
    let cases = [
        text.replacen("molten.world-commit.v1", "molten.world-commit.v2", 1),
        text.replacen("<version \"v1\">", "<version \"v2\">", 1),
        text.replacen("<profile \"logical\"", "<profile \"future\"", 1),
        text.replacen("<typed-root \"artifact\"", "<typed-root \"capability\"", 1),
    ];
    for case in cases {
        assert!(canonicalize_world_commit_text(case.as_bytes()).is_err());
    }
    Ok(())
}

#[test]
fn duplicate_domains_references_and_missing_roots_are_denied() -> TestResult {
    let text = logical_text()?;
    let duplicate_domain = text.replacen("<typed-root \"schema\"", "<typed-root \"artifact\"", 1);
    let issues = deny_text(&duplicate_domain)?;
    assert!(
        issues
            .iter()
            .any(|issue| matches!(issue, WorldConformanceIssue::DuplicateRootDomain { .. }))
    );

    let duplicate_reference = text.replacen(
        "blake3:1453dfe8bbdad46b15ce112b00a18f018e8c2f391bb7b3bb0be03a4c2aecc605",
        "blake3:503d92b9c978f1e2d2079f0caf68d926be702664d7e54c46e5f0ccd6b4b9e707",
        1,
    );
    let issues = deny_text(&duplicate_reference)?;
    assert!(
        issues
            .iter()
            .any(|issue| matches!(issue, WorldConformanceIssue::DuplicateRootReference { .. }))
    );

    let admitted = logical_fixture()?;
    let mut value = admitted.value;
    value.roots.pop();
    assert!(
        validate_world_commit_value(&value)
            .iter()
            .any(|issue| matches!(
                issue,
                WorldConformanceIssue::MissingRequiredRoot { domain } if domain == "policy"
            ))
    );
    Ok(())
}

#[test]
fn wrong_references_hidden_fields_and_oversized_roots_are_denied() -> TestResult {
    let text = logical_text()?;
    let wrong_reference = text.replacen("blake3:503", "sha256:503", 1);
    let issues = deny_text(&wrong_reference)?;
    assert!(
        issues
            .iter()
            .any(|issue| matches!(issue, WorldConformanceIssue::InvalidReference { .. }))
    );

    let hidden_field = text.replacen(" <completeness", " \"secret-material\" <completeness", 1);
    assert!(canonicalize_world_commit_text(hidden_field.as_bytes()).is_err());

    let admitted = logical_fixture()?;
    let mut value = admitted.value;
    let template = value
        .roots
        .first()
        .cloned()
        .ok_or_else(|| "logical fixture has no roots".to_string())?;
    while value.roots.len() <= MAX_WORLD_COMMIT_ROOTS {
        value.roots.push(WorldRoot {
            domain: template.domain,
            reference: content_ref(value.roots.len() + OVERSIZED_ROOT_REFERENCE_BASE),
        });
    }
    assert!(
        validate_world_commit_value(&value)
            .iter()
            .any(|issue| matches!(
                issue,
                WorldConformanceIssue::BoundExceeded { field, .. } if field == "roots"
            ))
    );
    Ok(())
}

fn logical_fixture() -> Result<AdmittedWorldCommit, String> {
    canonicalize_world_commit_text(LOGICAL_TEXT)
        .map_err(|issues| format!("logical fixture denied: {issues:?}"))
}

fn logical_text() -> Result<String, String> {
    std::str::from_utf8(LOGICAL_TEXT)
        .map(ToString::to_string)
        .map_err(|error| format!("logical fixture is not UTF-8: {error}"))
}

fn admit_text(text: &str) -> Result<AdmittedWorldCommit, String> {
    canonicalize_world_commit_text(text.as_bytes())
        .map_err(|issues| format!("synthetic world denied: {issues:?}"))
}

fn deny_text(text: &str) -> Result<Vec<WorldConformanceIssue>, String> {
    match canonicalize_world_commit_text(text.as_bytes()) {
        Ok(_) => Err("negative world fixture was admitted".to_string()),
        Err(issues) => Ok(issues),
    }
}

fn synthetic_world(kind: WorldProfileKind, include_authority: bool) -> String {
    let required = kind.required_roots();
    let mut domains = required.to_vec();
    if include_authority {
        domains.push(WorldRootDomain::AuthorityObservation);
    }
    domains.sort();
    let roots = domains
        .iter()
        .enumerate()
        .map(|(index, domain)| {
            format!(
                "<typed-root \"{}\" \"{}\">",
                domain.as_str(),
                content_ref(index + 1)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let completeness = required
        .iter()
        .map(|domain| format!("\"{}\"", domain.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    let cohort = match kind {
        WorldProfileKind::Logical => "<none>".to_string(),
        WorldProfileKind::Opaque | WorldProfileKind::Mixed => format!(
            "<some \"{}\">",
            content_ref(SYNTHETIC_COHORT_REFERENCE_INDEX)
        ),
    };
    format!(
        "<molten-world-commit-v1 \"molten.world-commit.v1\" <version \"v1\"> <profile \"{}\" \"{}\" {}> <parents []> <roots [{}]> <completeness [{}]>>",
        kind.as_str(),
        content_ref(SYNTHETIC_PROFILE_REFERENCE_INDEX),
        cohort,
        roots,
        completeness,
    )
}

fn content_ref(index: usize) -> String {
    format!("blake3:{index:064x}")
}
