builtins.fetchGit {
  # Descriptive name to make the store path easier to identify
  name = "nixpkgs-unstable-2020-01-02";
  url = https://github.com/nixos/nixpkgs/;
  # `git ls-remote https://github.com/nixos/nixpkgs-channels nixpkgs-unstable`
  rev = "9f03cb8562645cb1cd515b8553d1dc94c3402a2e";
}
