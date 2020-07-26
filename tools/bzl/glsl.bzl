def _glsl_binary(ctx):
    srcs = ctx.files.srcs
    binary = ctx.outputs.binary
    compiler = ctx.executable._compiler

    main = srcs[0].path

    args = [main, "-o", binary.path]

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
            default=Label("@shaderc//:glslc"),
            allow_single_file=True,
            executable=True,
            cfg="host",
        ),
    },
    outputs = {
        "binary": "%{name}.spv",
    },
)
