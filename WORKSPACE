workspace(name = "abrasion")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_rust",
    # master HEAD as of 2020/01/19
    sha256 = "66ea4cb3296016234143511e8ce3435b5f186a217a84c251c31d04dc10ca1807",
    strip_prefix = "rules_rust-a9103cd6260433fb04b36d9a3e1dc4d3ddceaa22",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/a9103cd6260433fb04b36d9a3e1dc4d3ddceaa22.tar.gz",
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
    iso_date = "2020-01-17",
)

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")
