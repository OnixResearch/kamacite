# Bend and HVM2 Evaluation Pilot Specification

## Purpose

Evaluate one exact CPU-only Bend and HVM2 cohort as a bounded pure adapter target before any Onix runtime adoption.

## Requirements

### Requirement: Bend and HVM remain non-canonical adapter targets

r[kamacite.bend_hvm2_pilot.boundary] Kamacite MUST preserve Bend source as a source envelope and MUST classify generated HVM, C, executable, and run outputs as target or observation artifacts. Kamacite MUST NOT treat Bend, HVM, generated C, executable bytes, or matching output as canonical Kamacite IR or semantic-equivalence proof.

#### Scenario: Pilot artifacts retain separate roles

- GIVEN admitted Bend source produces generated C, an executable, and an observed output
- WHEN Kamacite records the pilot
- THEN each layer MUST have a separate typed BLAKE3 identity, parent role, and non-claim.

#### Scenario: Generated output claims canonical authority

- GIVEN a descriptor or receipt claims that Bend, HVM, generated C, or executable bytes replace Preserves authority
- WHEN boundary validation runs
- THEN Kamacite MUST reject the claim before it can satisfy the pilot.

### Requirement: The toolchain cohort is exact and licensed

r[kamacite.bend_hvm2_pilot.cohort] The pilot MUST bind Bend 0.2.38 at revision `814453670d0e0d6777c1313c972764dba0491b7f`, HVM2 2.0.22 at revision `654276018084b8f44a22b562dd68ab18583bfb5b`, their Apache-2.0 licenses, Bend's exact HVM2 dependency, the host C compiler, target triple, and CPU command profiles. The first cohort MUST exclude CUDA, HVM3, HVM4, Kind, and unpinned tools.

#### Scenario: Exact CPU cohort is present

- GIVEN every source, package, license, dependency, compiler, target, and command identity matches the reviewed cohort
- WHEN cohort admission runs
- THEN Kamacite MAY admit the tools for evaluation-only execution.

#### Scenario: A tool or license fact drifts

- GIVEN a source revision, package version, dependency, compiler, target, command profile, license, or attribution fact differs
- WHEN cohort admission runs
- THEN Kamacite MUST deny execution and require a new reviewed cohort.

### Requirement: The workload is pure and numerically explicit

r[kamacite.bend_hvm2_pilot.workload] The first pilot MUST reduce one ordered non-empty power-of-two leaf vector through a finite balanced binary tree using declared `u24` wrapping addition. The vector MUST fit a caller-declared leaf limit. The Bend source MUST include public type annotations, MUST reject explicit `Any`, and MUST receive no I/O or host authority.

#### Scenario: Pure balanced workload is admitted

- GIVEN the source, ordered leaves, power-of-two length, leaf limit, annotations, and `u24` policy match the reviewed profile
- WHEN workload admission runs
- THEN Kamacite MAY construct an evaluation plan without granting host capabilities.

#### Scenario: Source or input violates the profile

- GIVEN the source omits a required annotation, uses explicit `Any`, changes numeric semantics, requests I/O or host authority, or receives empty, non-power-of-two, over-limit, or out-of-domain input
- WHEN workload admission runs
- THEN Kamacite MUST deny before Bend execution.

### Requirement: An independent Rust oracle participates

r[kamacite.bend_hvm2_pilot.oracle] The pilot MUST execute the exact Trellis optimized iterative `u24` sum oracle over the same canonical leaf vector used by both Bend routes. The oracle MUST NOT call Bend or HVM2. A passing result MUST require canonical equality between the Trellis oracle, `bend run-rs`, and the executable produced through `bend gen-c` and the pinned C compiler.

#### Scenario: All three routes agree

- GIVEN the Trellis oracle and both Bend routes receive the same admitted canonical leaf vector and numeric policy
- WHEN result comparison runs
- THEN Kamacite MAY continue only if all canonical outputs are equal.

#### Scenario: Bend routes agree but the oracle differs

- GIVEN `bend run-rs` and generated C return the same value while the Trellis oracle returns a different value
- WHEN result comparison runs
- THEN Kamacite MUST deny a passing receipt and record the disagreement.

### Requirement: Trellis oracle evidence is exact and locally validated

r[kamacite.bend_hvm2_pilot.trellis_oracle_evidence] The pilot MUST bind the exact Trellis source revision, algorithm profile, executable oracle symbol, requirement and proof set, verifier, proof-gap references, BLAKE3 proof-evidence identity, claims, and non-claims. Kamacite MUST validate or rerun the required proof receipt through its pinned Nix environment before assigning a verified-oracle role.

#### Scenario: Exact proof handoff passes

- GIVEN every Trellis source, profile, symbol, proof, verifier, proof-gap, identity, non-claim, and downstream receipt fact matches
- WHEN proof-handoff admission runs
- THEN Kamacite MAY mark the Rust path as a verified Trellis-local oracle.

#### Scenario: Proof handoff drifts or overclaims

- GIVEN a required Trellis fact is missing or different, the proof rerun fails, or metadata promotes the local theorem to Bend or HVM correctness
- WHEN proof-handoff admission runs
- THEN Kamacite MUST deny verified-oracle status and MUST deny an `adopt` decision.

### Requirement: Pilot decisions use a functional core

r[kamacite.bend_hvm2_pilot.functional_core] Cohort, Trellis proof-handoff, profile, source metadata, annotation policy, numeric semantics, artifact links, resource facts, observations, backend equality, benchmark policy, decision, receipt, and Valence projection validation MUST be pure deterministic logic over explicit inputs.

#### Scenario: Equal facts produce equal decisions

- GIVEN identical admitted facts and observations
- WHEN the core evaluates the pilot
- THEN it MUST return the same ordered diagnostics or decision without filesystem, process, environment, clock, network, compiler, or output effects.

#### Scenario: Execution fails before observations are complete

- GIVEN the shell reports a compiler failure, process failure, timeout, malformed output, or missing observation
- WHEN the core evaluates the supplied facts
- THEN it MUST return a deterministic denied or deferred result without inventing output or measurements.

### Requirement: Pilot evidence binds execution and non-claims

r[kamacite.bend_hvm2_pilot.evidence] Each run and decision receipt MUST bind the exact cohort, Trellis proof artifact and downstream receipt, source envelope, workload and numeric policy, canonical leaf input, command profile, generated artifacts, compiler and target, executable, bounded stdout and stderr, exit class, canonical output, resource observations, interaction counts when available, parent identities, checked laws, and explicit non-claims.

#### Scenario: Complete run evidence is recorded

- GIVEN one route finishes under the admitted profile with complete bounded observations
- WHEN its run receipt is constructed
- THEN the receipt MUST bind each required identity and MUST label timing and interaction counts as cohort-bound observations.

#### Scenario: Receipt swaps an artifact or broadens a claim

- GIVEN a receipt changes a source, tool, policy, input, generated artifact, executable, output, observation, parent, or non-claim
- WHEN receipt validation runs
- THEN Kamacite MUST reject the receipt.

### Requirement: Benchmark admission is declared before execution

r[kamacite.bend_hvm2_pilot.benchmark] A typed Nickel policy MUST declare the workload, host profile, repetition policy, warm-up handling, summary statistic, variance limit, resource limits, and performance target against the verified optimized Rust oracle before measured Bend execution. Benchmark admission MUST treat wall time, memory, and interaction counts as observations for the exact cohort only.

#### Scenario: Stable measurements meet the policy

- GIVEN all routes produce equal outputs and the repeated observations satisfy the declared variance, resource, and performance rules
- WHEN benchmark admission runs
- THEN the result MAY satisfy the performance part of an `adopt` decision.

#### Scenario: Results are noisy or slower than the target

- GIVEN output equality holds but the observations exceed variance or resource limits or miss the declared performance target
- WHEN benchmark admission runs
- THEN Kamacite MUST return `defer` or `reject` with the exact failed criterion.

### Requirement: The pilot has positive and negative fixtures

r[kamacite.bend_hvm2_pilot.fixtures] The pilot MUST include positive fixtures for exact Trellis proof handoff, admitted workload, canonical input mapping, and three-route equality. It MUST include negative fixtures for cohort or proof drift, failed proof rerun, license or attribution omission, input-mapping drift, empty or non-power-of-two input, leaf-limit or `u24` violation, annotation and `Any` violations, forbidden authority, malformed artifacts, tool failures, resource limits, output disagreement, unstable measurements, stale receipts, and semantic overclaims.

#### Scenario: Positive fixture completes

- GIVEN the exact cohort, admitted workload, bounded input, complete observations, and equal outputs
- WHEN the pilot fixture suite runs
- THEN the positive fixture MUST reach its declared decision state.

#### Scenario: Negative fixture reaches the wrong boundary

- GIVEN a negative fixture declares one expected pre-execution, execution, post-output, benchmark, or receipt failure
- WHEN the fixture suite runs
- THEN the suite MUST fail if the case passes or fails at a different boundary.

### Requirement: Promotion is explicit and consumer-owned

r[kamacite.bend_hvm2_pilot.promotion] The completed pilot MUST emit exactly one `adopt`, `defer`, or `reject` decision. Only `adopt` MAY support a later consumer proposal. Every consumer MUST create a separate Cairn change and MUST retain Kamacite's identities, evidence limits, and non-claims.

#### Scenario: Pilot reaches adopt

- GIVEN Trellis proof-handoff, equality, fixture, resource, stability, performance, lifecycle, and evidence gates pass
- WHEN the final decision is constructed
- THEN Kamacite MAY emit `adopt` as evaluation evidence without changing a consumer runtime.

#### Scenario: Pilot cannot satisfy a gate

- GIVEN one required gate fails or lacks evidence
- WHEN the final decision is constructed
- THEN Kamacite MUST emit `defer` or `reject` with an exact blocker and MUST NOT create runtime adoption facts.

### Requirement: Pilot validation is reviewable

r[kamacite.bend_hvm2_pilot.validation] Changes to pilot schemas, profiles, Trellis proof handoff, source admission, pure decisions, shell execution, observations, receipts, fixtures, projections, packaging, or policy MUST run focused positive and negative tests, the pinned Trellis proof validation or rerun, manifest freshness, adapter validation, Valence projection conformance, Clippy and Octet policy, lifecycle validation, Cairn gates, and relevant Nix checks.

#### Scenario: Pilot evidence is assembled

- GIVEN implementation or evidence changes
- WHEN validation runs
- THEN the evidence MUST identify the exact cohort, commands, policy, fixtures, results, decision, blockers, and non-claims.

#### Scenario: Validation omits a required rail

- GIVEN one required test, freshness, conformance, lifecycle, policy, or Nix result is absent or failing
- WHEN closeout runs
- THEN the change MUST remain active and MUST NOT present the pilot decision as accepted evidence.
