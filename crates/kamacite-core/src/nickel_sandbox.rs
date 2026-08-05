// r[impl kamacite.findings.nickel_sandbox.profile]
// r[impl kamacite.findings.nickel_sandbox.import_closure]
// r[impl kamacite.findings.nickel_sandbox.enforced_evidence]
// r[impl kamacite.findings.nickel_sandbox.resource_limits]
use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::identity::{AdapterIdentity, IdentityDomain, WireIdentity};
use crate::receipt::{ReceiptError, canonical_json_bytes};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const NICKEL_SANDBOX_SCHEMA_VERSION: &str = "kamacite.nickel-sandbox.v1";
pub const NICKEL_SANDBOX_KIND: &str = "bubblewrap-linux-v1";
pub const NICKEL_SANDBOX_EVALUATOR_POLICY: &str = "nickel-reference-cli";
pub const NICKEL_SANDBOX_TIMEOUT_MILLISECONDS: u64 = 5_000;
pub const NICKEL_SANDBOX_CPU_SECONDS: u64 = 5;
pub const NICKEL_SANDBOX_MEMORY_LIMIT_BYTES: u64 = 1_073_741_824;
pub const NICKEL_SANDBOX_OUTPUT_LIMIT_BYTES: u64 = 1_048_576;
pub const NICKEL_SANDBOX_DIAGNOSTIC_LIMIT_BYTES: u64 = 65_536;

const REQUIREMENT_PROFILE: &str = "r[kamacite.findings.nickel_sandbox.profile]";
const REQUIREMENT_IMPORT_CLOSURE: &str = "r[kamacite.findings.nickel_sandbox.import_closure]";
const REQUIREMENT_EVIDENCE: &str = "r[kamacite.findings.nickel_sandbox.enforced_evidence]";
const REQUIREMENT_LIMITS: &str = "r[kamacite.findings.nickel_sandbox.resource_limits]";
const SANDBOX_HOME: &str = "/homeless";
const SANDBOX_LOCALE: &str = "C";
const SANDBOX_TIMEZONE: &str = "UTC";
const SANDBOX_TEMPORARY_DIRECTORY: &str = "/tmp";
pub const NICKEL_SANDBOX_SOURCE_PATH: &str = "/work/source.ncl";
pub const NICKEL_SANDBOX_IMPORT_ROOT: &str = "/work";
const CAPABILITY_FIXTURE_READ: &str = "fixture-read";
const CAPABILITY_REFERENCE_EXPORT: &str = "nickel-reference-export";
const CAPABILITY_CAPTURE_STDOUT: &str = "capture-stdout";
const OBSERVATION_SANDBOX: &str = "sandbox-enforced";
const OBSERVATION_IMPORT_CLOSURE: &str = "read-only-import-closure";
const OBSERVATION_ENVIRONMENT: &str = "environment-sanitized";
const OBSERVATION_NETWORK: &str = "network-isolated";
const OBSERVATION_OUTPUT: &str = "bounded-output";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NickelSandboxLimits {
    pub execution_timeout_milliseconds: u64,
    pub cpu_seconds: u64,
    pub memory_limit_bytes: u64,
    pub output_limit_bytes: u64,
    pub diagnostic_limit_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NickelSandboxProfile {
    pub schema_version: String,
    pub sandbox_kind: String,
    pub evaluator_profile: String,
    pub environment: BTreeMap<String, String>,
    pub read_only_source_path: String,
    pub read_only_import_root: String,
    pub network_allowed: bool,
    pub process_context_visible: bool,
    pub writable_paths: Vec<String>,
    pub admitted_capabilities: Vec<String>,
    pub limits: NickelSandboxLimits,
    pub profile_identity: AdapterIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NickelSandboxOutcome {
    Success,
    ExitFailure,
    Timeout,
    ResourceLimit,
    OutputLimit,
    DiagnosticLimit,
    LaunchFailure,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NickelSandboxEvidence {
    pub profile_identity: WireIdentity,
    pub evaluator_identity: WireIdentity,
    pub import_identities: BTreeMap<String, WireIdentity>,
    pub requested_capabilities: Vec<String>,
    pub admitted_capabilities: Vec<String>,
    pub expected_effects: Vec<String>,
    pub observed_execution_facts: Vec<String>,
    pub outcome: NickelSandboxOutcome,
    pub exit_code: Option<i32>,
    pub stdout_identity: Option<WireIdentity>,
    pub stderr_identity: WireIdentity,
    pub sandbox_enforced: bool,
}

#[derive(Serialize)]
struct NickelSandboxProfileBody<'a> {
    schema_version: &'a str,
    sandbox_kind: &'a str,
    evaluator_profile: &'a str,
    environment: &'a BTreeMap<String, String>,
    read_only_source_path: &'a str,
    read_only_import_root: &'a str,
    network_allowed: bool,
    process_context_visible: bool,
    writable_paths: &'a [String],
    admitted_capabilities: &'a [String],
    limits: &'a NickelSandboxLimits,
}

pub fn nickel_sandbox_profile() -> Result<NickelSandboxProfile, ReceiptError> {
    let mut environment = BTreeMap::new();
    environment.insert("HOME".to_string(), SANDBOX_HOME.to_string());
    environment.insert("LANG".to_string(), SANDBOX_LOCALE.to_string());
    environment.insert("LC_ALL".to_string(), SANDBOX_LOCALE.to_string());
    environment.insert("TZ".to_string(), SANDBOX_TIMEZONE.to_string());
    let writable_paths = vec![SANDBOX_TEMPORARY_DIRECTORY.to_string()];
    let admitted_capabilities = vec![
        CAPABILITY_CAPTURE_STDOUT.to_string(),
        CAPABILITY_FIXTURE_READ.to_string(),
        CAPABILITY_REFERENCE_EXPORT.to_string(),
    ];
    let limits = NickelSandboxLimits {
        execution_timeout_milliseconds: NICKEL_SANDBOX_TIMEOUT_MILLISECONDS,
        cpu_seconds: NICKEL_SANDBOX_CPU_SECONDS,
        memory_limit_bytes: NICKEL_SANDBOX_MEMORY_LIMIT_BYTES,
        output_limit_bytes: NICKEL_SANDBOX_OUTPUT_LIMIT_BYTES,
        diagnostic_limit_bytes: NICKEL_SANDBOX_DIAGNOSTIC_LIMIT_BYTES,
    };
    let body = NickelSandboxProfileBody {
        schema_version: NICKEL_SANDBOX_SCHEMA_VERSION,
        sandbox_kind: NICKEL_SANDBOX_KIND,
        evaluator_profile: NICKEL_SANDBOX_EVALUATOR_POLICY,
        environment: &environment,
        read_only_source_path: NICKEL_SANDBOX_SOURCE_PATH,
        read_only_import_root: NICKEL_SANDBOX_IMPORT_ROOT,
        network_allowed: false,
        process_context_visible: false,
        writable_paths: &writable_paths,
        admitted_capabilities: &admitted_capabilities,
        limits: &limits,
    };
    let profile_identity = AdapterIdentity::compute(&canonical_json_bytes(&body)?);
    Ok(NickelSandboxProfile {
        schema_version: NICKEL_SANDBOX_SCHEMA_VERSION.to_string(),
        sandbox_kind: NICKEL_SANDBOX_KIND.to_string(),
        evaluator_profile: NICKEL_SANDBOX_EVALUATOR_POLICY.to_string(),
        environment,
        read_only_source_path: NICKEL_SANDBOX_SOURCE_PATH.to_string(),
        read_only_import_root: NICKEL_SANDBOX_IMPORT_ROOT.to_string(),
        network_allowed: false,
        process_context_visible: false,
        writable_paths,
        admitted_capabilities,
        limits,
        profile_identity,
    })
}

pub fn nickel_sandbox_evidence_identity(
    evidence: &NickelSandboxEvidence,
) -> Result<AdapterIdentity, ReceiptError> {
    Ok(AdapterIdentity::compute(&canonical_json_bytes(evidence)?))
}

#[must_use]
pub fn nickel_sandbox_observed_facts() -> Vec<String> {
    vec![
        OBSERVATION_ENVIRONMENT.to_string(),
        OBSERVATION_NETWORK.to_string(),
        OBSERVATION_OUTPUT.to_string(),
        OBSERVATION_IMPORT_CLOSURE.to_string(),
        OBSERVATION_SANDBOX.to_string(),
    ]
}

#[must_use]
pub fn validate_nickel_sandbox(
    profile: &NickelSandboxProfile,
    evidence: &NickelSandboxEvidence,
    declared_capabilities: &[String],
    projection_identity: Option<&WireIdentity>,
    evaluator_identity: Option<&WireIdentity>,
) -> Vec<Diagnostic> {
    let mut diagnostics = validate_nickel_sandbox_profile(profile);
    if evidence.profile_identity != profile.profile_identity.to_wire() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_PROFILE,
            "nickel.sandbox_evidence.profile_identity",
            "Nickel sandbox evidence does not bind the admitted sandbox profile",
        ));
    }
    if evidence
        .evaluator_identity
        .validate_domain(IdentityDomain::Evaluator)
        .is_err()
        || evaluator_identity != Some(&evidence.evaluator_identity)
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UnsupportedEvaluatorIdentity,
            REQUIREMENT_EVIDENCE,
            "nickel.sandbox_evidence.evaluator_identity",
            "Nickel sandbox evidence must bind the observed evaluator identity",
        ));
    }
    if !evidence.sandbox_enforced {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UndeclaredAuthority,
            REQUIREMENT_EVIDENCE,
            "nickel.sandbox_evidence.sandbox_enforced",
            "Nickel execution evidence does not establish shell-enforced containment",
        ));
    }
    validate_import_identities(&evidence.import_identities, &mut diagnostics);
    validate_capability_sets(profile, evidence, declared_capabilities, &mut diagnostics);
    validate_observed_execution_facts(evidence, &mut diagnostics);
    validate_sandbox_outcome(evidence, projection_identity, &mut diagnostics);
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

#[must_use]
pub fn validate_nickel_sandbox_profile(profile: &NickelSandboxProfile) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let expected = match nickel_sandbox_profile() {
        Ok(expected) => expected,
        Err(error) => {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::MalformedProjectionOutput,
                REQUIREMENT_PROFILE,
                "nickel.sandbox_profile",
                format!("Nickel sandbox profile identity failed: {error}"),
            ));
            return diagnostics;
        }
    };
    if profile != &expected {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UndeclaredAuthority,
            REQUIREMENT_PROFILE,
            "nickel.sandbox_profile",
            "Nickel sandbox profile differs from the accepted denied-by-default profile",
        ));
    }
    diagnostics
}

fn validate_import_identities(
    imports: &BTreeMap<String, WireIdentity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if imports.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_IMPORT_CLOSURE,
            "nickel.sandbox_evidence.import_identities",
            "Nickel sandbox evidence must bind the source/import closure",
        ));
    }
    for (path, identity) in imports {
        if path.is_empty() || identity.validate_domain(IdentityDomain::Source).is_err() {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::IdentityDomainMismatch,
                REQUIREMENT_IMPORT_CLOSURE,
                "nickel.sandbox_evidence.import_identities",
                "Nickel import closure entries require non-empty paths and source identities",
            ));
        }
    }
}

fn validate_capability_sets(
    profile: &NickelSandboxProfile,
    evidence: &NickelSandboxEvidence,
    declared_capabilities: &[String],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let requested = evidence
        .requested_capabilities
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let declared = declared_capabilities
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let admitted = evidence
        .admitted_capabilities
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let profile_admitted = profile
        .admitted_capabilities
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if requested != declared {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UndeclaredCapability,
            REQUIREMENT_EVIDENCE,
            "nickel.sandbox_evidence.requested_capabilities",
            "Nickel requested capabilities differ from the adapter declaration",
        ));
    }
    if admitted != profile_admitted || !requested.is_subset(&admitted) {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UndeclaredCapability,
            REQUIREMENT_EVIDENCE,
            "nickel.sandbox_evidence.admitted_capabilities",
            "Nickel sandbox admitted capabilities differ from the enforced profile",
        ));
    }
}

fn validate_observed_execution_facts(
    evidence: &NickelSandboxEvidence,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let expected = nickel_sandbox_observed_facts()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let observed = evidence
        .observed_execution_facts
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if observed != expected {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::UndeclaredAuthority,
            REQUIREMENT_EVIDENCE,
            "nickel.sandbox_evidence.observed_execution_facts",
            "Nickel containment facts must come from the enforced shell profile",
        ));
    }
}

fn validate_sandbox_outcome(
    evidence: &NickelSandboxEvidence,
    projection_identity: Option<&WireIdentity>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match evidence.outcome {
        NickelSandboxOutcome::Success => {
            if evidence.exit_code != Some(0) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::MalformedProjectionOutput,
                    REQUIREMENT_EVIDENCE,
                    "nickel.sandbox_evidence.exit_code",
                    "successful Nickel sandbox evidence requires exit code zero",
                ));
            }
        }
        NickelSandboxOutcome::Timeout
        | NickelSandboxOutcome::ResourceLimit
        | NickelSandboxOutcome::OutputLimit
        | NickelSandboxOutcome::DiagnosticLimit => diagnostics.push(
            Diagnostic::new(
                DiagnosticCode::ResourceLimitExceeded,
                REQUIREMENT_LIMITS,
                "nickel.sandbox_evidence.outcome",
                "Nickel evaluator exceeded a named sandbox resource limit",
            )
            .with_detail("outcome", format!("{:?}", evidence.outcome))
            .with_detail(
                "exit_code",
                evidence
                    .exit_code
                    .map_or_else(|| "signal".to_string(), |code| code.to_string()),
            ),
        ),
        NickelSandboxOutcome::ExitFailure | NickelSandboxOutcome::LaunchFailure => {
            diagnostics.push(
                Diagnostic::new(
                    DiagnosticCode::MalformedProjectionOutput,
                    REQUIREMENT_EVIDENCE,
                    "nickel.sandbox_evidence.outcome",
                    "Nickel sandbox did not produce a successful export",
                )
                .with_detail("outcome", format!("{:?}", evidence.outcome))
                .with_detail(
                    "exit_code",
                    evidence
                        .exit_code
                        .map_or_else(|| "signal".to_string(), |code| code.to_string()),
                ),
            );
        }
    }
    if let (Some(declared), Some(observed)) = (projection_identity, &evidence.stdout_identity)
        && declared != observed
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::StaleIdentity,
            REQUIREMENT_EVIDENCE,
            "nickel.sandbox_evidence.stdout_identity",
            "Nickel sandbox stdout identity differs from the export projection identity",
        ));
    }
    if evidence.stdout_identity.is_none() && evidence.outcome == NickelSandboxOutcome::Success {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::MissingInput,
            REQUIREMENT_EVIDENCE,
            "nickel.sandbox_evidence.stdout_identity",
            "successful Nickel sandbox evidence must bind bounded stdout",
        ));
    }
    if evidence
        .stderr_identity
        .validate_domain(IdentityDomain::Projection)
        .is_err()
    {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::IdentityDomainMismatch,
            REQUIREMENT_EVIDENCE,
            "nickel.sandbox_evidence.stderr_identity",
            "Nickel sandbox diagnostics must use the projection identity domain",
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::compute_identity;

    fn valid_evidence(profile: &NickelSandboxProfile) -> NickelSandboxEvidence {
        let output = b"{\"accepted\":true}";
        let mut imports = BTreeMap::new();
        imports.insert(
            "source.ncl".to_string(),
            compute_identity(IdentityDomain::Source, b"{ accepted = true }"),
        );
        NickelSandboxEvidence {
            profile_identity: profile.profile_identity.to_wire(),
            evaluator_identity: compute_identity(
                IdentityDomain::Evaluator,
                b"nickel-reference-cli:test",
            ),
            import_identities: imports,
            requested_capabilities: profile.admitted_capabilities.clone(),
            admitted_capabilities: profile.admitted_capabilities.clone(),
            expected_effects: Vec::new(),
            observed_execution_facts: nickel_sandbox_observed_facts(),
            outcome: NickelSandboxOutcome::Success,
            exit_code: Some(0),
            stdout_identity: Some(compute_identity(IdentityDomain::Projection, output)),
            stderr_identity: compute_identity(IdentityDomain::Projection, b""),
            sandbox_enforced: true,
        }
    }

    #[test]
    fn accepted_profile_and_enforced_evidence_validate() {
        let profile = nickel_sandbox_profile();
        assert!(profile.is_ok());
        let Ok(profile) = profile else {
            return;
        };
        let evidence = valid_evidence(&profile);
        let diagnostics = validate_nickel_sandbox(
            &profile,
            &evidence,
            &profile.admitted_capabilities,
            evidence.stdout_identity.as_ref(),
            Some(&evidence.evaluator_identity),
        );
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
    }

    #[test]
    fn substituted_evaluator_identity_is_rejected() {
        const SUBSTITUTED_EVALUATOR_DESCRIPTOR: &[u8] = b"nickel-wrapper:substituted";

        let profile = nickel_sandbox_profile();
        assert!(profile.is_ok());
        let Ok(profile) = profile else {
            return;
        };
        let mut evidence = valid_evidence(&profile);
        let declared_evaluator_identity = evidence.evaluator_identity.clone();
        evidence.evaluator_identity =
            compute_identity(IdentityDomain::Evaluator, SUBSTITUTED_EVALUATOR_DESCRIPTOR);

        let diagnostics = validate_nickel_sandbox(
            &profile,
            &evidence,
            &profile.admitted_capabilities,
            evidence.stdout_identity.as_ref(),
            Some(&declared_evaluator_identity),
        );
        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.code == DiagnosticCode::UnsupportedEvaluatorIdentity
            })
        );
    }

    #[test]
    fn self_reported_or_unbounded_evidence_is_rejected() {
        let profile = nickel_sandbox_profile();
        assert!(profile.is_ok());
        let Ok(profile) = profile else {
            return;
        };
        let mut evidence = valid_evidence(&profile);
        evidence.sandbox_enforced = false;
        evidence.admitted_capabilities.push("network".to_string());
        evidence.observed_execution_facts = evidence.expected_effects.clone();
        evidence.outcome = NickelSandboxOutcome::Timeout;
        evidence.exit_code = None;
        evidence.stdout_identity = None;

        let diagnostics = validate_nickel_sandbox(
            &profile,
            &evidence,
            &profile.admitted_capabilities,
            None,
            Some(&evidence.evaluator_identity),
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::UndeclaredAuthority })
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::UndeclaredCapability })
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| { diagnostic.code == DiagnosticCode::ResourceLimitExceeded })
        );
    }
}
