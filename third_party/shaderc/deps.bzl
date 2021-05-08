load("//third_party/shaderc:version.bzl", "shaderc_version")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Shaderc workspace rules (external repositories).
# This downloads shaderc and its dependencies, at versions specified in
# version.bzl.

def _http_archive(name, strip_prefix, gh_name, **kw):
    version, remote, sha = shaderc_version[name]
    http_archive(
        name = name,
        sha256 = sha,
        strip_prefix = (strip_prefix + remote) if strip_prefix.endswith('-') else strip_prefix,
        url = "https://github.com/{}/archive/{}.tar.gz".format(gh_name, remote),
        **kw,
    )

def shaderc_deps():
    _http_archive("spirv_headers", "SPIRV-Headers-", "KhronosGroup/SPIRV-Headers")
    _http_archive("spirv_tools", "SPIRV-Tools-", "KhronosGroup/SPIRV-Tools")
    _http_archive("glslang", "glslang-", "KhronosGroup/glslang")
    _http_archive("shaderc", "shaderc-2021.0", "google/shaderc", build_file="//third_party/shaderc:BUILD.external")
