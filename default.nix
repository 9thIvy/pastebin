{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  nativeBuildInputs = [ pkgs.pkg-pkg-config pkgs.systemd pkgs.coreutils ];

  buildPhase = ''
    cargo build --release --locked
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp target/release/${pname} $out/bin
    
    mkdir -p $out/lib/systemd/system
  '';


}
