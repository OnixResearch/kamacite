## Why

Kamacite already owns canonical application-adjacent artifacts, adapter descriptors, effect rows, resource references, target artifacts, and receipts. It does not own one portable artifact that binds an executable application entrypoint to one selected host profile.

Consumers can currently combine these identities in product-local records. That leaves host selection, effect closure, cohort drift, and claim boundaries harder to inspect across Animus, Lattice, and Aspen.

Roc makes the application/platform split a primary user concept. Kamacite can adapt that boundary as canonical data without adopting the Roc compiler, ABI, runtime, or platform authority model.

## What Changes

- Define a versioned canonical Preserves `ApplicationHostBinding` artifact.
- Bind one application artifact and entrypoint to input/output schemas, one closed effect summary, exactly one host profile, one adapter cohort, one resource profile, policy identity, and explicit non-claims.
- Reject missing or multiple host profiles, open or unknown executable effect rows, stale identities, incompatible cohorts, undeclared effects, and authority overclaims.
- Keep symbolic effect requirements separate from host capability, authorization, sandbox, and current runtime enforcement evidence.
- Add deterministic validation receipts and a Valence-friendly projection without making Valence or a runtime a Kamacite dependency.
- Add a frozen Animus pilot fixture only after an exact upstream producer cohort is available.

## Impact

- **Schemas**: a new application-host binding artifact and receipt family under Kamacite-owned Preserves schemas.
- **Core**: pure binding admission, canonicalization, identity, compatibility, and diagnostic logic.
- **Shell**: bounded manifest loading and receipt projection through existing Kamacite validation commands.
- **Fixtures**: positive, negative, migration, Animus producer, and Valence projection cases.
- **Consumers**: Animus is the first pilot. Lattice and Aspen remain deferred until the first schema and pilot are stable.

## Out of Scope

- Adding Roc as a compiler or runtime dependency.
- Executing applications, selecting hosts dynamically, granting authority, or enforcing resource limits.
- Replacing product-owned runtime, policy, capability, sandbox, or lifecycle receipts.
- Requiring one host profile for an entire multi-runtime workflow. The invariant applies to one executable unit.
