final: prev:

let
  pkg = final.callPackage ../default.nix { };
  mod = import ./pineapple_pastebin.nix;
in {
  pineapple_pastebin = pkg;
  nixosModules.pineapple_pastebin = mod;
}