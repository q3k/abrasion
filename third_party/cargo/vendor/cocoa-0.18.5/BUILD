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
    name = "cocoa",
    crate_root = "src/lib.rs",
    crate_type = "rlib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/block-0.1.6:block",
        "//third_party/cargo/vendor/core-foundation-0.6.4:core_foundation",
        "//third_party/cargo/vendor/core-graphics-0.17.3:core_graphics",
        "//third_party/cargo/vendor/foreign-types-0.3.2:foreign_types",
        "//third_party/cargo/vendor/libc-0.2.67:libc",
        "//third_party/cargo/vendor/objc-0.2.7:objc",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.18.5",
    crate_features = [
    ],
)

# Unsupported target "foundation" with type "test" omitted
# Unsupported target "fullscreen" with type "example" omitted
# Unsupported target "hello_world" with type "example" omitted
# Unsupported target "tab_view" with type "example" omitted
