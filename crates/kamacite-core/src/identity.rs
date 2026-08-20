use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub const BLAKE3_HEX_LENGTH: usize = blake3::OUT_LEN * HEX_CHARS_PER_BYTE;
const HEX_CHARS_PER_BYTE: usize = 2;
const BLAKE3_ALGORITHM: &str = "blake3";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityError {
    EmptyHash,
    InvalidLength {
        actual: usize,
        expected: usize,
    },
    InvalidHex,
    WrongAlgorithm {
        actual: String,
    },
    WrongDomain {
        actual: IdentityDomain,
        expected: IdentityDomain,
    },
}

impl Display for IdentityError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyHash => formatter.write_str("identity hash is empty"),
            Self::InvalidLength { actual, expected } => {
                write!(formatter, "identity hash length {actual} != {expected}")
            }
            Self::InvalidHex => formatter.write_str("identity hash is not lowercase hex"),
            Self::WrongAlgorithm { actual } => {
                write!(formatter, "identity algorithm {actual} is not blake3")
            }
            Self::WrongDomain { actual, expected } => {
                write!(formatter, "identity domain {actual} is not {expected}")
            }
        }
    }
}

impl std::error::Error for IdentityError {}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum IdentityDomain {
    Source,
    CanonicalValue,
    Projection,
    TargetArtifact,
    Adapter,
    Evaluator,
    Compiler,
    Receipt,
    FunctionAddress,
    DependencyClosure,
    AlgebraicEffect,
    EffectOperation,
    EffectLog,
    FsmSpec,
    FsmUniverse,
    FsmCoverage,
    DocumentationIr,
    DocumentationOutput,
    DocumentationFacts,
    DialectPayload,
    EvidenceChain,
    ApplicationHostBinding,
    Compatibility,
}

impl Display for IdentityDomain {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Source => "source",
            Self::CanonicalValue => "canonical-value",
            Self::Projection => "projection",
            Self::TargetArtifact => "target-artifact",
            Self::Adapter => "adapter",
            Self::Evaluator => "evaluator",
            Self::Compiler => "compiler",
            Self::Receipt => "receipt",
            Self::FunctionAddress => "function-address",
            Self::DependencyClosure => "dependency-closure",
            Self::AlgebraicEffect => "algebraic-effect",
            Self::EffectOperation => "effect-operation",
            Self::EffectLog => "effect-log",
            Self::FsmSpec => "fsm-spec",
            Self::FsmUniverse => "fsm-universe",
            Self::FsmCoverage => "fsm-coverage",
            Self::DocumentationIr => "documentation-ir",
            Self::DocumentationOutput => "documentation-output",
            Self::DocumentationFacts => "documentation-facts",
            Self::DialectPayload => "dialect-payload",
            Self::EvidenceChain => "evidence-chain",
            Self::ApplicationHostBinding => "application-host-binding",
            Self::Compatibility => "compatibility",
        };
        formatter.write_str(label)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Blake3Hex(String);

impl<'de> Deserialize<'de> for Blake3Hex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

impl Blake3Hex {
    pub fn parse(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();
        if value.is_empty() {
            return Err(IdentityError::EmptyHash);
        }
        if value.len() != BLAKE3_HEX_LENGTH {
            return Err(IdentityError::InvalidLength {
                actual: value.len(),
                expected: BLAKE3_HEX_LENGTH,
            });
        }
        if !value.bytes().all(is_lower_hex_byte) {
            return Err(IdentityError::InvalidHex);
        }
        Ok(Self(value))
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(blake3::hash(bytes).to_hex().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_lower_hex_byte(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
}

/// Versioned wire identity record. The serde container name stays
/// `Identity` because the canonical Preserves codec embeds the struct name
/// as the record label; renaming the label would change canonical bytes.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename = "Identity")]
pub struct WireIdentity {
    pub domain: IdentityDomain,
    pub algorithm: String,
    pub hex: Blake3Hex,
}

impl WireIdentity {
    pub fn new(domain: IdentityDomain, hex: Blake3Hex) -> Self {
        Self {
            domain,
            algorithm: BLAKE3_ALGORITHM.to_string(),
            hex,
        }
    }

    pub fn validate(&self) -> Result<(), IdentityError> {
        if self.algorithm != BLAKE3_ALGORITHM {
            return Err(IdentityError::WrongAlgorithm {
                actual: self.algorithm.clone(),
            });
        }
        Ok(())
    }

    pub fn validate_domain(&self, expected: IdentityDomain) -> Result<(), IdentityError> {
        self.validate()?;
        if self.domain != expected {
            return Err(IdentityError::WrongDomain {
                actual: self.domain.clone(),
                expected,
            });
        }
        Ok(())
    }
}

pub fn compute_identity(domain: IdentityDomain, bytes: &[u8]) -> WireIdentity {
    WireIdentity::new(domain, Blake3Hex::from_bytes(bytes))
}

mod marker_seal {
    pub trait Sealed {}
}

/// Compile-time marker for one maintained identity domain.
///
/// The sealed trait keeps the domain catalog closed: only this module can
/// declare new markers, so cross-domain substitution stays a compile error.
pub trait IdentityDomainMarker: marker_seal::Sealed {
    const DOMAIN: IdentityDomain;
}

macro_rules! declare_identity_domain {
    ($marker:ident, $alias:ident, $ref_alias:ident, $variant:ident) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $marker;

        impl marker_seal::Sealed for $marker {}

        impl IdentityDomainMarker for $marker {
            const DOMAIN: IdentityDomain = IdentityDomain::$variant;
        }

        pub type $alias = Identity<$marker>;
        pub type $ref_alias<'a> = IdentityRef<'a, $marker>;
    };
}

declare_identity_domain!(SourceDomain, SourceIdentity, SourceIdentityRef, Source);
declare_identity_domain!(
    CanonicalValueDomain,
    CanonicalValueIdentity,
    CanonicalValueIdentityRef,
    CanonicalValue
);
declare_identity_domain!(
    ProjectionDomain,
    ProjectionIdentity,
    ProjectionIdentityRef,
    Projection
);
declare_identity_domain!(
    TargetArtifactDomain,
    TargetArtifactIdentity,
    TargetArtifactIdentityRef,
    TargetArtifact
);
declare_identity_domain!(AdapterDomain, AdapterIdentity, AdapterIdentityRef, Adapter);
declare_identity_domain!(
    EvaluatorDomain,
    EvaluatorIdentity,
    EvaluatorIdentityRef,
    Evaluator
);
declare_identity_domain!(
    CompilerDomain,
    CompilerIdentity,
    CompilerIdentityRef,
    Compiler
);
declare_identity_domain!(ReceiptDomain, ReceiptIdentity, ReceiptIdentityRef, Receipt);
declare_identity_domain!(
    FunctionAddressDomain,
    FunctionAddressIdentity,
    FunctionAddressIdentityRef,
    FunctionAddress
);
declare_identity_domain!(
    DependencyClosureDomain,
    DependencyClosureIdentity,
    DependencyClosureIdentityRef,
    DependencyClosure
);
declare_identity_domain!(
    AlgebraicEffectDomain,
    EffectFamilyIdentity,
    EffectFamilyIdentityRef,
    AlgebraicEffect
);
declare_identity_domain!(
    EffectOperationDomain,
    EffectOperationIdentity,
    EffectOperationIdentityRef,
    EffectOperation
);
declare_identity_domain!(
    EffectLogDomain,
    EffectLogIdentity,
    EffectLogIdentityRef,
    EffectLog
);
declare_identity_domain!(FsmSpecDomain, FsmSpecIdentity, FsmSpecIdentityRef, FsmSpec);
declare_identity_domain!(
    FsmUniverseDomain,
    FsmUniverseIdentity,
    FsmUniverseIdentityRef,
    FsmUniverse
);
declare_identity_domain!(
    FsmCoverageDomain,
    FsmCoverageIdentity,
    FsmCoverageIdentityRef,
    FsmCoverage
);
declare_identity_domain!(
    DocumentationIrDomain,
    DocumentationIrIdentity,
    DocumentationIrIdentityRef,
    DocumentationIr
);
declare_identity_domain!(
    DocumentationOutputDomain,
    DocumentationOutputIdentity,
    DocumentationOutputIdentityRef,
    DocumentationOutput
);
declare_identity_domain!(
    DocumentationFactsDomain,
    DocumentationFactsIdentity,
    DocumentationFactsIdentityRef,
    DocumentationFacts
);
declare_identity_domain!(
    DialectPayloadDomain,
    DialectPayloadIdentity,
    DialectPayloadIdentityRef,
    DialectPayload
);
declare_identity_domain!(
    EvidenceChainDomain,
    EvidenceChainIdentity,
    EvidenceChainIdentityRef,
    EvidenceChain
);
declare_identity_domain!(
    ApplicationHostBindingDomain,
    ApplicationHostBindingIdentity,
    ApplicationHostBindingIdentityRef,
    ApplicationHostBinding
);
declare_identity_domain!(
    CompatibilityDomain,
    CompatibilityIdentity,
    CompatibilityIdentityRef,
    Compatibility
);

/// Nominal typed identity over private validated BLAKE3 content.
///
/// The zero-sized marker separates domains at compile time. Canonical wire
/// bytes stay authoritative; this type only stops wrong-domain values from
/// crossing internal Rust APIs.
///
/// Compile-pass fixture: same-domain owned, borrowed, and wire flows
/// compile and round trip.
///
/// ```
/// use kamacite_core::{ReceiptIdentity, SourceIdentity};
///
/// let owned = SourceIdentity::compute(b"source bytes");
/// let borrowed = owned.as_ref();
/// assert_eq!(borrowed.to_owned(), owned);
/// let wire = owned.to_wire();
/// assert_eq!(SourceIdentity::from_wire(wire), Ok(owned.clone()));
/// let _ = ReceiptIdentity::compute(b"receipt bytes");
/// ```
///
/// Compile-fail fixture: a receipt identity cannot stand in for a source
/// identity.
///
/// ```compile_fail
/// use kamacite_core::{ReceiptIdentity, SourceIdentity};
///
/// let receipt = ReceiptIdentity::compute(b"receipt bytes");
/// let source: SourceIdentity = receipt;
/// ```
///
/// Compile-fail fixture: an adapter identity cannot stand in for a target
/// artifact identity.
///
/// ```compile_fail
/// use kamacite_core::{AdapterIdentity, TargetArtifactIdentity};
///
/// fn needs_target(identity: TargetArtifactIdentity) {}
/// needs_target(AdapterIdentity::compute(b"adapter bytes"));
/// ```
///
/// Compile-fail fixture: an evaluator identity cannot stand in for a
/// compiler identity.
///
/// ```compile_fail
/// use kamacite_core::{CompilerIdentity, EvaluatorIdentity};
///
/// fn needs_compiler(identity: CompilerIdentity) {}
/// needs_compiler(EvaluatorIdentity::compute(b"evaluator bytes"));
/// ```
///
/// Compile-fail fixture: a canonical-value identity cannot stand in for a
/// projection identity.
///
/// ```compile_fail
/// use kamacite_core::{CanonicalValueIdentity, ProjectionIdentity};
///
/// let value = CanonicalValueIdentity::compute(b"value bytes");
/// let projection: ProjectionIdentity = value;
/// ```
///
/// Compile-fail fixture: an effect-family identity cannot stand in for an
/// effect-operation identity.
///
/// ```compile_fail
/// use kamacite_core::{EffectFamilyIdentity, EffectOperationIdentity};
///
/// fn needs_operation(identity: EffectOperationIdentity) {}
/// needs_operation(EffectFamilyIdentity::compute(b"family bytes"));
/// ```
pub struct Identity<D: IdentityDomainMarker> {
    hex: Blake3Hex,
    marker: PhantomData<fn() -> D>,
}

impl<D: IdentityDomainMarker> Identity<D> {
    pub fn from_hex(hex: Blake3Hex) -> Self {
        Self {
            hex,
            marker: PhantomData,
        }
    }

    pub fn compute(bytes: &[u8]) -> Self {
        Self::from_hex(Blake3Hex::from_bytes(bytes))
    }

    pub fn hex(&self) -> &Blake3Hex {
        &self.hex
    }

    pub fn domain(&self) -> IdentityDomain {
        D::DOMAIN
    }

    pub fn as_ref(&self) -> IdentityRef<'_, D> {
        IdentityRef {
            hex: &self.hex,
            marker: PhantomData,
        }
    }

    pub fn to_wire(&self) -> WireIdentity {
        WireIdentity::new(D::DOMAIN, self.hex.clone())
    }

    pub fn from_wire(wire: WireIdentity) -> Result<Self, IdentityError> {
        wire.validate_domain(D::DOMAIN)?;
        Ok(Self::from_hex(wire.hex))
    }
}

impl<D: IdentityDomainMarker> Clone for Identity<D> {
    fn clone(&self) -> Self {
        Self::from_hex(self.hex.clone())
    }
}

impl<D: IdentityDomainMarker> std::fmt::Debug for Identity<D> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Identity<{}>({})", D::DOMAIN, self.hex.as_str())
    }
}

impl<D: IdentityDomainMarker> PartialEq for Identity<D> {
    fn eq(&self, other: &Self) -> bool {
        self.hex == other.hex
    }
}

impl<D: IdentityDomainMarker> Eq for Identity<D> {}

impl<D: IdentityDomainMarker> PartialOrd for Identity<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<D: IdentityDomainMarker> Ord for Identity<D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hex.cmp(&other.hex)
    }
}

impl<D: IdentityDomainMarker> Hash for Identity<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hex.hash(state);
    }
}

impl<D: IdentityDomainMarker> Serialize for Identity<D> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_wire().serialize(serializer)
    }
}

impl<'de, D: IdentityDomainMarker> Deserialize<'de> for Identity<D> {
    fn deserialize<De: Deserializer<'de>>(deserializer: De) -> Result<Self, De::Error> {
        Self::from_wire(WireIdentity::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// Borrowed typed identity for adapter paths that must not clone digests.
pub struct IdentityRef<'a, D: IdentityDomainMarker> {
    hex: &'a Blake3Hex,
    marker: PhantomData<fn() -> D>,
}

impl<'a, D: IdentityDomainMarker> IdentityRef<'a, D> {
    pub fn hex(&self) -> &'a Blake3Hex {
        self.hex
    }

    pub fn domain(&self) -> IdentityDomain {
        D::DOMAIN
    }

    pub fn to_owned(&self) -> Identity<D> {
        Identity::from_hex(self.hex.clone())
    }

    pub fn to_wire(&self) -> WireIdentity {
        WireIdentity::new(D::DOMAIN, self.hex.clone())
    }
}

impl<D: IdentityDomainMarker> Clone for IdentityRef<'_, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: IdentityDomainMarker> Copy for IdentityRef<'_, D> {}

impl<D: IdentityDomainMarker> std::fmt::Debug for IdentityRef<'_, D> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "IdentityRef<{}>({})",
            D::DOMAIN,
            self.hex.as_str()
        )
    }
}

impl<D: IdentityDomainMarker> PartialEq for IdentityRef<'_, D> {
    fn eq(&self, other: &Self) -> bool {
        self.hex == other.hex
    }
}

impl<D: IdentityDomainMarker> Eq for IdentityRef<'_, D> {}

macro_rules! declare_any_identity {
    ($( { $variant:ident, $alias:ident } )*) => {
/// Closed heterogeneous identity for manifests and receipts that
/// carry several domains. Admission from a wire record is checked;
/// unknown domains fail closed.
///
/// Compile-pass fixture: heterogeneous flows admit and pattern-match.
///
/// ```
/// use kamacite_core::{AnyIdentity, compute_identity};
/// use kamacite_core::IdentityDomain;
///
/// let wire = compute_identity(IdentityDomain::Receipt, b"receipt bytes");
/// let admitted = AnyIdentity::admit(wire);
/// assert!(matches!(admitted, Ok(AnyIdentity::Receipt(_))));
/// ```
///
/// Compile-fail fixture: a closed enum cannot hold an undeclared domain.
///
/// ```compile_fail
/// use kamacite_core::AnyIdentity;
///
/// let forged = AnyIdentity::Custom(String::new());
/// ```
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum AnyIdentity {
            $( $variant($alias), )*
        }

        impl AnyIdentity {
            pub fn admit(wire: WireIdentity) -> Result<Self, IdentityError> {
                match &wire.domain {
                    $( IdentityDomain::$variant => Ok(Self::$variant($alias::from_wire(wire)?)), )*
                }
            }

            pub fn domain(&self) -> IdentityDomain {
                match self {
                    $( Self::$variant(_) => IdentityDomain::$variant, )*
                }
            }

            pub fn hex(&self) -> &Blake3Hex {
                match self {
                    $( Self::$variant(identity) => identity.hex(), )*
                }
            }

            pub fn to_wire(&self) -> WireIdentity {
                match self {
                    $( Self::$variant(identity) => identity.to_wire(), )*
                }
            }
        }

        impl Serialize for AnyIdentity {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.to_wire().serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for AnyIdentity {
            fn deserialize<De: Deserializer<'de>>(deserializer: De) -> Result<Self, De::Error> {
                Self::admit(WireIdentity::deserialize(deserializer)?)
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

declare_any_identity!(
    { Source, SourceIdentity }
    { CanonicalValue, CanonicalValueIdentity }
    { Projection, ProjectionIdentity }
    { TargetArtifact, TargetArtifactIdentity }
    { Adapter, AdapterIdentity }
    { Evaluator, EvaluatorIdentity }
    { Compiler, CompilerIdentity }
    { Receipt, ReceiptIdentity }
    { FunctionAddress, FunctionAddressIdentity }
    { DependencyClosure, DependencyClosureIdentity }
    { AlgebraicEffect, EffectFamilyIdentity }
    { EffectOperation, EffectOperationIdentity }
    { EffectLog, EffectLogIdentity }
    { FsmSpec, FsmSpecIdentity }
    { FsmUniverse, FsmUniverseIdentity }
    { FsmCoverage, FsmCoverageIdentity }
    { DocumentationIr, DocumentationIrIdentity }
    { DocumentationOutput, DocumentationOutputIdentity }
    { DocumentationFacts, DocumentationFactsIdentity }
    { DialectPayload, DialectPayloadIdentity }
    { EvidenceChain, EvidenceChainIdentity }
    { ApplicationHostBinding, ApplicationHostBindingIdentity }
    { Compatibility, CompatibilityIdentity }
);

#[cfg(test)]
mod tests {
    use super::*;

    const INVALID_SHORT_HEX: &str = "abc";
    const ZERO_BYTES: &[u8] = b"";

    #[test]
    fn parses_valid_blake3_hex() {
        let identity = compute_identity(IdentityDomain::Source, ZERO_BYTES);
        assert_eq!(identity.hex.as_str().len(), BLAKE3_HEX_LENGTH);
        assert!(identity.validate_domain(IdentityDomain::Source).is_ok());
    }

    #[test]
    fn rejects_short_hex() {
        let result = Blake3Hex::parse(INVALID_SHORT_HEX);
        assert!(matches!(
            result,
            Err(IdentityError::InvalidLength {
                actual: _,
                expected: _
            })
        ));
    }

    const UPPERCASE_HEX: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    const NON_HEX: &str = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";

    fn valid_wire(domain: IdentityDomain) -> WireIdentity {
        compute_identity(domain, b"domain-test-bytes")
    }

    #[test]
    fn typed_identity_accepts_same_domain_wire() {
        let wire = valid_wire(IdentityDomain::CanonicalValue);
        let result = CanonicalValueIdentity::from_wire(wire.clone());
        assert!(result.is_ok());
        let Ok(identity) = result else {
            return;
        };
        assert_eq!(identity.domain(), IdentityDomain::CanonicalValue);
        assert_eq!(identity.to_wire(), wire);
    }

    #[test]
    fn typed_identity_rejects_cross_domain_wire() {
        let wire = valid_wire(IdentityDomain::Receipt);
        let result = SourceIdentity::from_wire(wire);
        assert!(matches!(
            result,
            Err(IdentityError::WrongDomain {
                actual: IdentityDomain::Receipt,
                expected: IdentityDomain::Source
            })
        ));
    }

    #[test]
    fn typed_identity_rejects_wrong_algorithm_wire() {
        let mut wire = valid_wire(IdentityDomain::Source);
        wire.algorithm = "sha256".to_string();
        let result = SourceIdentity::from_wire(wire);
        assert!(matches!(
            result,
            Err(IdentityError::WrongAlgorithm { actual: _ })
        ));
    }

    #[test]
    fn blake3_hex_rejects_uppercase_and_non_hex() {
        assert!(matches!(
            Blake3Hex::parse(UPPERCASE_HEX),
            Err(IdentityError::InvalidHex)
        ));
        assert!(matches!(
            Blake3Hex::parse(NON_HEX),
            Err(IdentityError::InvalidHex)
        ));
        assert!(matches!(
            Blake3Hex::parse(String::new()),
            Err(IdentityError::EmptyHash)
        ));
    }

    #[test]
    fn any_identity_admits_each_known_domain() {
        let domains = [
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
            IdentityDomain::ApplicationHostBinding,
            IdentityDomain::Compatibility,
        ];
        for domain in domains {
            let wire = valid_wire(domain.clone());
            let result = AnyIdentity::admit(wire.clone());
            assert!(result.is_ok());
            let Ok(admitted) = result else {
                return;
            };
            assert_eq!(admitted.domain(), domain);
            assert_eq!(admitted.to_wire(), wire);
        }
    }

    #[test]
    fn any_identity_rejects_conflicting_algorithm() {
        let mut wire = valid_wire(IdentityDomain::Adapter);
        wire.algorithm = "md5".to_string();
        assert!(matches!(
            AnyIdentity::admit(wire),
            Err(IdentityError::WrongAlgorithm { actual: _ })
        ));
    }

    #[test]
    fn typed_identity_wire_round_trip_preserves_bytes() {
        let wire = valid_wire(IdentityDomain::EffectOperation);
        let Ok(json) = serde_json::to_string(&wire) else {
            return;
        };
        let Ok(typed) = serde_json::from_str::<EffectOperationIdentity>(&json) else {
            return;
        };
        let Ok(rendered) = serde_json::to_string(&typed) else {
            return;
        };
        assert_eq!(json, rendered);
    }

    #[test]
    fn typed_identity_deserialization_rejects_cross_domain_json() {
        let wire = valid_wire(IdentityDomain::Receipt);
        let Ok(json) = serde_json::to_string(&wire) else {
            return;
        };
        let result: Result<SourceIdentity, _> = serde_json::from_str(&json);
        assert!(result.is_err());
    }

    #[test]
    fn any_identity_json_round_trip_preserves_bytes() {
        let wire = valid_wire(IdentityDomain::DialectPayload);
        let Ok(json) = serde_json::to_string(&wire) else {
            return;
        };
        let Ok(admitted) = serde_json::from_str::<AnyIdentity>(&json) else {
            return;
        };
        let Ok(rendered) = serde_json::to_string(&admitted) else {
            return;
        };
        assert_eq!(json, rendered);
    }

    #[test]
    fn any_identity_rejects_unknown_domain_json() {
        let json = serde_json::json!({
            "domain": "hyperdrive-core",
            "algorithm": "blake3",
            "hex": compute_identity(IdentityDomain::Source, b"bytes").hex.as_str(),
        });
        let Ok(text) = serde_json::to_string(&json) else {
            return;
        };
        let result: Result<AnyIdentity, _> = serde_json::from_str(&text);
        assert!(result.is_err());
    }

    #[test]
    fn borrowed_identity_ref_tracks_owner() {
        let identity = ReceiptIdentity::compute(b"borrowed-bytes");
        let borrowed = identity.as_ref();
        assert_eq!(borrowed.domain(), IdentityDomain::Receipt);
        assert_eq!(borrowed.hex(), identity.hex());
        assert_eq!(borrowed.to_owned(), identity);
        assert_eq!(borrowed.to_wire(), identity.to_wire());
    }
}
