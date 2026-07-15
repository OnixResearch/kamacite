## Context

Kamacite already runs `cargo clippy --workspace --all-targets -- -D warnings` in Nix and has a negative hidden-panic fixture. `-D warnings` does not enable allow-by-default restriction lints. Octet now owns a narrow `canonical-determinism` profile whose production-safety and byte/order restrictions match Kamacite's canonical artifact scope. The stronger profile passes the current Kamacite workspace.

## Decisions

### 1. Adopt the canonical profile, not the blanket restriction group

**Choice:** Deny exactly `dbg_macro`, `mem_forget`, `undocumented_unsafe_blocks`, `multiple_unsafe_ops_per_block`, `host_endian_bytes`, and `iter_over_hash_type`. Never enable `clippy::restriction` as a group.

**Rationale:** These lints close concrete safety and deterministic-codec gaps without importing preference lints or duplicating Kamacite's panic policy. Kamacite's canonical values and receipts justify the byte/order-specific profile.

### 2. Keep one repo-owned flag set

**Choice:** Define one named Nix list of Clippy restriction flags and interpolate it into the positive workspace gate and negative fixture gate.

**Rationale:** One local projection prevents the two enforced commands from drifting while avoiding a runtime or workspace-relative dependency on an Octet checkout.

### 3. Preserve positive and negative evidence

**Choice:** The clean workspace must pass all targets. The existing negative fixture will contain both an `expect` call and a `dbg!` call, and the check must observe both lint identities.

**Rationale:** A positive check alone cannot prove the newly selected allow-by-default lint is actually enabled.

### 4. Keep claim boundaries explicit

**Choice:** Treat this as compiler source-shape evidence only. Narrow reasoned exceptions remain permissible at real kernel, FFI, or platform boundaries, but broad allows are not.

**Rationale:** Clippy does not prove canonical semantics, runtime determinism, panic freedom, memory safety, compiler correctness, formal proof, or release eligibility.

## Risks / Trade-offs

- A future pinned Clippy release may rename or change a selected lint; toolchain updates must replay both fixtures.
- `host_endian_bytes` may require a narrow reasoned exception for a real native ABI boundary.
- `iter_over_hash_type` is syntactic evidence and does not replace Kamacite's deterministic canonicalization tests.
