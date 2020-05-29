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
    name = "core_video_sys",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/cfg-if-0.1.10:cfg_if",
        "//third_party/cargo/vendor/core-foundation-sys-0.7.0:core_foundation_sys",
        "//third_party/cargo/vendor/core-graphics-0.19.0:core_graphics",
        "//third_party/cargo/vendor/libc-0.2.71:libc",
        "//third_party/cargo/vendor/objc-0.2.7:objc",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.1.4",
    crate_features = [
        "core-graphics",
        "display_link",
        "opengl",
    ],
)

