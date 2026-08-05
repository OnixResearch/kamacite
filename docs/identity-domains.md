# Identity domains as Rust types

<!-- r[impl kamacite.lossless_ir.nominal_identity] -->
<!-- r[impl kamacite.lossless_ir.nominal_identity_wire] -->
<!-- r[impl kamacite.lossless_ir.nominal_identity_migration] -->

Kamacite validates every maintained identity domain with a distinct Rust
type. Each typed identity wraps private validated BLAKE3 content behind a
sealed zero-sized marker. Wrong-domain values cannot cross internal Rust
APIs, and the compiler rejects cross-domain substitution.

The type catalog covers every domain in `IdentityDomain`:

- `SourceIdentity`, `CanonicalValueIdentity`, `ProjectionIdentity`,
  `TargetArtifactIdentity`;
- `AdapterIdentity`, `EvaluatorIdentity`, `CompilerIdentity`,
  `ReceiptIdentity`;
- `FunctionAddressIdentity`, `DependencyClosureIdentity`;
- `EffectFamilyIdentity`, `EffectOperationIdentity`, `EffectLogIdentity`;
- `FsmSpecIdentity`, `FsmUniverseIdentity`, `FsmCoverageIdentity`;
- `DocumentationIrIdentity`, `DocumentationOutputIdentity`,
  `DocumentationFactsIdentity`;
- `DialectPayloadIdentity` and `EvidenceChainIdentity`.

Each alias has a borrowed `IdentityRef` form for adapter paths that must
not clone digests.

## Wire admission

<!-- r[impl kamacite.lossless_ir.nominal_identity.validation] -->

Interchange records keep explicit `domain` and `algorithm` fields in the
versioned `WireIdentity` record. Untrusted input DTOs hold wire records so
that malformed, wrong-domain, and wrong-algorithm values still parse and
fail with deterministic diagnostics at admission.

Checked admission converts wire records into typed identities:

- `Identity::<D>::from_wire` admits one known-role record and rejects a
  wrong domain or algorithm.
- `AnyIdentity::admit` admits a heterogeneous record into the closed enum.
  An unknown domain cannot deserialize; a wrong algorithm fails closed.

Known-role core APIs accept exact aliases. Domain checks that once ran
inside core functions now happen at the admission edge, and the type proves
the domain afterward.

## Canonical authority

<!-- r[impl kamacite.lossless_ir.nominal_identity.compatibility] -->

Canonical Preserves bytes remain authoritative. The wire record keeps the
serde container name `Identity` because the canonical codec embeds the
container name as the record label. Typed identities and `AnyIdentity`
serialize through the same wire shape, so canonical bytes, JSON
projections, manifest identities, and golden receipts do not change.

## Semantic-operation ownership

The generic `EffectOperationIdentity` stores only validated identity
content for its domain. The semantic operation descriptor owns family,
behavior, continuation, replay, and dependency meaning. Generic identity
mechanics do not absorb algebraic-effect semantics.

## Consumer migration

Consumers that parse Kamacite artifacts keep the same JSON and Preserves
bytes. Consumers that call the Rust API must switch from `Identity` to
`WireIdentity` at untrusted boundaries and to the domain alias for
known-role flows. `compute_identity` still builds wire records for
admission-layer code.

## Claim boundary

<!-- r[impl kamacite.lossless_ir.nominal_identity_docs] -->
<!-- r[impl kamacite.lossless_ir.nominal_identity_final_checks] -->

Nominal identity types prove local domain separation and checked identity
syntax only. They do not prove source fidelity, adapter law satisfaction,
semantic equivalence, authority, runtime correctness, or release
eligibility. Equal digest bytes in different domains remain different
roles, and identity possession is not a capability.
