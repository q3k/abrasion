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


# Unsupported target "compositor_info" with type "example" omitted
# Unsupported target "image_viewer" with type "example" omitted
# Unsupported target "kbd_input" with type "example" omitted
# Unsupported target "pointer_input" with type "example" omitted
# Unsupported target "selection" with type "example" omitted

rust_library(
    name = "smithay_client_toolkit",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/andrew-0.2.1:andrew",
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/dlib-0.4.1:dlib",
        "//third_party/cargo/vendor/lazy_static-1.4.0:lazy_static",
        "//third_party/cargo/vendor/memmap-0.7.0:memmap",
        "//third_party/cargo/vendor/nix-0.14.1:nix",
        "//third_party/cargo/vendor/wayland-client-0.21.13:wayland_client",
        "//third_party/cargo/vendor/wayland-commons-0.21.13:wayland_commons",
        "//third_party/cargo/vendor/wayland-protocols-0.21.13:wayland_protocols",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.6",
    crate_features = [
    ],
)

