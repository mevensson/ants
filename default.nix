let
  nixpkgs = import ./nixpkgs.nix;
  pkgs = import nixpkgs {
    overlays = [
      (import ./overlay.nix)
    ];
  };
in
  pkgs.ants
