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


# Unsupported target "manual_global_list" with type "example" omitted

rust_library(
    name = "wayland_commons",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/nix-0.14.1:nix",
        "//third_party/cargo/vendor/wayland-sys-0.23.6:wayland_sys",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.23.6",
    crate_features = [
    ],
)

