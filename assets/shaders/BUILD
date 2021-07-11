load("//tools/bzl:glsl.bzl", "glsl_binary")

glsl_binary(
    name = "forward_vert",
    srcs = [
        "forward.vert",
    ],
    visibility = ["//engine/render:__pkg__"],
)

glsl_binary(
    name = "forward_frag",
    srcs = [
        "forward.frag",

        "forward_brdf.frag",
        "forward_defs.frag",
    ],
    visibility = ["//engine/render:__pkg__"],
)
