with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "rust-openvr-sys";
  buildInputs = (with pkgs; [ rustChannels.stable.rust cmake ]);
}
