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
    name = "wayland_scanner",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/proc-macro2-0.4.30:proc_macro2",
        "//third_party/cargo/vendor/quote-0.6.13:quote",
        "//third_party/cargo/vendor/xml-rs-0.8.0:xml_rs",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.23.6",
    crate_features = [
    ],
)

