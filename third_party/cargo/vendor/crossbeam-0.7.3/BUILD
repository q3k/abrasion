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
    name = "crossbeam",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/cfg-if-0.1.10:cfg_if",
        "//third_party/cargo/vendor/crossbeam-channel-0.4.2:crossbeam_channel",
        "//third_party/cargo/vendor/crossbeam-deque-0.7.3:crossbeam_deque",
        "//third_party/cargo/vendor/crossbeam-epoch-0.8.2:crossbeam_epoch",
        "//third_party/cargo/vendor/crossbeam-queue-0.2.1:crossbeam_queue",
        "//third_party/cargo/vendor/crossbeam-utils-0.7.2:crossbeam_utils",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.7.3",
    crate_features = [
        "crossbeam-channel",
        "crossbeam-deque",
        "crossbeam-queue",
        "default",
        "std",
    ],
)

# Unsupported target "subcrates" with type "test" omitted
