# Located Value Documents

## ADDED Requirements

### Requirement: Located-value schemas are versioned

r[kamacite.located_values.schemas] Kamacite MUST define versioned Preserves schemas for structural paths, source origins, located values, diagnostics, and receipts. Each canonical artifact MUST bind its schema identity and named limit profile. A new canonical shape MUST receive a new version and MUST NOT reinterpret prior bytes.

#### Scenario: A prior source envelope has no span index

r[kamacite.located_values.schemas.absent_index]
- GIVEN a valid earlier source envelope contains no span index
- WHEN the new validator admits the envelope
- THEN the original bytes and source identity MUST retain their prior meaning
- AND the validator MUST represent the index as explicitly absent

### Requirement: Structural paths are schema-bound and bounded

r[kamacite.located_values.paths] A structural path MUST bind one schema identity and an ordered bounded segment list. Segments MUST distinguish record fields, variant arguments, sequence indexes, and dictionary key identities. Dictionary segments MUST use canonical key identity rather than display text or iteration order.

#### Scenario: A valid path locates one value

r[kamacite.located_values.paths.lookup]
- GIVEN an admitted canonical value and a schema-bound path within the selected limits
- WHEN deterministic lookup runs
- THEN it MUST return exactly one target value and its canonical identity

#### Scenario: An invalid path fails closed

r[kamacite.located_values.paths.invalid]
- GIVEN a path has the wrong schema, wrong segment kind, missing field, unknown key identity, invalid index, or excessive depth
- WHEN path admission or lookup runs
- THEN it MUST return a stable path diagnostic
- AND it MUST NOT return a partial target

### Requirement: Replacement is immutable and identity-guarded

r[kamacite.located_values.lookup_replace] A replacement plan MUST bind the root identity, target path, target identity, and replacement identity. Application MUST validate all pre-state facts before it constructs a new root. It MUST NOT mutate the admitted input value.

#### Scenario: A valid replacement preserves the lookup law

r[kamacite.located_values.lookup_replace.law]
- GIVEN a valid admitted root, target path, target identity, and replacement value
- WHEN the pure core applies the replacement plan
- THEN lookup at the same path in the new root MUST return the replacement value
- AND the original root MUST remain unchanged

#### Scenario: A stale plan fails closed

r[kamacite.located_values.lookup_replace.stale]
- GIVEN the root identity or target identity differs from the replacement plan
- WHEN replacement application runs
- THEN it MUST reject the plan before constructing a result

### Requirement: Source origin is an explicit alternative

r[kamacite.located_values.source_origins] A source origin MUST be `exact`, `derived`, or `absent`. An exact origin MUST bind one source-envelope identity, locator identity, and bounded ordered byte spans. It MUST also bind parser identity when a parser produced the spans. A derived origin MUST bind bounded parent origins and a derivation class. An absent origin MUST carry a stable reason.

#### Scenario: Exact spans stay within source bytes

r[kamacite.located_values.source_origins.exact]
- GIVEN an exact origin references one source envelope
- WHEN origin validation runs
- THEN every byte span MUST be ordered and within the preserved source bytes
- AND line and column ranges MUST remain validated projections

#### Scenario: A computed value does not gain a false exact span

r[kamacite.located_values.source_origins.derived]
- GIVEN a value results from evaluation, merge, generation, wrapping, or deserialization
- WHEN no exact one-to-one source relation is available
- THEN the origin MUST be derived or absent
- AND it MUST NOT be labeled exact

### Requirement: Located diagnostics are bounded and redacted

r[kamacite.located_values.diagnostics] A located diagnostic MUST bind a stable class, structural path when known, source origin, parser identity when used, and redaction status. Diagnostic fields and source-origin collections MUST obey named bounds. Source excerpts and secret-like values MUST remain excluded unless a separate policy admits them.

#### Scenario: A diagnostic preserves safe location facts

r[kamacite.located_values.diagnostics.safe]
- GIVEN an adapter reports a bounded path and valid source origin
- WHEN Kamacite normalizes the diagnostic
- THEN the stable diagnostic MUST preserve those location facts
- AND message redaction MUST NOT change the byte coordinates

### Requirement: Delayed location refinement uses one snapshot

r[kamacite.located_values.diagnostics.snapshot] A shell adapter MAY run a location-aware parser after fast conversion fails. Both passes MUST use the same captured source bytes and parser identity. The shell MUST NOT reopen a mutable path or repeat evaluation to refine the location.

#### Scenario: Source changes after capture

r[kamacite.located_values.diagnostics.snapshot_drift]
- GIVEN repository source changes after the adapter captures its input
- WHEN delayed location refinement runs
- THEN it MUST use the captured bytes
- AND it MUST NOT report a location from the changed file

### Requirement: Effects and evidence stay outside the pure component

r[kamacite.located_values.boundaries] Located-value validation, lookup, replacement, and origin classification MUST be pure deterministic logic over admitted in-memory values. Parsers, evaluators, filesystem access, process execution, mutation, clocks, randomness, credentials, presentation, and evidence publication MUST remain in shell adapters.

#### Scenario: A Nickel parser supplies syntax spans

r[kamacite.located_values.boundaries.parser]
- GIVEN tree-sitter Nickel supplies spans for a captured source envelope
- WHEN Kamacite records the locations
- THEN it MUST classify them as syntax evidence
- AND it MUST NOT claim Nickel evaluation semantics

#### Scenario: Valence receives an evidence projection

r[kamacite.located_values.boundaries.valence]
- GIVEN Kamacite exports an admitted source origin to Valence
- WHEN the projection is built
- THEN it MUST preserve source identity, byte ranges, roles, locator identity, optional parser identity, and non-claims
- AND Valence MUST remain the owner of evidence identity and linkage

### Requirement: Located-value evidence covers success and failure

r[kamacite.located_values.fixtures] Kamacite MUST include positive, negative, property, and golden receipt fixtures for paths, lookup, replacement, source origins, delayed refinement, adapter projections, bounds, redaction, and non-claims.

#### Scenario: The focused fixture matrix runs

r[kamacite.located_values.fixtures.matrix]
- GIVEN the located-value schemas, component, adapters, and receipts are complete
- WHEN the focused validation matrix runs
- THEN every valid fixture MUST produce the expected deterministic result
- AND every malformed, stale, excessive, leaking, or overclaiming fixture MUST fail as declared
