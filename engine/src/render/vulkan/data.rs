use cgmath as cgm;

#[derive(Default, Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
    tex: [f32; 2],
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 3], tex: [f32; 2]) -> Self {
        Self {
            pos, color, tex,
        }
    }
}
vulkano::impl_vertex!(Vertex, pos, color, tex);

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

#[derive(Copy, Clone)]
pub struct UniformBufferObject {
    pub view: cgm::Matrix4<f32>,
}
