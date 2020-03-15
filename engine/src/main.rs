use log;
use env_logger;
use std::sync::Arc;

use cgmath as cgm;

mod render;
mod util;

use render::vulkan::data;
use render::renderable::Renderable;

fn main() {
    env_logger::init();
    log::info!("Starting...");

    let mesh_cube = {
        let vertices = Arc::new(vec![
            data::Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
            data::Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
            data::Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),
            data::Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 1.0])
        ]);
        let indices = Arc::new(vec![
            0, 1, 2, 2, 3, 0,
        ]);
        Arc::new(render::renderable::Mesh::new(vertices, indices))
    };

    let mut renderables: Vec<Arc<dyn Renderable>> = Vec::new();
    for i in 1..100000 {
        let transform = cgm::Matrix4::from_translation(cgm::Vector3::new(0.0, 0.0, (i as f32)/1000.0));
        let cube = render::renderable::Object {
            mesh: mesh_cube.clone(),
            transform
        };
        renderables.push(Arc::new(cube));
    }

    let mut renderer = render::Renderer::initialize();
    renderer.set_renderables(renderables);
    renderer.main_loop();
}
