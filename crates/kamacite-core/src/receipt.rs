use crate::diagnostic::Diagnostic;
use crate::identity::{ReceiptIdentity, WireIdentity};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub const RECEIPT_SCHEMA_VERSION: &str = "kamacite.receipt.v1";
const RECEIPT_CANONICAL_CODEC: &str = "kamacite-deterministic-json-receipt-projection-v1";
const PRETTY_JSON_INDENT_SPACES: usize = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReceiptError {
    Serialize(String),
}

impl Display for ReceiptError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialize(message) => {
                write!(formatter, "receipt serialization failed: {message}")
            }
        }
    }
}

impl std::error::Error for ReceiptError {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptBody {
    pub schema_version: String,
    pub canonical_codec: String,
    pub fixture_id: String,
    pub artifact_class: String,
    pub verdict: String,
    pub requirement_ids: Vec<String>,
    pub input_identities: BTreeMap<String, WireIdentity>,
    pub output_identities: BTreeMap<String, WireIdentity>,
    pub checked_laws: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
    pub non_claims: Vec<String>,
}

impl ReceiptBody {
    pub fn new(
        fixture_id: impl Into<String>,
        artifact_class: impl Into<String>,
        verdict: impl Into<String>,
        requirement_ids: Vec<String>,
        non_claims: Vec<String>,
    ) -> Self {
        Self {
            schema_version: RECEIPT_SCHEMA_VERSION.to_string(),
            canonical_codec: RECEIPT_CANONICAL_CODEC.to_string(),
            fixture_id: fixture_id.into(),
            artifact_class: artifact_class.into(),
            verdict: verdict.into(),
            requirement_ids,
            input_identities: BTreeMap::new(),
            output_identities: BTreeMap::new(),
            checked_laws: Vec::new(),
            diagnostics: Vec::new(),
            non_claims,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiptEnvelope {
    pub body: ReceiptBody,
    pub receipt_identity: ReceiptIdentity,
}

/// Untrusted receipt envelope as read from interchange bytes.
///
/// The receipt identity stays a wire record until admission so wrong-domain
/// or stale values produce deterministic diagnostics instead of parse
/// failures. `ReceiptEnvelope::admit` converts it into the typed core value.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WireReceiptEnvelope {
    pub body: ReceiptBody,
    pub receipt_identity: WireIdentity,
}

impl ReceiptEnvelope {
    pub fn admit(wire: WireReceiptEnvelope) -> Result<Self, crate::identity::IdentityError> {
        Ok(Self {
            body: wire.body,
            receipt_identity: ReceiptIdentity::from_wire(wire.receipt_identity)?,
        })
    }

    pub fn to_wire_envelope(&self) -> WireReceiptEnvelope {
        WireReceiptEnvelope {
            body: self.body.clone(),
            receipt_identity: self.receipt_identity.to_wire(),
        }
    }
}

impl ReceiptEnvelope {
    pub fn finalize(body: ReceiptBody) -> Result<Self, ReceiptError> {
        let body_bytes = canonical_json_bytes(&body)?;
        let receipt_identity = ReceiptIdentity::compute(&body_bytes);
        Ok(Self {
            body,
            receipt_identity,
        })
    }
}

pub fn canonical_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, ReceiptError> {
    serde_json::to_vec(value).map_err(|error| ReceiptError::Serialize(error.to_string()))
}

pub fn receipt_json(receipt: &ReceiptEnvelope) -> Result<String, ReceiptError> {
    let mut buffer = Vec::new();
    let formatter =
        serde_json::ser::PrettyFormatter::with_indent(&[b' '; PRETTY_JSON_INDENT_SPACES]);
    let mut serializer = serde_json::Serializer::with_formatter(&mut buffer, formatter);
    receipt
        .serialize(&mut serializer)
        .map_err(|error| ReceiptError::Serialize(error.to_string()))?;
    String::from_utf8(buffer).map_err(|error| ReceiptError::Serialize(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_ID: &str = "receipt.identity.separate";
    const ARTIFACT_CLASS: &str = "source-envelope";
    const VERDICT: &str = "accepted";
    const REQUIREMENT_ID: &str = "r[kamacite.lossless_ir.receipt_vocabulary.identity_domain]";
    const NON_CLAIM: &str = "not runtime authorization";

    #[test]
    fn receipt_identity_uses_separate_domain() {
        let body = ReceiptBody::new(
            FIXTURE_ID,
            ARTIFACT_CLASS,
            VERDICT,
            vec![REQUIREMENT_ID.to_string()],
            vec![NON_CLAIM.to_string()],
        );
        let finalized = ReceiptEnvelope::finalize(body);
        assert!(matches!(
            finalized,
            Ok(ReceiptEnvelope { ref receipt_identity, .. })
                if receipt_identity.domain() == crate::identity::IdentityDomain::Receipt
        ));
    }
}
