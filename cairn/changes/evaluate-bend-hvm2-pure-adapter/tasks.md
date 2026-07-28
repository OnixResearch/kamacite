## Phase 1: Cohort and baseline

- [ ] [serial] Record the exact Bend 0.2.38 and HVM2 2.0.22 source, package, license, dependency, host-compiler, and target identities. Add bounded README references without transferring upstream claims. r[kamacite.bend_hvm2_pilot.cohort]
- [ ] [serial] Select one finite pure workload, record the optimized Rust baseline, and define the typed Nickel workload, numeric, resource, repetition, variance, and promotion policy before Bend execution. r[kamacite.bend_hvm2_pilot.workload] r[kamacite.bend_hvm2_pilot.benchmark]

## Phase 2: Boundary and functional core

- [ ] [depends:bend-hvm2-cohort] Add Bend source-envelope, evaluator-profile, compiler-target, observation, decision, and receipt schemas with separate typed BLAKE3 identity domains and explicit non-claims. r[kamacite.bend_hvm2_pilot.boundary] r[kamacite.bend_hvm2_pilot.evidence]
- [ ] [depends:bend-hvm2-policy] Implement pure cohort, workload, annotation, `Any`, numeric-semantics, artifact-link, limit, observation, backend-equality, benchmark-decision, and receipt validation. r[kamacite.bend_hvm2_pilot.workload] r[kamacite.bend_hvm2_pilot.functional_core] r[kamacite.bend_hvm2_pilot.benchmark]
- [ ] [parallel] Add a deterministic Valence-friendly projection that preserves each layer identity, recorded observation, decision, and non-claim. r[kamacite.bend_hvm2_pilot.evidence]

## Phase 3: Toolchain and shell

- [ ] [depends:bend-hvm2-cohort] Package the exact CPU-only Bend/HVM2 cohort and pinned C compiler through Nix. Update `flake.nix`, then use Nix to generate any required `flake.lock` change. r[kamacite.bend_hvm2_pilot.cohort]
- [ ] [depends:bend-hvm2-core] Add the thin shell for `bend run-rs`, `bend gen-c`, C compilation, executable invocation, cleared environment, network denial, resource limits, bounded output, and confined writes. r[kamacite.bend_hvm2_pilot.functional_core] r[kamacite.bend_hvm2_pilot.evidence]
- [ ] [depends:bend-hvm2-shell] Implement and run the independent optimized Rust oracle plus both Bend routes. Reject every backend disagreement before a passing receipt. r[kamacite.bend_hvm2_pilot.oracle]

## Phase 4: Fixtures and decision evidence

- [ ] [parallel] Add positive fixtures for the admitted pure workload and exact three-route result equality. Add negative fixtures for cohort drift, license or attribution omission, missing annotations, explicit `Any`, unsupported numeric semantics, forbidden I/O or authority, malformed generated artifacts, compile or run failure, timeout, memory or output exhaustion, backend disagreement, unstable measurements, stale receipts, and semantic overclaims. r[kamacite.bend_hvm2_pilot.fixtures]
- [ ] [depends:bend-hvm2-fixtures] Run the declared repeated measurements and emit one `adopt`, `defer`, or `reject` decision with exact evidence and blockers. r[kamacite.bend_hvm2_pilot.benchmark] r[kamacite.bend_hvm2_pilot.promotion]
- [ ] [depends:bend-hvm2-decision] Document the result, non-claims, reproduction commands, retained evidence, and the requirement for a separate consumer-owned Cairn change. r[kamacite.bend_hvm2_pilot.promotion]

## Phase 5: Validation and closeout

- [ ] [serial] Run focused positive and negative tests, manifest freshness, adapter validation, Valence projection conformance, Clippy and Octet policy, lifecycle validation, Cairn proposal/design/tasks gates, and relevant Nix checks. r[kamacite.bend_hvm2_pilot.validation]
- [ ] [depends:bend-hvm2-validation] Validate that the accepted decision and evidence satisfy this delta, sync accepted requirements, archive the completed change, and preserve rejection or deferred blockers without broadening claims. r[kamacite.bend_hvm2_pilot.validation]
