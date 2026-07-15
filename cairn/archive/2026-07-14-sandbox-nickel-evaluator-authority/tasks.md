## Phase 1: Sandbox profile

- [x] [serial] Define the versioned Nickel sandbox profile and typed receipt fields for evaluator, import closure, environment, authority, and named resource policies. r[kamacite.findings.nickel_sandbox.profile]
- [x] [serial] Implement a sandboxed evaluator shell with a reviewed read-only source/import closure and denied ambient filesystem, environment, network, process, credential, and mutation authority. r[kamacite.findings.nickel_sandbox.import_closure]
- [x] [serial] Enforce named execution, memory, output, and diagnostic limits with deterministic termination results. r[kamacite.findings.nickel_sandbox.resource_limits]

## Phase 2: Evidence integrity

- [x] [serial] Separate requested, admitted, expected, and observed execution facts and stop treating manifest observations as containment evidence. r[kamacite.findings.nickel_sandbox.enforced_evidence]
- [x] [serial] Bind sandbox profile, import closure, evaluator, source, exit, and bounded output identities into Nickel adapter receipts and goldens. r[kamacite.findings.nickel_sandbox.profile] r[kamacite.findings.nickel_sandbox.enforced_evidence]

## Phase 3: Regression evidence

- [x] [serial] Add positive bounded export fixtures and negative import escape, symlink import, undeclared package, environment dependence, resource exhaustion, and oversized-output fixtures. r[kamacite.findings.nickel_sandbox.import_closure] r[kamacite.findings.nickel_sandbox.resource_limits]
- [x] [serial] Run Nickel sandbox integration tests, adapter validation, workspace tests, Clippy, Cairn gates, and Nix checks. r[kamacite.findings.nickel_sandbox.profile] r[kamacite.findings.nickel_sandbox.enforced_evidence]
