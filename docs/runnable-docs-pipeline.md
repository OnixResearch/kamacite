# Runnable documentation pipeline

Kamacite treats documentation as evidence data, not as canonical source syntax or a publication runtime.

The checked boundary is:

1. preserve human-authored documentation or example bytes as source envelopes;
2. validate documentation IR artifacts with explicit page, section, anchor, link, prose, code-block, runnable-cell, dependency, metadata, and non-claim fields;
3. render HTML, Markdown, and AsciiDoc as deterministic `documentation-output` projections;
4. validate runnable-cell receipts only under an explicit run profile with declared language, evaluator identity, declared capabilities, expected output or expected diagnostic, and non-claims;
5. bind documentation IR, renderer, output, runnable-cell, and Valence-friendly export identities in documentation receipts.

Default validation remains non-executing. Fixtures may carry stored results for Nickel, Steel, Rust-validator, and Preserves examples, but those receipts do not prove arbitrary source equivalence, runtime correctness, or evaluator soundness.
