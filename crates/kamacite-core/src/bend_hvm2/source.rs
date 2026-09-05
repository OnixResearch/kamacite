#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::ambiguous_params,
        tigerstyle::assertion_density,
        tigerstyle::compound_condition,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "Workload admission compares explicit source and input facts and returns all deterministic denials without assertion panics."
    )
)]

use super::model::{BendHvm2Diagnostic, BendHvm2Profile, PilotBoundary, U24_MODULUS};
use super::profile::diagnostic;
use crate::{
    CanonicalValueIdentity, PreservesLimits, PreservesSchema, ReceiptError, encode_typed_preserves,
};
use serde::{Deserialize, Serialize};

pub const BEND_HVM2_INPUT_SCHEMA: &str = "kamacite.bend-hvm2-leaf-input.v1";
const BEND_HVM2_INPUT_ROOT: &str = "BendHvm2LeafInput";
const BEND_SOURCE_HEADER: &str = "type U24Tree:\n  Leaf { value: u24 }\n  Node { ~left: U24Tree, ~right: U24Tree }\n\ndef sum(tree: U24Tree) -> u24:\n  fold tree:\n    case U24Tree/Leaf:\n      return tree.value\n    case U24Tree/Node:\n      return tree.left + tree.right\n\ndef main() -> u24:\n";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BendHvm2LeafInput {
    pub schema_version: String,
    pub numeric_type: String,
    pub leaves: Vec<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedBendHvm2Workload {
    pub input: BendHvm2LeafInput,
    pub canonical_bytes: Vec<u8>,
    pub canonical_identity: CanonicalValueIdentity,
}

pub fn admit_workload(
    profile: &BendHvm2Profile,
    source_bytes: &[u8],
    input_bytes: &[u8],
) -> Result<AdmittedBendHvm2Workload, Vec<BendHvm2Diagnostic>> {
    let mut diagnostics = Vec::new();
    let input = match decode_leaf_input(input_bytes) {
        Ok(input) => input,
        Err(error) => {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "input-decode",
                "canonical_input",
                error,
            ));
            return Err(diagnostics);
        }
    };
    validate_input(profile, &input, &mut diagnostics);
    validate_source(profile, source_bytes, &input.leaves, &mut diagnostics);
    let canonical_bytes = match canonical_leaf_input_bytes(&input) {
        Ok(bytes) => bytes,
        Err(error) => {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "input-canonicalization",
                "canonical_input",
                error.to_string(),
            ));
            Vec::new()
        }
    };
    diagnostics.sort();
    diagnostics.dedup();
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    Ok(AdmittedBendHvm2Workload {
        canonical_identity: CanonicalValueIdentity::compute(&canonical_bytes),
        input,
        canonical_bytes,
    })
}

pub fn decode_leaf_input(bytes: &[u8]) -> Result<BendHvm2LeafInput, String> {
    serde_json::from_slice(bytes).map_err(|error| format!("leaf input decoding failed: {error}"))
}

pub fn canonical_leaf_input_bytes(input: &BendHvm2LeafInput) -> Result<Vec<u8>, ReceiptError> {
    encode_typed_preserves(
        input,
        PreservesSchema {
            schema_version: BEND_HVM2_INPUT_SCHEMA,
            root_label: BEND_HVM2_INPUT_ROOT,
        },
        PreservesLimits::default(),
    )
    .map(|artifact| artifact.packed_bytes)
    .map_err(|error| ReceiptError::Serialize(error.to_string()))
}

pub fn render_bend_workload(leaves: &[u64]) -> Result<Vec<u8>, String> {
    const REQUIRED_LEAVES: usize = 4;
    if leaves.len() != REQUIRED_LEAVES {
        return Err("the reviewed Bend renderer requires exactly four leaves".to_string());
    }
    if leaves.iter().any(|leaf| *leaf >= U24_MODULUS) {
        return Err("the Bend renderer rejects leaves outside the u24 domain".to_string());
    }
    let source = format!(
        "{BEND_SOURCE_HEADER}  tree = U24Tree/Node {{\n    left: U24Tree/Node {{\n      left: U24Tree/Leaf {{ value: {} }},\n      right: U24Tree/Leaf {{ value: {} }}\n    }},\n    right: U24Tree/Node {{\n      left: U24Tree/Leaf {{ value: {} }},\n      right: U24Tree/Leaf {{ value: {} }}\n    }}\n  }}\n  return sum(tree)\n",
        leaves[0], leaves[1], leaves[2], leaves[3]
    );
    Ok(source.into_bytes())
}

fn validate_input(
    profile: &BendHvm2Profile,
    input: &BendHvm2LeafInput,
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    if input.schema_version != BEND_HVM2_INPUT_SCHEMA {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "input-schema",
            "canonical_input.schema_version",
            "leaf input uses an unsupported schema",
        ));
    }
    if input.numeric_type != "u24" {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "numeric-semantics",
            "canonical_input.numeric_type",
            "leaf input must declare u24 wrapping semantics",
        ));
    }
    let has_invalid_shape = input.leaves.is_empty() || !input.leaves.len().is_power_of_two();
    let has_exceeded_leaf_limit = usize::try_from(profile.workload.leaf_limit)
        .map_or(true, |leaf_limit_count| {
            input.leaves.len() > leaf_limit_count
        });
    if has_invalid_shape || has_exceeded_leaf_limit {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "leaf-shape",
            "canonical_input.leaves",
            "leaf input must be non-empty, power-of-two, and within the declared limit",
        ));
    }
    if input.leaves.iter().any(|leaf| *leaf >= U24_MODULUS) {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "u24-domain",
            "canonical_input.leaves",
            "leaf input contains a value outside the u24 domain",
        ));
    }
    if input.leaves != profile.workload.leaves {
        diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "input-mapping-drift",
            "canonical_input.leaves",
            "canonical leaves differ from the reviewed workload policy",
        ));
    }
}

fn validate_source(
    profile: &BendHvm2Profile,
    source_bytes: &[u8],
    leaves: &[u64],
    diagnostics: &mut Vec<BendHvm2Diagnostic>,
) {
    let source = match std::str::from_utf8(source_bytes) {
        Ok(source) => source,
        Err(error) => {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "source-encoding",
                "bend_source",
                format!("Bend source is not UTF-8: {error}"),
            ));
            return;
        }
    };
    for annotation in &profile.workload.required_annotations {
        if !source.contains(annotation) {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "missing-annotation",
                "bend_source",
                format!("required Bend annotation is absent: {annotation}"),
            ));
        }
    }
    for token in &profile.workload.forbidden_source_tokens {
        if source.contains(token) {
            diagnostics.push(diagnostic(
                PilotBoundary::PreExecution,
                "forbidden-authority",
                "bend_source",
                format!("forbidden Bend source token is present: {token}"),
            ));
        }
    }
    match render_bend_workload(leaves) {
        Ok(expected) if expected == source_bytes => {}
        Ok(_) => diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "input-mapping-drift",
            "bend_source",
            "Bend source does not equal the deterministic rendering of the canonical leaves",
        )),
        Err(error) => diagnostics.push(diagnostic(
            PilotBoundary::PreExecution,
            "source-render",
            "bend_source",
            error,
        )),
    }
}
