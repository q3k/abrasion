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


# Unsupported target "decoder" with type "bench" omitted

rust_library(
    name = "png",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/crc32fast-1.2.0:crc32fast",
        "//third_party/cargo/vendor/deflate-0.8.3:deflate",
        "//third_party/cargo/vendor/inflate-0.4.5:inflate",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.16.1",
    crate_features = [
        "default",
        "deflate",
        "png-encoding",
    ],
)

# Unsupported target "pngcheck" with type "example" omitted
# Unsupported target "show" with type "example" omitted
