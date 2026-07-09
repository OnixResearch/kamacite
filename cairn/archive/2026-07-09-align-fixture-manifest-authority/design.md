## Context

The walking-skeleton harness consumes `fixtures/manifest.json` because Rust should not execute Nickel during core validation. The project documentation still names `fixtures/manifest.ncl` as the human-reviewed authority, but the source manifest retained only the first three seed entries after the fixture corpus expanded. The immediate risk is that reviewers and automation accidentally trust the JSON projection as the product source of truth.

This change should not introduce Nickel evaluation into the Rust core. The shell may read the Nickel source bytes and compare deterministic projection evidence, but the core boundary remains normalized DTOs and bytes only.

## Decisions

### 1. Keep the full corpus in the Nickel source manifest

**Choice:** `fixtures/manifest.ncl` carries the complete fixture list with a typed shallow contract and a declared projection identity. Payload details may remain dynamically typed until adapter-specific manifest contracts are introduced.

**Rationale:** The source file must be reviewable without treating JSON as the authoring surface. A shallow typed contract catches the stable fixture metadata now while avoiding premature typing of every payload variant.

### 2. Validate projection freshness in the CLI shell

**Choice:** The CLI reads the Nickel source manifest in the imperative shell, extracts deterministic source-owned projection evidence, computes the BLAKE3 identity of the runtime JSON projection, and rejects stale projections before trusting fixture results.

**Rationale:** This preserves the functional-core boundary. The shell already owns file reads and JSON projection loading; the core continues to receive explicit DTOs and artifact bytes.

### 3. Check fixture ID agreement as a cheap source/projection guard

**Choice:** The freshness check compares the fixture IDs declared in the Nickel source with the fixture IDs loaded from the JSON projection in order.

**Rationale:** The projection hash catches byte drift, while fixture ID comparison catches obvious corpus-shape drift and gives deterministic diagnostics that are easier to review than a hash mismatch alone.

### 4. Keep projection updates explicit

**Choice:** Normal validation remains non-mutating. Updating `fixtures/manifest.json` or the source-declared projection identity remains a reviewed file change, not an automatic validation side effect.

**Rationale:** Generated or derived evidence should not silently rewrite itself during the trust check. This follows the existing golden receipt update boundary.

## Risks / Trade-offs

- The source manifest uses a shallow dynamic payload contract for this slice; future adapter changes should introduce narrower payload contracts when those payloads become source-authoritative behavior.
- The shell freshness check proves exact projection identity and fixture ID alignment, not Nickel evaluation semantics.
- Raw-byte projection hashing means harmless JSON formatting changes still require updating the source-declared projection identity, which is intentional evidence friction.
