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
    name = "wayland_protocols_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2015",
    deps = [
        "//third_party/cargo/vendor/wayland-scanner-0.23.6:wayland_scanner",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
      "client",
      "unstable_protocols",
      "wayland-client",
    ],
    data = glob(["*"]),
    version = "0.23.6",
    visibility = ["//visibility:private"],
)

genrule(
    name = "wayland_protocols_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["wayland_protocols_out_dir_outputs.tar.gz"],
    tools = [
      ":wayland_protocols_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/wayland_protocols_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_CLIENT=1;"
        + " export CARGO_FEATURE_UNSTABLE_PROTOCOLS=1;"
        + " export CARGO_FEATURE_WAYLAND_CLIENT=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/wayland_protocols_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :wayland_protocols_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)


rust_library(
    name = "wayland_protocols",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/wayland-client-0.23.6:wayland_client",
        "//third_party/cargo/vendor/wayland-commons-0.23.6:wayland_commons",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    out_dir_tar = ":wayland_protocols_build_script_executor",
    version = "0.23.6",
    crate_features = [
        "client",
        "unstable_protocols",
        "wayland-client",
    ],
)

