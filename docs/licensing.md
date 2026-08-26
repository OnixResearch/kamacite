# Kamacite licensing boundary

| Surface | License |
|---|---|
| `crates/kamacite-core/**` | `MIT OR Apache-2.0` |
| Canonical schemas and reusable interchange fixtures authored by Kamacite | `MIT OR Apache-2.0` |
| `fixtures/world-commit/source/**` and product-derived world-commit vectors | Molten's `AGPL-3.0-or-later`; see `fixtures/world-commit/source/NOTICE.md` |
| `crates/kamacite-cli/**` | `AGPL-3.0-or-later` |
| Repository documentation and lifecycle material not carrying another notice | `AGPL-3.0-or-later` |
| Dependencies, generated artifacts containing upstream material, and external references | Their stated upstream terms |

The core must not depend on `kamacite-cli`. The CLI may depend on the core. Core consumers receive no implicit filesystem, evaluator, process, or network authority.

Complete texts are in `LICENSES/MIT.txt`, `LICENSES/Apache-2.0.txt`, and `LICENSES/AGPL-3.0-or-later.txt`. Each Cargo package directory carries the applicable text so package archives are self-contained.

Prior grants remain valid. Package licensing is outside canonical envelope, Preserves-value, adapter, target-artifact, and receipt identity unless a versioned schema explicitly includes it. This map does not establish legal compliance or behavioral correctness.
