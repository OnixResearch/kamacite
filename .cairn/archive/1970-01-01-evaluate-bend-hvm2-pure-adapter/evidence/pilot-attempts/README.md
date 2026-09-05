# Pilot Attempts

## Attempt 1

The first sandbox omitted `/proc`. Bend stopped in `stacker` because `pthread_getattr_np` returned `ENOENT`.

The final sandbox uses an isolated PID namespace and a sandbox-local `/proc`. It does not expose the host process namespace.

## Attempt 2

The second attempt used the declared 1 GiB address-space limit. Source generation and GCC compilation passed.

The Trellis oracle returned `1` in all six runs. Each run also reported three abstract combines.

`bend run-rs` returned no HVM result. The generated executable stopped with signal-class exit code `139`.

The generated C declares these fixed global buffers:

- `2^29` node pairs, which reserve 4 GiB
- `2^29` variable ports, which reserve 2 GiB
- 32 thread bags of `2^24` pairs, which reserve 4 GiB

These arrays reserve at least 10 GiB before smaller structures. This lower bound exceeds the declared 1 GiB limit.

The final core classifies this result as `defer`. It does not classify missing resource-bound output as semantic disagreement.
