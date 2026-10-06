{
  description = "hw-pp: hello world plusplus (lib + CLI)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    systems = {
      url = "github:nix-systems/default";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      systems,
      git-hooks,
      ...
    }:
    let
      forEachSystem =
        f:
        nixpkgs.lib.genAttrs (import systems) (
          system:
          f system (
            import nixpkgs {
              inherit system;
              overlays = [ rust-overlay.overlays.default ];
            }
          )
        );

      # Single source of truth, shared with rustup / CI via rust-toolchain.toml.
      mkToolchain = pkgs: pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in
    {
      checks = forEachSystem (
        system: pkgs:
        let
          toolchain = mkToolchain pkgs;
        in
        {
          pre-commit-check = git-hooks.lib.${system}.run {
            src = ./.;
            # Sandboxed `nix flake check` has no network: vendor crates from Cargo.lock.
            settings.rust.check.cargoDeps = pkgs.rustPlatform.importCargoLock {
              lockFile = ./Cargo.lock;
            };
            hooks = {
              # Nix hygiene.
              nixfmt.enable = true;
              statix.enable = true;
              deadnix.enable = true;

              # Rust, using the same toolchain as the dev shell.
              rustfmt = {
                enable = true;
                packageOverrides = {
                  cargo = toolchain;
                  rustfmt = toolchain;
                };
              };
              clippy = {
                enable = true;
                # Same bar as CI: warnings (incl. pedantic) block the commit.
                settings = {
                  allFeatures = true;
                  denyWarnings = true;
                };
                packageOverrides = {
                  cargo = toolchain;
                  clippy = toolchain;
                };
              };

              # just-hook = {
              #    enable = true;
              #    name = "just hook";
              #    entry = "${pkgs.writeShellScriptBin "just-hook" ''
              #      just review-count
              #      # Nudge only (never blocks the commit): remind to refresh the
              #      # README image when UI files are staged. Screenshot stays a
              #      # manual `just screenshot` so PNG blobs don't bloat git history.
              #      if git diff --cached --name-only | grep -qE '^(app|components)/'; then
              #        echo "note: UI files staged - refresh the README image with: just screenshot"
              #      fi
              #    ''}/bin/just-hook";
              #    language = "system";
              #    pass_filenames = false;
              #    always_run = true;
              #    stages = [ "pre-commit" ];
              #  };
              #
              # typecheck = {
              #   enable = true;
              #   name = "typecheck";
              #   entry = "${pkgs.writeShellScriptBin "typecheck-hook" ''
              #     just typecheck
              #   ''}/bin/typecheck-hook";
              #   language = "system";
              #   pass_filenames = false;
              #   always_run = true;
              #   stages = [ "pre-push" ];
              # };
              #
            };
          };
        }
      );

      devShells = forEachSystem (
        system: pkgs:
        let
          inherit (self.checks.${system}.pre-commit-check) shellHook enabledPackages;
        in
        {
          default = pkgs.mkShell {
            # shellHook = installs the git pre-commit hook defined above.
            inherit shellHook;
            packages = [
              (mkToolchain pkgs)
              pkgs.shellcheck
              pkgs.cargo-nextest
              pkgs.just
            ]
            ++ enabledPackages;
          };
        }
      );

      packages = forEachSystem (
        _: pkgs:
        let
          toolchain = mkToolchain pkgs;
        in
        {
          default = pkgs.callPackage ./package.nix {
            rustPlatform = pkgs.makeRustPlatform {
              cargo = toolchain;
              rustc = toolchain;
            };
          };
        }
      );

      formatter = forEachSystem (_: pkgs: pkgs.nixfmt-tree);
    };
}
