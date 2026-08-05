## Phase 1: Baseline and schema

- [x] [serial] I1 Record baseline algebraic-effect, adapter, resource, manifest, Valence projection, workspace, and lifecycle results before adding the binding artifact. r[kamacite.application_host_binding.fixtures]
- [x] [serial] I2 Define canonical Preserves schemas for `ApplicationHostBinding` and its validation receipt, including exact identity domains and required non-claims. r[kamacite.application_host_binding.artifact] r[kamacite.application_host_binding.receipt]
- [x] [depends:kamacite.application_host_binding.artifact] I3 Register the new artifact and receipt families in Kamacite manifests, schema vocabularies, conformance tables, and compatibility projections. r[kamacite.application_host_binding.artifact] r[kamacite.application_host_binding.projection]

## Phase 2: Pure validation core and shell integration

- [x] [depends:kamacite.application_host_binding.artifact] I4 Implement pure DTO admission, canonicalization, BLAKE3 identity, exactly-one-host checks, closed-effect checks, cohort checks, and deterministic diagnostics in `kamacite-core`. r[kamacite.application_host_binding.host_selection] r[kamacite.application_host_binding.effects] r[kamacite.application_host_binding.cohorts] r[kamacite.application_host_binding.boundary]
- [x] [depends:kamacite.application_host_binding.receipt] I5 Extend existing manifest validation and receipt projection to load binding artifacts through the bounded CLI shell without executing applications or hosts. r[kamacite.application_host_binding.receipt] r[kamacite.application_host_binding.boundary]
- [x] [depends:kamacite.application_host_binding.artifact] I6 Add a Valence-friendly projection that preserves application, entrypoint, host, effect, adapter, resource, binding, diagnostic, and non-claim roles. r[kamacite.application_host_binding.projection]

## Phase 3: Pilot fixtures and verification

- [ ] [depends:kamacite.application_host_binding.cohorts] I7 Add an exact frozen Animus producer fixture only after its schema, producer revision, and binding projection are available. r[kamacite.application_host_binding.projection] r[kamacite.application_host_binding.fixtures]
- [ ] [parallel] V1 Add positive fixtures for capability-free and closed-effect bindings, metadata rename stability, exact Animus projection, and Valence projection. r[kamacite.application_host_binding.fixtures]
- [x] [parallel] V2 Add negative fixtures for missing or multiple hosts, open or unknown effects, undeclared operations, stale identities, wrong cohorts, unsupported versions, unsafe diagnostics, and authority overclaims. r[kamacite.application_host_binding.fixtures] r[kamacite.application_host_binding.boundary]
- [x] [depends:kamacite.application_host_binding.fixtures] V3 Add property tests for canonical ordering, metadata exclusion, deterministic identity, and semantic single-field re-keying. r[kamacite.application_host_binding.artifact] r[kamacite.application_host_binding.fixtures]
- [x] [serial] I8 Document the binding, Roc design reference, consumer rollout boundary, and non-claims without presenting Roc as a current dependency. r[kamacite.application_host_binding.boundary]
- [ ] [depends:kamacite.application_host_binding.fixtures] V4 Run focused fixture validation, workspace tests, manifest freshness, Valence projection checks, Clippy, lifecycle validation, Cairn gates, and relevant Nix checks. r[kamacite.application_host_binding.fixtures] r[kamacite.application_host_binding.boundary]

## Current external blocker

The binding artifact, validation core, receipt family, manifest integration, Valence projection, fixture corpus, property tests, and documentation are implemented and validated. Two tasks remain open: I7 needs an exact frozen Animus producer fixture, and the Animus producer side (`animus/cairn/changes/pilot-application-host-binding`) is still an active change with no completed tasks. V1 also stays open because its literal scope includes that exact Animus projection fixture; the capability-free, closed-effect, metadata-rename, and Valence projection fixtures it names are already in place. V4 stays open because it depends on the complete fixture corpus. Closeout remains blocked with `external-animus-producer-unavailable` until Animus publishes a schema, producer revision, and binding projection.
