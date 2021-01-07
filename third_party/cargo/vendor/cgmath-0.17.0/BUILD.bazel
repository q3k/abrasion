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
  "notice", # "Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_binary(
    name = "cgmath_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2015",
    deps = [
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
    ],
    data = glob(["*"]),
    version = "0.17.0",
    visibility = ["//visibility:private"],
)

genrule(
    name = "cgmath_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["cgmath_out_dir_outputs.tar.gz"],
    tools = [
      ":cgmath_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/cgmath_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/cgmath_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :cgmath_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "angle" with type "test" omitted

rust_library(
    name = "cgmath",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/approx-0.3.2:approx",
        "//third_party/cargo/vendor/num-traits-0.2.11:num_traits",
        "//third_party/cargo/vendor/rand-0.6.5:rand",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    out_dir_tar = ":cgmath_build_script_executor",
    version = "0.17.0",
    crate_features = [
    ],
)

# Unsupported target "construction" with type "bench" omitted
# Unsupported target "mat" with type "bench" omitted
# Unsupported target "matrix" with type "test" omitted
# Unsupported target "point" with type "test" omitted
# Unsupported target "projection" with type "test" omitted
# Unsupported target "quat" with type "bench" omitted
# Unsupported target "quaternion" with type "test" omitted
# Unsupported target "rotation" with type "test" omitted
# Unsupported target "swizzle" with type "test" omitted
# Unsupported target "transform" with type "test" omitted
# Unsupported target "vec" with type "bench" omitted
# Unsupported target "vector" with type "test" omitted
# Unsupported target "vector4f32" with type "test" omitted
