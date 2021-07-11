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

use std::sync::Arc;

use crate::mesh::Vertex;
vulkano::impl_vertex!(Vertex, pos, normal, tex);

#[derive(Default, Copy, Clone)]
pub struct Instance {
    model: [f32; 16],
}

impl Instance {
    pub fn new(model: &cgmath::Matrix4<f32>) -> Self {
        let slice: &[f32; 16] = model.as_ref();
        Self { 
            model: slice.clone(),
        }
    }
}
vulkano::impl_vertex!(Instance, model);

#[derive(Copy, Clone, Debug)]
pub struct OmniLight {
    pub pos: [f32; 4],
    pub color: [f32; 4],
}

#[derive(Copy, Clone, Debug)]
pub struct PushConstantObject {
    pub view: cgmath::Matrix4<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct FragmentUniformBufferObject {
    pub camera_pos: cgmath::Vector4<f32>,
    pub omni_lights: [OmniLight; 4],
}

#[derive(Clone, Debug)]
pub struct Textures {
    // diffuse: RGB
    pub diffuse: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>,
    // roughness: R
    pub roughness: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>,
}

pub struct VertexData {
    pub vbuffer: Arc<vulkano::buffer::ImmutableBuffer<[Vertex]>>,
    pub ibuffer: Arc<vulkano::buffer::ImmutableBuffer<[u16]>>,
}

impl std::fmt::Debug for VertexData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexData").finish()
    }
}
