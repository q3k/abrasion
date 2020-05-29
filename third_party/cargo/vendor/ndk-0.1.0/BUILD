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
    name = "ndk",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/jni-sys-0.3.0:jni_sys",
        "//third_party/cargo/vendor/ndk-sys-0.1.0:ndk_sys",
        "//third_party/cargo/vendor/num_enum-0.4.3:num_enum",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.1.0",
    crate_features = [
    ],
)

