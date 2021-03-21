load("@rules_rust//rust:rust.bzl", "rust_library")
load("@com_github_google_flatbuffers//:build_defs.bzl", "flatbuffer_library_public")

flatbuffer_library_public(
    name = "q3d_rs_src",
    srcs = [
        ":q3d_object.fbs",
    ],
    outs = [
        "q3d_object_generated.rs",
    ],
    language_flag = "--rust",
)

rust_library(
    name = "q3d_rs",
    edition = "2018",
    srcs = [
        ":q3d_rs_src",
    ],
    visibility = ["//visibility:public"],
    deps = [
        # ew.
        "@//third_party/cargo:flatbuffers",
    ],
)
