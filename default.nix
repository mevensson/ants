with (import <nixpkgs> {});
{
  ants = pkgs.callPackage ./ants.nix {};
}
