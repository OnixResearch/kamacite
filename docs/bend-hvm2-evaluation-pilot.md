# Bend and HVM2 Evaluation Pilot

## Scope

This pilot evaluates one CPU-only Bend and HVM2 cohort. It does not add a production runtime.

The cohort contains these exact sources:

- Bend 0.2.38 at `814453670d0e0d6777c1313c972764dba0491b7f`
- HVM2 2.0.22 at `654276018084b8f44a22b562dd68ab18583bfb5b`
- Trellis at `f26e3cd465101698ade12f8d142e0dc95578326b`
- Octet production Verus from `374bd16b26cee2af34211a29bfa531c016811f51`
- GCC 15.2.0 for `x86_64-unknown-linux-gnu`

Bend and HVM2 use Apache-2.0. The repository retains both exact license files.

CUDA, HVM3, HVM4, Kind, runtime I/O, and consumer integration are outside this pilot.

## Workload

The workload contains the ordered leaves `[16777215, 1, 0, 1]`. The result is `1` under `u24` wrapping addition.

The Bend program uses a finite balanced binary tree. Every public function has an explicit type.

The source rejects `Any` and all declared authority tokens. The pure core also rejects empty, irregular, oversized, and invalid leaf vectors.

The typed Nickel profile declares all limits before execution. It also declares repetitions, spread limits, and performance ratios.

## Architecture

The pure core owns all cohort, source, proof, input, observation, benchmark, decision, receipt, and projection rules.

The shell owns file access, process execution, the clock, stream capture, and receipt writes.

The shell uses fixed argument vectors. It clears the environment and gives each child one writable scratch directory.

Bubblewrap creates separate user, mount, PID, and network namespaces. The sandbox mounts a local `/proc` for Bend stack inspection.

The local `/proc` shows only sandbox processes. It does not expose the host process namespace.

`prlimit` bounds CPU time, address space, file size, and process count. The shell also bounds captured output and diagnostics.

These controls are recorded observations. They do not prove complete sandboxing.

## Proof handoff

The pilot consumes `module:bounded_modular_tree_fold` from the exact Trellis proof bundle.

The handoff binds the algorithm profile and this oracle symbol:

`verified_logic::bounded_modular_tree_fold::sequential_u24_sum`

The Nix rail reruns the exact Trellis source through the pinned Octet production Verus provider.

The proof bundle checker also validates source identities, requirements, proof functions, proof gaps, and non-claims.

The Trellis result is a local verified oracle. It is not proof of Bend, HVM2, generated C, or GCC.

## Execution routes

The shell runs these independent routes over the same canonical leaves:

1. The Trellis oracle binary calls the exact optimized Rust symbol.
2. `bend run-rs` uses the reviewed Bend frontend and Rust HVM route.
3. `bend gen-c` emits C, GCC builds it, and the shell runs the executable.

The second and third routes share the Bend frontend. Their agreement alone is not sufficient.

The final decision requires equality with the Trellis oracle. It also requires all declared resource and benchmark rules.

## Artifact roles

Each artifact keeps a separate BLAKE3 identity and role:

- Bend source envelope
- canonical leaf input
- adapter profile
- Trellis proof bundle
- Trellis proof validation
- `bend run-rs` observations
- generated C
- compiler invocation
- generated executable
- generated-executable observations
- pilot decision and receipt

Preserves remains the canonical Kamacite IR. Bend, HVM, C, and executable bytes remain target or observation artifacts.

The Valence projection marks only the Trellis proof validation as `verified-trellis-local`. Generated artifacts remain `recorded-only`.

## Reproduction

Create an empty scratch directory:

```bash
scratch=$(mktemp -d)
```

Run the pinned pilot:

```bash
NIX_CONFIG='builders =' nix run .#bend-hvm2-pilot -- --root . --scratch-root "$scratch"
```

Write a retained evidence bundle only after you review the target directory:

```bash
NIX_CONFIG='builders =' nix run .#bend-hvm2-pilot -- \
  --root . \
  --scratch-root "$scratch" \
  --update-evidence
```

The default evidence directory is `fixtures/bend-hvm2/golden/`.

## Decision

The pilot decision is `defer` for the 1 GiB address-space profile.

Source generation and GCC compilation pass. The Trellis oracle returns `1` for every warm-up and measured run.

`bend run-rs` returns no HVM result under the limit. The generated executable stops with signal-class exit code `139`.

The generated C reserves at least 10 GiB in fixed global buffers. This lower bound excludes smaller runtime structures.

The pilot does not emit a benchmark summary because the two Bend routes do not complete. It does not classify missing output as semantic disagreement.

The retained decision and observations are in `fixtures/bend-hvm2/golden/`.

A later evaluation can review a larger address-space profile or a bounded HVM2 layout. That evaluation requires a new profile identity.

Only an `adopt` decision can support a later consumer proposal. Every consumer still requires a separate Cairn change.

## Non-claims

This pilot does not prove these properties:

- Bend correctness
- HVM2 correctness
- generated-C semantic equivalence
- host-compiler correctness
- verifier soundness
- sandbox completeness
- portable performance
- runtime adoption
- production readiness
- release eligibility
