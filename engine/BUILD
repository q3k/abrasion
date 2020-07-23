load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")
load("@rules_pkg//:pkg.bzl", "pkg_tar")

rust_binary(
    name = "engine",
    edition = "2018",
    rustc_flags = [
        "-C", "opt-level=3",
        "-C", "overflow-checks=n",
        "-C", "force-frame-pointers=y",
    ],
    srcs = [
        "src/main.rs",
        "src/physics/mod.rs",
        "src/physics/color.rs",
        "src/render/light.rs",
        "src/render/material.rs",
        "src/render/mesh.rs",
        "src/render/mod.rs",
        "src/render/renderable.rs",
        "src/render/vulkan/data.rs",
        "src/render/vulkan/mod.rs",
        "src/render/vulkan/pipeline.rs",
        "src/render/vulkan/pipeline_forward.rs",
        "src/render/vulkan/qfi.rs",
        "src/render/vulkan/shaders.rs",
        "src/render/vulkan/surface_binding.rs",
        "src/render/vulkan/swapchain_binding.rs",
        "src/render/vulkan/worker.rs",
        "src/util/counter.rs",
        "src/util/file.rs",
        "src/util/mod.rs",
        "src/util/profiler.rs",
        "src/util/resourcemap.rs",
    ],
    deps = [
        "//third_party/cargo:cgmath",
        "//third_party/cargo:image",
        "//third_party/cargo:winit",
        "//third_party/cargo:log",
        "//third_party/cargo:env_logger",
        "//third_party/cargo:vulkano",
        "//third_party/cargo:vulkano_win",
        "@com_github_q3k_q3d//:q3d_rs",
        "@io_bazel_rules_rust//tools/runfiles",
    ],
    data = [
        "//engine/shaders:forward_vert",
        "//engine/shaders:forward_frag",
        "//assets:test-128px.png",
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
    ],
    strip_prefix = '/',
    package_dir = 'demo',
    remap_paths = {
        "engine/dist_start.sh": "start.sh",
    },
)
