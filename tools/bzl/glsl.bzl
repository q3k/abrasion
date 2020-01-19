def _glsl_binary(ctx):
    srcs = ctx.files.srcs
    binary = ctx.outputs.binary
    compiler = ctx.executable._compiler

    args = ["-V", "-o", binary.path] + [s.path for s in srcs]

    ctx.actions.run(
        inputs=srcs,
        outputs=[binary],
        executable=compiler,
        arguments=args,
        mnemonic="glslc",
        progress_message='Compiling shader {}'.format(binary.short_path)
    )

glsl_binary = rule(
    implementation = _glsl_binary,
    attrs = {
        "srcs": attr.label_list(
            allow_files=True,
        ),
        "_compiler": attr.label(
            default=Label("@glslang//:glslangValidator"),
            allow_single_file=True,
            executable=True,
            cfg="host",
        ),
    },
    outputs = {
        "binary": "%{name}.spv",
    },
)
