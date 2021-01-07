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
    name = "crossbeam_deque",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/crossbeam-epoch-0.8.2:crossbeam_epoch",
        "//third_party/cargo/vendor/crossbeam-utils-0.7.2:crossbeam_utils",
        "//third_party/cargo/vendor/maybe-uninit-2.0.0:maybe_uninit",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.7.3",
    crate_features = [
    ],
)

# Unsupported target "fifo" with type "test" omitted
# Unsupported target "injector" with type "test" omitted
# Unsupported target "lifo" with type "test" omitted
# Unsupported target "steal" with type "test" omitted
