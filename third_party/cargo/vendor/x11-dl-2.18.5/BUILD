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
    name = "x11_dl_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2015",
    deps = [
        "//third_party/cargo/vendor/pkg-config-0.3.17:pkg_config",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
    ],
    data = glob(["*"]),
    version = "2.18.5",
    visibility = ["//visibility:private"],
)

genrule(
    name = "x11_dl_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["x11_dl_out_dir_outputs.tar.gz"],
    tools = [
      ":x11_dl_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/x11_dl_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/x11_dl_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :x11_dl_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "hello-world-dl" with type "example" omitted

rust_library(
    name = "x11_dl",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/lazy_static-1.4.0:lazy_static",
        "//third_party/cargo/vendor/libc-0.2.67:libc",
        "//third_party/cargo/vendor/maybe-uninit-2.0.0:maybe_uninit",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    out_dir_tar = ":x11_dl_build_script_executor",
    version = "2.18.5",
    crate_features = [
    ],
)

