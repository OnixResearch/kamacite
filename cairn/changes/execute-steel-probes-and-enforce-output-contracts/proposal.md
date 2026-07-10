## Why

`validate-steel-adapter` does not execute Steel. DTO-probe mode only replaces the evaluator descriptor with arbitrary CLI text, hashes that text, and reuses prerecorded output bytes. The core accepts DTO/projection output when it is any syntactically valid JSON and only checks that schema labels are nonempty. This can turn stored, schema-incompatible output into apparently fresh evaluator-bound evidence.

## What Changes

- Make explicit Steel probe mode execute an admitted embedded engine or reference runner and distinguish executed evidence from stored evidence in the receipt type.
- Derive evaluator identity from the actual engine/version/profile and bind source, modules, host functions, inputs, outputs, and enforced capabilities.
- Resolve content-addressed input/output DTO schemas and validate values against them instead of treating schema labels and JSON syntax as conformance.
- Add negative fixtures for stale prerecorded output, fake evaluator descriptors, wrong-shaped JSON, undeclared imports/functions, and execution/output failures.

## Impact

- **Surfaces**: Steel command shell, Steel adapter DTOs, schema vocabulary, golden receipts, and package checks.
- **Behavior**: explicit probe mode either performs and validates a bounded run or fails; it cannot relabel stored output as fresh.
- **Validation**: embedded-runner tests, schema conformance tests, positive/negative adapter fixtures, and Cairn gates.
