{
  description = "Kamacite lossless interchange boundary tools";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.cairn.url = "github:OnixResearch/cairn";
  inputs.cairn.inputs.nixpkgs.follows = "nixpkgs";
  inputs.schema-migration-core-src = {
    url = "git+https://seed.radicle.garden/z1C4YVMgDGyVdQa72uPNj3UDS5cY.git?rev=4fe90e130f2871cf69a6febcdc70785adca98aea";
    flake = false;
  };

  outputs =
    {
      self,
      nixpkgs,
      cairn,
      schema-migration-core-src,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      nickelExportCargoHash = "sha256-QbvbyqFoqlR+eW4QgRiTZPhxTHuEpf30mrXCOmxJ0gk=";
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
        in
        {
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
          default = self.apps.${system}.kamacite;
        }
      );

      checks = eachSystem (
        pkgs:
        let
          system = pkgs.stdenv.hostPlatform.system;
          kamacite = "${self.packages.${system}.kamacite}/bin/kamacite";
          lifecycleValidate = "${self.packages.${system}.lifecycle-validate}/bin/kamacite-lifecycle-validate";
          copySource = ''
            cp -r ${self} source
            chmod -R u+w source
            cd source
          '';
          valenceConformanceIdentity = "d3ae754d7899a794d09781914d5fb4eb6ebdb9becce16ddc0615988864725830";
          staleProjectionIdentity = "0000000000000000000000000000000000000000000000000000000000000000";
          canonicalDeterminismClippyFlags = pkgs.lib.concatStringsSep " " [
            "-D clippy::dbg_macro"
            "-D clippy::mem_forget"
            "-D clippy::undocumented_unsafe_blocks"
            "-D clippy::multiple_unsafe_ops_per_block"
            "-D clippy::host_endian_bytes"
            "-D clippy::iter_over_hash_type"
          ];
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
              pkgs.rustc
              pkgs.rustfmt
              pkgs.util-linux
              cairn.packages.${system}.cairn
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
