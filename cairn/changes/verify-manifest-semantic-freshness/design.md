## Context

The source check currently treats Nickel as unparsed text. The embedded projection hash proves which JSON file the text names, but not that the rest of the Nickel source exports that JSON. Fixture ID alignment catches only one narrow class of drift.

## Decisions

### 1. Compare complete checked projections

**Choice:** Obtain a typed manifest projection by running the pinned Nickel contract/export in an explicit shell, or verify a generation receipt that binds exact source identity, evaluator/profile identity, contract identity, and exact projection bytes. Compare the complete result with the runtime JSON.

**Rationale:** Every normative source field must influence freshness, not only IDs.

### 2. Keep default validation deterministic

**Choice:** Default validation may consume a checked generation receipt instead of executing Nickel, but the receipt is valid only while all bound identities match. Explicit generation/update mode runs Nickel and emits the receipt.

**Rationale:** This preserves receipt-only defaults without reducing source authority to regex extraction.

### 3. Replace `Dyn` payload trust with a closed Nickel contract

**Choice:** Define typed payload alternatives for every admitted fixture kind and reject unknown fields/kinds according to explicit compatibility policy.

**Rationale:** A top-level typed record with a dynamic payload does not validate the metadata that drives core behavior.

### 4. Parsing heuristics are removed from trust decisions

**Choice:** Comments, whitespace, duplicated-looking assignments, and quoted strings are handled by Nickel evaluation, not custom line scanning.

**Rationale:** Textual spoofing must not influence trusted projection selection.

## Risks / Trade-offs

- The generated projection workflow must avoid circularly embedding its own hash; generation receipts should own that linkage.
- Nickel version changes can produce intentional projection drift and require explicit review.
- Full structural comparison may expose existing source/JSON inconsistencies that fixture-ID checks missed.
