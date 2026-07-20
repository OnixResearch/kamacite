# Mantle hello handoff fixture

This frozen fixture records one real producer/consumer boundary from Mantle into Kamacite.

## Provenance

- Producer repository: `../mantle/`
- Producer commit: `9bf75a5ba1a3f4d35754ee7751f3a2f7370ca1f1`
- Capture date: `2026-07-20`
- Producer rail:

  ```sh
  CARGO_TARGET_DIR=/tmp/mantle-handoff-target \
    nix develop -c cargo test -p mantle --test wasm_component_cli \
    production_cli_executes_pinned_pipeline_and_publishes_rehashable_component_evidence \
    -- --exact
  ```

The producer test passed before these three files were copied from its bounded publication directory.

## Frozen identities

- Mantle bundle schema: `mantle-wasm-component-materialization-bundle-v1`
- Mantle bundle identity: `604321e963f859feff992ae6f97ad37e506ec7c16af8e1101876835cbba12b65`
- Selected WIT BLAKE3: `fd3be085e9be42ba03ba132e243c4ac9f6c015398a7410995e6d602206bb17b7`
- Final portable component BLAKE3: `d188411044c9e7cacee8330a82102f642ced6d4ffc1d40861bffa87fd3049ae6`
- Mantle expected runtime-profile BLAKE3: `e9b83b41aca9816f2ac79a0da43fb6fe629a0c212c131dc718aa199cbc030068`

`materialization-bundle.json` retains the producer's original temporary logical locators because those strings participate in the canonical bundle identity. They are recorded locators, not valid paths in this repository.

## Claim boundary

Kamacite validates the canonical bundle identity and required declaration links, then remeasures `selected-world.wit` and `normalized.wasm` against the bundle. The fixture proves that bounded schema-and-byte handoff only.

It does **not** provide the bundle's other stage files, independently verify Mantle's build correctness, transfer Mantle or Octet trust, prove component behavior, or authorize execution. Its `demo:app@0.1.0` / `app` world and runtime-profile identity differ from Kamacite's pinned Molten adapter cohort, so it is not a positive Kamacite execution-plan fixture.
