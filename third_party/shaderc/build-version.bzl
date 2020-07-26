load("//third_party/shaderc:version.bzl", "shaderc_version")

# Shaderc build-version.inc genrule.
# This replaces the python script from shaderc's release machinery, and instead
# generates a file with the same format using version strings defined in
# version.bzl.

def _build_version(ctx):
    v_shaderc = shaderc_version["shaderc"][0]
    v_spvtools = shaderc_version["spirv_tools"][0]
    v_glslang = shaderc_version["glslang"][0]

    versions = [
        "shaderc v{} v{}-abrasion".format(v_shaderc, v_shaderc),
        "spirv-tools v{} v{}-abrasion".format(v_spvtools, v_spvtools),
        "glslang {}-abrasion".format(v_glslang),
    ]
    out = "\n".join(['"{}\\n"'.format(v) for v in versions])
    ctx.actions.write(ctx.outputs.inc, out + "\n")

build_version = rule(
    implementation = _build_version,
    outputs = {
        "inc": "%{name}.inc",
    },
)
