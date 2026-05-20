{
  perSystem = { pkgs, ... }: {
    packages = {
      pineapple_pastebin = pkgs.callPackage ../default.nix { inherit pkgs;};
    };
  };
}