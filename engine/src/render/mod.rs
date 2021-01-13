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

use std::sync::Arc;
use std::collections::BTreeMap;

use cgmath as cgm;

use winit::{
    dpi::LogicalSize,
    window::Window,
    window::WindowBuilder,
    event_loop::EventLoop,
    event::Event,
    event::WindowEvent,
    platform::run_return::EventLoopExtRunReturn,
};
use vulkano_win::VkSurfaceBuild;
use vulkano::instance as vi;
use vulkano::swapchain as vs;

use ecs::Join;

pub mod light;
pub mod material;
pub mod mesh;
pub mod renderable;
pub mod resource;
pub mod vulkan;

pub use light::Light;
pub use material::Material;
pub use mesh::Mesh;
pub use renderable::{Transform, Renderable};
pub use resource::{Resource, ResourceID};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Renderer {
    instance: vulkan::Instance<Window>,
    events_loop: EventLoop<()>,
    rm: resource::Manager,
}

#[derive(Clone, Debug)]
pub struct Status {
    pub closed: bool,
}
impl ecs::Global for Status {}

#[derive(Clone, Debug)]
pub struct SceneInfo {
    pub camera: cgm::Point3<f32>,
    pub view: cgm::Matrix4<f32>,
}
impl ecs::Global for SceneInfo {}

impl<'a> ecs::System<'a> for Renderer {
    type SystemData = (
        ecs::ReadComponent<'a, Transform>,
        ecs::ReadComponent<'a, Renderable>,
        ecs::ReadWriteGlobal<'a, Status>,
        ecs::ReadGlobal<'a, SceneInfo>,
    );

    fn run(&mut self,
        ( transforms
        , renderables
        , status
        , scene): Self::SystemData,
    ) {
        let transformedRenderables = (transforms, renderables);

        let mut rd = vulkan::RenderData {
            meshes: BTreeMap::new(),
            lights: Vec::new(),
        };
        for (transform, renderable) in transformedRenderables.join_all() {
            match renderable {
                Renderable::Light(lrid) => {
                    rd.lights.push((*lrid, transform.xyzw()));
                },
                Renderable::Mesh(mesh_id, material_id) => {
                    rd.meshes.entry((*mesh_id, *material_id)).or_insert(Vec::new()).push(transform.m4());
                },
                _ => (),
            }
        }

        let camera = &scene.get().camera;
        let view = &scene.get().view;
        self.instance.flip(camera, view, &rd, &self.rm);

        if self.poll_close() {
            status.get().closed = true;
        }
    }
}

impl Renderer {
    pub fn initialize(world: &mut ecs::World) -> Self {
        world.set_global(Status{
            closed: false,
        });

        let mut instance = vulkan::Instance::new("abrasion".to_string());
        let (events_loop, surface) = Self::init_window(instance.get_vulkan());
        instance.use_surface(&surface);

        Self {
            instance,
            events_loop,
            rm: resource::Manager::new(),
        }
    }

    fn init_window(instance: Arc<vi::Instance>) -> (EventLoop<()>, Arc<vs::Surface<Window>>) {
        let events_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .with_title("abrasion")
            .with_inner_size(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build_vk_surface(&events_loop, instance.clone())
            .expect("could not create surface");
        (events_loop, surface)
    }

    fn poll_close(&mut self) -> bool {
        let mut close = false;
        // TODO(q3k): migrate to EventLoop::run
        self.events_loop.run_return(|ev, _, control_flow| {
            if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = ev {
                close = true;
            }
            *control_flow = winit::event_loop::ControlFlow::Exit;
        });
        return close;
    }

    pub fn add_resource<T: Resource>(&mut self, r: T) -> ResourceID<T> {
        self.rm.add(r)
    }
}
