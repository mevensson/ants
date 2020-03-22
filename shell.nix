let
  vscode_overlay = self: super: {
    vscode-with-extensions = super.vscode-with-extensions.override {
      vscodeExtensions = with super.vscode-extensions; [
        bbenoist.Nix
      ] ++ super.vscode-utils.extensionsFromVscodeMarketplace [
        {
          name = "githistory";
          publisher = "donjayamanne";
          version = "0.4.13";
          sha256 = "0alhmd1mi441l7cfv2p7xmz2w34l6vgs0dz8nhw2pap8w67mnyr3";
        }
        {
          name = "gitlens";
          publisher = "eamodio";
          version = "10.2.0";
          sha256 = "0qnq9lr4m0j0syaciyv0zbj8rwm45pshpkagpfbf4pqkscsf80nr";
        }
        {
          name = "rust";
          publisher = "rust-lang";
          version = "0.7.0";
          sha256 = "16n787agjjfa68r6xv0lyqvx25nfwqw7bqbxf8x8mbb61qhbkws0";
        }
        {
          name = "python";
          publisher = "ms-python";
          version = "2020.1.58038";
          sha256 = "09iawy1p2akan090461137d4p5gqqf0aanm9i534p0kmbxmjfpqv";
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

      python27Packages.python

      python37Packages.Keras
      python37Packages.pydot
      python37Packages.pylint
      python37Packages.tensorflow
      python37Packages.tensorflow-tensorboard
    ];
  }
