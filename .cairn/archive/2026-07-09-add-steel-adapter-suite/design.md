## Context

Kamacite already validates source envelopes, canonical Preserves values, adapter receipts, Nickel reference-export receipts, target/provenance artifacts, algebraic-effect artifacts, stack role fixtures, and Valence export projections. Steel is still represented by generic adapter receipts that assert DTO-only and ambient-authority behavior without executing a bounded Steel evaluator or binding a Steel-specific evaluator/module/host-function profile.

The accepted adapter boundary says Steel is a bounded adapter/probe evaluator, not canonical IR and not an ambient scripting environment. The suite must therefore be narrower than a general Steel runtime integration. Kamacite owns source preservation, adapter DTO contracts, host-function declarations, output projection identity, deterministic diagnostics, checked laws, receipts, and non-claims. Steel owns evaluation behavior for the executed fragment, and that behavior remains evidence rather than semantic-equivalence proof.

## Decisions

### 1. Execute Steel only in an explicit shell

**Choice:** Steel evaluation, if a fixture requires it, happens only in `kamacite-cli` or a dedicated adapter-runner shell under an explicit Steel-capable validation command. `kamacite-core` receives normalized bytes and DTOs: source identity, evaluator identity, module/import declarations, host-function declarations, capabilities, effect class, output bytes, output projection identity, diagnostics, checked laws, and non-claims.

**Rationale:** The core must remain pure and unit-testable. Steel evaluation may involve an engine, module loading, host-function registration, and evaluator version discovery; those are shell responsibilities and must be represented as explicit evidence before core validation.

### 2. Start with DTO-only probes

**Choice:** The first executable Steel suite should admit only DTO-only probes that consume declared input values, call declared Kamacite host functions, and emit declared output DTO/projection bytes. It should not attempt arbitrary Scheme module semantics, package loading, filesystem probes, network access, clocks, randomness, credentials, mutation, dynamic libraries, or higher-order portable function values.

**Rationale:** DTO-only evaluation proves the Kamacite adapter boundary without turning the suite into a sandbox or Steel semantics project. Broader module or capability behavior can arrive later with narrower fixtures and capability claims.

### 3. Bind evaluator, source, module, and output identities separately

**Choice:** A trusted Steel adapter receipt must bind distinct identities for preserved Steel source when source is present, Steel evaluator/profile data, module or descriptor bytes, and output projection bytes. None of those identities may substitute for a canonical Preserves value unless a later canonical-value adapter explicitly produces Preserves bytes and a separate canonical identity.

**Rationale:** Steel results are adapter evidence. Keeping identity domains separate prevents source bytes, evaluator descriptors, bytecode, closures, JSON, or output DTOs from being mistaken for canonical Kamacite IR.

### 4. Require declared host functions and imports

**Choice:** Every host function available to a Steel fixture must be declared with stable id, input DTO schema, output DTO schema, effect class, required capabilities, and checked laws. Built-in modules and imports must be declared by descriptor before trusted evaluation. Missing, extra, or undeclared host functions/imports fail closed.

**Rationale:** Steel's usefulness comes from embedded host-function boundaries, but those boundaries are also where ambient authority can leak. Typed declarations make host effects reviewable and receipt-bound.

### 5. Keep contracts and callables descriptor-bound

**Choice:** Steel contracts and exported callables may be recorded only as descriptors with stable id, arity, source/module identity, input/output contracts, effect class, and non-claims. Opaque closures, bytecode, and dynamically loaded modules are not portable Kamacite values and must not be exported as canonical IR.

**Rationale:** Higher-order values are not lossless interchange artifacts by default. Descriptors preserve boundary facts without claiming closure serialization, evaluator soundness, or source-to-source semantic equivalence.

### 6. Default validation remains receipt-only

**Choice:** The normal `validate` harness continues to validate stored Steel descriptor/receipt evidence without executing Steel. Fresh Steel execution requires an explicit Steel-capable command or mode, and any golden update must be requested explicitly with update evidence.

**Rationale:** External evaluator availability should not be required for the default fixture gate. Evidence drift from evaluator or output changes must be reviewed, not silently accepted.

### 7. Negative fixtures precede trust

**Choice:** The implementation must add negative fixtures for missing evaluator identity, unsupported evaluator profile, undeclared host function, undeclared ambient authority, stale output projection identity, malformed output, opaque closure export, and Steel bytecode or JSON-as-canonical-IR overclaim before any executed Steel behavior is trusted.

**Rationale:** These cases name the most likely ways a Steel adapter could broaden Kamacite's claim or accidentally grant host authority. The suite is incomplete without fail-closed evidence.

### 8. Defer sandbox/runtime authority claims

**Choice:** This change does not claim Kamacite sandboxes Steel, proves Steel evaluator soundness, verifies arbitrary Steel programs, or authorizes host effects. Runtime confinement, WASI/Wasmtime-backed runners, capability-safe filesystem handles, or direct Steel engine crate coupling may be added only by later lifecycle changes with positive and negative fixtures.

**Rationale:** Kamacite's current role is receipt and adapter-boundary validation. Overclaiming sandbox or evaluator properties would blur the non-claim boundary.

## Risks / Trade-offs

- The Steel evaluator command or embedded engine version may not be available in every development shell. The explicit Steel command should report unavailable evaluator evidence deterministically rather than weakening default validation.
- Binding exact evaluator descriptors and output bytes will surface harmless evaluator-version drift as receipt drift. This is intentional review friction for executable evidence.
- Starting DTO-only leaves richer Steel module/contract behavior deferred, but it keeps the first executable slice small enough to validate fail-closed boundaries.
- If implementation uses a direct Steel Rust crate rather than an external command, crate API churn can affect the shell. The core DTO and receipt boundary should remain stable either way.
