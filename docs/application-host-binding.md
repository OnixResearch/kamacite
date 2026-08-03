# Application-host binding

Kamacite defines one portable `ApplicationHostBinding` artifact for one executable unit.

The artifact binds an application entrypoint to one exact host composition. It does not execute the application or grant host authority.

## Canonical fields

The version 1 artifact uses these existing identity domains:

| Role | Required identity domain |
|---|---|
| Application artifact | `target-artifact` |
| Entrypoint | `function-address` |
| Input and output schemas | `canonical-value` |
| Closed effect summary | `algebraic-effect` |
| Effect operations | `effect-operation` |
| Host profile | `adapter` |
| Adapter cohort | `adapter` |
| Resource profile | `adapter` |
| Policy | `canonical-value` |
| Compatibility evidence | `evidence-chain` |

The binding has its own `application-host-binding` identity domain. Kamacite computes that identity with BLAKE3 over canonical packed Preserves bytes.

Display names, aliases, help text, source spans, and diagnostics do not affect the binding identity. Validation receipts and Valence-friendly projections can preserve safe source spans.

## Admission rules

Admission requires all these conditions:

- The schema version is supported.
- Exactly one host profile is selected.
- The effect summary is closed and known.
- Every required effect uses an exact declared semantic operation identity.
- Every application, schema, effect, host, adapter, resource, policy, and compatibility identity matches the reviewed cohort.
- The binding includes all required non-claims.
- Metadata diagnostics do not contain unsafe host material.
- The binding does not claim authority.

A capability-free application uses a closed effect summary with an empty operation set.

## Receipts and projection

Validation emits a canonical Preserves receipt. The receipt binds input identities, the optional admitted binding identity, verdict, diagnostics, checked rules, policy identity, source spans, and non-claims.

A passing receipt can produce a Valence-friendly projection. The projection keeps application, entrypoint, schema, effect, host, adapter, resource, policy, compatibility, binding, and receipt roles separate.

A rejected receipt cannot produce a passing projection.

## CLI

Use the capability-rooted binding validator:

```sh
nix run .#kamacite-application-host-binding -- validate \
  --root . \
  --input fixtures/application-host-binding/valid-closed-effect.json
```

The command is read-only by default. Output files require `--update` and a bounded `--receipt-output` path. A projection output also requires an admitted binding.

The normal fixture manifest also loads application-host binding inputs through the existing confined root shell.

## Animus pilot

Animus is the first producer pilot. The accepted producer fixture must bind these facts:

- one non-default deterministic fake-backed headless entrypoint
- one existing session runtime authority profile
- one closed symbolic effect summary
- one exact adapter cohort
- one exact resource profile
- one policy identity
- explicit authority and runtime non-claims

The fixture records an exact Animus producer revision. It does not create a direct crate dependency.

Lattice and Molten consumption remain deferred until the first producer fixture and schema are stable.

## Design reference

Roc makes the application and platform boundary visible to users. Kamacite adapts that presentation rule as canonical data.

Kamacite does not adopt the Roc compiler, ABI, runtime, platform authority model, or package lifecycle.

## Non-claims

A valid binding proves canonical identity, required fields, one selected host, closed declared effects, and exact reviewed cohort linkage.

It does not prove application correctness, effect completeness beyond supplied evidence, authorization, host behavior, sandbox enforcement, resource enforcement, runtime success, or release eligibility.
