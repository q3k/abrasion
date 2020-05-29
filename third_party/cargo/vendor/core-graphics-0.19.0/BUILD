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
    name = "core_graphics",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/core-foundation-0.7.0:core_foundation",
        "//third_party/cargo/vendor/foreign-types-0.3.2:foreign_types",
        "//third_party/cargo/vendor/libc-0.2.71:libc",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.19.0",
    crate_features = [
        "default",
    ],
)

