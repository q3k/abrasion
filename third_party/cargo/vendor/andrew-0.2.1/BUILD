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
    name = "andrew",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/line_drawing-0.7.0:line_drawing",
        "//third_party/cargo/vendor/walkdir-2.3.1:walkdir",
        "//third_party/cargo/vendor/xdg-2.2.0:xdg",
        "//third_party/cargo/vendor/xml-rs-0.8.0:xml_rs",
        "//third_party/cargo/vendor/rusttype-0.8.2:rusttype",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.2.1",
    crate_features = [
    ],
)

# Unsupported target "test" with type "example" omitted
