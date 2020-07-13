// vim: set ft=glsl:
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(binding = 0) uniform sampler2D texSamplerDiffuse;
layout(binding = 1) uniform sampler2D texSamplerRoughness;
layout(location = 0) in vec2 fragTexCoord;
layout(location = 0) out vec4 outColor;

void main() {
    outColor = texture(texSamplerDiffuse, fragTexCoord) * texture(texSamplerRoughness, fragTexCoord).x;
}
