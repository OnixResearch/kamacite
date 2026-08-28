#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::ambiguous_params,
        tigerstyle::assertion_density,
        tigerstyle::borrowed_argument_types,
        tigerstyle::excessive_file_length,
        tigerstyle::missing_const_fn,
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        tigerstyle::path_segment_repetition,
        reason = "The positive and negative parser fixtures need a separate byte-parity-preserving API decomposition."
    )
)]

use std::collections::BTreeSet;

use preserves::{IOValue, Value, ValueImpl};

use crate::identity::CanonicalValueIdentity;
use crate::preserves_codec::{
    CanonicalPreservesArtifact, PreservesLimits, PreservesSchema, PreservesSyntax,
    canonicalize_preserves,
};
use crate::world_commit::model::{
    AdmittedWorldCommit, MAX_WORLD_COMMIT_BYTES, MAX_WORLD_COMMIT_PARENTS, MAX_WORLD_COMMIT_ROOTS,
    WORLD_COMMIT_FRAME_VERSION, WORLD_COMMIT_IDENTITY_DOMAIN, WORLD_COMMIT_RECORD_LABEL,
    WORLD_COMMIT_SCHEMA, WORLD_COMMIT_VERSION, WorldCommitValue, WorldConformanceIssue,
    WorldProfile, WorldProfileKind, WorldRoot, WorldRootDomain,
};

const WORLD_COMMIT_ARITY: usize = 6;
const PROFILE_ARITY: usize = 3;
const ROOT_ARITY: usize = 2;
const WRAPPER_ARITY: usize = 1;
const FRAME_SEPARATOR: &[u8] = &[0];
const BLAKE3_PREFIX: &str = "blake3:";
const BLAKE3_HEX_LENGTH: usize = 64;

// r[impl kamacite.world_commit_conformance.rust]
// r[impl kamacite.world_commit_conformance.domains]
pub fn admit_world_commit_packed(
    input: &[u8],
) -> Result<AdmittedWorldCommit, Vec<WorldConformanceIssue>> {
    admit_world_commit(input, PreservesSyntax::Packed)
}

pub fn canonicalize_world_commit_text(
    input: &[u8],
) -> Result<AdmittedWorldCommit, Vec<WorldConformanceIssue>> {
    admit_world_commit(input, PreservesSyntax::Text)
}

fn admit_world_commit(
    input: &[u8],
    syntax: PreservesSyntax,
) -> Result<AdmittedWorldCommit, Vec<WorldConformanceIssue>> {
    if input.len() > MAX_WORLD_COMMIT_BYTES {
        return Err(vec![WorldConformanceIssue::BoundExceeded {
            field: "canonical-bytes".to_string(),
            actual: input.len(),
            maximum: MAX_WORLD_COMMIT_BYTES,
        }]);
    }
    let artifact = canonicalize_preserves(
        input,
        syntax,
        PreservesSchema {
            schema_version: WORLD_COMMIT_VERSION,
            root_label: WORLD_COMMIT_RECORD_LABEL,
        },
        world_commit_limits(),
    )
    .map_err(|error| {
        vec![WorldConformanceIssue::MalformedCanonical {
            detail: error.to_string(),
        }]
    })?;
    admit_artifact(artifact)
}

fn admit_artifact(
    artifact: CanonicalPreservesArtifact,
) -> Result<AdmittedWorldCommit, Vec<WorldConformanceIssue>> {
    let value = parse_world_commit_value(&artifact.value)?;
    let issues = validate_world_commit_value(&value);
    if !issues.is_empty() {
        return Err(issues);
    }
    let molten_world_commit_ref = identify_molten_world_commit(&artifact.packed_bytes)?;
    Ok(AdmittedWorldCommit {
        value_identity: CanonicalValueIdentity::compute(&artifact.packed_bytes),
        packed_bytes: artifact.packed_bytes,
        value,
        molten_world_commit_ref,
    })
}

pub fn parse_world_commit_value(
    value: &IOValue,
) -> Result<WorldCommitValue, Vec<WorldConformanceIssue>> {
    let fields = record_fields(value, WORLD_COMMIT_RECORD_LABEL, WORLD_COMMIT_ARITY)?;
    let schema = required_string(&fields[0], "schema")?;
    let version = wrapped_string(&fields[1], "version")?;
    let profile = parse_profile(&fields[2])?;
    let parents = wrapped_strings(&fields[3], "parents", MAX_WORLD_COMMIT_PARENTS)?;
    let roots = parse_roots(&fields[4])?;
    let completeness = wrapped_strings(&fields[5], "completeness", MAX_WORLD_COMMIT_ROOTS)?
        .into_iter()
        .map(|domain| {
            WorldRootDomain::parse(&domain).ok_or_else(|| {
                vec![WorldConformanceIssue::RootDomainSubstitution {
                    field: "completeness".to_string(),
                    expected: "known world root domain".to_string(),
                    actual: domain,
                }]
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(WorldCommitValue {
        schema,
        version,
        profile,
        parents,
        roots,
        completeness,
    })
}

pub fn validate_world_commit_value(value: &WorldCommitValue) -> Vec<WorldConformanceIssue> {
    let mut issues = Vec::new();
    if value.schema != WORLD_COMMIT_SCHEMA {
        issues.push(WorldConformanceIssue::CohortField {
            field: "schema".to_string(),
            detail: format!("expected {WORLD_COMMIT_SCHEMA}, found {}", value.schema),
        });
    }
    if value.version != WORLD_COMMIT_VERSION {
        issues.push(WorldConformanceIssue::UnsupportedVersion {
            actual: value.version.clone(),
        });
    }
    validate_profile(&value.profile, &mut issues);
    validate_parents(&value.parents, &mut issues);
    validate_roots(value, &mut issues);
    validate_completeness(value, &mut issues);
    issues.sort();
    issues.dedup();
    issues
}

fn parse_profile(value: &Value<IOValue>) -> Result<WorldProfile, Vec<WorldConformanceIssue>> {
    let fields = record_fields(value, "profile", PROFILE_ARITY)?;
    let kind_text = required_string(&fields[0], "profile.kind")?;
    let kind = WorldProfileKind::parse(&kind_text).ok_or_else(|| {
        vec![WorldConformanceIssue::UnsupportedProfile {
            actual: kind_text.clone(),
        }]
    })?;
    let profile_ref = required_string(&fields[1], "profile.profile-ref")?;
    let cohort_ref = optional_reference(&fields[2])?;
    Ok(WorldProfile {
        kind,
        profile_ref,
        cohort_ref,
    })
}

fn parse_roots(value: &Value<IOValue>) -> Result<Vec<WorldRoot>, Vec<WorldConformanceIssue>> {
    let values = wrapped_values(value, "roots", MAX_WORLD_COMMIT_ROOTS)?;
    values
        .as_slice()
        .iter()
        .map(|root| {
            let fields = record_fields(root, "typed-root", ROOT_ARITY)?;
            let domain_text = required_string(&fields[0], "roots.domain")?;
            let domain = WorldRootDomain::parse(&domain_text).ok_or_else(|| {
                vec![WorldConformanceIssue::RootDomainSubstitution {
                    field: "roots.domain".to_string(),
                    expected: "known world root domain".to_string(),
                    actual: domain_text,
                }]
            })?;
            Ok(WorldRoot {
                domain,
                reference: required_string(&fields[1], "roots.reference")?,
            })
        })
        .collect()
}

fn validate_profile(profile: &WorldProfile, issues: &mut Vec<WorldConformanceIssue>) {
    validate_reference("profile.profile-ref", &profile.profile_ref, issues);
    match (profile.kind, &profile.cohort_ref) {
        (WorldProfileKind::Logical, Some(_)) => issues.push(WorldConformanceIssue::CohortField {
            field: "profile.cohort-ref".to_string(),
            detail: "logical profiles must not carry an opaque cohort".to_string(),
        }),
        (WorldProfileKind::Opaque | WorldProfileKind::Mixed, None) => {
            issues.push(WorldConformanceIssue::CohortField {
                field: "profile.cohort-ref".to_string(),
                detail: "opaque and mixed profiles require a cohort".to_string(),
            })
        }
        (_, Some(reference)) => validate_reference("profile.cohort-ref", reference, issues),
        (_, None) => {}
    }
}

fn validate_parents(parents: &[String], issues: &mut Vec<WorldConformanceIssue>) {
    if parents.len() > MAX_WORLD_COMMIT_PARENTS {
        issues.push(WorldConformanceIssue::BoundExceeded {
            field: "parents".to_string(),
            actual: parents.len(),
            maximum: MAX_WORLD_COMMIT_PARENTS,
        });
    }
    if !parents.windows(2).all(|pair| pair[0] < pair[1]) {
        issues.push(WorldConformanceIssue::CanonicalOrder {
            field: "parents".to_string(),
        });
    }
    let mut seen = BTreeSet::new();
    for parent in parents {
        validate_reference("parents", parent, issues);
        if !seen.insert(parent) {
            issues.push(WorldConformanceIssue::DuplicateParent {
                reference: parent.clone(),
            });
        }
    }
}

fn validate_roots(value: &WorldCommitValue, issues: &mut Vec<WorldConformanceIssue>) {
    if value.roots.len() > MAX_WORLD_COMMIT_ROOTS {
        issues.push(WorldConformanceIssue::BoundExceeded {
            field: "roots".to_string(),
            actual: value.roots.len(),
            maximum: MAX_WORLD_COMMIT_ROOTS,
        });
    }
    if !value.roots.windows(2).all(|pair| pair[0] < pair[1]) {
        issues.push(WorldConformanceIssue::CanonicalOrder {
            field: "roots".to_string(),
        });
    }
    let required = value.profile.kind.required_roots();
    let mut domains = BTreeSet::new();
    let mut references = BTreeSet::new();
    for root in &value.roots {
        validate_reference("roots.reference", &root.reference, issues);
        if !domains.insert(root.domain) {
            issues.push(WorldConformanceIssue::DuplicateRootDomain {
                domain: root.domain.as_str().to_string(),
            });
        }
        if !references.insert(&root.reference) {
            issues.push(WorldConformanceIssue::DuplicateRootReference {
                reference: root.reference.clone(),
            });
        }
        if !required.contains(&root.domain) && root.domain != WorldRootDomain::AuthorityObservation
        {
            issues.push(WorldConformanceIssue::UnexpectedRoot {
                domain: root.domain.as_str().to_string(),
            });
        }
    }
    for domain in required {
        if !domains.contains(domain) {
            issues.push(WorldConformanceIssue::MissingRequiredRoot {
                domain: domain.as_str().to_string(),
            });
        }
    }
}

fn validate_completeness(value: &WorldCommitValue, issues: &mut Vec<WorldConformanceIssue>) {
    if value.completeness != value.profile.kind.required_roots() {
        issues.push(WorldConformanceIssue::CompletenessMismatch);
    }
}

fn validate_reference(field: &str, reference: &str, issues: &mut Vec<WorldConformanceIssue>) {
    let Some(hex) = reference.strip_prefix(BLAKE3_PREFIX) else {
        issues.push(WorldConformanceIssue::InvalidReference {
            field: field.to_string(),
        });
        return;
    };
    if hex.len() != BLAKE3_HEX_LENGTH
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        issues.push(WorldConformanceIssue::InvalidReference {
            field: field.to_string(),
        });
    }
}

fn identify_molten_world_commit(
    canonical_bytes: &[u8],
) -> Result<String, Vec<WorldConformanceIssue>> {
    let length = u64::try_from(canonical_bytes.len()).map_err(|_| {
        vec![WorldConformanceIssue::BoundExceeded {
            field: "canonical-bytes".to_string(),
            actual: canonical_bytes.len(),
            maximum: MAX_WORLD_COMMIT_BYTES,
        }]
    })?;
    let mut hasher = blake3::Hasher::new_derive_key(WORLD_COMMIT_IDENTITY_DOMAIN);
    hasher.update(WORLD_COMMIT_FRAME_VERSION);
    hasher.update(FRAME_SEPARATOR);
    hasher.update(&length.to_be_bytes());
    hasher.update(canonical_bytes);
    Ok(format!("blake3:{}", hasher.finalize().to_hex()))
}

fn wrapped_string(
    value: &Value<IOValue>,
    label: &str,
) -> Result<String, Vec<WorldConformanceIssue>> {
    let fields = record_fields(value, label, WRAPPER_ARITY)?;
    required_string(&fields[0], label)
}

fn wrapped_strings(
    value: &Value<IOValue>,
    label: &str,
    maximum: usize,
) -> Result<Vec<String>, Vec<WorldConformanceIssue>> {
    wrapped_values(value, label, maximum)?
        .as_slice()
        .iter()
        .map(|item| required_string(item, label))
        .collect()
}

fn wrapped_values(
    value: &Value<IOValue>,
    label: &str,
    maximum: usize,
) -> Result<Vec<Value<IOValue>>, Vec<WorldConformanceIssue>> {
    let fields = record_fields(value, label, WRAPPER_ARITY)?;
    let values = fields[0].as_sequence().ok_or_else(|| malformed(label))?;
    let values = values.collect::<Vec<_>>();
    if values.len() > maximum {
        return Err(vec![WorldConformanceIssue::BoundExceeded {
            field: label.to_string(),
            actual: values.len(),
            maximum,
        }]);
    }
    Ok(values)
}

fn optional_reference(
    value: &Value<IOValue>,
) -> Result<Option<String>, Vec<WorldConformanceIssue>> {
    if value.collect_simple_record("none", Some(0)).is_some() {
        return Ok(None);
    }
    let fields = record_fields(value, "some", WRAPPER_ARITY)?;
    Ok(Some(required_string(&fields[0], "profile.cohort-ref")?))
}

fn record_fields<V>(
    value: &V,
    label: &str,
    arity: usize,
) -> Result<Vec<Value<IOValue>>, Vec<WorldConformanceIssue>>
where
    V: ValueImpl<IOValue> + ?Sized,
{
    value
        .collect_simple_record(label, Some(arity))
        .map(|record| record.fields_iter().cloned().collect())
        .ok_or_else(|| malformed(label))
}

fn required_string<V>(value: &V, field: &str) -> Result<String, Vec<WorldConformanceIssue>>
where
    V: ValueImpl<IOValue> + ?Sized,
{
    value
        .as_string()
        .map(|text| text.into_owned())
        .ok_or_else(|| malformed(field))
}

fn malformed(field: &str) -> Vec<WorldConformanceIssue> {
    vec![WorldConformanceIssue::MalformedCanonical {
        detail: format!("field {field} has the wrong Preserves shape"),
    }]
}

fn world_commit_limits() -> PreservesLimits {
    const MAX_DEPTH: usize = 32;
    const MAX_COLLECTION_ITEMS: usize = 4_096;
    const MAX_VALUES: usize = 4_096;
    PreservesLimits {
        max_input_bytes: MAX_WORLD_COMMIT_BYTES,
        max_output_bytes: MAX_WORLD_COMMIT_BYTES,
        max_depth: MAX_DEPTH,
        max_collection_items: MAX_COLLECTION_ITEMS,
        max_atom_bytes: MAX_WORLD_COMMIT_BYTES,
        max_values: MAX_VALUES,
    }
}
