## Context

The Nickel shell invokes a user-selected executable directly and trusts manifest-declared observations. Path admission constrains only the initial source argument; Nickel imports and process ambient state remain outside that check.

## Decisions

### 1. Sandbox profile is explicit data

**Choice:** Define a versioned Nickel execution profile containing evaluator identity policy, read-only source/import roots, environment allowlist, network/process policy, writable outputs, and named resource limits.

**Rationale:** Shell authority must be reviewable and receipt-bound rather than implicit in host defaults.

### 2. Import closure is admitted before trust

**Choice:** Materialize the reviewed source/import closure in an isolated read-only tree or equivalent sandbox mount. Imports outside that closure fail before output is trusted.

**Rationale:** A root-relative entry file can otherwise read arbitrary host files through imports.

### 3. Enforced facts replace self-reported effects

**Choice:** Receipts distinguish requested capabilities, sandbox-admitted capabilities, and observed execution results. Manifest `observed_effects` may be expected evidence but cannot substitute for shell enforcement.

**Rationale:** The input cannot attest to its own containment.

### 4. Bounded process orchestration stays in the shell

**Choice:** Apply explicit execution, CPU, memory, output, and diagnostic budgets; capture exit status and bounded stderr/stdout; terminate exceeded runs; normalize results before calling the core.

**Rationale:** Deterministic validation must not hang or exhaust the host.

### 5. Linux enforcement uses Bubblewrap and prlimit

**Choice:** Run both the evaluator version probe and export inside admitted Nix-store Bubblewrap and `prlimit` tools, with isolated namespaces, an allowlisted environment, a validated read-only Nix store closure, and named resource enforcement. Copy bytes read through root-admitted source/import handles into anonymous read-only sandbox mounts so pathname replacement cannot detach execution from recorded identities. The Nix package provides the tools and closure manifest; unsupported, substituted, or unavailable enforcement fails closed.

**Rationale:** Evaluator identity is only meaningful when it is observed under the same bounded authority model as the export, and ambient runtime-closure paths must not become an authority injection surface.

## Risks / Trade-offs

- Portable sandboxing may require Nix/Linux-specific implementation with explicit unsupported-platform behavior.
- Import-closure discovery must use Nickel semantics or a conservative sandbox, not an incomplete regex parser.
- Resource-limit receipts bound the observed run but do not prove evaluator soundness.
