## Why

Kamacite receipts currently have a `DependencyClosure` identity domain for function-address records, while Octet's first producer evidence is a source-token dependency set. The receipt vocabulary should preserve that distinction so portable receipts do not overclaim compiler-resolved or semantic dependency closure.

## What Changes

- Add or clarify dependency identity kind metadata in function-address receipts.
- Distinguish source-token dependency sets from semantic dependency closure.
- Preserve compatibility with existing v1 receipts through bounded aliases or migration notes.
- Add positive and negative fixtures for dependency identity kind and overclaim rejection.

## Impact

Kamacite remains a lossless portable receipt layer without promoting Octet source-shape evidence into semantic dependency claims.
