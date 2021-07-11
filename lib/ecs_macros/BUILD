load("@rules_rust//rust:defs.bzl", "rust_proc_macro")

rust_proc_macro(
    name = "ecs_macros",
    edition = "2018",
    srcs = [
        "src/lib.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//third_party/cargo:proc_macro2",
        "//third_party/cargo:quote",
        "//third_party/cargo:syn",
    ],
)
