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
    name = "crossbeam_epoch",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/cfg-if-0.1.10:cfg_if",
        "//third_party/cargo/vendor/crossbeam-utils-0.7.2:crossbeam_utils",
        "//third_party/cargo/vendor/lazy_static-1.4.0:lazy_static",
        "//third_party/cargo/vendor/maybe-uninit-2.0.0:maybe_uninit",
        "//third_party/cargo/vendor/memoffset-0.5.4:memoffset",
        "//third_party/cargo/vendor/scopeguard-1.1.0:scopeguard",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.8.2",
    crate_features = [
        "default",
        "lazy_static",
        "std",
    ],
)

# Unsupported target "defer" with type "bench" omitted
# Unsupported target "flush" with type "bench" omitted
# Unsupported target "pin" with type "bench" omitted
# Unsupported target "sanitize" with type "example" omitted
# Unsupported target "treiber_stack" with type "example" omitted
