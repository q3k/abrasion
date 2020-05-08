// vim: set ft=glsl:
#version 450

layout(push_constant) uniform UniformBufferObject {
    mat4 view;
} ubo;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in mat4 model;
layout(location = 6) in vec2 tex;

layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec2 fragTexCoord;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    gl_Position = ubo.view * model * vec4(pos, 1.0);
    fragColor = color;

    fragTexCoord = tex;

    // Vulkanize
    gl_Position.y = -gl_Position.y;
}
