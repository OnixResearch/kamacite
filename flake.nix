{
  description = "Kamacite lossless interchange boundary tools";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.cairn.url = "github:OnixResearch/cairn";
  inputs.cairn.inputs.nixpkgs.follows = "nixpkgs";

  outputs =
    {
      self,
      nixpkgs,
      cairn,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = eachSystem (
        pkgs:
        let
          system = pkgs.stdenv.hostPlatform.system;
          nickelRuntimePath = pkgs.lib.makeBinPath [ pkgs.nickel ];
        in
        {
          kamacite = pkgs.rustPlatform.buildRustPackage {
            pname = "kamacite";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "kamacite-cli"
            ];
            nativeBuildInputs = [ pkgs.makeWrapper ];
            postInstall = ''
              wrapProgram $out/bin/kamacite-validate \
                --prefix PATH : ${nickelRuntimePath}
              makeWrapper $out/bin/kamacite-validate $out/bin/kamacite \
                --prefix PATH : ${nickelRuntimePath}
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
          manifestProjectionIdentity = "10694659591006cb20d19eb18ab24745f10fc84200edaa41f91d4826f749c162";
          valenceConformanceIdentity = "d3ae754d7899a794d09781914d5fb4eb6ebdb9becce16ddc0615988864725830";
          staleProjectionIdentity = "0000000000000000000000000000000000000000000000000000000000000000";
        in
        {
          package = self.packages.${system}.kamacite;

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
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "--workspace"
              "--all-targets"
            ];
            cargoTestFlags = [ "--workspace" ];
          };

          clippy = pkgs.rustPlatform.buildRustPackage {
            pname = "kamacite-clippy";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.clippy ];
            buildPhase = ''
              runHook preBuild
              cargo clippy --workspace --all-targets -- -D warnings
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
                if cargo clippy --manifest-path fixtures/clippy-negative/Cargo.toml -- -D warnings >clippy-negative.out 2>&1; then
                  echo "expected forbidden hidden panic fixture to fail Clippy" >&2
                  exit 1
                fi
                grep -q "expect_used" clippy-negative.out
                touch $out
              '';

          manifest-projection-fresh = pkgs.runCommand "kamacite-manifest-projection-fresh" { } ''
            ${copySource}
            ${kamacite} check-manifest-projection \
              --source fixtures/manifest.ncl \
              --projection fixtures/manifest.json >manifest-projection.json
            touch $out
          '';

          fixture-validation = pkgs.runCommand "kamacite-fixture-validation" { } ''
            ${copySource}
            ${kamacite} validate --root . --manifest fixtures/manifest.json >fixture-validation.json
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
                substituteInPlace fixtures/manifest.ncl \
                  --replace-fail '${manifestProjectionIdentity}' '${staleProjectionIdentity}'
                if ${kamacite} validate --root . --manifest fixtures/manifest.json >stale-manifest.out 2>stale-manifest.err; then
                  echo "expected stale manifest projection validation to fail" >&2
                  exit 1
                fi
                grep -q "manifest-projection-stale" stale-manifest.out stale-manifest.err
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
              pkgs.cargo
              pkgs.clippy
              pkgs.nickel
              pkgs.rustc
              pkgs.rustfmt
              cairn.packages.${system}.cairn
              self.packages.${system}.kamacite
            ];
          };
        }
      );
    };
}
