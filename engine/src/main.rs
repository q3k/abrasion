use log;
use env_logger;
use std::rc::Rc;

use cgmath as cgm;

mod render;
mod util;

use render::vulkan::data;
use render::renderable::Renderable;

fn main() {
    env_logger::init();
    log::info!("Starting...");

    let mut renderables = Vec::new();
    for i in 1..1000 {
        let elapsed = 0.0;
        let transform = cgm::Matrix4::from_angle_z(cgm::Rad::from(cgm::Deg(elapsed as f32 * 0.180))) *
            cgm::Matrix4::from_translation(cgm::Vector3::new(0.0, 0.0, (i as f32)/1000.0));
        let vertices = Rc::new(vec![
            data::Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
            data::Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
            data::Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),
            data::Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 1.0])
        ]);
        let indices = Rc::new(vec![
            0, 1, 2, 2, 3, 0,
        ]);
        let demo = render::renderable::Mesh {
            transform, vertices, indices
        };
        renderables.push(demo.data().unwrap());
    }

    let mut renderer = render::Renderer::initialize();
    renderer.set_render_data(renderables);
    renderer.main_loop();
}
