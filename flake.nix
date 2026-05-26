{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  # probably ai garbage
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: 
      let 
        pkgs = import nixpkgs { inherit system; };
        lib = pkgs.lib;
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;

        pineapple_pastebin = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
      in {
        packages.default = pineapple_pastebin;

        nixosModules.default = (import ./nix/pineapple_pastebin.nix { inherit pkgs lib; pineapple_pastebin = pineapple_pastebin; });
      });
}