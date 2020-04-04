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



rust_library(
    name = "gif",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/color_quant-1.0.1:color_quant",
        "//third_party/cargo/vendor/lzw-0.10.0:lzw",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.10.3",
    crate_features = [
        "default",
        "raii_no_panic",
    ],
)

