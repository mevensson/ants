builtins.fetchGit {
  # Descriptive name to make the store path easier to identify
  name = "nixpkgs-unstable-2020-01-02";
  url = https://github.com/nixos/nixpkgs/;
  # `git ls-remote https://github.com/nixos/nixpkgs-channels nixpkgs-unstable`
  rev = "dcb64ea42e64aaecd8e6fef65cc86245c9666818";
}
