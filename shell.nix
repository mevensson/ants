let
  vscode_overlay = self: super: {
    vscode-with-extensions = super.vscode-with-extensions.override {
      vscodeExtensions = with super.vscode-extensions; [
        bbenoist.Nix
      ] ++ super.vscode-utils.extensionsFromVscodeMarketplace [
        {
          name = "githistory";
          publisher = "donjayamanne";
          version = "0.4.4";
          sha256 = "12vhs0jrzqh1gq38qfq0h3pfpnan01sx4a4ik14hdby0558han9j";
        }
        {
          name = "rust";
          publisher = "rust-lang";
          version = "0.5.3";
          sha256 = "0nkf6cg1hmmsrvryjs5r0pdwsilfmrmy44wz47jjygyy62ixcad9";
        }
      ];
    };
  };
  nixpkgs = import <nixpkgs> {
    overlays = [ vscode_overlay ];
  };
in
  with nixpkgs;
  with import ./default.nix;

  pkgs.mkShell {
    inputsFrom = [
      ants
    ];
    buildInputs = [
      vscode-with-extensions
      which
      git
      cargo
      rustup
    ];
  }
