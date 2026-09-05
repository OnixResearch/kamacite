{
  description = "Kamacite lossless interchange boundary tools";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.cairn.url = "github:OnixResearch/cairn";
  inputs.cairn.inputs.nixpkgs.follows = "nixpkgs";
  inputs.octet = {
    url = "github:OnixResearch/octet/cf04e894e53eb0947230118a086ef6066ddba38c";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.bend-src = {
    url = "github:HigherOrderCO/Bend/814453670d0e0d6777c1313c972764dba0491b7f";
    flake = false;
  };
  inputs.hvm2-src = {
    url = "github:HigherOrderCO/HVM/654276018084b8f44a22b562dd68ab18583bfb5b";
    flake = false;
  };
  inputs.trellis-src = {
    url = "github:OnixResearch/trellis/f26e3cd465101698ade12f8d142e0dc95578326b";
    flake = false;
  };
  inputs.trellis-octet.url = "github:OnixResearch/octet/374bd16b26cee2af34211a29bfa531c016811f51";
  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay/c807e83cc2e32adc35f51138b3bdef722c0812ab";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.schema-migration-core-src = {
    url = "git+https://seed.radicle.garden/z1C4YVMgDGyVdQa72uPNj3UDS5cY.git?rev=4fe90e130f2871cf69a6febcdc70785adca98aea";
    flake = false;
  };

  outputs =
    {
      self,
      nixpkgs,
      cairn,
      octet,
      bend-src,
      hvm2-src,
      trellis-src,
      trellis-octet,
      rust-overlay,
      schema-migration-core-src,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      nickelExportCargoHash = "sha256-QbvbyqFoqlR+eW4QgRiTZPhxTHuEpf30mrXCOmxJ0gk=";
      octetLintNames = nixpkgs.lib.filter (name: name != "") (
        nixpkgs.lib.splitString "\n" (builtins.readFile "${octet}/tests/fixtures/lint_inventory_names.txt")
      );
      octetDenyFlags = nixpkgs.lib.concatMapStringsSep " " (name: "-D ${name}") (
        [ "unknown_lints" ] ++ octetLintNames
      );
    in
    {
      packages = eachSystem (
        pkgs:
        let
          system = pkgs.stdenv.hostPlatform.system;
          nickelSandboxTools = [
            pkgs.bubblewrap
            pkgs.nickel
            pkgs.util-linux
          ];
          nickelRuntimePath = pkgs.lib.makeBinPath nickelSandboxTools;
          nickelRuntimeClosure = pkgs.closureInfo { rootPaths = [ pkgs.nickel ]; };
          trellisRustPackages = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          trellisRustToolchain = trellisRustPackages.rust-bin.fromRustupToolchainFile "${trellis-src}/rust-toolchain.toml";
          trellisRustPlatform = trellisRustPackages.makeRustPlatform {
            cargo = trellisRustToolchain;
            rustc = trellisRustToolchain;
          };
          trellisPilotSource = pkgs.runCommand "trellis-bend-hvm2-pilot-source" { } ''
            mkdir -p "$out"
            cp -R ${trellis-src}/. "$out/"
            chmod -R u+w "$out"
            install -m 0644 ${./tools/bend-hvm2-oracle.rs} \
              "$out/src/bin/kamacite-bend-hvm2-oracle.rs"
          '';
          trellisPilotTools = trellisRustPlatform.buildRustPackage {
            pname = "trellis-bend-hvm2-pilot-tools";
            version = "0.2.0";
            src = trellisPilotSource;
            cargoLock.lockFile = "${trellis-src}/Cargo.lock";
            nativeBuildInputs = with trellisRustPackages; [
              pkg-config
              clang
              mold
              nix
            ];
            buildInputs = with trellisRustPackages; [ openssl ];
            cargoBuildFlags = [
              "--bin"
              "kamacite-bend-hvm2-oracle"
              "--bin"
              "proof_evidence"
            ];
            cargoInstallFlags = [
              "--bin"
              "kamacite-bend-hvm2-oracle"
              "--bin"
              "proof_evidence"
            ];
            doCheck = false;
          };
          hvm2 = pkgs.rustPlatform.buildRustPackage {
            pname = "hvm";
            version = "2.0.22";
            src = hvm2-src;
            cargoLock.lockFile = "${hvm2-src}/Cargo.lock";
            doCheck = false;
          };
          bendHvm2 = pkgs.rustPlatform.buildRustPackage {
            pname = "bend-hvm2-pilot";
            version = "0.2.38";
            src = bend-src;
            cargoLock.lockFile = "${bend-src}/Cargo.lock";
            postPatch = ''
              cat >> Cargo.toml <<'EOF'

              [patch.crates-io]
              hvm = { path = "${hvm2-src}" }
              EOF
            '';
            nativeBuildInputs = [ pkgs.makeWrapper ];
            postInstall = ''
              wrapProgram $out/bin/bend --set HVM_BIN ${hvm2}/bin/hvm
            '';
            doCheck = false;
          };
          trellisVerifier = trellis-octet.packages.${system}.octet-production-verus;
          trellisProofRlimit = 3600;
          trellisProofResult =
            pkgs.runCommand "trellis-verus-proofs-exact-source-v1" { nativeBuildInputs = [ trellisVerifier ]; }
              ''
                candidate_root=${trellisVerifier}/libexec/verus
                octet-production-verus \
                  --triggers-mode silent \
                  --rlimit ${toString trellisProofRlimit} \
                  --crate-type=lib \
                  --extern vstd="$candidate_root/libvstd.rlib" \
                  --extern builtin="$candidate_root/libverus_builtin.rlib" \
                  --extern builtin_macros="$candidate_root/libverus_builtin_macros.so" \
                  --extern state_machines_macros="$candidate_root/libverus_state_machines_macros.so" \
                  -L "$candidate_root" \
                  --edition 2021 \
                  ${trellis-src}/src/lib.rs > "$out"
                grep -F 'verification results::' "$out" > /dev/null
                cd ${trellis-src}
                ${trellisPilotTools}/bin/proof_evidence check >> "$out"
              '';
          bendPilotRuntimeClosure = pkgs.closureInfo {
            rootPaths = [
              bendHvm2
              hvm2
              pkgs.stdenv.cc
              trellisPilotTools
            ];
          };
          bendPilotRuntimePath = pkgs.lib.makeBinPath [
            bendHvm2
            hvm2
            pkgs.stdenv.cc
            trellisPilotTools
            pkgs.bubblewrap
            pkgs.util-linux
          ];
          bendPilotRunner = pkgs.writeShellApplication {
            name = "kamacite-bend-hvm2-pilot";
            runtimeInputs = [ ];
            text = ''
              export PATH=${bendPilotRuntimePath}
              exec ${self.packages.${system}.kamacite}/bin/check-bend-hvm2-pilot \
                --runtime-closure-file ${bendPilotRuntimeClosure}/store-paths \
                --proof-result ${trellisProofResult} \
                --verifier-identity-file ${trellisVerifier}/share/identity.txt \
                "$@"
            '';
          };
        in
        {
          inherit hvm2;
          bend-hvm2 = bendHvm2;
          bend-hvm2-pilot = bendPilotRunner;
          trellis-bend-hvm2-tools = trellisPilotTools;
          trellis-bend-hvm2-proof = trellisProofResult;

          kamacite = pkgs.rustPlatform.buildRustPackage {
            pname = "kamacite";
            version = "0.1.0";
            src = self;
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes."nickel-export-core-0.1.0" = nickelExportCargoHash;
            };
            cargoBuildFlags = [
              "-p"
              "kamacite-cli"
            ];
            nativeBuildInputs = [ pkgs.makeWrapper ];
            postInstall = ''
              wrapProgram $out/bin/kamacite-validate \
                --prefix PATH : ${nickelRuntimePath} \
                --set KAMACITE_NICKEL_RUNTIME_CLOSURE_FILE ${nickelRuntimeClosure}/store-paths
              makeWrapper $out/bin/kamacite-validate $out/bin/kamacite \
                --prefix PATH : ${nickelRuntimePath} \
                --set KAMACITE_NICKEL_RUNTIME_CLOSURE_FILE ${nickelRuntimeClosure}/store-paths
              wrapProgram $out/bin/check-world-commit-conformance \
                --prefix PATH : ${nickelRuntimePath} \
                --set KAMACITE_NICKEL_RUNTIME_CLOSURE_FILE ${nickelRuntimeClosure}/store-paths \
                --set STEEL_HOME /homeless \
                --set STEEL_SEARCH_PATHS ""
            '';
            doCheck = false;
          };

          lifecycle-validate = pkgs.writeShellApplication {
            name = "kamacite-lifecycle-validate";
            runtimeInputs = [ cairn.packages.${system}.cairn ];
            text = ''
              exec cairn validate --policy ${cairn}/cairn-policy/generated/cairn-policy.json "$@"
            '';
          };

          default = self.packages.${system}.kamacite;
        }
      );

      apps = eachSystem (
        pkgs:
        let
          system = pkgs.stdenv.hostPlatform.system;
        in
        {
          kamacite = {
            type = "app";
            program = "${self.packages.${system}.kamacite}/bin/kamacite";
            meta.description = "Run the Kamacite CLI";
          };
          kamacite-validate = {
            type = "app";
            program = "${self.packages.${system}.kamacite}/bin/kamacite-validate";
            meta.description = "Run the Kamacite validation CLI by its Cargo binary name";
          };
          lifecycle-validate = {
            type = "app";
            program = "${self.packages.${system}.lifecycle-validate}/bin/kamacite-lifecycle-validate";
            meta.description = "Validate Kamacite Cairn lifecycle artifacts with the pinned Cairn tool";
          };
          world-commit-conformance = {
            type = "app";
            program = "${self.packages.${system}.kamacite}/bin/check-world-commit-conformance";
            meta.description = "Check the pinned Molten world-commit interchange cohort";
          };
          bend-hvm2-pilot = {
            type = "app";
            program = "${self.packages.${system}.bend-hvm2-pilot}/bin/kamacite-bend-hvm2-pilot";
            meta.description = "Run the exact CPU-only Bend and HVM2 evaluation pilot";
          };
          default = self.apps.${system}.kamacite;
        }
      );

      checks = eachSystem (
        pkgs:
        let
          system = pkgs.stdenv.hostPlatform.system;
          kamacite = "${self.packages.${system}.kamacite}/bin/kamacite";
          bendPilot = "${self.packages.${system}.bend-hvm2-pilot}/bin/kamacite-bend-hvm2-pilot";
          bend = "${self.packages.${system}.bend-hvm2}/bin/bend";
          hvm2 = "${self.packages.${system}.hvm2}/bin/hvm";
          trellisPilotTools = self.packages.${system}.trellis-bend-hvm2-tools;
          trellisProofResult = self.packages.${system}.trellis-bend-hvm2-proof;
          lifecycleValidate = "${self.packages.${system}.lifecycle-validate}/bin/kamacite-lifecycle-validate";
          copySource = ''
            cp -r ${self} source
            chmod -R u+w source
            cd source
          '';
          valenceConformanceIdentity = "d3ae754d7899a794d09781914d5fb4eb6ebdb9becce16ddc0615988864725830";
          staleProjectionIdentity = "0000000000000000000000000000000000000000000000000000000000000000";
          bendNegativeCaseCount = 20;
          bendPilotObservationCount = 20;
          bendPilotMaximumLeaf = 16777215;
          bendPilotCombineCount = 3;
          canonicalDeterminismClippyFlags = pkgs.lib.concatStringsSep " " [
            "-D clippy::dbg_macro"
            "-D clippy::mem_forget"
            "-D clippy::undocumented_unsafe_blocks"
            "-D clippy::multiple_unsafe_ops_per_block"
            "-D clippy::host_endian_bytes"
            "-D clippy::iter_over_hash_type"
          ];
          bendPilotRun = pkgs.runCommand "kamacite-bend-hvm2-pilot-run" { } ''
            ${copySource}
            mkdir scratch
            ${bendPilot} --root . --scratch-root "$PWD/scratch" \
              --update-evidence > pilot-summary.json
            mkdir -p "$out"
            cp pilot-summary.json "$out/summary.json"
            cp -R scratch "$out/scratch"
            cp -R fixtures/bend-hvm2/golden "$out/golden"
          '';
        in
        {
          package = self.packages.${system}.kamacite;

          schema-migration-core-pilot-source =
            pkgs.runCommand "kamacite-schema-migration-core-pilot-source"
              { nativeBuildInputs = [ pkgs.ripgrep ]; }
              ''
                test '${schema-migration-core-src.rev}' = '4fe90e130f2871cf69a6febcdc70785adca98aea'
                rg --fixed-strings '4fe90e130f2871cf69a6febcdc70785adca98aea' ${self}/pilots/schema-migration/Cargo.toml ${self}/pilots/schema-migration/source.ncl > /dev/null
                rg --fixed-strings 'kamacite.schema-migration-plan-receipt.v1' ${self}/pilots/schema-migration/source.ncl ${self}/pilots/schema-migration/src/receipt.rs ${self}/schemas/preserves/schema-migration-plan-receipt-v1.preserves > /dev/null
                rg --fixed-strings 'root_rust_version = "1.88"' ${self}/pilots/schema-migration/source.ncl > /dev/null
                rg --fixed-strings 'pilot_rust_version = "1.95.0"' ${self}/pilots/schema-migration/source.ncl > /dev/null
                touch "$out"
              '';

          bend-hvm2-profile =
            pkgs.runCommand "kamacite-bend-hvm2-profile"
              {
                nativeBuildInputs = [
                  pkgs.diffutils
                  pkgs.gnugrep
                  pkgs.jq
                  pkgs.nickel
                ];
              }
              ''
                ${copySource}
                test '${bend-src.rev}' = '814453670d0e0d6777c1313c972764dba0491b7f'
                test '${hvm2-src.rev}' = '654276018084b8f44a22b562dd68ab18583bfb5b'
                test '${trellis-src.rev}' = 'f26e3cd465101698ade12f8d142e0dc95578326b'
                nickel export --format json adapters/bend-hvm2/profile.ncl > profile.json
                cmp profile.json adapters/bend-hvm2/generated/profile.json
                nickel export --format json fixtures/bend-hvm2/negative-cases.ncl > negative-cases.json
                cmp negative-cases.json fixtures/bend-hvm2/negative-cases.json
                test "$(jq 'length' negative-cases.json)" -eq ${toString bendNegativeCaseCount}
                grep -F 'hvm = "=2.0.22"' ${bend-src}/Cargo.toml > /dev/null
                cmp ${bend-src}/LICENSE-APACHE adapters/bend-hvm2/upstream/bend/LICENSE-APACHE
                cmp ${hvm2-src}/LICENSE adapters/bend-hvm2/upstream/hvm2/LICENSE
                for schema in \
                  schemas/preserves/bend-source-envelope-v1.preserves \
                  schemas/preserves/bend-hvm2-evaluator-profile-v1.preserves \
                  schemas/preserves/bend-hvm2-trellis-proof-handoff-v1.preserves \
                  schemas/preserves/bend-hvm2-compiler-target-v1.preserves \
                  schemas/preserves/bend-hvm2-observation-v1.preserves \
                  schemas/preserves/bend-hvm2-decision-v1.preserves \
                  schemas/preserves/bend-hvm2-pilot-receipt-v1.preserves \
                  schemas/preserves/bend-hvm2-valence-projection-v1.preserves
                do
                  test -s "$schema"
                done
                test "$(${bend} --version)" = 'bend-lang 0.2.38'
                test "$(${hvm2} --version)" = 'hvm 2.0.22'
                ${trellisPilotTools}/bin/kamacite-bend-hvm2-oracle \
                  ${toString bendPilotMaximumLeaf} 1 0 1 > oracle.out
                grep -F 'Result: 1' oracle.out > /dev/null
                grep -F -- '- COMBINES: ${toString bendPilotCombineCount}' oracle.out > /dev/null
                test -f ${trellisProofResult}
                touch "$out"
              '';

          bend-hvm2-pilot-run = bendPilotRun;

          bend-hvm2-pilot =
            pkgs.runCommand "kamacite-bend-hvm2-pilot" { nativeBuildInputs = [ pkgs.gnugrep ]; }
              ''
                grep -F '"decision": "defer"' ${bendPilotRun}/summary.json > /dev/null
                grep -F '"code": "hvm2-address-space-profile"' ${bendPilotRun}/summary.json > /dev/null
                grep -F '"benchmark_present": false' ${bendPilotRun}/summary.json > /dev/null
                grep -F '"observation_count": ${toString bendPilotObservationCount}' ${bendPilotRun}/summary.json > /dev/null
                ${copySource}
                mkdir dirty-scratch
                touch dirty-scratch/unexpected
                if ${bendPilot} --root . --scratch-root "$PWD/dirty-scratch" > dirty.out 2>&1; then
                  echo 'expected non-empty scratch root to fail' >&2
                  exit 1
                fi
                grep -F 'scratch root must be empty' dirty.out > /dev/null
                touch "$out"
              '';

          octet-deny-all =
            (octet.lib.mkConsumerCheck {
              inherit system;
              src = self;
              packages = [
                "kamacite-core"
                "kamacite-cli"
              ];
              cargoExtraArgs = "--all-targets --all-features";
              cargoLock = ./Cargo.lock;
            }).overrideAttrs
              (_previous: {
                DYLINT_RUSTFLAGS = octetDenyFlags;
              });

          rustfmt =
            pkgs.runCommand "kamacite-rustfmt-check"
              {
                nativeBuildInputs = [
                  pkgs.cargo
                  pkgs.rustfmt
                ];
              }
              ''
                ${copySource}
                cargo fmt --check
                touch $out
              '';

          rust-tests = pkgs.rustPlatform.buildRustPackage {
            pname = "kamacite-tests";
            version = "0.1.0";
            src = self;
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes."nickel-export-core-0.1.0" = nickelExportCargoHash;
            };
            cargoBuildFlags = [
              "--workspace"
              "--all-targets"
            ];
            cargoTestFlags = [ "--workspace" ];
            nativeBuildInputs = [ pkgs.nickel ];
          };

          clippy = pkgs.rustPlatform.buildRustPackage {
            pname = "kamacite-clippy";
            version = "0.1.0";
            src = self;
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes."nickel-export-core-0.1.0" = nickelExportCargoHash;
            };
            nativeBuildInputs = [ pkgs.clippy ];
            buildPhase = ''
              runHook preBuild
              cargo clippy --workspace --all-targets -- -D warnings ${canonicalDeterminismClippyFlags}
              runHook postBuild
            '';
            installPhase = ''
              runHook preInstall
              touch $out
              runHook postInstall
            '';
            doCheck = false;
          };

          clippy-rejects-forbidden-panic =
            pkgs.runCommand "kamacite-clippy-rejects-forbidden-panic"
              {
                nativeBuildInputs = [
                  pkgs.cargo
                  pkgs.clippy
                  pkgs.gnugrep
                ];
              }
              ''
                ${copySource}
                if cargo clippy --manifest-path fixtures/clippy-negative/Cargo.toml -- -D warnings ${canonicalDeterminismClippyFlags} >clippy-negative.out 2>&1; then
                  echo "expected hidden panic and restriction violations to fail Clippy" >&2
                  exit 1
                fi
                grep -q "expect_used" clippy-negative.out
                grep -q "dbg_macro" clippy-negative.out
                touch $out
              '';

          manifest-projection-fresh = pkgs.runCommand "kamacite-manifest-projection-fresh" { } ''
            ${copySource}
            ${kamacite} check-manifest-projection \
              --source fixtures/manifest.ncl \
              --contract fixtures/manifest-contract.ncl \
              --projection fixtures/manifest.json >manifest-projection.json
            ${kamacite} generate-manifest --root . >manifest-generation.json
            touch $out
          '';

          ast-grep-adapter-audits =
            pkgs.runCommand "kamacite-ast-grep-adapter-audits"
              {
                nativeBuildInputs = [
                  pkgs.ast-grep
                  pkgs.diffutils
                  pkgs.gnugrep
                  pkgs.nickel
                ];
              }
              ''
                ${copySource}
                nickel export --format json audits/ast-grep/profile.ncl >profile.json
                cmp profile.json audits/ast-grep/generated/profile.json
                {
                  for path in \
                    audits/ast-grep/sgconfig.yml \
                    audits/ast-grep/rules/core-ambient-process.yml \
                    audits/ast-grep/rule-tests/core-ambient-process-test.yml \
                    audits/ast-grep/rule-tests/__snapshots__/core-ambient-process-snapshot.yml
                  do
                    printf '%s\n' "--- $path"
                    cat "$path"
                    printf '\n'
                  done
                } >rule-bundle.txt
                cmp rule-bundle.txt audits/ast-grep/generated/rule-bundle.txt
                ast-grep test --config audits/ast-grep/sgconfig.yml
                ast-grep scan --warning \
                  --rule audits/ast-grep/rules/core-ambient-process.yml \
                  --json=compact audits/ast-grep/fixtures/positive >positive.json
                grep -q '"ruleId":"core-ambient-process"' positive.json
                ast-grep scan --warning \
                  --rule audits/ast-grep/rules/core-ambient-process.yml \
                  --json=compact audits/ast-grep/fixtures/negative >negative.json
                test "$(cat negative.json)" = "[]"
                ast-grep scan \
                  --rule audits/ast-grep/rules/core-ambient-process.yml \
                  --json=compact crates/kamacite-core/src >core.json
                test "$(cat core.json)" = "[]"
                touch $out
              '';

          fixture-validation = pkgs.runCommand "kamacite-fixture-validation" { } ''
            ${copySource}
            ${kamacite} validate --root . --manifest fixtures/manifest.json >fixture-validation.json
            touch $out
          '';

          world-commit-conformance =
            pkgs.runCommand "kamacite-world-commit-conformance"
              {
                nativeBuildInputs = [
                  pkgs.b3sum
                  pkgs.diffutils
                  pkgs.gnugrep
                  pkgs.nickel
                ];
              }
              ''
                ${copySource}
                checker=${self.packages.${system}.kamacite}/bin/check-world-commit-conformance
                revision=2f5d7bf9017b3b00df36e55cf0bbef6d68187611
                rejected_revision=1111111111111111111111111111111111111111
                "$checker" --root . --plan-update "$rejected_revision" >world-commit-plan.json
                grep -q '"requires_new_cohort": true' world-commit-plan.json
                grep -q '"admitted": false' world-commit-plan.json
                "$checker" --root . >world-commit-check.json
                grep -q '"verdict": "pass"' world-commit-check.json
                "$checker" --root . --update --accept-revision "$revision" >world-commit-update-first.json
                b3sum \
                  fixtures/world-commit/product-source-manifest.json \
                  fixtures/world-commit/implementation-manifest.json \
                  fixtures/world-commit/vector-manifest.json \
                  fixtures/world-commit/cohort.json \
                  fixtures/world-commit/golden/world-commit-conformance-receipt.preserves \
                  fixtures/world-commit/golden/world-commit-conformance-receipt.json \
                  fixtures/world-commit/golden/world-commit-valence-projection.json \
                  >world-commit-first.hashes
                "$checker" --root . --update --accept-revision "$revision" >world-commit-update-second.json
                b3sum \
                  fixtures/world-commit/product-source-manifest.json \
                  fixtures/world-commit/implementation-manifest.json \
                  fixtures/world-commit/vector-manifest.json \
                  fixtures/world-commit/cohort.json \
                  fixtures/world-commit/golden/world-commit-conformance-receipt.preserves \
                  fixtures/world-commit/golden/world-commit-conformance-receipt.json \
                  fixtures/world-commit/golden/world-commit-valence-projection.json \
                  >world-commit-second.hashes
                cmp world-commit-first.hashes world-commit-second.hashes
                if "$checker" --root . --update --accept-revision "$rejected_revision" \
                  >unsafe-update.out 2>&1; then
                  echo "expected unapproved world-commit update to fail" >&2
                  exit 1
                fi
                grep -q 'requires --accept-revision' unsafe-update.out
                printf '\n' >>fixtures/world-commit/source/logical-v1.preserves
                if "$checker" --root . >source-drift.out 2>&1; then
                  echo "expected altered Molten source fixture to fail" >&2
                  exit 1
                fi
                grep -q 'stale' source-drift.out
                touch $out
              '';

          wasm-component-static-profile =
            pkgs.runCommand "kamacite-wasm-component-static-profile"
              {
                nativeBuildInputs = [
                  pkgs.diffutils
                  pkgs.nickel
                ];
              }
              ''
                ${copySource}
                nickel export --format json adapters/wasm-component/profile.ncl >profile.json
                cmp profile.json adapters/wasm-component/generated/profile.json
                nickel export --format json fixtures/wasm-component/manifest.ncl >fixture-manifest.json
                cmp fixture-manifest.json fixtures/wasm-component/manifest.json
                test -s adapters/wasm-component/upstream/runtime.wit
                test -s adapters/wasm-component/upstream/molten-profile.json
                touch $out
              '';

          spacewasm-mvp-static-profile =
            pkgs.runCommand "kamacite-spacewasm-mvp-static-profile"
              {
                nativeBuildInputs = [
                  pkgs.b3sum
                  pkgs.diffutils
                  pkgs.jq
                  pkgs.nickel
                ];
              }
              ''
                ${copySource}
                nickel export --format json adapters/spacewasm-mvp/profile.ncl >profile.json
                cmp profile.json adapters/spacewasm-mvp/generated/profile.json
                nickel export --format json fixtures/spacewasm-mvp/manifest.ncl >fixture-manifest.json
                cmp fixture-manifest.json fixtures/spacewasm-mvp/manifest.json
                cmp adapters/spacewasm-mvp/upstream/mantle/mvp-positive.wasm \
                  adapters/spacewasm-mvp/upstream/octet/wasm-artifact.bin

                test "$(b3sum adapters/spacewasm-mvp/upstream/mantle/bundle-manifest.json | cut -d ' ' -f 1)" = \
                  "e3f4afcee53cf9bf0401a2aedc5ca47c5d53b92e01dc3b72f2dadc657abd5562"
                test "$(b3sum adapters/spacewasm-mvp/upstream/mantle/materialization-report.json | cut -d ' ' -f 1)" = \
                  "4b9a91e0ef9ac2774ae053f70e690485f516b52b42e1dbcc1c96cdae6b66bece"
                test "$(b3sum adapters/spacewasm-mvp/upstream/mantle/profile.json | cut -d ' ' -f 1)" = \
                  "742ab4c8f4eb81f49de04f8eb74d5b7a87d43638215bd0a71d2ccd8e493d0348"
                test "$(b3sum adapters/spacewasm-mvp/upstream/mantle/mvp-positive.wasm | cut -d ' ' -f 1)" = \
                  "b2568cffcad3c442a90dd65ccb82d6b7e285bf71c2c9908ce539b6b515217bde"
                test "$(b3sum adapters/spacewasm-mvp/upstream/octet/profiles.json | cut -d ' ' -f 1)" = \
                  "f58715eb73f91d8a6fffd565cc3b030e7f1f8296ec8e91851155d1bb5e52bafd"
                test "$(b3sum adapters/spacewasm-mvp/upstream/octet/evidence-rail-receipt.json | cut -d ' ' -f 1)" = \
                  "0362dec1532a9a138f27e1b70d49cd68bf5d6e27653f11cc3af081f0a69a77ab"

                jq -e '.valid == true and .bundle_identity_blake3 == "7775e57a4d07da89ca57ff282ab0d321f1adf70e9d650f3de4e3a329c92a71e6"' \
                  adapters/spacewasm-mvp/upstream/mantle/bundle-verification.json >/dev/null
                jq -e '.status == "passed" and .profile == "spacewasm-mvp" and (.findings | length) == 0 and (.diagnostics | length) == 0' \
                  adapters/spacewasm-mvp/upstream/octet/evidence-rail-receipt.json >/dev/null
                touch $out
              '';

          nickel-preserves-adapter =
            pkgs.runCommand "kamacite-nickel-preserves-adapter"
              {
                nativeBuildInputs = [
                  pkgs.diffutils
                  pkgs.gnugrep
                  pkgs.nickel
                ];
              }
              ''
                ${copySource}
                nickel export --format json adapters/nickel-preserves/profile.ncl >profile.json
                cmp profile.json adapters/nickel-preserves/generated/profile.json
                ${kamacite} adapt nickel-export \
                  --root . \
                  --manifest fixtures/nickel-export/service-config/generated/manifest.json \
                  --export examples/service-config/generated/service.json \
                  --artifact fixtures/nickel-export/service-config/generated/service.json \
                  --packed-output fixtures/nickel-export/service-config/generated/service.preserves \
                  --text-output fixtures/nickel-export/service-config/generated/service.preserves.txt \
                  --receipt fixtures/nickel-export/service-config/generated/service.kamacite-receipt.json \
                  --check >nickel-preserves-validation.json
                printf '\n' >>fixtures/nickel-export/service-config/generated/service.json
                if ${kamacite} adapt nickel-export \
                  --root . \
                  --manifest fixtures/nickel-export/service-config/generated/manifest.json \
                  --export examples/service-config/generated/service.json \
                  --artifact fixtures/nickel-export/service-config/generated/service.json \
                  --packed-output fixtures/nickel-export/service-config/generated/service.preserves \
                  --receipt fixtures/nickel-export/service-config/generated/service.kamacite-receipt.json \
                  --check >stale-artifact.out 2>&1; then
                  echo "expected stale nickel-export artifact to fail" >&2
                  exit 1
                fi
                grep -q "artifact verification failed" stale-artifact.out
                touch $out
              '';

          nickel-adapter-validation = pkgs.runCommand "kamacite-nickel-adapter-validation" { } ''
            ${copySource}
            ${kamacite} validate-nickel-adapter --root . --manifest fixtures/manifest.json >nickel-adapter-validation.json
            touch $out
          '';

          steel-adapter-validation = pkgs.runCommand "kamacite-steel-adapter-validation" { } ''
            ${copySource}
            ${kamacite} validate-steel-adapter --root . --manifest fixtures/manifest.json >steel-adapter-validation.json
            touch $out
          '';

          valence-export-validation =
            pkgs.runCommand "kamacite-valence-export-validation"
              {
                nativeBuildInputs = [
                  pkgs.gnugrep
                ];
              }
              ''
                ${copySource}
                run_valence_export() {
                  ${kamacite} export-valence-receipt \
                    --root . \
                    --receipt fixtures/golden/source-envelope.valid.nickel-minimal.receipt.json \
                    --conformance fixtures/valence-export/conformance.json \
                    --golden fixtures/valence-export/golden/source-envelope.valid.nickel-minimal.valence-export.json
                }
                run_valence_export >valence-export-validation.json
                substituteInPlace fixtures/valence-export/conformance.json \
                  --replace-fail '${valenceConformanceIdentity}' '${staleProjectionIdentity}'
                if run_valence_export >valence-export-stale.out 2>&1; then
                  echo "expected stale Valence conformance identity to fail" >&2
                  exit 1
                fi
                grep -q "conformance admission failed" valence-export-stale.out
                touch $out
              '';

          function-address-receipt-validation =
            pkgs.runCommand "kamacite-function-address-receipt-validation"
              {
                nativeBuildInputs = [
                  pkgs.diffutils
                  pkgs.gnugrep
                ];
              }
              ''
                ${copySource}
                mkdir -p target/function-address-check
                ${kamacite} emit-function-address-receipt \
                  --root . \
                  --report fixtures/function-address/octet-valid-report.json \
                  --source fixtures/function-address/source.rs=fixtures/function-address/source.rs \
                  --out target/function-address-check/receipt.json \
                  --valence-out target/function-address-check/valence.json \
                  >target/function-address-check/report.json
                cmp fixtures/function-address/octet-valid-receipt-v3.json \
                  target/function-address-check/receipt.json
                cmp fixtures/function-address/octet-valid-valence-v3.json \
                  target/function-address-check/valence.json
                if ${kamacite} emit-function-address-receipt \
                  --root . \
                  --report fixtures/function-address/octet-wrong-domain-report.json \
                  --source fixtures/function-address/source.rs=fixtures/function-address/source.rs \
                  >target/function-address-check/negative.out 2>&1; then
                  echo "expected wrong-domain function-address fixture to fail" >&2
                  exit 1
                fi
                grep -q "invalid structured Octet identity" \
                  target/function-address-check/negative.out
                touch $out
              '';

          stale-manifest-detects-drift =
            pkgs.runCommand "kamacite-stale-manifest-detects-drift"
              {
                nativeBuildInputs = [ pkgs.gnugrep ];
              }
              ''
                ${copySource}
                printf '\n' >> fixtures/manifest.json
                if ${kamacite} validate --root . --manifest fixtures/manifest.json >stale-manifest.out 2>stale-manifest.err; then
                  echo "expected stale manifest generation evidence to fail" >&2
                  exit 1
                fi
                grep -q "projection_identity" stale-manifest.out stale-manifest.err
                touch $out
              '';

          cairn-lifecycle = pkgs.runCommand "kamacite-cairn-lifecycle" { } ''
            ${copySource}
            ${lifecycleValidate} --root . >cairn-lifecycle.json
            touch $out
          '';
        }
      );

      devShells = eachSystem (
        pkgs:
        let
          system = pkgs.stdenv.hostPlatform.system;
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.ast-grep
              pkgs.bubblewrap
              pkgs.cargo
              pkgs.clippy
              pkgs.nickel
              pkgs.nix
              pkgs.pre-commit
              pkgs.rustc
              pkgs.rustfmt
              pkgs.util-linux
              cairn.packages.${system}.cairn
              octet.packages.${system}.cargo-octet
              self.packages.${system}.kamacite
            ];
            KAMACITE_NICKEL_RUNTIME_CLOSURE_FILE = "${
              pkgs.closureInfo { rootPaths = [ pkgs.nickel ]; }
            }/store-paths";
          };
        }
      );
    };
}
