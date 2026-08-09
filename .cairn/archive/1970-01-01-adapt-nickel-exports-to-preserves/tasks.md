## Phase 1: Prerequisites, baseline, and profile

- [x] [serial] Complete and archive the real Preserves codec and root-confined filesystem gateway prerequisites before trusting canonical output or write mode. [depends:make-preserves-authority-real] [depends:confine-root-relative-filesystem-io]
- [x] [serial] Record the pre-change workspace, Nickel adapter, manifest, fixture, lifecycle, and package-check baseline before modifying core logic. r[kamacite.nickel_preserves_adapter.fixtures]
- [x] [serial] Define a typed Nickel `nickel-json-to-preserves-v1` policy/contract with freshness-checked runtime projection for accepted manifest/receipt cohorts, mapping and codec versions, output classes, and named depth, collection, string, numeric, input-byte, and output-byte limits. r[kamacite.nickel_preserves_adapter.mapping_profile] r[kamacite.nickel_preserves_adapter.command]
- [x] [serial] Pin and document the reviewed `nickel-export-core` revision/API cohort used for opaque manifest admission and supplied-artifact verification without transferring evaluator semantics into Kamacite. r[kamacite.nickel_preserves_adapter.admission] r[kamacite.nickel_preserves_adapter.composition]

## Phase 2: Pure canonical adaptation core

- [x] [depends:nickel-preserves-profile] Add Kamacite-owned admitted-export, mapping-policy, canonical-artifact, optional text-projection, diagnostic, and receipt DTOs with distinct typed identity domains. r[kamacite.nickel_preserves_adapter.mapping_profile] r[kamacite.nickel_preserves_adapter.receipt]
- [x] [depends:preserves-real-codec] Implement the pure total JSON-alternative mapper with explicit numeric classification, deterministic object ordering, Unicode handling, checked conversions, and named resource-limit enforcement. r[kamacite.nickel_preserves_adapter.mapping_profile]
- [x] [depends:nickel-preserves-mapper] Construct and validate one typed Preserves value, then derive canonical packed bytes, BLAKE3 identity, and optional deterministic review text from that same value through the real codec. r[kamacite.nickel_preserves_adapter.canonical_output] r[kamacite.nickel_preserves_adapter.text_projection]
- [x] [depends:nickel-preserves-canonical-output] Construct deterministic adapter receipts binding manifest, selected export, declared input, source, evaluator artifact/plan, exact JSON bytes, mapping policy, codec, packed output, optional text projection, checked laws, diagnostics, and non-claims. r[kamacite.nickel_preserves_adapter.receipt] r[kamacite.nickel_preserves_adapter.non_claim]

## Phase 3: Thin command shell and materialization

- [x] [depends:nickel-preserves-profile] Integrate the pinned `nickel-export-core` admission and artifact-verification APIs in `kamacite-cli`, select one exact JSON destination, and normalize only admitted in-memory facts into the Kamacite core DTO. r[kamacite.nickel_preserves_adapter.admission] r[kamacite.nickel_preserves_adapter.composition]
- [x] [depends:nickel-preserves-receipt] Add the explicit `adapt nickel-export` check/write command surface with root-relative manifest, selected export, packed output, optional text output, receipt, profile, and codec arguments. r[kamacite.nickel_preserves_adapter.command]
- [x] [depends:root-confined-io] Route every adapter read, comparison, temporary output, replacement, and receipt publication through the root-confined gateway; reject partial publication and emit no trusted success receipt after mutation failure. r[kamacite.nickel_preserves_adapter.command]
- [x] [depends:nickel-preserves-command] Package the pinned adapter dependency and typed policy projection without adding implicit Nickel evaluation, network access, ambient imports, or Markdown/AsciiDoc rendering. r[kamacite.nickel_preserves_adapter.composition] r[kamacite.nickel_preserves_adapter.non_claim]

## Phase 4: Positive and negative evidence

- [x] [parallel] Add positive unit/property fixtures for every admitted JSON alternative, insignificant whitespace/member-order convergence, text/binary value equivalence, identity separation, receipt stability, check mode, and explicit write materialization. r[kamacite.nickel_preserves_adapter.fixtures] r[kamacite.nickel_preserves_adapter.canonical_output]
- [x] [parallel] Add negative core fixtures for ambiguous or out-of-policy numbers, malformed JSON, unsupported mapping/codec/schema versions, every named limit, detached packed bytes/DTOs, stale text projections, and arbitrary-Nickel-equivalence overclaims. r[kamacite.nickel_preserves_adapter.fixtures] r[kamacite.nickel_preserves_adapter.non_claim]
- [x] [parallel] Add negative shell fixtures for unknown or malformed manifests, non-JSON exports, stale manifest/receipt/output identities, missing artifacts, duplicate or absent destination selection, absolute/parent/symlink/replacement escapes, no-op or stale checks, and interrupted/partial publication. r[kamacite.nickel_preserves_adapter.admission] r[kamacite.nickel_preserves_adapter.command] r[kamacite.nickel_preserves_adapter.fixtures]
- [x] [parallel] Add Preserves schema artifacts, golden adapter receipts, Valence-friendly projection fixtures where applicable, operator documentation, and explicit guidance that Nickel-generated Markdown/AsciiDoc remains raw text rather than canonical interchange. r[kamacite.nickel_preserves_adapter.receipt] r[kamacite.nickel_preserves_adapter.text_projection] r[kamacite.nickel_preserves_adapter.non_claim]

## Phase 5: Validation and archive readiness

- [x] [depends:nickel-preserves-fixtures] Run focused core and CLI tests, positive/negative adapter validation, Nickel policy freshness, manifest checks, workspace tests, Clippy, lifecycle validation, proposal/design/tasks gates, Valence export checks where applicable, and the relevant Nix checks; record exact receipts and remaining non-claims. r[kamacite.nickel_preserves_adapter.fixtures] r[kamacite.nickel_preserves_adapter.receipt]
