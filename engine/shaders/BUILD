load("//tools/bzl:glsl.bzl", "glsl_binary")

glsl_binary(
    name = "forward_vert",
    srcs = [
        "forward.vert",
    ],
    visibility = ["//engine:__pkg__"],
)

glsl_binary(
    name = "forward_frag",
    srcs = [
        "forward.frag",
        "forward_brdf.frag",
        #"forward_camera.frag",
    ],
    visibility = ["//engine:__pkg__"],
)
