load("@rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "engine",
    edition = "2018",
    rustc_flags = [
        "-C", "opt-level=3",
        "-C", "overflow-checks=n",
        "-C", "force-frame-pointers=y",
        "-C", "lto=off",
        "-Zpolonius",
        "-Zborrowck=mir",
    ],
    srcs = [
        "lib.rs",

        "globals.rs",
        "scripting.rs",
    ],
    deps = [
        "//engine/util",
        "//engine/physics",
        "//engine/render",
        "//lib/ecs",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:image",
        "//third_party/cargo:mlua",
        "//third_party/cargo:log",
        "//third_party/cargo:env_logger",
    ],
    proc_macro_deps = [
        "//lib/ecs_macros",
    ],
    data = [
        "//assets:test-128px.png",
        "//assets:test-128px-roughness.png",
        "//assets/shaders:forward_vert",
        "//assets/shaders:forward_frag",
    ] + glob(["lua/**"]),
    visibility = [
        "//visibility:public",
    ],
)
