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
use cgmath::SquareMatrix;

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

use crate::input;

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
    pub input_device_id: u64,
    pub resolution: [u32; 2],
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
        ecs::ReadWriteGlobal<'a, input::Input>,
    );

    fn run(&mut self,
        ( transforms
        , renderables
        , status
        , scene
        , input): Self::SystemData,
    ) {
        let transformedRenderables = (transforms, renderables);
        let mut input = input.get();
        let mut status = status.get();

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

        match self.instance.swapchain_dimensions() {
            Some(res) => {
                status.resolution = res.clone()
            },
            None => (),
        }

        if status.input_device_id == 0 {
            status.input_device_id = input.allocate_device();
        }
        let (close, events) = self.poll_close();
        if close {
            status.closed = true;
        } else {
            let mut device = input.devices.entry(status.input_device_id).or_insert(input::Device::MouseCursor(input::MouseCursor::new()));
            if let &mut input::Device::MouseCursor(cursor) = &mut device {
                for event in events {
                    match event {
                        InternalEvent::MousePressed(button) => cursor.set_mouse_pressed(button),
                        InternalEvent::MouseReleased(button) => cursor.set_mouse_released(button),
                        InternalEvent::MouseMoved(x, y) => {
                            let (rx, ry) = (status.resolution[0], status.resolution[1]);
                            if rx != 0 && ry != 0 {
                                cursor.x = (x as f32) / (rx as f32);
                                cursor.y = (y as f32) / (ry as f32);
                            }
                        },
                    }
                }
            }
        }
    }
}

#[derive(Clone,Debug)]
enum InternalEvent {
    MousePressed(input::MouseButton),
    MouseReleased(input::MouseButton),
    MouseMoved(f64, f64),
}

impl Renderer {
    pub fn initialize(world: &mut ecs::World) -> Self {
        world.set_global(SceneInfo {
            camera: cgm::Point3::new(0.0, 0.0, 0.0),
            view: cgm::Matrix4::identity(),
        });
        world.set_global(Status {
            closed: false,
            input_device_id: 0,
            resolution: [0u32; 2],
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

    fn poll_close(&mut self) -> (bool, Vec<InternalEvent>) {
        let mut close = false;

        let mut events = vec![];
        // TODO(q3k): migrate to EventLoop::run
        self.events_loop.run_return(|ev, _, control_flow| {
            match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    close = true;
                },
                Event::MainEventsCleared => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                },
                Event::WindowEvent {
                    event:  WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    events.push(InternalEvent::MouseMoved(position.x, position.y));
                },
                Event::WindowEvent { 
                    event:  WindowEvent::MouseInput { state, button, .. },
                    ..
                } => {
                    let button = match button {
                        winit::event::MouseButton::Left => input::MouseButton::Left,
                        winit::event::MouseButton::Middle => input::MouseButton::Middle,
                        winit::event::MouseButton::Right => input::MouseButton::Right,
                        _ => input::MouseButton::Other,
                    };
                    match state {
                        winit::event::ElementState::Pressed => {
                            events.push(InternalEvent::MousePressed(button));
                        },
                        winit::event::ElementState::Released => {
                            events.push(InternalEvent::MouseReleased(button));
                        },
                    }
                },
                _ => {
                    *control_flow = winit::event_loop::ControlFlow::Poll;
                },
            }
        });
        return (close, events);
    }

    pub fn add_resource<T: Resource>(&mut self, r: T) -> ResourceID<T> {
        self.rm.add(r)
    }
}
