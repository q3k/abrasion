// vim: set ft=glsl:
#version 450

struct OmniLight {
    vec3 pos;
    vec3 color;
};

layout(push_constant) uniform UniformBufferObject {
    mat4 view;
    vec3 cameraPos;
    OmniLight omniLights[2];
} ubo;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 normal;
layout(location = 2) in mat4 model;
layout(location = 6) in vec2 tex;

layout(location = 0) out vec2 fragTexCoord;
layout(location = 1) out vec3 fragWorldPos;
layout(location = 2) out vec3 fragNormal;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    vec4 world = model * vec4(pos, 1.0);

    fragTexCoord = tex;
    fragNormal = normal;
    fragWorldPos = world.xyz / world.w;

    gl_Position = ubo.view * world;
    // Vulkanize
    gl_Position.y = -gl_Position.y;
}
