builtins.fetchGit {
  # Descriptive name to make the store path easier to identify
  name = "nixos-unstable-2019-09-29";
  url = https://github.com/nixos/nixpkgs/;
  # `git ls-remote https://github.com/nixos/nixpkgs-channels nixos-unstable`
  rev = "2436c27541b2f52deea3a4c1691216a02152e729";
}
