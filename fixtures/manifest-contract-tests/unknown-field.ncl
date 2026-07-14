{
  schema_version = 2,
  authority_non_claims = ["negative contract fixture"],
  fixtures = [{
    id = "manifest-contract.negative.unknown-field",
    fixture_class = "negative",
    artifact_class = "receipt",
    requirement_ids = ["r[kamacite.findings.manifest_semantic_freshness.typed_contract]"],
    non_claims = ["not runtime corpus evidence"],
    expected_verdict = "rejected",
    checked_laws = ["closed-payload"],
    golden_receipt = null,
    payload = {
      kind = "identity-use",
      required_domain = "receipt",
      unsupported_field = "must fail closed",
      supplied_identity = {
        domain = "source",
        algorithm = "blake3",
        hex = "0000000000000000000000000000000000000000000000000000000000000000",
      },
    },
  }],
}
