{ rust-bin, lib, makeRustPlatform, symlinkJoin, makeWrapper, pkgs, runCommand, ... }:

let
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.stable.latest.default;
    rustc = rust-bin.stable.latest.default;
  };
  keepass-pinentry-unwrapped =rustPlatform.buildRustPackage {
  pname = "pinentry-keepassxc";
  version = "0.3.1";

  src = ./.;

  cargoHash = "sha256-dsQGzNfo4YukGhPAXaMfF+UZnAUTWoVJSUwTXMspu7M=";

  meta = with lib; {
    description = "A pinentry program that reads from your keepass database";
    license = licenses.mit;
    platforms = lib.lists.intersectLists lib.platforms.linux lib.platforms.x86_64;
    mainProgram = "pinentry-keepassxc";
  };
};

in
runCommand keepass-pinentry-unwrapped.name  {
  inherit (keepass-pinentry-unwrapped) pname version meta;
  nativeBuildInputs = [makeWrapper];
} ''
     makeWrapper ${keepass-pinentry-unwrapped}/bin/pinentry-keepassxc  $out/bin/pinentry-keepassxc \
     --prefix PATH : "${lib.makeBinPath [pkgs.pinentry-all]}"
  ''
