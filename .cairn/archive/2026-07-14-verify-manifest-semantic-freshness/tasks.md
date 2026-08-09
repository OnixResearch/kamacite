## Phase 1: Source authority

- [x] [serial] Capture baseline manifest validation and preserve the reproduced metadata-drift case that currently passes. r[kamacite.findings.manifest_semantic_freshness.complete_projection]
- [x] [serial] Replace the dynamic payload contract with closed typed Nickel fixture alternatives and negative contract fixtures. r[kamacite.findings.manifest_semantic_freshness.typed_contract]
- [x] [serial] Implement explicit checked manifest export and a generation receipt binding source, contract, evaluator/profile, and exact projection identities. r[kamacite.findings.manifest_semantic_freshness.generation_receipt]

## Phase 2: Complete comparison

- [x] [serial] Remove lexical hash/fixture-ID trust and compare the complete checked projection before materialization. r[kamacite.findings.manifest_semantic_freshness.complete_projection]
- [x] [serial] Keep default validation receipt-only when a fresh generation receipt is available and make explicit generation/update non-implicit and receipt-producing. r[kamacite.findings.manifest_semantic_freshness.generation_receipt]

## Phase 3: Regression evidence

- [x] [serial] Add malformed Nickel, comment/string spoof, metadata drift, duplicate/reordered/extra fixture, stale evaluator, stale contract, and stale generation-receipt cases. r[kamacite.findings.manifest_semantic_freshness.negative_cases]
- [x] [serial] Run manifest projection checks, default and adapter validation, workspace tests, Clippy, Cairn gates, and Nix checks. r[kamacite.findings.manifest_semantic_freshness.complete_projection] r[kamacite.findings.manifest_semantic_freshness.negative_cases]
