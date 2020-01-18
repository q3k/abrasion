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

rust_binary(
    name = "wayland_client_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2015",
    deps = [
        "//third_party/cargo/vendor/wayland-scanner-0.21.13:wayland_scanner",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
      "cursor",
      "dlopen",
      "egl",
      "native_lib",
      "wayland-commons",
      "wayland-sys",
    ],
    data = glob(["*"]),
    version = "0.21.13",
    visibility = ["//visibility:private"],
)

genrule(
    name = "wayland_client_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["wayland_client_out_dir_outputs.tar.gz"],
    tools = [
      ":wayland_client_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/wayland_client_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_CURSOR=1;"
        + " export CARGO_FEATURE_DLOPEN=1;"
        + " export CARGO_FEATURE_EGL=1;"
        + " export CARGO_FEATURE_NATIVE_LIB=1;"
        + " export CARGO_FEATURE_WAYLAND_COMMONS=1;"
        + " export CARGO_FEATURE_WAYLAND_SYS=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/wayland_client_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :wayland_client_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "dynamic_globals" with type "example" omitted
# Unsupported target "list_globals" with type "example" omitted
# Unsupported target "simple_window" with type "example" omitted

rust_library(
    name = "wayland_client",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/downcast-rs-1.1.1:downcast_rs",
        "//third_party/cargo/vendor/libc-0.2.66:libc",
        "//third_party/cargo/vendor/nix-0.14.1:nix",
        "//third_party/cargo/vendor/wayland-commons-0.21.13:wayland_commons",
        "//third_party/cargo/vendor/wayland-sys-0.21.13:wayland_sys",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    out_dir_tar = ":wayland_client_build_script_executor",
    version = "0.21.13",
    crate_features = [
        "cursor",
        "dlopen",
        "egl",
        "native_lib",
        "wayland-commons",
        "wayland-sys",
    ],
)

