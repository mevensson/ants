let
  vscode_overlay = self: super: {
    vscode-with-extensions = super.vscode-with-extensions.override {
      vscodeExtensions = with super.vscode-extensions; [
        bbenoist.Nix
      ] ++ super.vscode-utils.extensionsFromVscodeMarketplace [
        {
          name = "githistory";
          publisher = "donjayamanne";
          version = "0.6.5";
          sha256 = "0v6f8mkdwm6c0ypjhra22iv5fmmjpdc8fnja70zjkvbgxb48cd2s";
        }
        {
          name = "gitlens";
          publisher = "eamodio";
          version = "10.2.2";
          sha256 = "00fp6pz9jqcr6j6zwr2wpvqazh1ssa48jnk1282gnj5k560vh8mb";
        }
        {
          name = "rust";
          publisher = "rust-lang";
          version = "0.7.8";
          sha256 = "039ns854v1k4jb9xqknrjkj8lf62nfcpfn0716ancmjc4f0xlzb3";
        }
        {
          name = "python";
          publisher = "ms-python";
          version = "2020.5.86806";
          sha256 = "0j3333gppvnn2igw77cbzpsgw6lbkb44l4w7rnpzn9z0q3piy6d4";
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

      python37Packages.autopep8
      python37Packages.Keras
      python37Packages.pydot
      python37Packages.pylint
      python37Packages.tensorflow
      python37Packages.tensorflow-tensorboard
    ];
  }
