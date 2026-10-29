{
  lib,
  rustPlatform,

  stdenv,
  buildPackages,

  installShellFiles,
  versionCheckHook,
}:
let
  manifest = (lib.importTOML ./Cargo.toml).workspace.package;
in
rustPlatform.buildRustPackage (finalAttrs: {
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

  postInstall = lib.optionalString (stdenv.hostPlatform.emulatorAvailable buildPackages) (
    let
      emulator = stdenv.hostPlatform.emulator buildPackages;
    in
    ''
      installShellCompletion --cmd hw-pp \
        --bash <(${emulator} $out/bin/${finalAttrs.meta.mainProgram} completion bash) \
        --fish <(${emulator} $out/bin/${finalAttrs.meta.mainProgram} completion fish) \
        --zsh <(${emulator} $out/bin/${finalAttrs.meta.mainProgram} completion zsh)
    ''
  );

  nativeBuildInputs = [ installShellFiles ];

  nativeInstallCheckInputs = [ versionCheckHook ];
  versionCheckProgramArg = "--version";
  doInstallCheck = true;

  meta = {
    description = "Hello world++";
    homepage = manifest.repository;
    license = lib.licenses.mit;
    mainProgram = "hw-pp";
  };
})
