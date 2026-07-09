## Context

Kamacite already validates source envelopes, canonical Preserves values, adapter receipts, target/provenance artifacts, algebraic-effect artifacts, stack role fixtures, and an executable Nickel adapter suite. The accepted stack integration boundary says Valence may wrap Kamacite receipts as evidence, but the repository does not yet define the concrete export profile, deterministic identity, command behavior, or fail-closed fixture coverage for that wrapping step.

The change should remain narrower than a direct Valence integration. Kamacite owns source preservation, canonical value identity, adapter classification, checked laws, and non-claims. Valence owns downstream evidence/linkage semantics. This slice only produces a reviewed, deterministic, Valence-friendly projection of Kamacite receipt facts.

## Decisions

### 1. Treat Valence export as a compatibility evidence projection

**Choice:** The Valence export profile is a derived projection from a Kamacite receipt. It must bind the source Kamacite receipt identity and produce its own projection/export identity, but it must not replace the original receipt or become canonical Kamacite IR.

**Rationale:** This keeps the identity model honest. Consumers can check which Kamacite receipt was exported and which projection bytes were emitted without mistaking the export for a stronger semantic claim.

### 2. Keep the mapper in the pure core

**Choice:** `kamacite-core` should expose a pure mapping function from validated receipt DTOs to Valence export DTOs. The function receives explicit in-memory data and returns deterministic export data or domain diagnostics.

**Rationale:** The export semantics are claim-preservation logic, not filesystem or process orchestration. Keeping them in the core makes the non-claim and identity behavior unit-testable without standing up Valence, Nickel, or a repository tree.

### 3. Keep file IO in a thin CLI shell

**Choice:** `kamacite-cli` should provide an explicit export command or mode that reads reviewed receipt inputs, invokes the pure mapper, compares or writes outputs only when requested, and renders deterministic reports. Default export validation should be non-mutating.

**Rationale:** The shell already owns fixture IO and golden comparison. Adding another explicit command avoids silently turning default validation into a downstream export workflow.

### 4. Preserve non-claims and checked laws exactly

**Choice:** Every export must carry forward Kamacite non-claims, checked laws, diagnostic summaries, artifact roles, adapter identities, and conformance identities. If an export input drops, rewrites, or contradicts those fields, validation fails closed.

**Rationale:** The most important Valence boundary is that wrapping evidence must not broaden Kamacite's claim. Losing non-claims is a correctness bug even if the hash fields remain valid.

### 5. Represent source spans as explicit optional evidence

**Choice:** If a Kamacite receipt has source span metadata, the export preserves it. If a receipt has no source span metadata, the export records absence explicitly and must not fabricate spans from paths, labels, or diagnostics.

**Rationale:** Valence can use spans when present, but path- or label-derived spans would create false precision and weaken receipt determinism.

### 6. Require positive and negative export fixtures

**Choice:** The implementation must add positive fixtures for ordinary receipt export and negative fixtures for dropped non-claims, identity-domain substitution, malformed receipt input, stale export identity, missing conformance entry, and semantic-equivalence overclaims.

**Rationale:** A projection is only trustworthy if fail-closed cases are exercised. The negative cases name the ways an evidence wrapper could accidentally broaden Kamacite claims.

### 7. Defer direct crate coupling to Valence

**Choice:** This change should not add a direct Valence crate dependency unless implementation proves the profile cannot be tested otherwise. Any future direct dependency needs its own accepted Cairn change after the export profile stabilizes.

**Rationale:** Initial coupling by receipts and fixtures avoids turning Valence API churn into Kamacite core semantics and keeps the stack ownership boundary explicit.

## Risks / Trade-offs

- A Valence-friendly DTO may later need a schema migration when Valence's accepted Evidence IR shape changes. Binding export-profile identity and version keeps that migration reviewable.
- Strict non-claim preservation can make seemingly harmless presentation changes fail validation. That friction is intentional because downstream evidence must not erase claim boundaries.
- Deferring direct Valence crates means the first slice proves compatibility shape, not end-to-end Valence ingestion. A later change can add ingestion once both sides agree on a stable API.
