## Context

Kamacite separates source envelopes, canonical values, adapter descriptors, target artifacts, and receipts. The Shareable View adapter must use this layering without absorbing Tile product semantics.

The external Tile contract remains the authority for manifest fields and their application meaning. Kamacite owns only the declared mapping and canonical envelope identity.

## Goals

- Preserve exact source bytes and schema identity.
- Produce one canonical Preserves Shareable View envelope.
- Keep every identity domain and artifact role explicit.
- Support constrained round trips for exact published source profiles.
- Reject hidden authority and executable data.
- Produce deterministic receipts and compatibility evidence.

## Ownership

| Concern | Owner |
| --- | --- |
| Shareable View product meaning and source schema | Tile |
| Conversation projection meaning | Animus |
| Canonical envelope vocabulary and adapter receipts | Kamacite |
| State, action, and presentation meaning | Producer application and Tile |
| Transport, storage, caching, and retention | Consumer runtime |
| Current read, action, subscription, and fork authority | Consumer policy and authority adapters |
| Rendering and platform behavior | Tile frontend |

## Decision

### Artifact layering

The adapter keeps these artifacts separate:

1. exact external source manifest bytes and source envelope;
2. external source schema and parser profile;
3. Kamacite adapter descriptor;
4. canonical Shareable View envelope bytes;
5. optional compatibility projection bytes;
6. mapping or round-trip receipt;
7. optional Valence-friendly evidence projection.

No artifact identity substitutes for another layer.

### Canonical vocabulary

The first canonical root is `kamacite-shareable-view-envelope-v1`. It uses canonical packed Preserves under the current reviewed codec.

The root binds these typed fields:

- envelope schema and version;
- external contract and source artifact references;
- logical object and immutable snapshot references with named owner domains;
- application state schema and state artifact references;
- complete ordered dependency entries with role, required status, privacy class, and byte-length facts;
- action catalog, presentation profile, accessibility, and fallback references;
- snapshot, live, or fork mode descriptor;
- producer generation and predecessor facts when declared;
- sensitivity, privacy domain, redaction, and disclosure summary;
- adapter profile and explicit non-claims.

References are inert canonical data. The envelope contains no payload bytes unless a selected bounded field explicitly admits them.

### Live and fork descriptors

A live descriptor can name external object, snapshot, producer, stream, sequence, and schema facts. It cannot contain endpoint secrets, bearer tokens, transport handles, sockets, callbacks, commands, or implicit authority.

A fork descriptor can name the requested external profile and exact source snapshot. It cannot claim clone success, returned-object authority, activation, or source mutation.

### Adapter descriptors

Each adapter descriptor binds:

- exact source repository and immutable revision;
- source schema and fixture identities;
- parser or decoder identity;
- field mapping and allowed loss class;
- canonical target schema and codec;
- identity-domain mapping;
- bounds and privacy rules;
- unsupported fields and modes;
- compatibility direction;
- non-claims.

The first descriptor maps the published Tile source profile. A second descriptor maps the published Animus conversation state profile into the same canonical envelope through Tile-owned field meaning.

An adapter cannot infer missing fields, invent action semantics, execute source, resolve dependencies, contact a route, or authorize operations.

### Identity

Source identity is BLAKE3 over the exact preserved source bytes under the source-envelope domain.

Canonical target identity is BLAKE3 over canonical packed Preserves bytes under a new Kamacite domain. The domain, codec, schema, and bytes are identity material.

External logical object, snapshot, state, dependency, action, presentation, and transport references remain nominal tagged values. Kamacite validates their declared shape and role only.

### Functional core

The pure core owns:

- descriptor and profile validation;
- source-fact admission;
- field and role mapping;
- bounds and privacy checks;
- canonical target construction;
- identity calculation over supplied bytes;
- reverse mapping plans;
- round-trip comparison;
- receipt construction and validation;
- optional evidence projection.

The core performs no file, process, network, clock, random, credential, evaluator, renderer, transport, or storage effects.

### Imperative shell

The CLI shell owns source reads, bounded decoding, canonical encoding, target writes, read-back, explicit update mode, diagnostics, and receipt publication.

Mutation uses existing capability-rooted and atomic publication boundaries. Dry-run remains the default for migration or fixture updates.

### Round-trip claims

A Tile round trip proves that all declared fields in one exact supported source profile map to canonical fields and return to an equivalent supported source value.

It does not prove arbitrary source equivalence, renderer behavior, action behavior, transport delivery, authority, or application correctness.

An Animus-to-envelope mapping proves only the declared projection. It does not reconstruct an Animus ledger from the envelope.

### Authority and code exclusion

The vocabulary rejects fields or values that claim bearer authority, host capabilities, executable callbacks, shell commands, dynamic code, opaque closures, or ambient resources.

A reference to a renderer or Wasm component remains inert. Execution requires a separate admitted adapter profile and current host authority.

## Data Flow

```text
exact external source bytes
  -> source envelope and source identity
  -> exact adapter descriptor
  -> pure field and role mapping
  -> canonical Preserves Shareable View envelope
  -> canonical target identity
  -> read-back and round-trip comparison
  -> deterministic adapter receipt
```

## Error Model

The adapter distinguishes unsupported source profile, malformed source, stale schema, descriptor mismatch, unknown field, unknown role, unknown mode, identity confusion, bound excess, privacy downgrade, missing required reference, executable field, hidden authority, noncanonical target, round-trip loss, and receipt mismatch.

A schema error does not become a transport or runtime error. An unavailable dependency does not make its canonical reference malformed.

## Testing Strategy

Positive fixtures cover Tile snapshot, Tile live descriptor, Tile fork descriptor, explicit fallback, and Animus conversation projection mappings.

Negative fixtures cover source drift, unknown roles, crossed identities, duplicate dependencies, missing state, privacy downgrade, callbacks, commands, tokens, handles, code, noncanonical bytes, stale receipts, and overclaims.

Independent fixture validation compares exact source, canonical target, reverse projection, and receipt identities.

## Alternatives

### Use JSON as the canonical envelope

JSON cannot preserve the current Kamacite canonical identity and value constraints. JSON remains a compatibility projection.

### Put Shareable View semantics in Kamacite

This would move product meaning away from Tile and application owners. The design keeps Kamacite as an interchange boundary.

### Execute presentation modules during adaptation

This mixes interchange with runtime authority. The design treats all code references as inert.

## Risks

- The Tile source contract can drift before a stable published revision exists.
- Generic reference fields can hide identity-domain confusion.
- A live descriptor can be mistaken for a connection grant.
- Round-trip language can overstate semantic equivalence.

Exact source pins, nominal reference domains, authority-free descriptors, and narrow claim language address these risks.
