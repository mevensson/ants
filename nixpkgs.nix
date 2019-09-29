builtins.fetchGit {
  # Descriptive name to make the store path easier to identify
  name = "nixos-unstable-2019-08-29";
  url = https://github.com/nixos/nixpkgs/;
  # `git ls-remote https://github.com/nixos/nixpkgs-channels nixos-unstable`
  rev = "8d1510abfb592339e13ce8f6db6f29c1f8b72924";
}
