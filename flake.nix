{
  description = "hw-pp: hello world++ (lib + CLI)";

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

      # The CI toolchain plus editor-only components, kept out of rust-toolchain.toml so CI
      # does not download them.
      mkDevToolchain =
        pkgs:
        (mkToolchain pkgs).override {
          extensions = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain.components ++ [
            "rust-src"
            "rust-analyzer"
          ];
        };

      # Nix hygiene: the only hooks `nix flake check` runs, since CI's `lint` job already
      # covers rustfmt and clippy.
      nixHooks = {
        nixfmt.enable = true;
        statix.enable = true;
        deadnix.enable = true;
      };

      # Rust, using the same toolchain as CI; local pre-commit only.
      mkRustHooks =
        toolchain:
        let
          tools = {
            cargo = toolchain;
            clippy = toolchain;
            rustfmt = toolchain;
          };
        in
        {
          rustfmt = {
            enable = true;
            packageOverrides = { inherit (tools) cargo rustfmt; };
          };
          clippy = {
            enable = true;
            # Same bar as CI: warnings (incl. pedantic) block the commit.
            settings = {
              allFeatures = true;
              denyWarnings = true;
            };
            packageOverrides = { inherit (tools) cargo clippy; };
          };

          test = {
            enable = true;
            name = "cargo test";
            entry = "${toolchain}/bin/cargo test";
            language = "system";
            pass_filenames = false;
            always_run = true;
            stages = [ "pre-commit" ];
          };
          #
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
    in
    {
      checks = forEachSystem (
        system: _: {
          pre-commit-check = git-hooks.lib.${system}.run {
            src = ./.;
            hooks = nixHooks;
          };
          # Builds the CLI and runs its `--version` install check.
          package = self.packages.${system}.default;
        }
      );

      devShells = forEachSystem (
        system: pkgs:
        let
          inherit
            (git-hooks.lib.${system}.run {
              src = ./.;
              hooks = nixHooks // mkRustHooks (mkToolchain pkgs);
            })
            shellHook
            enabledPackages
            ;
        in
        {
          default = pkgs.mkShell {
            # shellHook = installs the git pre-commit hooks defined above.
            inherit shellHook;
            packages = [
              (mkDevToolchain pkgs)
              pkgs.shellcheck
              pkgs.cargo-nextest
              pkgs.cargo-deny
              pkgs.just
              # `just bench`
              pkgs.hyperfine
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
          default = pkgs.callPackage ./default.nix {
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
