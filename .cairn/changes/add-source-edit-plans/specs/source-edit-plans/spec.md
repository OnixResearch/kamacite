# Source Edit Plans

## ADDED Requirements

### Requirement: Source edit plans bind exact pre-state

r[kamacite.source_edits.plan] A source edit plan MUST bind the source-envelope identity, source byte length, parser identity, path and origin schema identities, and named limit profile. Each patch MUST bind a half-open byte range, original-range BLAKE3, replacement bytes, replacement BLAKE3, and target descriptor.

#### Scenario: A valid plan is admitted

r[kamacite.source_edits.plan.valid]
- GIVEN a plan references one exact source envelope and bounded ordered patches
- WHEN pure plan admission runs
- THEN every range and identity MUST match the captured source facts
- AND the plan MUST receive a distinct canonical BLAKE3 identity

#### Scenario: A malformed plan fails closed

r[kamacite.source_edits.plan.invalid]
- GIVEN patches overlap, duplicate, use invalid order, overflow, exceed source bytes, or exceed named limits
- WHEN plan admission runs
- THEN it MUST return a stable plan diagnostic
- AND it MUST NOT construct output bytes

### Requirement: Patch application uses one immutable snapshot

r[kamacite.source_edits.snapshot] Patch application MUST use the same captured source bytes that established the source-envelope and range identities. It MUST validate every pre-state identity before constructing output. It MUST NOT reopen a path or substitute later bytes.

#### Scenario: Repository source changes after capture

r[kamacite.source_edits.snapshot.drift]
- GIVEN repository source changes after the plan captures its input
- WHEN patch application runs
- THEN it MUST apply only to the captured bytes
- AND it MUST reject any mismatched source or range identity

### Requirement: No-op edits preserve exact identity

r[kamacite.source_edits.no_op] A plan with no effective patch MUST return the original bytes and source identity. It MUST report an explicit no-op outcome and MUST NOT request publication.

#### Scenario: Replacement bytes equal the original range

r[kamacite.source_edits.no_op.equal]
- GIVEN every replacement identity equals its original range identity
- WHEN the plan is applied
- THEN the result MUST be byte-identical to the input
- AND the source identity MUST remain unchanged

### Requirement: Untouched bytes remain exact

r[kamacite.source_edits.preservation] Patch application MUST copy every byte outside admitted ranges without normalization. It MUST NOT change whitespace, comments, quoting, line endings, encoding, trailing bytes, or final newline state outside those ranges.

#### Scenario: One token changes inside a comment-rich file

r[kamacite.source_edits.preservation.local]
- GIVEN one admitted patch targets a valid token range
- WHEN the patch is applied
- THEN only that range MUST differ
- AND every other byte MUST equal the captured source

### Requirement: Language adapters own syntax admission

r[kamacite.source_edits.language_boundary] Nickel and Steel adapters MUST validate parser identity, source encoding, syntax-node boundaries, and target roles before constructing generic plans. Parser output MUST remain syntax evidence and MUST NOT establish evaluation semantics.

#### Scenario: A parser identity drifts

r[kamacite.source_edits.language_boundary.parser_drift]
- GIVEN a saved target span came from another parser identity
- WHEN a language adapter prepares a plan
- THEN it MUST reject the target before generic patch application

### Requirement: Publication uses the shared one-file mechanism

r[kamacite.source_edits.publication] The Kamacite shell MUST publish changed bytes through `durable-file-publication` from `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM` at revision `8e05e74e24b45f752d77145c4455385daaf6d6ab`. Cargo and Nix MUST bind that same revision. Kamacite MUST retain root authority, payload meaning, source identity, receipts, and product policy.

#### Scenario: Parent synchronization fails after rename

r[kamacite.source_edits.publication.failure]
- GIVEN publication makes the destination visible and parent synchronization then fails
- WHEN the shell classifies the outcome
- THEN it MUST preserve `CommittedDurabilityUnknown`
- AND it MUST NOT report durable success

### Requirement: Changed source creates a new envelope and receipt

r[kamacite.source_edits.receipt] An effective edit MUST create a new source envelope and edit receipt. The receipt MUST bind before and after source identities, plan identity, parser identity, patch identities, checked laws, diagnostics, publication outcome, and explicit non-claims.

#### Scenario: A receipt claims semantic equivalence

r[kamacite.source_edits.receipt.non_claim]
- GIVEN an edit receipt claims evaluator correctness or semantic equivalence
- WHEN receipt validation runs
- THEN validation MUST reject the overclaim

### Requirement: Two language pilots cover success and failure

r[kamacite.source_edits.fixtures] Kamacite MUST validate the generic patch core through one Nickel pilot and one Steel pilot. Both pilots MUST include positive, negative, property, and golden receipt fixtures.

#### Scenario: The Nickel pilot runs

r[kamacite.source_edits.fixtures.nickel]
- GIVEN Nickel fixtures cover no-op, comments, quotes, line endings, valid replacement, stale input, invalid boundaries, and parser drift
- WHEN the focused Nickel edit matrix runs
- THEN every fixture MUST produce its declared deterministic result

#### Scenario: The Steel pilot runs

r[kamacite.source_edits.fixtures.steel]
- GIVEN Steel fixtures cover no-op, comments, quotes, line endings, valid replacement, stale input, invalid boundaries, and parser drift
- WHEN the focused Steel edit matrix runs
- THEN every fixture MUST produce its declared deterministic result
