use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};

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
            Self::Compatibility => "compatibility",
        };
        formatter.write_str(label)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Identity {
    pub domain: IdentityDomain,
    pub algorithm: String,
    pub hex: Blake3Hex,
}

impl Identity {
    pub fn new(domain: IdentityDomain, hex: Blake3Hex) -> Self {
        Self {
            domain,
            algorithm: BLAKE3_ALGORITHM.to_string(),
            hex,
        }
    }

    pub fn validate_domain(&self, expected: IdentityDomain) -> Result<(), IdentityError> {
        if self.algorithm != BLAKE3_ALGORITHM {
            return Err(IdentityError::WrongAlgorithm {
                actual: self.algorithm.clone(),
            });
        }
        if self.domain != expected {
            return Err(IdentityError::WrongDomain {
                actual: self.domain.clone(),
                expected,
            });
        }
        Ok(())
    }
}

pub fn compute_identity(domain: IdentityDomain, bytes: &[u8]) -> Identity {
    Identity::new(domain, Blake3Hex::from_bytes(bytes))
}

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
}
