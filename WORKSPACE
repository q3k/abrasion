workspace(name = "abrasion")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    # master HEAD as of 2021/03/21
    sha256 = "94982f79b84c1d8a8ddf73293044819390116bb97f93c63308a3877a858fec63",
    strip_prefix = "rules_rust-73cb9389700cef4a863526820dd9169e0c3e1e25",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/73cb9389700cef4a863526820dd9169e0c3e1e25.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("//third_party/shaderc:deps.bzl", "shaderc_deps")
shaderc_deps()

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

http_archive(
    name = "com_github_google_flatbuffers",
    sha256 = "1c1b7ae5bf4763f2fabc42002c4cfa70160b79ec33cac8cc59d2d5ab83ffe260",
    strip_prefix = "flatbuffers-ac203b20926b13a35ff85277d2e5d3c38698eee8",
    urls = [
        "https://github.com/google/flatbuffers/archive/ac203b20926b13a35ff85277d2e5d3c38698eee8.tar.gz",
    ],
)

http_archive(
    name = "com_github_q3k_q3d",
    sha256 = "7631310022b09447279ac227cf84045b8b552f9c863d6fe17d459e506058a9b7",
    strip_prefix = "q3d-360206ac7487da4a6d86fd22f9e74e8731454f43",
    urls = [
        "https://github.com/q3k/q3d/archive/360206ac7487da4a6d86fd22f9e74e8731454f43.tar.gz",
    ],
    build_file = "//third_party/q3d:BUILD",
)
