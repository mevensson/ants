builtins.fetchGit {
  # Descriptive name to make the store path easier to identify
  name = "nixpkgs-unstable-2020-01-02";
  url = https://github.com/nixos/nixpkgs/;
  # `git ls-remote https://github.com/nixos/nixpkgs-channels nixpkgs-unstable`
  rev = "8fc744839bdc6a23e37f6de9f67bd4916839acc9";
}
