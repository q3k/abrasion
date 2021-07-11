// Forward rendering fragment shader.
//
// Copyright 2020 Sergiusz 'q3k' Bazanski <q3k@q3k.org>
//
// This file is part of Abrasion.
//
// Abrasion is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, version 3.
//
// Abrasion is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// Abrasion.  If not, see <https://www.gnu.org/licenses/>.
//
// vim: set ft=glsl:

#ifndef _FORWARD_DEFS_FRAG_
#define _FORWARD_DEFS_FRAG_

const float PI = 3.14159;

struct OmniLight {
    vec4 pos;
    vec4 color;
};

layout(binding = 0) uniform FragmentUniformBufferObject {
    vec4 cameraPos;
    OmniLight omniLights[4];
} ubo;
layout(binding = 1) uniform sampler2D texSamplerDiffuse;
layout(binding = 2) uniform sampler2D texSamplerRoughness;

layout(location = 0) in vec2 fragTexCoord;
layout(location = 1) in vec3 fragWorldPos;
layout(location = 2) in vec3 fragNormal;

layout(location = 0) out vec4 outColor;

#endif
