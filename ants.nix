{ rustPlatform, pkgconfig, openssl, libtensorflow }:

rustPlatform.buildRustPackage rec {
  pname = "ants";
  version = "1.0";
  name = "${pname}-${version}";

  nativeBuildInputs = [
    pkgconfig
  ];

  buildInputs = [
    openssl
    libtensorflow
  ];

  src = ./.;

  cargoSha256 = "0f1618knbgsych2lsm3fm1fph32y7g1gqfiknhxfpf51mgnpb846";
}
