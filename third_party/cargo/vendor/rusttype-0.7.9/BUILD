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


# Unsupported target "cache" with type "bench" omitted
# Unsupported target "draw" with type "bench" omitted
# Unsupported target "gpu_cache" with type "example" omitted
# Unsupported target "image" with type "example" omitted
# Unsupported target "issues" with type "test" omitted
# Unsupported target "render_reference" with type "test" omitted

rust_library(
    name = "rusttype",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/rusttype-0.8.2:rusttype",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.7.9",
    crate_features = [
    ],
)

# Unsupported target "simple" with type "example" omitted
