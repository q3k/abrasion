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

use log;
use env_logger;
use std::sync::Arc;
use std::time;

use cgmath as cgm;

mod render;
mod util;
mod physics;

use ecs::{component, world};
use render::vulkan::data;
use render::light::Omni;
use render::material::{Texture, PBRMaterialBuilder};
use render::mesh::Mesh;
use render::renderable::{Light, Object, Renderable, ResourceManager};
use physics::color;

#[derive(Debug)]
struct Dupa {
    zupa: u32,
    kupa: Vec<u32>,
}

impl component::Component for Dupa {}

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl component::Component for Position {}

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
    let light2 = rm.add_light(Omni::test(cgm::Vector3::new(-10.0, -10.0, -5.0)));

    // The Sun (Sol) is 1AU from the Earth. We ignore the diameter of the Sun and the Earth, as
    // these are negligible at this scale.
    let sun_distance: f32 = 149_597_870_700.0;
    // Solar constant: solar radiant power per square meter of earth's area [w/m^2].
    let solar_constant: f32 = 1366.0;
    // Solar luminous emittance (assuming 93 luminous efficacy) [lm/m^2].
    let sun_luminous_emittance: f32 = solar_constant * 93.0;
    // Solar luminour power (integrating over a sphere of radius == sun_distance) [lm].
    let sun_lumen: f32 = sun_luminous_emittance * (4.0 * 3.14159 * sun_distance * sun_distance);

    // In our scene, the sun at a 30 degree zenith.
    let sun_angle: f32 = (3.14159 * 2.0) / (360.0 / 30.0);
    let sun = rm.add_light(
        Omni::with_color(
            cgm::Vector3::new(0.0, sun_angle.sin() * sun_distance, sun_angle.cos() * sun_distance),
            color::XYZ::new(sun_lumen/3.0, sun_lumen/3.0, sun_lumen/3.0)
        )
    );


    let mut renderables: Vec<Box<dyn Renderable>> = cubes.into_iter().map(|e| e as Box<dyn Renderable>).collect();
    renderables.push(Box::new(Light{ light: light1 }));
    renderables.push(Box::new(Light{ light: light2 }));
    renderables.push(Box::new(Light{ light: sun }));

    let start = time::Instant::now();
    let mut renderer = render::Renderer::initialize();
    loop {
        let instant_ns = time::Instant::now().duration_since(start).as_nanos() as u64;
        let instant = ((instant_ns/1000) as f32) / 1_000_000.0;

        let position = (instant / 10.0) * 3.14 * 2.0;

        let camera = cgm::Point3::new(
            7.0 + (position / 4.0).sin(),
            12.0 + (position / 4.0).cos(),
            3.0
        );

        rm.light_mut(&light1).as_mut().unwrap().position = cgm::Vector3::new(
            -0.0 + (position*3.0).sin() * 4.0,
            -0.0 + (position*4.0).cos() * 4.0,
            -0.0 + (position*2.0).sin() * 3.0,
        );
        rm.light_mut(&light2).as_mut().unwrap().position = cgm::Vector3::new(
            -0.0 + (position*3.0).cos() * 4.0,
            -0.0 + (position*4.0).sin() * 4.0,
            -0.0 + (position*2.0).cos() * 3.0,
        );

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
