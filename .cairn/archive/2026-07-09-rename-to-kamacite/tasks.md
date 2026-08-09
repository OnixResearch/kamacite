## Phase 1: Naming decision

- [x] [serial] Define Kamacite as the project identity and `kamacite` as the machine namespace. r[kamacite.naming.project_identity] r[kamacite.naming.machine_namespace]
- [x] [serial] Preserve the existing non-claim and scope boundary during the rename. r[kamacite.naming.scope_preservation]

## Phase 2: Implementation

- [x] [serial] Rename documentation, accepted specs, archived lifecycle artifacts, crate/package names, CLI binary, schema IDs, requirement namespaces, fixture manifests, and golden receipts to Kamacite/`kamacite`. r[kamacite.naming.project_identity] r[kamacite.naming.machine_namespace]
- [x] [serial] Regenerate checked golden receipts through the explicit update workflow after namespace and schema IDs change. r[kamacite.naming.receipt_identity_update]
- [x] [serial] Verify Rust checks, fixture validation, Cairn gates, sync, archive, and remaining-reference searches. r[kamacite.naming.validation]
