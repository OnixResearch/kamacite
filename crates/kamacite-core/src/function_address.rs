use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{Identity, IdentityDomain, compute_identity};
use crate::receipt::ReceiptError;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION: &str = "kamacite.function-address-receipt.v1";
pub const FUNCTION_ADDRESS_CANONICAL_CODEC: &str = "kamacite-preserves-function-address-receipt-v1";
pub const FUNCTION_ADDRESS_JSON_PROJECTION: &str =
    "kamacite.function-address-receipt.v1.compat-json";
pub const FUNCTION_ADDRESS_VALENCE_PROJECTION_SCHEMA_VERSION: &str =
    "kamacite.function-address-valence-projection.v1";
const CANONICAL_BYTES_PREFIX: &[u8] = b"kamacite-preserves:function-address-receipt:v1\n";
const REQUIREMENT_CONTRACT: &str = "r[kamacite.function_address_receipts.contract]";
const REQUIREMENT_CANONICAL_IDENTITY: &str =
    "r[kamacite.function_address_receipts.canonical_identity]";
const REQUIREMENT_VALIDATION: &str = "r[kamacite.function_address_receipts.validation]";
const REQUIREMENT_OCTET_INTEROP: &str = "r[kamacite.function_address_receipts.octet_interop]";
const NON_CLAIM_HOST_LANGUAGE_SEMANTICS: &str = "not host-language semantic correctness";
const NON_CLAIM_COMPILER_CORRECTNESS: &str = "not compiler correctness";
const NON_CLAIM_RUNTIME_BEHAVIOR: &str = "not runtime behavior";
const NON_CLAIM_WHOLE_PROGRAM_SAFETY: &str = "not whole-program safety";
const NON_CLAIM_VERIFIER_SOUNDNESS: &str = "not verifier soundness";
const NON_CLAIM_RELEASE_ELIGIBILITY: &str = "not release eligibility";
const REQUIRED_NON_CLAIMS: &[&str] = &[
    NON_CLAIM_HOST_LANGUAGE_SEMANTICS,
    NON_CLAIM_COMPILER_CORRECTNESS,
    NON_CLAIM_RUNTIME_BEHAVIOR,
    NON_CLAIM_WHOLE_PROGRAM_SAFETY,
    NON_CLAIM_VERIFIER_SOUNDNESS,
    NON_CLAIM_RELEASE_ELIGIBILITY,
];
const FORBIDDEN_CLAIM_FRAGMENTS: &[&str] = &[
    "semantic correctness",
    "compiler correctness",
    "runtime behavior",
    "whole-program safety",
    "verifier soundness",
    "release eligibility",
    "semantic equivalence",
    "source equivalence",
];
const OCTET_PRODUCER: &str = "octet";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressProducer {
    pub name: String,
    pub canonicalizer: String,
    pub tool_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressSourceEnvelope {
    pub key: String,
    pub display_path: String,
    pub language: String,
    pub source_bytes: String,
    pub source_identity: Identity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressSpan {
    pub source_key: String,
    pub start_byte: u64,
    pub end_byte: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressRecord {
    pub locator: String,
    pub display_name: String,
    pub source_key: String,
    pub span: Option<FunctionAddressSpan>,
    pub function_address: Identity,
    pub dependency_closure: Option<Identity>,
    pub caveats: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressDependencyEdge {
    pub from_locator: String,
    pub to_locator: String,
    pub dependency_identity: Option<Identity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressCaveat {
    pub locator: String,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressReceiptInput {
    pub producer: FunctionAddressProducer,
    pub source_envelopes: Vec<FunctionAddressSourceEnvelope>,
    pub functions: Vec<FunctionAddressRecord>,
    pub dependency_edges: Vec<FunctionAddressDependencyEdge>,
    pub caveats: Vec<FunctionAddressCaveat>,
    pub semantic_claims: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressReceiptBody {
    pub schema_version: String,
    pub canonical_codec: String,
    pub producer: FunctionAddressProducer,
    pub source_envelopes: Vec<FunctionAddressSourceEnvelope>,
    pub functions: Vec<FunctionAddressRecord>,
    pub dependency_edges: Vec<FunctionAddressDependencyEdge>,
    pub caveats: Vec<FunctionAddressCaveat>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressReceiptEnvelope {
    pub body: FunctionAddressReceiptBody,
    pub receipt_identity: Identity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionAddressReceiptResult {
    pub passed: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub receipt: Option<FunctionAddressReceiptEnvelope>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressJsonProjection {
    pub schema_version: String,
    pub compatibility_projection: String,
    pub canonical_identity_owner: String,
    pub receipt_identity: Identity,
    pub receipt: FunctionAddressReceiptBody,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressValenceProjection {
    pub schema_version: String,
    pub source_receipt_identity: Identity,
    pub producer: String,
    pub artifact_identities: Vec<FunctionAddressValenceArtifact>,
    pub dependency_edges: Vec<FunctionAddressDependencyEdge>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionAddressValenceArtifact {
    pub role: String,
    pub key: String,
    pub identity: Identity,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct CanonicalSourceEnvelope {
    source_identity: Identity,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct CanonicalFunctionRecord {
    source_identity: Identity,
    function_address: Identity,
    dependency_closure: Option<Identity>,
    caveats: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct CanonicalReceipt<'a> {
    schema_version: &'a str,
    canonical_codec: &'a str,
    producer_name: &'a str,
    canonicalizer: &'a str,
    source_envelopes: Vec<CanonicalSourceEnvelope>,
    functions: Vec<CanonicalFunctionRecord>,
    dependency_edges: Vec<CanonicalDependencyEdge>,
    caveats: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct CanonicalDependencyEdge {
    from_address: Identity,
    to_address: Identity,
    dependency_identity: Option<Identity>,
}

pub fn validate_function_address_receipt(
    input: &FunctionAddressReceiptInput,
) -> FunctionAddressReceiptResult {
    let mut diagnostics = Vec::new();
    validate_producer(&input.producer, &mut diagnostics);
    validate_non_claims(&input.non_claims, &mut diagnostics);
    validate_semantic_claims(&input.semantic_claims, &mut diagnostics);
    let source_identities = validate_sources(&input.source_envelopes, &mut diagnostics);
    let function_addresses = validate_functions(input, &source_identities, &mut diagnostics);
    validate_dependency_edges(input, &function_addresses, &mut diagnostics);
    validate_caveats(input, &function_addresses, &mut diagnostics);

    diagnostics.sort();
    diagnostics.dedup();
    if !diagnostics.is_empty() {
        return FunctionAddressReceiptResult {
            passed: false,
            diagnostics,
            receipt: None,
        };
    }

    match build_receipt(input, &source_identities) {
        Ok(receipt) => FunctionAddressReceiptResult {
            passed: true,
            diagnostics,
            receipt: Some(receipt),
        },
        Err(error) => FunctionAddressReceiptResult {
            passed: false,
            diagnostics: vec![Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_CANONICAL_IDENTITY,
                "receipt",
                format!("function-address receipt canonicalization failed: {error}"),
            )],
            receipt: None,
        },
    }
}

pub fn function_address_canonical_bytes(
    input: &FunctionAddressReceiptInput,
) -> Result<Vec<u8>, ReceiptError> {
    let source_identities = source_identity_map(&input.source_envelopes);
    let canonical = canonical_receipt(input, &source_identities);
    let mut bytes = CANONICAL_BYTES_PREFIX.to_vec();
    bytes.extend(crate::receipt::canonical_json_bytes(&canonical)?);
    Ok(bytes)
}

pub fn function_address_json_projection(
    receipt: &FunctionAddressReceiptEnvelope,
) -> FunctionAddressJsonProjection {
    FunctionAddressJsonProjection {
        schema_version: FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION.to_string(),
        compatibility_projection: FUNCTION_ADDRESS_JSON_PROJECTION.to_string(),
        canonical_identity_owner: FUNCTION_ADDRESS_CANONICAL_CODEC.to_string(),
        receipt_identity: receipt.receipt_identity.clone(),
        receipt: receipt.body.clone(),
    }
}

pub fn function_address_projection_json(
    receipt: &FunctionAddressReceiptEnvelope,
) -> Result<String, ReceiptError> {
    let projection = function_address_json_projection(receipt);
    serde_json::to_string_pretty(&projection)
        .map_err(|error| ReceiptError::Serialize(error.to_string()))
}

pub fn export_function_address_valence_projection(
    receipt: &FunctionAddressReceiptEnvelope,
) -> FunctionAddressValenceProjection {
    let mut artifact_identities = Vec::new();
    for source in &receipt.body.source_envelopes {
        artifact_identities.push(FunctionAddressValenceArtifact {
            role: "source-envelope".to_string(),
            key: source.key.clone(),
            identity: source.source_identity.clone(),
        });
    }
    for function in &receipt.body.functions {
        artifact_identities.push(FunctionAddressValenceArtifact {
            role: "function-address".to_string(),
            key: function.locator.clone(),
            identity: function.function_address.clone(),
        });
    }
    artifact_identities
        .sort_by(|left, right| left.key.cmp(&right.key).then(left.role.cmp(&right.role)));
    FunctionAddressValenceProjection {
        schema_version: FUNCTION_ADDRESS_VALENCE_PROJECTION_SCHEMA_VERSION.to_string(),
        source_receipt_identity: receipt.receipt_identity.clone(),
        producer: receipt.body.producer.name.clone(),
        artifact_identities,
        dependency_edges: receipt.body.dependency_edges.clone(),
        non_claims: receipt.body.non_claims.clone(),
    }
}

fn build_receipt(
    input: &FunctionAddressReceiptInput,
    source_identities: &BTreeMap<String, Identity>,
) -> Result<FunctionAddressReceiptEnvelope, ReceiptError> {
    let body = FunctionAddressReceiptBody {
        schema_version: FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION.to_string(),
        canonical_codec: FUNCTION_ADDRESS_CANONICAL_CODEC.to_string(),
        producer: input.producer.clone(),
        source_envelopes: input.source_envelopes.clone(),
        functions: input.functions.clone(),
        dependency_edges: input.dependency_edges.clone(),
        caveats: input.caveats.clone(),
        non_claims: input.non_claims.clone(),
    };
    let canonical = canonical_receipt(input, source_identities);
    let mut bytes = CANONICAL_BYTES_PREFIX.to_vec();
    bytes.extend(crate::receipt::canonical_json_bytes(&canonical)?);
    let receipt_identity = compute_identity(IdentityDomain::Receipt, &bytes);
    Ok(FunctionAddressReceiptEnvelope {
        body,
        receipt_identity,
    })
}

fn canonical_receipt<'a>(
    input: &'a FunctionAddressReceiptInput,
    source_identities: &BTreeMap<String, Identity>,
) -> CanonicalReceipt<'a> {
    let mut source_envelopes = source_identities
        .values()
        .cloned()
        .map(|source_identity| CanonicalSourceEnvelope { source_identity })
        .collect::<Vec<_>>();
    source_envelopes.sort();
    source_envelopes.dedup();

    let mut function_address_by_locator = BTreeMap::new();
    for function in &input.functions {
        function_address_by_locator
            .insert(function.locator.clone(), function.function_address.clone());
    }

    let mut functions = input
        .functions
        .iter()
        .filter_map(|function| {
            source_identities
                .get(&function.source_key)
                .map(|source_identity| CanonicalFunctionRecord {
                    source_identity: source_identity.clone(),
                    function_address: function.function_address.clone(),
                    dependency_closure: function.dependency_closure.clone(),
                    caveats: function.caveats.clone(),
                })
        })
        .collect::<Vec<_>>();
    for function in &mut functions {
        function.caveats.sort();
        function.caveats.dedup();
    }
    functions.sort();
    functions.dedup();

    let mut dependency_edges = input
        .dependency_edges
        .iter()
        .filter_map(|edge| {
            let from_address = function_address_by_locator.get(&edge.from_locator)?;
            let to_address = function_address_by_locator.get(&edge.to_locator)?;
            Some(CanonicalDependencyEdge {
                from_address: from_address.clone(),
                to_address: to_address.clone(),
                dependency_identity: edge.dependency_identity.clone(),
            })
        })
        .collect::<Vec<_>>();
    dependency_edges.sort();
    dependency_edges.dedup();

    let mut caveats = input
        .caveats
        .iter()
        .map(|caveat| caveat.reason.clone())
        .collect::<Vec<_>>();
    caveats.sort();
    caveats.dedup();

    let mut non_claims = input.non_claims.clone();
    non_claims.sort();
    non_claims.dedup();

    CanonicalReceipt {
        schema_version: FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION,
        canonical_codec: FUNCTION_ADDRESS_CANONICAL_CODEC,
        producer_name: &input.producer.name,
        canonicalizer: &input.producer.canonicalizer,
        source_envelopes,
        functions,
        dependency_edges,
        caveats,
        non_claims,
    }
}

fn validate_producer(producer: &FunctionAddressProducer, diagnostics: &mut Vec<Diagnostic>) {
    if producer.name != OCTET_PRODUCER {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::UnsupportedClaim,
                REQUIREMENT_OCTET_INTEROP,
                "producer.name",
                "first function-address receipt slice only accepts Octet producer input",
            )
            .with_detail("actual", producer.name.clone())
            .with_detail("expected", OCTET_PRODUCER),
        );
    }
    if producer.canonicalizer.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_CONTRACT,
            "producer.canonicalizer",
            "function-address receipts must bind the producer canonicalizer identity",
        ));
    }
}

fn validate_non_claims(non_claims: &[String], diagnostics: &mut Vec<Diagnostic>) {
    for required in REQUIRED_NON_CLAIMS {
        if !non_claims.iter().any(|non_claim| non_claim == required) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingNonClaim,
                    REQUIREMENT_VALIDATION,
                    "non_claims",
                    "function-address receipts must preserve required non-claim boundaries",
                )
                .with_detail("missing", *required),
            );
        }
    }
}

fn validate_semantic_claims(claims: &[String], diagnostics: &mut Vec<Diagnostic>) {
    for claim in claims {
        let lowered = claim.to_ascii_lowercase();
        if FORBIDDEN_CLAIM_FRAGMENTS
            .iter()
            .any(|fragment| lowered.contains(fragment))
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::SemanticOverclaim,
                    REQUIREMENT_VALIDATION,
                    "semantic_claims",
                    "function-address receipt contains forbidden overclaim text",
                )
                .with_detail("claim", claim.clone()),
            );
        }
    }
}

fn validate_sources(
    sources: &[FunctionAddressSourceEnvelope],
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, Identity> {
    let mut identities = BTreeMap::new();
    let mut keys = BTreeSet::new();
    for source in sources {
        if !keys.insert(source.key.clone()) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::DuplicateArtifactIdentity,
                    REQUIREMENT_VALIDATION,
                    "source_envelopes.key",
                    "source envelope keys must be unique within a receipt",
                )
                .with_detail("key", source.key.clone()),
            );
        }
        validate_identity_domain(
            &source.source_identity,
            IdentityDomain::Source,
            "source_envelopes.source_identity",
            diagnostics,
        );
        let computed = compute_identity(IdentityDomain::Source, source.source_bytes.as_bytes());
        if source.source_identity.domain == IdentityDomain::Source
            && source.source_identity.hex != computed.hex
        {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::StaleIdentity,
                    REQUIREMENT_VALIDATION,
                    "source_envelopes.source_identity",
                    "source envelope identity does not match preserved source bytes",
                )
                .with_detail("key", source.key.clone()),
            );
        }
        identities.insert(source.key.clone(), source.source_identity.clone());
    }
    if sources.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_CONTRACT,
            "source_envelopes",
            "function-address receipts require at least one source envelope",
        ));
    }
    identities
}

fn validate_functions(
    input: &FunctionAddressReceiptInput,
    source_identities: &BTreeMap<String, Identity>,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, Identity> {
    let mut function_addresses = BTreeMap::new();
    let mut locators = BTreeSet::new();
    for function in &input.functions {
        if !locators.insert(function.locator.clone()) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::DuplicateArtifactIdentity,
                    REQUIREMENT_VALIDATION,
                    "functions.locator",
                    "function locators must be unique within a receipt",
                )
                .with_detail("locator", function.locator.clone()),
            );
        }
        if !source_identities.contains_key(&function.source_key) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "functions.source_key",
                    "function record references an absent source envelope",
                )
                .with_detail("locator", function.locator.clone())
                .with_detail("source_key", function.source_key.clone()),
            );
        }
        validate_identity_domain(
            &function.function_address,
            IdentityDomain::FunctionAddress,
            "functions.function_address",
            diagnostics,
        );
        if let Some(dependency_closure) = &function.dependency_closure {
            validate_identity_domain(
                dependency_closure,
                IdentityDomain::DependencyClosure,
                "functions.dependency_closure",
                diagnostics,
            );
        }
        function_addresses.insert(function.locator.clone(), function.function_address.clone());
    }
    if input.functions.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_CONTRACT,
            "functions",
            "function-address receipts require at least one function record",
        ));
    }
    function_addresses
}

fn validate_dependency_edges(
    input: &FunctionAddressReceiptInput,
    function_addresses: &BTreeMap<String, Identity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for edge in &input.dependency_edges {
        if !function_addresses.contains_key(&edge.from_locator) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "dependency_edges.from_locator",
                    "dependency edge source function is absent from receipt",
                )
                .with_detail("from_locator", edge.from_locator.clone()),
            );
        }
        if !function_addresses.contains_key(&edge.to_locator) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "dependency_edges.to_locator",
                    "dependency edge target function is absent from receipt",
                )
                .with_detail("to_locator", edge.to_locator.clone()),
            );
        }
        if let Some(identity) = &edge.dependency_identity {
            validate_identity_domain(
                identity,
                IdentityDomain::DependencyClosure,
                "dependency_edges.dependency_identity",
                diagnostics,
            );
        }
    }
}

fn validate_caveats(
    input: &FunctionAddressReceiptInput,
    function_addresses: &BTreeMap<String, Identity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for caveat in &input.caveats {
        if !function_addresses.contains_key(&caveat.locator) {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "caveats.locator",
                    "caveat references an absent function locator",
                )
                .with_detail("locator", caveat.locator.clone()),
            );
        }
        if caveat.reason.is_empty() {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MissingInput,
                    REQUIREMENT_VALIDATION,
                    "caveats.reason",
                    "caveats must include a reviewable reason",
                )
                .with_detail("locator", caveat.locator.clone()),
            );
        }
    }
}

fn validate_identity_domain(
    identity: &Identity,
    expected: IdentityDomain,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if identity.validate_domain(expected.clone()).is_err() {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_VALIDATION,
                field,
                "function-address receipt identity uses the wrong domain or algorithm",
            )
            .with_detail("actual", identity.domain.to_string())
            .with_detail("expected", expected.to_string()),
        );
    }
}

fn source_identity_map(sources: &[FunctionAddressSourceEnvelope]) -> BTreeMap<String, Identity> {
    sources
        .iter()
        .map(|source| (source.key.clone(), source.source_identity.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::{Blake3Hex, IdentityError};

    const SOURCE_KEY: &str = "src/lib.rs";
    const SOURCE_PATH: &str = "src/lib.rs";
    const RENAMED_SOURCE_PATH: &str = "src/renamed.rs";
    const SOURCE_BYTES: &str = "pub fn f() -> u32 { 1 }\npub fn g() -> u32 { f() }\n";
    const FUNCTION_F_LOCATOR: &str = "src/lib.rs::f";
    const FUNCTION_G_LOCATOR: &str = "src/lib.rs::g";
    const FUNCTION_F_RENAMED_LOCATOR: &str = "src/renamed.rs::renamed_f";
    const FUNCTION_F_NAME: &str = "f";
    const FUNCTION_G_NAME: &str = "g";
    const RENAMED_FUNCTION_NAME: &str = "renamed_f";
    const CANONICALIZER: &str = "octet-source-token-function-address-v1";
    const TOOL_VERSION: &str = "octet-test";
    const CLAIM_RELEASE_ELIGIBILITY: &str = "this proves release eligibility";
    const INVALID_SHORT_HEX: &str = "abc";
    const SPAN_START: u64 = 0;
    const SPAN_END: u64 = 23;

    fn valid_input() -> FunctionAddressReceiptInput {
        let source_identity = compute_identity(IdentityDomain::Source, SOURCE_BYTES.as_bytes());
        let function_f = compute_identity(IdentityDomain::FunctionAddress, b"function f");
        let function_g = compute_identity(IdentityDomain::FunctionAddress, b"function g");
        let dependency = compute_identity(IdentityDomain::DependencyClosure, b"f -> g");
        FunctionAddressReceiptInput {
            producer: FunctionAddressProducer {
                name: OCTET_PRODUCER.to_string(),
                canonicalizer: CANONICALIZER.to_string(),
                tool_version: TOOL_VERSION.to_string(),
            },
            source_envelopes: vec![FunctionAddressSourceEnvelope {
                key: SOURCE_KEY.to_string(),
                display_path: SOURCE_PATH.to_string(),
                language: "rust".to_string(),
                source_bytes: SOURCE_BYTES.to_string(),
                source_identity,
            }],
            functions: vec![
                FunctionAddressRecord {
                    locator: FUNCTION_F_LOCATOR.to_string(),
                    display_name: FUNCTION_F_NAME.to_string(),
                    source_key: SOURCE_KEY.to_string(),
                    span: Some(FunctionAddressSpan {
                        source_key: SOURCE_KEY.to_string(),
                        start_byte: SPAN_START,
                        end_byte: SPAN_END,
                    }),
                    function_address: function_f,
                    dependency_closure: Some(dependency.clone()),
                    caveats: Vec::new(),
                },
                FunctionAddressRecord {
                    locator: FUNCTION_G_LOCATOR.to_string(),
                    display_name: FUNCTION_G_NAME.to_string(),
                    source_key: SOURCE_KEY.to_string(),
                    span: None,
                    function_address: function_g,
                    dependency_closure: Some(dependency.clone()),
                    caveats: vec!["dependency-closure-review-required".to_string()],
                },
            ],
            dependency_edges: vec![FunctionAddressDependencyEdge {
                from_locator: FUNCTION_G_LOCATOR.to_string(),
                to_locator: FUNCTION_F_LOCATOR.to_string(),
                dependency_identity: Some(dependency),
            }],
            caveats: vec![FunctionAddressCaveat {
                locator: FUNCTION_G_LOCATOR.to_string(),
                reason: "macro or unresolved dependency caveat retained".to_string(),
            }],
            semantic_claims: Vec::new(),
            non_claims: REQUIRED_NON_CLAIMS
                .iter()
                .map(|claim| (*claim).to_string())
                .collect(),
        }
    }

    fn valid_receipt() -> FunctionAddressReceiptEnvelope {
        validate_function_address_receipt(&valid_input())
            .receipt
            .expect("valid receipt")
    }

    #[test]
    fn valid_octet_receipt_preserves_inventory_and_projection_identity() {
        let input = valid_input();

        let result = validate_function_address_receipt(&input);

        assert!(
            result.passed,
            "unexpected diagnostics: {:?}",
            result.diagnostics
        );
        let receipt = result.receipt.expect("receipt emitted");
        assert_eq!(
            receipt.body.schema_version,
            FUNCTION_ADDRESS_RECEIPT_SCHEMA_VERSION
        );
        assert_eq!(receipt.receipt_identity.domain, IdentityDomain::Receipt);
        let projection = function_address_json_projection(&receipt);
        assert_eq!(projection.receipt_identity, receipt.receipt_identity);
        assert_eq!(
            projection.compatibility_projection,
            FUNCTION_ADDRESS_JSON_PROJECTION
        );
    }

    #[test]
    fn metadata_rename_and_path_churn_do_not_change_canonical_identity() {
        let original = valid_receipt();
        let mut renamed = valid_input();
        renamed.source_envelopes[0].display_path = RENAMED_SOURCE_PATH.to_string();
        renamed.functions[0].locator = FUNCTION_F_RENAMED_LOCATOR.to_string();
        renamed.functions[0].display_name = RENAMED_FUNCTION_NAME.to_string();
        renamed.dependency_edges[0].to_locator = FUNCTION_F_RENAMED_LOCATOR.to_string();

        let renamed_receipt = validate_function_address_receipt(&renamed)
            .receipt
            .expect("renamed receipt");

        assert_eq!(renamed_receipt.receipt_identity, original.receipt_identity);
    }

    #[test]
    fn valence_projection_preserves_dependencies_and_non_claims() {
        let receipt = valid_receipt();

        let projection = export_function_address_valence_projection(&receipt);

        assert_eq!(projection.source_receipt_identity, receipt.receipt_identity);
        assert_eq!(
            projection.dependency_edges.len(),
            receipt.body.dependency_edges.len()
        );
        assert!(
            projection
                .non_claims
                .iter()
                .any(|claim| claim == NON_CLAIM_RELEASE_ELIGIBILITY)
        );
        assert!(
            projection
                .artifact_identities
                .iter()
                .any(|artifact| artifact.role == "function-address")
        );
    }

    #[test]
    fn stale_source_identity_is_rejected() {
        let mut input = valid_input();
        input.source_envelopes[0].source_identity =
            compute_identity(IdentityDomain::Source, b"stale");

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::StaleIdentity)
        );
    }

    #[test]
    fn duplicate_locator_is_rejected() {
        let mut input = valid_input();
        input.functions[1].locator = FUNCTION_F_LOCATOR.to_string();

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::DuplicateArtifactIdentity)
        );
    }

    #[test]
    fn missing_dependency_target_is_rejected() {
        let mut input = valid_input();
        input.dependency_edges[0].to_locator = "src/lib.rs::absent".to_string();

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field == "dependency_edges.to_locator")
        );
    }

    #[test]
    fn missing_non_claim_and_overclaim_text_are_rejected() {
        let mut input = valid_input();
        input.non_claims.clear();
        input
            .semantic_claims
            .push(CLAIM_RELEASE_ELIGIBILITY.to_string());

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MissingNonClaim)
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::SemanticOverclaim)
        );
    }

    #[test]
    fn wrong_identity_domain_is_rejected() {
        let mut input = valid_input();
        input.functions[0].function_address =
            compute_identity(IdentityDomain::Source, b"wrong-domain");

        let result = validate_function_address_receipt(&input);

        assert!(!result.passed);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::IdentityDomainMismatch)
        );
    }

    #[test]
    fn malformed_blake3_digest_fails_deserialization() {
        let malformed = Blake3Hex::parse(INVALID_SHORT_HEX);
        assert!(matches!(
            malformed,
            Err(IdentityError::InvalidLength { .. })
        ));
    }
}
