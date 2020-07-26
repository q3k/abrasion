# Shaderc version definitions.
# This should mirror DEPS from github.com/google/shaderc, but you will need two
# extra things that aren't there:
#  - the sha256 of the tarball
#  - the pretty 'release' version of the tool (which you can get from the changelog)

shaderc_version = {
    # dep: (version, GH remote ver, sha256)
    "shaderc": ("2020.2", "2020.2", "a4d5680d4f0199e29ab77b357c88c147c5704f9ee2ac0a2d117e640e6f87d030"),
    "spirv_tools": ("2020.5", "969f0286479b89267b6c89f6d5223285c265e6ae", "6915b4bca8b2369e26812ffb3c1b726089bb9cd38d1cb7e9b8ef0d3bb7dd8162"),
    "spirv_headers": ("2020.5", "979924c8bc839e4cb1b69d03d48398551f369ce7", "7ebc04ebb4602c051d4886c9544de684195adac2a179e949469d29beb1f034e4"),
    "glslang": ("11.0.0-dev", "3ee5f2f1d3316e228916788b300d786bb574d337", "7b2f8b93958c7594942f730659c00dec0bffeafaa6853b67b5f72f915c287b1f"),
}
