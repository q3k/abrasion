"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//third_party/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "control_flow" with type "example" omitted
# Unsupported target "cursor" with type "example" omitted
# Unsupported target "cursor_grab" with type "example" omitted
# Unsupported target "custom_events" with type "example" omitted
# Unsupported target "fullscreen" with type "example" omitted
# Unsupported target "handling_close" with type "example" omitted
# Unsupported target "min_max_size" with type "example" omitted
# Unsupported target "minimize" with type "example" omitted
# Unsupported target "monitor_list" with type "example" omitted
# Unsupported target "multithreaded" with type "example" omitted
# Unsupported target "multiwindow" with type "example" omitted
# Unsupported target "request_redraw" with type "example" omitted
# Unsupported target "request_redraw_threaded" with type "example" omitted
# Unsupported target "resizable" with type "example" omitted
# Unsupported target "send_objects" with type "test" omitted
# Unsupported target "serde_objects" with type "test" omitted
# Unsupported target "sync_object" with type "test" omitted
# Unsupported target "timer" with type "example" omitted
# Unsupported target "transparent" with type "example" omitted
# Unsupported target "video_modes" with type "example" omitted
# Unsupported target "web" with type "example" omitted
# Unsupported target "window" with type "example" omitted
# Unsupported target "window_debug" with type "example" omitted
# Unsupported target "window_icon" with type "example" omitted
# Unsupported target "window_run_return" with type "example" omitted

rust_library(
    name = "winit",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//third_party/cargo/vendor/bitflags-1.2.1:bitflags",
        "//third_party/cargo/vendor/instant-0.1.4:instant",
        "//third_party/cargo/vendor/lazy_static-1.4.0:lazy_static",
        "//third_party/cargo/vendor/libc-0.2.71:libc",
        "//third_party/cargo/vendor/log-0.4.8:log",
        "//third_party/cargo/vendor/mio-0.6.22:mio",
        "//third_party/cargo/vendor/mio-extras-2.0.6:mio_extras",
        "//third_party/cargo/vendor/parking_lot-0.10.2:parking_lot",
        "//third_party/cargo/vendor/percent-encoding-2.1.0:percent_encoding",
        "//third_party/cargo/vendor/raw-window-handle-0.3.3:raw_window_handle",
        "//third_party/cargo/vendor/smithay-client-toolkit-0.6.6:smithay_client_toolkit",
        "//third_party/cargo/vendor/wayland-client-0.23.6:wayland_client",
        "//third_party/cargo/vendor/x11-dl-2.18.5:x11_dl",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.22.2",
    crate_features = [
    ],
)

