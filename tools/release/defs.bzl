def _abrasion_release_impl(ctx):
    main = ctx.files.deps[0]

    runfiles = depset([], transitive = [
        dep[DefaultInfo].default_runfiles.files for dep in ctx.attr.deps
    ]).to_list()
    # HACK: flatbuffer/ruest rules use genrules, which propagate source rust
    # files unnecessarily into their runfiles. Strip 'em out here.
    runfiles = [rf for rf in runfiles if not rf.path.endswith(".rs")]
    runfiles = [rf for rf in runfiles if not rf.path == main.path]

    # Proprietary little manifest format, for //tools/release/pack.go to use.
    runfile_manifest = ctx.actions.declare_file(ctx.attr.name + "-manifest.text.pb")
    ctx.actions.write(runfile_manifest, proto.encode_text(struct(file = [
        struct(short_path=rf.short_path, path=rf.path)
        for rf in runfiles
    ])))

    zipfile = ctx.actions.declare_file(ctx.attr.name + ".zip")

    ctx.actions.run(
        mnemonic = "AbrasionPack",
        executable = ctx.executable._pack,
        inputs = runfiles + [
            runfile_manifest,
            main
        ],
        outputs = [zipfile],
        arguments = [
            "-pack_manifest", runfile_manifest.path,
            "-pack_exe", main.path,
            "-pack_zip", zipfile.path
        ]
    )

    return [
        DefaultInfo(files=depset([zipfile]))
    ]

abrasion_release = rule(
    implementation = _abrasion_release_impl,
    attrs = {
        "deps": attr.label_list(
        ),
        "_pack": attr.label(
            executable = True,
            cfg = "exec",
            default = Label("//tools/release:pack"),
        ),
    }
)
