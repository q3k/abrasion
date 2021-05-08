# Shaderc version definitions.
# This should mirror DEPS from github.com/google/shaderc, but you will need two
# extra things that aren't there:
#  - the sha256 of the tarball
#  - the pretty 'release' version of the tool (which you can get from the changelog)

shaderc_version = {
    # dep: (version, GH remote ver, sha256)
    "shaderc": ("2021.0", "v2021.0", "99762c51b0ceddea4be9b8de3240a7dbf1de9131179eed94b568ce52a5496a8f"),
    "spirv_tools": ("2021.1", "c2d5375fa7cc87c93f692e7200d5d974283d4391", "f60bdef464ffb7cba297675cc485d4fef0b1718968f0890e0639e2fb68600465"),
    "spirv_headers": ("2021.1", "dafead1765f6c1a5f9f8a76387dcb2abe4e54acd", "e1c8530c95fc8c70fa6a7cbc269ebd1b10da3872efa0e3c6eb82452c3e180cda"),
    "glslang": ("11.3.0-dev", "60ce877de03ff03bb87fb9ef6b744ee33d076034", "f9a0188d796e9e4fbc59f2543b6f87e9f8423933b4599a0fc2b9f0216aace26e"),
}
