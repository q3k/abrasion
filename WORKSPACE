workspace(name = "abrasion")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_python",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.3.0/rules_python-0.3.0.tar.gz",
    sha256 = "934c9ceb552e84577b0faf1e5a2f0450314985b4d8712b2b70717dc679fdc01b",
)

load("@rules_python//python:pip.bzl", "pip_parse")

pip_parse(
    name = "pydeps",
    requirements_lock = "//third_party/py:requirements.txt",
    python_interpreter = "python",
)

load("@pydeps//:requirements.bzl", "install_deps")
install_deps()
http_archive(
    name = "rules_rust",
    strip_prefix = "rules_rust-feaeb7ab712da45c1c94f7950e799d79e367ddeb",
    sha256 = "4f7c843aae20fd50832252c85c9d5f350e38a40c5dea7593555e579952b8d449",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/feaeb7ab712da45c1c94f7950e799d79e367ddeb.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories(
    version = "nightly",
    iso_date = "2020-12-31",
    edition = "2018",
)

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
    patches = [
        "//third_party/flatbuffers:bashless.diff",
    ],
    patch_args = ["-p1"],
)

http_archive(
    name = "com_github_q3k_q3d",
    sha256 = "6cfbbd8111e78ae99d631f147cc2314f088409320619660c8003cd52bcc4577e",
    strip_prefix = "q3d-e507a91e1b4966d0d4425e6078f6f1feda68a2b5",
    urls = [
        "https://github.com/q3k/q3d/archive/e507a91e1b4966d0d4425e6078f6f1feda68a2b5.tar.gz",
    ],
    build_file = "//third_party/q3d:BUILD.external",
)

http_archive(
    name = "io_bazel_rules_go",
    sha256 = "69de5c704a05ff37862f7e0f5534d4f479418afc21806c887db544a316f3cb6b",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.27.0/rules_go-v0.27.0.tar.gz",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.27.0/rules_go-v0.27.0.tar.gz",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.16")

http_archive(
    name = "com_google_protobuf",
    sha256 = "9748c0d90e54ea09e5e75fb7fac16edce15d2028d4356f32211cfa3c0e956564",
    strip_prefix = "protobuf-3.11.4",
    urls = ["https://github.com/protocolbuffers/protobuf/archive/v3.11.4.zip"],
)

load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")

protobuf_deps()
