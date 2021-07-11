load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")
load("@rules_pkg//:pkg.bzl", "pkg_tar")

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
        "input.rs",
        "scripting.rs",

        "physics/mod.rs",
        "physics/color.rs",

        "render/mod.rs",
        "render/light.rs",
        "render/material.rs",
        "render/mesh.rs",
        "render/renderable.rs",
        "render/resource.rs",

        "render/vulkan/mod.rs",
        "render/vulkan/data.rs",
        "render/vulkan/material.rs",
        "render/vulkan/pipeline.rs",
        "render/vulkan/pipeline_forward.rs",
        "render/vulkan/qfi.rs",
        "render/vulkan/shaders.rs",
        "render/vulkan/surface_binding.rs",
        "render/vulkan/swapchain_binding.rs",
        "render/vulkan/worker.rs",

        "util/mod.rs",
        "util/counter.rs",
        "util/file.rs",
        "util/profiler.rs",
        "util/resourcemap.rs",
    ],
    deps = [
        "//lib/ecs",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:image",
        "//third_party/cargo:mlua",
        "//third_party/cargo:winit",
        "//third_party/cargo:log",
        "//third_party/cargo:env_logger",
        "//third_party/cargo:vulkano",
        "//third_party/cargo:vulkano_win",
        "@rules_rust//tools/runfiles",
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
