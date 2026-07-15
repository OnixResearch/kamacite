## Context

Kamacite's accepted Nickel boundary already separates source-envelope identity, evaluator identity, derived export identity, and optional canonical Preserves identity. Its current fixture runner can execute or validate stored Nickel JSON exports, but no implementation derives a real canonical Preserves value from those bytes. The active `make-preserves-authority-real` change is repairing the lower codec layer; this change defines the adapter above that codec.

The sibling `nickel-export` project owns deterministic evaluator requests, declared input fingerprints, exact source/dependency/evaluator/output evidence, opaque `VerifiedManifest` admission, and artifact verification. Kamacite should not copy that canonical receipt encoding or absorb evaluator ownership. It should compose the admitted export with its own value-interchange core.

## Decisions

### 1. Compose independently owned pure cores

**Choice:** `kamacite-cli` will pin the reviewed `nickel-export-core` compatibility surface, deserialize the versioned manifest wire form, obtain its opaque admitted manifest state, verify supplied artifact bytes, select one JSON export by exact destination, and normalize the admitted fields into a Kamacite-owned DTO. `kamacite-core` will accept only that in-memory DTO plus explicit mapping/codec policy.

**Rationale:** Reimplementing Nickel export manifest identity would create two semantic owners. Invoking an external verifier and trusting its prose report would weaken the type-state boundary. Keeping the dependency in the shell preserves Kamacite core's evaluator neutrality while still composing two testable pure cores.

### 2. Preserve projection bytes and value identity separately

**Choice:** The adapter will hash and retain exact JSON bytes as the Nickel export projection, then strictly decode those bytes and map the decoded value under `nickel-json-to-preserves-v1`. Canonical Preserves identity belongs to the mapped value, so insignificant JSON formatting or object-order differences may produce different projection identities but the same canonical-value identity.

**Rationale:** Exact export evidence answers which bytes Nickel produced. Canonical value evidence answers which admitted value those bytes represented. Conflating them would either erase provenance or make presentation formatting canonical.

### 3. Define the mapping and limits as typed policy

**Choice:** A human-reviewed Nickel contract/profile will define accepted manifest/receipt versions, the JSON-to-Preserves profile and schema versions, canonical codec version, output classes, and named depth, collection, string, numeric, input, and output limits. A freshness-checked runtime projection will feed typed Rust policy DTOs. The pure mapper will handle JSON alternatives explicitly and reject unsupported or ambiguous representations.

**Rationale:** Mapping and resource policy are configuration, not hidden implementation defaults. Explicit alternatives and named limits prevent silent numeric coercion, unbounded traversal, and magic-number drift.

### 4. One validated value owns canonical bytes and identity

**Choice:** The mapper will produce one closed Preserves value. Schema validation, deterministic review-text rendering, canonical packed encoding, and BLAKE3 identity will all derive from that value through the codec introduced by `make-preserves-authority-real`. The API will not accept detached caller-supplied canonical bytes beside independently validated fields.

**Rationale:** This extends the real-codec change's central invariant into the Nickel adapter and prevents substitution between validated content and hashed content.

### 5. Receipts retain the complete evidence chain

**Choice:** A versioned adapter receipt will bind the Nickel export manifest, selected receipt and declared input, source, evaluator artifact and execution plan, exact JSON output, mapping profile, canonical codec, packed Preserves output, and optional text projection. Checked laws and non-claims will state that only the decoded JSON-compatible value is preserved.

**Rationale:** Operators need to localize drift to evaluator input, export bytes, mapping policy, codec, or presentation without promoting one layer's evidence into another layer's authority.

### 6. Check is default-safe; mutation is explicit

**Choice:** The command will expose mutually exclusive `--check` and `--write` modes. Check mode recomputes and compares without mutation. Write mode will publish packed output and receipt through the root-confined filesystem gateway established by `confine-root-relative-filesystem-io`; optional text output is a separately identified projection. A partial write cannot produce a trusted success receipt.

**Rationale:** Generated canonical artifacts need deterministic freshness checks and explicit operator intent. Root-relative lexical checks alone are already known to permit symlink escapes.

### 7. Markdown and AsciiDoc remain separate concerns

**Choice:** This adapter will not add Markdown or AsciiDoc output formats. Nickel-authored string templates can use `nickel-export` raw output. Structured document rendering remains a documentation-IR responsibility and must not be hidden inside a canonical value adapter.

**Rationale:** Text templating and canonical interchange have different schemas, identities, and validation laws. Combining them would broaden this change and repeat the current documentation renderer ambiguity.

## Dependencies and sequencing

- `make-preserves-authority-real` MUST establish the bounded real codec before this adapter can claim canonical output.
- `confine-root-relative-filesystem-io` MUST establish the output gateway before write mode is trusted.
- The implementation MUST pin and record the reviewed `nickel-export-core` revision/schema cohort used for manifest admission.
- `sandbox-nickel-evaluator-authority` is not an execution prerequisite because this command does not run Nickel; any imported export retains the dependency-observation and sandbox non-claims carried by its own receipt.
- `make-documentation-commands-fail-closed` remains the owner of real Markdown/AsciiDoc rendering and is not a dependency of canonical value adaptation.

## Risks / Trade-offs

- A direct pinned `nickel-export-core` shell dependency adds cross-repository release coordination, but avoids duplicate manifest semantics and untyped verifier-report trust.
- JSON export necessarily loses Nickel source syntax and values outside the JSON-compatible export boundary; explicit non-claims are required even when conversion succeeds.
- Numeric mapping changes would alter canonical identities, so mapping profile versions and migration receipts must change together.
- Canonical packed Preserves output is not convenient for human review; optional text improves reviewability but remains a non-authoritative projection.
- Publishing output plus receipt requires fail-closed replacement behavior; implementation must not claim atomicity stronger than the selected root gateway provides.
