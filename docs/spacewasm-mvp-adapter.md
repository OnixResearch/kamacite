# SpaceWasm MVP adapter boundary

Kamacite defines `kamacite-spacewasm-mvp` as an experimental WebAssembly 1.0 core-module adapter profile. It is disjoint from the WIT Component Model suite: a component, WIT world, absent runtime selector, or fallback request is rejected rather than translated or routed to SpaceWasm.

## Exact external cohort

The checked profile binds these committed producers and copied evidence bytes:

- Mantle commit `a141fcbaafe41f9a413a81275a33fe915bfca370`, bundle identity `7775e57a4d07da89ca57ff282ab0d321f1adf70e9d650f3de4e3a329c92a71e6`;
- Octet commit `4367300e10740ecc99ba4b2171ace561b4787327`, static profile `spacewasm-mvp`;
- NASA SpaceWasm commit `e24cf09355a90497148eb5029fdb8e3400bd63e3`;
- exact module identity `b2568cffcad3c442a90dd65ccb82d6b7e285bf71c2c9908ce539b6b515217bde`.

Mantle's pinned bundler independently verifies the full immutable bundle. Octet independently remeasures the selected module and emits a passing execution-free static report. Kamacite checks the exact copied bytes, semantic profile/cohort/module links, typed identity domains, descriptor parent edges, and canonical packed Preserves input.

Replay the external producers from their exact commits with:

```sh
nix build 'git+https://github.com/OnixResearch/mantle?rev=a141fcbaafe41f9a413a81275a33fe915bfca370#spacewasm-reference-bundle'
nix run 'git+https://github.com/OnixResearch/mantle?rev=a141fcbaafe41f9a413a81275a33fe915bfca370#spacewasm-reference-bundler' -- verify "$(readlink -f result)"

# From an Octet checkout at 4367300e10740ecc99ba4b2171ace561b4787327:
cargo octet evidence collect --rail wasm-artifact --input MODULE.wasm \
  --config standards/wasm-artifact/generated/profiles.json \
  --profile spacewasm-mvp --artifact-dir artifacts/spacewasm
cargo octet artifact verify --artifact-dir artifacts/spacewasm
```

## Byte ABI and current blocker

The Kamacite ABI `kamacite.spacewasm.bytes.v1` requires exported memory, `kamacite_alloc`, `kamacite_dealloc`, and `kamacite_invoke`. Inputs and successful outputs are bounded canonical packed Preserves or source-envelope bytes; the runtime never owns canonical IR.

The available Mantle reference module exports only hostless `run`, has no memory, and is correctly accepted by Octet's narrower hostless profile. Therefore its passing external evidence cannot establish the Kamacite byte ABI. Static validation succeeds only as `static-only` with all four blockers:

1. the Mantle reference module does not implement the Kamacite byte ABI;
2. Octet's current `spacewasm-mvp` profile permits only the hostless `run` export;
3. no Mantle bundle materializes a Kamacite guest module under this ABI;
4. no admitted consumer SpaceWasm shell exists.

No execution plan or passing execution receipt can be constructed from these artifacts. Kamacite can instead emit `kamacite.spacewasm-static-receipt.v1`: canonical packed Preserves denial evidence that is rebuilt by rerunning static validation over the exact source, profile, bundle, module, Octet report, descriptor, schema, capability/effect/resource plan, and canonical-input identities. A supplied receipt validates only when it exactly matches that recomputation.

The pure core also exposes canonical-output validation under an explicit output schema and byte bound, but output validation alone is not execution evidence and cannot remove any blocker. Its deterministic Valence-friendly projection assigns only `recorded-only` or `boundary` verification classes, preserves every typed parent edge, records exact byte spans where local bytes own the identity, and records explicit absent-span reasons otherwise. Projection identity and linkage do not transfer Mantle, Octet, SpaceWasm, Kamacite, or Valence trust.

The next external handoff must add a Mantle-produced Kamacite guest module and a matching Octet profile/report without weakening either producer's current claim boundary.

## Authority and non-claims

The initial profile admits no host functions, WASI, filesystem, network, process, environment, clock, random, credential, device, compiler, evaluator, package-manager, or mutation authority. Descriptor possession and static conformance grant no execution authority.

`fixtures/spacewasm-mvp/manifest.ncl` is the typed positive/negative scenario inventory. Execution-dependent cases record the same exact blockers rather than fabricating runtime results; implemented static, identity, canonical-value, overclaim, and stale-receipt cases are exercised by the pure-core tests.

This boundary is not WIT/component compatibility, source-language semantic equivalence, SpaceWasm correctness, memory safety, WebAssembly conformance, sandbox completeness, portable interpreter state, flight qualification, certification, production readiness, or release eligibility.
