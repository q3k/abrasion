use log;
use env_logger;
use std::sync::Arc;
use std::time;

use cgmath as cgm;

mod render;
mod util;
mod physics;

use render::vulkan::data;
use render::light::Omni;
use render::material::{Texture, PBRMaterialBuilder};
use render::mesh::Mesh;
use render::renderable::{Light, Object, Renderable, ResourceManager};
use physics::color;

fn main() {
    env_logger::init();
    log::info!("Starting...");

    let mut rm = ResourceManager::new();

    let mesh = {
        let vertices = Arc::new(vec![
            data::Vertex::new([-0.5, -0.5,  0.5], [ 0.0,  0.0,  1.0], [1.0, 0.0]),
            data::Vertex::new([ 0.5, -0.5,  0.5], [ 0.0,  0.0,  1.0], [0.0, 0.0]),
            data::Vertex::new([ 0.5,  0.5,  0.5], [ 0.0,  0.0,  1.0], [0.0, 1.0]),
            data::Vertex::new([-0.5,  0.5,  0.5], [ 0.0,  0.0,  1.0], [1.0, 1.0]),

            data::Vertex::new([ 0.5, -0.5, -0.5], [ 1.0,  0.0,  0.0], [0.0, 1.0]),
            data::Vertex::new([ 0.5,  0.5, -0.5], [ 1.0,  0.0,  0.0], [1.0, 1.0]),
            data::Vertex::new([ 0.5,  0.5,  0.5], [ 1.0,  0.0,  0.0], [1.0, 0.0]),
            data::Vertex::new([ 0.5, -0.5,  0.5], [ 1.0,  0.0,  0.0], [0.0, 0.0]),

            data::Vertex::new([-0.5, -0.5, -0.5], [-1.0,  0.0,  0.0], [1.0, 1.0]),
            data::Vertex::new([-0.5,  0.5, -0.5], [-1.0,  0.0,  0.0], [0.0, 1.0]),
            data::Vertex::new([-0.5,  0.5,  0.5], [-1.0,  0.0,  0.0], [0.0, 0.0]),
            data::Vertex::new([-0.5, -0.5,  0.5], [-1.0,  0.0,  0.0], [1.0, 0.0]),

            data::Vertex::new([-0.5, -0.5, -0.5], [ 0.0, -1.0,  0.0], [0.0, 1.0]),
            data::Vertex::new([ 0.5, -0.5, -0.5], [ 0.0, -1.0,  0.0], [1.0, 1.0]),
            data::Vertex::new([ 0.5, -0.5,  0.5], [ 0.0, -1.0,  0.0], [1.0, 0.0]),
            data::Vertex::new([-0.5, -0.5,  0.5], [ 0.0, -1.0,  0.0], [0.0, 0.0]),

            data::Vertex::new([-0.5,  0.5, -0.5], [ 0.0,  1.0,  0.0], [1.0, 1.0]),
            data::Vertex::new([ 0.5,  0.5, -0.5], [ 0.0,  1.0,  0.0], [0.0, 1.0]),
            data::Vertex::new([ 0.5,  0.5,  0.5], [ 0.0,  1.0,  0.0], [0.0, 0.0]),
            data::Vertex::new([-0.5,  0.5,  0.5], [ 0.0,  1.0,  0.0], [1.0, 0.0]),

            data::Vertex::new([-0.5, -0.5, -0.5], [ 0.0,  0.0, -1.0], [0.0, 0.0]),
            data::Vertex::new([ 0.5, -0.5, -0.5], [ 0.0,  0.0, -1.0], [1.0, 0.0]),
            data::Vertex::new([ 0.5,  0.5, -0.5], [ 0.0,  0.0, -1.0], [1.0, 1.0]),
            data::Vertex::new([-0.5,  0.5, -0.5], [ 0.0,  0.0, -1.0], [0.0, 1.0]),
        ]);
        let indices = Arc::new(vec![
            0, 1, 2, 2, 3, 0,

            4, 5, 6, 6, 7, 4,
            8, 10, 9, 10, 8, 11,

            12, 13, 14, 14, 15, 12,
            16, 18, 17, 18, 16, 19,

            20, 22, 21, 22, 20, 23,

        ]);
        rm.add_mesh(Mesh::new(vertices, indices))
    };

    let material = rm.add_material(PBRMaterialBuilder {
        diffuse: Texture::from_image(String::from("assets/test-128px.png")),
        roughness: Texture::from_color(color::LinearF32::new(1.0)),
    }.build());


    let mut cubes: Vec<Box<Object>> = vec![];
    for x in -20..20 {
        for y in -20..20 {
            for z in -20..20 {
                let transform = cgm::Matrix4::from_translation(cgm::Vector3::new((x as f32)*4.0, (y as f32)*4.0, (z as f32)*4.0));
                let cube = render::renderable::Object {
                    mesh, material, transform,
                };
                cubes.push(Box::new(cube));
            }
        }
    }

    let light1 = rm.add_light(Omni::test(cgm::Vector3::new(-10.0, -10.0, -5.0)));
    let light2 = rm.add_light(Omni::test(cgm::Vector3::new(10.0, 10.0, -5.0)));


    let mut renderables: Vec<Box<dyn Renderable>> = cubes.into_iter().map(|e| e as Box<dyn Renderable>).collect();
    renderables.push(Box::new(Light{ light: light1 }));
    renderables.push(Box::new(Light{ light: light2 }));

    let start = time::Instant::now();
    let mut renderer = render::Renderer::initialize();
    loop {
        let instant_ns = time::Instant::now().duration_since(start).as_nanos() as u64;
        let instant = ((instant_ns/1000) as f32) / 1_000_000.0;

        let position = (instant / 10.0) * 3.14 * 2.0;

        let camera = cgm::Point3::new(
            //position.cos() * 10.0 * (((position*2.0).cos()/2.0)+1.0),
            //position.sin() * 10.0 * (((position*2.0).cos()/2.0)+1.0),
            7.0 + (position / 4.0).sin(),
            12.0 + (position / 4.0).cos(),
            3.0
        );

        if let Some(light) = rm.light_mut(&light1) {
            light.position = cgm::Vector3::new(
                -0.0 + (position*3.0).sin() * 4.0,
                -0.0 + (position*4.0).cos() * 4.0,
                -0.0 + (position*2.0).sin() * 1.0,
            );
        }

        let view = cgm::Matrix4::look_at(
            camera.clone(),
            cgm::Point3::new(0.0, 0.0, 0.0),
            cgm::Vector3::new(0.0, 0.0, 1.0)
        );

        renderer.draw_frame(&camera, &view, &rm, &renderables);
        if renderer.poll_close() {
            return;
        }
    }
}
