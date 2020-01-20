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

pub fn vertices() -> [Vertex; 3] {
    [
        Vertex::new([0.0, -0.5, 0.0], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([-0.5, 0.5, 0.0], [0.0, 0.0, 1.])
    ]
}
