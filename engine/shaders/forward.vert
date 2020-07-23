// vim: set ft=glsl:
#version 450

layout(push_constant) uniform PushConstantObject {
    mat4 view;
} pco;

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

    gl_Position = pco.view * world;
    // Vulkanize (see comment about counter-clockwise triangles in //engine/src/render/vulkan/pipeline_forward.rs).
    gl_Position.y = -gl_Position.y;
}
