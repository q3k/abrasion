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


# Unsupported target "decode_images" with type "test" omitted
# Unsupported target "encode_images" with type "test" omitted
# Unsupported target "fuzz_tests" with type "test" omitted

rust_library(
    name = "tiff",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/byteorder-1.3.4:byteorder",
        "//third_party/cargo/vendor/lzw-0.10.0:lzw",
        "//third_party/cargo/vendor/miniz_oxide-0.3.6:miniz_oxide",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.0",
    crate_features = [
    ],
)

