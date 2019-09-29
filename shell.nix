let
  vscode_overlay = self: super: {
    vscode-with-extensions = super.vscode-with-extensions.override {
      vscodeExtensions = with super.vscode-extensions; [
        bbenoist.Nix
      ] ++ super.vscode-utils.extensionsFromVscodeMarketplace [
        {
          name = "githistory";
          publisher = "donjayamanne";
          version = "0.4.6";
          sha256 = "1wj838iv1xg25a604j4mccdcqhjjcgpaaq6yhnng1ph0s16ypin1";
        }
        {
          name = "gitlens";
          publisher = "eamodio";
          version = "10.0.1";
          sha256 = "0jp1hj3n5xc179afrvarjldnfg3h8sw16kjxcxv1a8wkwzir7iid";
        }
        {
          name = "rust";
          publisher = "rust-lang";
          version = "0.6.3";
          sha256 = "1r5q1iclr64wmgglsr3na3sv0fha5di8xyccv7xwcv5jf8w5rz5y";
        }
        {
          name = "python";
          publisher = "ms-python";
          version = "2019.9.34911";
          sha256 = "18c806dk1chmcnklr8v74fawaal2lkd644yq27pbvffriwj98fib";
        }
      ];
    };
  };
  nixpkgs = import ./nixpkgs.nix;
in
  with import nixpkgs {
    overlays = [
      vscode_overlay
      (import ./overlay.nix)
    ];
  };

  pkgs.mkShell {
    inputsFrom = [
      ants
    ];
    buildInputs = [
      vscode-with-extensions

      git
      less
      which

      rls

      python37Packages.Keras
      python37Packages.tensorflow
    ];
  }
