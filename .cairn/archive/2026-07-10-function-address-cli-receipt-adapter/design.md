## Design

The pure core already validates function-address receipt inputs. The new work adds a thin CLI shell that reads an Octet report and source envelope bytes, builds the existing `FunctionAddressReceiptInput`, runs validation, and writes receipt/projection outputs.

### Decisions

1. **Core stays pure.** Octet report parsing and source file reads live in the CLI shell; receipt validation remains in `kamacite-core`.
2. **Canonical receipt first.** The CLI reports the Kamacite canonical receipt identity and writes compatibility JSON only as a projection.
3. **Projection is explicit.** A flag emits the Valence projection so downstream tools can validate linkage without re-implementing Kamacite internals.
4. **Fail closed on source mismatch.** Source envelope bytes must hash to the declared source identity before a receipt is emitted.

### Validation shape

Positive fixtures cover an Octet function-address report with one or more Rust functions and matching source envelope bytes. Negative fixtures cover malformed JSON, unsupported producer, missing source bytes, stale source identity, invalid identity domain, missing required non-claims, and forbidden semantic claims.
