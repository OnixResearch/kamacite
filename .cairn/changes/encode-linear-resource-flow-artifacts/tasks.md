# Tasks: Encode linear resource-flow artifacts

## Baseline and contract audit

- [ ] [serial] Record current effect-row, semantic-operation, FSM, identity, migration, manifest, receipt, and Valence-export baselines. r[kamacite.linear_resource_flow.evidence]
- [ ] [serial] Reconcile field meanings with Trellis change `add-linear-resource-flow-primitives` without importing Trellis runtime or proof authority. r[kamacite.linear_resource_flow.profile]
- [ ] [parallel] Audit existing Kamacite artifact classes and reuse canonical identity, diagnostics, migration, and fixture mechanisms. r[kamacite.linear_resource_flow.identity]

## Schemas and core validation

- [ ] [depends:kamacite.linear_resource_flow.profile] Define typed resource kind, occurrence, row expression, closed row, substitution, transition, witness, and admission-receipt values. r[kamacite.linear_resource_flow.profile]
- [ ] [depends:kamacite.linear_resource_flow.profile] Add canonical Preserves schemas for every artifact class under the additive profile. r[kamacite.linear_resource_flow.schema]
- [ ] [depends:kamacite.linear_resource_flow.schema] Add domain-separated BLAKE3 identity framing over canonical packed bytes. r[kamacite.linear_resource_flow.identity]
- [ ] [depends:kamacite.linear_resource_flow.schema] Implement pure bounded shape, unique-occurrence, closure, witness-linkage, and stale-identity validation. r[kamacite.linear_resource_flow.validation]
- [ ] [depends:kamacite.linear_resource_flow.validation] Add admission-receipt construction with exact Trellis, Choregraph, artifact, diagnostic, decision, and non-claim identities. r[kamacite.linear_resource_flow.receipt]
- [ ] [parallel] Reject live handles, credentials, host endpoints, authority claims, execution claims, and runtime observations in semantic artifacts. r[kamacite.linear_resource_flow.boundary]

## Fixtures and migration

- [ ] [parallel] Add positive canonical text, packed, round-trip, open-row, closed-row, substitution, transition, witness, and receipt fixtures. r[kamacite.linear_resource_flow.fixtures]
- [ ] [parallel] Add negative malformed, unsupported, duplicate, unresolved, noncanonical, stale, wrong-witness, authority-confusion, and execution-claim fixtures. r[kamacite.linear_resource_flow.fixtures]
- [ ] [depends:kamacite.linear_resource_flow.identity] Add explicit adjacent-version migration hooks and reject automatic conversion from existing effect artifacts. r[kamacite.linear_resource_flow.migration]
- [ ] [depends:kamacite.linear_resource_flow.receipt] Add positive and negative Valence-friendly receipt projections without expanding claim scope. r[kamacite.linear_resource_flow.receipt]

## Consumer and documentation

- [ ] [depends:kamacite.linear_resource_flow.receipt] Publish a Choregraph consumer profile with exact schema, codec, identity-domain, fixture, and receipt identities. r[kamacite.linear_resource_flow.consumer]
- [ ] [depends:kamacite.linear_resource_flow.consumer] Add stale producer, stale consumer, partial bundle, and cross-profile mismatch cases. r[kamacite.linear_resource_flow.consumer]
- [ ] [parallel] Update the fixture manifest source, generated manifest, algebraic-effect stack roles, identity documentation, migration documentation, and README artifact inventory. r[kamacite.linear_resource_flow.evidence]

## Validation and closeout

- [ ] [serial] Run formatting, workspace tests, Clippy with warnings denied, Kamacite manifest validation, Preserves fixtures, migration tests, adapter audits, and lifecycle validation. r[kamacite.linear_resource_flow.evidence]
- [ ] [serial] Run positive and negative Choregraph consumer conformance against the published profile. r[kamacite.linear_resource_flow.consumer]
- [ ] [serial] Validate Cairn proposal, design, tasks, and repository state. Then sync accepted requirements and archive with fresh evidence. r[kamacite.linear_resource_flow.evidence]
