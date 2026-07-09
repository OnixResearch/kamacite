#![forbid(unsafe_code)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::unwrap_used)]

mod diagnostic;
mod fixture;
mod identity;
mod receipt;

pub use diagnostic::{Diagnostic, DiagnosticCode};
pub use fixture::{
    AdapterReceiptInput, AlgebraicEffectInput, ArtifactClass, CanonicalValueInput,
    CapabilityShellInput, EffectOperation, FixtureCase, FixtureClass, FixturePayload,
    FixtureResult, FixtureSuite, FixtureSuiteReport, HandlerDescriptorInput, HandlerReceiptInput,
    IdentityUseInput, NickelAdapterReceiptInput, ReplayProfileInput, SourceEnvelopeInput,
    StackEvidenceInput, TargetArtifactInput, UnsupportedClaimInput, ValidationVerdict,
    validate_fixture, validate_fixture_suite,
};
pub use identity::{
    BLAKE3_HEX_LENGTH, Blake3Hex, Identity, IdentityDomain, IdentityError, compute_identity,
};
pub use receipt::{ReceiptBody, ReceiptEnvelope, ReceiptError, canonical_json_bytes, receipt_json};
