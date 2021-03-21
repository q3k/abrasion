load("@rules_rust//rust:rust.bzl", "rust_test", "rust_library")

rust_library(
    name = "ecs",
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
    visibility = ["//engine:__subpackages__"],
)

rust_test(
    name = "ecs_test",
    crate = ":ecs",
)
