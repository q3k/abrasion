// vim: set ft=glsl:
#version 450

layout(push_constant) uniform UniformBufferObject {
    mat4 model;
} ubo;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;

layout(location = 0) out vec3 fragColor;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    gl_Position = ubo.model * vec4(pos, 1.0);
    fragColor = color;

    // Vulkanize
    gl_Position.y = -gl_Position.y;
}
