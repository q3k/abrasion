use cgmath as cgm;

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 3]) -> Self {
        Self { pos, color }
    }
}
vulkano::impl_vertex!(Vertex, pos, color);

#[derive(Copy, Clone)]
pub struct UniformBufferObject {
    pub model: cgm::Matrix4<f32>,
}
