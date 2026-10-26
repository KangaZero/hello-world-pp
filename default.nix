{
  lib,
  rustPlatform,
  versionCheckHook,
}:
let
  manifest = (lib.importTOML ./Cargo.toml).workspace.package;
in
rustPlatform.buildRustPackage {
  pname = "hw-pp";
  inherit (manifest) version;

  # Only what cargo needs: editing docs/ or flake.nix won't trigger a rebuild.
  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./crates
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;
  # Ship only the CLI; hw-pp-core's dev-only `stats` bin stays out of $out/bin.
  cargoBuildFlags = [
    "-p"
    "hw-pp-cli"
  ];

  nativeInstallCheckInputs = [ versionCheckHook ];
  versionCheckProgramArg = "--version";
  doInstallCheck = true;

  meta = {
    description = "Hello world++";
    homepage = manifest.repository;
    license = lib.licenses.mit;
    mainProgram = "hw-pp";
  };
}
