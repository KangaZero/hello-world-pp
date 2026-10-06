{
  lib,
  rustPlatform,
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

  meta = {
    description = "Hello world plusplus";
    homepage = manifest.repository;
    license = lib.licenses.mit;
    mainProgram = "hw-pp";
  };
}
