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
    name = "wayland_sys",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/dlib-0.4.1:dlib",
        "//third_party/cargo/vendor/lazy_static-1.4.0:lazy_static",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.21.13",
    crate_features = [
        "client",
        "cursor",
        "dlopen",
        "egl",
        "lazy_static",
    ],
)

