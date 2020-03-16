workspace(name = "abrasion")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_rust",
    # master HEAD as of 2020/03/16
    sha256 = "3f6db529492d821a91560c230e2634e34b3e0a3147fc5c4c41ac5bc6ccd45d3f",
    strip_prefix = "rules_rust-fe50d3b54aecbaeac48abdc2ca7cd00a94969e15",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/fe50d3b54aecbaeac48abdc2ca7cd00a94969e15.tar.gz",
    ],
)

http_archive(
    name = "bazel_skylib",
    sha256 = "9a737999532daca978a158f94e77e9af6a6a169709c0cee274f0a4c3359519bd",
    strip_prefix = "bazel-skylib-1.0.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/1.0.0.tar.gz",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repository_set")
rust_repository_set(
    name = "rust_linux_x86_64",
    exec_triple = "x86_64-unknown-linux-gnu",
    extra_target_triples = [],
    version = "nightly",
    # Any newer and you get::
    # thread 'main' panicked at 'attempted to leave type `std::mem::ManuallyDrop<xlib_xcb::Xlib_xcb>` uninitialized, which is invalid'
    iso_date = "2020-03-11",
)

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

http_archive(
    name = "glslang",
    sha256 = "0b79a120ac7826ac31bcd47ba3e11d79d8d2709bfaf17125aa759eeec6799dd3",
    strip_prefix = "glslang-b0ada80356ca7b560c600b93a596af1331442542",
    url = "https://github.com/KhronosGroup/glslang/archive/b0ada80356ca7b560c600b93a596af1331442542.tar.gz",
)

http_archive(
    name = "rules_pkg",
    urls = [
        "https://github.com/bazelbuild/rules_pkg/releases/download/0.2.5/rules_pkg-0.2.5.tar.gz",
        "https://mirror.bazel.build/github.com/bazelbuild/rules_pkg/releases/download/0.2.5/rules_pkg-0.2.5.tar.gz",
    ],
    sha256 = "352c090cc3d3f9a6b4e676cf42a6047c16824959b438895a76c2989c6d7c246a",
)
load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")
rules_pkg_dependencies()
