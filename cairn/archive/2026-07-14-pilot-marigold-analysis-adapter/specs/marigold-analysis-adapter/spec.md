# Marigold Analysis Adapter Specification

## Purpose

Define a removable advisory adapter profile that preserves Marigold source identity and exact analyzer provenance while admitting only bounded operator-shape analysis projections and explicitly rejecting execution, authority, canonical-IR, whole-program-cost, determinism, replay, and semantic-equivalence claims.

## Requirements

### Requirement: Marigold analysis has an exact advisory profile

r[kamacite.marigold_analysis.profile] Kamacite MUST bind Marigold analysis to a versioned Kamacite profile, exact supported `marigold-grammar` tool version, finite report bounds, required checked laws, and the `advisory` evidence role.

#### Scenario: Supported analyzer report is admitted
- GIVEN a report names the supported profile and exact analyzer cohort and stays within every finite bound
- WHEN profile admission runs
- THEN Kamacite MAY validate the report as advisory projection evidence.

#### Scenario: Analyzer cohort or profile drifts
- GIVEN the profile, tool name, tool version, or report bound differs from the accepted cohort
- WHEN profile admission runs
- THEN Kamacite MUST reject the report without silently selecting another analyzer.

### Requirement: Source, evaluator, projection, and receipt identities remain separate

r[kamacite.marigold_analysis.identity] Kamacite MUST bind exact `.marigold` source bytes as a `source` identity, the exact analyzer descriptor as an `evaluator` identity, normalized report bytes as a `projection` identity, and the final evidence envelope as a `receipt` identity using BLAKE3.

#### Scenario: Identical source and analyzer produce the same report
- GIVEN identical source bytes, analyzer cohort, normalized report, laws, and non-claims
- WHEN receipt construction runs twice
- THEN every identity and receipt MUST be identical.

#### Scenario: One identity substitutes for another role
- GIVEN a source, evaluator, projection, or receipt field carries another domain or stale bytes
- WHEN identity validation runs
- THEN Kamacite MUST reject the role confusion or freshness mismatch.

### Requirement: Analysis decisions have a functional core

r[kamacite.marigold_analysis.functional_core] Owned report validation, finite-bound checks, identity construction, freshness checks, checked-law and non-claim admission, overclaim rejection, diagnostic ordering, and receipt construction MUST be pure deterministic logic independent of Marigold and host I/O dependencies.

#### Scenario: Identical normalized input is evaluated twice
- GIVEN identical already-loaded values
- WHEN the core evaluates them twice
- THEN it MUST return identical decisions, diagnostics, projection identities, and receipts without filesystem, process, async runtime, environment, network, clock, or output effects.

### Requirement: The shell analyzes one root-confined source

r[kamacite.marigold_analysis.shell] The CLI shell MUST accept one root-relative source path, reject absolute and parent-traversal paths, read bounded UTF-8 source, invoke the exact in-process analyzer, map its result into Kamacite-owned DTOs, revalidate through the pure core, and optionally write the receipt under the selected root.

#### Scenario: Valid root-relative source is analyzed
- GIVEN a valid bounded `.marigold` source under the selected root
- WHEN `analyze-marigold` runs
- THEN the shell MUST emit a passing receipt report bound to the exact source and analyzer identities.

#### Scenario: Source path escapes or syntax is malformed
- GIVEN an absolute/escaping path or malformed Marigold source
- WHEN the command runs
- THEN it MUST fail before receipt acceptance and MUST NOT read outside the root or execute the program.

### Requirement: The pilot never executes Marigold programs

r[kamacite.marigold_analysis.no_execution] The pilot MUST NOT compile or execute generated Rust, invoke Cargo, create Marigold home-cache state, enable async runtime features, or dispatch pipeline-declared filesystem, network, process, clock, random, credential, device, package, or mutation effects.

#### Scenario: Source declares an effectful pipeline
- GIVEN source contains a Marigold output or I/O operation that the parser can analyze
- WHEN the pilot runs
- THEN only source-shape analysis MAY occur and no declared effect may be executed.

### Requirement: Receipts bind bounded analysis facts

r[kamacite.marigold_analysis.receipt] A passing receipt MUST bind the exact source, evaluator, and normalized report projection identities, the supported profile/tool cohort, sorted checked laws, sorted required non-claims, and the advisory role.

#### Scenario: Valid analysis receipt is constructed
- GIVEN every profile, report, identity, law, and non-claim check passes
- WHEN receipt construction runs
- THEN Kamacite MUST emit a deterministic receipt identity over those bounded facts.

#### Scenario: Report identity or required boundary is stale
- GIVEN the declared projection identity differs or a required checked law/non-claim is absent
- WHEN receipt construction runs
- THEN Kamacite MUST return deterministic diagnostics and no accepted receipt.

### Requirement: Marigold analysis cannot promote semantic claims

r[kamacite.marigold_analysis.nonclaims] The profile MUST state that analysis does not inspect arbitrary function bodies and does not prove runtime cost, resource safety, host authority, deterministic execution, replay, canonical Kamacite IR, compiler correctness, or semantic equivalence; forbidden promotion claims MUST fail closed.

#### Scenario: Advisory report preserves all limitations
- GIVEN the exact required non-claims are present and semantic claims are empty
- WHEN validation runs
- THEN the report MAY remain advisory.

#### Scenario: Analysis is presented as cost, runtime, replay, or equivalence proof
- GIVEN a claim promotes analyzer output into any forbidden role
- WHEN claim validation runs
- THEN Kamacite MUST reject the overclaim.

### Requirement: Positive and negative evidence accompanies the pilot

r[kamacite.marigold_analysis.fixtures] The pilot MUST include positive valid-source and receipt evidence plus negative malformed-source, unsupported-cohort, stale/wrong-domain identity, malformed report, missing-law/non-claim, and semantic-overclaim cases.

#### Scenario: Analyzer or profile behavior changes
- GIVEN parser, mapping, validation, identity, or receipt behavior changes
- WHEN focused validation runs
- THEN both positive and negative cases MUST execute before acceptance.

### Requirement: Pilot validation remains bounded and reproducible

r[kamacite.marigold_analysis.validation] The change MUST run focused core and CLI tests, formatting, Clippy, positive/negative command fixtures, manifest validation, lifecycle validation, Cairn proposal/design/tasks gates, and relevant Nix checks before archive.

#### Scenario: A broad check is unavailable
- GIVEN host or infrastructure prevents a broad check
- WHEN closeout evidence is recorded
- THEN the exact blocker, affected claim, and next-best deterministic check MUST be named without marking the blocked rail passed.
