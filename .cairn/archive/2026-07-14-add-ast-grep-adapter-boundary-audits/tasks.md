## Tasks

- [x] [serial] r[kamacite.ast_grep_adapter_audits.profile] Define ast-grep adapter-boundary audit profiles and scan scopes.
- [x] [depends:ast-grep-adapter-profile] r[kamacite.ast_grep_adapter_audits.language_boundary] Record built-in or custom tree-sitter language identity for each audit profile.
- [x] [depends:ast-grep-adapter-profile] r[kamacite.ast_grep_adapter_audits.identity] Bind rule bundles, source-envelope scopes, scan receipts, and findings with BLAKE3 identity.
- [x] [depends:ast-grep-adapter-identity] r[kamacite.ast_grep_adapter_audits.fixtures] Add positive and negative fixtures before any rule participates in adapter-conformance gates.
- [x] [depends:ast-grep-adapter-fixtures] r[kamacite.ast_grep_adapter_audits.codemods] Preserve rewrite recipes as dry-run migration artifacts with post-check commands and non-claims.
- [x] [depends:ast-grep-adapter-fixtures] r[kamacite.ast_grep_adapter_audits.valence_export] Export findings only as recorded-only or boundary evidence.
- [x] [depends:ast-grep-adapter-valence-export] r[kamacite.ast_grep_adapter_audits.validation] Run ast-grep rule tests, fixture scans, receipt validation, Cairn gates, and Kamacite validation checks.
