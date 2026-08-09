# Function Address Receipts Specification

## Purpose

Bind normative function-address producer and canonicalizer provenance into receipt identity.

## Requirements

### Requirement: Producer profile is content-addressed

r[kamacite.findings.function_address_producer.profile_identity] Function-address receipts MUST use a content-addressed producer profile covering producer kind, tool version/build identity, canonicalizer profile/version, normative options, and producer non-claims.

#### Scenario: Producer tool version changes

- GIVEN two otherwise identical function inventories were produced by different tool versions
- WHEN Kamacite constructs producer profiles
- THEN the producer profile identities MUST differ.

### Requirement: Receipt identity binds producer profile

r[kamacite.findings.function_address_producer.receipt_binding] Canonical function-address receipt bytes MUST include the producer profile identity. Every normative producer field displayed by a compatibility projection MUST resolve to that bound profile.

#### Scenario: JSON displays an unbound version

- GIVEN a JSON receipt projection changes the displayed tool version while retaining the old producer profile and receipt identities
- WHEN projection validation runs
- THEN validation MUST fail with deterministic producer-profile drift evidence.

### Requirement: Display metadata remains non-normative

r[kamacite.findings.function_address_producer.metadata_boundary] Path aliases, display names, and labels MAY remain outside canonical identity only when source, function, dependency, producer-profile, and receipt identities are unchanged and the fields are explicitly classified as metadata.

#### Scenario: Display-only rename occurs

- GIVEN two receipts differ only in admitted display names or path aliases
- WHEN canonical receipt identities are computed
- THEN the identities MUST remain equal while both projections retain their visible metadata.

### Requirement: Legacy unbound versions are caveated

r[kamacite.findings.function_address_producer.migration] Kamacite MUST version the producer-bound receipt profile and MUST NOT claim legacy receipt identities bind tool version. Legacy inputs MAY remain recorded evidence only with an explicit tool-version-unbound caveat.

#### Scenario: Legacy v1 receipt is loaded

- GIVEN a v1 receipt omits producer profile identity from canonical hash material
- WHEN a producer-bound validator reads it
- THEN the validator MUST reject current-profile trust or preserve it only under the legacy caveat without rewriting its identity.
