# Nickel export to Preserves adapter

Kamacite composes with the pinned `nickel-export-core` admission API before it decodes or maps an export. The adapter does not evaluate Nickel. It admits an `onix-nickel-export-manifest/v3` receipt set, selects exactly one `format=json` destination, verifies the supplied artifact bytes, then maps that closed JSON-compatible value to one typed Preserves root.

## Mapping profile

`adapters/nickel-preserves/profile.ncl` is the human-authored profile. Its checked JSON projection is `adapters/nickel-preserves/generated/profile.json`; the Nix check regenerates and byte-compares it.

Version 1 admits only:

- JSON null, Boolean, and Unicode string values;
- signed or unsigned 64-bit integers written without a decimal point or exponent;
- bounded arrays; and
- bounded objects whose keys are sorted lexicographically before canonicalization.

Decimal, exponent, ambiguous, oversized, malformed, or out-of-range number forms fail closed. Duplicate object keys also reject instead of being merged. Input bytes, packed output bytes, depth, collection size, total values, string bytes, and numeric lexeme bytes have named profile limits.

## Command

Check committed artifacts without evaluating Nickel or writing files:

```console
kamacite-validate adapt nickel-export \
  --root . \
  --manifest fixtures/nickel-export/service-config/generated/manifest.json \
  --export examples/service-config/generated/service.json \
  --artifact fixtures/nickel-export/service-config/generated/service.json \
  --packed-output fixtures/nickel-export/service-config/generated/service.preserves \
  --text-output fixtures/nickel-export/service-config/generated/service.preserves.txt \
  --receipt fixtures/nickel-export/service-config/generated/service.kamacite-receipt.json \
  --check
```

Use `--write` instead of `--check` for explicit publication. All paths are confined beneath `--root`; packed Preserves bytes are written first, optional review text second, and the receipt last. A write that would change nothing is rejected. Check mode requires byte-identical packed, text, and receipt artifacts.

`--artifact` identifies the root-confined file supplying bytes for the selected manifest destination. It is separate from the destination label because committed cross-repository fixtures need not recreate the producer's directory layout.

## Identity and claim boundary

The receipt preserves distinct identities for the exact manifest wire bytes, admitted manifest, export receipt, declared input, source, evaluator artifact, evaluation plan, exact JSON projection, mapping profile, packed Preserves value, review text, and adapter receipt. Packed Preserves bytes under `preserves-packed-canonical-v1` are authoritative. The text rendering is review-only.

This establishes deterministic value preservation for the admitted JSON-compatible projection. It does not establish arbitrary Nickel source losslessness, Nickel semantic equivalence, evaluator correctness, import-closure completeness, authorization, runtime behavior, or release eligibility. Markdown and AsciiDoc rendering are outside this adapter.
