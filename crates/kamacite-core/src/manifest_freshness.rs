use crate::{
    AdapterIdentity, Diagnostic, DiagnosticCode, EvaluatorIdentity, Identity, IdentityDomainMarker,
    PreservesCodecError, PreservesLimits, PreservesSchema, ProjectionIdentity, ReceiptIdentity,
    SourceIdentity, encode_typed_preserves,
};
use serde::{Deserialize, Serialize};

pub const MANIFEST_GENERATION_RECEIPT_SCHEMA: &str = "kamacite.manifest-generation-receipt.v1";
pub const MANIFEST_GENERATION_PRESERVES_ROOT_LABEL: &str = "ManifestGenerationReceiptBody";
pub const MANIFEST_GENERATION_PRESERVES_VERSION: &str = "1";
pub const MANIFEST_EVALUATOR_PROFILE: &str = "nickel-reference-cli-1.17.0";
pub const MANIFEST_EVALUATOR_DESCRIPTOR: &str =
    "nickel-reference-cli:nickel-lang-cli nickel 1.17.0 (rev cargore)";

const REQUIREMENT_GENERATION_RECEIPT: &str =
    "r[kamacite.findings.manifest_semantic_freshness.generation_receipt]";
const CHECKED_LAW_COMPLETE_PROJECTION: &str = "complete-nickel-projection-bound";
const CHECKED_LAW_CONTRACT_IDENTITY: &str = "closed-manifest-contract-bound";
const CHECKED_LAW_EVALUATOR_IDENTITY: &str = "evaluator-profile-bound";
const NON_CLAIM_EVALUATOR_CORRECTNESS: &str = "not Nickel evaluator correctness";
const NON_CLAIM_FUTURE_DETERMINISM: &str = "not proof of future deterministic evaluation";
const NON_CLAIM_SOURCE_SEMANTICS: &str = "not Nickel source semantic correctness";

#[derive(Clone, Copy, Debug)]
pub struct ManifestGenerationInput<'a> {
    pub source_bytes: &'a [u8],
    pub contract_bytes: &'a [u8],
    pub evaluator_profile: &'a str,
    pub evaluator_descriptor: &'a str,
    pub projection_bytes: &'a [u8],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestGenerationReceiptBody {
    pub schema: String,
    pub source_identity: SourceIdentity,
    pub contract_identity: AdapterIdentity,
    pub evaluator_profile: String,
    pub evaluator_descriptor: String,
    pub evaluator_identity: EvaluatorIdentity,
    pub projection_identity: ProjectionIdentity,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestGenerationReceipt {
    pub body: ManifestGenerationReceiptBody,
    pub receipt_identity: ReceiptIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ManifestGenerationValidation {
    pub passed: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub computed_receipt: Option<ManifestGenerationReceipt>,
}

// r[impl kamacite.findings.manifest_semantic_freshness.generation_receipt]
pub fn build_manifest_generation_receipt(
    input: ManifestGenerationInput<'_>,
) -> Result<ManifestGenerationReceipt, PreservesCodecError> {
    let body = ManifestGenerationReceiptBody {
        schema: MANIFEST_GENERATION_RECEIPT_SCHEMA.to_string(),
        source_identity: SourceIdentity::compute(input.source_bytes),
        contract_identity: AdapterIdentity::compute(input.contract_bytes),
        evaluator_profile: input.evaluator_profile.to_string(),
        evaluator_descriptor: input.evaluator_descriptor.to_string(),
        evaluator_identity: EvaluatorIdentity::compute(input.evaluator_descriptor.as_bytes()),
        projection_identity: ProjectionIdentity::compute(input.projection_bytes),
        checked_laws: vec![
            CHECKED_LAW_COMPLETE_PROJECTION.to_string(),
            CHECKED_LAW_CONTRACT_IDENTITY.to_string(),
            CHECKED_LAW_EVALUATOR_IDENTITY.to_string(),
        ],
        non_claims: vec![
            NON_CLAIM_EVALUATOR_CORRECTNESS.to_string(),
            NON_CLAIM_FUTURE_DETERMINISM.to_string(),
            NON_CLAIM_SOURCE_SEMANTICS.to_string(),
        ],
    };
    let canonical = encode_typed_preserves(
        &body,
        PreservesSchema {
            schema_version: MANIFEST_GENERATION_PRESERVES_VERSION,
            root_label: MANIFEST_GENERATION_PRESERVES_ROOT_LABEL,
        },
        PreservesLimits::default(),
    )?;
    Ok(ManifestGenerationReceipt {
        body,
        receipt_identity: ReceiptIdentity::compute(&canonical.packed_bytes),
    })
}

// r[impl kamacite.findings.manifest_semantic_freshness.complete_projection]
pub fn validate_manifest_generation_receipt(
    input: ManifestGenerationInput<'_>,
    supplied: &ManifestGenerationReceipt,
) -> ManifestGenerationValidation {
    let expected = match build_manifest_generation_receipt(input) {
        Ok(receipt) => receipt,
        Err(error) => {
            return ManifestGenerationValidation {
                passed: false,
                diagnostics: vec![Diagnostic::new(
                    DiagnosticCode::MalformedProjectionOutput,
                    REQUIREMENT_GENERATION_RECEIPT,
                    "manifest_generation_receipt",
                    format!("manifest generation receipt canonicalization failed: {error}"),
                )],
                computed_receipt: None,
            };
        }
    };
    let mut diagnostics = Vec::new();
    compare_identity(
        "source_identity",
        &expected.body.source_identity,
        &supplied.body.source_identity,
        &mut diagnostics,
    );
    compare_identity(
        "contract_identity",
        &expected.body.contract_identity,
        &supplied.body.contract_identity,
        &mut diagnostics,
    );
    compare_text(
        "evaluator_profile",
        &expected.body.evaluator_profile,
        &supplied.body.evaluator_profile,
        &mut diagnostics,
    );
    compare_text(
        "evaluator_descriptor",
        &expected.body.evaluator_descriptor,
        &supplied.body.evaluator_descriptor,
        &mut diagnostics,
    );
    compare_identity(
        "evaluator_identity",
        &expected.body.evaluator_identity,
        &supplied.body.evaluator_identity,
        &mut diagnostics,
    );
    compare_identity(
        "projection_identity",
        &expected.body.projection_identity,
        &supplied.body.projection_identity,
        &mut diagnostics,
    );
    if supplied.body.schema != expected.body.schema {
        diagnostics.push(mismatch(
            "schema",
            "manifest generation receipt schema is unsupported or stale",
        ));
    }
    if supplied.body.checked_laws != expected.body.checked_laws {
        diagnostics.push(mismatch(
            "checked_laws",
            "manifest generation checked laws are incomplete or stale",
        ));
    }
    if supplied.body.non_claims != expected.body.non_claims {
        diagnostics.push(mismatch(
            "non_claims",
            "manifest generation non-claims are incomplete or stale",
        ));
    }
    compare_identity(
        "receipt_identity",
        &expected.receipt_identity,
        &supplied.receipt_identity,
        &mut diagnostics,
    );
    diagnostics.sort();
    diagnostics.dedup();
    ManifestGenerationValidation {
        passed: diagnostics.is_empty(),
        diagnostics,
        computed_receipt: Some(expected),
    }
}

fn compare_identity<D: IdentityDomainMarker>(
    field: &str,
    expected: &Identity<D>,
    supplied: &Identity<D>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if expected != supplied {
        diagnostics.push(
            mismatch(
                field,
                "manifest generation identity does not match current exact input bytes",
            )
            .with_detail("expected_domain", format!("{:?}", expected.domain()))
            .with_detail("supplied_domain", format!("{:?}", supplied.domain())),
        );
    }
}

fn compare_text(field: &str, expected: &str, supplied: &str, diagnostics: &mut Vec<Diagnostic>) {
    if expected != supplied {
        diagnostics.push(mismatch(
            field,
            "manifest generation evaluator evidence is unsupported or stale",
        ));
    }
}

fn mismatch(field: &str, message: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::ManifestGenerationMismatch,
        REQUIREMENT_GENERATION_RECEIPT,
        field,
        message,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &[u8] = b"{fixtures = []}";
    const CONTRACT: &[u8] = b"{fixtures | Array { id | String }}";
    const PROJECTION: &[u8] = b"{\"fixtures\":[]}";
    const CHANGED: &[u8] = b"changed";

    fn input() -> ManifestGenerationInput<'static> {
        ManifestGenerationInput {
            source_bytes: SOURCE,
            contract_bytes: CONTRACT,
            evaluator_profile: MANIFEST_EVALUATOR_PROFILE,
            evaluator_descriptor: MANIFEST_EVALUATOR_DESCRIPTOR,
            projection_bytes: PROJECTION,
        }
    }

    // r[verify kamacite.findings.manifest_semantic_freshness.generation_receipt]
    #[test]
    fn accepts_exact_generation_evidence() {
        let built = build_manifest_generation_receipt(input());
        assert!(built.is_ok());
        let Ok(receipt) = built else {
            return;
        };
        let result = validate_manifest_generation_receipt(input(), &receipt);
        assert!(result.passed);
        assert!(result.diagnostics.is_empty());
    }

    // r[verify kamacite.findings.manifest_semantic_freshness.negative_cases]
    #[test]
    fn rejects_stale_source_contract_evaluator_projection_and_receipt() {
        let built = build_manifest_generation_receipt(input());
        assert!(built.is_ok());
        let Ok(receipt) = built else {
            return;
        };

        let stale_inputs = [
            ManifestGenerationInput {
                source_bytes: CHANGED,
                ..input()
            },
            ManifestGenerationInput {
                contract_bytes: CHANGED,
                ..input()
            },
            ManifestGenerationInput {
                evaluator_descriptor: "nickel-reference-cli:stale",
                ..input()
            },
            ManifestGenerationInput {
                projection_bytes: CHANGED,
                ..input()
            },
        ];
        for stale in stale_inputs {
            let result = validate_manifest_generation_receipt(stale, &receipt);
            assert!(!result.passed);
            assert!(!result.diagnostics.is_empty());
        }

        let mut tampered = receipt;
        tampered.body.non_claims.clear();
        let result = validate_manifest_generation_receipt(input(), &tampered);
        assert!(!result.passed);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == DiagnosticCode::ManifestGenerationMismatch
                && diagnostic.field == "non_claims"
        }));
    }
}
