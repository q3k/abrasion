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
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "decode" with type "example" omitted
# Unsupported target "decoding_benchmark" with type "bench" omitted

rust_library(
    name = "jpeg_decoder",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/byteorder-1.3.4:byteorder",
        "//third_party/cargo/vendor/rayon-1.3.0:rayon",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.1.19",
    crate_features = [
        "rayon",
    ],
)

