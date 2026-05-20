{ inputs, ... }: {
  flake = {
    nixosConfigurations.main = inputs.nixpkgs.lib.nixosSystem {
    modules = [
      ./configuration.nix
      ];
    };
  };
}