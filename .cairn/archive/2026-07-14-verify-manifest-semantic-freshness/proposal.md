## Why

Manifest freshness currently line-scans `manifest.ncl` for one hash assignment and fixture IDs. It never evaluates the Nickel contract or compares the source-declared fixture metadata with `manifest.json`. A reproduced change that flipped every accepted fixture verdict to rejected while retaining IDs and the embedded JSON hash still passed with `124/124` fixture IDs, so the JSON projection can contradict the reviewed Nickel authority without detection.

## What Changes

- Replace lexical assignment/ID extraction with a checked Nickel export or a verified generation receipt binding exact source bytes, evaluator identity, contract, and runtime projection bytes.
- Strengthen the Nickel manifest contract so payload variants and trust metadata are typed rather than hidden behind `Dyn`.
- Compare the complete exported manifest, not only fixture IDs, before fixture execution or golden receipt trust.
- Add negative fixtures for metadata-only drift, malformed Nickel, spoofed assignments/comments, duplicate IDs, and stale generation receipts.

## Impact

- **Surfaces**: `fixtures/manifest.ncl`, runtime projection generation, default validation, package checks, and update evidence.
- **Behavior**: any source/projection semantic disagreement fails before fixture materialization.
- **Validation**: focused source/projection tests, packaged stale-manifest checks, fixture validation, and Cairn gates.
