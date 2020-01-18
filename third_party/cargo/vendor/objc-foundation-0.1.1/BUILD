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


# Unsupported target "custom_class" with type "example" omitted
# Unsupported target "example" with type "example" omitted

rust_library(
    name = "objc_foundation",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/block-0.1.6:block",
        "//third_party/cargo/vendor/objc-0.2.7:objc",
        "//third_party/cargo/vendor/objc_id-0.1.1:objc_id",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.1.1",
    crate_features = [
    ],
)

