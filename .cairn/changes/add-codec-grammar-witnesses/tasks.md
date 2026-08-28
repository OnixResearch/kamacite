## Phase 1: Baseline and vocabulary

- [ ] [serial] Run baseline workspace, adapter, documentation, and lifecycle tests. r[kamacite.codec_witness.fixtures]
- [ ] [serial] Add versioned Preserves schemas for witnesses, rule graphs, exactness, and generation receipts. r[kamacite.codec_witness.vocabulary]
- [ ] [serial] Add the isolated pure component with named rule, graph, string, and collection limits. r[kamacite.codec_witness.vocabulary]

## Phase 2: Paired production and admission

- [ ] [serial] Produce codec plans and witnesses from one checked schema definition or generated descriptor. r[kamacite.codec_witness.source_of_truth]
- [ ] [parallel] Add stale, detached, cross-schema, and one-sided-generation failure fixtures. r[kamacite.codec_witness.source_of_truth.drift]
- [ ] [serial] Implement exact and conservative witness admission with explicit custom-condition omissions. r[kamacite.codec_witness.exactness]
- [ ] [parallel] Add unknown-field, default, variant, recursion, reference, and bound failures. r[kamacite.codec_witness.exactness.conservative]

## Phase 3: Adapter and tooling integration

- [ ] [serial] Bind witness, codec, schema, generator, and limit identities in adapter descriptors and receipts. r[kamacite.codec_witness.adapter_binding]
- [ ] [parallel] Add compatibility tests for adapters that make no grammar-backed claim. r[kamacite.codec_witness.adapter_binding.compatibility]
- [ ] [serial] Add non-authoritative documentation, completion, sample, and fixture projections. r[kamacite.codec_witness.derived_uses]
- [ ] [parallel] Prove generated samples still require codec admission. r[kamacite.codec_witness.derived_uses.admission]

## Phase 4: Evidence and closeout

- [ ] [serial] Add bounded differential corpora with positive, negative, property, and golden receipt evidence. r[kamacite.codec_witness.fixtures]
- [ ] [serial] Document exactness limits and record the upstream reference and license. r[kamacite.codec_witness.exactness]
- [ ] [serial] Run final workspace tests, documentation checks, Clippy, Octet, Cairn gates, and `nix flake check -L`. r[kamacite.codec_witness.fixtures]
