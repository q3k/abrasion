load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "engine",
    edition = "2018",
    srcs = [
        "src/main.rs",
        "src/render/mod.rs",
        "src/render/vulkan/mod.rs",
        "src/render/vulkan/binding.rs",
        "src/render/vulkan/qfi.rs",
        "src/render/vulkan/shaders.rs",
        "src/render/vulkan/swapchains.rs",
        "src/util/mod.rs",
        "src/util/counter.rs",
    ],
    deps = [
        "//third_party/cargo:winit",
        "//third_party/cargo:log",
        "//third_party/cargo:env_logger",
        "//third_party/cargo:vulkano",
        "//third_party/cargo:vulkano_win",
        "@io_bazel_rules_rust//tools/runfiles",
    ],
    data = [
        "//engine/shaders:forward_vert",
        "//engine/shaders:forward_frag",
    ],
)
