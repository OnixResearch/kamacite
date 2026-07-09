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
mod valence_export;

pub use diagnostic::{Diagnostic, DiagnosticCode};
pub use fixture::{
    AdapterReceiptInput, AlgebraicEffectInput, ArtifactClass, CanonicalValueInput,
    CapabilityShellInput, EffectOperation, FixtureCase, FixtureClass, FixturePayload,
    FixtureResult, FixtureSuite, FixtureSuiteReport, FsmDenialRowInput, FsmEventInput,
    FsmGuardInput, FsmReceiptInput, FsmSideEffectInput, FsmSpecInput, FsmStateInput,
    FsmTraceStepInput, FsmTransitionRowInput, HandlerDescriptorInput, HandlerReceiptInput,
    IdentityUseInput, NickelAdapterReceiptInput, ReplayProfileInput, SourceEnvelopeInput,
    StackEvidenceInput, SteelAdapterReceiptInput, SteelHostFunctionInput, SteelOutputClass,
    TargetArtifactInput, UnsupportedClaimInput, ValidationVerdict, validate_fixture,
    validate_fixture_suite,
};
pub use identity::{
    BLAKE3_HEX_LENGTH, Blake3Hex, Identity, IdentityDomain, IdentityError, compute_identity,
};
pub use receipt::{ReceiptBody, ReceiptEnvelope, ReceiptError, canonical_json_bytes, receipt_json};
pub use valence_export::{
    SourceSpan, SourceSpanSet, VALENCE_EXPORT_CANONICAL_CODEC, VALENCE_EXPORT_DEPENDENCY_PROFILE,
    VALENCE_EXPORT_PROFILE_VERSION, VALENCE_EXPORT_SCHEMA_VERSION, ValenceArtifactIdentity,
    ValenceExportConformance, ValenceReceiptExportBody, ValenceReceiptExportEnvelope,
    ValenceReceiptExportInput, ValenceReceiptExportResult, export_identity_key,
    export_valence_receipt,
};
