# Valence receipt export profile

Kamacite Valence receipt export is a derived evidence projection from a validated Kamacite receipt. It is not canonical Kamacite IR, not Valence ownership of Kamacite semantics, not a live Valence ingestion test, and not a semantic-equivalence, evaluator-soundness, compiler-correctness, runtime-authorization, or runtime-correctness claim.

## Profile

The profile is `kamacite.valence_receipt_export.v1`. It binds:

- the source Kamacite receipt identity in the `receipt` domain;
- a distinct export projection identity in the `projection` domain;
- input/output artifact identities from the source receipt with explicit roles;
- the Valence export conformance identity;
- checked laws from the source receipt and conformance entry;
- source receipt diagnostics as a deterministic diagnostic summary;
- source spans when present, or explicit absence when spans are not present;
- all source receipt non-claims plus export-profile non-claims.

The export body is hashed with BLAKE3 over deterministic JSON projection bytes without the export identity. The export identity never substitutes for the source receipt identity.

## Conformance entry

`fixtures/valence-export/conformance.json` is the reviewed conformance entry for the current profile. It names the source schema, output schema, required identity domains, output identity domain, checked laws, artifacts-first dependency profile, fixture coverage, and non-claims. Valence-like exports without a matching conformance entry fail closed.

## Command

Run the command with explicit inputs:

```bash
nix run .#kamacite -- export-valence-receipt --root . --receipt fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json --conformance fixtures/valence-export/conformance.json --golden fixtures/valence-export/golden/source-envelope.valid.nickel-minimal.valence-export.json
```

Default mode is non-mutating. If the computed export differs from the supplied golden projection, the command reports the mismatch and exits nonzero without rewriting receipts, manifests, export projections, or golden evidence.

The only writer is the explicit update mode:

```bash
nix run .#kamacite -- export-valence-receipt --root . --receipt fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json --conformance fixtures/valence-export/conformance.json --golden fixtures/valence-export/golden/source-envelope.valid.nickel-minimal.valence-export.json --update-golden --update-report target/kamacite-valence-export-update-receipt.json
```

The update report records the source receipt identity, old export identity when present, new export identity, checked laws, and non-claims.

## Source spans

Source spans are optional evidence. A receipt export with span metadata preserves explicit span objects. A receipt export without span metadata records absence explicitly. Kamacite does not fabricate spans from paths, labels, artifact names, or diagnostics.

## Coupling boundary

This slice is artifacts-first. It does not add a direct Valence crate dependency and does not invoke a live Valence runtime. A later direct dependency must come through a separate accepted lifecycle change with versioning, API ownership, and positive/negative integration fixtures.
