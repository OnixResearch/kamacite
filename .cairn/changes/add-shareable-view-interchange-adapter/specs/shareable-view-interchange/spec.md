# Shareable View Interchange

## ADDED Requirements

### Requirement: Kamacite owns only the Shareable View interchange boundary [r[kamacite.shareable_view_interchange.boundary]]

Kamacite MUST preserve external Shareable View source artifacts and map supported profiles into canonical Preserves envelopes. It MUST NOT own UI, application, action, presentation, transport, cache, retention, runtime, or authority semantics.

#### Scenario: Exact external profile is admitted [r[kamacite.shareable_view_interchange.boundary.external]]

- GIVEN a caller selects the Tile Shareable View adapter
- WHEN adapter activation runs
- THEN Kamacite MUST require one exact published source revision, schema profile, fixture identity, bounds set, and compatibility result
- AND a sibling path, draft branch, screenshot, renderer tree, or undocumented DTO MUST NOT satisfy activation

#### Scenario: Boundary ownership is reviewed [r[kamacite.shareable_view_interchange.boundary.ownership]]

- GIVEN the interchange vocabulary or adapter changes
- WHEN architecture validation runs
- THEN every source field, target field, identity, transformation, effect, receipt, and claim MUST have one explicit owner
- AND the pure core MUST remain free of filesystem, process, network, clock, random, credential, evaluator, renderer, transport, and storage effects

### Requirement: Canonical envelopes preserve declared Shareable View structure [r[kamacite.shareable_view_interchange.envelope]]

Kamacite MUST define a versioned canonical Preserves envelope for supported Shareable View profiles. The envelope MUST bind declared fields without adding runtime meaning.

#### Scenario: Canonical envelope is complete [r[kamacite.shareable_view_interchange.envelope.schema]]

- GIVEN a supported source value declares contract, object, snapshot, state, dependency, action, presentation, accessibility, fallback, mode, privacy, adapter, and non-claim facts
- WHEN canonical target construction runs
- THEN the envelope MUST contain the mapped bounded facts under one closed schema and reviewed codec
- AND unknown fields, roles, modes, duplicate members, missing required references, or excessive values MUST fail closed

#### Scenario: Live or fork descriptor is present

- GIVEN the source declares live or fork mode
- WHEN envelope validation runs
- THEN Kamacite MAY preserve inert object, snapshot, producer, stream, sequence, schema, and requested-profile references
- AND it MUST reject endpoint secrets, bearer tokens, transport handles, callbacks, commands, host capabilities, clone-success claims, or implicit authority

### Requirement: Interchange identities remain nominal and separate [r[kamacite.shareable_view_interchange.identity]]

Kamacite MUST keep source envelope, canonical target, external object, snapshot, state, dependency, action, presentation, transport, adapter, and receipt identities in separate declared domains.

#### Scenario: Source maps to canonical target [r[kamacite.shareable_view_interchange.identity.domains]]

- GIVEN exact preserved source bytes and one admitted descriptor
- WHEN source and target identities are derived
- THEN source identity MUST bind exact source bytes and target identity MUST bind canonical packed Preserves bytes under distinct domains
- AND equal digests, labels, paths, field positions, or debug output MUST NOT substitute one identity domain for another

#### Scenario: External reference has valid shape

- GIVEN an external reference satisfies its declared syntax and role
- WHEN Kamacite validates the envelope
- THEN Kamacite MAY preserve it as inert canonical data
- AND the reference MUST NOT grant authenticity, currentness, read access, action authority, execution, transport, retention, or deletion rights

### Requirement: Exact adapters produce bounded receipts [r[kamacite.shareable_view_interchange.adapter]]

Each Shareable View adapter MUST bind its source revision, source schema, parser, fixture identities, field mapping, loss class, target schema, codec, bounds, privacy rules, unsupported fields, compatibility direction, and non-claims.

#### Scenario: Adapter descriptor is complete [r[kamacite.shareable_view_interchange.adapter.descriptor]]

- GIVEN a Tile or Animus adapter descriptor contains every required source, target, mapping, identity, bound, privacy, compatibility, and non-claim fact
- WHEN descriptor validation runs
- THEN Kamacite MUST produce one deterministic descriptor identity
- AND missing or role-confused facts MUST fail before source transformation

#### Scenario: Pure adapter constructs a plan [r[kamacite.shareable_view_interchange.adapter.functional_core]]

- GIVEN identical admitted source facts, descriptor, profile, and canonical codec
- WHEN the core maps the source
- THEN it MUST return identical target values, reverse-map plans, compatibility results, receipts, or blockers
- AND it MUST perform no ambient or host effect

#### Scenario: CLI publishes a target [r[kamacite.shareable_view_interchange.adapter.shell]]

- GIVEN pure validation returns one admitted target and publication plan
- WHEN the CLI executes explicit update mode
- THEN it MUST write, read back, canonicalize, remeasure, and compare the target before a passing receipt
- AND default validation or dry-run mode MUST NOT mutate source, target, manifest, or receipt files

#### Scenario: Receipt preserves layer boundaries [r[kamacite.shareable_view_interchange.adapter.receipt]]

- GIVEN a mapping or round trip completes
- WHEN receipt construction and validation run
- THEN the receipt MUST bind exact source, descriptor, target, reverse projection, compatibility, privacy, result, and non-claim identities
- AND it MUST NOT claim Tile rendering, Animus transcript truth, Molten delivery, application action success, current authority, or fork correctness

### Requirement: Envelope data and references grant no authority [r[kamacite.shareable_view_interchange.non_authority]]

Kamacite MUST reject source or target data that embeds executable behavior, ambient capabilities, bearer authority, or unsupported host references. Valid inert references MUST remain non-executing data.

#### Scenario: Executable or authority-bearing value appears

- GIVEN a source or envelope contains callbacks, closures, shell commands, dynamic code, credentials, bearer tokens, sockets, paths, handles, processes, PTYs, Wayland objects, or GPU resources
- WHEN source, mapping, or target validation runs
- THEN Kamacite MUST reject the value before publication, execution, or authority use
- AND diagnostics MUST NOT echo private or reusable authority material

#### Scenario: Renderer or Wasm reference appears

- GIVEN a supported profile contains an inert presentation artifact reference
- WHEN interchange validation runs
- THEN Kamacite MAY preserve the declared reference and role
- AND this change MUST NOT load, compile, instantiate, execute, authorize, or claim correctness for that artifact

### Requirement: Interchange validation includes positive and negative fixtures [r[kamacite.shareable_view_interchange.validation]]

Kamacite MUST provide deterministic source, canonical, reverse, receipt, compatibility, privacy, and non-authority fixtures for every supported adapter profile.

#### Scenario: Baseline is recorded [r[kamacite.shareable_view_interchange.validation.baseline]]

- GIVEN implementation is about to change core or adapter behavior
- WHEN focused baseline validation runs
- THEN current source-envelope, canonical Preserves, adapter, identity, receipt, privacy, Tile-review, and lifecycle results MUST be recorded
- AND existing failures and external blockers MUST remain visible

#### Scenario: Positive corpus runs [r[kamacite.shareable_view_interchange.validation.positive]]

- GIVEN exact Tile snapshot, live, fork, fallback, and Animus conversation fixtures
- WHEN the focused adapter corpus runs
- THEN every supported declared field MUST survive its constrained mapping and selected round trip
- AND repeated inputs MUST produce identical canonical bytes, identities, compatibility results, and receipts

#### Scenario: Negative corpus runs [r[kamacite.shareable_view_interchange.validation.negative]]

- GIVEN a fixture contains source drift, unknown role, crossed identity, duplicate dependency, missing state, privacy downgrade, callback, command, token, handle, code, noncanonical bytes, stale receipt, or overclaim
- WHEN validation runs
- THEN the fixture MUST fail at its declared boundary before publication, execution, authority use, or passing receipt creation
- AND rejection MUST NOT modify accepted sources or targets

#### Scenario: Change is ready to archive [r[kamacite.shareable_view_interchange.validation.closeout]]

- GIVEN maintainers intend to accept the interchange adapter
- WHEN closeout validation runs
- THEN focused core, CLI, canonical fixture, round-trip, privacy, Octet, lifecycle, Cairn, and relevant Nix checks MUST pass
- AND accepted specifications MUST retain runtime execution, semantic equivalence, authority, transport, and production readiness as non-claims
