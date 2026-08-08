# Application-host binding artifacts

<!-- r[impl kamacite.application_host_binding.artifact] -->
<!-- r[impl kamacite.application_host_binding.host_selection] -->
<!-- r[impl kamacite.application_host_binding.effects] -->
<!-- r[impl kamacite.application_host_binding.cohorts] -->
<!-- r[impl kamacite.application_host_binding.boundary] -->

Kamacite owns a versioned canonical Preserves `ApplicationHostBinding`
artifact. It binds one application artifact and one executable entrypoint
to one input schema, one output schema, one closed effect summary, exactly
one host profile, one adapter cohort, one resource profile, optional policy
and compatibility identities, and explicit non-claims.

The artifact adapts the Roc application/platform presentation split as
canonical data. Roc compiler, ABI, runtime, and platform authority remain
outside Kamacite.

## Identity and metadata

The binding identity is BLAKE3 over canonical packed Preserves bytes in the
`application-host-binding` identity domain. Display names, aliases, paths,
source spans, and documentation stay review metadata. A metadata rename
does not change the binding identity. A change to any semantic field
produces a new identity.

## Host selection

An executable binding contains exactly one host-profile identity. A
missing, alternative, repeated, implicit, or fallback host selection fails
admission with a deterministic diagnostic.

## Effect closure

An executable binding references one closed Kamacite effect summary with
exact semantic effect-operation identities. An open summary or an operation
outside the `effect-operation` domain fails admission. The effect summary
remains symbolic requirement data. It does not grant host capabilities.

## Cohort exactness

A fixture or caller can supply expected adapter-cohort, resource-profile,
and host-profile identities. The binding must match them exactly. A stale
or role-mismatched substitution fails with a cohort diagnostic.

## Receipts and projection

A passing admission emits a deterministic receipt that binds every linked
identity, the verdict, checked laws, and non-claims. The Valence-friendly
projection keeps application, entrypoint, host, effect, adapter, resource,
binding, policy, and compatibility roles distinct. It does not claim
execution, authorization, semantic correctness, or release eligibility.

## Consumer boundary

Animus is the first admitted producer pilot. Kamacite freezes producer
revision `c09991fa299f1f74b04d066bf31382e34108fa82` in
`fixtures/application-host-binding/animus-producer.json`.

The producer fixture has BLAKE3
`f8fbfda873094923004b99ac1150eb4e402b49deaa866b0647bd84e758e8a0ef`.
The accepted Kamacite receipt identity is
`1bac624751d47bd89ecba53923eda72f9cf20cdcc31dc01186882abf8da1cbb2`.

`fixtures/application-host-binding/animus-conformance.json` binds these
identities for Animus. Its BLAKE3 is
`d5a082fb3ee92bc0fdbccdd0a08bbc9e072b3eea95615df8a17be70fbf9c540c`.

Lattice and Aspen adoption waits for a stable schema and pilot. A passing
binding proves identity and declared compatibility only. Product hosts
keep authorization, capability, sandbox, and runtime enforcement.
