with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "rust-openvr";
  buildInputs = (with pkgs; [ rustChannels.stable.rust cmake ]);
}
