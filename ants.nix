{ rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "ants";
  version = "1.0";
  name = "${pname}-${version}";

  src = ./.;

  cargoSha256 = "0jacm96l1gw9nxwavqi1x4669cg6lzy9hr18zjpwlcyb3qkw9z7f";
}
