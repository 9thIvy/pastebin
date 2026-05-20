{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
  }:
  flake-parts.lib.mkFlake { inherit inputs; }
  {
    imports = [
      ./default.nix
      ./nix/nixos.nix
      ./nix/package.nix
    ];

    overlays = {
      pineapple_pastebin = import ./nix/overlay.nix;
    };
    
  };

  
}