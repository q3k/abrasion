load("@rules_rust//rust:rust.bzl", "rust_test", "rust_library")

rust_library(
    name = "ecs",
    edition = "2018",
    srcs = [
        "src/lib.rs",

        "src/borrow.rs",
        "src/component.rs",
        "src/componentmap.rs",
        "src/globalmap.rs",
        "src/entity.rs",
        "src/system.rs",
        "src/world.rs",
    ],
    deps = [
        "//third_party/cargo:log",
    ],
    visibility = ["//engine:__subpackages__"],
)

rust_test(
    name = "ecs_test",
    crate = ":ecs",
)
