use kamacite_cli::root_io::RootCapability;
use kamacite_core::SourceIdentity;
use kamacite_core::world_commit::{
    MAX_WORLD_COMMIT_PARENTS, MOLTEN_REVISION, WORLD_COMMIT_CODEC, WorldCohortUpdateRequest,
    plan_world_commit_cohort_update,
};

use crate::world_commit_shell::io::{pretty_json, write_or_compare};
use crate::world_commit_shell::model::{AlteredSourceVector, CohortConfig, HostileClaimVector};

const ORDERED_PARENT_COUNT: usize = 2;
const DERIVED_REFERENCE_BASE: usize = 256;
const OVERSIZED_ROOT_ADDITIONS: usize = 6;
const NEXT_MOLTEN_REVISION: &str = "1111111111111111111111111111111111111111";
const NONCANONICAL_PACKED_RECORD: &[u8] = &[
    0xb4, 0xb3, 0x0b, b'B', b'o', b'u', b'n', b'd', b'R', b'e', b'c', b'o', b'r', b'd', 0xb1, 0x05,
    b'a', b'l', b'p', b'h', b'a', 0xb5, 0xb0, 0x02, 0x00, 0x01, 0xb0, 0x01, 0x02, 0x84, 0x84,
];
const UNDECLARED_STEEL_SOURCE: &[u8] =
    b"(define (resolve-world-vector vector-id) (kamacite.read-file vector-id))\n";

pub fn materialize_derived_vectors(
    root: &RootCapability,
    config: &CohortConfig,
    primary_text: &str,
    update: bool,
) -> Result<(), String> {
    let ordered = with_parents(primary_text, ORDERED_PARENT_COUNT)?;
    write_vector(
        root,
        config,
        "ordered-parents-v1",
        ordered.as_bytes(),
        update,
    )?;
    let bounded = with_parents(primary_text, MAX_WORLD_COMMIT_PARENTS)?;
    write_vector(
        root,
        config,
        "bounded-parents-v1",
        bounded.as_bytes(),
        update,
    )?;

    let negative = [
        (
            "unknown-version",
            primary_text.replacen("<version \"v1\">", "<version \"v2\">", 1),
        ),
        (
            "unknown-schema",
            primary_text.replacen("molten.world-commit.v1", "molten.world-commit.v2", 1),
        ),
        (
            "unknown-root-domain",
            primary_text.replacen("<typed-root \"artifact\"", "<typed-root \"capability\"", 1),
        ),
        (
            "duplicate-root-domain",
            primary_text.replacen("<typed-root \"schema\"", "<typed-root \"artifact\"", 1),
        ),
        (
            "duplicate-root-reference",
            primary_text.replacen(
                "blake3:1453dfe8bbdad46b15ce112b00a18f018e8c2f391bb7b3bb0be03a4c2aecc605",
                "blake3:503d92b9c978f1e2d2079f0caf68d926be702664d7e54c46e5f0ccd6b4b9e707",
                1,
            ),
        ),
        ("missing-required-root", remove_policy_root(primary_text)?),
        (
            "wrong-typed-reference",
            primary_text.replacen("blake3:503", "sha256:503", 1),
        ),
        (
            "oversized-parents",
            with_parents(primary_text, MAX_WORLD_COMMIT_PARENTS + 1)?,
        ),
        ("oversized-roots", with_oversized_roots(primary_text)?),
        (
            "hidden-secret-field",
            primary_text.replacen(" <completeness", " \"secret-material\" <completeness", 1),
        ),
        ("order-drift", with_root_order_drift(primary_text)?),
        (
            "embedded-signature",
            primary_text.replacen(
                " <completeness",
                " <signature \"detached-signature-must-not-be-content\"> <completeness",
                1,
            ),
        ),
    ];
    for (id, bytes) in negative {
        write_vector(root, config, id, bytes.as_bytes(), update)?;
    }
    write_vector(
        root,
        config,
        "noncanonical-packed",
        NONCANONICAL_PACKED_RECORD,
        update,
    )?;
    write_vector(
        root,
        config,
        "undeclared-steel-authority",
        UNDECLARED_STEEL_SOURCE,
        update,
    )?;
    materialize_structured_negative_vectors(root, config, update)
}

fn materialize_structured_negative_vectors(
    root: &RootCapability,
    config: &CohortConfig,
    update: bool,
) -> Result<(), String> {
    let altered = AlteredSourceVector {
        schema: "kamacite.world-commit-altered-source-vector.v1".to_string(),
        source_path: "fixtures/world-commit/source/molten-model.rs".to_string(),
        replacement_text: "altered source bytes".to_string(),
        expected_verdict: "deny".to_string(),
    };
    write_vector(
        root,
        config,
        "altered-source",
        &pretty_json(&altered)?,
        update,
    )?;

    let source = SourceIdentity::compute(b"world-cohort-source");
    let update_request = WorldCohortUpdateRequest {
        current_revision: MOLTEN_REVISION.to_string(),
        proposed_revision: NEXT_MOLTEN_REVISION.to_string(),
        current_schema_identity: source.clone(),
        proposed_schema_identity: SourceIdentity::compute(b"world-cohort-schema-next"),
        current_codec_identity: source.clone(),
        proposed_codec_identity: source,
        explicit_regeneration: false,
        convergence_passes: 0,
        removed_negative_vectors: Vec::new(),
    };
    if plan_world_commit_cohort_update(&update_request).admitted {
        return Err("unapproved cohort update vector was unexpectedly admitted".to_string());
    }
    write_vector(
        root,
        config,
        "unapproved-cohort-update",
        &pretty_json(&update_request)?,
        update,
    )?;
    write_claim_vector(
        root,
        config,
        "codec-drift",
        HostileClaimVector {
            schema: "kamacite.world-commit-hostile-claim.v1".to_string(),
            class: "codec-drift".to_string(),
            claim: "changed codec bytes remain in the existing cohort".to_string(),
            current_identity: Some(WORLD_COMMIT_CODEC.to_string()),
            proposed_identity: Some("future-codec".to_string()),
            expected_verdict: "deny".to_string(),
        },
        update,
    )?;
    write_claim_vector(
        root,
        config,
        "json-as-authority",
        HostileClaimVector {
            schema: "kamacite.world-commit-hostile-claim.v1".to_string(),
            class: "json-as-authority".to_string(),
            claim: "JSON projection is canonical world identity".to_string(),
            current_identity: None,
            proposed_identity: None,
            expected_verdict: "deny".to_string(),
        },
        update,
    )?;
    write_claim_vector(
        root,
        config,
        "semantic-equivalence-overclaim",
        HostileClaimVector {
            schema: "kamacite.world-commit-hostile-claim.v1".to_string(),
            class: "semantic-equivalence-overclaim".to_string(),
            claim: "adapter parity proves evaluator semantic equivalence".to_string(),
            current_identity: None,
            proposed_identity: None,
            expected_verdict: "deny".to_string(),
        },
        update,
    )
}

fn write_claim_vector(
    root: &RootCapability,
    config: &CohortConfig,
    id: &str,
    vector: HostileClaimVector,
    update: bool,
) -> Result<(), String> {
    write_vector(root, config, id, &pretty_json(&vector)?, update)
}

fn write_vector(
    root: &RootCapability,
    config: &CohortConfig,
    id: &str,
    bytes: &[u8],
    update: bool,
) -> Result<(), String> {
    let vector = config
        .positive_vectors
        .iter()
        .chain(&config.negative_vectors)
        .find(|vector| vector.id == id)
        .ok_or_else(|| format!("cohort config does not declare vector {id}"))?;
    write_or_compare(root, &vector.path, bytes, update)
}

fn with_parents(primary_text: &str, count: usize) -> Result<String, String> {
    let parents = (0..count)
        .map(|index| format!("\"{}\"", content_ref(DERIVED_REFERENCE_BASE + index)))
        .collect::<Vec<_>>()
        .join(", ");
    let replacement = format!("<parents [{parents}]>");
    replace_once(primary_text, "<parents []>", &replacement)
}

fn remove_policy_root(primary_text: &str) -> Result<String, String> {
    let marker = "<typed-root \"policy\"";
    let start = primary_text
        .find(marker)
        .ok_or_else(|| "primary fixture has no policy root".to_string())?;
    let end_offset = primary_text[start..]
        .find('>')
        .ok_or_else(|| "primary policy root is unterminated".to_string())?;
    let end = start + end_offset + 1;
    let removal_start = start
        .checked_sub(2)
        .filter(|candidate| &primary_text[*candidate..start] == ", ")
        .unwrap_or(start);
    let mut output = String::with_capacity(primary_text.len());
    output.push_str(&primary_text[..removal_start]);
    output.push_str(&primary_text[end..]);
    Ok(output)
}

fn with_root_order_drift(primary_text: &str) -> Result<String, String> {
    let temporary = replace_once(
        primary_text,
        "<typed-root \"artifact\"",
        "<typed-root \"temporary-root-domain\"",
    )?;
    let moved_artifact = replace_once(
        &temporary,
        "<typed-root \"schema\"",
        "<typed-root \"artifact\"",
    )?;
    replace_once(
        &moved_artifact,
        "<typed-root \"temporary-root-domain\"",
        "<typed-root \"schema\"",
    )
}

fn with_oversized_roots(primary_text: &str) -> Result<String, String> {
    let marker = "]> <completeness";
    let additions = (0..OVERSIZED_ROOT_ADDITIONS)
        .map(|index| {
            format!(
                ", <typed-root \"authority-observation\" \"{}\">",
                content_ref(DERIVED_REFERENCE_BASE + MAX_WORLD_COMMIT_PARENTS + index)
            )
        })
        .collect::<String>();
    replace_once(primary_text, marker, &format!("{additions}{marker}"))
}

fn replace_once(input: &str, from: &str, to: &str) -> Result<String, String> {
    if input.matches(from).count() != 1 {
        return Err(format!("fixture marker is not unique: {from}"));
    }
    Ok(input.replacen(from, to, 1))
}

fn content_ref(index: usize) -> String {
    format!("blake3:{index:064x}")
}
