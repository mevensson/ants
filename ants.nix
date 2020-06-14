{ rustPlatform, pkgconfig, openssl, libtensorflow-bin }:

rustPlatform.buildRustPackage rec {
  pname = "ants";
  version = "1.0";
  name = "${pname}-${version}";

  nativeBuildInputs = [
    pkgconfig
  ];

  buildInputs = [
    openssl
    libtensorflow-bin
  ];

  src = ./.;

  cargoSha256 = "0vk89smdj7fpm203c0bbg875xip6y7h75ij24cw2i9m2nvj1ycn0";
}
