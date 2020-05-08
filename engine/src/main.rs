use log;
use env_logger;
use std::sync::Arc;
use std::time;

use cgmath as cgm;

mod render;
mod util;

use render::vulkan::data;
use render::renderable::{Object, Renderable};

fn main() {
    env_logger::init();
    log::info!("Starting...");

    let mesh_cube = {
        let vertices = Arc::new(vec![
            data::Vertex::new([-0.5, -0.5, 0.5], [1.0, 1.0, 1.0], [1.0, 0.0]),
            data::Vertex::new([0.5, -0.5, 0.5], [1.0, 1.0, 0.0], [0.0, 0.0]),
            data::Vertex::new([0.5, 0.5, 0.5], [0.0, 1.0, 1.0], [0.0, 1.0]),
            data::Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0, 1.0], [1.0, 1.0]),

            data::Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0, 1.0], [0.0, 1.0]),
            data::Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 0.0], [1.0, 1.0]),
            data::Vertex::new([0.5, 0.5, 0.5], [0.0, 1.0, 1.0], [1.0, 0.0]),
            data::Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0, 1.0], [0.0, 0.0]),

            data::Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0], [1.0, 1.0]),
            data::Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0, 0.0], [0.0, 1.0]),
            data::Vertex::new([-0.5, 0.5, 0.5], [0.0, 1.0, 1.0], [0.0, 0.0]),
            data::Vertex::new([-0.5, -0.5, 0.5], [1.0, 0.0, 1.0], [1.0, 0.0]),

            data::Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0], [0.0, 1.0]),
            data::Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0, 0.0], [1.0, 1.0]),
            data::Vertex::new([0.5, -0.5, 0.5], [0.0, 1.0, 1.0], [1.0, 0.0]),
            data::Vertex::new([-0.5, -0.5, 0.5], [1.0, 0.0, 1.0], [0.0, 0.0]),

            data::Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0, 1.0], [1.0, 1.0]),
            data::Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 0.0], [0.0, 1.0]),
            data::Vertex::new([0.5, 0.5, 0.5], [0.0, 1.0, 1.0], [0.0, 0.0]),
            data::Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0, 1.0], [1.0, 0.0]),

            data::Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0], [0.0, 0.0]),
            data::Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0, 0.0], [1.0, 0.0]),
            data::Vertex::new([0.5, 0.5, -0.5], [0.0, 1.0, 1.0], [1.0, 1.0]),
            data::Vertex::new([-0.5, 0.5, -0.5], [1.0, 0.0, 1.0], [0.0, 1.0]),
        ]);
        let indices = Arc::new(vec![
            0, 1, 2, 2, 3, 0,

            4, 5, 6, 6, 7, 4,
            8, 10, 9, 10, 8, 11,

            12, 13, 14, 14, 15, 12,
            16, 18, 17, 18, 16, 19,

            20, 22, 21, 22, 20, 23,

        ]);
        Arc::new(render::renderable::Mesh::new(vertices, indices))
    };

    let path = &crate::util::file::resource_path(String::from("assets/test-128px.png"));
    let image = Arc::new(image::open(path).unwrap());
    let texture_cube = Arc::new(render::renderable::Texture::new(image));

    let mut renderer = render::Renderer::initialize();

    let mut cubes: Vec<Arc<Object>> = Vec::new();
    for x in -20..20 {
        for y in -20..20 {
            for z in -20..20 {
                let transform = cgm::Matrix4::from_translation(cgm::Vector3::new((x as f32)*4.0, (y as f32)*4.0, (z as f32)*4.0));
                let cube = render::renderable::Object {
                    mesh: mesh_cube.clone(),
                    transform,
                    texture: texture_cube.clone(),
                };
                cubes.push(Arc::new(cube));
            }
        }
    }

    let mut renderables: Vec<Arc<dyn Renderable>> = Vec::with_capacity(2000);
    for c in cubes.iter() {
        renderables.push(c.clone());
    }

    let start = time::Instant::now();
    loop {
        let instant_ns = time::Instant::now().duration_since(start).as_nanos() as u64;
        let instant = ((instant_ns/1000) as f32) / 1_000_000.0;

        let position = (instant / 10.0) * 3.14 * 2.0;

        let view = cgm::Matrix4::look_at(
            cgm::Point3::new(
                position.cos() * 10.0 * (((position*2.0).cos()/2.0)+1.0),
                position.sin() * 10.0 * (((position*2.0).cos()/2.0)+1.0),
                3.0
            ),
            cgm::Point3::new(0.0, 0.0, 0.0),
            cgm::Vector3::new(0.0, 0.0, 1.0)
        );

        renderer.draw_frame(&view, &renderables);
        if renderer.poll_close() {
            return;
        }
    }
}
