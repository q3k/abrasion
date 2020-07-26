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

// We implement a Lambertiand & Cook-Torrance BRDF-based lighting system.
// All of this is based on a number of scientific papers, meta-studies and modern sources. We do
// our best to cite as much as possible for future reference.
// Most of the maths is used straight from [Kar13].
//
// A good summary of different research is available this blog post by Brian Karis, that attempts
// to catalogue all existing BRDF-related functions:
// http://graphicrants.blogspot.com/2013/08/specular-brdf-reference.html
//
/// Bibliography:
//
// [Bec63]
// P. Beckmann & A. Spizzichino. 1963. "The Scattering of Electromagnetic Waves from Rough Surfaces"
// MacMillan, New York
//
// [Smi67]
// Bruce Smith. 1967. "Geometrical shadowing of a random rough surface."
// IEEE transactions on antennas and propagation 15.5 (1967): 668-671.
//
// [CT82]
// Robert L. Cook, Kenneth E. Torrance. 1982. "A Reflectance Model for Computer Graphics"
// ACM Transactions on Graphics, 1(1), 7–24.
// doi: 10.1145/357290.357293
//
// [Sch94]
// Christophe Schlick. 1994. "An Inexpensive BRDF Model for Physically-based Rendering"
// Computer Graphics Forum, 13(3), 233–246.
// doi: 10.1111/1467-8659.1330233
//
// [Wa07]
// Bruce Walter et al. 2007. "Microfacet Models for Refraction through Rough Surfaces."
// Proceedings of the Eurographics Symposium on Rendering.
//
// [Bur12]
// Brent Burley. 2012. "Physically-Based Shading at Disney"
// URL: https://disney-animation.s3.amazonaws.com/library/s2012_pbs_disney_brdf_notes_v2.pdf
//
// [Kar13]
// Brian Karis. 2013. "Real Shading in Unreal Engine 4"
// URL: https://blog.selfshadow.com/publications/s2013-shading-course/karis/s2013_pbs_epic_notes_v2.pdf
//
// [Hei14]
// Eric Heitz. 2014. "Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs"
// Journal of Computer Graphics Techniques, 3 (2).
//
// [GA19]
// Romain Guy, Mathias Agopian, "Physically Based Rendering in Filament"
// URL: https://google.github.io/filament/Filament.html

#include "forward_defs.frag"

// [Sch94] Fresnel approximation, used for F in Cook-Torrance BRDF.
vec3 FresnelSchlick(float HdotV, vec3 F0) {
    return F0 + (1.0 - F0) * pow(1.0 - HdotV, 5.0);
}

// Microfacet Normal Distribution Function, used for D in Cook-Torrance BRDF.
float DistributionGGX(float NdotH, float roughness) {
    // 'Roughness remapping' as per [Bur12]
    float a = roughness * roughness;

    // NDF from [Kar13], that cites [Bur12], which in turn cites [Wa07].
    // However, I could not find the same equation form in [Bur12] or deduce it myself from [Wa07],
    // and ended up taking the direct, untraceable form from [Kar13], so take this with a grain of salt.
    float a2 = a * a;
    float NdotH2 = NdotH * NdotH;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    return (a * a) / (PI * denom * denom);
}


float GeometrySchlickGGX(float NdotV, float roughness) {
    // Remapping of K for analytical (non-IBL) lighting per [Kar13].
    float r = (roughness + 1.0);
    float k = (r * r) / 8.0;

    // [Sch94] approximation of [Smi67] equation for [Bec63].
    return (NdotV) / (NdotV * (1.0 - k) + k);
}

// Geometric shadowing function, used for G in Cook-Torrance BRDF.
float GeometrySmith(float NdotV, float NdotL, float roughness) {
    // Smith geometric shadowing function.
    // [GA19] cites [Hei14] as demonstrating [Smi97] to be correct.
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);
    return ggx1 * ggx2;
}

// Cook-Torrance [CT82] specular model.
vec3 SpecularCookTorrance(float NdotH, float NdotV, float NdotL, vec3 F, float roughness) {
    float NDF = DistributionGGX(NdotH, roughness);
    float G = GeometrySmith(NdotV, NdotL, roughness);
    // F is taken in as a pre-computed argument for optimization purposes (it's reused for the
    // lambertian component of the lighting model).

    // Form from [Kar13], decuced from [CT82].
    vec3 specular = (NDF * G * F) / max((4.0 * NdotV * NdotL), 0.0001);
    return specular;
}

vec3 BRDFIlluminance(vec3 N, vec3 V, vec3 F0, vec3 albedo, float dielectric, float roughness) {
    // Luminance of this fragment.
    // Luminance is defined as the sum (integral) of all ilncoming illuminance over the half-sphere
    // 'above' that point. As we currently only support analytic lighting (ie. omni lights), we
    // integrate by iterating over all luminance sources, that currently are point lights.
    vec3 Lo = vec3(0.0);
    for (int i = 0; i < 4; ++i) {
        vec3 lightPos = ubo.omniLights[i].pos.xyz;
        vec3 lightColor = ubo.omniLights[i].color.xyz;

        // Unit vector pointing at light from fragment.
        vec3 L = normalize(lightPos - fragWorldPos);
        // Half-vector between to-light and to-camera unit vectors.
        vec3 H = normalize(V + L);

        // Dot products re-used across further computation for this (fragment, light) pair.
        float HdotV = max(dot(H, V), 0.0);
        float NdotH = max(dot(N, H), 0.0);
        float NdotV = max(dot(N, V), 0.0);
        float NdotL = max(dot(N, L), 0.0);

        // Translate luminous flux (lumen) into luminous intensity at this solid angle (candela).
        // This follows the derivation in [GA19] (58).
        float distance = length(lightPos - fragWorldPos);
        vec3 intensity = (lightColor / (4 * PI * (distance * distance)));

        // The Fresnel component from the Cook-Torrance specular BRDF is also used to calculate the
        // lambertian diffuse weight kD. We calculate it outside of the function.
        vec3 F = FresnelSchlick(HdotV, F0);
        // Cook-Torrance specular value.
        vec3 specular = SpecularCookTorrance(NdotH, NdotV, NdotL, F, roughness);

        // Lambertian diffuse component, influenced by fresnel and dielectric/metalness.
        vec3 kD = (vec3(1.0) - F) * dielectric;
        // Lambertian diffuse value.
        vec3 diffuse = albedo / PI;

        // Illuminance for this point from this light is a result of scaling the luminous
        // intensity of this light by the BRDL and by (N o L). This follows the definitions
        // of illuminance and luminous intensity.
        vec3 Li = (kD * diffuse + specular) * intensity * NdotL;

        // Integration of luminance from illuminance.
        Lo += Li;
    }
    return Lo;
}
