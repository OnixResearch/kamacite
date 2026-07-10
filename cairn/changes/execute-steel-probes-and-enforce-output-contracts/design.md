## Context

Stored and executed Steel evidence currently share the same DTO. Explicit mode changes only evaluator descriptor text, and output validation is equivalent to `serde_json::Value` parsing plus nonempty schema labels.

## Decisions

### 1. Execution provenance is a closed alternative

**Choice:** Model `StoredEvidence` and `ExecutedProbe` separately. Executed evidence requires a runner receipt carrying engine identity, source/module/input identities, enforced capabilities, exit result, output bytes, and observations.

**Rationale:** Stored bytes cannot be relabeled as a fresh run by changing one descriptor.

### 2. Use a controlled embedded Steel engine where possible

**Choice:** Instantiate the admitted engine with an explicit built-in/module set and typed host-function registry. If a process runner is used, it must satisfy an equivalent sandbox profile.

**Rationale:** The shell must control the exact callable and authority surface it records.

### 3. Schemas are content-addressed executable contracts

**Choice:** Replace free-form schema names with descriptors/identities resolvable to a closed DTO contract. Validate decoded input and output values against those contracts in the pure core.

**Rationale:** Syntactic JSON validity does not establish integer, record, union, required-field, or domain constraints.

### 4. Golden updates require a completed execution receipt

**Choice:** `--update-golden` for executed fixtures is admitted only after successful bounded execution and schema validation.

**Rationale:** Update mode must not bless stale output or arbitrary evaluator labels.

## Risks / Trade-offs

- Embedding Steel adds a larger dependency and versioning surface.
- Existing free-form schema labels need a compatibility migration.
- Higher-order or opaque values remain outside portable DTO conformance and must fail closed or stay descriptor-only.
