## Context

Kamacite's current lifecycle package establishes the lossless interchange boundary: source envelopes preserve original bytes, Preserves-backed canonical values own value identity, adapters emit bounded receipts, and Rust is expected to provide pure deterministic validation logic. The repository has not yet introduced a Rust crate, so this is the right moment to narrow what Kamacite Rust is allowed to do.

The main risk is not that Rust cannot implement the validators. The risk is that early convenience choices become implicit product semantics: raw strings for hashes, `HashMap` iteration in canonical paths, `anyhow` in pure cores, evaluator calls hidden behind validators, diagnostics whose wording or order depends on the host, or fixtures that prove only happy paths. Those choices would make later Nickel, Steel, WASM, LLVM, Valence, and Octet integrations harder to reason about.

This change makes the first Rust slice boring on purpose. Rust should be a small, pure, typed admission controller over explicit inputs. Effectful shells may read files, call external tools, print diagnostics, or run evaluators, but only after they normalize facts into explicit DTOs that the core can validate deterministically.

The deeper Rust prior-art pass gives Kamacite a useful name and boundary. Ferrocene/Sealed Rust and Verus show that restricted Rust profiles can be useful without becoming a separate everyday language. HIF shows a bounded, explicit-host-function machine with explicit errors and limits. `cap-std`, WASI, WASI Virt, and Wasmtime's deterministic-execution guidance show how to deny or virtualize ambient capabilities. Oxide projects such as `iddqd`, `typify`, `openapi-lint`, `drift`, and `daft` show how to keep identity, schema translation, bad interchange shapes, unhandled comparisons, and structural diffs honest. Clippy, Dylint, cargo-geiger, Safety Dance, cargo-vet, cargo-deny, and cargo-auditable are source/provenance rails; Kani, Prusti, Creusot, and Verus can supply optional proof or model-check evidence. Kamacite borrows these as prior art, not as ownership transfers.

## Decisions

### 1. Rust is a deterministic admission controller

**Choice:** Kamacite Rust starts with a pure `kamacite-core` style crate for deterministic validation, canonicalization, hash-material construction, artifact classification, adapter DTO checks, and receipt construction. It does not execute Nickel, Steel, WASM, compilers, network clients, filesystem traversals, clocks, random sources, or credential lookups in core logic.

**Rationale:** The core has to be testable without standing up the world. Keeping all ambient effects in a thin shell makes validation repeatable and keeps receipts tied to explicit inputs.

**Rejected alternative:** Let the Rust core invoke evaluators or host services directly. That would make validation depend on ambient state and blur the adapter boundary.

### 2. Core and shell are separate crates or modules

**Choice:** The pure core and effectful shell must be separated structurally before code is trusted. A future `kamacite-cli` or adapter runner may perform IO, discover fixtures, spawn tools, or render diagnostics, but it must feed explicit in-memory DTOs to the core.

**Rationale:** Kamacite needs deterministic validation receipts. IO can still exist, but it should be too thin to hide product semantics.

**Rejected alternative:** Put filesystem discovery, process execution, and receipt validation in one module and rely on review discipline. That would make nondeterminism harder to detect.

### 3. Identity domains are Rust types

**Choice:** Source, canonical value, projection, target artifact, adapter, evaluator, compiler, and receipt identities are distinct Rust types. Hash strings or byte arrays may exist inside those types, but they are not accepted as interchangeable parameters.

**Rationale:** Cross-domain substitution is one of the easiest ways to overclaim evidence. Newtype-style separation makes many invalid states unrepresentable and gives the validator a precise deterministic diagnostic when a boundary value is malformed.

**Rejected alternative:** Use one `Hash` or `String` type everywhere. That would make accidental substitutions compile and push safety into ad hoc runtime checks.

### 4. Artifact classes are closed alternatives

**Choice:** The Rust core models supported artifact classes as explicit closed alternatives: source envelope, canonical value, adapter descriptor, target artifact, receipt, and unsupported claim. Unknown versions, codecs, classes, target kinds, capabilities, and claims fail closed.

**Rationale:** Kamacite must not infer the nearest supported behavior from unknown inputs. Unsupported data can be preserved or diagnosed, but it cannot silently become trusted evidence.

**Rejected alternative:** Accept unknown classes with best-effort defaults. That would create compatibility surprises and weaken non-claim boundaries.

### 5. Preserves bytes own canonical identity

**Choice:** Where Kamacite controls identity, the Rust core hashes canonical Preserves bytes with BLAKE3. JSON exports and Serde DTOs are compatibility projections with separate projection identities.

**Rationale:** Kamacite already names Preserves as the canonical encoded IR candidate. The Rust implementation must not accidentally replace that authority with convenient JSON hashes.

**Rejected alternative:** Use JSON serialization as canonical hash material. JSON cannot carry enough lossless distinctions for Kamacite and is too easy to reorder or normalize inconsistently.

### 6. Determinism is observable

**Choice:** Validation output, diagnostics, receipt fields, canonical collections, and fixture results are ordered deterministically. Canonical paths use ordered maps/sets or explicit sorting before hashing, serialization, or presentation. Host-dependent ordering, locale, current directory, modification time, random seed, or OS error wording cannot affect core outputs.

**Rationale:** Determinism has to be visible in golden receipts and replayable validations. If two equivalent runs produce different ordering or messages, downstream evidence consumers cannot safely compare them.

**Rejected alternative:** Treat nondeterministic ordering as a presentation detail. In Kamacite, presentation and receipt data can become evidence, so ordering must be controlled before emission.

### 7. Diagnostics are total data, not panics

**Choice:** The Rust core returns typed diagnostics with stable codes, field paths or spans when available, artifact class or identity context when available, and explicit rejected claims. Validation paths do not panic, unwrap, truncate, silently default, coerce identity domains, or accept partial success for rejected artifacts.

**Rationale:** Negative fixtures are first-class evidence. A bad artifact should produce a stable reason for rejection, not a stack trace, host-specific error, or accepted partial value.

**Rejected alternative:** Use panic or opaque error strings for impossible cases. Kamacite's validator must handle hostile or malformed input as normal data.

### 8. Effects and capabilities are declared before use

**Choice:** Any Rust surface that validates adapter/evaluator receipts must treat filesystem, process, network, clock, random, credential, mutation, compiler, evaluator, and similar authority as explicit declared capabilities. Undeclared capabilities fail closed with deterministic diagnostics.

**Rationale:** Kamacite separates algebraic effect descriptions, adapter capabilities, and host authority. Rust validation must enforce that split rather than infer permission from names or adapter kinds.

**Rejected alternative:** Trust adapter identity or handler names to imply allowed effects. Names are metadata and cannot grant authority.

### 9. Fixtures are the admission gate

**Choice:** New Rust validation behavior is trusted only after positive, negative, stale-hash, wrong-identity-domain, unsupported-version, malformed-input, and golden-receipt fixtures exist for the behavior.

**Rationale:** Happy-path tests cannot prove Kamacite fails safely. The fixture corpus should demonstrate both acceptance and deterministic rejection before claims are admitted.

**Rejected alternative:** Add fixtures after the validator is considered usable. That would let untested claim shapes leak into receipts.

### 10. Crate-level rails block convenience drift

**Choice:** The first Rust crate forbids unsafe code in trusted core paths, denies panic-prone and debug-only helpers in core validation paths, uses domain error enums in the core, and leaves generalized error plumbing to the shell.

**Rationale:** These rails are not enough by themselves, but they make review and automation cheaper. The core should expose domain facts and diagnostics, while the shell can translate them into CLI exits or human-readable output.

**Rejected alternative:** Start with a permissive crate and tighten later. Early permissive patterns tend to become de facto API contracts.

### 11. The first implementation slice is deliberately small

**Choice:** The first Rust implementation slice should include only source envelope DTOs, BLAKE3 source identity, typed identity wrappers, closed artifact classification, bounded non-claim receipts, deterministic diagnostics, and fixtures for those surfaces.

**Rationale:** Kamacite can validate the Rust discipline before adding Nickel, Steel, WASM, LLVM, algebraic-effect, Valence, or Octet integrations.

**Rejected alternative:** Implement multiple adapter integrations in the first Rust crate. That would mix core discipline work with integration risk.

### 12. Call this a Kamacite Rust Core Profile, not a new language

**Choice:** Kamacite names the restricted trusted Rust surface the Kamacite Rust Core Profile. The profile is a reviewed implementation profile over ordinary Rust code, not a Rust fork, language extension, compiler qualification, or certification claim.

**Rationale:** Ferrocene/Sealed Rust and Verus are useful prior art for explicit restricted Rust surfaces, but Kamacite's goal is narrower: deterministic Kamacite validation and receipt construction. Profile language makes the boundary clear without implying that Kamacite owns Rust semantics.

**Rejected alternative:** Call the surface a Kamacite Rust dialect or an OxCaml-style language extension. That would overclaim compiler and type-system changes Kamacite is not making.

### 13. Borrow prior art by role, not wholesale

**Choice:** Each prior-art project has a bounded role before Kamacite adopts a pattern. HIF informs explicit host-function and limit modeling. `cap-std`, WASI, WASI Virt, and Wasmtime determinism inform shell/adapter capability controls. Oxide schema and identity projects inform typed identity, fail-closed maps, and honest schema/interchange boundaries. Clippy, Dylint, cargo-geiger, Safety Dance, cargo-vet, cargo-deny, and cargo-auditable inform source/provenance rails. Kani, Prusti, Creusot, Verus, and Trellis can provide optional proof or model-check evidence.

**Rationale:** Prior art is most useful when its claim boundary is explicit. Kamacite should not inherit another project's runtime, identity format, proof semantics, diagnostic renderer, supply-chain policy, or certification claim by reference.

**Rejected alternative:** Treat one prior-art project as the Kamacite Rust profile authority. That would blur Kamacite's product semantics and make later Valence/Octet integration less precise.

### 14. Capability libraries belong in shells, not core

**Choice:** Capability-oriented libraries such as `cap-std`, WASI, and WASI Virt may be used later in shells or adapter runners, but the trusted Kamacite core still receives only explicit normalized DTOs, capability declarations, artifact bytes, diagnostics, and receipts. For filesystem-facing shell code that handles user-controlled relative paths, Kamacite should prefer a declared root capability such as `cap_std::fs::Dir` or an equivalent capability-safe resolver over raw ambient paths.

**Rationale:** Capability handles are still authority. They can make shells safer by making path traversal, absolute-path mistakes, and symlink escapes explicit denial cases, but passing those handles into the core would violate the pure admission-controller boundary. Shells should turn capability-mediated observations into deterministic DTOs before invoking core validation.

**Rejected alternative:** Use capability-safe APIs as a reason to allow filesystem, network, clock, random, or environment access in core validation. That would make the core effectful even if the effect is better-scoped.

**Rejected alternative:** Treat `cap-std` as a sandbox for untrusted Rust adapters. `cap-std` can constrain well-behaved shell IO, but untrusted adapter execution still needs a stronger isolation boundary such as WASI or Wasmtime with explicit imports.

### 15. Pretty diagnostics and snapshots are non-canonical helpers

**Choice:** Libraries such as `miette`, `insta`, `snapbox`, and `trycmd` may help shells render diagnostics and test output, but Kamacite canonical diagnostics and receipts remain stable data structures with Kamacite-owned canonical identity rules.

**Rationale:** Human-friendly rendering and snapshot review are valuable but should not become the canonical evidence format by accident.

**Rejected alternative:** Hash pretty terminal output or snapshot formatting as Kamacite evidence. That would bind identity to presentation details.

## Risks / Trade-offs

- **More boilerplate up front:** Mitigate with small newtypes, focused constructors, and fixture helpers rather than generic stringly-typed DTOs.
- **Slower adapter progress:** Mitigate by implementing the tiny core first, then layering adapters behind explicit descriptors.
- **Duplicate-looking identities:** Mitigate by documenting domain-specific wrappers and projections in receipts instead of sharing one hash type.
- **Strict diagnostics maintenance:** Mitigate with golden fixtures and stable diagnostic codes.
- **Lint overreach:** Mitigate by limiting the strongest bans to core validation paths and allowing shell-only plumbing where deterministic DTOs have already been produced.
- **Preserves library uncertainty:** Mitigate by keeping canonical hash material behind a narrow interface and using fixtures to lock behavior before broad adoption.
