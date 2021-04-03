load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")
load("@rules_pkg//:pkg.bzl", "pkg_tar")

rust_binary(
    name = "engine",
    edition = "2018",
    rustc_flags = [
        "-C", "opt-level=3",
        "-C", "overflow-checks=n",
        "-C", "force-frame-pointers=y",
        "-C", "lto=off",
    ],
    srcs = [
        "src/main.rs",

        "src/physics/mod.rs",
        "src/physics/color.rs",

        "src/render/mod.rs",
        "src/render/light.rs",
        "src/render/material.rs",
        "src/render/mesh.rs",
        "src/render/renderable.rs",
        "src/render/resource.rs",

        "src/render/vulkan/mod.rs",
        "src/render/vulkan/data.rs",
        "src/render/vulkan/material.rs",
        "src/render/vulkan/pipeline.rs",
        "src/render/vulkan/pipeline_forward.rs",
        "src/render/vulkan/qfi.rs",
        "src/render/vulkan/shaders.rs",
        "src/render/vulkan/surface_binding.rs",
        "src/render/vulkan/swapchain_binding.rs",
        "src/render/vulkan/worker.rs",

        "src/util/mod.rs",
        "src/util/counter.rs",
        "src/util/file.rs",
        "src/util/profiler.rs",
        "src/util/resourcemap.rs",
    ],
    deps = [
        "//lib/ecs",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:image",
        "//third_party/cargo:winit",
        "//third_party/cargo:log",
        "//third_party/cargo:env_logger",
        "//third_party/cargo:vulkano",
        "//third_party/cargo:vulkano_win",
        "@com_github_q3k_q3d//:q3d_rs",
        "@rules_rust//tools/runfiles",
    ],
    proc_macro_deps = [
        "//lib/ecs_macros",
    ],
    data = [
        "//engine/shaders:forward_vert",
        "//engine/shaders:forward_frag",
        "//assets:test-128px.png",
        "//assets:test-128px-roughness.png",
    ],
)

pkg_tar(
    name = "demo",
    srcs = [
        ":engine",
        "dist_start.sh",
        "//engine/shaders:forward_vert",
        "//engine/shaders:forward_frag",
        "//assets:test-128px.png",
        "//assets:test-128px-roughness.png",
    ],
    strip_prefix = '/',
    package_dir = 'demo',
    remap_paths = {
        "engine/dist_start.sh": "start.sh",
    },
)
