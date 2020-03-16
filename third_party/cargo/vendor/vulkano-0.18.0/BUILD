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


# Unsupported target "build-script-build" with type "custom-build" omitted

rust_library(
    name = "vulkano",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/crossbeam-0.7.3:crossbeam",
        "//third_party/cargo/vendor/fnv-1.0.6:fnv",
        "//third_party/cargo/vendor/half-1.5.0:half",
        "//third_party/cargo/vendor/lazy_static-1.4.0:lazy_static",
        "//third_party/cargo/vendor/shared_library-0.1.9:shared_library",
        "//third_party/cargo/vendor/smallvec-1.2.0:smallvec",
        "//third_party/cargo/vendor/vk-sys-0.5.1:vk_sys",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.18.0",
    crate_features = [
    ],
)

