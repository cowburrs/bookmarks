{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      advisory-db,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;

        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [
            # Add additional build inputs here
          ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
          # CARGO_PROFILE = "release";
          # CARGO_PROFILE_RELEASE_LTO = "fat";
          # CARGO_PROFILE_RELEASE_OPT_LEVEL = "3";
          # Its not strictly necessary tbh
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        my-crate = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            nativeBuildInputs = [ pkgs.installShellFiles ];
            postInstall = ''
              installManPage target/man/*.1
              installShellCompletion --cmd bm \
              --bash <(cat target/completions/bm.bash) 
            '';
          }
        );
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit my-crate;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-crate-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings --deny clippy::suspicious --deny clippy::style --deny clippy::complexity --deny clippy::perf";
            }
          );

          my-crate-doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              # This can be commented out or tweaked as necessary, e.g. set to
              # `--deny rustdoc::broken-intra-doc-links` to only enforce that lint
              env.RUSTDOCFLAGS = "--deny warnings";
            }
          );

          # Check formatting
          my-crate-fmt = craneLib.cargoFmt {
            inherit src;
          };

          my-crate-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
            # taplo arguments can be further customized below as needed
            # taploExtraArgs = "--config ./taplo.toml";
          };

          # Audit dependencies
          my-crate-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Audit licenses
          my-crate-deny = craneLib.cargoDeny {
            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          my-crate-nextest = craneLib.cargoNextest (
            commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
              cargoNextestPartitionsExtraArgs = "--no-tests=pass";
            }
          );
        };

        packages =
          let
            bm = my-crate;
            book = pkgs.stdenv.mkDerivation {
              name = "mdbook";
              src = ./.;
              nativeBuildInputs = with pkgs; [
                mdbook
              ];
              installPhase = ''
                runHook preInstall
                mdbook build ./docs
                cp -r ./docs/book $out
                runHook postInstall
              '';
            };
          in
          {
            inherit bm;
            default = pkgs.stdenv.mkDerivation {
              name = "bm";
              dontUnpack = true;
              nativeBuildInputs = [
                pkgs.installShellFiles
                pkgs.makeWrapper
              ];

              installPhase = ''
                runHook preInstall
                mkdir -p $out/bin

                makeWrapper ${bm}/bin/bookmarks $out/bin/bm \
                --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.zoxide ]}

                runHook postInstall
              '';

              postInstall = ''
                mkdir -p $out/share/man
                ln -s ${bm}/share/man/man1 $out/share/man/man1

                mkdir -p $out/share/bash-completion
                ln -s ${bm}/share/bash-completion/completions $out/share/bash-completion/completions
              '';
            };
            book = pkgs.writeShellApplication {
              name = "mkbook";
              text = ''
                xdg-open ${book}/index.html
              '';
            };
          };

        # apps.default = flake-utils.lib.mkApp {
        #   drv = packages.default;
        # };
        # apps.default = packages.default;

        nixosModules.default =
          {
            config,
            lib,
            pkgs,
            ...
          }:
          let
            inherit (lib.options) mkEnableOption mkOption;
            inherit (lib.modules) mkIf;
            inherit (lib.types) package;

            cfg = config.programs.bookmarks;
          in
          {
            options.programs.bookmarks = {
              enable = mkEnableOption "bookmarks, blazingly fast bashmarks alternative";
              package = mkOption {
                type = package;
                default = self.packages.${pkgs.system}.default;
                description = "The bookmarks package to use.";
              };
              enableBashIntegration = mkEnableOption "Bash integration" // {
                default = true;
              };
            };
            config = mkIf cfg.enable {
              environment.systemPackages = [ cfg.package ];
              programs = {
                bash.interactiveShellInit = mkIf cfg.enableBashIntegration ''
                  g() {
                      eval "$(${cfg.package}/bin/bookmarks go "$@")"
                  }
                '';
              };
            };
          };
        devShells = {
          cli = pkgs.mkShell {
            # packages = builtins.attrValues self.packages.${system};
            packages = [
              self.packages.${system}.default
            ];
            shellHook = ''
              export REPO_ROOT=$(git rev-parse --show-toplevel)
              export PS1="Pomotimer $"
              export PS1="\[\e[38;5;141m\]❯\[\e[0m\] "
              clear
            '';
          };

          default = craneLib.devShell {
            packages = (
              with pkgs;
              [
                rustfmt
                rust-analyzer
                prettier
                mdbook
                mdformat
              ]
            );

            shellHook = ''
              export REPO_ROOT=$(git rev-parse --show-toplevel)
              export PS1="\n\[\033[1;32m\][nix-shell:\w]\$\[\033[0m\] "
              cargo() {
              case "$1" in
              build|run) echo "use nix to build/run instead" ;;
              *) command cargo "$@" ;;
              esac
              }
              export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH" # Needed on Wayland to report the correct display scale
            '';
          };
        };
      }
    );
}
