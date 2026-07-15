let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in
let OneOf = fun allowed => std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value allowed
) in
let RootRelativePath = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && !(std.string.is_match "^/" value)
  && !(std.string.is_match "(^|/)\\.\\.(/|$)" value)
) in
let Hex64 = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let Identity = {
  domain | OneOf ["source", "adapter", "evaluator"],
  algorithm | OneOf ["blake3"],
  hex | Hex64,
} in
let LanguageProfile = {
  language | OneOf ["Rust", "Nix", "Yaml", "Json"],
  parser_kind | OneOf ["built-in", "custom-tree-sitter"],
  parser_identity | Identity,
  extensions | NonEmptyArray | Array NonEmptyString,
} in
let RuleCoverage = {
  rule_id | NonEmptyString,
  language | NonEmptyString,
  rule_path | RootRelativePath,
  rule_identity | Identity,
  positive_fixture_path | RootRelativePath,
  positive_fixture_identity | Identity,
  negative_fixture_path | RootRelativePath,
  negative_fixture_identity | Identity,
  positive_cases | Number,
  negative_cases | Number,
  gate_eligible | Bool,
} in
{
  Profile = {
    schema_version | OneOf ["kamacite.ast-grep-profile.v1"],
    profile_id | NonEmptyString,
    tool | {
      name | OneOf ["ast-grep"],
      version | NonEmptyString,
    },
    sgconfig_path | RootRelativePath,
    bundle_path | RootRelativePath,
    bundle_identity | Identity,
    bundle_inputs | NonEmptyArray | Array RootRelativePath,
    scan_scope | {
      paths | NonEmptyArray | Array RootRelativePath,
      globs | Array NonEmptyString,
    },
    language_profiles | NonEmptyArray | Array LanguageProfile,
    rules | NonEmptyArray | Array RuleCoverage,
    severity_posture | OneOf ["advisory", "warning", "error"],
    verification_role | OneOf ["recorded-only", "boundary"],
    codemod | {
      mode | OneOf ["dry-run-only"],
      recipe_path | RootRelativePath,
      validation_commands | NonEmptyArray | Array NonEmptyString,
    },
    non_claims | NonEmptyArray | Array NonEmptyString,
  },
}
