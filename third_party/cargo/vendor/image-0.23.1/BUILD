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


# Unsupported target "BMPEncoder" with type "bench" omitted
# Unsupported target "encode_jpeg" with type "bench" omitted

rust_library(
    name = "image",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bytemuck-1.2.0:bytemuck",
        "//third_party/cargo/vendor/byteorder-1.3.4:byteorder",
        "//third_party/cargo/vendor/gif-0.10.3:gif",
        "//third_party/cargo/vendor/jpeg-decoder-0.1.18:jpeg_decoder",
        "//third_party/cargo/vendor/num-iter-0.1.40:num_iter",
        "//third_party/cargo/vendor/num-rational-0.2.3:num_rational",
        "//third_party/cargo/vendor/num-traits-0.2.11:num_traits",
        "//third_party/cargo/vendor/png-0.16.1:png",
        "//third_party/cargo/vendor/scoped_threadpool-0.1.9:scoped_threadpool",
        "//third_party/cargo/vendor/tiff-0.4.0:tiff",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.23.1",
    crate_features = [
        "bmp",
        "dds",
        "default",
        "dxt",
        "gif",
        "hdr",
        "ico",
        "jpeg",
        "jpeg_rayon",
        "png",
        "pnm",
        "scoped_threadpool",
        "tga",
        "tiff",
        "webp",
    ],
)

# Unsupported target "load" with type "bench" omitted
