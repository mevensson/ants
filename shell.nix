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
          version = "9.5.1";
          sha256 = "10s2g98wv8i0w6fr0pr5xyi8zmh229zn30jn1gg3m5szpaqi1v92";
        }
        {
          name = "rust";
          publisher = "rust-lang";
          version = "0.5.4";
          sha256 = "1l4m31b4n4gwrfdk008ygj2yzb225kdm44hf7bj6rc7w6qzic2mj";
        }
        {
          name = "python";
          publisher = "ms-python";
          version = "2019.5.18875";
          sha256 = "10kywpisqd31sg8bj1jzri5ir68p65mlz17v9hlp21i1ngjvp5nz";
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
      rls
      openssl
      pkgconfig

      libtensorflow
      python37Packages.Keras
      python37Packages.tensorflow
    ];
  }
