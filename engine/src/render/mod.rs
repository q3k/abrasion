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

pub mod light;
pub mod material;
pub mod mesh;
pub mod renderable;
pub mod vulkan;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Renderer {
    instance: vulkan::Instance<Window>,
    events_loop: EventLoop<()>,
}

impl Renderer {
    pub fn initialize() -> Self {
        let mut instance = vulkan::Instance::new("abrasion".to_string());
        let (events_loop, surface) = Self::init_window(instance.get_vulkan());
        instance.use_surface(&surface);

        Self {
            instance,
            events_loop,
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

    pub fn draw_frame(
        &mut self,
        camera: &cgm::Point3<f32>,
        view: &cgm::Matrix4<f32>,
        rm: &renderable::ResourceManager,
        renderables: &Vec<Box<dyn renderable::Renderable>>
    ) {
        self.instance.flip(camera, view, rm, renderables);
    }

    pub fn poll_close(&mut self) -> bool {
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
}
