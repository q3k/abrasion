# _prepend_workspace from github.com/google/subpar.
# Copyright 2016 Google Inc. All Rights Reserved.
# Licensed under the Apache License, Version 2.0, see
# //third_party/licenses/APACHE-2.0.txt.
def _prepend_workspace(path, ctx):
    # It feels like there should be an easier, less fragile way.
    if path.startswith("../"):
        # External workspace, for example
        # '../protobuf/python/google/protobuf/any_pb2.py'
        stored_path = path[len("../"):]
    elif path.startswith("external/"):
        # External workspace, for example
        # 'external/protobuf/python/__init__.py'
        stored_path = path[len("external/"):]
    else:
        # Main workspace, for example 'mypackage/main.py'
        stored_path = ctx.workspace_name + "/" + path
    return stored_path

def _generate_subdirs(directory):
    parts = directory.split('/')
    cur = parts[0]
    res = []
    for p in parts[1:]:
        res.append(cur)
        cur += '/' + p
    res.append(cur)
    return res

def _blender_addon_impl(ctx):
    root = ctx.attr.name + "/"

    out = []
    importpaths = []
    directories = {}
    initpys = {}
    for dep in ctx.attr.deps:
        for s in dep[PyInfo].transitive_sources.to_list():
            p = _prepend_workspace(s.short_path, ctx)
            f = ctx.actions.declare_file(root + p)
            ctx.actions.symlink(output=f, target_file=s)
            out.append(f)

            directory = '/'.join(p.split('/')[:-1])
            directories[directory] = True

            if p.endswith('/__init__.py'):
                initpys[p] = True

        importpaths.append(dep[PyInfo].imports)
    importpaths = depset([], transitive=importpaths).to_list()

    need_initpys = {}
    for d in directories.keys():
        for subdir in _generate_subdirs(d)[::-1]:
            if subdir in importpaths:
                break
            ipy = subdir + '/__init__.py'
            if initpys.get(ipy):
                continue
            need_initpys[ipy] = True

    for ipy in need_initpys.keys():
        f = ctx.actions.declare_file(root + ipy)
        ctx.actions.write(f, "")
        out.append(f)

    addon_name = ctx.attr.addon_name or ctx.attr.name

    initfile = ctx.actions.declare_file(root + "__init__.py")
    ctx.actions.expand_template(
        template = ctx.file._initfile_template,
        output = initfile,
        substitutions = {
            '%importpaths%': json.encode(importpaths),
            '%module%': ctx.attr.module,
            '%name%': addon_name,
            '%location%': ctx.attr.addon_location,
            '%category%': ctx.attr.addon_category,
        },
    )
    out.append(initfile)

    return [
        DefaultInfo(
            files = depset(out),
            runfiles = ctx.runfiles(out),
        )
    ]

blender_addon = rule(
    implementation = _blender_addon_impl,
    attrs = {
        'deps': attr.label_list(
            providers = [PyInfo],
        ),
        'module': attr.string(
            mandatory = True,
        ),
        'addon_name': attr.string(),
        'addon_location': attr.string(
            default = '',
        ),
        'addon_category': attr.string(
            default = 'Import-Export',
        ),

        '_initfile_template': attr.label(
            default = Label('//blender/build:__inittmpl__.py'),
            allow_single_file = True,
        ),
    },
)
