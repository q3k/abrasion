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
    surface: Arc<vs::Surface<Window>>,
    rm: resource::Manager,

    cursor_locked: bool,
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

    pub lock_cursor: bool,
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
        let scene = scene.get();

        // Render sceneinfo and renderables.
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
        let camera = &scene.camera;
        let view = &scene.view;
        self.instance.flip(camera, view, &rd, &self.rm);

        // Retrieve current resolution into status.
        match self.instance.swapchain_dimensions() {
            Some(res) => {
                status.resolution = res.clone()
            },
            None => (),
        }

        // Process events.
        if status.input_device_id == 0 {
            status.input_device_id = input.allocate_device();
        }
        let (close, events) = self.poll_close();
        if close {
            status.closed = true;
        } else {
            let mut device = input.devices.entry(status.input_device_id).or_insert(input::Device::MouseCursor(input::MouseCursor::new()));
            if let &mut input::Device::MouseCursor(cursor) = &mut device {

                let mut per_axis: BTreeMap<u32, Vec<f32>> = BTreeMap::new();
                let (rx, ry) = (status.resolution[0], status.resolution[1]);

                for event in events {
                    match event {
                        InternalEvent::MousePressed(button) => cursor.set_mouse_pressed(button),
                        InternalEvent::MouseReleased(button) => cursor.set_mouse_released(button),
                        InternalEvent::MouseMoved(x, y) => {
                            if rx != 0 && ry != 0 {
                                cursor.x = (x as f32) / (rx as f32);
                                cursor.y = (y as f32) / (ry as f32);
                            }
                        },
                        InternalEvent::AxisMotion(axis, delta) => {
                            per_axis.entry(axis).or_insert(vec![]).push(delta as f32);
                        },
                    }
                }

                // Has there been movement in any axis 0 (x) or 1 (y)? This happens if we receive
                // multiple AxisMotion events for a given axis in a single frame.
                let mut dx = 0f32;
                let mut dy = 0f32;
                if let Some(ldx) = per_axis.get(&0) {
                    dx = ldx.last().unwrap() - ldx.first().unwrap();
                }
                if let Some(ldy) = per_axis.get(&1) {
                    dy = ldy.last().unwrap() - ldy.first().unwrap();
                }

                if rx != 0 && ry != 0 {
                    cursor.dx = dx / (rx as f32);
                    cursor.dy = dy / (ry as f32);
                }
            }
        }

        let window = self.surface.window();
        if self.cursor_locked {
            if let Some(res) = self.instance.swapchain_dimensions() {
                let (x, y) = (res[0], res[1]);
                window.set_cursor_position(winit::dpi::PhysicalPosition::new(x / 2, y / 2));
            }
        }

        // Lock cursor, if requested.
        if scene.lock_cursor && !self.cursor_locked {
            window.set_cursor_visible(false);
            window.set_cursor_grab(true);
            self.cursor_locked = true;
        } else if self.cursor_locked && !scene.lock_cursor {
            window.set_cursor_visible(true);
            window.set_cursor_grab(false);
            self.cursor_locked = false;
        }
    }
}

#[derive(Clone,Debug)]
enum InternalEvent {
    MousePressed(input::MouseButton),
    MouseReleased(input::MouseButton),
    MouseMoved(f64, f64),
    AxisMotion(u32, f64),
}

impl Renderer {
    pub fn initialize(world: &mut ecs::World) -> Self {
        world.set_global(SceneInfo {
            camera: cgm::Point3::new(0.0, 0.0, 0.0),
            view: cgm::Matrix4::identity(),
            lock_cursor: false,
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
            surface,
            rm: resource::Manager::new(),

            cursor_locked: false,
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
            *control_flow = winit::event_loop::ControlFlow::Poll;
            match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    close = true;
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                },

                Event::MainEventsCleared => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                },

                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    events.push(InternalEvent::MouseMoved(position.x, position.y));
                },

                Event::WindowEvent { 
                    event: WindowEvent::MouseInput { state, button, .. },
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

                Event::WindowEvent {
                    event: WindowEvent::AxisMotion { axis, value, .. },
                    ..
                } => {
                    events.push(InternalEvent::AxisMotion(axis, value));
                },

                _ => {},
            }
        });
        return (close, events);
    }

    pub fn add_resource<T: Resource>(&mut self, r: T) -> ResourceID<T> {
        self.rm.add(r)
    }
}
