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
    name = "ndk_glue",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/android_log-sys-0.1.2:android_log_sys",
        "//third_party/cargo/vendor/lazy_static-1.4.0:lazy_static",
        "//third_party/cargo/vendor/libc-0.2.71:libc",
        "//third_party/cargo/vendor/log-0.4.8:log",
        "//third_party/cargo/vendor/ndk-0.1.0:ndk",
        "//third_party/cargo/vendor/ndk-sys-0.1.0:ndk_sys",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.1.0",
    crate_features = [
    ],
)

