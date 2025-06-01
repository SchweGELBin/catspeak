{
  lib,
  rustPlatform,
  versionCheckHook,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "catspeak";
  version = "1.2.1";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  doInstallCheck = true;
  nativeInstallCheckInputs = [ versionCheckHook ];
  versionCheckProgramArg = "--version";

  meta = {
    description = "Cowsay like program of a speaking cat";
    homepage = "https://github.com/SchweGELBin/catspeak";
    changelog = "https://github.com/SchweGELBin/catspeak/blob/v${finalAttrs.version}/docs/CHANGELOG.md";
    license = lib.licenses.mit;
    mainProgram = finalAttrs.pname;
    maintainers = with lib.maintainers; [ SchweGELBin ];
  };
})
