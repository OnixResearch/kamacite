# Human-authored fixture manifest authority for the walking-skeleton gate.
# Runtime Rust consumes fixtures/manifest.json as a deterministic projection.

let Identity = {
  domain | String,
  algorithm | String,
  hex | String,
} in

let Fixture = {
  id | String,
  fixture_class | String,
  artifact_class | String,
  input_artifacts | Array String,
  expected_verdict | String,
  expected_diagnostics | Array String,
  non_claims | Array String,
  dependency_identities | Array Identity,
  requirement_ids | Array String,
  golden_receipt | String,
} in

{
  schema_version | Number = 1,
  runtime_projection | String = "fixtures/manifest.json",
  authority_non_claims | Array String = [
    "manifest source is review authority",
    "json projection is runtime compatibility input only",
    "manifest does not execute Nickel, Steel, WASM, compilers, or host effects",
  ],
  fixtures | Array Fixture = [
    {
      id = "source-envelope.valid.nickel-minimal",
      fixture_class = "positive",
      artifact_class = "source-envelope",
      input_artifacts = ["fixtures/core/source-envelopes/valid.nickel"],
      expected_verdict = "accepted",
      expected_diagnostics = [],
      non_claims = ["not Nickel evaluation semantics", "not arbitrary source translation"],
      dependency_identities = [
        { domain = "source", algorithm = "blake3", hex = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93" },
      ],
      requirement_ids = [
        "r[taenite.validation_harness.seed_fixtures.valid_source_envelope]",
        "r[taenite.lossless_ir.positive_fixtures.source_bytes]",
      ],
      golden_receipt = "fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json",
    },
    {
      id = "source-envelope.negative.stale-source-hash",
      fixture_class = "negative",
      artifact_class = "source-envelope",
      input_artifacts = ["fixtures/core/source-envelopes/stale.nickel"],
      expected_verdict = "rejected",
      expected_diagnostics = ["stale-identity"],
      non_claims = ["not trusted source identity"],
      dependency_identities = [
        { domain = "source", algorithm = "blake3", hex = "dee483bc97a1e84b5e4da7a288a66abfb02665de4edd4036290ce1f31cec3d93" },
      ],
      requirement_ids = [
        "r[taenite.validation_harness.seed_fixtures.stale_source_hash]",
        "r[taenite.rust_determinism.total_diagnostics.stale_hash]",
      ],
      golden_receipt = "fixtures/golden/source-envelope.negative.stale-source-hash.receipt.json",
    },
    {
      id = "source-envelope.negative.wrong-identity-domain",
      fixture_class = "negative",
      artifact_class = "source-envelope",
      input_artifacts = ["fixtures/core/source-envelopes/wrong-domain.nickel"],
      expected_verdict = "rejected",
      expected_diagnostics = ["identity-domain-mismatch"],
      non_claims = ["not cross-domain coercion"],
      dependency_identities = [
        { domain = "projection", algorithm = "blake3", hex = "afe694dfcd747b297c6638d11318a9a5063015cb50123a9bae763d160e2ad2b5" },
      ],
      requirement_ids = [
        "r[taenite.validation_harness.seed_fixtures.wrong_identity_domain]",
        "r[taenite.rust_determinism.typed_identity_domains.constructor_validation]",
      ],
      golden_receipt = "fixtures/golden/source-envelope.negative.wrong-identity-domain.receipt.json",
    },
  ],
}
