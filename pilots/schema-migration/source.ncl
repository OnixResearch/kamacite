let Revision = "4fe90e130f2871cf69a6febcdc70785adca98aea" in
let Adapter = "https://seed.radicle.garden/z1C4YVMgDGyVdQa72uPNj3UDS5cY.git" in
let SourceContract = {
  schema | String,
  repository_id | String,
  adapter | String,
  reviewed_revision | String,
  cargo_revision | String,
  nix_revision | String,
  receipt_schema | String,
  receipt_root_label | String,
  adjacent_versions_only | Bool,
  root_rust_version | String,
  pilot_rust_version | String,
  executes_migrations | Bool,
  non_claims | Array String,
} in
{
  schema = "kamacite.schema-migration-pilot.v1",
  repository_id = "rad:z1C4YVMgDGyVdQa72uPNj3UDS5cY",
  adapter = Adapter,
  reviewed_revision = Revision,
  cargo_revision = Revision,
  nix_revision = Revision,
  receipt_schema = "kamacite.schema-migration-plan-receipt.v1",
  receipt_root_label = "kamacite-schema-migration-plan-receipt-v1",
  adjacent_versions_only = true,
  root_rust_version = "1.88",
  pilot_rust_version = "1.95.0",
  executes_migrations = false,
  non_claims = [
    "planning does not prove migration execution",
    "planning does not prove migration implementation correctness",
    "planning does not grant storage or transaction authority",
    "effect requests do not grant execution authority",
  ],
} | SourceContract
