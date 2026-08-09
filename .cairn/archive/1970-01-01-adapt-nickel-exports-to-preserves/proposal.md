## Why

Kamacite already requires Nickel source identity, derived export identity, and optional canonical Preserves identity to remain distinct, but it has no trusted path that turns an independently admitted Nickel JSON export into real canonical Preserves bytes. Callers must currently stop at JSON, hand-author Preserves text, or reuse unrelated receipt-canonicalization helpers. Those workarounds can detach validated fields from hashed bytes and can overstate JSON-compatible projection preservation as arbitrary Nickel losslessness.

The sibling `nickel-export` project now provides versioned manifests, exact source/dependency/evaluator/output identities, opaque admitted manifest states, and artifact verification without reevaluating Nickel. Kamacite should consume that evidence at its adapter boundary and remain the owner of the subsequent JSON-value-to-Preserves mapping, canonical codec, identity, and receipt.

## What Changes

- Add a versioned `nickel-json-to-preserves-v1` adapter profile for one selected JSON-format export from an admitted `onix-nickel-export-manifest/v3` manifest.
- Use the canonical `nickel-export-core` admission and supplied-artifact verification APIs at the shell boundary instead of reimplementing Nickel export receipt identity.
- Decode the exact verified JSON output under named limits, map the admitted JSON value through a closed profile, and emit canonical packed Preserves bytes from Kamacite's real codec.
- Emit an adapter receipt that separately binds Nickel source, evaluator, manifest, JSON projection, mapping profile, canonical codec, packed Preserves output, and any optional review-text projection identities.
- Add an explicit non-mutating check mode and explicit write mode; neither mode reevaluates Nickel or treats JSON bytes as canonical IR.
- Keep Markdown and AsciiDoc outside this change. Nickel-generated prose can continue to use `nickel-export` raw output; real structured documentation rendering remains owned by the documentation pipeline.

## Impact

- **Surfaces**: `kamacite-core`, `kamacite-cli`, Preserves schemas, typed Nickel adapter policy, fixture/golden receipts, packaging, and adapter documentation.
- **Dependencies**: implementation follows `make-preserves-authority-real` and `confine-root-relative-filesystem-io`; the CLI pins the reviewed `nickel-export-core` compatibility surface while Kamacite core remains evaluator- and filesystem-neutral.
- **Compatibility**: existing Nickel JSON receipts remain derived projection evidence. The new canonical identity is additive and MUST NOT silently reinterpret legacy prefixed-JSON or hand-authored Preserves identities.
- **Testing**: positive manifest/artifact admission and canonicalization fixtures plus negative stale, malformed, non-JSON, unsupported-profile, resource-limit, detached-byte, path-escape, and overclaim fixtures.
- **Non-claims**: conversion preserves only the decoded JSON-compatible exported value under the named mapping profile; it does not prove arbitrary Nickel source preservation, Nickel semantic equivalence, evaluator soundness, authorization, or release eligibility.
