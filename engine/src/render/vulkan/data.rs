use std::sync::Arc;

use vulkano::buffer as vb;
use vulkano::image as vm;
use vulkano::format::Format;

use cgmath as cgm;

#[derive(Default, Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    normal: [f32; 3],
    tex: [f32; 2],
}

impl Vertex {
    pub fn new(pos: [f32; 3], normal: [f32; 3], tex: [f32; 2]) -> Self {
        Self {
            pos, normal, tex,
        }
    }
}
vulkano::impl_vertex!(Vertex, pos, normal, tex);

#[derive(Default, Copy, Clone)]
pub struct Instance {
    model: [f32; 16],
}

impl Instance {
    pub fn new(model: &cgm::Matrix4<f32>) -> Self {
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
    pub view: cgm::Matrix4<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct FragmentUniformBufferObject {
    pub camera_pos: cgm::Vector4<f32>,
    pub omni_lights: [OmniLight; 4],
}

#[derive(Clone)]
pub struct Textures {
    // diffuse: RGB
    pub diffuse: Arc<vm::ImmutableImage<Format>>,
    // roughness: R
    pub roughness: Arc<vm::ImmutableImage<Format>>,
}

pub struct VertexData {
    pub vbuffer: Arc<vb::ImmutableBuffer<[Vertex]>>,
    pub ibuffer: Arc<vb::ImmutableBuffer<[u16]>>,
}
