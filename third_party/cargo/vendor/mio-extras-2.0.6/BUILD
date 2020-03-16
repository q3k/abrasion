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
  "restricted", # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "mio_extras",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/lazycell-1.2.1:lazycell",
        "//third_party/cargo/vendor/log-0.4.8:log",
        "//third_party/cargo/vendor/mio-0.6.21:mio",
        "//third_party/cargo/vendor/slab-0.4.2:slab",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "2.0.6",
    crate_features = [
    ],
)

# Unsupported target "test" with type "test" omitted
