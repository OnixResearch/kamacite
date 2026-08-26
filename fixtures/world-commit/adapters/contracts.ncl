let OptionalString = std.contract.from_predicate (fun value =>
  std.is_string value || value == null
) in
let Profile = {
  kind | String,
  profile_ref | String,
  cohort_ref | OptionalString,
} in
let Root = {
  domain | String,
  reference | String,
} in
let WorldCommitProjection = {
  schema | String,
  version | String,
  profile | Profile,
  parents | Array String,
  roots | Array Root,
  completeness | Array String,
} in
{
  WorldCommit = WorldCommitProjection,
}
