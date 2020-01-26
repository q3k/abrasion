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

    let elapsed = 0.0;
    let transform = cgm::Matrix4::from_angle_z(cgm::Rad::from(cgm::Deg(elapsed as f32 * 0.180)));
    let vertices = Arc::new(vec![
        data::Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
        data::Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
        data::Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),
        data::Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 1.0])
    ]);
    let indices = Arc::new(vec![
        0, 1, 2, 2, 3, 0,
    ]);
    let demo = render::renderable::Mesh {
        transform, vertices, indices
    };

    let mut renderer = render::Renderer::initialize();
    renderer.set_render_data(vec![demo.data().unwrap()]);
    renderer.main_loop();
}
