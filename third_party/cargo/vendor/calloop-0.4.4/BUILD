"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//third_party/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "calloop",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/mio-0.6.22:mio",
        "//third_party/cargo/vendor/mio-extras-2.0.6:mio_extras",
        "//third_party/cargo/vendor/nix-0.14.1:nix",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.4",
    crate_features = [
    ],
)

# Unsupported target "signals" with type "test" omitted
