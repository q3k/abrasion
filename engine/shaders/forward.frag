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

#version 450
#extension GL_ARB_separate_shader_objects : enable

#include "forward_defs.frag"
#include "forward_brdf.frag"

/// Camera settings.
// Aperture (in f-stops)
const float CAMERA_APERTURE = 4.0; // f/8 and be there
// Shutter speed (in seconds)
const float CAMERA_SHUTTER = 1.0 / 60; // 180° shutter at 30FPS.
// Film sensitivity ('ISO')
const float CAMERA_SENSITIVITY = 3200.0;

// Exposure Value at ISO 100, per [Ray00] equation (12).
const float CAMERA_EV_100 = log2((CAMERA_APERTURE * CAMERA_APERTURE)/CAMERA_SHUTTER);
// Exposure value at CAMERA_SENSITIVITY
const float CAMERA_EV = CAMERA_EV_100 - log2(CAMERA_SENSITIVITY / 100);
const float CAMERA_EXPOSURE = 1.0 / (pow(2.0, CAMERA_EV) * 1.2);

const mat3 XYZ_TO_SRGB = mat3(
    3.2406, -0.9689, 0.0557,
    -1.5372, 1.8758, -0.2040,
    -0.4986, 0.0415, 1.0570
);

float GammaCorrect(float v) {
    if (v <= 0.0031308) {
        return 12.92 * v;
    }
    return 1.055 * pow(v, (1.0/2.4)) - 0.055;
}


void main() {
    vec3 cameraPos = ubo.cameraPos.xyz / ubo.cameraPos.w;

    // Normal of this fragment.
    vec3 N = normalize(fragNormal);
    // Unit vector pointing at camera from this fragment.
    vec3 V = normalize(cameraPos - fragWorldPos);

    // Texture parameters for this fragment.
    vec3 albedo = texture(texSamplerDiffuse, fragTexCoord).xyz;
    float roughness = texture(texSamplerRoughness, fragTexCoord).x;
    float metallic = 0.0;
    float dielectric = 1.0 - metallic;

    // Absolute Specular Reflectance at normal incidence. Ie., the base reflectivity of a
    // material when looking straight at it.
    // Trick from https://learnopengl.com/PBR/Theory : encode the reflectivity in the albedo for
    // metallic materials (as they have no albedo). Otherwise, default to a typical reflectivity
    // for non-metallic (dielectric) materials (0.04).
    vec3 F0 = vec3(0.04);
    F0 = mix(F0, albedo, metallic);

    vec3 Lo = BRDFIlluminance(N, V, F0, albedo, dielectric, roughness);
    vec3 ambient = vec3(0.00) * albedo;
    vec3 color = XYZ_TO_SRGB * ((ambient + Lo) * CAMERA_EXPOSURE);

    outColor = vec4(GammaCorrect(color.x), GammaCorrect(color.y), GammaCorrect(color.z), 1.0);
}
