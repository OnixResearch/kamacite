## Why

Function-address receipt identity includes the producer name and canonicalizer label but omits `producer.tool_version`. Two reproduced reports differing only in tool version emitted the same receipt identity while their JSON bodies displayed different versions. Downstream evidence can therefore retain an old identity across a producer upgrade that may change extraction behavior.

## What Changes

- Introduce a content-addressed producer/canonicalizer descriptor that includes tool version and every normative canonicalization-profile input.
- Bind that descriptor identity into canonical function-address receipt bytes while keeping display names and path aliases non-normative metadata.
- Reject compatibility projections whose displayed normative producer fields are not covered by the canonical identity.
- Version and migrate affected receipts with positive metadata-churn tests and negative producer-version/configuration drift tests.

## Impact

- **Surfaces**: Octet handoff DTO, function-address canonical bytes, JSON/Valence projections, fixtures, and golden receipts.
- **Behavior**: producer or canonicalizer version changes create a new receipt identity; display-only renames do not.
- **Validation**: identity differential tests, CLI integration fixtures, projection validation, and Cairn gates.
