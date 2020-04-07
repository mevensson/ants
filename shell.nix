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
          version = "10.2.1";
          sha256 = "1bh6ws20yi757b4im5aa6zcjmsgdqxvr1rg86kfa638cd5ad1f97";
        }
        {
          name = "rust";
          publisher = "rust-lang";
          version = "0.7.5";
          sha256 = "0xgaqyg55bkjsbq5ci73va37pj8d3yr7g8kp7aiql0gz107500fw";
        }
        {
          name = "python";
          publisher = "ms-python";
          version = "2020.4.76186";
          sha256 = "0arqz1778hp3sk95f3i1p1kyykpn9y7k1h256jpxn1m4723ccbn6";
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
