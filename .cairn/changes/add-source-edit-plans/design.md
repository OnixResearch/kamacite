## Context

Kamacite source envelopes make original bytes authoritative. Parsed values and syntax spans are derived evidence.

The located-value change adds structural paths and source origins. It does not own source rewriting or publication.

`sexplib` revision `e5a4df62c104c8f7c2e8a11ce36e88808649fce4` retains comments, positions, and atom spellings in layout trees. Its renderer does not promise an exact byte round trip for all whitespace.

Kamacite therefore adapts the preservation goal through exact byte patches. It does not adapt the layout renderer.

## Goals

- Preserve every byte outside an admitted edit range.
- Reject stale, overlapping, excessive, or out-of-range edits.
- Preserve exact no-op identity.
- Separate language syntax checks from generic patch application.
- Publish one changed file through a reviewed shared component.

## Non-goals

- General formatting or source regeneration.
- Multi-file atomic transactions.
- Nickel or Steel evaluation.
- Semantic equivalence claims.
- Publication authority inside the pure core.

## Decisions

### Decision: Add one narrow pure source-edit component

**Choice:** Add `crates/kamacite-source-edit/` for plan admission, patch validation, exact application, and receipt fact construction.

The component accepts source-envelope bytes and admitted plans in memory. Shell adapters own parser calls, file reads, publication, and presentation.

**Rationale:** Byte-patch semantics form one coherent capability. They do not require host authority.

### Decision: Bind plans and patches to exact pre-state

**Choice:** A plan binds the source-envelope identity, source byte length, parser identity, path and origin schema identities, and named limit profile.

Each patch binds a half-open byte range, original-range BLAKE3, replacement bytes, replacement BLAKE3, and target descriptor.

Plans store patches in ascending range order. Admission rejects overlap, duplicates, invalid ordering, excessive counts, excessive replacement bytes, and arithmetic overflow.

**Rationale:** Exact pre-state facts prevent edits from applying to changed or substituted source.

### Decision: Preserve untouched bytes instead of rerendering trees

**Choice:** Application copies untouched source ranges exactly and inserts only admitted replacement bytes.

The generic core does not normalize whitespace, quoting, comments, line endings, encoding, or final newline state.

**Rationale:** Whole-tree rendering changes unrelated bytes. Range application makes the preservation claim testable.

### Decision: Make no-op behavior exact

**Choice:** A plan with no patches returns the original bytes and source identity. It produces an explicit no-op outcome.

A patch whose replacement equals its original range is also classified as no-op after identity comparison.

**Rationale:** No-op operations must not create false source changes or publication work.

### Decision: Keep language checks in adapters

**Choice:** Nickel and Steel adapters validate parser identity, syntax-node boundaries, text encoding, and target roles before they construct a generic plan.

Tree-sitter Nickel remains syntax evidence only. A Steel parser remains syntax evidence only.

The core does not infer token or comment semantics from bytes.

**Rationale:** Language rules differ. The generic component owns only exact patch mechanics.

### Decision: Emit a new envelope and bounded receipt

**Choice:** A changed result receives a new source-envelope identity. The receipt binds before and after identities, plan identity, parser identity, patch identities, checked laws, diagnostics, and non-claims.

A receipt does not claim evaluator correctness or semantic equivalence.

**Rationale:** Source mutation creates new content. The old envelope remains immutable evidence.

### Decision: Reuse Durable File Publication

**Choice:** The shell uses `durable-file-publication` from `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM` at revision `8e05e74e24b45f752d77145c4455385daaf6d6ab`.

Cargo and Nix must use the same immutable revision. Kamacite retains root authority, payload meaning, source identity, receipts, and product policy.

The shared component owns one-file staging, rename behavior, synchronization outcomes, and partial durability classification.

**Rationale:** Kamacite must not duplicate publication mechanics or use temporary rename as an untyped success result.

### Decision: Require two language pilots

**Choice:** One Nickel pilot and one Steel pilot use the same generic patch core.

Each pilot includes no-op, comment, quoting, line-ending, valid replacement, stale input, invalid boundary, and parser-drift fixtures.

**Rationale:** Two pilots test whether the component is language-neutral without creating a separate repository.

### Decision: Use the upstream code only as a design reference

**Choice:** The implementation copies no OCaml source text. Any copied upstream expression must retain the MIT notice.

**Rationale:** Exact patch application differs from the upstream layout renderer.

## Adversarial Audit

The audit must try these failures:

- source bytes differ from the plan identity;
- a patch range hash is stale;
- ranges overlap, duplicate, overflow, or exceed source bytes;
- replacement bytes exceed the named bound;
- a no-op changes bytes or identity;
- untouched comments, quotes, line endings, or trailing bytes change;
- a language adapter accepts the wrong parser identity;
- a syntax span is promoted to evaluation evidence;
- publication reports durability after an unknown outcome;
- a receipt claims semantic preservation.

Any successful counterexample blocks closeout.

## Risks / Trade-offs

- Range patches cannot express every formatter operation. This limit protects untouched bytes.
- Parser upgrades can change syntax boundaries. Parser identity makes that drift explicit.
- Multi-file edits remain consumer-owned. One-file publication cannot provide transaction authority.
- Two pilots add work before extraction. They provide evidence that the core is not Nickel-specific.
