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


# Unsupported target "argument-buffer" with type "example" omitted
# Unsupported target "bind" with type "example" omitted
# Unsupported target "caps" with type "example" omitted
# Unsupported target "compute" with type "example" omitted
# Unsupported target "compute-argument-buffer" with type "example" omitted
# Unsupported target "embedded-lib" with type "example" omitted
# Unsupported target "library" with type "example" omitted

rust_library(
    name = "metal",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/block-0.1.6:block",
        "//third_party/cargo/vendor/cocoa-0.18.5:cocoa",
        "//third_party/cargo/vendor/core-graphics-0.17.3:core_graphics",
        "//third_party/cargo/vendor/foreign-types-0.3.2:foreign_types",
        "//third_party/cargo/vendor/libc-0.2.67:libc",
        "//third_party/cargo/vendor/log-0.4.8:log",
        "//third_party/cargo/vendor/objc-0.2.7:objc",
        "//third_party/cargo/vendor/objc-foundation-0.1.1:objc_foundation",
        "//third_party/cargo/vendor/objc_id-0.1.1:objc_id",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.13.1",
    crate_features = [
        "default",
    ],
)

# Unsupported target "reflection" with type "example" omitted
# Unsupported target "window" with type "example" omitted
