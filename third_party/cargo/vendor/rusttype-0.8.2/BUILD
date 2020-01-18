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
    name = "rusttype",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/approx-0.3.2:approx",
        "//third_party/cargo/vendor/arrayvec-0.5.1:arrayvec",
        "//third_party/cargo/vendor/ordered-float-1.0.2:ordered_float",
        "//third_party/cargo/vendor/stb_truetype-0.3.1:stb_truetype",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.8.2",
    crate_features = [
        "default",
        "has-atomics",
        "stb_truetype",
        "std",
    ],
)

