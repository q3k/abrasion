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

use ecs::{Component, World, Processor};
use render::vulkan::data;
use render::material::{Texture, PBRMaterialBuilder};
use render::{Light, Material, Mesh, Transform, Renderable};
use physics::color;

struct Time {
    start: time::Instant,
    now: time::Instant,
}
impl ecs::Global for Time {}

impl Time {
    pub fn instant(&self) -> f32  {
        let instant_ns = self.now.duration_since(self.start).as_nanos() as u64;
        let instant = ((instant_ns/1000) as f32) / 1_000_000.0;
        instant
    }
}


struct Main {
    light1: ecs::EntityID,
    light2: ecs::EntityID,
}

impl Main {
    pub fn new(world: &mut World, renderer: &mut render::Renderer) -> Self {
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
            renderer.add_resource(Mesh::new(vertices, indices))
        };

        let material = renderer.add_resource(PBRMaterialBuilder {
            diffuse: Texture::from_image(String::from("assets/test-128px.png")),
            roughness: Texture::from_color(color::LinearF32::new(1.0)),
        }.build());

        for x in -20..20 {
            for y in -20..20 {
                for z in -20..20 {
                    world.new_entity()
                        .with(Transform::at((x as f32)*4.0, (y as f32)*4.0, (z as f32)*4.0))
                        .with(Renderable::Mesh(mesh, material))
                        .build();
                }
            }
        }

        let light = renderer.add_resource(Light::omni_test());

        // The Sun (Sol) is 1AU from the Earth. We ignore the diameter of the Sun and the Earth, as
        // these are negligible at this scale.
        let sun_distance: f32 = 149_597_870_700.0;
        // Solar constant: solar radiant power per square meter of earth's area [w/m^2].
        let solar_constant: f32 = 1366.0;
        // Solar luminous emittance (assuming 93 luminous efficacy) [lm/m^2].
        let sun_luminous_emittance: f32 = solar_constant * 93.0;
        // Solar luminour power (integrating over a sphere of radius == sun_distance) [lm].
        let sun_lumen: f32 = sun_luminous_emittance * (4.0 * 3.14159 * sun_distance * sun_distance);

        let sun_color = color::XYZ::new(sun_lumen/3.0, sun_lumen/3.0, sun_lumen/3.0);
        let sun = renderer.add_resource(Light::omni_with_color(sun_color));

        // In our scene, the sun at a 30 degree zenith.
        let sun_angle: f32 = (3.14159 * 2.0) / (360.0 / 30.0);
        
        let light1 = world.new_entity()
            .with(Transform::at(-10.0, -10.0, -5.0))
            .with(Renderable::Light(light))
            .build();
        let light2 = world.new_entity()
            .with(Transform::at(-10.0, -10.0, -5.0))
            .with(Renderable::Light(light))
            .build();
        world.new_entity()
            .with(Transform::at(0.0, sun_angle.sin() * sun_distance, sun_angle.cos() * sun_distance))
            .with(Renderable::Light(sun))
            .build();

        Self {
            light1, light2
        }
    }
}

impl<'a> ecs::System <'a> for Main {
    type SystemData = ( ecs::ReadWriteGlobal<'a, render::SceneInfo>
                      , ecs::ReadGlobal<'a, Time>
                      );
    fn run(&mut self, (scene_info, time): Self::SystemData) {
        let position = (time.get().instant() / 10.0) * 3.14 * 2.0;

        let camera = cgm::Point3::new(
            7.0 + (position / 4.0).sin(),
            12.0 + (position / 4.0).cos(),
            3.0
        );

        let view = cgm::Matrix4::look_at(
            camera.clone(),
            cgm::Point3::new(0.0, 0.0, 0.0),
            cgm::Vector3::new(0.0, 0.0, 1.0)
        );

        scene_info.get().camera = camera;
        scene_info.get().view = view;

        //rm.light_mut(&light1).as_mut().unwrap().position = cgm::Vector3::new(
        //    -0.0 + (position*3.0).sin() * 4.0,
        //    -0.0 + (position*4.0).cos() * 4.0,
        //    -0.0 + (position*2.0).sin() * 3.0,
        //);
        //rm.light_mut(&light2).as_mut().unwrap().position = cgm::Vector3::new(
        //    -0.0 + (position*3.0).cos() * 4.0,
        //    -0.0 + (position*4.0).sin() * 4.0,
        //    -0.0 + (position*2.0).cos() * 3.0,
        //);

    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();
    let mut renderer = render::Renderer::initialize(&mut world);
    let main = Main::new(&mut world, &mut renderer);

    log::info!("Starting...");

    let mut p = Processor::new(&world);
    p.add_system(renderer);
    p.add_system(main);

    let start = time::Instant::now();
    world.set_global(Time{
        start,
        now: start,
    });
    loop {
        world.global_mut::<Time>().get().now = time::Instant::now();

        p.run();
        let status = world.global::<render::Status>().get();
        if status.closed {
            log::info!("Renderer closed, exiting.");
            return;
        }
    }
}
