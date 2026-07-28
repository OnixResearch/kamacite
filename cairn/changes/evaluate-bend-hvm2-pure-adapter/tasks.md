## Phase 1: Cohort and baseline

- [ ] [serial] Record the exact Bend 0.2.38 and HVM2 2.0.22 source, package, license, dependency, host-compiler, and target identities. Add bounded README references without transferring upstream claims. r[kamacite.bend_hvm2_pilot.cohort]
- [ ] [serial] Fix the workload as an ordered non-empty power-of-two `u24` leaf vector reduced by balanced wrapping addition. Define its leaf limit plus typed Nickel resource, repetition, variance, and promotion policy before Bend execution. r[kamacite.bend_hvm2_pilot.workload] r[kamacite.bend_hvm2_pilot.benchmark]
- [ ] [serial] Complete or pin the Trellis `add-bounded-modular-tree-fold-primitives` change, including its exact source, algorithm profile, executable oracle symbol, verifier, proof gaps, proof-evidence BLAKE3 identity, and non-claims. r[kamacite.bend_hvm2_pilot.trellis_oracle_evidence]

## Phase 2: Boundary and functional core

- [ ] [depends:bend-hvm2-cohort] Add Bend source-envelope, evaluator-profile, Trellis proof-handoff, compiler-target, observation, decision, and receipt schemas with separate typed BLAKE3 identity domains and explicit non-claims. r[kamacite.bend_hvm2_pilot.boundary] r[kamacite.bend_hvm2_pilot.trellis_oracle_evidence] r[kamacite.bend_hvm2_pilot.evidence]
- [ ] [depends:bend-hvm2-policy] Implement pure cohort, proof-handoff, workload, annotation, `Any`, numeric-semantics, artifact-link, limit, observation, backend-equality, benchmark-decision, and receipt validation. r[kamacite.bend_hvm2_pilot.workload] r[kamacite.bend_hvm2_pilot.trellis_oracle_evidence] r[kamacite.bend_hvm2_pilot.functional_core] r[kamacite.bend_hvm2_pilot.benchmark]
- [ ] [parallel] Add a deterministic Valence-friendly projection that preserves each layer identity, Trellis proof role, recorded observation, decision, and non-claim. r[kamacite.bend_hvm2_pilot.evidence]

## Phase 3: Toolchain and shell

- [ ] [depends:bend-hvm2-cohort] Package the exact CPU-only Bend/HVM2 cohort, exact Trellis proof source, verified-logic runtime crate, verifier rail, and pinned C compiler through Nix. Update `flake.nix`, then use Nix to generate any required `flake.lock` change. r[kamacite.bend_hvm2_pilot.cohort] r[kamacite.bend_hvm2_pilot.trellis_oracle_evidence]
- [ ] [depends:trellis-proof-pin] Rerun or validate the exact Trellis proof receipt and deny verified-oracle status on identity, profile, symbol, verifier, proof-gap, non-claim, or receipt drift. r[kamacite.bend_hvm2_pilot.trellis_oracle_evidence]
- [ ] [depends:bend-hvm2-core] Add the thin shell for `bend run-rs`, `bend gen-c`, C compilation, executable invocation, cleared environment, network denial, resource limits, bounded output, and confined writes. r[kamacite.bend_hvm2_pilot.functional_core] r[kamacite.bend_hvm2_pilot.evidence]
- [ ] [depends:trellis-proof-rerun] Execute the Trellis optimized Rust oracle and both Bend routes over the same canonical leaf vector. Reject every mapping or backend disagreement before a passing receipt. r[kamacite.bend_hvm2_pilot.oracle] r[kamacite.bend_hvm2_pilot.trellis_oracle_evidence]

## Phase 4: Fixtures and decision evidence

- [ ] [parallel] Add positive fixtures for exact proof handoff, admitted balanced input, and three-route equality. Add negative fixtures for cohort or proof drift, license or attribution omission, input-mapping drift, empty or non-power-of-two input, leaf-limit or `u24` violation, missing annotations, explicit `Any`, forbidden authority, malformed artifacts, tool failure, resource exhaustion, backend disagreement, unstable measurements, stale receipts, and semantic overclaims. r[kamacite.bend_hvm2_pilot.fixtures]
- [ ] [depends:bend-hvm2-fixtures] Run the declared repeated measurements and emit one `adopt`, `defer`, or `reject` decision with exact proof, conformance, performance, and blocker evidence. r[kamacite.bend_hvm2_pilot.benchmark] r[kamacite.bend_hvm2_pilot.promotion]
- [ ] [depends:bend-hvm2-decision] Document the result, Trellis and Kamacite non-claims, reproduction commands, retained evidence, and the requirement for a separate runtime-consumer Cairn change. r[kamacite.bend_hvm2_pilot.promotion]

## Phase 5: Validation and closeout

- [ ] [serial] Run focused positive and negative tests, Trellis proof-receipt validation, manifest freshness, adapter validation, Valence projection conformance, Clippy and Octet policy, lifecycle validation, Cairn proposal/design/tasks gates, and relevant Nix checks. r[kamacite.bend_hvm2_pilot.validation]
- [ ] [depends:bend-hvm2-validation] Validate that the accepted decision and evidence satisfy this delta, sync accepted requirements, archive the completed change, and preserve rejection or deferred blockers without broadening claims. r[kamacite.bend_hvm2_pilot.validation]
