//! Nominal identity domains as distinct Rust types.
//!
//! Kamacite maintains several canonical identity domains over common BLAKE3
//! text. The legacy [`crate::identity::Identity`] record stores the domain
//! beside the digest and checks expected domains at runtime. This module adds
//! a nominal layer on top of that record:
//!
//! - [`TypedIdentity<Domain>`] stores validated BLAKE3 content behind a
//!   zero-sized role marker. Cross-domain values are distinct types, so
//!   passing a receipt identity to a source-identity API does not compile.
//! - [`TypedIdentityRef<'a, Domain>`] is the borrowed view for adapter paths.
//! - [`WireIdentity`] is the versioned interchange record. Its emitted JSON
//!   shape is byte-identical to the legacy [`crate::identity::Identity`]
//!   record for current versions, so canonical projections stay stable.
//! - [`AnyIdentity`] is the closed heterogeneous enum for manifests and
//!   receipts that carry several domains. Admission converts a wire record
//!   into the matching typed variant or fails closed.
//!
//! The generic type here is named [`TypedIdentity`] rather than
//! `Identity<Domain>` because the legacy wire struct already owns that name.
//! A later bounded migration step renames the legacy wire type and can then
//! adopt the `Identity<Domain>` name without changing behavior.
//!
//! # Claim boundary
//!
//! Nominal identity types prove local domain separation and checked identity
//! syntax only. They do not prove source fidelity, adapter laws, semantic
//! equivalence, authority, runtime correctness, or release eligibility.
//!
//! # Compile-time domain separation
//!
//! A same-domain flow compiles and runs:
//!
//! ```
//! use kamacite_core::SourceIdentity;
//!
//! fn requires_source(_identity: SourceIdentity) {}
//!
//! let identity = SourceIdentity::compute(b"exact source bytes");
//! requires_source(identity);
//! ```
//!
//! A receipt identity cannot satisfy a source-identity parameter:
//!
//! ```compile_fail
//! use kamacite_core::{ReceiptIdentity, SourceIdentity};
//!
//! fn requires_source(_identity: SourceIdentity) {}
//!
//! let identity = ReceiptIdentity::compute(b"exact source bytes");
//! requires_source(identity);
//! ```
//!
//! An adapter identity cannot satisfy a target-artifact parameter:
//!
//! ```compile_fail
//! use kamacite_core::{AdapterIdentity, TargetArtifactIdentity};
//!
//! fn requires_target(_identity: TargetArtifactIdentity) {}
//!
//! let identity = AdapterIdentity::compute(b"exact adapter bytes");
//! requires_target(identity);
//! ```
//!
//! An evaluator identity cannot satisfy a compiler parameter:
//!
//! ```compile_fail
//! use kamacite_core::{CompilerIdentity, EvaluatorIdentity};
//!
//! fn requires_compiler(_identity: CompilerIdentity) {}
//!
//! let identity = EvaluatorIdentity::compute(b"exact evaluator bytes");
//! requires_compiler(identity);
//! ```
//!
//! A canonical-value identity cannot satisfy a projection parameter:
//!
//! ```compile_fail
//! use kamacite_core::{CanonicalValueIdentity, ProjectionIdentity};
//!
//! fn requires_projection(_identity: ProjectionIdentity) {}
//!
//! let identity = CanonicalValueIdentity::compute(b"exact value bytes");
//! requires_projection(identity);
//! ```
//!
//! An algebraic-effect identity cannot satisfy an effect-operation parameter:
//!
//! ```compile_fail
//! use kamacite_core::{AlgebraicEffectIdentity, EffectOperationIdentity};
//!
//! fn requires_operation(_identity: EffectOperationIdentity) {}
//!
//! let identity = AlgebraicEffectIdentity::compute(b"exact effect bytes");
//! requires_operation(identity);
//! ```

use crate::identity::{Blake3Hex, IdentityDomain, IdentityError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

/// Wire record version currently emitted and admitted.
pub const WIRE_IDENTITY_VERSION: u8 = 1;

/// The only supported digest algorithm for admitted identities.
const SUPPORTED_ALGORITHM: &str = "blake3";

/// Deterministic admission and construction diagnostics for nominal identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityAdmissionError {
    /// The digest text is empty.
    EmptyHash,
    /// The digest text has the wrong length.
    InvalidLength { actual: usize, expected: usize },
    /// The digest text is not lowercase hexadecimal.
    InvalidHex,
    /// The requested algorithm is not supported.
    UnsupportedAlgorithm { actual: String },
    /// The declared domain label has no maintained identity domain.
    UnknownDomain { label: String },
    /// The declared wire version is not the current version.
    StaleIdentity { version: u8, expected: u8 },
    /// A typed identity received a wire record for another domain.
    WrongDomain { actual: String, expected: String },
}

impl Display for IdentityAdmissionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyHash => formatter.write_str("identity digest is empty"),
            Self::InvalidLength { actual, expected } => {
                write!(formatter, "identity digest length {actual} != {expected}")
            }
            Self::InvalidHex => formatter.write_str("identity digest is not lowercase hex"),
            Self::UnsupportedAlgorithm { actual } => {
                write!(
                    formatter,
                    "identity algorithm {actual} is not {SUPPORTED_ALGORITHM}"
                )
            }
            Self::UnknownDomain { label } => {
                write!(
                    formatter,
                    "identity domain label {label} is not a maintained domain"
                )
            }
            Self::StaleIdentity { version, expected } => {
                write!(
                    formatter,
                    "identity wire version {version} is not the current version {expected}"
                )
            }
            Self::WrongDomain { actual, expected } => {
                write!(
                    formatter,
                    "identity domain {actual} is not the expected domain {expected}"
                )
            }
        }
    }
}

impl std::error::Error for IdentityAdmissionError {}

fn map_hex_error(error: IdentityError) -> IdentityAdmissionError {
    match error {
        IdentityError::EmptyHash => IdentityAdmissionError::EmptyHash,
        IdentityError::InvalidLength { actual, expected } => {
            IdentityAdmissionError::InvalidLength { actual, expected }
        }
        IdentityError::InvalidHex => IdentityAdmissionError::InvalidHex,
        IdentityError::WrongAlgorithm { actual } => {
            IdentityAdmissionError::UnsupportedAlgorithm { actual }
        }
        IdentityError::WrongDomain { actual, expected } => IdentityAdmissionError::WrongDomain {
            actual: actual.to_string(),
            expected: expected.to_string(),
        },
    }
}

/// Role marker that names one maintained identity domain.
///
/// Markers are zero-sized and do not carry values. They exist so that
/// [`TypedIdentity`] instantiations for different domains are distinct Rust
/// types. A marker is a public unit struct so the generic types remain usable
/// in public signatures without private-in-public lint friction.
pub trait IdentityDomainMarker: Sized {
    /// The kebab-case wire label for this domain.
    const LABEL: &'static str;
    /// The canonical wire domain record for this domain.
    fn domain() -> IdentityDomain;
}

/// Owned nominal identity over validated BLAKE3 content for one domain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedIdentity<Domain> {
    hex: Blake3Hex,
    marker: PhantomData<Domain>,
}

impl<Domain: IdentityDomainMarker> TypedIdentity<Domain> {
    /// Compute the domain identity over the exact canonical input bytes.
    pub fn compute(bytes: &[u8]) -> Self {
        Self {
            hex: Blake3Hex::from_bytes(bytes),
            marker: PhantomData,
        }
    }

    /// Construct from validated lowercase BLAKE3 hex text.
    pub fn from_hex(hex: impl Into<String>) -> Result<Self, IdentityAdmissionError> {
        let hex = Blake3Hex::parse(hex.into()).map_err(map_hex_error)?;
        Ok(Self {
            hex,
            marker: PhantomData,
        })
    }

    /// Admit a versioned wire record and bind it to this exact domain.
    pub fn from_wire(wire: WireIdentity) -> Result<Self, IdentityAdmissionError> {
        if wire.version != WIRE_IDENTITY_VERSION {
            return Err(IdentityAdmissionError::StaleIdentity {
                version: wire.version,
                expected: WIRE_IDENTITY_VERSION,
            });
        }
        if wire.algorithm != SUPPORTED_ALGORITHM {
            return Err(IdentityAdmissionError::UnsupportedAlgorithm {
                actual: wire.algorithm,
            });
        }
        if wire.domain != Domain::domain() {
            return Err(IdentityAdmissionError::WrongDomain {
                actual: wire.domain.to_string(),
                expected: Domain::LABEL.to_string(),
            });
        }
        let hex = Blake3Hex::parse(wire.hex.as_str().to_string()).map_err(map_hex_error)?;
        Ok(Self {
            hex,
            marker: PhantomData,
        })
    }

    /// Project this typed identity to its versioned wire record.
    pub fn as_wire(&self) -> WireIdentity {
        WireIdentity {
            domain: Domain::domain(),
            algorithm: SUPPORTED_ALGORITHM.to_string(),
            hex: self.hex.clone(),
            version: WIRE_IDENTITY_VERSION,
        }
    }

    /// The validated lowercase BLAKE3 hex text.
    pub fn hex_str(&self) -> &str {
        self.hex.as_str()
    }

    /// The canonical wire domain for this identity.
    pub fn domain(&self) -> IdentityDomain {
        Domain::domain()
    }

    /// A borrowed view of this identity.
    pub fn as_ref(&self) -> TypedIdentityRef<'_, Domain> {
        TypedIdentityRef {
            hex: self.hex.as_str(),
            marker: PhantomData,
        }
    }
}

impl<Domain: IdentityDomainMarker> Serialize for TypedIdentity<Domain> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_wire().serialize(serializer)
    }
}

impl<'de, Domain: IdentityDomainMarker> Deserialize<'de> for TypedIdentity<Domain> {
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: Deserializer<'de>,
    {
        let wire = WireIdentity::deserialize(deserializer)?;
        Self::from_wire(wire).map_err(serde::de::Error::custom)
    }
}

/// Borrowed nominal identity view over validated BLAKE3 hex text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypedIdentityRef<'a, Domain> {
    hex: &'a str,
    marker: PhantomData<&'a Domain>,
}

impl<'a, Domain: IdentityDomainMarker> TypedIdentityRef<'a, Domain> {
    /// Construct a borrowed view from validated lowercase BLAKE3 hex text.
    pub fn try_new(hex: &'a str) -> Result<Self, IdentityAdmissionError> {
        Blake3Hex::parse(hex.to_string()).map_err(map_hex_error)?;
        Ok(Self {
            hex,
            marker: PhantomData,
        })
    }

    /// The validated lowercase BLAKE3 hex text.
    pub fn hex_str(&self) -> &str {
        self.hex
    }

    /// The owned identity with the same validated content.
    pub fn to_owned(&self) -> TypedIdentity<Domain> {
        match Blake3Hex::parse(self.hex.to_string()) {
            Ok(hex) => TypedIdentity {
                hex,
                marker: PhantomData,
            },
            Err(_) => TypedIdentity {
                // A borrowed view always carries validated hex. This fallback
                // stays deterministic for the impossible malformed case.
                hex: Blake3Hex::from_bytes(self.hex.as_bytes()),
                marker: PhantomData,
            },
        }
    }
}

/// Versioned interchange record for one identity.
///
/// The emitted JSON shape is byte-identical to the legacy
/// [`crate::identity::Identity`] record for the current version: the version
/// field is omitted while it equals [`WIRE_IDENTITY_VERSION`]. A declared
/// version that differs from the current version deserializes and then fails
/// closed at admission.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WireIdentity {
    /// The kebab-case domain label.
    pub domain: IdentityDomain,
    /// The digest algorithm.
    pub algorithm: String,
    /// The validated lowercase BLAKE3 hex digest.
    pub hex: Blake3Hex,
    /// The wire record version. Omitted while current.
    #[serde(
        default = "default_wire_version",
        skip_serializing_if = "is_current_wire_version"
    )]
    pub version: u8,
}

fn default_wire_version() -> u8 {
    WIRE_IDENTITY_VERSION
}

fn is_current_wire_version(version: &u8) -> bool {
    *version == WIRE_IDENTITY_VERSION
}

impl WireIdentity {
    /// A current-version BLAKE3 wire record for the given domain.
    pub fn new(domain: IdentityDomain, hex: Blake3Hex) -> Self {
        Self {
            domain,
            algorithm: SUPPORTED_ALGORITHM.to_string(),
            hex,
            version: WIRE_IDENTITY_VERSION,
        }
    }

    /// Admit this wire record into the closed typed identity enum.
    pub fn admit(self) -> Result<AnyIdentity, IdentityAdmissionError> {
        if self.version != WIRE_IDENTITY_VERSION {
            return Err(IdentityAdmissionError::StaleIdentity {
                version: self.version,
                expected: WIRE_IDENTITY_VERSION,
            });
        }
        if self.algorithm != SUPPORTED_ALGORITHM {
            return Err(IdentityAdmissionError::UnsupportedAlgorithm {
                actual: self.algorithm,
            });
        }
        any_from_checked(self.domain, self.hex)
    }

    /// Admit raw untrusted fields by kebab-case label.
    ///
    /// Unknown labels fail closed before any typed value is constructed.
    pub fn admit_from_parts(
        version: u8,
        algorithm: &str,
        domain_label: &str,
        hex: impl Into<String>,
    ) -> Result<AnyIdentity, IdentityAdmissionError> {
        if version != WIRE_IDENTITY_VERSION {
            return Err(IdentityAdmissionError::StaleIdentity {
                version,
                expected: WIRE_IDENTITY_VERSION,
            });
        }
        if algorithm != SUPPORTED_ALGORITHM {
            return Err(IdentityAdmissionError::UnsupportedAlgorithm {
                actual: algorithm.to_string(),
            });
        }
        let hex = Blake3Hex::parse(hex.into()).map_err(map_hex_error)?;
        let domain = domain_from_label(domain_label).ok_or_else(|| {
            IdentityAdmissionError::UnknownDomain {
                label: domain_label.to_string(),
            }
        })?;
        any_from_checked(domain, hex)
    }

    /// Build a current-version wire record from a legacy wire struct.
    pub fn from_legacy(identity: &crate::identity::Identity) -> Self {
        Self {
            domain: identity.domain.clone(),
            algorithm: identity.algorithm.clone(),
            hex: identity.hex.clone(),
            version: WIRE_IDENTITY_VERSION,
        }
    }
}

impl From<WireIdentity> for crate::identity::Identity {
    fn from(wire: WireIdentity) -> Self {
        Self {
            domain: wire.domain,
            algorithm: wire.algorithm,
            hex: wire.hex,
        }
    }
}

/// The closed heterogeneous enum of admitted domain identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnyIdentity {
    Source(TypedIdentity<Source>),
    CanonicalValue(TypedIdentity<CanonicalValue>),
    Projection(TypedIdentity<Projection>),
    TargetArtifact(TypedIdentity<TargetArtifact>),
    Adapter(TypedIdentity<Adapter>),
    Evaluator(TypedIdentity<Evaluator>),
    Compiler(TypedIdentity<Compiler>),
    Receipt(TypedIdentity<Receipt>),
    FunctionAddress(TypedIdentity<FunctionAddress>),
    DependencyClosure(TypedIdentity<DependencyClosure>),
    AlgebraicEffect(TypedIdentity<AlgebraicEffect>),
    EffectOperation(TypedIdentity<EffectOperation>),
    EffectLog(TypedIdentity<EffectLog>),
    FsmSpec(TypedIdentity<FsmSpec>),
    FsmUniverse(TypedIdentity<FsmUniverse>),
    FsmCoverage(TypedIdentity<FsmCoverage>),
    DocumentationIr(TypedIdentity<DocumentationIr>),
    DocumentationOutput(TypedIdentity<DocumentationOutput>),
    DocumentationFacts(TypedIdentity<DocumentationFacts>),
    DialectPayload(TypedIdentity<DialectPayload>),
    EvidenceChain(TypedIdentity<EvidenceChain>),
}

impl AnyIdentity {
    /// The canonical wire domain for this admitted identity.
    pub fn domain(&self) -> IdentityDomain {
        match self {
            Self::Source(_) => IdentityDomain::Source,
            Self::CanonicalValue(_) => IdentityDomain::CanonicalValue,
            Self::Projection(_) => IdentityDomain::Projection,
            Self::TargetArtifact(_) => IdentityDomain::TargetArtifact,
            Self::Adapter(_) => IdentityDomain::Adapter,
            Self::Evaluator(_) => IdentityDomain::Evaluator,
            Self::Compiler(_) => IdentityDomain::Compiler,
            Self::Receipt(_) => IdentityDomain::Receipt,
            Self::FunctionAddress(_) => IdentityDomain::FunctionAddress,
            Self::DependencyClosure(_) => IdentityDomain::DependencyClosure,
            Self::AlgebraicEffect(_) => IdentityDomain::AlgebraicEffect,
            Self::EffectOperation(_) => IdentityDomain::EffectOperation,
            Self::EffectLog(_) => IdentityDomain::EffectLog,
            Self::FsmSpec(_) => IdentityDomain::FsmSpec,
            Self::FsmUniverse(_) => IdentityDomain::FsmUniverse,
            Self::FsmCoverage(_) => IdentityDomain::FsmCoverage,
            Self::DocumentationIr(_) => IdentityDomain::DocumentationIr,
            Self::DocumentationOutput(_) => IdentityDomain::DocumentationOutput,
            Self::DocumentationFacts(_) => IdentityDomain::DocumentationFacts,
            Self::DialectPayload(_) => IdentityDomain::DialectPayload,
            Self::EvidenceChain(_) => IdentityDomain::EvidenceChain,
        }
    }

    /// Project this admitted identity to its versioned wire record.
    pub fn as_wire(&self) -> WireIdentity {
        WireIdentity::new(self.domain(), self.hex().clone())
    }

    fn hex(&self) -> &Blake3Hex {
        match self {
            Self::Source(identity) => &identity.hex,
            Self::CanonicalValue(identity) => &identity.hex,
            Self::Projection(identity) => &identity.hex,
            Self::TargetArtifact(identity) => &identity.hex,
            Self::Adapter(identity) => &identity.hex,
            Self::Evaluator(identity) => &identity.hex,
            Self::Compiler(identity) => &identity.hex,
            Self::Receipt(identity) => &identity.hex,
            Self::FunctionAddress(identity) => &identity.hex,
            Self::DependencyClosure(identity) => &identity.hex,
            Self::AlgebraicEffect(identity) => &identity.hex,
            Self::EffectOperation(identity) => &identity.hex,
            Self::EffectLog(identity) => &identity.hex,
            Self::FsmSpec(identity) => &identity.hex,
            Self::FsmUniverse(identity) => &identity.hex,
            Self::FsmCoverage(identity) => &identity.hex,
            Self::DocumentationIr(identity) => &identity.hex,
            Self::DocumentationOutput(identity) => &identity.hex,
            Self::DocumentationFacts(identity) => &identity.hex,
            Self::DialectPayload(identity) => &identity.hex,
            Self::EvidenceChain(identity) => &identity.hex,
        }
    }
}

fn any_from_checked(
    domain: IdentityDomain,
    hex: Blake3Hex,
) -> Result<AnyIdentity, IdentityAdmissionError> {
    Ok(match domain {
        IdentityDomain::Source => AnyIdentity::Source(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::CanonicalValue => AnyIdentity::CanonicalValue(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::Projection => AnyIdentity::Projection(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::TargetArtifact => AnyIdentity::TargetArtifact(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::Adapter => AnyIdentity::Adapter(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::Evaluator => AnyIdentity::Evaluator(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::Compiler => AnyIdentity::Compiler(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::Receipt => AnyIdentity::Receipt(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::FunctionAddress => AnyIdentity::FunctionAddress(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::DependencyClosure => AnyIdentity::DependencyClosure(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::AlgebraicEffect => AnyIdentity::AlgebraicEffect(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::EffectOperation => AnyIdentity::EffectOperation(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::EffectLog => AnyIdentity::EffectLog(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::FsmSpec => AnyIdentity::FsmSpec(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::FsmUniverse => AnyIdentity::FsmUniverse(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::FsmCoverage => AnyIdentity::FsmCoverage(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::DocumentationIr => AnyIdentity::DocumentationIr(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::DocumentationOutput => AnyIdentity::DocumentationOutput(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::DocumentationFacts => AnyIdentity::DocumentationFacts(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::DialectPayload => AnyIdentity::DialectPayload(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
        IdentityDomain::EvidenceChain => AnyIdentity::EvidenceChain(TypedIdentity {
            hex,
            marker: PhantomData,
        }),
    })
}

/// Map a kebab-case label to the matching maintained wire domain.
fn domain_from_label(label: &str) -> Option<IdentityDomain> {
    match label {
        "source" => Some(IdentityDomain::Source),
        "canonical-value" => Some(IdentityDomain::CanonicalValue),
        "projection" => Some(IdentityDomain::Projection),
        "target-artifact" => Some(IdentityDomain::TargetArtifact),
        "adapter" => Some(IdentityDomain::Adapter),
        "evaluator" => Some(IdentityDomain::Evaluator),
        "compiler" => Some(IdentityDomain::Compiler),
        "receipt" => Some(IdentityDomain::Receipt),
        "function-address" => Some(IdentityDomain::FunctionAddress),
        "dependency-closure" => Some(IdentityDomain::DependencyClosure),
        "algebraic-effect" => Some(IdentityDomain::AlgebraicEffect),
        "effect-operation" => Some(IdentityDomain::EffectOperation),
        "effect-log" => Some(IdentityDomain::EffectLog),
        "fsm-spec" => Some(IdentityDomain::FsmSpec),
        "fsm-universe" => Some(IdentityDomain::FsmUniverse),
        "fsm-coverage" => Some(IdentityDomain::FsmCoverage),
        "documentation-ir" => Some(IdentityDomain::DocumentationIr),
        "documentation-output" => Some(IdentityDomain::DocumentationOutput),
        "documentation-facts" => Some(IdentityDomain::DocumentationFacts),
        "dialect-payload" => Some(IdentityDomain::DialectPayload),
        "evidence-chain" => Some(IdentityDomain::EvidenceChain),
        _ => None,
    }
}

macro_rules! identity_domain {
    ($label:literal, $marker:ident, $variant:ident, $owned:ident, $borrowed:ident, $downcast:ident) => {
        #[doc = concat!("Zero-sized role marker for the `", $label, "` identity domain.")]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $marker;

        impl IdentityDomainMarker for $marker {
            const LABEL: &'static str = $label;
            fn domain() -> IdentityDomain {
                IdentityDomain::$variant
            }
        }

        #[doc = concat!("Owned typed identity alias for the `", $label, "` domain.")]
        pub type $owned = TypedIdentity<$marker>;

        #[doc = concat!("Borrowed typed identity alias for the `", $label, "` domain.")]
        pub type $borrowed<'a> = TypedIdentityRef<'a, $marker>;

        impl $owned {
            /// Move this typed identity into the closed heterogeneous enum.
            pub fn into_any(self) -> AnyIdentity {
                AnyIdentity::$variant(self)
            }
        }

        impl AnyIdentity {
            #[doc = concat!("Downcast to the typed `", $label, "` identity, if admitted as that domain.")]
            pub fn $downcast(&self) -> Option<&$owned> {
                match self {
                    AnyIdentity::$variant(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}

identity_domain!(
    "source",
    Source,
    Source,
    SourceIdentity,
    SourceIdentityRef,
    as_source
);
identity_domain!(
    "canonical-value",
    CanonicalValue,
    CanonicalValue,
    CanonicalValueIdentity,
    CanonicalValueIdentityRef,
    as_canonical_value
);
identity_domain!(
    "projection",
    Projection,
    Projection,
    ProjectionIdentity,
    ProjectionIdentityRef,
    as_projection
);
identity_domain!(
    "target-artifact",
    TargetArtifact,
    TargetArtifact,
    TargetArtifactIdentity,
    TargetArtifactIdentityRef,
    as_target_artifact
);
identity_domain!(
    "adapter",
    Adapter,
    Adapter,
    AdapterIdentity,
    AdapterIdentityRef,
    as_adapter
);
identity_domain!(
    "evaluator",
    Evaluator,
    Evaluator,
    EvaluatorIdentity,
    EvaluatorIdentityRef,
    as_evaluator
);
identity_domain!(
    "compiler",
    Compiler,
    Compiler,
    CompilerIdentity,
    CompilerIdentityRef,
    as_compiler
);
identity_domain!(
    "receipt",
    Receipt,
    Receipt,
    ReceiptIdentity,
    ReceiptIdentityRef,
    as_receipt
);
identity_domain!(
    "function-address",
    FunctionAddress,
    FunctionAddress,
    FunctionAddressIdentity,
    FunctionAddressIdentityRef,
    as_function_address
);
identity_domain!(
    "dependency-closure",
    DependencyClosure,
    DependencyClosure,
    DependencyClosureIdentity,
    DependencyClosureIdentityRef,
    as_dependency_closure
);
identity_domain!(
    "algebraic-effect",
    AlgebraicEffect,
    AlgebraicEffect,
    AlgebraicEffectIdentity,
    AlgebraicEffectIdentityRef,
    as_algebraic_effect
);
identity_domain!(
    "effect-operation",
    EffectOperation,
    EffectOperation,
    EffectOperationIdentity,
    EffectOperationIdentityRef,
    as_effect_operation
);
identity_domain!(
    "effect-log",
    EffectLog,
    EffectLog,
    EffectLogIdentity,
    EffectLogIdentityRef,
    as_effect_log
);
identity_domain!(
    "fsm-spec",
    FsmSpec,
    FsmSpec,
    FsmSpecIdentity,
    FsmSpecIdentityRef,
    as_fsm_spec
);
identity_domain!(
    "fsm-universe",
    FsmUniverse,
    FsmUniverse,
    FsmUniverseIdentity,
    FsmUniverseIdentityRef,
    as_fsm_universe
);
identity_domain!(
    "fsm-coverage",
    FsmCoverage,
    FsmCoverage,
    FsmCoverageIdentity,
    FsmCoverageIdentityRef,
    as_fsm_coverage
);
identity_domain!(
    "documentation-ir",
    DocumentationIr,
    DocumentationIr,
    DocumentationIrIdentity,
    DocumentationIrIdentityRef,
    as_documentation_ir
);
identity_domain!(
    "documentation-output",
    DocumentationOutput,
    DocumentationOutput,
    DocumentationOutputIdentity,
    DocumentationOutputIdentityRef,
    as_documentation_output
);
identity_domain!(
    "documentation-facts",
    DocumentationFacts,
    DocumentationFacts,
    DocumentationFactsIdentity,
    DocumentationFactsIdentityRef,
    as_documentation_facts
);
identity_domain!(
    "dialect-payload",
    DialectPayload,
    DialectPayload,
    DialectPayloadIdentity,
    DialectPayloadIdentityRef,
    as_dialect_payload
);
identity_domain!(
    "evidence-chain",
    EvidenceChain,
    EvidenceChain,
    EvidenceChainIdentity,
    EvidenceChainIdentityRef,
    as_evidence_chain
);

#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::panic,
        clippy::type_complexity,
        clippy::unwrap_used
    )]

    use super::*;
    use crate::identity::{BLAKE3_HEX_LENGTH, Identity, compute_identity};
    use serde_json::json;

    const SOURCE_BYTES: &[u8] = b"exact source bytes";
    const RECEIPT_BYTES: &[u8] = b"exact receipt bytes";

    #[test]
    fn every_domain_computes_admits_and_projects_legacy_identical_json() {
        let cases: Vec<(IdentityDomain, &str, Box<dyn Fn(&[u8]) -> AnyIdentity>)> = vec![
            (
                IdentityDomain::Source,
                "source",
                Box::new(|b| SourceIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::CanonicalValue,
                "canonical-value",
                Box::new(|b| CanonicalValueIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::Projection,
                "projection",
                Box::new(|b| ProjectionIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::TargetArtifact,
                "target-artifact",
                Box::new(|b| TargetArtifactIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::Adapter,
                "adapter",
                Box::new(|b| AdapterIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::Evaluator,
                "evaluator",
                Box::new(|b| EvaluatorIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::Compiler,
                "compiler",
                Box::new(|b| CompilerIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::Receipt,
                "receipt",
                Box::new(|b| ReceiptIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::FunctionAddress,
                "function-address",
                Box::new(|b| FunctionAddressIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::DependencyClosure,
                "dependency-closure",
                Box::new(|b| DependencyClosureIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::AlgebraicEffect,
                "algebraic-effect",
                Box::new(|b| AlgebraicEffectIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::EffectOperation,
                "effect-operation",
                Box::new(|b| EffectOperationIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::EffectLog,
                "effect-log",
                Box::new(|b| EffectLogIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::FsmSpec,
                "fsm-spec",
                Box::new(|b| FsmSpecIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::FsmUniverse,
                "fsm-universe",
                Box::new(|b| FsmUniverseIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::FsmCoverage,
                "fsm-coverage",
                Box::new(|b| FsmCoverageIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::DocumentationIr,
                "documentation-ir",
                Box::new(|b| DocumentationIrIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::DocumentationOutput,
                "documentation-output",
                Box::new(|b| DocumentationOutputIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::DocumentationFacts,
                "documentation-facts",
                Box::new(|b| DocumentationFactsIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::DialectPayload,
                "dialect-payload",
                Box::new(|b| DialectPayloadIdentity::compute(b).into_any()),
            ),
            (
                IdentityDomain::EvidenceChain,
                "evidence-chain",
                Box::new(|b| EvidenceChainIdentity::compute(b).into_any()),
            ),
        ];

        for (domain, label, construct) in cases {
            let any = construct(SOURCE_BYTES);
            assert_eq!(any.domain(), domain);
            assert_eq!(any.as_wire().domain, domain);
            assert_eq!(any.as_wire().hex.as_str().len(), BLAKE3_HEX_LENGTH);

            let wire = any.as_wire();
            assert_eq!(wire.algorithm, SUPPORTED_ALGORITHM);
            assert_eq!(wire.version, WIRE_IDENTITY_VERSION);

            let readmitted = wire.clone().admit();
            assert!(readmitted.is_ok(), "admission failed for {label}");
            assert_eq!(readmitted.unwrap(), any);

            let from_hex = WireIdentity::admit_from_parts(
                WIRE_IDENTITY_VERSION,
                SUPPORTED_ALGORITHM,
                label,
                any.as_wire().hex.as_str().to_string(),
            );
            assert!(from_hex.is_ok(), "label admission failed for {label}");
            assert_eq!(from_hex.unwrap(), any);

            let legacy = compute_identity(domain, SOURCE_BYTES);
            let typed = any.as_wire();
            assert_eq!(
                serde_json::to_string(&typed).unwrap(),
                serde_json::to_string(&legacy).unwrap(),
                "typed wire JSON must be byte-identical to the legacy record for {label}"
            );

            let expected_json = json!({
                "domain": label,
                "algorithm": "blake3",
                "hex": typed.hex.as_str(),
            });
            assert_eq!(
                serde_json::to_string(&typed).unwrap(),
                serde_json::to_string(&expected_json).unwrap(),
                "legacy-shaped JSON projection changed for {label}"
            );
        }
    }

    #[test]
    fn canonical_bytes_match_legacy_compute_for_every_domain() {
        assert_eq!(
            SourceIdentity::compute(SOURCE_BYTES).hex_str(),
            compute_identity(IdentityDomain::Source, SOURCE_BYTES)
                .hex
                .as_str()
        );
        assert_eq!(
            ReceiptIdentity::compute(RECEIPT_BYTES).hex_str(),
            compute_identity(IdentityDomain::Receipt, RECEIPT_BYTES)
                .hex
                .as_str()
        );
        assert_eq!(
            AdapterIdentity::compute(SOURCE_BYTES).hex_str(),
            compute_identity(IdentityDomain::Adapter, SOURCE_BYTES)
                .hex
                .as_str()
        );
        assert_eq!(
            TargetArtifactIdentity::compute(SOURCE_BYTES).hex_str(),
            compute_identity(IdentityDomain::TargetArtifact, SOURCE_BYTES)
                .hex
                .as_str()
        );
        assert_eq!(
            CanonicalValueIdentity::compute(SOURCE_BYTES).hex_str(),
            compute_identity(IdentityDomain::CanonicalValue, SOURCE_BYTES)
                .hex
                .as_str()
        );
        assert_eq!(
            AlgebraicEffectIdentity::compute(SOURCE_BYTES).hex_str(),
            compute_identity(IdentityDomain::AlgebraicEffect, SOURCE_BYTES)
                .hex
                .as_str()
        );
        assert_eq!(
            EffectOperationIdentity::compute(SOURCE_BYTES).hex_str(),
            compute_identity(IdentityDomain::EffectOperation, SOURCE_BYTES)
                .hex
                .as_str()
        );
    }

    #[test]
    fn borrowed_view_matches_owned_hex_and_round_trips() {
        let owned = SourceIdentity::compute(SOURCE_BYTES);
        let borrowed = owned.as_ref();
        assert_eq!(borrowed.hex_str(), owned.hex_str());
        assert_eq!(borrowed.to_owned(), owned);

        let reference = TypedIdentityRef::<Source>::try_new(owned.hex_str());
        assert!(reference.is_ok());
        assert_eq!(reference.unwrap().hex_str(), owned.hex_str());

        let reference = TypedIdentity::<Source>::from_hex(owned.hex_str().to_string());
        assert!(reference.is_ok());
        assert_eq!(reference.unwrap(), owned);
    }

    #[test]
    fn legacy_shaped_json_without_version_admits_as_current() {
        let typed = SourceIdentity::compute(SOURCE_BYTES);
        let wire = typed.as_wire();
        assert_eq!(wire.version, WIRE_IDENTITY_VERSION);

        let legacy_json = serde_json::to_string(&wire).unwrap();
        assert!(!legacy_json.contains("version"));

        let from_legacy: WireIdentity = serde_json::from_str(&legacy_json).unwrap();
        assert_eq!(from_legacy.version, WIRE_IDENTITY_VERSION);
        let admitted = from_legacy.admit();
        assert!(admitted.is_ok());
        assert_eq!(admitted.unwrap(), typed.into_any());
    }

    #[test]
    fn downcast_returns_none_for_wrong_artifact_role() {
        let receipt = ReceiptIdentity::compute(RECEIPT_BYTES).into_any();
        assert!(receipt.as_receipt().is_some());
        assert!(receipt.as_source().is_none());
        assert!(receipt.as_adapter().is_none());
        assert!(receipt.as_target_artifact().is_none());

        let source = SourceIdentity::compute(SOURCE_BYTES).into_any();
        assert!(source.as_source().is_some());
        assert!(source.as_receipt().is_none());
    }

    #[test]
    fn rejects_empty_and_wrong_length_hex() {
        assert_eq!(
            SourceIdentity::from_hex(""),
            Err(IdentityAdmissionError::EmptyHash)
        );
        assert!(matches!(
            SourceIdentity::from_hex("abc"),
            Err(IdentityAdmissionError::InvalidLength { actual: 3, .. })
        ));
    }

    #[test]
    fn rejects_uppercase_and_non_hex_spellings() {
        let computed = SourceIdentity::compute(SOURCE_BYTES);
        let lower = computed.hex_str().to_string();
        let upper = lower.to_uppercase();
        assert!(SourceIdentity::from_hex(upper.clone()).is_err());

        let mut non_hex = lower.clone();
        non_hex.truncate(BLAKE3_HEX_LENGTH - 2);
        non_hex.push_str("zz");
        assert!(matches!(
            SourceIdentity::from_hex(non_hex),
            Err(IdentityAdmissionError::InvalidHex)
        ));
    }

    #[test]
    fn rejects_unknown_domain_label() {
        assert_eq!(
            WireIdentity::admit_from_parts(
                WIRE_IDENTITY_VERSION,
                SUPPORTED_ALGORITHM,
                "widget",
                hex(SOURCE_BYTES)
            ),
            Err(IdentityAdmissionError::UnknownDomain {
                label: "widget".to_string()
            })
        );
    }

    #[test]
    fn rejects_unsupported_algorithm() {
        assert_eq!(
            WireIdentity::admit_from_parts(
                WIRE_IDENTITY_VERSION,
                "sha256",
                "source",
                SourceIdentity::compute(SOURCE_BYTES).hex_str().to_string(),
            ),
            Err(IdentityAdmissionError::UnsupportedAlgorithm {
                actual: "sha256".to_string()
            })
        );
        assert_eq!(
            WireIdentity {
                domain: IdentityDomain::Source,
                algorithm: "sha512".to_string(),
                hex: Blake3Hex::from_bytes(SOURCE_BYTES),
                version: WIRE_IDENTITY_VERSION,
            }
            .admit(),
            Err(IdentityAdmissionError::UnsupportedAlgorithm {
                actual: "sha512".to_string()
            })
        );
    }

    #[test]
    fn rejects_stale_wire_version_from_json() {
        let typed = SourceIdentity::compute(SOURCE_BYTES);
        let legacy_json = serde_json::to_string(&typed.as_wire()).unwrap();
        let stale_json = legacy_json.trim_end_matches('}').to_string() + ",\"version\":7}";
        let stale: WireIdentity = serde_json::from_str(&stale_json).unwrap();
        assert_eq!(stale.version, 7);
        assert_eq!(
            stale.admit(),
            Err(IdentityAdmissionError::StaleIdentity {
                version: 7,
                expected: WIRE_IDENTITY_VERSION
            })
        );
    }

    #[test]
    fn rejects_cross_domain_wire_records_at_typed_admission() {
        let receipt_wire = ReceiptIdentity::compute(RECEIPT_BYTES).as_wire();
        let admitted = SourceIdentity::from_wire(receipt_wire);
        assert!(matches!(
            admitted,
            Err(IdentityAdmissionError::WrongDomain {
                actual,
                expected
            }) if actual == "receipt" && expected == "source"
        ));

        let source_wire = SourceIdentity::compute(SOURCE_BYTES).as_wire();
        let accepted = SourceIdentity::from_wire(source_wire);
        assert!(accepted.is_ok());
        assert_eq!(accepted.unwrap(), SourceIdentity::compute(SOURCE_BYTES));
    }

    /// Reject a wire record whose declared domain disagrees with the target
    /// typed field when the field itself uses serde deserialization.
    #[test]
    fn rejects_conflicting_artifact_role_metadata_on_deserialize() {
        let receipt_wire = ReceiptIdentity::compute(RECEIPT_BYTES).as_wire();
        let wire_json = serde_json::to_string(&receipt_wire).unwrap();
        let deserialized: Result<SourceIdentity, _> = serde_json::from_str(&wire_json);
        assert!(deserialized.is_err());

        let source_json =
            serde_json::to_string(&SourceIdentity::compute(SOURCE_BYTES).as_wire()).unwrap();
        let accepted: SourceIdentity = serde_json::from_str(&source_json).unwrap();
        assert_eq!(accepted, SourceIdentity::compute(SOURCE_BYTES));
    }

    #[test]
    fn label_catalog_covers_every_identity_domain_variant() {
        use crate::identity::IdentityDomain;
        for domain in [
            IdentityDomain::Source,
            IdentityDomain::CanonicalValue,
            IdentityDomain::Projection,
            IdentityDomain::TargetArtifact,
            IdentityDomain::Adapter,
            IdentityDomain::Evaluator,
            IdentityDomain::Compiler,
            IdentityDomain::Receipt,
            IdentityDomain::FunctionAddress,
            IdentityDomain::DependencyClosure,
            IdentityDomain::AlgebraicEffect,
            IdentityDomain::EffectOperation,
            IdentityDomain::EffectLog,
            IdentityDomain::FsmSpec,
            IdentityDomain::FsmUniverse,
            IdentityDomain::FsmCoverage,
            IdentityDomain::DocumentationIr,
            IdentityDomain::DocumentationOutput,
            IdentityDomain::DocumentationFacts,
            IdentityDomain::DialectPayload,
            IdentityDomain::EvidenceChain,
        ] {
            let label = domain.to_string();
            assert_eq!(
                domain_from_label(&label),
                Some(domain),
                "label catalog must cover every maintained domain"
            );
        }
    }

    #[test]
    fn wire_and_legacy_records_convert_losslessly() {
        let typed = SourceIdentity::compute(SOURCE_BYTES);
        let legacy: Identity = typed.as_wire().into();
        assert_eq!(legacy.domain, IdentityDomain::Source);
        assert_eq!(legacy.algorithm, "blake3");
        assert_eq!(legacy.hex.as_str(), typed.hex_str());
        let back = WireIdentity::from_legacy(&legacy);
        let admitted = back.admit();
        assert!(admitted.is_ok());
        assert_eq!(admitted, Ok(typed.into_any()));
    }

    #[test]
    fn any_identity_round_trips_through_wire_without_semantic_promotion() {
        let original = AdapterIdentity::compute(SOURCE_BYTES).into_any();
        let wire = original.as_wire();
        let readmitted = wire.admit().unwrap();
        assert_eq!(readmitted, original);
        assert!(readmitted.as_adapter().is_some());
        assert_eq!(
            readmitted.as_adapter().unwrap().hex_str(),
            original.as_adapter().unwrap().hex_str()
        );
    }

    #[test]
    fn legacy_wire_struct_is_unchanged_and_typed_wire_matches_its_shape() {
        let legacy: Identity = compute_identity(IdentityDomain::Source, SOURCE_BYTES);
        let legacy_json = serde_json::to_string(&legacy).unwrap();
        let typed_wire = SourceIdentity::compute(SOURCE_BYTES).as_wire();
        let typed_json = serde_json::to_string(&typed_wire).unwrap();
        assert_eq!(typed_json, legacy_json);
    }

    fn hex(bytes: &[u8]) -> String {
        Blake3Hex::from_bytes(bytes).as_str().to_string()
    }
}
