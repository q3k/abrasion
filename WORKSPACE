workspace(name = "abrasion")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "71bcdc2d56adf2cf71e53ad453ae19757d2d52704c2f598b73339fe9b9326fcf",
    strip_prefix = "rules_rust-92f56af72f2929089d00116c12d5d6bc029d839d",
    urls = [
        # Need https://github.com/bazelbuild/rules_rust/pull/285 for rusttype
        "https://github.com/GregBowyer/rules_rust/archive/92f56af72f2929089d00116c12d5d6bc029d839d.tar.gz",
    ],
)

http_archive(
    name = "bazel_skylib",
    sha256 = "9a737999532daca978a158f94e77e9af6a6a169709c0cee274f0a4c3359519bd",
    strip_prefix = "bazel-skylib-1.0.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/1.0.0.tar.gz",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repository_set")
#rust_repositories()
rust_repository_set(
    name = "rust_linux_x86_64",
    exec_triple = "x86_64-unknown-linux-gnu",
    extra_target_triples = [],
    version = "nightly",
    iso_date = "2020-01-17",
)

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")
